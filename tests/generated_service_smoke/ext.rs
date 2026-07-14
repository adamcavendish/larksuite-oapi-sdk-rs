use super::prelude::*;

// ── Ext ──

#[tokio::test]
async fn ext_drive_explorer_create_file_smoke() {
    let response_body = r#"{"code":0,"msg":"ok","data":{"url":"https://example.com/file","token":"file-1","revision":2}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, response_body)]).await;

    let client = client_for(addr);
    let resp = client
        .ext()
        .drive_explorer
        .create_file(
            "folder-1",
            serde_json::json!({"title":"Spec","type":"docx"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.data.as_ref().and_then(|data| data.token.as_deref()),
        Some("file-1")
    );
    assert_eq!(resp.data.as_ref().and_then(|data| data.revision), Some(2));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/explorer/v2/file/folder-1 "));
    assert!(request.contains(r#""title":"Spec""#));
    assert!(request.contains(r#""type":"docx""#));
}
