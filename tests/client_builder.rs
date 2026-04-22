use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::constants::AppType;
use std::time::Duration;

#[tokio::test]
async fn builder_defaults() {
    let client = Client::builder("app_id", "app_secret").build();
    let config = client.config();
    assert_eq!(config.app_id, "app_id");
    assert_eq!(config.app_secret, "app_secret");
    assert_eq!(config.app_type, AppType::SelfBuilt);
    assert!(config.enable_token_cache);
    assert_eq!(config.max_retries, 2);
}

#[tokio::test]
async fn builder_marketplace() {
    let client = Client::builder("app", "secret").marketplace().build();
    assert_eq!(client.config().app_type, AppType::Marketplace);
}

#[tokio::test]
async fn builder_base_url() {
    let client = Client::builder("app", "secret")
        .base_url("https://open.larksuite.com")
        .build();
    assert_eq!(client.config().base_url, "https://open.larksuite.com");
}

#[tokio::test]
async fn builder_disable_token_cache() {
    let client = Client::builder("app", "secret")
        .disable_token_cache()
        .build();
    assert!(!client.config().enable_token_cache);
}

#[tokio::test]
async fn builder_timeout() {
    let client = Client::builder("app", "secret")
        .timeout(Duration::from_secs(10))
        .build();
    assert_eq!(client.config().req_timeout, Duration::from_secs(10));
}

#[tokio::test]
async fn builder_helpdesk_credential() {
    let client = Client::builder("app", "secret")
        .helpdesk_credential("hd_id", "hd_token")
        .build();
    let config = client.config();
    assert_eq!(config.helpdesk_id.as_deref(), Some("hd_id"));
    assert_eq!(config.helpdesk_token.as_deref(), Some("hd_token"));
    assert!(config.helpdesk_auth_token.is_some());
    let decoded = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        config.helpdesk_auth_token.as_ref().unwrap(),
    )
    .unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hd_id:hd_token");
}

#[tokio::test]
async fn builder_skip_sign_verify() {
    let client = Client::builder("app", "secret").skip_sign_verify().build();
    assert!(client.config().skip_sign_verify);
}

#[tokio::test]
async fn builder_max_retries() {
    let client = Client::builder("app", "secret").max_retries(5).build();
    assert_eq!(client.config().max_retries, 5);
}

#[tokio::test]
async fn builder_log_level() {
    let client = Client::builder("app", "secret")
        .log_level(tracing::Level::TRACE)
        .build();
    assert_eq!(client.config().log_level, Some(tracing::Level::TRACE));
}

#[tokio::test]
async fn builder_default_headers() {
    use http::HeaderMap;
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", "value".parse().unwrap());
    let client = Client::builder("app", "secret")
        .default_headers(headers)
        .build();
    assert_eq!(
        client
            .config()
            .default_headers
            .get("X-Custom")
            .unwrap()
            .to_str()
            .unwrap(),
        "value"
    );
}

#[tokio::test]
async fn builder_custom_token_cache() {
    use larksuite_oapi_sdk_rs::cache::{Cache, LocalCache};
    use std::sync::Arc;

    let cache = Arc::new(LocalCache::new());
    cache
        .set("pre", "val", Duration::from_secs(60))
        .await
        .unwrap();

    let client = Client::builder("app", "secret")
        .token_cache(cache.clone())
        .build();

    let val = client.config().token_cache.get("pre").await.unwrap();
    assert_eq!(val.as_deref(), Some("val"));
}
