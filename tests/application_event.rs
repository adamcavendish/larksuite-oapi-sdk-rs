use larksuite_oapi_sdk_rs::events::application::{
    P2ApplicationAppVersionAuditV6, P2ApplicationAppVersionPublishApplyV6, P2ApplicationBotMenuV6,
    P2ApplicationCreatedV6, P2ApplicationFeedbackCreatedV6, P2ApplicationFeedbackUpdatedV6,
    P2ApplicationVisibilityAddedV6,
};

#[test]
fn application_created_and_bot_menu_events_are_typed() {
    let created: P2ApplicationCreatedV6 = serde_json::from_value(serde_json::json!({
        "operator_id": { "open_id": "ou_operator", "user_id": "user_1" },
        "app_id": "cli_app",
        "name": "App",
        "description": "desc",
        "avatar": "https://example.test/avatar.png",
        "app_scene_type": 0,
        "primary_language": "zh_cn",
        "create_source": "base"
    }))
    .unwrap();
    assert_eq!(created.app_id.as_deref(), Some("cli_app"));
    assert_eq!(
        created.operator_id.as_ref().unwrap().open_id(),
        Some("ou_operator")
    );
    assert_eq!(
        created.operator_id.as_ref().unwrap().user_id(),
        Some("user_1")
    );

    let menu: P2ApplicationBotMenuV6 = serde_json::from_value(serde_json::json!({
        "operator": {
            "operator_name": "Ada",
            "operator_id": { "union_id": "on_operator" }
        },
        "event_key": "menu_key",
        "timestamp": 1710000000000i64
    }))
    .unwrap();
    assert_eq!(
        menu.operator.as_ref().unwrap().union_id(),
        Some("on_operator")
    );
    assert_eq!(menu.event_key.as_deref(), Some("menu_key"));
    assert_eq!(menu.timestamp, Some(1710000000000));
}

#[test]
fn application_app_version_events_are_typed() {
    let audit: P2ApplicationAppVersionAuditV6 = serde_json::from_value(serde_json::json!({
        "operator_id": { "open_id": "ou_auditor" },
        "version_id": "ver_1",
        "creator_id": { "user_id": "creator_1" },
        "app_id": "cli_app",
        "operation": "pass",
        "remark": "ok",
        "audit_source": "admin"
    }))
    .unwrap();
    assert_eq!(
        audit.operator_id.as_ref().unwrap().open_id(),
        Some("ou_auditor")
    );
    assert_eq!(
        audit.creator_id.as_ref().unwrap().user_id(),
        Some("creator_1")
    );

    let publish: P2ApplicationAppVersionPublishApplyV6 =
        serde_json::from_value(serde_json::json!({
            "operator_id": { "union_id": "on_operator" },
            "online_version": {
                "app_id": "cli_app",
                "version": "1.0.0",
                "version_id": "ver_online",
                "app_name": "App",
                "scopes": [{
                    "scope": "contact:user.base",
                    "description": "user base",
                    "level": 1,
                    "token_types": ["user", "tenant"]
                }],
                "i18n": [{ "i18n_key": "en_us", "name": "App" }],
                "ability": {
                    "gadget": {
                        "enable_pc_mode": 1,
                        "schema_urls": ["lark://client/app"],
                        "pc_use_mobile_pkg": true
                    },
                    "web_app": { "pc_url": "https://pc.example.test" },
                    "bot": { "card_request_url": "https://bot.example.test" },
                    "navigate": {
                        "pc": { "version": "1.0.0", "image_url": "https://img.example.test" }
                    },
                    "cloud_doc": {
                        "space_url": "https://doc.example.test",
                        "i18n": [{ "i18n_key": "en_us", "name": "Docs" }],
                        "mode": 1
                    },
                    "docs_blocks": [{
                        "block_type_id": "blk_1",
                        "i18n": [{ "i18n_key": "en_us", "name": "Block" }]
                    }],
                    "message_action": {
                        "pc_app_link": "https://action.example.test",
                        "i18n": [{ "i18n_key": "en_us", "name": "Action" }]
                    },
                    "plus_menu": { "mobile_app_link": "https://plus.example.test" }
                },
                "remark": {
                    "remark": "publish",
                    "update_remark": "update",
                    "visibility": {
                        "is_all": false,
                        "visible_list": {
                            "open_ids": [{ "open_id": "ou_visible" }],
                            "department_ids": ["od_1"]
                        }
                    }
                }
            },
            "under_audit_version": { "version_id": "ver_audit" },
            "app_status": 1
        }))
        .unwrap();
    let online = publish.online_version.as_ref().unwrap();
    assert_eq!(online.scopes[0].token_types, vec!["user", "tenant"]);
    assert_eq!(
        online
            .ability
            .as_ref()
            .unwrap()
            .gadget
            .as_ref()
            .unwrap()
            .enable_pc_mode,
        Some(1)
    );
    assert_eq!(
        online
            .ability
            .as_ref()
            .unwrap()
            .cloud_doc
            .as_ref()
            .unwrap()
            .i18n[0]
            .name
            .as_deref(),
        Some("Docs")
    );
    assert_eq!(
        online
            .remark
            .as_ref()
            .unwrap()
            .visibility
            .as_ref()
            .unwrap()
            .visible_list
            .as_ref()
            .unwrap()
            .open_ids[0]
            .open_id(),
        Some("ou_visible")
    );
}

#[test]
fn application_feedback_events_are_typed() {
    let created: P2ApplicationFeedbackCreatedV6 = serde_json::from_value(serde_json::json!({
        "user_id": { "open_id": "ou_user" },
        "app_id": "cli_app",
        "feedback_time": "2026-01-01 00:00:00",
        "tenant_name": "Tenant",
        "feedback_type": 1,
        "fault_type": [1, 3],
        "fault_time": "2026-01-01 00:00:01",
        "source": 2,
        "contact": "ada@example.test",
        "description": "broken",
        "images": ["https://example.test/1.png"],
        "feedback_id": "fb_1",
        "feedback_path": "/home"
    }))
    .unwrap();
    assert_eq!(created.user_id.as_ref().unwrap().open_id(), Some("ou_user"));
    assert_eq!(created.fault_type, vec![1, 3]);
    assert_eq!(created.images, vec!["https://example.test/1.png"]);

    let updated: P2ApplicationFeedbackUpdatedV6 = serde_json::from_value(serde_json::json!({
        "feedback_ids": ["fb_1", "fb_2"],
        "status": 1,
        "app_id": "cli_app",
        "update_time": "2026-01-02 00:00:00",
        "operator_id": { "union_id": "on_operator" }
    }))
    .unwrap();
    assert_eq!(updated.feedback_ids, vec!["fb_1", "fb_2"]);
    assert_eq!(
        updated.operator_id.as_ref().unwrap().union_id(),
        Some("on_operator")
    );
}

#[test]
fn application_visibility_added_uses_typed_users() {
    let visibility: P2ApplicationVisibilityAddedV6 = serde_json::from_value(serde_json::json!({
        "users": [
            { "user_id": { "open_id": "ou_1" } },
            { "user_id": { "user_id": "user_2" } }
        ],
        "source": 3
    }))
    .unwrap();
    assert_eq!(
        visibility.users[0].user_id.as_ref().unwrap().open_id(),
        Some("ou_1")
    );
    assert_eq!(
        visibility.users[1].user_id.as_ref().unwrap().user_id(),
        Some("user_2")
    );
    assert_eq!(visibility.source, Some(3));
}

#[test]
fn application_event_structs_accept_empty_and_null_payloads() {
    let created: P2ApplicationCreatedV6 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(created.operator_id.is_none());

    let publish: P2ApplicationAppVersionPublishApplyV6 =
        serde_json::from_value(serde_json::json!({
            "operator_id": null,
            "online_version": null
        }))
        .unwrap();
    assert!(publish.operator_id.is_none());
    assert!(publish.online_version.is_none());
}
