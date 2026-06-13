use larksuite_oapi_sdk_rs::events::im::{
    P2ChatAccessEventBotP2pChatEnteredV1, P2ChatDisbandedV1, P2ChatMemberBotAddedV1,
    P2ChatMemberBotDeletedV1, P2ChatMemberUserAddedV1, P2ChatMemberUserDeletedV1,
    P2ChatMemberUserWithdrawnV1, P2ChatUpdatedV1, P2MessageReactionCreatedV1,
    P2MessageReactionDeletedV1, P2MessageReadV1, P2MessageRecalledV1, P2MessageReceiveV1, UserId,
};

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

#[test]
fn message_read_reader_is_typed() {
    let event: P2MessageReadV1 = serde_json::from_value(serde_json::json!({
        "reader": {
            "reader_id": {
                "open_id": "ou_reader",
                "user_id": "user_reader",
                "union_id": "on_reader"
            },
            "read_time": "1710000000123",
            "tenant_key": "tenant_read"
        },
        "message_id_list": ["om_1", "om_2"]
    }))
    .unwrap();

    assert_eq!(event.reader.read_time, "1710000000123");
    assert_eq!(event.reader.tenant_key, "tenant_read");
    assert_eq!(event.reader.open_id(), Some("ou_reader"));
    assert_eq!(event.reader.user_id(), Some("user_reader"));
    assert_eq!(event.reader.union_id(), Some("on_reader"));
    assert_eq!(event.message_id_list.len(), 2);
}

#[test]
fn message_reaction_created_has_typed_emoji_and_user() {
    let event: P2MessageReactionCreatedV1 = serde_json::from_value(serde_json::json!({
        "message_id": "om_reaction",
        "reaction_type": { "emoji_type": "SMILE" },
        "operator_type": "user",
        "user_id": {
            "open_id": "ou_user",
            "user_id": "user_id_val",
            "union_id": ""
        },
        "app_id": "",
        "action_time": "1710000000000"
    }))
    .unwrap();

    assert_eq!(event.message_id, "om_reaction");
    assert_eq!(event.reaction_type.emoji_type, "SMILE");
    assert_eq!(event.operator_type, "user");
    assert_eq!(
        event.user_id.as_ref().and_then(UserId::open_id),
        Some("ou_user")
    );
}

#[test]
fn message_reaction_deleted_has_typed_emoji_and_app() {
    let event: P2MessageReactionDeletedV1 = serde_json::from_value(serde_json::json!({
        "message_id": "om_reaction",
        "reaction_type": { "emoji_type": "THUMBUP" },
        "operator_type": "app",
        "user_id": null,
        "app_id": "cli_app_123",
        "action_time": "1710000000001"
    }))
    .unwrap();

    assert_eq!(event.operator_type, "app");
    assert_eq!(event.app_id, "cli_app_123");
    assert_eq!(event.reaction_type.emoji_type, "THUMBUP");
}

#[test]
fn message_recalled_has_recall_type() {
    let event: P2MessageRecalledV1 = serde_json::from_value(serde_json::json!({
        "message_id": "om_recalled",
        "chat_id": "oc_group",
        "recall_time": "1710000000000",
        "recall_type": "manual"
    }))
    .unwrap();

    assert_eq!(event.recall_type, "manual");
    assert_eq!(event.chat_id, "oc_group");
}

#[test]
fn chat_member_user_added_has_typed_operator_and_users() {
    let event: P2ChatMemberUserAddedV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_group",
        "operator_id": {
            "open_id": "ou_op",
            "user_id": "user_op",
            "union_id": ""
        },
        "operator_type": "user",
        "external": true,
        "operator_tenant_key": "tenant_op",
        "users": [{
            "name": "Ada",
            "tenant_key": "tenant_u",
            "user_id": {
                "open_id": "ou_u",
                "user_id": "user_u",
                "union_id": ""
            }
        }],
        "name": "Team Chat",
        "i18n_names": { "zh_cn": "组名", "en_us": "Group", "ja_jp": "" }
    }))
    .unwrap();

    assert_eq!(event.chat_id, "oc_group");
    assert_eq!(
        event.operator_id.as_ref().and_then(UserId::open_id),
        Some("ou_op")
    );
    assert_eq!(event.users.len(), 1);
    assert_eq!(event.users[0].name, "Ada");
    assert_eq!(event.users[0].open_id(), Some("ou_u"));
    assert_eq!(event.name, "Team Chat");
    assert_eq!(event.i18n_names.as_ref().unwrap().zh_cn, "组名");
}

#[test]
fn chat_member_user_deleted_has_typed_operator_and_users() {
    let event: P2ChatMemberUserDeletedV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_group",
        "operator_id": { "open_id": "ou_op" },
        "external": false,
        "operator_tenant_key": "t1",
        "users": [],
        "name": "Chat",
        "i18n_names": {}
    }))
    .unwrap();

    assert!(event.users.is_empty());
    assert_eq!(event.name, "Chat");
    assert_eq!(
        event.operator_id.as_ref().and_then(UserId::open_id),
        Some("ou_op")
    );
}

#[test]
fn chat_member_user_withdrawn_has_typed_operator_and_users() {
    let event: P2ChatMemberUserWithdrawnV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_group",
        "operator_id": { "user_id": "user_op" },
        "external": false,
        "operator_tenant_key": "t1",
        "users": [{
            "name": "Bob",
            "tenant_key": "t2",
            "user_id": { "user_id": "user_b" }
        }],
        "name": "Chat",
        "i18n_names": {}
    }))
    .unwrap();

    assert_eq!(event.users.len(), 1);
    assert_eq!(event.users[0].name, "Bob");
    assert_eq!(
        event.operator_id.as_ref().and_then(UserId::user_id),
        Some("user_op")
    );
}

#[test]
fn chat_disbanded_has_name_i18n_and_typed_operator() {
    let event: P2ChatDisbandedV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_group",
        "operator_id": { "open_id": "ou_op" },
        "external": false,
        "operator_tenant_key": "t1",
        "name": "Old Chat",
        "i18n_names": { "zh_cn": "旧群", "en_us": "Old Group", "ja_jp": "古いグループ" }
    }))
    .unwrap();

    assert_eq!(event.name, "Old Chat");
    assert_eq!(event.i18n_names.as_ref().unwrap().en_us, "Old Group");
    assert_eq!(
        event.operator_id.as_ref().and_then(UserId::open_id),
        Some("ou_op")
    );
}

#[test]
fn chat_updated_has_typed_changes_and_moderator_list() {
    let event: P2ChatUpdatedV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_group",
        "operator_id": { "open_id": "ou_op" },
        "external": false,
        "operator_tenant_key": "t1",
        "after_change": {
            "name": "Renamed",
            "owner_id": { "open_id": "ou_owner" },
            "labels": ["label1"],
            "group_message_type": "thread",
            "restricted_mode_setting": { "status": true }
        },
        "before_change": {
            "name": "Original"
        },
        "moderator_list": {
            "added_member_list": [{
                "tenant_key": "t_mod",
                "user_id": { "open_id": "ou_mod" }
            }],
            "removed_member_list": []
        }
    }))
    .unwrap();

    assert_eq!(event.after_change.as_ref().unwrap().name, "Renamed");
    assert_eq!(event.before_change.as_ref().unwrap().name, "Original");
    assert_eq!(
        event.operator_id.as_ref().and_then(UserId::open_id),
        Some("ou_op")
    );
    let mod_list = event.moderator_list.as_ref().unwrap();
    assert_eq!(mod_list.added_member_list.len(), 1);
    assert_eq!(mod_list.added_member_list[0].open_id(), Some("ou_mod"));
}

#[test]
fn bot_added_and_deleted_have_name_i18n_and_typed_operator() {
    let added: P2ChatMemberBotAddedV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_group",
        "operator_id": { "open_id": "ou_op" },
        "external": false,
        "operator_tenant_key": "t1",
        "name": "Bot Chat",
        "i18n_names": { "zh_cn": "机器人群", "en_us": "Robot Group" }
    }))
    .unwrap();

    assert_eq!(added.name, "Bot Chat");
    assert_eq!(added.i18n_names.as_ref().unwrap().en_us, "Robot Group");
    assert_eq!(
        added.operator_id.as_ref().and_then(UserId::open_id),
        Some("ou_op")
    );

    let deleted: P2ChatMemberBotDeletedV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_group",
        "operator_id": { "open_id": "ou_op" },
        "external": false,
        "operator_tenant_key": "t1",
        "name": "Bot Chat",
        "i18n_names": {}
    }))
    .unwrap();

    assert_eq!(deleted.name, "Bot Chat");
    assert_eq!(
        deleted.operator_id.as_ref().and_then(UserId::open_id),
        Some("ou_op")
    );
}

#[test]
fn bot_p2p_chat_entered_has_typed_operator() {
    let event: P2ChatAccessEventBotP2pChatEnteredV1 = serde_json::from_value(serde_json::json!({
        "chat_id": "oc_p2p",
        "operator_id": { "open_id": "ou_enter" },
        "last_message_id": "om_last",
        "last_message_create_time": "1710000000000"
    }))
    .unwrap();

    assert_eq!(event.chat_id, "oc_p2p");
    assert_eq!(event.last_message_id, "om_last");
    assert_eq!(
        event.operator_id.as_ref().and_then(UserId::open_id),
        Some("ou_enter")
    );
}

#[test]
fn default_event_structs_deserialize_empty_payload() {
    let _: P2MessageReadV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2MessageRecalledV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2MessageReactionCreatedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2MessageReactionDeletedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatMemberUserAddedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatMemberUserDeletedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatMemberUserWithdrawnV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatDisbandedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatUpdatedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatMemberBotAddedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatMemberBotDeletedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    let _: P2ChatAccessEventBotP2pChatEnteredV1 =
        serde_json::from_value(serde_json::json!({})).unwrap();
}
