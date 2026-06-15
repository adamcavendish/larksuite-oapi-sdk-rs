use std::sync::Arc;
use std::time::Duration;

use http::HeaderMap;

use crate::cache::Cache;
use crate::config::Config;
use crate::constants::{AccessTokenType, AppType};
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, RawResponse};
use crate::service;
use crate::token::ClientAssertionProvider;
use crate::transport;

/// Builder for [`Client`]. Construct via [`Client::builder`].
#[must_use]
#[derive(Debug)]
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    pub fn marketplace(mut self) -> Self {
        self.config.app_type = AppType::Marketplace;
        self
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = url.into();
        self
    }

    pub fn disable_token_cache(mut self) -> Self {
        self.config.enable_token_cache = false;
        self
    }

    pub fn token_cache(mut self, cache: Arc<dyn Cache>) -> Self {
        self.config.token_cache = cache;
        self
    }

    pub fn oauth_base_url(mut self, url: impl Into<String>) -> Self {
        self.config.oauth_base_url = url.into();
        self
    }

    pub fn client_assertion_provider(mut self, provider: Arc<dyn ClientAssertionProvider>) -> Self {
        self.config.client_assertion_provider = Some(provider);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.req_timeout = timeout;
        self
    }

    pub fn helpdesk_credential(mut self, id: impl Into<String>, token: impl Into<String>) -> Self {
        let id = id.into();
        let token = token.into();
        let auth = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            format!("{id}:{token}"),
        );
        self.config.helpdesk_id = Some(id);
        self.config.helpdesk_token = Some(token);
        self.config.helpdesk_auth_token = Some(auth);
        self
    }

    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.config.default_headers = headers;
        self
    }

    pub fn skip_sign_verify(mut self) -> Self {
        self.config.skip_sign_verify = true;
        self
    }

    pub fn log_level(mut self, level: tracing::Level) -> Self {
        self.config.log_level = Some(level);
        self
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.config.max_retries = n;
        self
    }

    pub fn log_req_at_debug(mut self) -> Self {
        self.config.log_req_at_debug = true;
        self
    }

    pub fn build(self) -> Result<Client, LarkError> {
        let mut config = self.config;
        config.http_client = aioduct::TokioClient::builder()
            .tls(aioduct::tls::RustlsConnector::with_webpki_roots())
            .timeout(config.req_timeout)
            .build()?;
        let client = Client { config };
        client.resend_app_ticket_if_marketplace();
        Ok(client)
    }
}

/// Lark/Feishu API client. All service accessors borrow `&self` and are zero-cost wrappers.
#[derive(Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn builder(app_id: impl Into<String>, app_secret: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            config: Config::new(app_id, app_secret),
        }
    }

    fn resend_app_ticket_if_marketplace(&self) {
        use crate::constants::AppType;
        use crate::token::AppTicketManager;

        if self.config.app_type != AppType::Marketplace {
            return;
        }

        let config = self.config.clone();
        tokio::spawn(async move {
            let atm = AppTicketManager::new(config.token_cache.clone());
            if let Err(e) = atm.apply_app_ticket(&config).await {
                tracing::info!("resend app_ticket on startup: {e}");
            }
        });
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn app_ticket_manager(&self) -> crate::token::AppTicketManager {
        crate::token::AppTicketManager::new(self.config.token_cache.clone())
    }

    service_accessor!(admin, admin, v1, V1);
    service_accessor!(acs, acs, v1, V1);
    service_accessor!(aily, aily, v1, V1);
    service_accessor!(application, application, v1, V1);
    service_accessor!(application_v6, application, v6, V6);
    service_accessor!(attendance, attendance, v1, V1);
    service_accessor!(approval, approval, v4, V4);
    service_accessor!(auth, auth, v3, V3);
    service_accessor!(authen, authen, v1, V1);
    service_accessor!(bitable, bitable, v1, V1);
    service_accessor!(calendar, calendar, v4, V4);
    service_accessor!(contact, contact, v3, V3);
    service_accessor!(corehr, corehr, v1, V1);
    service_accessor!(corehr_v2, corehr, v2, V2);
    service_accessor!(docx, docx, v1, V1);
    service_accessor!(drive, drive, v1, V1);
    service_accessor!(drive_v2, drive, v2, V2);
    service_accessor!(helpdesk, helpdesk, v1, V1);
    service_accessor!(hire, hire, v1, V1);
    service_accessor!(hire_v2, hire, v2, V2);
    service_accessor!(im, im, v1, V1);
    service_accessor!(sheets, sheets, v3, V3);
    service_accessor!(task, task, v1, V1);
    service_accessor!(im_v2, im, v2, V2);
    service_accessor!(task_v2, task, v2, V2);
    service_accessor!(tenant, tenant, v2, V2);
    service_accessor!(baike, baike, v1, V1);
    service_accessor!(lingo, lingo, v1, V1);
    service_accessor!(mail, mail, v1, V1);
    service_accessor!(minutes, minutes, v1, V1);
    service_accessor!(okr, okr, v1, V1);
    service_accessor!(translation, translation, v1, V1);
    service_accessor!(search, search, v2, V2);
    service_accessor!(vc, vc, v1, V1);
    service_accessor!(
        wiki,
        wiki,
        v2,
        V2,
        deprecated = "use wiki_v2() for consistency — un-suffixed accessors return v1 elsewhere"
    );
    service_accessor!(wiki_v2, wiki, v2, V2);
    service_accessor!(wiki_v1, wiki, v1, V1);
    service_accessor!(passport, passport, v1, V1);
    service_accessor!(report, report, v1, V1);
    service_accessor!(workplace, workplace, v1, V1);
    service_accessor!(face_detection, face_detection, v1, V1);
    service_accessor!(human_authentication, human_authentication, v1, V1);
    service_accessor!(optical_char_recognition, optical_char_recognition, v1, V1);
    service_accessor!(speech_to_text, speech_to_text, v1, V1);
    service_accessor!(verification, verification, v1, V1);
    service_accessor!(document_ai, document_ai, v1, V1);
    service_accessor!(mdm, mdm, v1, V1);
    service_accessor!(mdm_v3, mdm, v3, V3);
    service_accessor!(personal_settings, personal_settings, v1, V1);
    service_accessor!(security_and_compliance, security_and_compliance, v1, V1);
    service_accessor!(security_and_compliance_v2, security_and_compliance, v2, V2);
    service_accessor!(moments, moments, v1, V1);
    service_accessor!(meeting_room, meeting_room, v1, V1);
    service_accessor!(ehr, ehr, v1, V1);
    service_accessor!(compensation, compensation, v1, V1);
    service_accessor!(payroll, payroll, v1, V1);
    service_accessor!(performance, performance, v1, V1);
    service_accessor!(performance_v2, performance, v2, V2);
    service_accessor!(directory, directory, v1, V1);
    service_accessor!(docs, docs, v1, V1);
    service_accessor!(apaas, apaas, v1, V1);
    service_accessor!(base_v2, base, v2, V2);
    service_accessor!(block, block, v1, V1);
    service_accessor!(block_v2, block, v2, V2);
    service_accessor!(board, board, v1, V1);
    service_accessor!(cardkit, cardkit, v1, V1);
    service_accessor!(event, event, v1, V1);
    pub fn ext(&self) -> service::ext::ExtService<'_> {
        service::ext::ExtService::new(&self.config)
    }

    /// Create a [`WsClient`](crate::ws::WsClient) that shares this client's token cache.
    #[cfg(feature = "ws")]
    pub fn ws_client(&self, dispatcher: crate::event::EventDispatcher) -> crate::ws::WsClient {
        crate::ws::WsClient::new(self.config.clone(), dispatcher)
    }

    pub async fn do_req(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        transport::request(&self.config, api_req, option).await
    }

    pub async fn do_req_typed<T: for<'de> serde::Deserialize<'de>>(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        transport::request_typed(&self.config, api_req, option).await
    }

    /// Execute a custom OpenAPI request with SDK auth and token handling.
    ///
    /// This is the raw escape hatch for endpoints that do not have a generated
    /// service wrapper yet. Prefer generated service methods when available.
    /// Set [`ApiReq::supported_access_token_types`] to the token accepted by
    /// the endpoint, or use [`Client::raw_request_with_token`] for the common
    /// single-token case.
    pub async fn raw_request(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        self.do_req(api_req, option).await
    }

    /// Execute a custom OpenAPI request with the given supported token type.
    pub async fn raw_request_with_token(
        &self,
        mut api_req: ApiReq,
        token_type: AccessTokenType,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        api_req.supported_access_token_types = vec![token_type];
        self.do_req(&api_req, option).await
    }

    /// Execute a custom OpenAPI request and deserialize its `data` field.
    ///
    /// Set [`ApiReq::supported_access_token_types`] to the token accepted by
    /// the endpoint, or use [`Client::raw_request_typed_with_token`] for the
    /// common single-token case.
    pub async fn raw_request_typed<T: for<'de> serde::Deserialize<'de>>(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        self.do_req_typed(api_req, option).await
    }

    /// Execute a custom OpenAPI request with the given supported token type and
    /// deserialize its `data` field.
    pub async fn raw_request_typed_with_token<T: for<'de> serde::Deserialize<'de>>(
        &self,
        mut api_req: ApiReq,
        token_type: AccessTokenType,
        option: &RequestOption,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        api_req.supported_access_token_types = vec![token_type];
        self.do_req_typed(&api_req, option).await
    }

    pub async fn get(&self, path: &str, option: &RequestOption) -> Result<ApiResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        self.do_req(&api_req, option).await
    }

    pub async fn post(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::Json(body));
        self.do_req(&api_req, option).await
    }

    pub async fn put(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::Json(body));
        self.do_req(&api_req, option).await
    }

    pub async fn patch(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::Json(body));
        self.do_req(&api_req, option).await
    }

    pub async fn delete(&self, path: &str, option: &RequestOption) -> Result<ApiResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        self.do_req(&api_req, option).await
    }

    pub async fn download_file(&self, url: &str) -> Result<Vec<u8>, LarkError> {
        let resp = self
            .config
            .http_client
            .request(http::Method::GET, url)?
            .send()
            .await
            .map_err(|e| LarkError::Http(e.into_error()))?;

        let status = resp.status();
        let bytes = resp.bytes().await?;

        if !status.is_success() {
            return Err(crate::LarkError::Http(aioduct::Error::Status(status)));
        }

        Ok(bytes.to_vec())
    }

    pub async fn get_app_access_token_by_self_built_app(
        &self,
        req: &crate::token::SelfBuiltAppTokenReq,
    ) -> Result<(ApiResp, crate::token::AppTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::APP_ACCESS_TOKEN_INTERNAL_URL_PATH,
            req,
        )
        .await
    }

    pub async fn get_app_access_token_by_marketplace_app(
        &self,
        req: &crate::token::MarketplaceAppTokenReq,
    ) -> Result<(ApiResp, crate::token::AppTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::APP_ACCESS_TOKEN_URL_PATH,
            req,
        )
        .await
    }

    pub async fn get_tenant_access_token_by_self_built_app(
        &self,
        req: &crate::token::SelfBuiltTenantTokenReq,
    ) -> Result<(ApiResp, crate::token::TenantTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH,
            req,
        )
        .await
    }

    pub async fn get_tenant_access_token_by_marketplace_app(
        &self,
        req: &crate::token::MarketplaceTenantTokenReq,
    ) -> Result<(ApiResp, crate::token::TenantTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::TENANT_ACCESS_TOKEN_URL_PATH,
            req,
        )
        .await
    }

    pub async fn resend_app_ticket(
        &self,
        req: &crate::token::ResendAppTicketRequest,
    ) -> Result<(ApiResp, crate::token::ResendAppTicketResponse), LarkError> {
        crate::token::token_endpoint(&self.config, crate::constants::APPLY_APP_TICKET_PATH, req)
            .await
    }
}
