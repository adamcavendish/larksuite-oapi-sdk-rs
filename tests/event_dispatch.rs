use std::sync::{Arc, Mutex};

use larksuite_oapi_sdk_rs::event::{EventDispatcher, EventReq};
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;

fn make_event_req(event_type: &str, event_payload: serde_json::Value) -> EventReq {
    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "test-id",
            "event_type": event_type,
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": event_payload
    });
    EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook/event".to_string(),
    }
}

#[tokio::test]
async fn test_url_verification() {
    let dispatcher = EventDispatcher::new("mytoken", "");
    let body = serde_json::json!({
        "type": "url_verification",
        "token": "mytoken",
        "challenge": "abc123"
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook/event".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["challenge"], "abc123");
}

#[tokio::test]
async fn test_event_dispatch_calls_handler() {
    let received = Arc::new(Mutex::new(None::<String>));
    let received_clone = Arc::clone(&received);

    let dispatcher =
        EventDispatcher::new("", "").on_p2_im_message_receive_v1(move |e: P2MessageReceiveV1| {
            let r = Arc::clone(&received_clone);
            async move {
                *r.lock().unwrap() = Some(e.message.message_id.clone());
                Ok(())
            }
        });

    let req = make_event_req(
        "im.message.receive_v1",
        serde_json::json!({
            "sender": { "sender_type": "user", "tenant_key": "t1" },
            "message": {
                "message_id": "om_xyz",
                "message_type": "text",
                "content": "{\"text\":\"hi\"}",
                "chat_id": "oc_1",
                "chat_type": "p2p",
                "create_time": "0",
                "update_time": "0",
                "root_id": "",
                "parent_id": ""
            }
        }),
    );

    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    assert_eq!(received.lock().unwrap().as_deref(), Some("om_xyz"));
}

#[tokio::test]
async fn test_unknown_event_returns_200() {
    let dispatcher = EventDispatcher::new("", "");
    let req = make_event_req("some.unknown.event_v1", serde_json::json!({"key": "val"}));
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
}

#[tokio::test]
async fn test_no_handler_registered_is_ok() {
    let dispatcher = EventDispatcher::new("", "");
    let req = make_event_req("im.message.receive_v1", serde_json::json!({}));
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
}

#[tokio::test]
async fn test_handler_deserialization_failure_returns_500() {
    // Register a handler expecting P2MessageReceiveV1, send it malformed JSON that
    // can't be coerced (e.g. "message" is a string instead of object).
    let dispatcher = EventDispatcher::new("", "")
        .on_p2_im_message_receive_v1(|_e: P2MessageReceiveV1| async { Ok(()) });

    // P2MessageReceiveV1 has `message: Message` with default — actually serde
    // will succeed with default fields, so test a type mismatch that cannot default:
    // send `sender` as an integer.
    let req = make_event_req(
        "im.message.receive_v1",
        serde_json::json!({ "sender": 42, "message": {} }),
    );
    // serde_json will fail to parse sender: 42 as MessageSender struct
    let resp = dispatcher.handle(req).await;
    // Should be 500 due to deserialization error
    assert_eq!(resp.status_code, 500);
}
