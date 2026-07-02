use super::*;

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    }
}

// ── Spark v1 ──

#[tokio::test]
async fn spark_app_methods_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app_id":"app-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let option = user_option();
    let app_body = serde_json::json!({"name":"Demo"});
    client.spark().app.create(&app_body, &option).await.unwrap();
    client
        .spark()
        .app
        .get_app_visibility("app-1", &option)
        .await
        .unwrap();
    client
        .spark()
        .app
        .list(
            &SparkPageQuery::new().page_size(20).page_token("next"),
            &option,
        )
        .await
        .unwrap();
    client
        .spark()
        .app
        .patch("app-1", &app_body, &option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/spark/v1/apps "));
    assert!(request.contains(r#""name":"Demo""#));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/access-scope "));
    assert!(request.contains("GET /open-apis/spark/v1/apps?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next"));
    assert!(request.contains("PATCH /open-apis/spark/v1/apps/app-1 "));
}

#[tokio::test]
async fn spark_app_upload_and_command_methods_smoke() {
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
    let command_body = serde_json::json!({"sql":"select 1"});
    let scope_body = serde_json::json!({"visibility":"all"});
    let icon_body = FormDataBuilder::new()
        .file(
            "file",
            "icon.png",
            b"icon-bytes".to_vec(),
            Some("image/png"),
        )
        .build();
    let html_body = FormDataBuilder::new()
        .file(
            "file",
            "index.html",
            b"<html></html>".to_vec(),
            Some("text/html"),
        )
        .build();

    client.spark().app.icon(icon_body, &option).await.unwrap();
    client
        .spark()
        .app
        .sql_commands("app-1", &command_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .app
        .update_app_visibility("app-1", &scope_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .app
        .upload_html_code_and_release("app-1", html_body, &option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/spark/v1/icon "));
    assert!(request.contains("filename=\"icon.png\""));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/sql_commands "));
    assert!(request.contains(r#""sql":"select 1""#));
    assert!(request.contains("PUT /open-apis/spark/v1/apps/app-1/access-scope "));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/upload_and_release_html_code "));
    assert!(request.contains("filename=\"index.html\""));
}

#[tokio::test]
async fn spark_storage_methods_smoke() {
    let file_body = "file-bytes";
    let json_body = r#"{"code":0,"msg":"ok","data":{"file_key":"file-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response_with_headers(
            200,
            "Content-Disposition: attachment; filename=\"spark.bin\"\r\nContent-Type: application/octet-stream\r\n",
            file_body,
        ),
        http_response(200, json_body),
        http_response(200, json_body),
        http_response(200, json_body),
        http_response(200, json_body),
    ])
    .await;

    let client = client_for(addr);
    let option = user_option();
    let upload_body = FormDataBuilder::new()
        .field("file_name", "spark.txt")
        .file(
            "file",
            "spark.txt",
            b"spark-bytes".to_vec(),
            Some("text/plain"),
        )
        .build();
    let part_body = FormDataBuilder::new()
        .field("upload_id", "upload-1")
        .field("chunk_index", "1")
        .file("file", "part", b"part-bytes".to_vec(), Some("text/plain"))
        .build();

    let download = client
        .spark()
        .app_storage
        .download_by_query(
            &DownloadAppStorageQuery::new("app-1").file_key("file-1"),
            &option,
        )
        .await
        .unwrap();
    client
        .spark()
        .app_storage
        .upload("app-1", upload_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .app_storage
        .upload_initialize(
            "app-1",
            &serde_json::json!({"file_name":"spark.txt","file_size":10}),
            &option,
        )
        .await
        .unwrap();
    client
        .spark()
        .app_storage
        .upload_part("app-1", part_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .app_storage
        .upload_complete(
            "app-1",
            &serde_json::json!({"upload_id":"upload-1"}),
            &option,
        )
        .await
        .unwrap();

    assert_eq!(download.file_name.as_deref(), Some("spark.bin"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/storage?"));
    assert!(request.contains("file_key=file-1"));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/storage/upload "));
    assert!(request.contains("filename=\"spark.txt\""));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/storage/upload/initialize "));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/storage/upload/part "));
    assert!(request.contains("filename=\"part\""));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/storage/upload/complete "));
}

#[tokio::test]
async fn spark_table_view_enum_and_directory_methods_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
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
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/enums/status "));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/enums "));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/tables/table-1 "));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/tables "));
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
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/views/view-1/records?"));
    assert!(request.contains("POST /open-apis/spark/v1/directory/user/id_convert "));
    assert!(request.contains(r#""id_convert_type":1"#));
}
