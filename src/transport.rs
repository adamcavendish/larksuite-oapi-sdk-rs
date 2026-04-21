use http::header::{CONTENT_TYPE, HeaderValue, USER_AGENT as UA_HEADER};
use tracing::Instrument;

use crate::config::Config;
use crate::constants::*;
use crate::error::{Error, Result};
use crate::req::{ApiReq, FormDataValue, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::token::{AppTicketManager, TokenManager};

pub(crate) async fn request(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
) -> Result<ApiResp> {
    let span = tracing::info_span!(
        "lark.request",
        method = %api_req.http_method,
        path = %api_req.api_path,
    );

    let token_type = span.in_scope(|| {
        validate(config, api_req, option)?;
        Ok::<_, Error>(determine_token_type(config, api_req, option))
    })?;

    do_request(config, api_req, option, token_type)
        .instrument(span)
        .await
}

pub(crate) async fn request_typed<T: for<'de> serde::Deserialize<'de>>(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
) -> Result<(ApiResp, RawResponse<T>)> {
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
        return Err(Error::Api(Box::new(raw.code_error)));
    }
    Ok((resp, raw))
}

fn validate(config: &Config, api_req: &ApiReq, option: &RequestOption) -> Result<()> {
    if config.app_id.is_empty() {
        return Err(Error::IllegalParam("app_id is empty".to_string()));
    }
    if config.app_secret.is_empty() {
        return Err(Error::IllegalParam("app_secret is empty".to_string()));
    }

    if config.app_type == AppType::Marketplace {
        let needs_tenant_token = api_req
            .supported_access_token_types
            .contains(&AccessTokenType::Tenant);
        if needs_tenant_token && option.tenant_key.is_none() && config.enable_token_cache {
            return Err(Error::IllegalParam(
                "marketplace app requires tenant_key for tenant access token".to_string(),
            ));
        }
    }

    validate_token_type(&api_req.supported_access_token_types, option)?;

    Ok(())
}

fn validate_token_type(supported: &[AccessTokenType], option: &RequestOption) -> Result<()> {
    if supported.is_empty() || supported.contains(&AccessTokenType::None) {
        return Ok(());
    }

    if option.user_access_token.is_some() && !supported.contains(&AccessTokenType::User) {
        return Err(Error::IllegalParam(
            "user access token is not supported for this API".to_string(),
        ));
    }
    if option.tenant_access_token.is_some() && !supported.contains(&AccessTokenType::Tenant) {
        return Err(Error::IllegalParam(
            "tenant access token is not supported for this API".to_string(),
        ));
    }
    if option.app_access_token.is_some() && !supported.contains(&AccessTokenType::App) {
        return Err(Error::IllegalParam(
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
) -> Result<ApiResp> {
    let max_retries = 2;
    let mut last_err = None;

    for _ in 0..max_retries {
        let bearer = resolve_bearer_token(config, option, token_type).await?;
        match raw_send(config, api_req, option, token_type, bearer.as_deref()).await {
            Ok(resp) => {
                if is_json_content_type(&resp)
                    && let Ok(code_err) = serde_json::from_slice::<CodeError>(&resp.raw_body)
                {
                    if code_err.code == ERR_CODE_APP_TICKET_INVALID {
                        let atm = AppTicketManager::new(config.token_cache.clone());
                        let _ = atm.apply_app_ticket(config).await;
                    }
                    if is_token_invalid_error(code_err.code) {
                        last_err = Some(Error::Api(Box::new(code_err)));
                        continue;
                    }
                }
                return Ok(resp);
            }
            Err(Error::DialFailed(msg)) => {
                last_err = Some(Error::DialFailed(msg));
                continue;
            }
            Err(Error::ServerTimeout(msg)) => {
                last_err = Some(Error::ServerTimeout(msg));
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Err(last_err.unwrap_or(Error::MaxRetries))
}

async fn resolve_bearer_token(
    config: &Config,
    option: &RequestOption,
    token_type: AccessTokenType,
) -> Result<Option<String>> {
    if !config.enable_token_cache {
        return match token_type {
            AccessTokenType::User => Ok(option.user_access_token.clone()),
            AccessTokenType::Tenant => Ok(option.tenant_access_token.clone()),
            AccessTokenType::App => Ok(option.app_access_token.clone()),
            AccessTokenType::None => Ok(None),
        };
    }

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

pub(crate) async fn raw_send(
    config: &Config,
    api_req: &ApiReq,
    option: &RequestOption,
    _token_type: AccessTokenType,
    bearer_token: Option<&str>,
) -> Result<ApiResp> {
    let full_url = build_url(config, api_req);

    let mut builder = config
        .http_client
        .request(api_req.http_method.clone(), &full_url)
        .map_err(|e| Error::IllegalParam(format!("invalid url: {e}")))?;

    builder = builder.header(UA_HEADER, HeaderValue::from_static(USER_AGENT));

    if let Some(ref id) = option.request_id {
        builder = builder
            .header_str(CUSTOM_REQUEST_ID, id.as_str())
            .map_err(|e| Error::IllegalParam(format!("invalid request id header: {e}")))?;
    } else {
        let id = uuid::Uuid::new_v4().to_string();
        builder = builder
            .header_str(CUSTOM_REQUEST_ID, &id)
            .map_err(|e| Error::IllegalParam(format!("invalid request id header: {e}")))?;
    }

    if let Some(token) = bearer_token {
        builder = builder.bearer_auth(token);
    }

    if option.need_helpdesk_auth
        && let Some(ref auth_token) = config.helpdesk_auth_token
    {
        builder = builder
            .header_str("X-Lark-Helpdesk-Authorization", auth_token.as_str())
            .map_err(|e| Error::IllegalParam(format!("invalid helpdesk header: {e}")))?;
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
            builder = builder
                .json(value)
                .map_err(|e| Error::IllegalParam(format!("failed to serialize json body: {e}")))?;
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

    let response = builder.send().await.map_err(|e| match e {
        aioduct::Error::Timeout => Error::ClientTimeout("request timed out".to_string()),
        aioduct::Error::Io(ref io_err)
            if matches!(
                io_err.kind(),
                std::io::ErrorKind::ConnectionRefused
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
            ) =>
        {
            Error::DialFailed(e.to_string())
        }
        other => Error::Http(other),
    })?;

    let status_code = response.status().as_u16();

    let enabled = config
        .log_level
        .is_none_or(|lvl| lvl <= tracing::Level::DEBUG);
    if enabled {
        tracing::debug!(status = status_code, url = %full_url, "lark.response");
    }

    if status_code == 504 {
        return Err(Error::ServerTimeout(
            "server returned 504 Gateway Timeout".to_string(),
        ));
    }

    let header = response.headers().clone();
    let raw_body = response.bytes().await.map_err(Error::Http)?.to_vec();

    Ok(ApiResp {
        status_code,
        header,
        raw_body,
    })
}

fn build_url(config: &Config, api_req: &ApiReq) -> String {
    let mut path = api_req.api_path.clone();

    for (key, value) in &api_req.path_params.0 {
        let placeholder = format!(":{key}");
        path = path.replace(&placeholder, value);
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
    resp.header
        .get(CONTENT_TYPE.as_str())
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
