use std::sync::Arc;
use std::time::Duration;

use aioduct::runtime::TokioRuntime;
use http::HeaderMap;

use crate::cache::{Cache, LocalCache};
use crate::constants::{AppType, FEISHU_BASE_URL};

#[derive(Clone)]
pub struct Config {
    pub base_url: String,
    pub app_id: String,
    pub app_secret: String,
    pub helpdesk_id: Option<String>,
    pub helpdesk_token: Option<String>,
    pub helpdesk_auth_token: Option<String>,
    pub req_timeout: Duration,
    pub http_client: aioduct::Client<TokioRuntime>,
    pub app_type: AppType,
    pub enable_token_cache: bool,
    pub token_cache: Arc<dyn Cache>,
    pub default_headers: HeaderMap,
    pub skip_sign_verify: bool,
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
