use std::sync::Arc;
use std::time::Duration;

use aioduct::runtime::TokioRuntime;
use http::HeaderMap;

use crate::cache::Cache;
use crate::config::Config;
use crate::constants::AppType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, RawResponse};
use crate::service;
use crate::transport;

pub struct ClientBuilder {
    config: Config,
    explicit_http_client: bool,
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

    pub fn http_client(mut self, client: aioduct::Client<TokioRuntime>) -> Self {
        self.config.http_client = client;
        self.explicit_http_client = true;
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

    pub fn build(self) -> Client {
        let mut config = self.config;
        if !self.explicit_http_client {
            config.http_client = aioduct::Client::builder()
                .timeout(config.req_timeout)
                .build();
        }
        Client { config }
    }
}

pub struct Client {
    config: Config,
}

impl Client {
    pub fn builder(app_id: impl Into<String>, app_secret: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            config: Config::new(app_id, app_secret),
            explicit_http_client: false,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn admin(&self) -> service::admin::v1::V1<'_> {
        service::admin::v1::V1::new(&self.config)
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

    pub fn docx(&self) -> service::docx::v1::V1<'_> {
        service::docx::v1::V1::new(&self.config)
    }

    pub fn drive(&self) -> service::drive::v1::V1<'_> {
        service::drive::v1::V1::new(&self.config)
    }

    pub fn helpdesk(&self) -> service::helpdesk::v1::V1<'_> {
        service::helpdesk::v1::V1::new(&self.config)
    }

    pub fn hire(&self) -> service::hire::v1::V1<'_> {
        service::hire::v1::V1::new(&self.config)
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

    pub fn wiki(&self) -> service::wiki::v2::V2<'_> {
        service::wiki::v2::V2::new(&self.config)
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
}
