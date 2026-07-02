use super::prelude::*;

// ── IM ──

#[tokio::test]
async fn im_list_message_reaction_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"reaction_id":"reaction-1","reaction_type":{"emoji_type":"OK"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message_reaction
        .list_by_query(
            &ListMessageReactionQuery::new("om_1")
                .reaction_type("OK")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/om_1/reactions?"));
    assert!(request.contains("reaction_type=OK"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}
