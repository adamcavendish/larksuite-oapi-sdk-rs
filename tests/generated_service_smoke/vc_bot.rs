use super::prelude::*;
use larksuite_oapi_sdk_rs::service::vc::v1::{ListBotEventQuery, UserActiveMeetingBotQuery};

#[tokio::test]
async fn vc_countdown_and_search_credentials_contract() {
    use larksuite_oapi_sdk_rs::service::go_compatibility::GoCompatibilityEndpoint;

    let ok = http_response(200, r#"{"code":0,"msg":"ok","data":{"items":[]}}"#);
    let denied = http_response(200, r#"{"code":99991672,"msg":"permission denied"}"#);
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![ok.clone(), ok.clone(), ok.clone(), ok, denied]).await;
    let client = client_for(addr);
    let mut bodies = Vec::new();
    for (user, token) in [(true, "user-token"), (false, "tenant-token")] {
        let option = if user {
            RequestOption {
                user_access_token: Some(token.into()),
                ..Default::default()
            }
        } else {
            RequestOption {
                tenant_access_token: Some(token.into()),
                ..Default::default()
            }
        };
        let mut body = serde_json::json!({"meeting_id":"1234567890123456789", "action":"set", "duration":"5", "reminder_before_end":"1"});
        if !user {
            body["need_play_audio_at_end"] = serde_json::json!(false);
        }
        assert!(
            client
                .vc()
                .bot
                .countdown(&body, &option)
                .await
                .unwrap()
                .success()
        );
        bodies.push(body);
        let search_body =
            json_value!({"query":"Weekly", "meeting_filter":{"organizer_ids":["ou_organizer"]}});
        let response = client
            .go_compatibility()
            .request_json(
                GoCompatibilityEndpoint::PostVcV1MeetingsSearch,
                std::iter::empty::<(&str, &str)>(),
                [("page_size", "10"), ("page_token", "next page")],
                Some(&search_body),
                &option,
            )
            .await
            .unwrap();
        assert!(response.success());
        assert_eq!(response.data.unwrap()["items"], serde_json::json!([]));
        bodies.push(serde_json::to_value(search_body).unwrap());
    }
    let denied = client
        .vc()
        .bot
        .countdown(
            json_value!({"meeting_id":"1234567890123456789", "action":"close_window"}),
            &RequestOption {
                user_access_token: Some("user-token".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(
        matches!(denied, larksuite_oapi_sdk_rs::LarkError::Api(error) if error.code == 99991672)
    );

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 5);
    for (index, request) in requests.iter().enumerate() {
        let (headers, body) = request.split_once("\r\n\r\n").unwrap();
        let target = if index == 1 || index == 3 {
            "POST /open-apis/vc/v1/meetings/search?page_size=10&page_token=next+page HTTP/1.1"
        } else {
            "POST /open-apis/vc/v1/bots/countdown HTTP/1.1"
        };
        assert_eq!(headers.lines().next(), Some(target));
        let token = if index == 2 || index == 3 {
            "tenant-token"
        } else {
            "user-token"
        };
        assert!(
            headers
                .lines()
                .any(|line| line == format!("authorization: Bearer {token}"))
        );
        let actual: serde_json::Value = serde_json::from_str(body).unwrap();
        if index < 4 {
            assert_eq!(actual, bodies[index]);
        } else {
            assert_eq!(
                actual,
                serde_json::json!({"meeting_id":"1234567890123456789", "action":"close_window"})
            );
        }
    }
}

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
