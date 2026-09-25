mod common;
use common::{http_response, mock_server, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::cache::{Cache, LocalCache};
use larksuite_oapi_sdk_rs::error::LarkError;
use larksuite_oapi_sdk_rs::token::{
    AppTicketManager, ClientAssertionProvider, Token, TokenManager,
};
use std::sync::Arc;

#[derive(Debug)]
struct TestAssertion;

impl ClientAssertionProvider for TestAssertion {
    fn retrieve_token(
        &self,
        _aud: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Token, LarkError>> + Send + '_>>
    {
        Box::pin(async {
            Ok(Token {
                value: "signed-assertion".into(),
                target_info: None,
            })
        })
    }
}

#[tokio::test]
async fn client_assertion_rejects_policy_denial_even_with_token() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, r#"{"code":21001,"msg":"issuance denied","access_token":"denied","expires_in":7200,"data":{"challenge_url":"https://open.feishu.cn/challenge","cli_hint":"review policy"}}"#),
        http_response(200, r#"{"code":0,"access_token":"allowed","expires_in":7200,"status_message":"scopes trimmed"}"#),
        http_response(200, r#"{"code":0,"access_token":"other","expires_in":7200}"#),
    ]).await;
    let client = LarkClient::builder("app_id", "secret")
        .oauth_base_url(format!("http://{addr}"))
        .client_assertion_provider(Arc::new(TestAssertion))
        .build()
        .unwrap();
    let tm = TokenManager::new(Arc::new(LocalCache::new()));

    let err = tm
        .get_tenant_access_token(client.config(), None, None)
        .await
        .unwrap_err();
    assert!(matches!(err, LarkError::OAuthTokenRejected {
        code: 21001,
        ref message,
        challenge_url: Some(ref challenge_url),
        hint: Some(ref hint),
    } if message == "issuance denied"
        && challenge_url == "https://open.feishu.cn/challenge"
        && hint == "review policy"));
    let result = tm
        .get_client_assertion_tenant_token(client.config(), None)
        .await
        .unwrap();
    assert_eq!(result.access_token, "allowed");
    assert_eq!(result.status_message.as_deref(), Some("scopes trimmed"));
    let cached = tm
        .get_client_assertion_tenant_token(client.config(), None)
        .await
        .unwrap();
    assert_eq!(cached.access_token, "allowed");
    assert_eq!(cached.status_message, None);
    assert_eq!(
        tm.get_tenant_access_token(client.config(), None, None)
            .await
            .unwrap(),
        "allowed"
    );
    let without_advisory = tm
        .get_client_assertion_tenant_token(client.config(), Some("other-tenant"))
        .await
        .unwrap();
    assert_eq!(without_advisory.access_token, "other");
    assert_eq!(without_advisory.status_message, None);
    assert_eq!(requests.lock().unwrap().len(), 3);
}

#[tokio::test]
async fn client_assertion_preserves_non_200_policy_denial() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(
        403,
        r#"{"code":21000,"msg":"challenge required","data":{"challenge_url":"https://open.feishu.cn/challenge"}}"#,
    )])
    .await;
    let client = LarkClient::builder("app_id", "secret")
        .oauth_base_url(format!("http://{addr}"))
        .client_assertion_provider(Arc::new(TestAssertion))
        .build()
        .unwrap();
    let tm = TokenManager::new(Arc::new(LocalCache::new()));
    let err = tm
        .get_client_assertion_tenant_token(client.config(), None)
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        LarkError::OAuthTokenRejected {
            code: 21000,
            challenge_url: Some(_),
            ..
        }
    ));
    assert_eq!(requests.lock().unwrap().len(), 1);
}

fn marketplace_client(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .marketplace()
        .disable_token_cache()
        .build()
        .unwrap()
}

fn self_built_client(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn token_marketplace_requires_app_ticket() {
    let client = LarkClient::builder("app_id", "secret")
        .marketplace()
        .disable_token_cache()
        .build()
        .unwrap();

    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
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
    let client = LarkClient::builder("app_id", "secret")
        .marketplace()
        .disable_token_cache()
        .build()
        .unwrap();

    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
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
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"app_access_token":"cached_token_abc","expire":7200}"#,
    )])
    .await;
    let client = LarkClient::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
    let tm = TokenManager::new(Arc::clone(&cache));
    let first = tm
        .get_app_access_token(client.config(), None)
        .await
        .unwrap();
    let second = tm
        .get_app_access_token(client.config(), None)
        .await
        .unwrap();

    assert_eq!(first, "cached_token_abc");
    assert_eq!(second, "cached_token_abc");
    assert_eq!(requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn token_tenant_cache_hit_returns_cached_value() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"tenant_access_token":"cached_tenant_token","expire":7200}"#,
    )])
    .await;
    let client = LarkClient::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
    let tm = TokenManager::new(Arc::clone(&cache));
    let first = tm
        .get_tenant_access_token(client.config(), Some("tenant_1"), None)
        .await
        .unwrap();
    let second = tm
        .get_tenant_access_token(client.config(), Some("tenant_1"), None)
        .await
        .unwrap();

    assert_eq!(first, "cached_tenant_token");
    assert_eq!(second, "cached_tenant_token");
    assert_eq!(requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn token_cache_does_not_cross_app_secret_rotation() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"app_access_token":"token-for-secret-a","expire":7200}"#,
        ),
        http_response(200, r#"{"code":19002,"msg":"invalid app secret"}"#),
    ])
    .await;
    let current = LarkClient::builder("app_id", "secret-a")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let rotated = LarkClient::builder("app_id", "secret-b")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let tm = TokenManager::new(Arc::new(LocalCache::new()));

    assert_eq!(
        tm.get_app_access_token(current.config(), None)
            .await
            .unwrap(),
        "token-for-secret-a"
    );
    assert!(
        tm.get_app_access_token(rotated.config(), None)
            .await
            .is_err()
    );
    assert_eq!(requests.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn tenant_token_cache_does_not_cross_app_secret_rotation() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"tenant_access_token":"token-for-secret-a","expire":7200}"#,
        ),
        http_response(200, r#"{"code":19002,"msg":"invalid app secret"}"#),
    ])
    .await;
    let current = LarkClient::builder("app_id", "secret-a")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let rotated = LarkClient::builder("app_id", "secret-b")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let tm = TokenManager::new(Arc::new(LocalCache::new()));

    assert_eq!(
        tm.get_tenant_access_token(current.config(), Some("tenant_1"), None)
            .await
            .unwrap(),
        "token-for-secret-a"
    );
    assert!(
        tm.get_tenant_access_token(rotated.config(), Some("tenant_1"), None)
            .await
            .is_err()
    );
    assert_eq!(requests.lock().unwrap().len(), 2);
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

    let client = LarkClient::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();

    let cache: Arc<dyn Cache> = Arc::new(LocalCache::new());
    let atm = AppTicketManager::new(Arc::clone(&cache));

    let result = atm.get(client.config()).await.unwrap();
    assert!(result.is_none());
}
