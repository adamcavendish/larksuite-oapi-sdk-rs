use super::prelude::*;

// ── Docx ──

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
