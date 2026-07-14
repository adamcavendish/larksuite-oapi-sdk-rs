use super::prelude::*;

// ── Docx ──

#[tokio::test]
async fn docx_get_document_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"document":{"document_id":"doc-1","title":"My Doc"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .docx()
        .document
        .get("doc-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let doc = resp.data.unwrap().document.unwrap();
    assert_eq!(doc.document_id.as_deref(), Some("doc-1"));
    assert_eq!(doc.title.as_deref(), Some("My Doc"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1"));
}

#[tokio::test]
async fn docx_document_by_query_smoke() {
    let document_body =
        r#"{"code":0,"msg":"ok","data":{"document":{"document_id":"doc-1","title":"Spec"}}}"#;
    let convert_body = r#"{"code":0,"msg":"ok","data":{"blocks":[{"block_id":"block-1"}]}}"#;
    let raw_body = r#"{"code":0,"msg":"ok","data":{"content":"hello"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, document_body),
        http_response(200, document_body),
        http_response(200, convert_body),
        http_response(200, raw_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateDocumentReqBody {
        folder_token: Some("folder-1".into()),
        title: Some("Spec".into()),
    };
    let convert_req = serde_json::json!({"content":"hello"});

    client
        .docx()
        .document
        .create_by_query(
            &CreateDocumentQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .document
        .get_by_query(
            &GetDocxDocumentQuery::new("doc-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .document
        .convert_by_query(
            &ConvertDocumentQuery::new(&convert_req),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let raw_content = client
        .docx()
        .document
        .raw_content_by_query(
            &GetDocumentRawContentQuery::new("doc-1").lang(1),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        raw_content
            .data
            .as_ref()
            .and_then(|data| data.content.as_deref()),
        Some("hello")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/docx/v1/documents "));
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1 "));
    assert!(request.contains("POST /open-apis/docx/v1/documents/blocks/convert "));
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1/raw_content?"));
    assert!(request.contains("lang=1"));
    assert!(request.contains(r#""folder_token":"folder-1""#));
    assert!(request.contains(r#""title":"Spec""#));
    assert!(request.contains(r#""content":"hello""#));
}
