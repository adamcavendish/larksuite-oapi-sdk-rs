use larksuite_oapi_sdk_rs::events::apaas::P2ApaasWorkspaceRecordChangeV1;

#[test]
fn apaas_workspace_record_change_event_matches_go_shape() {
    let event: P2ApaasWorkspaceRecordChangeV1 = serde_json::from_value(serde_json::json!({
        "workspace": "workspace_1",
        "app": "app_1",
        "table": "table_1",
        "type": "UPDATE",
        "operator": "user_1",
        "before": "{\"name\":\"old\"}",
        "after": "{\"name\":\"new\"}"
    }))
    .unwrap();

    assert_eq!(event.workspace.as_deref(), Some("workspace_1"));
    assert_eq!(event.app.as_deref(), Some("app_1"));
    assert_eq!(event.table.as_deref(), Some("table_1"));
    assert_eq!(event.type_.as_deref(), Some("UPDATE"));
    assert_eq!(event.operator.as_deref(), Some("user_1"));
    assert_eq!(event.before.as_deref(), Some("{\"name\":\"old\"}"));
    assert_eq!(event.after.as_deref(), Some("{\"name\":\"new\"}"));
}

#[test]
fn apaas_workspace_record_change_event_accepts_empty_and_null_payloads() {
    let empty: P2ApaasWorkspaceRecordChangeV1 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(empty.workspace.is_none());
    assert!(empty.type_.is_none());

    let null_fields: P2ApaasWorkspaceRecordChangeV1 = serde_json::from_value(serde_json::json!({
        "before": null,
        "after": null
    }))
    .unwrap();
    assert!(null_fields.before.is_none());
    assert!(null_fields.after.is_none());
}
