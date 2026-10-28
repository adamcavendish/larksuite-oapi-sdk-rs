mod common;
use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

#[tokio::test]
async fn session_contracts_preserve_paths_tokens_bodies_and_pagination() {
    let (addr, handle, requests) = mock_server_with_requests((0..6).map(|_| http_response(200,
        r#"{"code":0,"data":{"future":true,"queued_count":1,"latest_turn":{"status":"completed"},"next_page_token":"next"}}"#)).collect()).await;
    let client = LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let option = RequestOption {
        user_access_token: Some("session-user".into()),
        ..Default::default()
    };
    let spark = client.spark();
    let s = &spark.session;
    let data = s.create("app/a", &option).await.unwrap().data.unwrap();
    assert_eq!(data.as_value()["future"], true);
    assert_eq!(data.as_value()["queued_count"], 1);
    assert!(data.as_value().get("next_poll_after_ms").is_none());
    s.list(
        "app/a",
        &PageQuery::new().page_size(20).page_token("cursor"),
        &option,
    )
    .await
    .unwrap();
    s.get("app/a", "sess/b", &option).await.unwrap();
    s.chat("app/a", "sess/b", " hello ", &option).await.unwrap();
    s.stop("app/a", "sess/b", " turn/c ", &option)
        .await
        .unwrap();
    let response = s
        .messages("app/a", "sess/b", "turn/c", Some("next"), &option)
        .await
        .unwrap();
    assert_eq!(response.data.unwrap().as_value()["next_page_token"], "next");
    let requests = requests.lock().unwrap();
    for (request, path) in requests.iter().zip([
        "POST /open-apis/spark/v1/apps/app%2Fa/sessions",
        "GET /open-apis/spark/v1/apps/app%2Fa/sessions?page_size=20&page_token=cursor",
        "GET /open-apis/spark/v1/apps/app%2Fa/sessions/sess%2Fb",
        "POST /open-apis/spark/v1/apps/app%2Fa/sessions/sess%2Fb/chat",
        "POST /open-apis/spark/v1/apps/app%2Fa/sessions/sess%2Fb/stop",
        "GET /open-apis/spark/v1/apps/app%2Fa/sessions/sess%2Fb/turns/turn%2Fc/reply_message?page_token=next",
    ]) {
        assert_eq!(request.lines().next().unwrap(), format!("{path} HTTP/1.1"));
        assert!(request.to_ascii_lowercase().contains("authorization: bearer session-user\r\n"));
    }
    for (index, expected) in [
        (3, serde_json::json!({"message":"hello"})),
        (4, serde_json::json!({"turn_id":"turn/c"})),
    ] {
        let body = requests[index].split_once("\r\n\r\n").unwrap().1;
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(body).unwrap(),
            expected
        );
    }
    assert!(requests[0].split_once("\r\n\r\n").unwrap().1.is_empty());
    handle.abort();
}

#[tokio::test]
async fn sessions_validate_locally_and_preserve_api_errors() {
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"code":99991672,"msg":"permission denied"}"#,
    )])
    .await;
    let client = LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap();
    let spark = client.spark();
    let s = &spark.session;
    let user = RequestOption {
        user_access_token: Some("user".into()),
        ..Default::default()
    };
    for token in [None, Some(""), Some(" \t")] {
        let option = RequestOption {
            user_access_token: token.map(Into::into),
            tenant_access_token: Some("wrong".into()),
            ..Default::default()
        };
        assert!(matches!(
            s.get("a", "s", &option).await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    assert!(matches!(
        s.chat("a", "s", "  ", &user).await,
        Err(LarkError::IllegalParam(_))
    ));
    assert!(matches!(
        s.get("a", " ", &user).await,
        Err(LarkError::IllegalParam(_))
    ));
    assert!(matches!(
        s.stop("a", "s", " ", &user).await,
        Err(LarkError::IllegalParam(_))
    ));
    assert!(matches!(
        s.list("a", &PageQuery::new().page_size(51), &user).await,
        Err(LarkError::IllegalParam(_))
    ));
    assert!(requests.lock().unwrap().is_empty());
    assert!(matches!(s.get("a", "s", &user).await, Err(LarkError::Api(e)) if e.code == 99991672));
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}

#[tokio::test]
async fn session_mutations_do_not_replay_ambiguous_gateway_failures() {
    for operation in ["create", "chat", "stop"] {
        let (addr, handle, requests) = mock_server_with_requests(vec![
            http_response(504, "{}"),
            http_response(200, r#"{"code":0,"data":{}}"#),
        ])
        .await;
        let client = LarkClient::builder("app", "secret")
            .base_url(format!("http://{addr}"))
            .max_retries(3)
            .build()
            .unwrap();
        let user = RequestOption {
            user_access_token: Some("session-user".into()),
            ..Default::default()
        };
        let spark = client.spark();
        let result = match operation {
            "create" => spark.session.create("app", &user).await,
            "chat" => spark.session.chat("app", "session", "message", &user).await,
            "stop" => spark.session.stop("app", "session", "turn", &user).await,
            _ => unreachable!(),
        };
        let request_count = requests.lock().unwrap().len();
        handle.abort();
        assert_eq!(request_count, 1, "{operation} must not replay a POST");
        assert!(
            matches!(result, Err(LarkError::ServerTimeout(_))),
            "{operation}: {result:?}"
        );
    }
}

#[tokio::test]
async fn session_reads_retain_configured_gateway_retries() {
    for operation in ["list", "get", "messages"] {
        let (addr, handle, requests) = mock_server_with_requests(vec![
            http_response(504, "{}"),
            http_response(200, r#"{"code":0,"data":{"future":true}}"#),
        ])
        .await;
        let client = LarkClient::builder("app", "secret")
            .base_url(format!("http://{addr}"))
            .max_retries(2)
            .build()
            .unwrap();
        let user = RequestOption {
            user_access_token: Some("session-user".into()),
            ..Default::default()
        };
        let spark = client.spark();
        let result = match operation {
            "list" => spark.session.list("app", &PageQuery::new(), &user).await,
            "get" => spark.session.get("app", "session", &user).await,
            "messages" => {
                spark
                    .session
                    .messages("app", "session", "turn", None, &user)
                    .await
            }
            _ => unreachable!(),
        };
        let request_count = requests.lock().unwrap().len();
        handle.abort();
        assert_eq!(request_count, 2, "{operation} should retain GET retries");
        assert_eq!(result.unwrap().data.unwrap().as_value()["future"], true);
    }
}
