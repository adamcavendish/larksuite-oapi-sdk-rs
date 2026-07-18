use super::prelude::*;

// ── Docx ──

#[tokio::test]
async fn docx_document_block_by_query_smoke() {
    let create_body_resp = r#"{"code":0,"msg":"ok","data":{"children":[{"block_id":"block-2","block_type":2}],"head_revision_id":2}}"#;
    let block_body_resp = r#"{"code":0,"msg":"ok","data":{"block":{"block_id":"block-1","block_type":2},"head_revision_id":2}}"#;
    let batch_body_resp = r#"{"code":0,"msg":"ok","data":{"blocks":[{"block_id":"block-1","block_type":2}],"head_revision_id":2}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"block_id":"block-1","block_type":2}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_body_resp),
        http_response(200, block_body_resp),
        http_response(200, block_body_resp),
        http_response(200, batch_body_resp),
        http_response(200, empty_body),
        http_response(200, list_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateBlockReqBody {
        children: Some(vec![DocxBlock {
            block_id: Some("block-2".into()),
            block_type: Some(2),
            ..Default::default()
        }]),
        index: Some(0),
    };
    let update_body = UpdateBlockReqBody {
        update_text_elements: Some(UpdateTextElementsRequest::default()),
        ..Default::default()
    };
    let batch_body = BatchUpdateBlockReqBody {
        requests: Some(vec![json_value!({"request_id":"r-1"})]),
    };
    let delete_body = DeleteBlocksReqBody {
        start_index: Some(0),
        end_index: Some(1),
    };

    client
        .docx()
        .block
        .create_by_query(
            &CreateDocumentBlockQuery::new("doc-1", "block-1", &create_body)
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .block
        .get_by_query(
            &GetDocumentBlockQuery::new("doc-1", "block-1")
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .block
        .update_by_query(
            &UpdateDocumentBlockQuery::new("doc-1", "block-1", &update_body)
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .block
        .batch_update_by_query(
            &BatchUpdateDocumentBlockQuery::new("doc-1", &batch_body)
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .block
        .delete_by_query(
            &DeleteDocumentBlockChildrenQuery::new("doc-1", "block-1", &delete_body)
                .document_revision_id(1),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .block
        .list_by_query(
            &ListDocumentBlockQuery::new("doc-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .docx()
        .block
        .list_children_by_query(
            &ListDocumentBlockChildrenQuery::new("doc-1", "block-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/docx/v1/documents/doc-1/blocks/block-1/children?"));
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1/blocks/block-1?"));
    assert!(request.contains("PATCH /open-apis/docx/v1/documents/doc-1/blocks/block-1?"));
    assert!(request.contains("PATCH /open-apis/docx/v1/documents/doc-1/blocks/batch_update?"));
    assert!(request.contains(
        "DELETE /open-apis/docx/v1/documents/doc-1/blocks/block-1/children/batch_delete?"
    ));
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1/blocks?"));
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1/blocks/block-1/children?"));
    assert!(request.contains("document_revision_id=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""index":0"#));
    assert!(request.contains(r#""request_id":"r-1""#));
    assert!(request.contains(r#""start_index":0"#));
}
