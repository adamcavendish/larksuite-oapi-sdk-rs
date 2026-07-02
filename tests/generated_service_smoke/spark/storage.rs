use super::prelude::*;

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    }
}

// ── Spark v1 ──

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
