use larksuite_oapi_sdk_rs::events::meeting_room::{
    P2MeetingRoomCreatedV1, P2MeetingRoomDeletedV1, P2MeetingRoomStatusChangedV1,
    P2MeetingRoomUpdatedV1,
};

#[test]
fn meeting_room_events_match_go_shape() {
    let created: P2MeetingRoomCreatedV1 = serde_json::from_value(serde_json::json!({
        "room_name": "Board Room",
        "room_id": "room_1"
    }))
    .unwrap();
    assert_eq!(created.room_name.as_deref(), Some("Board Room"));
    assert_eq!(created.room_id.as_deref(), Some("room_1"));

    let updated: P2MeetingRoomUpdatedV1 = serde_json::from_value(serde_json::json!({
        "room_name": "Board Room 2",
        "room_id": "room_2"
    }))
    .unwrap();
    assert_eq!(updated.room_name.as_deref(), Some("Board Room 2"));
    assert_eq!(updated.room_id.as_deref(), Some("room_2"));

    let deleted: P2MeetingRoomDeletedV1 = serde_json::from_value(serde_json::json!({
        "room_name": "Board Room 3",
        "room_id": "room_3"
    }))
    .unwrap();
    assert_eq!(deleted.room_name.as_deref(), Some("Board Room 3"));
    assert_eq!(deleted.room_id.as_deref(), Some("room_3"));

    let status_changed: P2MeetingRoomStatusChangedV1 = serde_json::from_value(serde_json::json!({
        "room_name": "Board Room 4",
        "room_id": "room_4"
    }))
    .unwrap();
    assert_eq!(status_changed.room_name.as_deref(), Some("Board Room 4"));
    assert_eq!(status_changed.room_id.as_deref(), Some("room_4"));
}

#[test]
fn meeting_room_events_accept_empty_and_null_payloads() {
    let created: P2MeetingRoomCreatedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(created.room_name.is_none());
    assert!(created.room_id.is_none());

    let updated: P2MeetingRoomUpdatedV1 = serde_json::from_value(serde_json::json!({
        "room_name": null,
        "room_id": null
    }))
    .unwrap();
    assert!(updated.room_name.is_none());
    assert!(updated.room_id.is_none());
}
