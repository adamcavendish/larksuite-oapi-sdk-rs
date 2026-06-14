use larksuite_oapi_sdk_rs::events::approval::P2ApprovalUpdatedV4;

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
