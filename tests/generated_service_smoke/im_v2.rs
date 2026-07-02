use super::prelude::*;

// ── IM v2 ──

#[tokio::test]
async fn im_v2_app_feed_card_and_feed_card_by_query_smoke() {
    let app_feed_body = r#"{"code":0,"msg":"ok","data":{"app_feed_card":{"id":"feed-card-1"}}}"#;
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
    let create_body = serde_json::json!({"card_id":"card-1","title":"Feed"});
    let delete_body = serde_json::json!({"card_ids":["card-1"]});
    let update_body = serde_json::json!({"cards":[{"card_id":"card-1","title":"Updated"}]});
    let chat_button_body = serde_json::json!({"chat_id":"oc_1","button":"enabled"});
    let time_sensitive_body = serde_json::json!({"card_id":"card-1","sensitive":true});
    let patch_body = serde_json::json!({"title":"Patched"});

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
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/im/v2/app_feed_card "));
    assert!(request.contains("DELETE /open-apis/im/v2/app_feed_card/batch "));
    assert!(request.contains("PUT /open-apis/im/v2/app_feed_card/batch "));
    assert!(request.contains("PUT /open-apis/im/v2/chat_button "));
    assert!(request.contains("PATCH /open-apis/im/v2/feed_cards/bot_time_sentive "));
    assert!(request.contains("PATCH /open-apis/im/v2/feed_cards/feed-card-1 "));
    assert!(request.contains(r#""title":"Feed""#));
    assert!(request.contains(r#""card_ids":["card-1"]"#));
    assert!(request.contains(r#""button":"enabled""#));
    assert!(request.contains(r#""sensitive":true"#));
    assert!(request.contains(r#""title":"Patched""#));
}

#[tokio::test]
async fn im_v2_biz_entity_tag_relation_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"biz_entity_tag_relation":{"id":"relation-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"entity_id":"entity-1","tag_id":"tag-1"});
    let update_body = serde_json::json!({"entity_id":"entity-1","tag_id":"tag-2"});

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
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/im/v2/biz_entity_tag_relation "));
    assert!(request.contains("GET /open-apis/im/v2/biz_entity_tag_relation "));
    assert!(request.contains("PUT /open-apis/im/v2/biz_entity_tag_relation "));
    assert!(request.contains(r#""entity_id":"entity-1""#));
    assert!(request.contains(r#""tag_id":"tag-1""#));
    assert!(request.contains(r#""tag_id":"tag-2""#));
}

#[tokio::test]
async fn im_v2_tag_and_url_preview_by_query_smoke() {
    let tag_body = r#"{"code":0,"msg":"ok","data":{"tag":{"id":"tag-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, tag_body),
        http_response(200, tag_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Important"});
    let patch_body = serde_json::json!({"name":"Renamed"});
    let preview_body = serde_json::json!({"items":[{"url":"https://example.com"}]});

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
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/im/v2/tags "));
    assert!(request.contains("PATCH /open-apis/im/v2/tags/tag-1 "));
    assert!(request.contains("POST /open-apis/im/v2/url_previews/batch_update "));
    assert!(request.contains(r#""name":"Important""#));
    assert!(request.contains(r#""name":"Renamed""#));
    assert!(request.contains(r#""url":"https://example.com""#));
}
