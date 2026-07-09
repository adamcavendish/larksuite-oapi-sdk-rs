use std::future::Future;
use std::pin::Pin;

use http::header::{CONTENT_TYPE, HeaderValue, USER_AGENT as UA_HEADER};
use serde_json::Value;
use tracing::Instrument;

use crate::config::Config;
use crate::constants::{
    AccessTokenType, AppType, CUSTOM_REQUEST_ID, ERR_CODE_ACCESS_TOKEN_INVALID,
    ERR_CODE_APP_ACCESS_TOKEN_INVALID, ERR_CODE_APP_TICKET_INVALID,
    ERR_CODE_TENANT_ACCESS_TOKEN_INVALID, USER_AGENT,
};
use crate::error::LarkError;
use crate::req::{ApiReq, FormDataValue, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::token::{AppTicketManager, TokenManager};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub(crate) fn request<'a>(
    config: &'a Config,
    api_req: &'a ApiReq,
    option: &'a RequestOption,
) -> BoxFuture<'a, Result<ApiResp, LarkError>> {
    Box::pin(async move {
        let span = request_span(api_req);

        let token_type = span.in_scope(|| prepare_request(config, api_req, option))?;

        do_request(config, api_req, option, token_type)
            .instrument(span)
            .await
    })
}

pub(crate) fn request_stream<'a>(
    config: &'a Config,
    api_req: &'a ApiReq,
    option: &'a RequestOption,
) -> BoxFuture<'a, Result<StreamResp, LarkError>> {
    Box::pin(async move {
        let span = request_span(api_req);

        let token_type = span.in_scope(|| prepare_request(config, api_req, option))?;

        do_request_stream(config, api_req, option, token_type)
            .instrument(span)
            .await
    })
}

pub(crate) async fn request_typed<T: for<'de> serde::Deserialize<'de>>(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
) -> Result<(ApiResp, RawResponse<T>), LarkError> {
    let resp = request(config, api_req, option).await?;

    if option.file_download {
        return Ok((
            resp,
            RawResponse {
                code_error: CodeError::default(),
                data: None,
            },
        ));
    }

    let raw: RawResponse<T> = serde_json::from_slice(&resp.raw_body)?;
    if !raw.code_error.success() {
        return Err(LarkError::Api(Box::new(raw.code_error)));
    }
    Ok((resp, raw))
}

#[derive(Debug)]
pub(crate) struct StreamResp {
    pub(crate) api_resp: ApiResp,
    pub(crate) body: StreamBody,
}

#[derive(Debug)]
pub(crate) struct StreamBody {
    replay: std::collections::VecDeque<bytes::Bytes>,
    inner: Option<aioduct::BodyStreamSend>,
}

impl StreamBody {
    fn streaming(inner: aioduct::BodyStreamSend) -> Self {
        Self {
            replay: std::collections::VecDeque::new(),
            inner: Some(inner),
        }
    }

    fn replay(chunk: bytes::Bytes) -> Self {
        Self {
            replay: [chunk].into(),
            inner: None,
        }
    }

    pub(crate) async fn next_chunk(&mut self) -> Result<Option<bytes::Bytes>, LarkError> {
        if let Some(chunk) = self.replay.pop_front() {
            return Ok(Some(chunk));
        }

        let Some(inner) = &mut self.inner else {
            return Ok(None);
        };

        match inner.next().await {
            Some(Ok(chunk)) => Ok(Some(chunk)),
            Some(Err(err)) => Err(LarkError::Http(err)),
            None => {
                self.inner = None;
                Ok(None)
            }
        }
    }
}

fn request_span(api_req: &ApiReq) -> tracing::Span {
    tracing::info_span!(
        "lark.request",
        method = %api_req.http_method,
        path = %api_req.api_path,
    )
}

fn prepare_request(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
) -> Result<AccessTokenType, LarkError> {
    validate(config, api_req, option)?;
    Ok(determine_token_type(config, api_req, option))
}

fn validate(config: &Config, api_req: &ApiReq, option: &RequestOption) -> Result<(), LarkError> {
    if config.app_id.is_empty() {
        return Err(LarkError::IllegalParam("app_id is empty".to_string()));
    }

    let has_manual_token = option.user_access_token.is_some()
        || option.tenant_access_token.is_some()
        || option.app_access_token.is_some();

    if config.client_assertion_provider.is_some() {
        if config.app_type == AppType::Marketplace {
            return Err(LarkError::ClientAssertion(
                "ClientAssertion mode is not supported for marketplace apps".to_string(),
            ));
        }
    } else if config.app_secret.is_empty() && !has_manual_token {
        return Err(LarkError::IllegalParam("app_secret is empty".to_string()));
    }

    if config.app_type == AppType::Marketplace {
        let needs_tenant_token = api_req
            .supported_access_token_types
            .contains(&AccessTokenType::Tenant);
        if needs_tenant_token && option.tenant_key.is_none() && config.enable_token_cache {
            return Err(LarkError::IllegalParam(
                "marketplace app requires tenant_key for tenant access token".to_string(),
            ));
        }
    }

    if let Some(ref headers) = option.headers {
        if headers.contains_key("X-Request-Id") {
            return Err(LarkError::IllegalParam(
                "use X-Request-Id as header key is not allowed".to_string(),
            ));
        }
        if headers.contains_key("Request-Id") {
            return Err(LarkError::IllegalParam(
                "use Request-Id as header key is not allowed".to_string(),
            ));
        }
    }

    validate_token_type(&api_req.supported_access_token_types, option)?;

    Ok(())
}

fn validate_token_type(
    supported: &[AccessTokenType],
    option: &RequestOption,
) -> Result<(), LarkError> {
    if supported.is_empty() || supported.contains(&AccessTokenType::None) {
        return Ok(());
    }

    if option.user_access_token.is_some() && !supported.contains(&AccessTokenType::User) {
        return Err(LarkError::IllegalParam(
            "user access token is not supported for this API".to_string(),
        ));
    }
    if option.tenant_access_token.is_some() && !supported.contains(&AccessTokenType::Tenant) {
        return Err(LarkError::IllegalParam(
            "tenant access token is not supported for this API".to_string(),
        ));
    }
    if option.app_access_token.is_some() && !supported.contains(&AccessTokenType::App) {
        return Err(LarkError::IllegalParam(
            "app access token is not supported for this API".to_string(),
        ));
    }

    Ok(())
}

fn determine_token_type(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
) -> AccessTokenType {
    if api_req.supported_access_token_types.is_empty()
        || api_req
            .supported_access_token_types
            .contains(&AccessTokenType::None)
    {
        return AccessTokenType::None;
    }

    if config.client_assertion_provider.is_some() {
        if option.user_access_token.is_some()
            && api_req
                .supported_access_token_types
                .contains(&AccessTokenType::User)
        {
            return AccessTokenType::User;
        }
        if api_req
            .supported_access_token_types
            .contains(&AccessTokenType::Tenant)
        {
            return AccessTokenType::Tenant;
        }
        if api_req
            .supported_access_token_types
            .contains(&AccessTokenType::App)
        {
            return AccessTokenType::App;
        }
        return AccessTokenType::None;
    }

    if !config.enable_token_cache {
        if option.user_access_token.is_some() {
            return AccessTokenType::User;
        }
        if option.tenant_access_token.is_some() {
            return AccessTokenType::Tenant;
        }
        if option.app_access_token.is_some() {
            return AccessTokenType::App;
        }
        return AccessTokenType::None;
    }

    if option.user_access_token.is_some() {
        return AccessTokenType::User;
    }

    if api_req
        .supported_access_token_types
        .contains(&AccessTokenType::Tenant)
    {
        return AccessTokenType::Tenant;
    }
    if api_req
        .supported_access_token_types
        .contains(&AccessTokenType::App)
    {
        return AccessTokenType::App;
    }

    AccessTokenType::None
}

async fn do_request(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    token_type: AccessTokenType,
) -> Result<ApiResp, LarkError> {
    retry_request(config, option, token_type, |bearer| async move {
        let resp = raw_send(config, api_req, option, token_type, bearer.as_deref()).await?;
        classify_buffered_response(config, resp).await
    })
    .await
}

async fn classify_buffered_response(
    config: &Config,
    resp: ApiResp,
) -> Result<RequestAttempt<ApiResp>, LarkError> {
    if is_json_content_type(&resp)
        && let Some(err) = json_code_error_retry(config, &resp.raw_body).await
    {
        return Ok(RequestAttempt::Retry(err));
    }

    Ok(RequestAttempt::Done(resp))
}

async fn json_code_error_retry(config: &Config, raw_body: &[u8]) -> Option<LarkError> {
    let Ok(code_err) = serde_json::from_slice::<CodeError>(raw_body) else {
        return None;
    };

    if code_err.code == ERR_CODE_APP_TICKET_INVALID {
        let atm = AppTicketManager::new(config.token_cache.clone());
        let _ = atm.apply_app_ticket(config).await;
    }
    if is_token_invalid_error(code_err.code) {
        return Some(LarkError::Api(Box::new(code_err)));
    }

    None
}

async fn do_request_stream(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    token_type: AccessTokenType,
) -> Result<StreamResp, LarkError> {
    retry_request(config, option, token_type, |bearer| async move {
        let resp = raw_send_stream(config, api_req, option, token_type, bearer.as_deref()).await?;
        classify_stream_response(config, resp).await
    })
    .await
}

async fn classify_stream_response(
    config: &Config,
    resp: StreamResp,
) -> Result<RequestAttempt<StreamResp>, LarkError> {
    if is_json_content_type(&resp.api_resp)
        && !resp.api_resp.raw_body.is_empty()
        && let Some(err) = json_code_error_retry(config, &resp.api_resp.raw_body).await
    {
        return Ok(RequestAttempt::Retry(err));
    }

    Ok(RequestAttempt::Done(resp))
}

enum RequestAttempt<T> {
    Done(T),
    Retry(LarkError),
}

async fn retry_request<T, F, Fut>(
    config: &Config,
    option: &RequestOption,
    token_type: AccessTokenType,
    mut send: F,
) -> Result<T, LarkError>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: Future<Output = Result<RequestAttempt<T>, LarkError>>,
{
    let max_retries = config.max_retries;
    let mut last_err = None;

    for _ in 0..max_retries {
        let bearer = resolve_bearer_token(config, option, token_type).await?;
        match send(bearer).await {
            Ok(RequestAttempt::Done(value)) => return Ok(value),
            Ok(RequestAttempt::Retry(err)) => {
                last_err = Some(err);
                continue;
            }
            Err(e) => handle_request_error(e, &mut last_err)?,
        }
    }

    Err(last_err.unwrap_or(LarkError::MaxRetries))
}

fn handle_request_error(err: LarkError, last_err: &mut Option<LarkError>) -> Result<(), LarkError> {
    match err {
        LarkError::DialFailed(msg) => {
            *last_err = Some(LarkError::DialFailed(msg));
            Ok(())
        }
        LarkError::ServerTimeout(msg) => {
            *last_err = Some(LarkError::ServerTimeout(msg));
            Ok(())
        }
        LarkError::RateLimited(_) => Err(LarkError::RateLimited(
            "server returned 429 Too Many Requests".to_string(),
        )),
        e => Err(e),
    }
}

async fn resolve_bearer_token(
    config: &Config,
    option: &RequestOption,
    token_type: AccessTokenType,
) -> Result<Option<String>, LarkError> {
    token_resolver(config)
        .resolve(config, option, token_type)
        .await
}

enum TokenResolver {
    ClientAssertion,
    Direct,
    Cached,
}

fn token_resolver(config: &Config) -> TokenResolver {
    if config.client_assertion_provider.is_some() {
        TokenResolver::ClientAssertion
    } else if !config.enable_token_cache {
        TokenResolver::Direct
    } else {
        TokenResolver::Cached
    }
}

impl TokenResolver {
    async fn resolve(
        self,
        config: &Config,
        option: &RequestOption,
        token_type: AccessTokenType,
    ) -> Result<Option<String>, LarkError> {
        match self {
            Self::ClientAssertion => {
                resolve_client_assertion_token(config, option, token_type).await
            }
            Self::Direct => resolve_direct_token(option, token_type),
            Self::Cached => resolve_cached_token(config, option, token_type).await,
        }
    }
}

async fn resolve_client_assertion_token(
    config: &Config,
    option: &RequestOption,
    token_type: AccessTokenType,
) -> Result<Option<String>, LarkError> {
    match token_type {
        AccessTokenType::User => Ok(option.user_access_token.clone()),
        AccessTokenType::Tenant => {
            let tm = TokenManager::new(config.token_cache.clone());
            let token = tm
                .get_tenant_access_token(config, option.tenant_key.as_deref(), None)
                .await?;
            Ok(Some(token))
        }
        AccessTokenType::None => Ok(None),
        AccessTokenType::App => Err(LarkError::ClientAssertion(
            "AppAccessToken APIs are not available in ClientAssertion mode".to_string(),
        )),
    }
}

fn resolve_direct_token(
    option: &RequestOption,
    token_type: AccessTokenType,
) -> Result<Option<String>, LarkError> {
    match token_type {
        AccessTokenType::User => Ok(option.user_access_token.clone()),
        AccessTokenType::Tenant => Ok(option.tenant_access_token.clone()),
        AccessTokenType::App => Ok(option.app_access_token.clone()),
        AccessTokenType::None => Ok(None),
    }
}

async fn resolve_cached_token(
    config: &Config,
    option: &RequestOption,
    token_type: AccessTokenType,
) -> Result<Option<String>, LarkError> {
    match token_type {
        AccessTokenType::User => Ok(option.user_access_token.clone()),
        AccessTokenType::App => {
            let tm = TokenManager::new(config.token_cache.clone());
            let token = tm
                .get_app_access_token(config, option.app_ticket.as_deref())
                .await?;
            Ok(Some(token))
        }
        AccessTokenType::Tenant => {
            let tm = TokenManager::new(config.token_cache.clone());
            let token = tm
                .get_tenant_access_token(
                    config,
                    option.tenant_key.as_deref(),
                    option.app_ticket.as_deref(),
                )
                .await?;
            Ok(Some(token))
        }
        AccessTokenType::None => Ok(None),
    }
}

pub(crate) async fn raw_send_absolute_url(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    bearer_token: Option<&str>,
) -> Result<ApiResp, LarkError> {
    raw_send_inner(config, api_req, option, bearer_token, true).await
}

pub(crate) async fn raw_send(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    _token_type: AccessTokenType,
    bearer_token: Option<&str>,
) -> Result<ApiResp, LarkError> {
    raw_send_inner(config, api_req, option, bearer_token, false).await
}

pub(crate) async fn raw_send_stream(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    _token_type: AccessTokenType,
    bearer_token: Option<&str>,
) -> Result<StreamResp, LarkError> {
    raw_send_stream_inner(config, api_req, option, bearer_token, false).await
}

async fn raw_send_inner(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    bearer_token: Option<&str>,
    absolute_url: bool,
) -> Result<ApiResp, LarkError> {
    let (full_url, response) =
        send_http_response(config, api_req, option, bearer_token, absolute_url).await?;
    let status_code = response.status().as_u16();
    check_status_errors(api_req, status_code, response.headers())?;

    let header = response.headers().clone();
    let raw_body = response.bytes().await.map_err(LarkError::Http)?.to_vec();

    log_buffered_response(config, &full_url, status_code, &header, &raw_body);

    Ok(ApiResp {
        status_code,
        header,
        raw_body,
    })
}

async fn raw_send_stream_inner(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    bearer_token: Option<&str>,
    absolute_url: bool,
) -> Result<StreamResp, LarkError> {
    let (full_url, response) =
        send_http_response(config, api_req, option, bearer_token, absolute_url).await?;
    let status_code = response.status().as_u16();
    check_status_errors(api_req, status_code, response.headers())?;

    let header = response.headers().clone();

    if !(200..300).contains(&status_code) && is_json_header(&header) {
        let raw_body = response.bytes().await.map_err(LarkError::Http)?.to_vec();
        log_buffered_response(config, &full_url, status_code, &header, &raw_body);

        return Ok(StreamResp {
            api_resp: ApiResp {
                status_code,
                header,
                raw_body: raw_body.clone(),
            },
            body: StreamBody::replay(bytes::Bytes::from(raw_body)),
        });
    }

    log_stream_response(config, &full_url, status_code);

    Ok(StreamResp {
        api_resp: ApiResp {
            status_code,
            header,
            raw_body: Vec::new(),
        },
        body: StreamBody::streaming(response.into_bytes_stream()),
    })
}

async fn send_http_response(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    bearer_token: Option<&str>,
    absolute_url: bool,
) -> Result<(String, aioduct::Response), LarkError> {
    let full_url = if absolute_url {
        api_req.api_path.clone()
    } else {
        build_url(config, api_req)
    };

    let mut builder = config
        .http_client
        .request(api_req.http_method.clone(), &full_url)
        .map_err(|e| LarkError::IllegalParam(format!("invalid url: {e}")))?;

    builder = builder.header(UA_HEADER, HeaderValue::from_static(USER_AGENT));

    if let Some(ref id) = option.request_id {
        builder = builder
            .header_str(CUSTOM_REQUEST_ID, id.as_str())
            .map_err(|e| LarkError::IllegalParam(format!("invalid request id header: {e}")))?;
    } else {
        let id = uuid::Uuid::new_v4().to_string();
        builder = builder
            .header_str(CUSTOM_REQUEST_ID, &id)
            .map_err(|e| LarkError::IllegalParam(format!("invalid request id header: {e}")))?;
    }

    if let Some(token) = bearer_token {
        builder = builder.bearer_auth(token);
    }

    if option.need_helpdesk_auth
        && let Some(ref auth_token) = config.helpdesk_auth_token
    {
        builder = builder
            .header_str("X-Lark-Helpdesk-Authorization", auth_token.as_str())
            .map_err(|e| LarkError::IllegalParam(format!("invalid helpdesk header: {e}")))?;
    }

    for (key, value) in &config.default_headers {
        builder = builder.header(key.clone(), value.clone());
    }
    if let Some(ref headers) = option.headers {
        for (key, value) in headers {
            builder = builder.header(key.clone(), value.clone());
        }
    }

    match &api_req.body {
        Some(ReqBody::Json(value)) => {
            builder = builder.json(value).map_err(|e| {
                LarkError::IllegalParam(format!("failed to serialize json body: {e}"))
            })?;
        }
        Some(ReqBody::FormData(fields)) => {
            let mut form = aioduct::Multipart::new();
            for field in fields {
                match &field.value {
                    FormDataValue::Text(text) => {
                        form = form.text(field.name.clone(), text.clone());
                    }
                    FormDataValue::File {
                        filename,
                        data,
                        content_type,
                    } => {
                        let ct = content_type
                            .as_deref()
                            .unwrap_or("application/octet-stream");
                        form = form.file(field.name.clone(), filename.clone(), ct, data.clone());
                    }
                }
            }
            builder = builder.multipart(form);
        }
        None => {}
    }

    if config.log_req_at_debug {
        let sensitive_endpoint = is_sensitive_log_endpoint(&full_url);
        match &api_req.body {
            Some(ReqBody::Json(_)) if sensitive_endpoint => {
                tracing::debug!(
                    method = %api_req.http_method,
                    url = %full_url,
                    body = "<omitted sensitive token request>",
                    "lark.request"
                );
            }
            Some(ReqBody::Json(v)) => {
                let redacted = redact_json_for_logging(v);
                tracing::debug!(
                    method = %api_req.http_method,
                    url = %full_url,
                    body = %redacted,
                    "lark.request"
                );
            }
            Some(ReqBody::FormData(_)) => {
                tracing::debug!(
                    method = %api_req.http_method,
                    url = %full_url,
                    body = "<multipart>",
                    "lark.request"
                );
            }
            None => {
                tracing::debug!(
                    method = %api_req.http_method,
                    url = %full_url,
                    "lark.request"
                );
            }
        }
    }

    let response = builder.send().await.map_err(|e| {
        let url = e.url().clone();
        let err = e.into_error();
        match err {
            aioduct::Error::Timeout => {
                tracing::debug!(%url, "request timed out");
                LarkError::ClientTimeout("request timed out".to_string())
            }
            aioduct::Error::Io(ref io_err)
                if matches!(
                    io_err.kind(),
                    std::io::ErrorKind::ConnectionRefused
                        | std::io::ErrorKind::ConnectionReset
                        | std::io::ErrorKind::ConnectionAborted
                ) =>
            {
                LarkError::DialFailed(err.to_string())
            }
            other => LarkError::Http(other),
        }
    })?;

    Ok((full_url, response))
}

fn check_status_errors(
    api_req: &ApiReq,
    status_code: u16,
    headers: &http::HeaderMap,
) -> Result<(), LarkError> {
    if status_code == 504 {
        let log_id = headers
            .get("X-Tt-Logid")
            .or_else(|| headers.get("X-Request-Id"))
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        tracing::info!(
            path = %api_req.api_path,
            log_id = %log_id,
            "server timeout (504)"
        );
        return Err(LarkError::ServerTimeout(
            "server returned 504 Gateway Timeout".to_string(),
        ));
    }

    if status_code == 429 {
        let log_id = headers
            .get("X-Tt-Logid")
            .or_else(|| headers.get("X-Request-Id"))
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        tracing::info!(
            path = %api_req.api_path,
            log_id = %log_id,
            "rate limited (429)"
        );
        return Err(LarkError::RateLimited(
            "server returned 429 Too Many Requests".to_string(),
        ));
    }

    Ok(())
}

fn log_buffered_response(
    config: &Config,
    full_url: &str,
    status_code: u16,
    header: &http::HeaderMap,
    raw_body: &[u8],
) {
    let enabled = config
        .log_level
        .is_none_or(|lvl| lvl <= tracing::Level::DEBUG);
    if enabled {
        if config.log_req_at_debug {
            let content_type = header
                .get(CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            let body_str = response_body_for_debug(full_url, content_type, raw_body);
            tracing::debug!(
                status = status_code,
                url = %full_url,
                body = %body_str,
                "lark.response"
            );
        } else {
            tracing::debug!(status = status_code, url = %full_url, "lark.response");
        }
    }
}

fn log_stream_response(config: &Config, full_url: &str, status_code: u16) {
    let enabled = config
        .log_level
        .is_none_or(|lvl| lvl <= tracing::Level::DEBUG);
    if enabled {
        if config.log_req_at_debug {
            tracing::debug!(
                status = status_code,
                url = %full_url,
                body = "<streaming body>",
                "lark.response"
            );
        } else {
            tracing::debug!(status = status_code, url = %full_url, "lark.response");
        }
    }
}

fn response_body_for_debug<'a>(
    raw_url: &str,
    content_type: &str,
    raw_body: &'a [u8],
) -> std::borrow::Cow<'a, str> {
    if is_sensitive_log_endpoint(raw_url) {
        return "<omitted sensitive token response>".into();
    }

    let content_type = content_type.to_ascii_lowercase();
    if content_type.contains("json") {
        match serde_json::from_slice::<Value>(raw_body) {
            Ok(value) => redact_json_for_logging(&value).to_string().into(),
            Err(_) => format!("<unparseable json> len {}", raw_body.len()).into(),
        }
    } else if content_type.contains("text") {
        String::from_utf8_lossy(raw_body)
    } else {
        format!("<binary> len {}", raw_body.len()).into()
    }
}

fn is_sensitive_log_endpoint(raw_url: &str) -> bool {
    let path = url::Url::parse(raw_url)
        .ok()
        .map(|url| url.path().to_string())
        .unwrap_or_else(|| raw_url.split('?').next().unwrap_or(raw_url).to_string());

    matches!(
        path.as_str(),
        crate::constants::OAUTH_TOKEN_URL_PATH
            | crate::constants::APP_ACCESS_TOKEN_INTERNAL_URL_PATH
            | crate::constants::APP_ACCESS_TOKEN_URL_PATH
            | crate::constants::TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH
            | crate::constants::TENANT_ACCESS_TOKEN_URL_PATH
            | crate::constants::APPLY_APP_TICKET_PATH
    )
}

fn redact_json_for_logging(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    let value = if is_sensitive_json_key(key) {
                        Value::String("<redacted>".to_string())
                    } else {
                        redact_json_for_logging(value)
                    };
                    (key.clone(), value)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(redact_json_for_logging).collect()),
        _ => value.clone(),
    }
}

fn is_sensitive_json_key(key: &str) -> bool {
    let normalized: String = key
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    matches!(
        normalized.as_str(),
        "appsecret"
            | "appticket"
            | "clientassertion"
            | "accesstoken"
            | "refreshtoken"
            | "appaccesstoken"
            | "tenantaccesstoken"
    )
}

fn build_url(config: &Config, api_req: &ApiReq) -> String {
    const PATH_SEGMENT_ENCODE_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'%')
        .add(b'/')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'[')
        .add(b'\\')
        .add(b']')
        .add(b'^')
        .add(b'`')
        .add(b'{')
        .add(b'|')
        .add(b'}');

    let mut path = api_req.api_path.clone();

    for (key, value) in &api_req.path_params.0 {
        let placeholder = format!(":{key}");
        let encoded =
            percent_encoding::utf8_percent_encode(value, PATH_SEGMENT_ENCODE_SET).to_string();
        path = path.replace(&placeholder, &encoded);
    }

    let query = api_req.query_params.encode();

    let mut url = format!("{}{}", config.base_url, path);
    if !query.is_empty() {
        url.push('?');
        url.push_str(&query);
    }

    url
}

fn is_json_content_type(resp: &ApiResp) -> bool {
    is_json_header(&resp.header)
}

fn is_json_header(header: &http::HeaderMap) -> bool {
    header
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.contains("application/json"))
}

fn is_token_invalid_error(code: i64) -> bool {
    matches!(
        code,
        ERR_CODE_ACCESS_TOKEN_INVALID
            | ERR_CODE_APP_ACCESS_TOKEN_INVALID
            | ERR_CODE_TENANT_ACCESS_TOKEN_INVALID
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::req::ApiReq;

    fn test_config() -> Config {
        Config::new("test_id", "test_secret")
    }

    #[tokio::test]
    async fn build_url_simple() {
        let config = test_config();
        let api_req = ApiReq::new(http::Method::GET, "/open-apis/im/v1/messages");
        assert_eq!(
            build_url(&config, &api_req),
            "https://open.feishu.cn/open-apis/im/v1/messages"
        );
    }

    #[tokio::test]
    async fn build_url_with_query_params() {
        let config = test_config();
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/im/v1/messages");
        api_req.query_params.set("page_size", "20");
        api_req.query_params.set("page_token", "abc");
        let url = build_url(&config, &api_req);
        assert!(url.starts_with("https://open.feishu.cn/open-apis/im/v1/messages?"));
        assert!(url.contains("page_size=20"));
        assert!(url.contains("page_token=abc"));
    }

    #[tokio::test]
    async fn build_url_with_path_params() {
        let config = test_config();
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/im/v1/messages/:message_id");
        api_req.path_params.set("message_id", "om_123");
        assert_eq!(
            build_url(&config, &api_req),
            "https://open.feishu.cn/open-apis/im/v1/messages/om_123"
        );
    }

    #[tokio::test]
    async fn build_url_custom_base() {
        let mut config = test_config();
        config.base_url = "https://open.larksuite.com".to_string();
        let api_req = ApiReq::new(http::Method::GET, "/open-apis/auth/v3/app_access_token");
        assert_eq!(
            build_url(&config, &api_req),
            "https://open.larksuite.com/open-apis/auth/v3/app_access_token"
        );
    }

    #[tokio::test]
    async fn build_url_path_params_percent_encoded() {
        let config = test_config();
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/drive/v1/files/:file_token");
        api_req.path_params.set("file_token", "a/b?c#d e");
        let url = build_url(&config, &api_req);
        assert!(!url.contains("a/b?c#d e"));
        assert!(url.contains("a%2Fb%3Fc%23d%20e"));
    }

    #[test]
    fn detects_sensitive_token_log_endpoints() {
        assert!(is_sensitive_log_endpoint(
            "https://open.feishu.cn/oauth/v3/token"
        ));
        assert!(is_sensitive_log_endpoint(
            "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal?x=1"
        ));
        assert!(is_sensitive_log_endpoint(
            "/open-apis/auth/v3/tenant_access_token"
        ));
        assert!(is_sensitive_log_endpoint(
            "/open-apis/auth/v3/app_ticket/resend"
        ));
        assert!(!is_sensitive_log_endpoint(
            "https://open.feishu.cn/open-apis/im/v1/messages"
        ));
    }

    #[test]
    fn redacts_sensitive_json_fields_recursively() {
        let value = serde_json::json!({
            "app_secret": "s3cr3t-value",
            "clientAssertion": "assertion-value",
            "nested": {
                "access_token": "access-value",
                "refreshToken": "refresh-value",
                "ordinary": "kept"
            },
            "items": [
                { "tenant_access_token": "tenant-value" },
                { "name": "visible" }
            ]
        });

        let redacted = redact_json_for_logging(&value);
        let rendered = redacted.to_string();
        assert!(!rendered.contains("s3cr3t-value"));
        assert!(!rendered.contains("assertion-value"));
        assert!(!rendered.contains("access-value"));
        assert!(!rendered.contains("refresh-value"));
        assert!(!rendered.contains("tenant-value"));
        assert!(rendered.contains("kept"));
        assert!(rendered.contains("visible"));
        assert_eq!(redacted["app_secret"], "<redacted>");
        assert_eq!(redacted["nested"]["ordinary"], "kept");
    }

    #[test]
    fn malformed_json_response_debug_body_is_omitted() {
        let body = br#"{"access_token":"leaked-token""#;
        let rendered =
            response_body_for_debug("/open-apis/im/v1/messages", "application/json", body);

        assert_eq!(rendered, "<unparseable json> len 30");
        assert!(!rendered.contains("leaked-token"));
    }
}
