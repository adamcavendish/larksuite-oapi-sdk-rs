use std::sync::{Arc, Mutex};

use larksuite_oapi_sdk_rs::event::{EventDispatcher, EventReq};
use larksuite_oapi_sdk_rs::events::approval::{P2InstanceStatusChangedV4, P2TaskStatusChangedV4};
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;
use larksuite_oapi_sdk_rs::events::vc::{
    P2VcBotMeetingActivityV1, P2VcBotMeetingEndedV1, P2VcBotMeetingInvitedV1,
    P2VcBotMeetingStartedV1, P2VcNoteGeneratedV1,
};

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
async fn typed_v3910_event_handlers_dispatch() {
    let received = Arc::new(Mutex::new(Vec::new()));

    let instance_received = Arc::clone(&received);
    let task_received = Arc::clone(&received);
    let note_received = Arc::clone(&received);
    let dispatcher = EventDispatcher::new("", "")
        .on_p2_approval_instance_status_changed_v4(move |event: P2InstanceStatusChangedV4| {
            let received = Arc::clone(&instance_received);
            async move {
                received.lock().unwrap().push(
                    event
                        .event
                        .and_then(|data| data.instance_code)
                        .unwrap_or_default(),
                );
                Ok(())
            }
        })
        .on_p2_approval_task_status_changed_v4(move |event: P2TaskStatusChangedV4| {
            let received = Arc::clone(&task_received);
            async move {
                received.lock().unwrap().push(
                    event
                        .event
                        .and_then(|data| data.task_id)
                        .unwrap_or_default(),
                );
                Ok(())
            }
        })
        .on_p2_vc_note_generated_v1(move |event: P2VcNoteGeneratedV1| {
            let received = Arc::clone(&note_received);
            async move {
                received.lock().unwrap().push(
                    event
                        .event
                        .and_then(|data| data.note_id)
                        .unwrap_or_default(),
                );
                Ok(())
            }
        });

    for (event_type, event_payload) in [
        (
            "approval.instance.status_changed_v4",
            serde_json::json!({ "event": { "instance_code": "instance_1" } }),
        ),
        (
            "approval.task.status_changed_v4",
            serde_json::json!({ "event": { "task_id": "task_1" } }),
        ),
        (
            "vc.note.generated_v1",
            serde_json::json!({ "event": { "note_id": "note_1" } }),
        ),
    ] {
        let resp = dispatcher
            .handle(make_event_req(event_type, event_payload))
            .await;
        assert_eq!(resp.status_code, 200);
    }

    assert_eq!(
        received.lock().unwrap().as_slice(),
        ["instance_1", "task_1", "note_1"]
    );
}

#[tokio::test]
async fn typed_vc_bot_event_handlers_dispatch() {
    let received = Arc::new(Mutex::new(Vec::new()));

    let activity_received = Arc::clone(&received);
    let ended_received = Arc::clone(&received);
    let invited_received = Arc::clone(&received);
    let started_received = Arc::clone(&received);
    let dispatcher = EventDispatcher::new("", "")
        .on_p2_vc_bot_meeting_activity_v1(move |event: P2VcBotMeetingActivityV1| {
            let received = Arc::clone(&activity_received);
            async move {
                let activity = event
                    .meeting_activity_items
                    .and_then(|items| items.into_iter().next())
                    .unwrap_or_default();
                let context = activity
                    .document_context_changed_items
                    .and_then(|items| items.into_iter().next())
                    .unwrap_or_default();
                received.lock().unwrap().push(format!(
                    "{}:{}:{}:{}:{}",
                    activity.activity_event_type.unwrap_or_default(),
                    context
                        .share_doc
                        .and_then(|document| document.title)
                        .unwrap_or_default(),
                    context
                        .comment_focus
                        .and_then(|focus| focus.comment_id)
                        .unwrap_or_default(),
                    context
                        .section_location
                        .and_then(|location| location.title)
                        .unwrap_or_default(),
                    context
                        .element_preview
                        .and_then(|preview| preview.element_token)
                        .unwrap_or_default(),
                ));
                Ok(())
            }
        })
        .on_p2_vc_bot_meeting_ended_v1(move |event: P2VcBotMeetingEndedV1| {
            let received = Arc::clone(&ended_received);
            async move {
                received.lock().unwrap().push(
                    event
                        .meeting
                        .and_then(|meeting| meeting.id)
                        .unwrap_or_default(),
                );
                Ok(())
            }
        })
        .on_p2_vc_bot_meeting_invited_v1(move |event: P2VcBotMeetingInvitedV1| {
            let received = Arc::clone(&invited_received);
            async move {
                received
                    .lock()
                    .unwrap()
                    .push(event.call_id.unwrap_or_default());
                Ok(())
            }
        })
        .on_p2_vc_bot_meeting_started_v1(move |event: P2VcBotMeetingStartedV1| {
            let received = Arc::clone(&started_received);
            async move {
                received.lock().unwrap().push(
                    event
                        .meeting
                        .and_then(|meeting| meeting.topic)
                        .unwrap_or_default(),
                );
                Ok(())
            }
        });

    for (event_type, event_payload) in [
        (
            "vc.bot.meeting_activity_v1",
            serde_json::json!({
                "meeting_activity_items": [{
                    "activity_event_type": "document_context_changed",
                    "document_context_changed_items": [{
                        "share_doc": { "url": "https://example.com/doc", "title": "Meeting notes" },
                        "comment_focus": { "comment_id": "comment_1", "focused": true },
                        "section_location": { "title": "Decisions", "level": 2, "parent_titles": ["Agenda"] },
                        "element_preview": { "action": "open", "element_type": "image", "element_token": "element_1", "block_id": "block_1" }
                    }]
                }]
            }),
        ),
        (
            "vc.bot.meeting_ended_v1",
            serde_json::json!({ "meeting": { "id": "meeting_ended" } }),
        ),
        (
            "vc.bot.meeting_invited_v1",
            serde_json::json!({
                "meeting": { "id": "meeting_invited" },
                "bot": { "id": "bot_1" },
                "inviter": { "id": "user_1" },
                "call_id": "call_1"
            }),
        ),
        (
            "vc.bot.meeting_started_v1",
            serde_json::json!({ "meeting": { "topic": "Weekly sync" } }),
        ),
    ] {
        let resp = dispatcher
            .handle(make_event_req(event_type, event_payload))
            .await;
        assert_eq!(resp.status_code, 200);
    }

    assert_eq!(
        received.lock().unwrap().as_slice(),
        [
            "document_context_changed:Meeting notes:comment_1:Decisions:element_1",
            "meeting_ended",
            "call_1",
            "Weekly sync"
        ]
    );
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
