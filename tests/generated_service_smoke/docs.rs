use super::*;

// ── Docs ──

#[tokio::test]
async fn docs_document_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"document":{"document_id":"doc-1","title":"My Doc"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .docs()
        .document
        .get_by_query(
            &GetDocsDocumentQuery::new("doc-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let doc = resp.data.unwrap().document.unwrap();
    assert_eq!(doc.document_id.as_deref(), Some("doc-1"));
    assert_eq!(doc.title.as_deref(), Some("My Doc"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/docs/v1/documents/doc-1 "));
}

#[tokio::test]
async fn docs_content_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"content":"hello","revision":3}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .docs()
        .content
        .get_by_query(
            &GetDocsContentQuery::new("doc-token-1")
                .doc_type("docx")
                .content_type("markdown")
                .lang("en"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.as_ref().and_then(|data| data.content.as_deref()),
        Some("hello")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/docs/v1/content?"));
    assert!(request.contains("doc_token=doc-token-1"));
    assert!(request.contains("doc_type=docx"));
    assert!(request.contains("content_type=markdown"));
    assert!(request.contains("lang=en"));
}
