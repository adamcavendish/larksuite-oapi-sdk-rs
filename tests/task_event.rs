use larksuite_oapi_sdk_rs::events::task::{
    P2TaskCommentUpdatedV1, P2TaskUpdateTenantV1, P2TaskUpdatedV1,
};

#[test]
fn task_update_tenant_event_is_typed() {
    let event: P2TaskUpdateTenantV1 = serde_json::from_value(serde_json::json!({
        "user_id_list": {
            "user_id_list": [
                { "open_id": "ou_1" },
                { "user_id": "user_2" },
                { "union_id": "on_3" }
            ]
        },
        "task_id": "task_1",
        "object_type": "task",
        "event_type": "update"
    }))
    .unwrap();
    let user_ids = &event.user_id_list.as_ref().unwrap().user_id_list;
    assert_eq!(user_ids[0].open_id(), Some("ou_1"));
    assert_eq!(user_ids[1].user_id(), Some("user_2"));
    assert_eq!(user_ids[2].union_id(), Some("on_3"));
    assert_eq!(event.task_id.as_deref(), Some("task_1"));
    assert_eq!(event.object_type.as_deref(), Some("task"));
    assert_eq!(event.event_type.as_deref(), Some("update"));
}

#[test]
fn task_updated_event_matches_go_shape() {
    let event: P2TaskUpdatedV1 = serde_json::from_value(serde_json::json!({
        "task_id": "task_1",
        "obj_type": 5
    }))
    .unwrap();
    assert_eq!(event.task_id.as_deref(), Some("task_1"));
    assert_eq!(event.obj_type, Some(5));
}

#[test]
fn task_comment_updated_event_matches_go_shape() {
    let event: P2TaskCommentUpdatedV1 = serde_json::from_value(serde_json::json!({
        "task_id": "task_1",
        "comment_id": "comment_1",
        "parent_id": "parent_1",
        "obj_type": 3
    }))
    .unwrap();
    assert_eq!(event.task_id.as_deref(), Some("task_1"));
    assert_eq!(event.comment_id.as_deref(), Some("comment_1"));
    assert_eq!(event.parent_id.as_deref(), Some("parent_1"));
    assert_eq!(event.obj_type, Some(3));
}

#[test]
fn task_event_structs_accept_empty_and_null_payloads() {
    let updated: P2TaskUpdatedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(updated.task_id.is_none());
    assert!(updated.obj_type.is_none());

    let tenant: P2TaskUpdateTenantV1 = serde_json::from_value(serde_json::json!({
        "user_id_list": null
    }))
    .unwrap();
    assert!(tenant.user_id_list.is_none());
}
