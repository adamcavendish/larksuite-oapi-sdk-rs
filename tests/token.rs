use larksuite_oapi_sdk_rs::Config;
use larksuite_oapi_sdk_rs::cache::LocalCache;
use larksuite_oapi_sdk_rs::error::Error;
use std::sync::Arc;

#[tokio::test]
async fn token_marketplace_requires_app_ticket() {
    use larksuite_oapi_sdk_rs::token::TokenManager;

    let config = {
        let mut c = Config::new("app_id", "secret");
        c.app_type = larksuite_oapi_sdk_rs::constants::AppType::Marketplace;
        // Disable token cache so it always fetches (but we're testing ticket validation)
        c.enable_token_cache = false;
        c
    };

    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    // Without an app_ticket, marketplace app should error before hitting network
    let err = tm.get_app_access_token(&config, None).await.unwrap_err();
    assert!(
        matches!(err, Error::Token(_)),
        "expected Token error, got {err:?}"
    );
}

#[tokio::test]
async fn token_marketplace_tenant_requires_app_ticket() {
    use larksuite_oapi_sdk_rs::token::TokenManager;

    let config = {
        let mut c = Config::new("app_id", "secret");
        c.app_type = larksuite_oapi_sdk_rs::constants::AppType::Marketplace;
        c.enable_token_cache = false;
        c
    };

    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let err = tm
        .get_tenant_access_token(&config, None, None)
        .await
        .unwrap_err();
    assert!(
        matches!(err, Error::Token(_)),
        "expected Token error, got {err:?}"
    );
}

#[tokio::test]
async fn token_cache_hit_returns_cached_value() {
    use larksuite_oapi_sdk_rs::cache::Cache;
    use larksuite_oapi_sdk_rs::token::TokenManager;

    let config = Config::new("app_id", "secret");
    let cache = Arc::new(LocalCache::new());

    // Pre-populate cache with a fake token
    let cache_key = format!("app_access_token-{}", config.app_id);
    cache
        .set(
            &cache_key,
            "cached_token_abc",
            std::time::Duration::from_secs(3600),
        )
        .await
        .unwrap();

    let tm = TokenManager::new(cache);
    let token = tm.get_app_access_token(&config, None).await.unwrap();
    assert_eq!(token, "cached_token_abc");
}
