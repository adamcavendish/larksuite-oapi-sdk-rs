use super::prelude::*;

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    }
}

fn request_for<'a>(requests: &'a [String], marker: &str) -> &'a str {
    requests
        .iter()
        .find(|request| request.starts_with(marker))
        .map(String::as_str)
        .unwrap_or_else(|| panic!("missing request {marker}; got {requests:#?}"))
}

// Mirrors the executable dry-run contracts added by Lark CLI commit 158d15b3.
// In particular, sync task mutations are POST JSON bodies, not query parameters.
#[tokio::test]
async fn spark_db_sync_mutations_match_cli_contracts() {
    let body = r#"{"code":0,"msg":"ok","data":{"task_id":"streaming_1","status":"enabled"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let option = user_option();
    let create_body = json_value!({
        "config": {"mode": "streaming", "target": {"table": {"action": "create"}}},
        "preview": true,
        "env": "dev",
    });
    let update_body = json_value!({
        "task_id": "streaming_1",
        "config": {"mode": "streaming", "field_maps": []},
        "env": "dev",
    });

    client
        .spark()
        .db_sync
        .create("app id", &create_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .db_sync
        .update("app id", &update_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .db_sync
        .enable("app id", "streaming_1", &option)
        .await
        .unwrap();
    client
        .spark()
        .db_sync
        .disable("app id", "streaming_1", &option)
        .await
        .unwrap();
    client
        .spark()
        .db_sync
        .delete("app id", "streaming_1", &option)
        .await
        .unwrap();

    let requests = requests.lock().unwrap();
    let create = request_for(
        &requests,
        "POST /open-apis/spark/v1/apps/app%20id/db/sync_create ",
    );
    assert!(
        create.contains("authorization: Bearer user-token"),
        "{create}"
    );
    assert!(create.contains(r#""preview":true"#), "{create}");
    assert!(create.contains(r#""env":"dev"#), "{create}");
    assert!(!create.contains("?env="), "{create}");

    let update = request_for(
        &requests,
        "PUT /open-apis/spark/v1/apps/app%20id/db/sync_update ",
    );
    assert!(update.contains(r#""task_id":"streaming_1"#), "{update}");
    assert!(update.contains(r#""env":"dev"#), "{update}");
    assert!(!update.contains("?env="), "{update}");

    for action in ["sync_enable", "sync_disable", "sync_del"] {
        let request = request_for(
            &requests,
            &format!("POST /open-apis/spark/v1/apps/app%20id/db/{action} "),
        );
        assert!(request.contains(r#""task_id":"streaming_1"#), "{request}");
        assert!(!request.contains("?task_id="), "{request}");
    }
}

#[tokio::test]
async fn spark_db_sync_reads_match_cli_contracts() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[],"has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let option = user_option();
    let list = ListDbSyncQuery::new("app id")
        .page_size(20)
        .page_token("next")
        .mode("streaming")
        .status("enabled")
        .table_name("source table")
        .env("dev");

    client.spark().db_sync.list(&list, &option).await.unwrap();
    client
        .spark()
        .db_sync
        .get("app id", "streaming_1", &option)
        .await
        .unwrap();

    let requests = requests.lock().unwrap();
    let list = request_for(
        &requests,
        "GET /open-apis/spark/v1/apps/app%20id/db/sync_list?",
    );
    for query in [
        "page_size=20",
        "page_token=next",
        "mode=streaming",
        "status=enabled",
        "table=source+table",
        "env=dev",
    ] {
        assert!(list.contains(query), "missing {query} in {list}");
    }
    assert!(list.contains("authorization: Bearer user-token"), "{list}");

    let get = request_for(
        &requests,
        "GET /open-apis/spark/v1/apps/app%20id/db/sync_task?",
    );
    assert!(get.contains("task_id=streaming_1"), "{get}");
    assert!(!get.contains("?env="), "{get}");
    assert!(get.contains("authorization: Bearer user-token"), "{get}");
}
