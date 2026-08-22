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
    let option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let resp = client
        .im()
        .message
        .read_users_by_query(
            &ReadUsersMessageQuery::new("om_1", "open_id")
                .page_size(20)
                .page_token("next-page"),
            &option,
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/om_1/read_users?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("authorization: Bearer user-token"));
}

#[tokio::test]
async fn im_read_status_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","read_status":"read"},{"message_id":"om_missing","read_status":"unexpected","unexpected_reason":"invalid"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let request_body = ReadStatusMessageReqBody::new([" om_1 ", "om_missing"]).unwrap();

    let response = client
        .im()
        .message
        .read_status(&request_body, &user_option)
        .await
        .unwrap();
    assert_eq!(request_body.message_ids(), ["om_1", "om_missing"]);
    assert_eq!(
        response
            .data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.get(1))
            .and_then(|item| item.unexpected_reason.as_deref()),
        Some("invalid")
    );

    let tenant_error = client
        .im()
        .message
        .read_status(&request_body, &tenant_option)
        .await
        .unwrap_err();
    assert!(
        tenant_error
            .to_string()
            .contains("tenant access token is not supported")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/im/v1/messages/read_status "));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains(r#""message_ids":["om_1","om_missing"]"#));
}

#[test]
fn im_read_status_request_rejects_invalid_message_id_sets() {
    assert!(ReadStatusMessageReqBody::new(Vec::<String>::new()).is_err());
    assert!(ReadStatusMessageReqBody::new(["om_1", " "]).is_err());
    assert!(ReadStatusMessageReqBody::new((0..51).map(|index| format!("om_{index}"))).is_err());
}
