use std::sync::Arc;
use std::time::Duration;

use http::HeaderMap;

use crate::cache::Cache;
use crate::config::Config;
use crate::constants::AppType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, RawResponse};
use crate::service;
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

    pub fn build(self) -> Client {
        let mut config = self.config;
        config.http_client = aioduct::Client::builder()
            .tls(aioduct::tls::RustlsConnector::with_webpki_roots())
            .timeout(config.req_timeout)
            .build();
        let client = Client { config };
        client.resend_app_ticket_if_marketplace();
        client
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

    pub fn admin(&self) -> service::admin::v1::V1<'_> {
        service::admin::v1::V1::new(&self.config)
    }

    pub fn acs(&self) -> service::acs::v1::V1<'_> {
        service::acs::v1::V1::new(&self.config)
    }

    pub fn aily(&self) -> service::aily::v1::V1<'_> {
        service::aily::v1::V1::new(&self.config)
    }

    pub fn application(&self) -> service::application::v1::V1<'_> {
        service::application::v1::V1::new(&self.config)
    }

    pub fn application_v6(&self) -> service::application::v6::V6<'_> {
        service::application::v6::V6::new(&self.config)
    }

    pub fn attendance(&self) -> service::attendance::v1::V1<'_> {
        service::attendance::v1::V1::new(&self.config)
    }

    pub fn approval(&self) -> service::approval::v4::V4<'_> {
        service::approval::v4::V4::new(&self.config)
    }

    pub fn auth(&self) -> service::auth::v3::V3<'_> {
        service::auth::v3::V3::new(&self.config)
    }

    pub fn authen(&self) -> service::authen::v1::V1<'_> {
        service::authen::v1::V1::new(&self.config)
    }

    pub fn bitable(&self) -> service::bitable::v1::V1<'_> {
        service::bitable::v1::V1::new(&self.config)
    }

    pub fn calendar(&self) -> service::calendar::v4::V4<'_> {
        service::calendar::v4::V4::new(&self.config)
    }

    pub fn contact(&self) -> service::contact::v3::V3<'_> {
        service::contact::v3::V3::new(&self.config)
    }

    pub fn corehr(&self) -> service::corehr::v1::V1<'_> {
        service::corehr::v1::V1::new(&self.config)
    }

    pub fn corehr_v2(&self) -> service::corehr::v2::V2<'_> {
        service::corehr::v2::V2::new(&self.config)
    }

    pub fn docx(&self) -> service::docx::v1::V1<'_> {
        service::docx::v1::V1::new(&self.config)
    }

    pub fn drive(&self) -> service::drive::v1::V1<'_> {
        service::drive::v1::V1::new(&self.config)
    }

    pub fn drive_v2(&self) -> service::drive::v2::V2<'_> {
        service::drive::v2::V2::new(&self.config)
    }

    pub fn helpdesk(&self) -> service::helpdesk::v1::V1<'_> {
        service::helpdesk::v1::V1::new(&self.config)
    }

    pub fn hire(&self) -> service::hire::v1::V1<'_> {
        service::hire::v1::V1::new(&self.config)
    }

    pub fn hire_v2(&self) -> service::hire::v2::V2<'_> {
        service::hire::v2::V2::new(&self.config)
    }

    pub fn im(&self) -> service::im::v1::V1<'_> {
        service::im::v1::V1::new(&self.config)
    }

    pub fn sheets(&self) -> service::sheets::v3::V3<'_> {
        service::sheets::v3::V3::new(&self.config)
    }

    pub fn task(&self) -> service::task::v1::V1<'_> {
        service::task::v1::V1::new(&self.config)
    }

    pub fn im_v2(&self) -> service::im::v2::V2<'_> {
        service::im::v2::V2::new(&self.config)
    }

    pub fn task_v2(&self) -> service::task::v2::V2<'_> {
        service::task::v2::V2::new(&self.config)
    }

    pub fn tenant(&self) -> service::tenant::v2::V2<'_> {
        service::tenant::v2::V2::new(&self.config)
    }

    pub fn baike(&self) -> service::baike::v1::V1<'_> {
        service::baike::v1::V1::new(&self.config)
    }

    pub fn lingo(&self) -> service::lingo::v1::V1<'_> {
        service::lingo::v1::V1::new(&self.config)
    }

    pub fn mail(&self) -> service::mail::v1::V1<'_> {
        service::mail::v1::V1::new(&self.config)
    }

    pub fn minutes(&self) -> service::minutes::v1::V1<'_> {
        service::minutes::v1::V1::new(&self.config)
    }

    pub fn okr(&self) -> service::okr::v1::V1<'_> {
        service::okr::v1::V1::new(&self.config)
    }

    pub fn translation(&self) -> service::translation::v1::V1<'_> {
        service::translation::v1::V1::new(&self.config)
    }

    pub fn search(&self) -> service::search::v2::V2<'_> {
        service::search::v2::V2::new(&self.config)
    }

    pub fn vc(&self) -> service::vc::v1::V1<'_> {
        service::vc::v1::V1::new(&self.config)
    }

    #[deprecated(
        note = "use wiki_v2() for consistency — un-suffixed accessors return v1 elsewhere"
    )]
    pub fn wiki(&self) -> service::wiki::v2::V2<'_> {
        service::wiki::v2::V2::new(&self.config)
    }

    pub fn wiki_v2(&self) -> service::wiki::v2::V2<'_> {
        service::wiki::v2::V2::new(&self.config)
    }

    pub fn wiki_v1(&self) -> service::wiki::v1::V1<'_> {
        service::wiki::v1::V1::new(&self.config)
    }

    pub fn passport(&self) -> service::passport::v1::V1<'_> {
        service::passport::v1::V1::new(&self.config)
    }

    pub fn report(&self) -> service::report::v1::V1<'_> {
        service::report::v1::V1::new(&self.config)
    }

    pub fn workplace(&self) -> service::workplace::v1::V1<'_> {
        service::workplace::v1::V1::new(&self.config)
    }

    pub fn face_detection(&self) -> service::face_detection::v1::V1<'_> {
        service::face_detection::v1::V1::new(&self.config)
    }

    pub fn human_authentication(&self) -> service::human_authentication::v1::V1<'_> {
        service::human_authentication::v1::V1::new(&self.config)
    }

    pub fn optical_char_recognition(&self) -> service::optical_char_recognition::v1::V1<'_> {
        service::optical_char_recognition::v1::V1::new(&self.config)
    }

    pub fn speech_to_text(&self) -> service::speech_to_text::v1::V1<'_> {
        service::speech_to_text::v1::V1::new(&self.config)
    }

    pub fn verification(&self) -> service::verification::v1::V1<'_> {
        service::verification::v1::V1::new(&self.config)
    }

    pub fn document_ai(&self) -> service::document_ai::v1::V1<'_> {
        service::document_ai::v1::V1::new(&self.config)
    }

    pub fn mdm(&self) -> service::mdm::v1::V1<'_> {
        service::mdm::v1::V1::new(&self.config)
    }

    pub fn mdm_v3(&self) -> service::mdm::v3::V3<'_> {
        service::mdm::v3::V3::new(&self.config)
    }

    pub fn personal_settings(&self) -> service::personal_settings::v1::V1<'_> {
        service::personal_settings::v1::V1::new(&self.config)
    }

    pub fn security_and_compliance(&self) -> service::security_and_compliance::v1::V1<'_> {
        service::security_and_compliance::v1::V1::new(&self.config)
    }

    pub fn security_and_compliance_v2(&self) -> service::security_and_compliance::v2::V2<'_> {
        service::security_and_compliance::v2::V2::new(&self.config)
    }

    pub fn moments(&self) -> service::moments::v1::V1<'_> {
        service::moments::v1::V1::new(&self.config)
    }

    pub fn meeting_room(&self) -> service::meeting_room::v1::V1<'_> {
        service::meeting_room::v1::V1::new(&self.config)
    }

    pub fn ehr(&self) -> service::ehr::v1::V1<'_> {
        service::ehr::v1::V1::new(&self.config)
    }

    pub fn compensation(&self) -> service::compensation::v1::V1<'_> {
        service::compensation::v1::V1::new(&self.config)
    }

    pub fn payroll(&self) -> service::payroll::v1::V1<'_> {
        service::payroll::v1::V1::new(&self.config)
    }

    pub fn performance(&self) -> service::performance::v1::V1<'_> {
        service::performance::v1::V1::new(&self.config)
    }

    pub fn performance_v2(&self) -> service::performance::v2::V2<'_> {
        service::performance::v2::V2::new(&self.config)
    }

    pub fn directory(&self) -> service::directory::v1::V1<'_> {
        service::directory::v1::V1::new(&self.config)
    }

    pub fn docs(&self) -> service::docs::v1::V1<'_> {
        service::docs::v1::V1::new(&self.config)
    }

    pub fn apaas(&self) -> service::apaas::v1::V1<'_> {
        service::apaas::v1::V1::new(&self.config)
    }

    pub fn base_v2(&self) -> service::base::v2::V2<'_> {
        service::base::v2::V2::new(&self.config)
    }

    pub fn block(&self) -> service::block::v1::V1<'_> {
        service::block::v1::V1::new(&self.config)
    }

    pub fn block_v2(&self) -> service::block::v2::V2<'_> {
        service::block::v2::V2::new(&self.config)
    }

    pub fn board(&self) -> service::board::v1::V1<'_> {
        service::board::v1::V1::new(&self.config)
    }

    pub fn cardkit(&self) -> service::cardkit::v1::V1<'_> {
        service::cardkit::v1::V1::new(&self.config)
    }

    pub fn event(&self) -> service::event::v1::V1<'_> {
        service::event::v1::V1::new(&self.config)
    }

    pub fn ext(&self) -> service::ext::ExtService<'_> {
        service::ext::ExtService::new(&self.config)
    }

    /// Create a [`WsClient`](crate::ws::WsClient) that shares this client's token cache.
    #[cfg(feature = "ws")]
    pub fn ws_client(&self, dispatcher: crate::event::EventDispatcher) -> crate::ws::WsClient {
        crate::ws::WsClient::new(self.config.clone(), dispatcher)
    }

    pub async fn do_req(&self, api_req: &ApiReq, option: &RequestOption) -> Result<ApiResp> {
        transport::request(&self.config, api_req, option).await
    }

    pub async fn do_req_typed<T: for<'de> serde::Deserialize<'de>>(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<(ApiResp, RawResponse<T>)> {
        transport::request_typed(&self.config, api_req, option).await
    }

    pub async fn get(&self, path: &str, option: &RequestOption) -> Result<ApiResp> {
        let api_req = ApiReq::new(http::Method::GET, path);
        self.do_req(&api_req, option).await
    }

    pub async fn post(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp> {
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.body = Some(ReqBody::Json(body));
        self.do_req(&api_req, option).await
    }

    pub async fn put(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp> {
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.body = Some(ReqBody::Json(body));
        self.do_req(&api_req, option).await
    }

    pub async fn patch(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp> {
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.body = Some(ReqBody::Json(body));
        self.do_req(&api_req, option).await
    }

    pub async fn delete(&self, path: &str, option: &RequestOption) -> Result<ApiResp> {
        let api_req = ApiReq::new(http::Method::DELETE, path);
        self.do_req(&api_req, option).await
    }

    pub async fn download_file(&self, url: &str) -> Result<Vec<u8>> {
        let resp = self
            .config
            .http_client
            .request(http::Method::GET, url)?
            .send()
            .await?;

        let status = resp.status();
        let bytes = resp.bytes().await?;

        if !status.is_success() {
            return Err(crate::Error::Http(aioduct::Error::Status(status)));
        }

        Ok(bytes.to_vec())
    }

    pub async fn get_app_access_token_by_self_built_app(
        &self,
        req: &crate::token::SelfBuiltAppTokenReq,
    ) -> Result<(ApiResp, crate::token::AppTokenResponse)> {
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
    ) -> Result<(ApiResp, crate::token::AppTokenResponse)> {
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
    ) -> Result<(ApiResp, crate::token::TenantTokenResponse)> {
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
    ) -> Result<(ApiResp, crate::token::TenantTokenResponse)> {
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
    ) -> Result<(ApiResp, crate::token::ResendAppTicketResponse)> {
        crate::token::token_endpoint(&self.config, crate::constants::APPLY_APP_TICKET_PATH, req)
            .await
    }
}
