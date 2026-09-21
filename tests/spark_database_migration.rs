mod common;

use common::{http_response, mock_server_with_requests};
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
        user_access_token: Some("migration-user".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn database_migration_preserves_preview_apply_and_status_contracts() {
    let responses = [
        serde_json::json!({
            "from": "dev",
            "to": "online",
            "changes": [{"type": "add_column", "future": true}]
        }),
        serde_json::json!({
            "from": "dev",
            "to": "online",
            "task_id": "task/a b",
            "future": {"accepted": true}
        }),
        serde_json::json!({
            "task_id": "task/a b",
            "status": "applied",
            "changes_applied": 1,
            "future": {"completed_at": "later"}
        }),
    ];
    let (addr, handle, requests) = mock_server_with_requests(
        responses
            .iter()
            .map(|data| http_response(200, &format!(r#"{{"code":0,"data":{data}}}"#)))
            .collect(),
    )
    .await;
    let client = client(addr);
    let option = user();

    let preview = client
        .spark()
        .app
        .preview_db_migration("app/a b", &option)
        .await
        .unwrap();
    assert_eq!(preview.data.unwrap().as_value(), &responses[0]);

    let apply = client
        .spark()
        .app
        .apply_db_migration("app/a b", &option)
        .await
        .unwrap();
    assert_eq!(apply.data.unwrap().as_value(), &responses[1]);

    let status = client
        .spark()
        .app
        .get_db_migration_status("app/a b", "task/a b", &option)
        .await
        .unwrap();
    assert_eq!(status.data.unwrap().as_value(), &responses[2]);

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    for request in requests.iter() {
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: bearer migration-user\r\n")
        );
    }
    assert_eq!(
        requests[0].lines().next().unwrap(),
        "POST /open-apis/spark/v1/apps/app%2Fa%20b/db/env_migrate HTTP/1.1"
    );
    assert_eq!(
        requests[1].lines().next().unwrap(),
        "POST /open-apis/spark/v1/apps/app%2Fa%20b/db/env_migrate HTTP/1.1"
    );
    assert_eq!(
        requests[2].lines().next().unwrap(),
        "GET /open-apis/spark/v1/apps/app%2Fa%20b/db/env_migrate_status?task_id=task%2Fa+b HTTP/1.1"
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(requests[0].split_once("\r\n\r\n").unwrap().1)
            .unwrap(),
        serde_json::json!({"dry_run": true})
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(requests[1].split_once("\r\n\r\n").unwrap().1)
            .unwrap(),
        serde_json::json!({"dry_run": false})
    );
    assert!(requests[2].split_once("\r\n\r\n").unwrap().1.is_empty());
    handle.abort();
}

#[tokio::test]
async fn database_migration_preserves_platform_errors() {
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"code":99991672,"msg":"migration locked"}"#,
    )])
    .await;
    let error = client(addr)
        .spark()
        .app
        .apply_db_migration("app", &user())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        LarkError::Api(ref error) if error.code == 99991672 && error.msg == "migration locked"
    ));
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}

#[tokio::test]
async fn database_migration_preserves_synchronous_apply_responses() {
    let data = serde_json::json!({
        "from": "dev",
        "to": "online",
        "status": "applied",
        "changes_applied": 2,
        "future": {"completed_at": "now"}
    });
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        &format!(r#"{{"code":0,"data":{data}}}"#),
    )])
    .await;
    let response = client(addr)
        .spark()
        .app
        .apply_db_migration("app", &user())
        .await
        .unwrap();
    assert_eq!(response.data.unwrap().as_value(), &data);
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}

#[tokio::test]
async fn database_migration_apply_does_not_retry_server_timeouts() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(504, r#"{"code":0}"#)]).await;
    let error = client(addr)
        .spark()
        .app
        .apply_db_migration("app", &user())
        .await
        .unwrap_err();
    assert!(matches!(error, LarkError::ServerTimeout(_)));
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}

#[tokio::test]
async fn database_migration_rejects_invalid_credentials_and_task_ids_without_io() {
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
            .preview_db_migration("app", &option)
            .await
            .unwrap_err();
        assert!(matches!(error, LarkError::IllegalParam(_)));
    }
    for task_id in ["", " \t "] {
        let error = client
            .spark()
            .app
            .get_db_migration_status("app", task_id, &user())
            .await
            .unwrap_err();
        assert!(matches!(error, LarkError::IllegalParam(_)));
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}
