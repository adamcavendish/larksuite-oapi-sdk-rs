use std::sync::Arc;
use std::sync::Once;
use std::time::Duration;

use http::HeaderMap;

use crate::cache::{Cache, LocalCache};
use crate::constants::{AppType, FEISHU_BASE_URL};
use crate::dpop::{
    DPoPClock, DPoPKey, DPoPKeyStore, DPoPMode, MemoryDPoPKeyStore, SystemDPoPClock,
};
use crate::token::ClientAssertionProvider;

static CRYPTO_PROVIDER_INIT: Once = Once::new();

/// Install the rustls ring crypto provider as process-wide default. Safe to
/// call multiple times; only the first call wins. Required because
/// `rustls::ClientConfig::builder_with_protocol_versions` (used by
/// `aioduct::tls::RustlsConnector::with_webpki_roots`) needs a crypto
/// provider installed, and aioduct does not install one on the library path.
pub(crate) fn install_default_crypto_provider() {
    CRYPTO_PROVIDER_INIT.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

/// SDK configuration. Construct via [`crate::LarkClientBuilder`].
#[derive(Clone)]
pub struct Config {
    pub(crate) base_url: String,
    pub(crate) oauth_base_url: String,
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
    pub(crate) helpdesk_id: Option<String>,
    pub(crate) helpdesk_token: Option<String>,
    pub(crate) helpdesk_auth_token: Option<String>,
    pub(crate) req_timeout: Duration,
    pub(crate) http_client: aioduct::TokioClient,
    pub(crate) dpop_http_client: aioduct::TokioClient,
    pub(crate) platform_url_resolver: Option<Arc<dyn crate::PlatformUrlResolver>>,
    pub(crate) app_type: AppType,
    pub(crate) enable_token_cache: bool,
    pub(crate) token_cache: Arc<dyn Cache>,
    pub(crate) default_headers: HeaderMap,
    pub(crate) skip_sign_verify: bool,
    pub(crate) max_retries: u32,
    pub(crate) log_level: Option<tracing::Level>,
    pub(crate) log_req_at_debug: bool,
    pub(crate) client_assertion_provider: Option<Arc<dyn ClientAssertionProvider>>,
    pub(crate) dpop_mode: DPoPMode,
    pub(crate) dpop_key: Option<DPoPKey>,
    pub(crate) dpop_clock: Arc<dyn DPoPClock>,
    pub(crate) dpop_key_store: Arc<dyn DPoPKeyStore>,
    pub(crate) dpop_key_id: String,
}

impl Config {
    pub(crate) fn resolved_dpop_key(&self) -> Result<Option<DPoPKey>, crate::LarkError> {
        if self.dpop_mode == DPoPMode::Disabled {
            return Ok(None);
        }
        if let Some(key) = &self.dpop_key {
            return Ok(Some(key.clone()));
        }
        self.dpop_key_store
            .load_or_generate(&self.dpop_key_id)
            .map(Some)
            .map_err(|e| crate::LarkError::DPoPBinding(e.to_string()))
    }

    pub(crate) fn rotate_dpop_key(&self) -> Result<(), crate::LarkError> {
        if self.dpop_key.is_some() {
            return Err(crate::LarkError::DPoPBinding(
                "cannot rotate an explicitly configured DPoP key; use a key store".into(),
            ));
        }
        self.dpop_key_store
            .remove(&self.dpop_key_id)
            .map_err(|e| crate::LarkError::DPoPBinding(e.to_string()))
    }
    pub(crate) fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        let timeout = Duration::from_secs(30);
        install_default_crypto_provider();
        let http_client = aioduct::TokioClient::new();
        Self {
            base_url: FEISHU_BASE_URL.to_string(),
            oauth_base_url: FEISHU_BASE_URL.to_string(),
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            helpdesk_id: None,
            helpdesk_token: None,
            helpdesk_auth_token: None,
            req_timeout: timeout,
            dpop_http_client: http_client.clone(),
            http_client,
            platform_url_resolver: None,
            app_type: AppType::default(),
            enable_token_cache: true,
            token_cache: Arc::new(LocalCache::new()),
            default_headers: HeaderMap::new(),
            skip_sign_verify: false,
            max_retries: 2,
            log_level: None,
            log_req_at_debug: false,
            client_assertion_provider: None,
            dpop_mode: DPoPMode::Disabled,
            dpop_key: None,
            dpop_clock: Arc::new(SystemDPoPClock),
            dpop_key_store: Arc::new(MemoryDPoPKeyStore::new()),
            dpop_key_id: "default".into(),
        }
    }

    #[must_use]
    #[inline]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    #[must_use]
    #[inline]
    pub fn oauth_base_url(&self) -> &str {
        &self.oauth_base_url
    }

    #[must_use]
    #[inline]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    #[must_use]
    #[inline]
    pub fn app_secret(&self) -> &str {
        &self.app_secret
    }

    #[must_use]
    #[inline]
    pub fn helpdesk_id(&self) -> Option<&str> {
        self.helpdesk_id.as_deref()
    }

    #[must_use]
    #[inline]
    pub fn helpdesk_token(&self) -> Option<&str> {
        self.helpdesk_token.as_deref()
    }

    #[must_use]
    #[inline]
    pub fn helpdesk_auth_token(&self) -> Option<&str> {
        self.helpdesk_auth_token.as_deref()
    }

    #[must_use]
    #[inline]
    pub fn req_timeout(&self) -> Duration {
        self.req_timeout
    }

    #[must_use]
    #[inline]
    pub fn app_type(&self) -> AppType {
        self.app_type
    }

    #[must_use]
    #[inline]
    pub fn enable_token_cache(&self) -> bool {
        self.enable_token_cache
    }

    #[must_use]
    #[inline]
    pub fn token_cache(&self) -> &Arc<dyn Cache> {
        &self.token_cache
    }

    #[must_use]
    #[inline]
    pub fn default_headers(&self) -> &HeaderMap {
        &self.default_headers
    }

    #[must_use]
    #[inline]
    pub fn skip_sign_verify(&self) -> bool {
        self.skip_sign_verify
    }

    #[must_use]
    #[inline]
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    #[must_use]
    #[inline]
    pub fn log_level(&self) -> Option<tracing::Level> {
        self.log_level
    }

    #[must_use]
    #[inline]
    pub fn log_req_at_debug(&self) -> bool {
        self.log_req_at_debug
    }

    #[must_use]
    #[inline]
    pub fn client_assertion_provider(&self) -> Option<&Arc<dyn ClientAssertionProvider>> {
        self.client_assertion_provider.as_ref()
    }

    pub fn dpop_mode(&self) -> DPoPMode {
        self.dpop_mode
    }

    pub fn dpop_key(&self) -> Option<&DPoPKey> {
        self.dpop_key.as_ref()
    }

    pub fn dpop_clock(&self) -> &Arc<dyn DPoPClock> {
        &self.dpop_clock
    }

    pub fn dpop_key_store(&self) -> &Arc<dyn DPoPKeyStore> {
        &self.dpop_key_store
    }

    pub fn dpop_key_id(&self) -> &str {
        &self.dpop_key_id
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dpop_config_resolves_store_keys_and_rejects_explicit_rotation() {
        let mut config = Config::new("app", "secret");
        config.dpop_mode = DPoPMode::Required;
        let first = config.resolved_dpop_key().unwrap().unwrap();
        assert_eq!(
            config.resolved_dpop_key().unwrap().unwrap().thumbprint(),
            first.thumbprint()
        );
        config.dpop_key = Some(DPoPKey::generate());
        assert!(config.rotate_dpop_key().is_err());
    }
}
