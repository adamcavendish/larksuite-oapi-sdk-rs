use larksuite_oapi_sdk_rs::events::mail::P2UserMailboxEventMessageReceivedV1;

#[test]
fn mail_message_received_event_is_typed() {
    let event: P2UserMailboxEventMessageReceivedV1 = serde_json::from_value(serde_json::json!({
        "mail_address": "ada@example.test",
        "message_id": "message_1",
        "mailbox_type": 1,
        "subscriber": {
            "user_ids": [
                { "open_id": "ou_1" },
                { "user_id": "user_2" },
                { "union_id": "on_3" }
            ]
        }
    }))
    .unwrap();

    let subscribers = &event.subscriber.as_ref().unwrap().user_ids;
    assert_eq!(event.mail_address.as_deref(), Some("ada@example.test"));
    assert_eq!(event.message_id.as_deref(), Some("message_1"));
    assert_eq!(event.mailbox_type, Some(1));
    assert_eq!(subscribers[0].open_id(), Some("ou_1"));
    assert_eq!(subscribers[1].user_id(), Some("user_2"));
    assert_eq!(subscribers[2].union_id(), Some("on_3"));
}

#[test]
fn mail_message_received_event_accepts_empty_and_null_payloads() {
    let empty: P2UserMailboxEventMessageReceivedV1 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(empty.mail_address.is_none());
    assert!(empty.subscriber.is_none());

    let null_subscriber: P2UserMailboxEventMessageReceivedV1 =
        serde_json::from_value(serde_json::json!({
            "subscriber": null
        }))
        .unwrap();
    assert!(null_subscriber.subscriber.is_none());
}
