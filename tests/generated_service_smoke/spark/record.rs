use super::prelude::*;

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    }
}

// ── Spark v1 ──

#[tokio::test]
async fn spark_table_record_methods_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
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
    let mutation_body = serde_json::json!({"records":"[{\"name\":\"Alice\"}]"});
    let mutation = SparkTableMutationQuery::new("app-1", "table-1", &mutation_body)
        .columns("name")
        .on_conflict("name")
        .filter("age=gt.10")
        .env("online")
        .user_identifier_type("open_id");
    let records = SparkRecordQuery::new("app-1", "table-1")
        .select("_id,name")
        .filter("age=gt.10")
        .order("age.desc")
        .env("online")
        .user_identifier_type("open_id")
        .page_size(20)
        .page_token("next");

    client
        .spark()
        .app_table
        .get_table_record_list(&records, &option)
        .await
        .unwrap();
    client
        .spark()
        .app_table
        .post_table_records(&mutation, &option)
        .await
        .unwrap();
    client
        .spark()
        .app_table
        .batch_update_table_records(&mutation, &option)
        .await
        .unwrap();
    client
        .spark()
        .app_table
        .delete_table_records(&mutation, &option)
        .await
        .unwrap();
    client
        .spark()
        .app_table
        .patch_table_records(&mutation, &option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/tables/table-1/records?"));
    assert!(request.contains("select=_id%2Cname"));
    assert!(request.contains("filter=age%3Dgt.10"));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/tables/table-1/records?"));
    assert!(
        request
            .contains("PATCH /open-apis/spark/v1/apps/app-1/tables/table-1/records_batch_update?")
    );
    assert!(request.contains("DELETE /open-apis/spark/v1/apps/app-1/tables/table-1/records?"));
    assert!(request.contains("PATCH /open-apis/spark/v1/apps/app-1/tables/table-1/records?"));
    assert!(request.contains("columns=name"));
    assert!(request.contains("on_conflict=name"));
}
