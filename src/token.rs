use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::config::Config;
use crate::constants::*;
use crate::error::{Error, Result};
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

pub struct TokenManager {
    cache: Arc<dyn Cache>,
}

impl TokenManager {
    pub fn new(cache: Arc<dyn Cache>) -> Self {
        Self { cache }
    }

    pub async fn get_app_access_token(
        &self,
        config: &Config,
        app_ticket: Option<&str>,
    ) -> Result<String> {
        let cache_key = format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{}", config.app_id);
        if let Some(token) = self.cache.get(&cache_key).await? {
            return Ok(token);
        }

        match config.app_type {
            AppType::SelfBuilt => self.fetch_self_built_app_token(config).await,
            AppType::Marketplace => {
                let ticket = app_ticket.ok_or_else(|| {
                    Error::Token("app ticket is required for marketplace apps".to_string())
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
    ) -> Result<String> {
        let tk = tenant_key.unwrap_or_default();
        let cache_key = format!("{TENANT_ACCESS_TOKEN_KEY_PREFIX}-{}-{tk}", config.app_id);
        if let Some(token) = self.cache.get(&cache_key).await? {
            return Ok(token);
        }

        match config.app_type {
            AppType::SelfBuilt => self.fetch_self_built_tenant_token(config).await,
            AppType::Marketplace => {
                let ticket = app_ticket.ok_or_else(|| {
                    Error::Token("app ticket is required for marketplace apps".to_string())
                })?;
                self.fetch_marketplace_tenant_token(config, tenant_key, ticket)
                    .await
            }
        }
    }

    async fn fetch_self_built_app_token(&self, config: &Config) -> Result<String> {
        let body = SelfBuiltAppAccessTokenReq {
            app_id: &config.app_id,
            app_secret: &config.app_secret,
        };

        let resp = self
            .token_request::<AppAccessTokenResp>(config, APP_ACCESS_TOKEN_INTERNAL_URL_PATH, &body)
            .await?;

        let cache_key = format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{}", config.app_id);
        let ttl = Duration::from_secs(resp.expire.saturating_sub(EXPIRY_DELTA_SECONDS));
        self.cache
            .set(&cache_key, &resp.app_access_token, ttl)
            .await?;

        Ok(resp.app_access_token)
    }

    async fn fetch_self_built_tenant_token(&self, config: &Config) -> Result<String> {
        let body = SelfBuiltTenantAccessTokenReq {
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
        self.cache
            .set(&cache_key, &resp.tenant_access_token, ttl)
            .await?;

        Ok(resp.tenant_access_token)
    }

    async fn fetch_marketplace_app_token(
        &self,
        config: &Config,
        app_ticket: &str,
    ) -> Result<String> {
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
        self.cache
            .set(&cache_key, &resp.app_access_token, ttl)
            .await?;

        Ok(resp.app_access_token)
    }

    async fn fetch_marketplace_tenant_token(
        &self,
        config: &Config,
        tenant_key: Option<&str>,
        app_ticket: &str,
    ) -> Result<String> {
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
        self.cache
            .set(&cache_key, &resp.tenant_access_token, ttl)
            .await?;

        Ok(resp.tenant_access_token)
    }

    async fn token_request<T: for<'de> Deserialize<'de>>(
        &self,
        config: &Config,
        path: &str,
        body: &impl Serialize,
    ) -> Result<T> {
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.body = Some(ReqBody::Json(serde_json::to_value(body)?));
        api_req.supported_access_token_types = vec![AccessTokenType::None];

        let option = RequestOption::default();
        let api_resp =
            transport::raw_send(config, &api_req, &option, AccessTokenType::None, None).await?;

        if api_resp.status_code != 200 {
            return Err(Error::Token(format!(
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

impl AppTicketManager {
    pub fn new(cache: Arc<dyn Cache>) -> Self {
        Self { cache }
    }

    pub async fn get(&self, config: &Config) -> Result<Option<String>> {
        let key = format!("{APP_TICKET_KEY_PREFIX}-{}", config.app_id);
        let ticket = self.cache.get(&key).await?;

        if ticket.is_none() {
            self.apply_app_ticket(config).await?;
        }

        self.cache.get(&key).await
    }

    pub async fn set(&self, app_id: &str, value: &str, ttl: Duration) -> Result<()> {
        let key = format!("{APP_TICKET_KEY_PREFIX}-{app_id}");
        self.cache.set(&key, value, ttl).await
    }

    pub(crate) async fn apply_app_ticket(&self, config: &Config) -> Result<()> {
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
struct SelfBuiltAppAccessTokenReq<'a> {
    app_id: &'a str,
    app_secret: &'a str,
}

#[derive(Serialize)]
struct SelfBuiltTenantAccessTokenReq<'a> {
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
