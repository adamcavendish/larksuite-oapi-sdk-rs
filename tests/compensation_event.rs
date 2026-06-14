use larksuite_oapi_sdk_rs::events::compensation::P2CompensationArchiveChangedV1;

#[test]
fn compensation_archive_changed_event_matches_go_shape() {
    let event: P2CompensationArchiveChangedV1 = serde_json::from_value(serde_json::json!({
        "operate_type": "modify",
        "employment_id": "employment_1",
        "effective_date": "2026-06-14",
        "before_tid": "before_1",
        "after_tid": "after_1"
    }))
    .unwrap();

    assert_eq!(event.operate_type.as_deref(), Some("modify"));
    assert_eq!(event.employment_id.as_deref(), Some("employment_1"));
    assert_eq!(event.effective_date.as_deref(), Some("2026-06-14"));
    assert_eq!(event.before_tid.as_deref(), Some("before_1"));
    assert_eq!(event.after_tid.as_deref(), Some("after_1"));
}

#[test]
fn compensation_archive_changed_event_accepts_empty_and_null_payloads() {
    let empty: P2CompensationArchiveChangedV1 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(empty.operate_type.is_none());
    assert!(empty.employment_id.is_none());
    assert!(empty.effective_date.is_none());

    let null_fields: P2CompensationArchiveChangedV1 = serde_json::from_value(serde_json::json!({
        "before_tid": null,
        "after_tid": null
    }))
    .unwrap();
    assert!(null_fields.before_tid.is_none());
    assert!(null_fields.after_tid.is_none());
}
