use super::prelude::*;

// ── IM ──

#[tokio::test]
async fn im_get_chat_moderation_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"moderation_setting":"all_members","items":[{"user_id_type":"open_id","user_id":"ou_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_moderation
        .get_by_query(
            &GetChatModerationQuery::new("oc_1")
                .user_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/moderation?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
