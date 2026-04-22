use larksuite_oapi_sdk_rs::Config;
use larksuite_oapi_sdk_rs::cache::LocalCache;
use larksuite_oapi_sdk_rs::error::Error;
use larksuite_oapi_sdk_rs::token::{AppTicketManager, TokenManager};
use std::sync::Arc;

async fn mock_server(
    responses: Vec<String>,
) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let responses = Arc::new(responses);
    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let handle = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let responses = Arc::clone(&responses);
            let counter = Arc::clone(&counter);
            tokio::spawn(async move {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};
                let mut buf = vec![0u8; 8192];
                let _ = stream.read(&mut buf).await;
                let idx = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let resp_idx = idx.min(responses.len() - 1);
                let _ = stream.write_all(responses[resp_idx].as_bytes()).await;
                let _ = stream.shutdown().await;
            });
        }
    });

    (addr, handle)
}

fn http_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )
}

fn marketplace_config(addr: std::net::SocketAddr) -> Config {
    let mut c = Config::new("app_id", "secret");
    c.base_url = format!("http://{addr}");
    c.app_type = larksuite_oapi_sdk_rs::constants::AppType::Marketplace;
    c.enable_token_cache = false;
    c
}

fn self_built_config(addr: std::net::SocketAddr) -> Config {
    let mut c = Config::new("app_id", "secret");
    c.base_url = format!("http://{addr}");
    c.enable_token_cache = false;
    c
}

#[tokio::test]
async fn token_marketplace_requires_app_ticket() {
    let config = {
        let mut c = Config::new("app_id", "secret");
        c.app_type = larksuite_oapi_sdk_rs::constants::AppType::Marketplace;
        c.enable_token_cache = false;
        c
    };

    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let err = tm.get_app_access_token(&config, None).await.unwrap_err();
    assert!(
        matches!(err, Error::Token(_)),
        "expected Token error, got {err:?}"
    );
}

#[tokio::test]
async fn token_marketplace_tenant_requires_app_ticket() {
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

    let config = Config::new("app_id", "secret");
    let cache = Arc::new(LocalCache::new());

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

#[tokio::test]
async fn token_tenant_cache_hit_returns_cached_value() {
    use larksuite_oapi_sdk_rs::cache::Cache;

    let config = Config::new("app_id", "secret");
    let cache = Arc::new(LocalCache::new());

    let cache_key = format!("tenant_access_token-{}-tenant_1", config.app_id);
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
        .get_tenant_access_token(&config, Some("tenant_1"), None)
        .await
        .unwrap();
    assert_eq!(token, "cached_tenant_token");
}

#[tokio::test]
async fn token_marketplace_app_token_fetch() {
    let body = r#"{"app_access_token":"mkt_app_token_123","expire":7200}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let config = marketplace_config(addr);
    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let token = tm
        .get_app_access_token(&config, Some("ticket_abc"))
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

    let config = marketplace_config(addr);
    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let token = tm
        .get_tenant_access_token(&config, Some("t1"), Some("ticket_xyz"))
        .await
        .unwrap();
    assert_eq!(token, "mkt_tenant_token");
}

#[tokio::test]
async fn token_request_non_200_returns_error() {
    let (addr, _h) = mock_server(vec![http_response(500, r#"{"error":"internal"}"#)]).await;

    let config = self_built_config(addr);
    let cache = Arc::new(LocalCache::new());
    let tm = TokenManager::new(cache);

    let err = tm.get_app_access_token(&config, None).await.unwrap_err();
    assert!(
        matches!(err, Error::Token(_)),
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

    let mut config = Config::new("app_id", "secret");
    config.base_url = format!("http://{addr}");

    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
    let atm = AppTicketManager::new(Arc::clone(&cache));

    let result = atm.get(&config).await.unwrap();
    assert!(result.is_none());
}
