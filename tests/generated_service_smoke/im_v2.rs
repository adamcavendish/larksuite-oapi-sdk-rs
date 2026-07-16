use super::prelude::*;

// ── IM v2 ──

#[tokio::test]
async fn im_v2_app_feed_card_and_feed_card_by_query_smoke() {
    let app_feed_body = r#"{"code":0,"msg":"ok","data":{"biz_id":"feed-card-1","failed_cards":[{"user_id":"u-1","reason":"forbidden"}]}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, app_feed_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateAppFeedCardV2ReqBody::default();
    let delete_body = DeleteAppFeedCardBatchV2ReqBody::default();
    let update_body = UpdateAppFeedCardBatchV2ReqBody::default();
    let chat_button_body = UpdateChatButtonV2ReqBody::default();
    let time_sensitive_body = BotTimeSensitiveFeedCardV2ReqBody::default();
    let patch_body = PatchFeedCardV2ReqBody::default();

    let create_resp = client
        .im_v2()
        .app_feed_card
        .create_by_query(
            &CreateAppFeedCardV2Query::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .im_v2()
        .app_feed_card_batch
        .delete_by_query(
            &DeleteAppFeedCardBatchV2Query::new(&delete_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .im_v2()
        .app_feed_card_batch
        .update_by_query(
            &UpdateAppFeedCardBatchV2Query::new(&update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let chat_button_resp = client
        .im_v2()
        .chat_button
        .update_by_query(
            &UpdateChatButtonV2Query::new(&chat_button_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let time_sensitive_resp = client
        .im_v2()
        .feed_card
        .bot_time_sensitive_by_query(
            &BotTimeSensitiveFeedCardV2Query::new(&time_sensitive_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let patch_resp = client
        .im_v2()
        .feed_card
        .patch_by_query(
            &PatchFeedCardV2Query::new("feed-card-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(delete_resp.success());
    assert!(update_resp.success());
    assert!(chat_button_resp.success());
    assert!(time_sensitive_resp.success());
    assert!(patch_resp.success());
    let data = create_resp.data.unwrap();
    assert_eq!(data.biz_id.as_deref(), Some("feed-card-1"));
    assert_eq!(
        data.failed_cards.unwrap()[0].reason.as_deref(),
        Some("forbidden")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/im/v2/app_feed_card "));
    assert!(request.contains("DELETE /open-apis/im/v2/app_feed_card/batch "));
    assert!(request.contains("PUT /open-apis/im/v2/app_feed_card/batch "));
    assert!(request.contains("PUT /open-apis/im/v2/chat_button "));
    assert!(request.contains("PATCH /open-apis/im/v2/feed_cards/bot_time_sentive "));
    assert!(request.contains("PATCH /open-apis/im/v2/feed_cards/feed-card-1 "));
}

#[tokio::test]
async fn im_v2_biz_entity_tag_relation_by_query_smoke() {
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let get_body = r#"{"code":0,"msg":"ok","data":{"tag_info_with_bind_versions":[{"tag_info":{"id":"tag-1","i18n_names":[{"locale":"en_us","name":"Important"}]},"bind_version":"v1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty_body),
        http_response(200, get_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = BizEntityTagRelationV2ReqBody::default();
    let update_body = BizEntityTagRelationV2ReqBody::default();

    let create_resp = client
        .im_v2()
        .biz_entity_tag_relation
        .create_by_query(
            &CreateBizEntityTagRelationV2Query::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let get_resp = client
        .im_v2()
        .biz_entity_tag_relation
        .get_by_query(
            &GetBizEntityTagRelationV2Query::new(),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .im_v2()
        .biz_entity_tag_relation
        .update_by_query(
            &UpdateBizEntityTagRelationV2Query::new(&update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(get_resp.success());
    assert!(update_resp.success());
    let tag_info_with_bind_version = get_resp
        .data
        .unwrap()
        .tag_info_with_bind_versions
        .unwrap()
        .remove(0);
    assert_eq!(
        tag_info_with_bind_version.bind_version.as_deref(),
        Some("v1")
    );
    assert_eq!(
        tag_info_with_bind_version
            .tag_info
            .unwrap()
            .i18n_names
            .unwrap()[0]
            .name
            .as_deref(),
        Some("Important")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/im/v2/biz_entity_tag_relation "));
    assert!(request.contains("GET /open-apis/im/v2/biz_entity_tag_relation "));
    assert!(request.contains("PUT /open-apis/im/v2/biz_entity_tag_relation "));
}

#[tokio::test]
async fn im_v2_tag_and_url_preview_by_query_smoke() {
    let create_tag_body = r#"{"code":0,"msg":"ok","data":{"id":"tag-1"}}"#;
    let patch_tag_body = r#"{"code":0,"msg":"ok","data":{"tag_info":{"id":"tag-1","name":"Renamed","i18n_names":[{"locale":"en_us","name":"Renamed"}]}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_tag_body),
        http_response(200, patch_tag_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateTagV2ReqBody::default();
    let patch_body = PatchTagV2ReqBody::default();
    let preview_body = BatchUpdateUrlPreviewV2ReqBody::default();

    let create_resp = client
        .im_v2()
        .tag
        .create_by_query(
            &CreateTagV2Query::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let patch_resp = client
        .im_v2()
        .tag
        .patch_by_query(
            &PatchTagV2Query::new("tag-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let preview_resp = client
        .im_v2()
        .url_preview
        .batch_update_by_query(
            &BatchUpdateUrlPreviewV2Query::new(&preview_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(patch_resp.success());
    assert!(preview_resp.success());
    assert_eq!(create_resp.data.unwrap().id.as_deref(), Some("tag-1"));
    let tag_info = patch_resp.data.unwrap().tag_info.unwrap();
    assert_eq!(tag_info.name.as_deref(), Some("Renamed"));
    assert_eq!(
        tag_info.i18n_names.unwrap()[0].locale.as_deref(),
        Some("en_us")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/im/v2/tags "));
    assert!(request.contains("PATCH /open-apis/im/v2/tags/tag-1 "));
    assert!(request.contains("POST /open-apis/im/v2/url_previews/batch_update "));
}
