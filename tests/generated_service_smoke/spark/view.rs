use super::prelude::*;

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    }
}

// ── Spark v1 ──

#[tokio::test]
async fn spark_view_and_directory_methods_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let option = user_option();
    let view_records = SparkViewRecordQuery::new("app-1", "view-1")
        .select("_id,name")
        .filter("age=gt.10")
        .order("age.desc")
        .env("online")
        .user_identifier_type("open_id")
        .page_size(20)
        .page_token("next");

    client
        .spark()
        .app_view
        .get_view_record_list_by_query(&view_records, &option)
        .await
        .unwrap();
    client
        .spark()
        .directory_user
        .id_convert(
            &serde_json::json!({"id_convert_type":1,"ids":["ou_1"]}),
            &option,
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("select=_id%2Cname"));
    assert!(request.contains("filter=age%3Dgt.10"));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/views/view-1/records?"));
    assert!(request.contains("POST /open-apis/spark/v1/directory/user/id_convert "));
    assert!(request.contains(r#""id_convert_type":1"#));
}
