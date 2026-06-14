use larksuite_oapi_sdk_rs::events::calendar::{
    P2CalendarAclCreatedV4, P2CalendarAclDeletedV4, P2CalendarChangedV4, P2CalendarEventChangedV4,
};

#[test]
fn calendar_changed_event_has_typed_user_list() {
    let event: P2CalendarChangedV4 = serde_json::from_value(serde_json::json!({
        "user_id_list": [
            { "open_id": "ou_1" },
            { "user_id": "user_2" },
            { "union_id": "on_3" }
        ]
    }))
    .unwrap();

    assert_eq!(event.user_id_list[0].open_id(), Some("ou_1"));
    assert_eq!(event.user_id_list[1].user_id(), Some("user_2"));
    assert_eq!(event.user_id_list[2].union_id(), Some("on_3"));
}

#[test]
fn calendar_acl_events_have_typed_scope_and_user_list() {
    let created: P2CalendarAclCreatedV4 = serde_json::from_value(serde_json::json!({
        "acl_id": "acl_1",
        "role": "reader",
        "scope": {
            "type": "user",
            "user_id": { "open_id": "ou_scope" }
        },
        "user_id_list": [{ "user_id": "user_subscriber" }]
    }))
    .unwrap();

    let created_scope = created.scope.as_ref().unwrap();
    assert_eq!(created.acl_id.as_deref(), Some("acl_1"));
    assert_eq!(created.role.as_deref(), Some("reader"));
    assert_eq!(created_scope.type_.as_deref(), Some("user"));
    assert_eq!(
        created_scope.user_id.as_ref().unwrap().open_id(),
        Some("ou_scope")
    );
    assert_eq!(created.user_id_list[0].user_id(), Some("user_subscriber"));

    let deleted: P2CalendarAclDeletedV4 = serde_json::from_value(serde_json::json!({
        "acl_id": "acl_2",
        "role": "writer",
        "scope": {
            "type": "user",
            "user_id": { "union_id": "on_scope" }
        },
        "user_id_list": [{ "open_id": "ou_subscriber" }]
    }))
    .unwrap();

    let deleted_scope = deleted.scope.as_ref().unwrap();
    assert_eq!(deleted.acl_id.as_deref(), Some("acl_2"));
    assert_eq!(
        deleted_scope.user_id.as_ref().unwrap().union_id(),
        Some("on_scope")
    );
    assert_eq!(deleted.user_id_list[0].open_id(), Some("ou_subscriber"));
}

#[test]
fn calendar_event_changed_has_current_go_shape() {
    let event: P2CalendarEventChangedV4 = serde_json::from_value(serde_json::json!({
        "calendar_id": "cal_1",
        "user_id_list": [{ "open_id": "ou_subscriber" }],
        "calendar_event_id": "event_1",
        "change_type": "rsvp_changed",
        "rsvp_infos": [{
            "from_user_id": { "user_id": "user_rsvp" },
            "rsvp_status": "accept"
        }]
    }))
    .unwrap();

    assert_eq!(event.calendar_id.as_deref(), Some("cal_1"));
    assert_eq!(event.calendar_event_id.as_deref(), Some("event_1"));
    assert_eq!(event.change_type.as_deref(), Some("rsvp_changed"));
    assert_eq!(event.user_id_list[0].open_id(), Some("ou_subscriber"));
    assert_eq!(event.rsvp_infos[0].user_id(), Some("user_rsvp"));
    assert_eq!(event.rsvp_infos[0].rsvp_status.as_deref(), Some("accept"));
}

#[test]
fn calendar_event_structs_accept_empty_and_null_payloads() {
    let changed: P2CalendarChangedV4 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(changed.user_id_list.is_empty());

    let acl: P2CalendarAclCreatedV4 = serde_json::from_value(serde_json::json!({
        "scope": null
    }))
    .unwrap();
    assert!(acl.scope.is_none());
    assert!(acl.user_id_list.is_empty());
}
