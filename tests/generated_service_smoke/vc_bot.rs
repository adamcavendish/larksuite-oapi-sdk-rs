use super::prelude::*;
use larksuite_oapi_sdk_rs::service::vc::v1::{ListBotEventQuery, UserActiveMeetingBotQuery};

#[tokio::test]
async fn vc_bot_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body); 5]).await;
    let client = client_for(addr);
    let option = RequestOption {
        user_access_token: Some("user-token".to_owned()),
        ..RequestOption::default()
    };

    client
        .vc()
        .bot
        .events(
            &ListBotEventQuery::new()
                .meeting_id("meeting id")
                .page(PageQuery::new().page_size(10)),
            &option,
        )
        .await
        .unwrap();
    client
        .vc()
        .bot
        .join(
            json_value!({"join_type":1,"join_identify":{"meeting_no":"123456789"}}),
            &option,
        )
        .await
        .unwrap();
    client
        .vc()
        .bot
        .leave(json_value!({"meeting_id":"meeting id"}), &option)
        .await
        .unwrap();
    client.vc().bot.message(json_value!({"meeting_id":"meeting id","msg_type":"text","content":"hello","uuid":"message-1"}), &option).await.unwrap();
    client
        .vc()
        .bot
        .user_active_meeting(
            &UserActiveMeetingBotQuery::new()
                .user_id("ou_1")
                .user_id_type("open_id"),
            &option,
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for needle in [
        "GET /open-apis/vc/v1/bots/events?meeting_id=meeting+id&page_size=10 ",
        "POST /open-apis/vc/v1/bots/join ",
        "POST /open-apis/vc/v1/bots/leave ",
        "POST /open-apis/vc/v1/bots/message ",
        "GET /open-apis/vc/v1/bots/user_active_meeting?user_id=ou_1&user_id_type=open_id ",
        "authorization: Bearer user-token",
        r#""uuid":"message-1""#,
    ] {
        assert!(request.contains(needle), "missing {needle}:\n{request}");
    }
}
