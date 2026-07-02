use super::prelude::*;

// Drive like smoke tests

#[tokio::test]
async fn drive_v2_file_like_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"like-1","name":"Alice","type":"user"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileLikeQuery::new("file-token-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive_v2()
        .file_like
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.len(), 1);
    assert_eq!(data.files[0].name.as_deref(), Some("Alice"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/files/file-token-1/likes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_v2_file_like_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive_v2()
        .file_like
        .list(
            "file-token-1",
            Some("open_id"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/files/file-token-1/likes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
