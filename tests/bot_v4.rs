mod common;

use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::bot::v4::{BotSearchFilter, SearchBotReqBody};

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn bot_v4_search_uses_user_token_and_pages() {
    let page_one = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"bot-1","display_info":"First"}],"page_token":"next","has_more":true}}"#;
    let page_two = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"bot-2"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, page_one),
        http_response(200, page_two),
    ])
    .await;
    let client = client_for(addr);
    let mut filter = BotSearchFilter::default();
    filter.chat_ids = vec!["oc_1".into()];
    filter.has_chatter = Some(true);
    let body = SearchBotReqBody::new("calendar").filter(filter);
    let option = RequestOption {
        user_access_token: Some("user-token".into()),
        ..Default::default()
    };

    let bot_v4 = client.bot_v4();
    let mut iterator = bot_v4
        .bot
        .search_by_iterator(&body, Some(10), Some("open_id"))
        .limit(2);
    assert_eq!(
        iterator.next(&option).await.unwrap().unwrap().id.as_deref(),
        Some("bot-1")
    );
    assert_eq!(
        iterator.next(&option).await.unwrap().unwrap().id.as_deref(),
        Some("bot-2")
    );
    assert!(iterator.next(&option).await.unwrap().is_none());

    let requests = requests.lock().unwrap().join("\n");
    assert!(requests.contains("POST /open-apis/bot/v4/bot/search?"));
    assert!(requests.contains("user_id_type=open_id"));
    assert!(requests.contains("page_token=next"));
    assert!(requests.contains(r#""chat_ids":["oc_1"]"#));
}
