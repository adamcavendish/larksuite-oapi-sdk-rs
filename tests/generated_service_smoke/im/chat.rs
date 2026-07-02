use super::prelude::*;

// ── IM ──

#[tokio::test]
async fn im_get_chat_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"chat_id":"oc_1","name":"team"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .get("oc_1", Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().name.as_deref(), Some("team"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1?user_id_type=open_id"));
}

#[tokio::test]
async fn im_list_chat_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"chat_id":"oc_1","name":"team"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .list(None, None, None, None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("team")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats"));
}

#[tokio::test]
async fn im_get_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"chat_id":"oc_1","name":"team"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .get_by_query(
            &GetImChatQuery::new("oc_1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().name.as_deref(), Some("team"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn im_list_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"chat_id":"oc_1","name":"team"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .list_by_query(
            &ListImChatQuery::new()
                .user_id_type("open_id")
                .sort_type("ByCreateTimeAsc")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("sort_type=ByCreateTimeAsc"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_search_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"chat_id":"oc_1","name":"team"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .search_by_query(
            &SearchImChatQuery::new()
                .user_id_type("open_id")
                .query("team")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("query=team"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_get_chat_members_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id_type":"open_id","member_id":"ou_1","name":"Alice"}],"has_more":false,"member_total":1}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_members
        .get_by_query(
            &GetChatMembersQuery::new("oc_1")
                .member_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/members?"));
    assert!(request.contains("member_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_is_in_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"is_in_chat":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_members
        .is_in_chat_by_query(
            &IsInChatMembersQuery::new("oc_1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().is_in_chat, Some(true));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/members/is_in_chat"));
}
