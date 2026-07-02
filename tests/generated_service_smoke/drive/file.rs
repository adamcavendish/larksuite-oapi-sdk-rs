use super::prelude::*;

// Drive file smoke tests

#[tokio::test]
async fn drive_list_files_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"file-1","name":"doc.pdf","type":"file"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileQuery::new()
        .folder_token("folder-1")
        .order_by("EditedTime")
        .direction("DESC")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.files.as_ref().unwrap()[0].name.as_deref(),
        Some("doc.pdf")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files?"));
    assert!(request.contains("folder_token=folder-1"));
    assert!(request.contains("order_by=EditedTime"));
    assert!(request.contains("direction=DESC"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_list_files_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"file-1","name":"doc.pdf","type":"file"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .list(
            None,
            None,
            None,
            None,
            None,
            None,
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.files.as_ref().unwrap()[0].name.as_deref(),
        Some("doc.pdf")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files"));
}

#[tokio::test]
async fn drive_file_download_smoke() {
    let body = "file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/download"));
}

#[tokio::test]
async fn drive_file_upload_all_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"file-token-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .upload_all(
            "report.txt",
            "explorer",
            "folder-token-1",
            3,
            Some("checksum-1"),
            b"abc".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().file_token.as_deref(),
        Some("file-token-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/upload_all"));
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("report.txt"));
    assert!(request.contains("name=\"parent_type\""));
    assert!(request.contains("explorer"));
    assert!(request.contains("name=\"parent_node\""));
    assert!(request.contains("folder-token-1"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n3\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"report.txt\""));
    assert!(request.contains("abc"));
}

#[tokio::test]
async fn drive_file_upload_part_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .upload_part(
            "upload-id-1",
            2,
            3,
            Some("checksum-1"),
            b"abc".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/upload_part"));
    assert!(request.contains("name=\"upload_id\""));
    assert!(request.contains("upload-id-1"));
    assert!(request.contains("name=\"seq\""));
    assert!(request.contains("\r\n2\r\n"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n3\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"part\""));
    assert!(request.contains("abc"));
}

#[tokio::test]
async fn drive_file_version_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"name":"v1"}],"page_token":"next-version","has_more":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileVersionQuery::new("file-token-1", "doc")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file_version
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(data.items.as_ref().unwrap()[0].name.as_deref(), Some("v1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/versions?"));
    assert!(request.contains("obj_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_file_view_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"viewer_id":"u1"}],"page_token":"next-view","has_more":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileViewRecordQuery::new("file-token-1", "doc")
        .viewer_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file_view_record
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.items.as_ref().unwrap()[0].viewer_id.as_deref(),
        Some("u1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/view_records?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("viewer_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
