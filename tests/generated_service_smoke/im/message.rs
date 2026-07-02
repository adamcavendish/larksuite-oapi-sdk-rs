use super::prelude::*;

// ── IM ──

#[tokio::test]
async fn im_get_message_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","chat_id":"oc_1","body":{"content":"hello"}}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message
        .get_by_query(
            &GetImMessageQuery::new("om_1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|message| message.body.as_ref())
            .and_then(|body| body.content.as_deref()),
        Some("hello")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/om_1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn im_list_message_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","chat_id":"oc_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message
        .list_by_query(
            &ListImMessageQuery::new("chat", "oc_1")
                .start_time("1782910000")
                .end_time("1782913600")
                .sort_type("ByCreateTimeAsc")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages?"));
    assert!(request.contains("container_id_type=chat"));
    assert!(request.contains("container_id=oc_1"));
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("sort_type=ByCreateTimeAsc"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_read_users_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id_type":"open_id","user_id":"ou_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message
        .read_users_by_query(
            &ReadUsersMessageQuery::new("om_1", "open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/om_1/read_users?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
