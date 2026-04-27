mod common;

use common::{http_response, mock_server};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::error::LarkError;
use larksuite_oapi_sdk_rs::req::RequestOption;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
}

#[tokio::test]
async fn im_message_list_by_iterator_pages_and_limits() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"m1"},{"message_id":"m2"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"m3"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client
        .im()
        .message
        .list_by_iterator("chat", "chat", None, None, None, Some(2))
        .limit(2);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = iter.next(&option).await.unwrap().unwrap();
    let second = iter.next(&option).await.unwrap().unwrap();
    let third = iter.next(&option).await.unwrap();

    assert_eq!(first.message_id.as_deref(), Some("m1"));
    assert_eq!(second.message_id.as_deref(), Some("m2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));
}

#[tokio::test]
async fn im_chat_members_iterator_requests_follow_up_page() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"u1"}],"page_token":"token-1","has_more":true,"member_total":3}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"u2"}],"page_token":"token-2","has_more":false,"member_total":3}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client
        .im()
        .chat_members
        .get_by_iterator("chat", None, Some(1));
    let option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..Default::default()
    };

    let first = iter.next(&option).await.unwrap().unwrap();
    let second = iter.next(&option).await.unwrap().unwrap();
    let third = iter.next(&option).await.unwrap();

    assert_eq!(first.member_id.as_deref(), Some("u1"));
    assert_eq!(second.member_id.as_deref(), Some("u2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("token-2"));
}

#[tokio::test]
async fn im_search_chat_iterator_errors_are_propagated() {
    let body = r#"{"code":20001,"msg":"bad request"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut iter = client
        .im()
        .chat
        .search_by_iterator(None, Some("hello"), Some(1));
    let option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..Default::default()
    };

    let err = iter.next(&option).await.unwrap_err();
    assert!(matches!(err, LarkError::Api(_)));
}
