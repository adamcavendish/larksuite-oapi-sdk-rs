use std::sync::{Arc, Mutex};

use larksuite_oapi_sdk_rs::event::{EventDispatcher, EventReq};
use larksuite_oapi_sdk_rs::events::helpdesk::{
    P2HelpdeskNotificationApproveV1, P2HelpdeskTicketCreatedV1, P2HelpdeskTicketMessageCreatedV1,
    P2HelpdeskTicketUpdatedV1,
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

#[test]
fn helpdesk_ticket_created_event_matches_go_shape() {
    let event: P2HelpdeskTicketCreatedV1 = serde_json::from_value(serde_json::json!({
        "ticket_id": "ticket_1",
        "helpdesk_id": "helpdesk_1",
        "guest": {
            "id": { "open_id": "ou_guest" },
            "avatar_url": "https://example.com/avatar.png",
            "name": "Guest",
            "email": "guest@example.com"
        },
        "stage": 2,
        "status": 3,
        "score": 4,
        "created_at": 1710000000,
        "updated_at": 1710000010,
        "closed_at": 1710000020,
        "channel": 9,
        "solve": 2,
        "customized_fields": [{
            "id": "field_1",
            "value": "value_1",
            "key_name": "priority",
            "display_name": "Priority",
            "position": 1,
            "required": true,
            "editable": false
        }],
        "chat_id": "oc_chat"
    }))
    .unwrap();

    assert_eq!(event.ticket_id.as_deref(), Some("ticket_1"));
    assert_eq!(event.helpdesk_id.as_deref(), Some("helpdesk_1"));
    let guest = event.guest.as_ref().unwrap();
    assert_eq!(guest.id.as_ref().unwrap().open_id(), Some("ou_guest"));
    assert_eq!(guest.name.as_deref(), Some("Guest"));
    assert_eq!(event.stage, Some(2));
    assert_eq!(event.status, Some(3));
    assert_eq!(event.score, Some(4));
    assert_eq!(event.channel, Some(9));
    assert_eq!(event.solve, Some(2));
    assert_eq!(
        event.customized_fields[0].key_name.as_deref(),
        Some("priority")
    );
    assert_eq!(event.customized_fields[0].required, Some(true));
    assert_eq!(event.chat_id.as_deref(), Some("oc_chat"));
}

#[test]
fn helpdesk_ticket_updated_event_matches_go_shape() {
    let event: P2HelpdeskTicketUpdatedV1 = serde_json::from_value(serde_json::json!({
        "object": {
            "ticket_id": "ticket_1",
            "helpdesk_id": "helpdesk_1",
            "guest": { "id": { "user_id": "user_1" } },
            "stage": 2,
            "status": 50,
            "created_at": 1710000000,
            "updated_at": 1710000030,
            "agents": [{ "id": { "open_id": "ou_agent" }, "name": "Agent" }],
            "closed_by": { "id": { "union_id": "on_closer" } },
            "collaborators": [{ "id": { "open_id": "ou_collab" } }],
            "customized_fields": [{ "id": "field_1", "value": "new" }],
            "chat_id": "oc_chat"
        },
        "old_object": {
            "stage": 1,
            "status": 3,
            "updated_at": 1710000010
        }
    }))
    .unwrap();

    let object = event.object.as_ref().unwrap();
    assert_eq!(object.ticket_id.as_deref(), Some("ticket_1"));
    assert_eq!(
        object
            .guest
            .as_ref()
            .unwrap()
            .id
            .as_ref()
            .unwrap()
            .user_id(),
        Some("user_1")
    );
    assert_eq!(
        object.agents[0].id.as_ref().unwrap().open_id(),
        Some("ou_agent")
    );
    assert_eq!(
        object
            .closed_by
            .as_ref()
            .unwrap()
            .id
            .as_ref()
            .unwrap()
            .union_id(),
        Some("on_closer")
    );
    assert_eq!(
        object.collaborators[0].id.as_ref().unwrap().open_id(),
        Some("ou_collab")
    );
    assert_eq!(object.customized_fields[0].value.as_deref(), Some("new"));

    let old = event.old_object.as_ref().unwrap();
    assert_eq!(old.stage, Some(1));
    assert_eq!(old.status, Some(3));
    assert_eq!(old.updated_at, Some(1710000010));
}

#[test]
fn helpdesk_ticket_message_created_event_matches_go_shape() {
    let event: P2HelpdeskTicketMessageCreatedV1 = serde_json::from_value(serde_json::json!({
        "ticket_message_id": "ticket_message_1",
        "message_id": "om_1",
        "msg_type": "text",
        "position": "right",
        "sender_id": { "open_id": "ou_sender" },
        "sender_type": 3,
        "text": "hello",
        "ticket": {
            "ticket_id": "ticket_1",
            "helpdesk_id": "helpdesk_1",
            "guest": {
                "id": "ou_guest",
                "name": "Guest",
                "department": "Support",
                "city": "Shanghai",
                "country": "CN"
            },
            "comments": [{
                "content": "note",
                "created_at": 1710000100,
                "id": 10,
                "user_avatar_url": "https://example.com/u.png",
                "user_name": "Agent",
                "user_id": 20
            }],
            "ticket_type": 2,
            "status": 3,
            "dissatisfaction_reason": { "zh_cn": "不满意", "en_us": "Bad" },
            "agents": [{ "id": "agent_1", "name": "Agent" }],
            "closed_by": { "id": "agent_2" },
            "collaborators": [{ "id": "agent_3" }],
            "customized_fields": [{ "id": "field_1", "display_name": "Field" }],
            "agent_service_duration": 1.5,
            "agent_first_response_duration": 60,
            "bot_service_duration": 30,
            "agent_resolution_time": 120,
            "actual_processing_time": 180,
            "agent_entry_time": 1710000200,
            "agent_first_response_time": 1710000210,
            "agent_last_response_time": 1710000300,
            "agent_owner": { "id": "agent_owner" }
        },
        "event_id": "event_1",
        "chat_id": "oc_chat",
        "content": {
            "content": "hello",
            "msg_type": "image",
            "image_keys": ["img_1", "img_2"],
            "image_key": "img_1"
        }
    }))
    .unwrap();

    assert_eq!(event.ticket_message_id.as_deref(), Some("ticket_message_1"));
    assert_eq!(event.message_id.as_deref(), Some("om_1"));
    assert_eq!(
        event.sender_id.as_ref().unwrap().open_id(),
        Some("ou_sender")
    );
    assert_eq!(event.sender_type, Some(3));
    assert_eq!(event.text.as_deref(), Some("hello"));
    assert_eq!(event.event_id.as_deref(), Some("event_1"));

    let ticket = event.ticket.as_ref().unwrap();
    assert_eq!(ticket.ticket_id.as_deref(), Some("ticket_1"));
    assert_eq!(
        ticket.guest.as_ref().unwrap().department.as_deref(),
        Some("Support")
    );
    assert_eq!(ticket.comments[0].content.as_deref(), Some("note"));
    assert_eq!(
        ticket
            .dissatisfaction_reason
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Bad")
    );
    assert_eq!(ticket.agents[0].id.as_deref(), Some("agent_1"));
    assert_eq!(ticket.agent_service_duration, Some(1.5));
    assert_eq!(ticket.agent_first_response_duration, Some(60));
    assert_eq!(
        ticket.agent_owner.as_ref().unwrap().id.as_deref(),
        Some("agent_owner")
    );

    let content = event.content.as_ref().unwrap();
    assert_eq!(content.msg_type.as_deref(), Some("image"));
    assert_eq!(content.image_keys, vec!["img_1", "img_2"]);
    assert_eq!(content.image_key.as_deref(), Some("img_1"));
}

#[test]
fn helpdesk_notification_approve_event_matches_go_shape() {
    let event: P2HelpdeskNotificationApproveV1 = serde_json::from_value(serde_json::json!({
        "notification_id": "notification_1",
        "helpdesk_id": "helpdesk_1",
        "approve_status": "APPROVED"
    }))
    .unwrap();

    assert_eq!(event.notification_id.as_deref(), Some("notification_1"));
    assert_eq!(event.helpdesk_id.as_deref(), Some("helpdesk_1"));
    assert_eq!(event.approve_status.as_deref(), Some("APPROVED"));
}

#[test]
fn helpdesk_events_accept_empty_and_null_payloads() {
    let created: P2HelpdeskTicketCreatedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(created.ticket_id.is_none());
    assert!(created.guest.is_none());
    assert!(created.customized_fields.is_empty());

    let updated: P2HelpdeskTicketUpdatedV1 = serde_json::from_value(serde_json::json!({
        "object": null,
        "old_object": null
    }))
    .unwrap();
    assert!(updated.object.is_none());
    assert!(updated.old_object.is_none());

    let message: P2HelpdeskTicketMessageCreatedV1 = serde_json::from_value(serde_json::json!({
        "sender_id": null,
        "ticket": null,
        "content": null
    }))
    .unwrap();
    assert!(message.sender_id.is_none());
    assert!(message.ticket.is_none());
    assert!(message.content.is_none());
}

#[tokio::test]
async fn helpdesk_dispatcher_uses_go_event_keys() {
    let ticket_id = Arc::new(Mutex::new(None::<String>));
    let ticket_id_clone = Arc::clone(&ticket_id);

    let dispatcher = EventDispatcher::new("", "").on_p2_helpdesk_ticket_created_v1(
        move |event: P2HelpdeskTicketCreatedV1| {
            let ticket_id = Arc::clone(&ticket_id_clone);
            async move {
                *ticket_id.lock().unwrap() = event.ticket_id;
                Ok(())
            }
        },
    );

    let resp = dispatcher
        .handle(make_event_req(
            "helpdesk.ticket.created_v1",
            serde_json::json!({ "ticket_id": "ticket_1" }),
        ))
        .await;

    assert_eq!(resp.status_code, 200);
    assert_eq!(ticket_id.lock().unwrap().as_deref(), Some("ticket_1"));

    let message_id = Arc::new(Mutex::new(None::<String>));
    let message_id_clone = Arc::clone(&message_id);

    let dispatcher = EventDispatcher::new("", "").on_p2_helpdesk_ticket_message_created_v1(
        move |event: P2HelpdeskTicketMessageCreatedV1| {
            let message_id = Arc::clone(&message_id_clone);
            async move {
                *message_id.lock().unwrap() = event.message_id;
                Ok(())
            }
        },
    );

    let resp = dispatcher
        .handle(make_event_req(
            "helpdesk.ticket_message.created_v1",
            serde_json::json!({ "message_id": "om_1" }),
        ))
        .await;

    assert_eq!(resp.status_code, 200);
    assert_eq!(message_id.lock().unwrap().as_deref(), Some("om_1"));
}
