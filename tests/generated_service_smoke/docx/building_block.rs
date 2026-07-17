use super::prelude::*;

// ── Docx ──

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
    let batch_body = BatchUpdateChatAnnouncementBlockReqBody {
        requests: Some(vec![UpdateBlockReqBody::default()]),
    };

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
    assert!(request.contains(r#""requests":[{}]"#));
}
