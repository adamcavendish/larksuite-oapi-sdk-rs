use super::*;

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
    client
        .docx()
        .document
        .raw_content_by_query(
            &GetDocumentRawContentQuery::new("doc-1").lang(1),
            &RequestOption::default(),
        )
        .await
        .unwrap();

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
        update_text_elements: Some(serde_json::json!({"elements":[]})),
        ..Default::default()
    };
    let batch_body = BatchUpdateBlockReqBody {
        requests: Some(vec![serde_json::json!({"request_id":"r-1"})]),
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

#[tokio::test]
async fn docx_building_block_and_chat_announcement_by_query_smoke() {
    let building_block_body = r#"{"code":0,"msg":"ok","data":{"building_block":{"token":"bb-1","host_token":"doc-1","host_type":"docx"}}}"#;
    let value_body = r#"{"code":0,"msg":"ok","data":{"items":[{"block_id":"block-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, building_block_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
    ])
    .await;

    let client = client_for(addr);
    let batch_body = serde_json::json!({"requests":[{"request_id":"r-1"}]});

    let building_block_resp = client
        .docx()
        .building_block
        .get_by_query(
            &GetBuildingBlockQuery::new("doc-1", "block-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let announcement_resp = client
        .docx()
        .chat_announcement
        .get_by_query(
            &GetDocxChatAnnouncementQuery::new("oc_1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let batch_update_resp = client
        .docx()
        .chat_announcement_block
        .batch_update_by_query(
            &BatchUpdateChatAnnouncementBlockQuery::new("oc_1", &batch_body)
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let block_resp = client
        .docx()
        .chat_announcement_block
        .get_by_query(
            &GetChatAnnouncementBlockQuery::new("oc_1", "block-1")
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let list_resp = client
        .docx()
        .chat_announcement_block
        .list_by_query(
            &ListChatAnnouncementBlockQuery::new("oc_1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(building_block_resp.success());
    assert!(announcement_resp.success());
    assert!(batch_update_resp.success());
    assert!(block_resp.success());
    assert!(list_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1/building_blocks/block-1 "));
    assert!(request.contains("GET /open-apis/docx/v1/chats/oc_1/announcement "));
    assert!(
        request.contains("PATCH /open-apis/docx/v1/chats/oc_1/announcement/blocks/batch_update?")
    );
    assert!(request.contains("GET /open-apis/docx/v1/chats/oc_1/announcement/blocks/block-1?"));
    assert!(request.contains("GET /open-apis/docx/v1/chats/oc_1/announcement/blocks?"));
    assert!(request.contains("document_revision_id=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""request_id":"r-1""#));
}

#[tokio::test]
async fn docx_chat_announcement_children_and_descendant_by_query_smoke() {
    let value_body = r#"{"code":0,"msg":"ok","data":{"items":[{"block_id":"block-2"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
    ])
    .await;

    let client = client_for(addr);
    let delete_body = serde_json::json!({"start_index":0,"end_index":1});
    let create_body = serde_json::json!({"children":[{"block_type":2}]});
    let descendant_body = serde_json::json!({"block_type":2});

    let delete_resp = client
        .docx()
        .chat_announcement_block_children
        .batch_delete_by_query(
            &BatchDeleteChatAnnouncementBlockChildrenQuery::new("oc_1", "block-1", &delete_body)
                .document_revision_id(1),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let create_resp = client
        .docx()
        .chat_announcement_block_children
        .create_by_query(
            &CreateChatAnnouncementBlockChildrenQuery::new("oc_1", "block-1", &create_body)
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let get_resp = client
        .docx()
        .chat_announcement_block_children
        .get_by_query(
            &GetChatAnnouncementBlockChildrenQuery::new("oc_1", "block-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let descendant_resp = client
        .docx()
        .document_block_descendant
        .create_by_query(
            &CreateDocumentBlockDescendantQuery::new("doc-1", "block-1", &descendant_body)
                .document_revision_id(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(delete_resp.success());
    assert!(create_resp.success());
    assert!(get_resp.success());
    assert!(descendant_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "DELETE /open-apis/docx/v1/chats/oc_1/announcement/blocks/block-1/children/batch_delete?"
    ));
    assert!(
        request
            .contains("POST /open-apis/docx/v1/chats/oc_1/announcement/blocks/block-1/children?")
    );
    assert!(
        request.contains("GET /open-apis/docx/v1/chats/oc_1/announcement/blocks/block-1/children?")
    );
    assert!(request.contains("POST /open-apis/docx/v1/documents/doc-1/blocks/block-1/descendant?"));
    assert!(request.contains("document_revision_id=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""start_index":0"#));
    assert!(request.contains(r#""children":[{"block_type":2}]"#));
    assert!(request.contains(r#""block_type":2"#));
}
