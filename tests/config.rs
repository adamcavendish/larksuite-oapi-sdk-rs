use std::time::Duration;

use larksuite_oapi_sdk_rs::Config;
use larksuite_oapi_sdk_rs::constants::{AppType, FEISHU_BASE_URL};

#[tokio::test]
async fn config_new_defaults() {
    let config = Config::new("myapp", "mysecret");
    assert_eq!(config.app_id, "myapp");
    assert_eq!(config.app_secret, "mysecret");
    assert_eq!(config.base_url, FEISHU_BASE_URL);
    assert_eq!(config.app_type, AppType::SelfBuilt);
    assert!(config.enable_token_cache);
    assert!(config.helpdesk_id.is_none());
    assert!(config.helpdesk_token.is_none());
    assert!(config.helpdesk_auth_token.is_none());
    assert!(!config.skip_sign_verify);
    assert_eq!(config.max_retries, 2);
    assert_eq!(config.req_timeout, Duration::from_secs(30));
    assert!(config.log_level.is_none());
    assert!(config.default_headers.is_empty());
}

#[tokio::test]
async fn config_debug_hides_secret() {
    let config = Config::new("myapp", "supersecret");
    let debug = format!("{config:?}");
    assert!(debug.contains("myapp"));
    assert!(!debug.contains("supersecret"));
}

#[tokio::test]
async fn config_clone_shares_cache() {
    let config = Config::new("app", "secret");
    let config2 = config.clone();

    config
        .token_cache
        .set("test_key", "test_val", Duration::from_secs(60))
        .await
        .unwrap();

    let val = config2.token_cache.get("test_key").await.unwrap();
    assert_eq!(val.as_deref(), Some("test_val"));
}
