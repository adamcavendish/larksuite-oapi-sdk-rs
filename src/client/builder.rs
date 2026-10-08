use std::sync::Arc;
use std::time::Duration;

use http::HeaderMap;

use super::LarkClient;
use crate::cache::Cache;
use crate::config::Config;
use crate::constants::AppType;
use crate::dpop::{DPoPClock, DPoPKey, DPoPKeyStore, DPoPMode};
use crate::error::LarkError;
use crate::token::ClientAssertionProvider;

/// Builder for [`LarkClient`]. Construct via [`LarkClient::builder`].
#[must_use]
#[derive(Debug)]
pub struct LarkClientBuilder {
    config: Config,
}

impl LarkClientBuilder {
    pub(crate) fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            config: Config::new(app_id, app_secret),
        }
    }

    pub fn marketplace(mut self) -> Self {
        self.config.app_type = AppType::Marketplace;
        self
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = url.into();
        self
    }

    /// Route SDK-owned platform requests through a trusted local resolver.
    ///
    /// Called before signing/sending API, OAuth, streaming requests, and
    /// WebSocket bootstrap/user-binding HTTP requests.
    /// Credentials go to the returned destination; external downloads and
    /// response links stay unchanged. DPoP requests do not follow redirects.
    /// Standalone registration uses `register_app_with_url_resolver` separately.
    pub fn platform_url_resolver(mut self, resolver: Arc<dyn crate::PlatformUrlResolver>) -> Self {
        self.config.platform_url_resolver = Some(resolver);
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

    pub fn dpop_mode(mut self, mode: DPoPMode) -> Self {
        self.config.dpop_mode = mode;
        self
    }

    pub fn dpop_key(mut self, key: DPoPKey) -> Self {
        self.config.dpop_key = Some(key);
        self
    }

    pub fn dpop_key_store(mut self, store: Arc<dyn DPoPKeyStore>) -> Self {
        self.config.dpop_key_store = store;
        self
    }

    pub fn dpop_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.config.dpop_key_id = key_id.into();
        self
    }

    pub fn dpop_clock(mut self, clock: Arc<dyn DPoPClock>) -> Self {
        self.config.dpop_clock = clock;
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

    pub fn build(self) -> Result<LarkClient, LarkError> {
        let mut config = self.config;
        config.http_client = aioduct::TokioClient::builder()
            .tls(aioduct::tls::RustlsConnector::with_webpki_roots())
            .timeout(config.req_timeout)
            .redirect_policy(if config.platform_url_resolver.is_some() {
                crate::url_resolver::redirect_policy()
            } else {
                aioduct::RedirectPolicy::default()
            })
            .sensitive_header(http::HeaderName::from_static("dpop"))
            .sensitive_header(http::HeaderName::from_static(
                "x-lark-helpdesk-authorization",
            ))
            .build()?;
        config.dpop_http_client = if config.platform_url_resolver.is_some() {
            aioduct::TokioClient::builder()
                .tls(aioduct::tls::RustlsConnector::with_webpki_roots())
                .timeout(config.req_timeout)
                .redirect_policy(aioduct::RedirectPolicy::none())
                .build()?
        } else {
            config.http_client.clone()
        };
        Ok(LarkClient { config })
    }
}
