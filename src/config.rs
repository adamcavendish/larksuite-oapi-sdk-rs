use std::sync::Arc;
use std::time::Duration;

use aioduct::runtime::TokioRuntime;
use http::HeaderMap;

use crate::cache::{Cache, LocalCache};
use crate::constants::{AppType, FEISHU_BASE_URL};

/// SDK configuration. Construct via [`ClientBuilder`](crate::ClientBuilder).
#[derive(Clone)]
pub struct Config {
    pub(crate) base_url: String,
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
    pub(crate) helpdesk_id: Option<String>,
    pub(crate) helpdesk_token: Option<String>,
    pub(crate) helpdesk_auth_token: Option<String>,
    pub(crate) req_timeout: Duration,
    pub(crate) http_client: aioduct::Client<TokioRuntime>,
    pub(crate) app_type: AppType,
    pub(crate) enable_token_cache: bool,
    pub(crate) token_cache: Arc<dyn Cache>,
    pub(crate) default_headers: HeaderMap,
    pub(crate) skip_sign_verify: bool,
    pub(crate) max_retries: u32,
    pub(crate) log_level: Option<tracing::Level>,
    pub(crate) log_req_at_debug: bool,
}

impl Config {
    pub(crate) fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
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
            max_retries: 2,
            log_level: None,
            log_req_at_debug: false,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn app_secret(&self) -> &str {
        &self.app_secret
    }

    pub fn helpdesk_id(&self) -> Option<&str> {
        self.helpdesk_id.as_deref()
    }

    pub fn helpdesk_token(&self) -> Option<&str> {
        self.helpdesk_token.as_deref()
    }

    pub fn helpdesk_auth_token(&self) -> Option<&str> {
        self.helpdesk_auth_token.as_deref()
    }

    pub fn req_timeout(&self) -> Duration {
        self.req_timeout
    }

    pub fn app_type(&self) -> AppType {
        self.app_type
    }

    pub fn enable_token_cache(&self) -> bool {
        self.enable_token_cache
    }

    pub fn token_cache(&self) -> &Arc<dyn Cache> {
        &self.token_cache
    }

    pub fn default_headers(&self) -> &HeaderMap {
        &self.default_headers
    }

    pub fn skip_sign_verify(&self) -> bool {
        self.skip_sign_verify
    }

    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub fn log_level(&self) -> Option<tracing::Level> {
        self.log_level
    }

    pub fn log_req_at_debug(&self) -> bool {
        self.log_req_at_debug
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
