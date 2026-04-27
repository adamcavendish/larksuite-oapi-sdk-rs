mod common;
use common::{http_response, mock_server};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::cache::LocalCache;
use larksuite_oapi_sdk_rs::error::LarkError;
use larksuite_oapi_sdk_rs::token::{AppTicketManager, TokenManager};
use std::sync::Arc;

fn marketplace_client(addr: std::net::SocketAddr) -> Client {
    Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .marketplace()
        .disable_token_cache()
        .build()
}

fn self_built_client(addr: std::net::SocketAddr) -> Client {
    Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
}

#[tokio::test]
async fn token_marketplace_requires_app_ticket() {
    let client = Client::builder("app_id", "secret")
        .marketplace()
        .disable_token_cache()
        .build();

    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let err = tm
        .get_app_access_token(client.config(), None)
        .await
        .unwrap_err();
    assert!(
        matches!(err, LarkError::Token(_)),
        "expected Token error, got {err:?}"
    );
}

#[tokio::test]
async fn token_marketplace_tenant_requires_app_ticket() {
    let client = Client::builder("app_id", "secret")
        .marketplace()
        .disable_token_cache()
        .build();

    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let err = tm
        .get_tenant_access_token(client.config(), None, None)
        .await
        .unwrap_err();
    assert!(
        matches!(err, LarkError::Token(_)),
        "expected Token error, got {err:?}"
    );
}

#[tokio::test]
async fn token_cache_hit_returns_cached_value() {
    use larksuite_oapi_sdk_rs::cache::Cache;

    let client = Client::builder("app_id", "secret").build();
    let cache = Arc::new(LocalCache::new());

    let cache_key = format!("app_access_token-{}", client.config().app_id());
    cache
        .set(
            &cache_key,
            "cached_token_abc",
            std::time::Duration::from_secs(3600),
        )
        .await
        .unwrap();

    let tm = TokenManager::new(cache);
    let token = tm
        .get_app_access_token(client.config(), None)
        .await
        .unwrap();
    assert_eq!(token, "cached_token_abc");
}

#[tokio::test]
async fn token_tenant_cache_hit_returns_cached_value() {
    use larksuite_oapi_sdk_rs::cache::Cache;

    let client = Client::builder("app_id", "secret").build();
    let cache = Arc::new(LocalCache::new());

    let cache_key = format!("tenant_access_token-{}-tenant_1", client.config().app_id());
    cache
        .set(
            &cache_key,
            "cached_tenant_token",
            std::time::Duration::from_secs(3600),
        )
        .await
        .unwrap();

    let tm = TokenManager::new(cache);
    let token = tm
        .get_tenant_access_token(client.config(), Some("tenant_1"), None)
        .await
        .unwrap();
    assert_eq!(token, "cached_tenant_token");
}

#[tokio::test]
async fn token_marketplace_app_token_fetch() {
    let body = r#"{"app_access_token":"mkt_app_token_123","expire":7200}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = marketplace_client(addr);
    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let token = tm
        .get_app_access_token(client.config(), Some("ticket_abc"))
        .await
        .unwrap();
    assert_eq!(token, "mkt_app_token_123");
}

#[tokio::test]
async fn token_marketplace_tenant_token_fetch() {
    let app_body = r#"{"app_access_token":"mkt_app_token","expire":7200}"#;
    let tenant_body = r#"{"tenant_access_token":"mkt_tenant_token","expire":7200}"#;
    let (addr, _h) = mock_server(vec![
        http_response(200, app_body),
        http_response(200, tenant_body),
    ])
    .await;

    let client = marketplace_client(addr);
    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let token = tm
        .get_tenant_access_token(client.config(), Some("t1"), Some("ticket_xyz"))
        .await
        .unwrap();
    assert_eq!(token, "mkt_tenant_token");
}

#[tokio::test]
async fn token_request_non_200_returns_error() {
    let (addr, _h) = mock_server(vec![http_response(500, r#"{"error":"internal"}"#)]).await;

    let client = self_built_client(addr);
    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let err = tm
        .get_app_access_token(client.config(), None)
        .await
        .unwrap_err();
    assert!(
        matches!(err, LarkError::Token(_)),
        "expected Token error, got {err:?}"
    );
}

#[tokio::test]
async fn app_ticket_manager_set_and_get() {
    use larksuite_oapi_sdk_rs::cache::Cache;

    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
    let atm = AppTicketManager::new(Arc::clone(&cache));

    atm.set(
        "test_app",
        "ticket_value",
        std::time::Duration::from_secs(600),
    )
    .await
    .unwrap();

    let key = "app_ticket-test_app";
    let stored = cache.get(key).await.unwrap();
    assert_eq!(stored.as_deref(), Some("ticket_value"));
}

#[tokio::test]
async fn app_ticket_manager_get_triggers_apply_when_missing() {
    use larksuite_oapi_sdk_rs::cache::Cache;

    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build();

    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
    let atm = AppTicketManager::new(Arc::clone(&cache));

    let result = atm.get(client.config()).await.unwrap();
    assert!(result.is_none());
}
