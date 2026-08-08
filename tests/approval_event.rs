use larksuite_oapi_sdk_rs::events::approval::{
    P2ApprovalUpdatedV4, P2InstanceStatusChangedV4, P2TaskStatusChangedV4,
};

#[test]
fn approval_updated_event_is_typed() {
    let event: P2ApprovalUpdatedV4 = serde_json::from_value(serde_json::json!({
        "object": {
            "approval_id": "approval_1",
            "approval_code": "code_1",
            "version_id": "version_1",
            "widget_group_type": 1,
            "form_definition_id": "form_1",
            "process_obj": "instance",
            "timestamp": "1710000000",
            "extra": "{\"key\":\"value\"}"
        }
    }))
    .unwrap();

    let object = event.object.as_ref().unwrap();
    assert_eq!(object.approval_id.as_deref(), Some("approval_1"));
    assert_eq!(object.approval_code.as_deref(), Some("code_1"));
    assert_eq!(object.version_id.as_deref(), Some("version_1"));
    assert_eq!(object.widget_group_type, Some(1));
    assert_eq!(object.form_definition_id.as_deref(), Some("form_1"));
    assert_eq!(object.process_obj.as_deref(), Some("instance"));
    assert_eq!(object.timestamp.as_deref(), Some("1710000000"));
    assert_eq!(object.extra.as_deref(), Some("{\"key\":\"value\"}"));
}

#[test]
fn approval_updated_event_accepts_empty_and_null_payloads() {
    let empty: P2ApprovalUpdatedV4 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(empty.object.is_none());

    let null_object: P2ApprovalUpdatedV4 = serde_json::from_value(serde_json::json!({
        "object": null
    }))
    .unwrap();
    assert!(null_object.object.is_none());
}

#[test]
fn approval_status_changed_events_match_go_payloads() {
    let instance: P2InstanceStatusChangedV4 = serde_json::from_value(serde_json::json!({
        "event": {
            "approval_code": "leave",
            "instance_code": "instance_1",
            "external_id": "external_1",
            "status": "APPROVED",
            "operate_time": "1710000000000",
            "start_user": { "open_id": "ou_starter" }
        }
    }))
    .unwrap();
    let instance_data = instance.event.as_ref().unwrap();
    assert_eq!(instance_data.instance_code.as_deref(), Some("instance_1"));
    assert_eq!(
        instance_data.start_user.as_ref().unwrap().open_id(),
        Some("ou_starter")
    );

    let task: P2TaskStatusChangedV4 = serde_json::from_value(serde_json::json!({
        "event": {
            "approval_code": "leave",
            "instance_code": "instance_1",
            "task_id": "task_1",
            "task_external_id": "external_task_1",
            "assigned_user": { "user_id": "user_assignee" },
            "status": "PENDING",
            "operate_time": "1710000000001"
        }
    }))
    .unwrap();
    let task_data = task.event.as_ref().unwrap();
    assert_eq!(task_data.task_id.as_deref(), Some("task_1"));
    assert_eq!(
        task_data.assigned_user.as_ref().unwrap().user_id(),
        Some("user_assignee")
    );
}

#[test]
fn approval_status_changed_events_accept_empty_and_null_payloads() {
    let instance: P2InstanceStatusChangedV4 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(instance.event.is_none());

    let task: P2TaskStatusChangedV4 =
        serde_json::from_value(serde_json::json!({ "event": null })).unwrap();
    assert!(task.event.is_none());
}
