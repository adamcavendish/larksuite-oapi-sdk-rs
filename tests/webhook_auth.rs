use larksuite_oapi_sdk_rs::{EventDispatcher, EventReq};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn request(value: serde_json::Value) -> EventReq {
    EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&value).unwrap(),
        request_uri: "/webhook".into(),
    }
}

#[tokio::test]
async fn configured_tokens_authenticate_before_event_dispatch() {
    for v2 in [false, true] {
        for token in [
            None,
            Some(serde_json::Value::Null),
            Some(serde_json::json!("")),
            Some(serde_json::json!(123)),
            Some(serde_json::json!({})),
            Some(serde_json::json!([])),
            Some(serde_json::json!("wrong")),
            Some(serde_json::json!("expected")),
        ] {
            let calls = Arc::new(AtomicUsize::new(0));
            let observed = calls.clone();
            let dispatcher = EventDispatcher::new("expected", "")
                .skip_sign_verify()
                .on_event("audit.test", move |_| {
                    observed.fetch_add(1, Ordering::SeqCst);
                    async { Ok(()) }
                });
            let mut body = if v2 {
                serde_json::json!({"schema":"2.0","header":{"event_type":"audit.test"},"event":{}})
            } else {
                serde_json::json!({"type":"event_callback","event":{"type":"audit.test"}})
            };
            let allowed = token.as_ref().and_then(|t| t.as_str()) == Some("expected");
            if let Some(token) = token {
                if v2 {
                    body["header"]["token"] = token;
                } else {
                    body["token"] = token;
                }
            }
            let response = dispatcher.handle(request(body)).await;
            assert_eq!(response.status_code == 200, allowed, "v2={v2}");
            assert_eq!(calls.load(Ordering::SeqCst), usize::from(allowed));
        }
    }
}

#[tokio::test]
async fn challenge_requires_a_string_token_when_configured() {
    for kind in ["type", "req_type"] {
        for token in [
            None,
            Some(serde_json::Value::Null),
            Some(serde_json::json!(123)),
            Some(serde_json::json!("")),
            Some(serde_json::json!("wrong")),
            Some(serde_json::json!("expected")),
        ] {
            let mut body = serde_json::json!({"challenge":"fixture"});
            body[kind] = serde_json::json!("url_verification");
            let allowed = token.as_ref().and_then(|t| t.as_str()) == Some("expected");
            if let Some(token) = token {
                body["token"] = token;
            }
            assert_eq!(
                EventDispatcher::new("expected", "")
                    .handle(request(body))
                    .await
                    .status_code
                    == 200,
                allowed
            );
        }
    }
}

#[tokio::test]
async fn v2_never_falls_back_to_top_level_token() {
    for header in [
        serde_json::Value::Null,
        serde_json::json!({"event_type":"audit.test"}),
        serde_json::json!({"event_type":"audit.test","token":"wrong"}),
    ] {
        let body = serde_json::json!({"schema":"2.0","header":header,"token":"expected","event":{"type":"audit.test"}});
        assert_ne!(
            EventDispatcher::new("expected", "")
                .handle(request(body))
                .await
                .status_code,
            200
        );
    }
}

#[tokio::test]
async fn v2_callbacks_authenticate_before_invocation() {
    use larksuite_oapi_sdk_rs::CardActionTriggerResponse;
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let dispatcher = EventDispatcher::new("expected", "").on_card_action_trigger(move |_| {
        count.fetch_add(1, Ordering::SeqCst);
        async {
            Ok(CardActionTriggerResponse {
                toast: None,
                card: None,
            })
        }
    });
    for (token, expected) in [("wrong", 500), ("expected", 200)] {
        assert_eq!(dispatcher.handle(request(serde_json::json!({"schema":"2.0","header":{"event_type":"card.action.trigger","token":token},"event":{}}))).await.status_code, expected);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn legacy_card_challenges_reject_missing_tokens() {
    use larksuite_oapi_sdk_rs::CardActionHandler;
    let cards = CardActionHandler::new("expected", "", |_| async { Ok(serde_json::json!({})) })
        .skip_sign_verify();
    for token in [
        None,
        Some(serde_json::Value::Null),
        Some(serde_json::json!(123)),
        Some(serde_json::json!("expected")),
    ] {
        let allowed = token.as_ref().and_then(|t| t.as_str()) == Some("expected");
        let mut body = serde_json::json!({"type":"url_verification","challenge":"x"});
        if let Some(token) = token {
            body["token"] = token;
        }
        assert_eq!(
            cards.handle(request(body)).await.status_code == 200,
            allowed
        );
    }
}
