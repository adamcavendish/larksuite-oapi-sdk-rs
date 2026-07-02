use super::prelude::*;

// ── IM ──

#[tokio::test]
async fn im_get_chat_announcement_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"content":"notice","revision":"1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_announcement
        .get_by_query(
            &GetChatAnnouncementQuery::new("oc_1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().content.as_deref(), Some("notice"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/announcement?"));
    assert!(request.contains("user_id_type=open_id"));
}
