use larksuite_oapi_sdk_rs::events::vc::{
    P2VcMeetingLeftV1, P2VcMeetingParticipantMeetingEndedV1, P2VcMeetingRecordingReadyV1,
    P2VcMeetingStartedV1, P2VcReserveConfigUpdatedV1, P2VcRoomCreatedV1, P2VcRoomLevelDeletedV1,
    P2VcRoomLevelUpdatedV1, P2VcRoomUpdatedV1,
};

#[test]
fn vc_meeting_events_have_typed_meeting_and_operator() {
    let event: P2VcMeetingStartedV1 = serde_json::from_value(serde_json::json!({
        "meeting": {
            "id": "meet_1",
            "topic": "Weekly Sync",
            "meeting_no": "123456789",
            "host_user": {
                "id": { "open_id": "ou_host", "user_id": "host_user" },
                "user_role": 1,
                "user_type": 1
            },
            "owner": {
                "id": { "union_id": "on_owner" }
            },
            "security_setting": {
                "security_level": 2,
                "user_ids": [{ "open_id": "ou_allowed" }]
            }
        },
        "operator": {
            "id": { "open_id": "ou_operator" },
            "user_role": 2
        }
    }))
    .unwrap();

    let meeting = event.meeting.as_ref().unwrap();
    assert_eq!(meeting.id.as_deref(), Some("meet_1"));
    assert_eq!(
        meeting.host_user.as_ref().unwrap().open_id(),
        Some("ou_host")
    );
    assert_eq!(meeting.owner.as_ref().unwrap().union_id(), Some("on_owner"));
    assert_eq!(
        meeting
            .security_setting
            .as_ref()
            .unwrap()
            .user_ids
            .as_ref()
            .unwrap()[0]
            .open_id(),
        Some("ou_allowed")
    );
    assert_eq!(
        event.operator.as_ref().unwrap().open_id(),
        Some("ou_operator")
    );
}

#[test]
fn vc_meeting_left_adds_leave_reason_and_leave_user() {
    let event: P2VcMeetingLeftV1 = serde_json::from_value(serde_json::json!({
        "meeting": { "id": "meet_1" },
        "operator": { "id": { "open_id": "ou_operator" } },
        "leave_reason": 3,
        "leave_user": { "id": { "open_id": "ou_left" } }
    }))
    .unwrap();

    assert_eq!(event.leave_reason, Some(3));
    assert_eq!(
        event.leave_user.as_ref().unwrap().open_id(),
        Some("ou_left")
    );
}

#[test]
fn vc_participant_and_recording_events_match_go_shape() {
    let participant: P2VcMeetingParticipantMeetingEndedV1 =
        serde_json::from_value(serde_json::json!({
            "meeting": { "id": "meet_1" },
            "operator": { "id": { "open_id": "ou_operator" } },
            "subscriber_ids": [{ "open_id": "ou_sub" }]
        }))
        .unwrap();
    assert_eq!(
        participant.subscriber_ids.as_ref().unwrap()[0].open_id(),
        Some("ou_sub")
    );

    let recording: P2VcMeetingRecordingReadyV1 = serde_json::from_value(serde_json::json!({
        "meeting": { "id": "meet_1" },
        "url": "https://example.test/recording",
        "duration": "120000"
    }))
    .unwrap();
    assert_eq!(
        recording.url.as_deref(),
        Some("https://example.test/recording")
    );
    assert_eq!(recording.duration.as_deref(), Some("120000"));
}

#[test]
fn vc_room_events_have_typed_room_payloads() {
    let created: P2VcRoomCreatedV1 = serde_json::from_value(serde_json::json!({
        "room": {
            "room_id": "room_1",
            "name": "Board Room",
            "capacity": 8,
            "room_status": {
                "status": true,
                "contact_ids": [{ "open_id": "ou_contact" }],
                "disable_notice": false
            },
            "device": [{ "name": "Display" }]
        }
    }))
    .unwrap();
    let room = created.room.as_ref().unwrap();
    assert_eq!(room.room_id.as_deref(), Some("room_1"));
    assert_eq!(room.room_status.as_ref().unwrap().status, Some(true));
    assert_eq!(
        room.room_status
            .as_ref()
            .unwrap()
            .contact_ids
            .as_ref()
            .unwrap()[0]
            .open_id(),
        Some("ou_contact")
    );

    let updated: P2VcRoomUpdatedV1 = serde_json::from_value(serde_json::json!({
        "room": { "room_id": "room_2", "name": "Updated" },
        "old_room": { "room_id": "ignored_by_current_go_shape" }
    }))
    .unwrap();
    assert_eq!(
        updated.room.as_ref().unwrap().name.as_deref(),
        Some("Updated")
    );
}

#[test]
fn vc_room_level_events_match_go_shape() {
    let updated: P2VcRoomLevelUpdatedV1 = serde_json::from_value(serde_json::json!({
        "room_level": {
            "room_level_id": "level_1",
            "name": "Floor 1",
            "parent_id": "root",
            "path": ["root", "level_1"],
            "has_child": true,
            "custom_group_id": "cg_1"
        }
    }))
    .unwrap();
    let room_level = updated.room_level.as_ref().unwrap();
    assert_eq!(room_level.room_level_id.as_deref(), Some("level_1"));
    assert_eq!(room_level.has_child, Some(true));

    let deleted: P2VcRoomLevelDeletedV1 = serde_json::from_value(serde_json::json!({
        "room_level_id": "level_1",
        "delete_child": true
    }))
    .unwrap();
    assert_eq!(deleted.room_level_id.as_deref(), Some("level_1"));
    assert_eq!(deleted.delete_child, Some(true));
}

#[test]
fn vc_reserve_config_updated_is_typed() {
    let event: P2VcReserveConfigUpdatedV1 = serde_json::from_value(serde_json::json!({
        "scope_id": "room_1",
        "scope_type": 2,
        "approve_config": {
            "approval_switch": 1,
            "meeting_duration": 1.5,
            "approvers": [{ "user_id": { "open_id": "ou_approver" } }]
        },
        "time_config": {
            "if_cover_child_scope": true,
            "time_switch": 1,
            "days_in_advance": 30
        },
        "reserve_scope_config": {
            "allow_all_users": 0,
            "allow_users": [{ "user_id": { "open_id": "ou_allowed" } }],
            "allow_depts": [{ "department_id": "od_1", "department_name": "Engineering" }]
        }
    }))
    .unwrap();

    assert_eq!(event.scope_id.as_deref(), Some("room_1"));
    assert_eq!(
        event.approve_config.as_ref().unwrap().meeting_duration,
        Some(1.5)
    );
    assert_eq!(
        event
            .approve_config
            .as_ref()
            .unwrap()
            .approvers
            .as_ref()
            .unwrap()[0]
            .user_id
            .as_ref()
            .unwrap()
            .open_id(),
        Some("ou_approver")
    );
    assert_eq!(
        event.time_config.as_ref().unwrap().days_in_advance,
        Some(30)
    );
    assert_eq!(
        event
            .reserve_scope_config
            .as_ref()
            .unwrap()
            .allow_depts
            .as_ref()
            .unwrap()[0]
            .department_name
            .as_deref(),
        Some("Engineering")
    );
}

#[test]
fn vc_event_structs_accept_empty_and_null_payloads() {
    let started: P2VcMeetingStartedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(started.meeting.is_none());

    let room: P2VcRoomCreatedV1 = serde_json::from_value(serde_json::json!({
        "room": null
    }))
    .unwrap();
    assert!(room.room.is_none());
}
