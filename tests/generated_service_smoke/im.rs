use super::*;

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

#[tokio::test]
async fn im_list_pin_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","chat_id":"oc_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .pin
        .list_by_query(
            &ListPinQuery::new("oc_1")
                .start_time("1782910000")
                .end_time("1782913600")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/pins?"));
    assert!(request.contains("chat_id=oc_1"));
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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

#[tokio::test]
async fn im_message_resource_download_smoke() {
    let body = "resource-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"resource.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message_resource
        .get("msg-1", "file-key-1", "image", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("resource.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/msg-1/resources/file-key-1?"));
    assert!(request.contains("type=image"));
}

#[tokio::test]
async fn im_file_download_smoke() {
    let body = "file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"im-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .file
        .get("file-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("im-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/files/file-key-1"));
}

#[tokio::test]
async fn im_image_download_smoke() {
    let body = "image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .image
        .get("image-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/images/image-key-1"));
}
