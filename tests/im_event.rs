use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;

#[test]
fn message_receive_topic_fields_are_typed() {
    let event: P2MessageReceiveV1 = serde_json::from_value(serde_json::json!({
        "sender": {
            "sender_id": {
                "open_id": "ou_sender",
                "user_id": "user_sender",
                "union_id": "on_sender"
            },
            "sender_type": "user",
            "tenant_key": "tenant_1"
        },
        "message": {
            "message_id": "om_reply",
            "root_id": "om_root",
            "parent_id": "om_parent",
            "thread_id": "omt_topic",
            "create_time": "1710000000000",
            "update_time": "1710000000001",
            "chat_id": "oc_group",
            "chat_type": "group",
            "message_type": "text",
            "content": "{\"text\":\"@_user_1 hello\"}",
            "mentions": [
                {
                    "key": "@_user_1",
                    "id": {
                        "open_id": "ou_mentioned",
                        "user_id": "user_mentioned",
                        "union_id": "on_mentioned"
                    },
                    "mentioned_type": "user",
                    "name": "Ada",
                    "tenant_key": "tenant_1"
                }
            ]
        }
    }))
    .unwrap();

    assert_eq!(event.message.message_id, "om_reply");
    assert_eq!(event.message.root_id, "om_root");
    assert_eq!(event.message.parent_id, "om_parent");
    assert_eq!(event.message.thread_id, "omt_topic");
    assert_eq!(event.message.chat_id, "oc_group");
    assert_eq!(event.message.chat_type, "group");
    assert_eq!(event.sender.open_id(), Some("ou_sender"));
    assert_eq!(event.sender.user_id(), Some("user_sender"));
    assert_eq!(event.sender.union_id(), Some("on_sender"));

    let mentions = event.message.typed_mentions().unwrap();
    let mention = &mentions[0];
    assert_eq!(mention.key, "@_user_1");
    assert_eq!(mention.open_id(), Some("ou_mentioned"));
    assert_eq!(mention.user_id(), Some("user_mentioned"));
    assert_eq!(mention.union_id(), Some("on_mentioned"));
}

#[test]
fn typed_mentions_returns_error_for_unexpected_shape() {
    let event: P2MessageReceiveV1 = serde_json::from_value(serde_json::json!({
        "message": {
            "mentions": ["not-an-object"]
        }
    }))
    .unwrap();

    assert!(event.message.typed_mentions().is_err());
}

#[test]
fn message_receive_missing_topic_fields_default() {
    let event: P2MessageReceiveV1 = serde_json::from_value(serde_json::json!({
        "sender": {},
        "message": {
            "message_id": "om_plain",
            "chat_id": "oc_group",
            "chat_type": "group",
            "message_type": "text",
            "content": "{\"text\":\"hello\"}"
        }
    }))
    .unwrap();

    assert_eq!(event.message.thread_id, "");
    assert_eq!(event.message.root_id, "");
    assert_eq!(event.message.parent_id, "");
    assert!(event.message.mentions.is_empty());
    assert_eq!(event.sender.open_id(), None);
}
