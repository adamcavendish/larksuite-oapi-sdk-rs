use super::prelude::*;

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    }
}

// ── Spark v1 ──

#[tokio::test]
async fn spark_enum_and_table_metadata_methods_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let option = user_option();

    client
        .spark()
        .app_enum
        .get_enum_detail("app-1", "status", &option)
        .await
        .unwrap();
    client
        .spark()
        .app_enum
        .get_enum_list("app-1", &option)
        .await
        .unwrap();
    client
        .spark()
        .app_table
        .get_table_detail("app-1", "table-1", &option)
        .await
        .unwrap();
    client
        .spark()
        .app_table
        .get_table_list("app-1", &option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/enums/status "));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/enums "));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/tables/table-1 "));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/tables "));
}
