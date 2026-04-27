use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::config::Config;
use crate::constants::{
    APP_ACCESS_TOKEN_INTERNAL_URL_PATH, APP_ACCESS_TOKEN_KEY_PREFIX, APP_ACCESS_TOKEN_URL_PATH,
    APP_TICKET_KEY_PREFIX, APPLY_APP_TICKET_PATH, AccessTokenType, AppType, EXPIRY_DELTA_SECONDS,
    TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH, TENANT_ACCESS_TOKEN_KEY_PREFIX,
    TENANT_ACCESS_TOKEN_URL_PATH,
};
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

pub struct TokenManager {
    cache: Arc<dyn Cache>,
}

impl std::fmt::Debug for TokenManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenManager").finish_non_exhaustive()
    }
}

impl TokenManager {
    pub fn new(cache: Arc<dyn Cache>) -> Self {
        Self { cache }
    }

    pub async fn get_app_access_token(
        &self,
        config: &Config,
        app_ticket: Option<&str>,
    ) -> Result<String, LarkError> {
        let cache_key = format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{}", config.app_id);
        if let Some(token) = self.cache.get(&cache_key).await? {
            return Ok(token);
        }

        match config.app_type {
            AppType::SelfBuilt => self.fetch_self_built_app_token(config).await,
            AppType::Marketplace => {
                let ticket = app_ticket.ok_or_else(|| {
                    LarkError::Token("app ticket is required for marketplace apps".to_string())
                })?;
                self.fetch_marketplace_app_token(config, ticket).await
            }
        }
    }

    pub async fn get_tenant_access_token(
        &self,
        config: &Config,
        tenant_key: Option<&str>,
        app_ticket: Option<&str>,
    ) -> Result<String, LarkError> {
        let tk = tenant_key.unwrap_or_default();
        let cache_key = format!("{TENANT_ACCESS_TOKEN_KEY_PREFIX}-{}-{tk}", config.app_id);
        if let Some(token) = self.cache.get(&cache_key).await? {
            return Ok(token);
        }

        match config.app_type {
            AppType::SelfBuilt => self.fetch_self_built_tenant_token(config).await,
            AppType::Marketplace => {
                let ticket = app_ticket.ok_or_else(|| {
                    LarkError::Token("app ticket is required for marketplace apps".to_string())
                })?;
                self.fetch_marketplace_tenant_token(config, tenant_key, ticket)
                    .await
            }
        }
    }

    async fn fetch_self_built_app_token(&self, config: &Config) -> Result<String, LarkError> {
        let body = SelfBuiltTokenReq {
            app_id: &config.app_id,
            app_secret: &config.app_secret,
        };

        let resp = self
            .token_request::<AppAccessTokenResp>(config, APP_ACCESS_TOKEN_INTERNAL_URL_PATH, &body)
            .await?;

        let cache_key = format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{}", config.app_id);
        let ttl = Duration::from_secs(resp.expire.saturating_sub(EXPIRY_DELTA_SECONDS));
        if let Err(e) = self
            .cache
            .set(&cache_key, &resp.app_access_token, ttl)
            .await
        {
            tracing::warn!("app access token save cache: {e}");
        }

        Ok(resp.app_access_token)
    }

    async fn fetch_self_built_tenant_token(&self, config: &Config) -> Result<String, LarkError> {
        let body = SelfBuiltTokenReq {
            app_id: &config.app_id,
            app_secret: &config.app_secret,
        };

        let resp = self
            .token_request::<TenantAccessTokenResp>(
                config,
                TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH,
                &body,
            )
            .await?;

        let cache_key = format!("{TENANT_ACCESS_TOKEN_KEY_PREFIX}-{}-", config.app_id);
        let ttl = Duration::from_secs(resp.expire.saturating_sub(EXPIRY_DELTA_SECONDS));
        if let Err(e) = self
            .cache
            .set(&cache_key, &resp.tenant_access_token, ttl)
            .await
        {
            tracing::warn!("tenant access token save cache: {e}");
        }

        Ok(resp.tenant_access_token)
    }

    async fn fetch_marketplace_app_token(
        &self,
        config: &Config,
        app_ticket: &str,
    ) -> Result<String, LarkError> {
        let body = MarketplaceAppAccessTokenReq {
            app_id: &config.app_id,
            app_secret: &config.app_secret,
            app_ticket,
        };

        let resp = self
            .token_request::<AppAccessTokenResp>(config, APP_ACCESS_TOKEN_URL_PATH, &body)
            .await?;

        let cache_key = format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{}", config.app_id);
        let ttl = Duration::from_secs(resp.expire.saturating_sub(EXPIRY_DELTA_SECONDS));
        if let Err(e) = self
            .cache
            .set(&cache_key, &resp.app_access_token, ttl)
            .await
        {
            tracing::warn!("marketplace app access token save cache: {e}");
        }

        Ok(resp.app_access_token)
    }

    async fn fetch_marketplace_tenant_token(
        &self,
        config: &Config,
        tenant_key: Option<&str>,
        app_ticket: &str,
    ) -> Result<String, LarkError> {
        let app_access_token = self.get_app_access_token(config, Some(app_ticket)).await?;

        let body = MarketplaceTenantAccessTokenReq {
            app_access_token: &app_access_token,
            tenant_key: tenant_key.unwrap_or_default(),
        };

        let resp = self
            .token_request::<TenantAccessTokenResp>(config, TENANT_ACCESS_TOKEN_URL_PATH, &body)
            .await?;

        let tk = tenant_key.unwrap_or_default();
        let cache_key = format!("{TENANT_ACCESS_TOKEN_KEY_PREFIX}-{}-{tk}", config.app_id);
        let ttl = Duration::from_secs(resp.expire.saturating_sub(EXPIRY_DELTA_SECONDS));
        if let Err(e) = self
            .cache
            .set(&cache_key, &resp.tenant_access_token, ttl)
            .await
        {
            tracing::warn!("marketplace tenant access token save cache: {e}");
        }

        Ok(resp.tenant_access_token)
    }

    async fn token_request<T: for<'de> Deserialize<'de>>(
        &self,
        config: &Config,
        path: &str,
        body: &impl Serialize,
    ) -> Result<T, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.body = Some(ReqBody::Json(serde_json::to_value(body)?));
        api_req.supported_access_token_types = vec![AccessTokenType::None];

        let option = RequestOption::default();
        let api_resp =
            transport::raw_send(config, &api_req, &option, AccessTokenType::None, None).await?;

        if api_resp.status_code != 200 {
            return Err(LarkError::Token(format!(
                "token request failed with status {}",
                api_resp.status_code
            )));
        }

        let resp: T = serde_json::from_slice(&api_resp.raw_body)?;
        Ok(resp)
    }
}

pub struct AppTicketManager {
    cache: Arc<dyn Cache>,
}

impl std::fmt::Debug for AppTicketManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppTicketManager").finish_non_exhaustive()
    }
}

impl AppTicketManager {
    pub fn new(cache: Arc<dyn Cache>) -> Self {
        Self { cache }
    }

    pub async fn get(&self, config: &Config) -> Result<Option<String>, LarkError> {
        let key = format!("{APP_TICKET_KEY_PREFIX}-{}", config.app_id);
        let ticket = self.cache.get(&key).await?;

        if ticket.is_none() {
            self.apply_app_ticket(config).await?;
        }

        self.cache.get(&key).await
    }

    pub async fn set(&self, app_id: &str, value: &str, ttl: Duration) -> Result<(), LarkError> {
        let key = format!("{APP_TICKET_KEY_PREFIX}-{app_id}");
        self.cache.set(&key, value, ttl).await
    }

    pub(crate) async fn apply_app_ticket(&self, config: &Config) -> Result<(), LarkError> {
        let body = ResendAppTicketReq {
            app_id: &config.app_id,
            app_secret: &config.app_secret,
        };

        let mut api_req = ApiReq::new(http::Method::POST, APPLY_APP_TICKET_PATH);
        api_req.body = Some(ReqBody::Json(serde_json::to_value(&body)?));
        api_req.supported_access_token_types = vec![AccessTokenType::None];

        let option = RequestOption::default();
        transport::raw_send(config, &api_req, &option, AccessTokenType::None, None).await?;

        Ok(())
    }
}

#[derive(Serialize)]
struct SelfBuiltTokenReq<'a> {
    app_id: &'a str,
    app_secret: &'a str,
}

#[derive(Serialize)]
struct MarketplaceAppAccessTokenReq<'a> {
    app_id: &'a str,
    app_secret: &'a str,
    app_ticket: &'a str,
}

#[derive(Serialize)]
struct MarketplaceTenantAccessTokenReq<'a> {
    app_access_token: &'a str,
    tenant_key: &'a str,
}

#[derive(Deserialize)]
struct AppAccessTokenResp {
    #[serde(default)]
    expire: u64,
    #[serde(default)]
    app_access_token: String,
}

#[derive(Deserialize)]
struct TenantAccessTokenResp {
    #[serde(default)]
    expire: u64,
    #[serde(default)]
    tenant_access_token: String,
}

#[derive(Serialize)]
struct ResendAppTicketReq<'a> {
    app_id: &'a str,
    app_secret: &'a str,
}

// ── Public token endpoint types (matching Go SDK) ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfBuiltAppTokenReq {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceAppTokenReq {
    pub app_id: String,
    pub app_secret: String,
    pub app_ticket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfBuiltTenantTokenReq {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceTenantTokenReq {
    pub app_access_token: String,
    pub tenant_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendAppTicketRequest {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AppTokenResponse {
    #[serde(default)]
    pub code: i64,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub expire: u64,
    #[serde(default)]
    pub app_access_token: String,
}

impl AppTokenResponse {
    pub fn success(&self) -> bool {
        self.code == 0
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TenantTokenResponse {
    #[serde(default)]
    pub code: i64,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub expire: u64,
    #[serde(default)]
    pub tenant_access_token: String,
}

impl TenantTokenResponse {
    pub fn success(&self) -> bool {
        self.code == 0
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ResendAppTicketResponse {
    #[serde(default)]
    pub code: i64,
    #[serde(default)]
    pub msg: String,
}

impl ResendAppTicketResponse {
    pub fn success(&self) -> bool {
        self.code == 0
    }
}

pub(crate) async fn token_endpoint<Req: Serialize, Resp: for<'de> Deserialize<'de>>(
    config: &Config,
    path: &str,
    body: &Req,
) -> Result<(crate::resp::ApiResp, Resp), LarkError> {
    let mut api_req = ApiReq::new(http::Method::POST, path);
    api_req.body = Some(ReqBody::Json(serde_json::to_value(body)?));
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let option = RequestOption::default();
    let api_resp =
        transport::raw_send(config, &api_req, &option, AccessTokenType::None, None).await?;

    let resp: Resp = serde_json::from_slice(&api_resp.raw_body)?;
    Ok((api_resp, resp))
}
