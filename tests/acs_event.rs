use larksuite_oapi_sdk_rs::events::acs::{P2AccessRecordCreatedV1, P2AcsUserUpdatedV1};

#[test]
fn acs_access_record_created_event_is_typed() {
    let event: P2AccessRecordCreatedV1 = serde_json::from_value(serde_json::json!({
        "access_record_id": "record_1",
        "user_id": { "open_id": "ou_user" },
        "device_id": "device_1",
        "is_clock_in": true,
        "is_door_open": false,
        "access_time": "1710000000"
    }))
    .unwrap();

    assert_eq!(event.access_record_id.as_deref(), Some("record_1"));
    assert_eq!(event.user_id.as_ref().unwrap().open_id(), Some("ou_user"));
    assert_eq!(event.device_id.as_deref(), Some("device_1"));
    assert_eq!(event.is_clock_in, Some(true));
    assert_eq!(event.is_door_open, Some(false));
    assert_eq!(event.access_time.as_deref(), Some("1710000000"));
}

#[test]
fn acs_user_updated_event_is_typed() {
    let event: P2AcsUserUpdatedV1 = serde_json::from_value(serde_json::json!({
        "user_id": { "union_id": "on_user" },
        "card": 12345,
        "face_uploaded": true
    }))
    .unwrap();

    assert_eq!(event.user_id.as_ref().unwrap().union_id(), Some("on_user"));
    assert_eq!(event.card, Some(12345));
    assert_eq!(event.face_uploaded, Some(true));
}

#[test]
fn acs_event_structs_accept_empty_and_null_payloads() {
    let access_record: P2AccessRecordCreatedV1 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(access_record.access_record_id.is_none());
    assert!(access_record.user_id.is_none());

    let user_updated: P2AcsUserUpdatedV1 = serde_json::from_value(serde_json::json!({
        "user_id": null
    }))
    .unwrap();
    assert!(user_updated.user_id.is_none());
}
