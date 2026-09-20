mod common;

use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::spark::v1::GetDbQuotaQuery;
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

fn client(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(1)
        .build()
        .unwrap()
}

fn user() -> RequestOption {
    RequestOption {
        user_access_token: Some("quota-user".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn database_quota_preserves_environment_and_response_contracts() {
    let data = [
        serde_json::json!({
            "storage_used_bytes": 2048,
            "storage_quota_bytes": 0,
            "usage_percent": 0,
            "tables": 2,
            "views": 0,
            "future": {"enabled": true}
        }),
        serde_json::json!({
            "storage_used_bytes": 1024,
            "storage_quota_bytes": 4096,
            "usage_percent": 25.12345,
            "tables": 3,
            "views": 1
        }),
        serde_json::json!({"storage_used_bytes": 0, "tables": 0, "views": 0}),
    ];
    let (addr, handle, requests) = mock_server_with_requests(
        data.iter()
            .map(|data| http_response(200, &format!(r#"{{"code":0,"data":{data}}}"#)))
            .collect(),
    )
    .await;
    let client = client(addr);
    let option = user();
    for (query, expected) in [
        GetDbQuotaQuery::new("app_a/b"),
        GetDbQuotaQuery::new("app_a/b").env("dev"),
        GetDbQuotaQuery::new("app_a/b").env("online"),
    ]
    .iter()
    .zip(&data)
    {
        let response = client
            .spark()
            .app
            .get_db_quota(query, &option)
            .await
            .unwrap();
        assert_eq!(response.data.unwrap().as_value(), expected);
    }

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    for (request, expected_line) in requests.iter().zip([
        "GET /open-apis/spark/v1/apps/app_a%2Fb/db/quota HTTP/1.1",
        "GET /open-apis/spark/v1/apps/app_a%2Fb/db/quota?env=dev HTTP/1.1",
        "GET /open-apis/spark/v1/apps/app_a%2Fb/db/quota?env=online HTTP/1.1",
    ]) {
        assert_eq!(request.lines().next().unwrap(), expected_line);
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: bearer quota-user\r\n")
        );
        assert!(request.split_once("\r\n\r\n").unwrap().1.is_empty());
    }
    handle.abort();
}

#[tokio::test]
async fn database_quota_preserves_api_errors() {
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"code":99991672,"msg":"permission denied"}"#,
    )])
    .await;
    let error = client(addr)
        .spark()
        .app
        .get_db_quota(&GetDbQuotaQuery::new("app_a"), &user())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        LarkError::Api(ref error) if error.code == 99991672 && error.msg == "permission denied"
    ));
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}

#[tokio::test]
async fn database_quota_rejects_missing_or_blank_user_tokens_without_io() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(200, r#"{"code":0,"data":{}}"#)]).await;
    let client = client(addr);
    for token in [None, Some(""), Some(" \t ")] {
        let option = RequestOption {
            user_access_token: token.map(Into::into),
            tenant_access_token: Some("tenant-must-not-be-used".into()),
            ..Default::default()
        };
        let error = client
            .spark()
            .app
            .get_db_quota(&GetDbQuotaQuery::new("app_a"), &option)
            .await
            .unwrap_err();
        assert!(matches!(error, LarkError::IllegalParam(_)));
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}
