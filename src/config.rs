use std::sync::Arc;
use std::time::Duration;

use aioduct::runtime::TokioRuntime;
use http::HeaderMap;

use crate::cache::{Cache, LocalCache};
use crate::constants::{AppType, FEISHU_BASE_URL};

/// SDK configuration. Prefer constructing via [`ClientBuilder`](crate::ClientBuilder).
#[derive(Clone)]
pub struct Config {
    /// Base URL for API requests (default: Feishu `https://open.feishu.cn`).
    pub base_url: String,
    pub app_id: String,
    pub app_secret: String,
    /// Helpdesk ID for helpdesk-authenticated endpoints.
    pub helpdesk_id: Option<String>,
    /// Helpdesk token for helpdesk-authenticated endpoints.
    pub helpdesk_token: Option<String>,
    /// Base64-encoded `{id}:{token}`, set automatically by [`ClientBuilder::helpdesk_credential`](crate::ClientBuilder::helpdesk_credential).
    pub helpdesk_auth_token: Option<String>,
    /// HTTP request timeout (default: 30s).
    pub req_timeout: Duration,
    pub http_client: aioduct::Client<TokioRuntime>,
    /// Self-built (default) or marketplace app.
    pub app_type: AppType,
    /// Whether to cache access tokens (default: true).
    pub enable_token_cache: bool,
    /// Shared token cache. Cloning `Config` shares the same cache via `Arc`.
    pub token_cache: Arc<dyn Cache>,
    /// Headers sent with every request.
    pub default_headers: HeaderMap,
    /// Skip event callback signature verification (for testing only).
    pub skip_sign_verify: bool,
    pub log_level: Option<tracing::Level>,
}

impl Config {
    pub fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        let timeout = Duration::from_secs(30);
        let http_client = aioduct::Client::builder().timeout(timeout).build();
        Self {
            base_url: FEISHU_BASE_URL.to_string(),
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            helpdesk_id: None,
            helpdesk_token: None,
            helpdesk_auth_token: None,
            req_timeout: timeout,
            http_client,
            app_type: AppType::default(),
            enable_token_cache: true,
            token_cache: Arc::new(LocalCache::new()),
            default_headers: HeaderMap::new(),
            skip_sign_verify: false,
            log_level: None,
        }
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("base_url", &self.base_url)
            .field("app_id", &self.app_id)
            .field("app_type", &self.app_type)
            .field("enable_token_cache", &self.enable_token_cache)
            .finish_non_exhaustive()
    }
}
