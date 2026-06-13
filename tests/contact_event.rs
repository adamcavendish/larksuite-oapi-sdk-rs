use larksuite_oapi_sdk_rs::events::contact::{
    P2ContactCustomAttrEventUpdatedV3, P2ContactDepartmentDeletedV3, P2ContactDepartmentUpdatedV3,
    P2ContactEmployeeTypeEnumActivedV3, P2ContactEmployeeTypeEnumCreatedV3,
    P2ContactScopeUpdatedV3, P2ContactUserCreatedV3, P2ContactUserDeletedV3,
    P2ContactUserUpdatedV3,
};

#[test]
fn contact_user_events_have_typed_objects() {
    let created: P2ContactUserCreatedV3 = serde_json::from_value(serde_json::json!({
        "object": {
            "open_id": "ou_user",
            "union_id": "on_user",
            "user_id": "user_1",
            "name": "Ada",
            "status": { "is_activated": true },
            "department_ids": ["od_1"],
            "job_level_id": "jl_1"
        }
    }))
    .unwrap();
    let object = created.object.as_ref().unwrap();
    assert_eq!(object.open_id(), Some("ou_user"));
    assert_eq!(object.union_id(), Some("on_user"));
    assert_eq!(object.user_id(), Some("user_1"));
    assert_eq!(object.status.as_ref().unwrap().is_activated, Some(true));

    let updated: P2ContactUserUpdatedV3 = serde_json::from_value(serde_json::json!({
        "object": { "open_id": "ou_after", "name": "Ada Lovelace" },
        "old_object": { "open_id": "ou_before", "name": "Ada" }
    }))
    .unwrap();
    assert_eq!(
        updated.object.as_ref().unwrap().name.as_deref(),
        Some("Ada Lovelace")
    );
    assert_eq!(
        updated.old_object.as_ref().unwrap().name.as_deref(),
        Some("Ada")
    );

    let deleted: P2ContactUserDeletedV3 = serde_json::from_value(serde_json::json!({
        "object": { "open_id": "ou_deleted" },
        "old_object": { "open_id": "ou_deleted", "department_ids": ["od_old"] }
    }))
    .unwrap();
    assert_eq!(
        deleted.old_object.as_ref().unwrap().open_id.as_deref(),
        Some("ou_deleted")
    );
}

#[test]
fn contact_department_events_have_typed_objects() {
    let updated: P2ContactDepartmentUpdatedV3 = serde_json::from_value(serde_json::json!({
        "object": {
            "department_id": "dept_1",
            "open_department_id": "od_1",
            "name": "Engineering",
            "order": 42,
            "department_hrbps": [{ "open_id": "ou_hr" }]
        },
        "old_object": { "name": "Eng" }
    }))
    .unwrap();
    let object = updated.object.as_ref().unwrap();
    assert_eq!(object.open_department_id.as_deref(), Some("od_1"));
    assert_eq!(object.order, Some(42));
    assert_eq!(
        object.department_hrbps.as_ref().unwrap()[0].open_id(),
        Some("ou_hr")
    );
    assert_eq!(
        updated.old_object.as_ref().unwrap().name.as_deref(),
        Some("Eng")
    );

    let deleted: P2ContactDepartmentDeletedV3 = serde_json::from_value(serde_json::json!({
        "object": { "open_department_id": "od_deleted" },
        "old_object": { "open_department_id": "od_deleted", "status": { "is_deleted": true } }
    }))
    .unwrap();
    assert_eq!(
        deleted
            .old_object
            .as_ref()
            .unwrap()
            .open_department_id
            .as_deref(),
        Some("od_deleted")
    );
    assert_eq!(
        deleted
            .old_object
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .unwrap()
            .is_deleted,
        Some(true)
    );
}

#[test]
fn contact_scope_updated_uses_added_and_removed_scope() {
    let event: P2ContactScopeUpdatedV3 = serde_json::from_value(serde_json::json!({
        "added": {
            "users": [{ "open_id": "ou_added", "name": "Ada" }],
            "departments": [{ "open_department_id": "od_added", "name": "Team" }],
            "user_groups": [{ "user_group_id": "ug_1", "name": "Group", "type": 1 }]
        },
        "removed": {
            "users": [{ "open_id": "ou_removed" }]
        }
    }))
    .unwrap();
    assert_eq!(
        event.added.as_ref().unwrap().users.as_ref().unwrap()[0]
            .open_id
            .as_deref(),
        Some("ou_added")
    );
    assert_eq!(
        event.added.as_ref().unwrap().user_groups.as_ref().unwrap()[0]
            .user_group_id
            .as_deref(),
        Some("ug_1")
    );
    assert_eq!(
        event.removed.as_ref().unwrap().users.as_ref().unwrap()[0]
            .open_id
            .as_deref(),
        Some("ou_removed")
    );
}

#[test]
fn contact_misc_events_are_typed() {
    let custom: P2ContactCustomAttrEventUpdatedV3 = serde_json::from_value(serde_json::json!({
        "object": { "contact_field_key": ["field_1"], "allow_open_query": true },
        "old_object": { "allow_open_query": false }
    }))
    .unwrap();
    assert_eq!(
        custom
            .object
            .as_ref()
            .unwrap()
            .contact_field_key
            .as_ref()
            .unwrap()[0],
        "field_1"
    );
    assert_eq!(
        custom.old_object.as_ref().unwrap().allow_open_query,
        Some(false)
    );

    let enum_created: P2ContactEmployeeTypeEnumCreatedV3 =
        serde_json::from_value(serde_json::json!({
            "new_enum": { "enum_id": "enum_1", "content": "Contractor" }
        }))
        .unwrap();
    assert_eq!(
        enum_created.new_enum.as_ref().unwrap().enum_id.as_deref(),
        Some("enum_1")
    );

    let enum_active: P2ContactEmployeeTypeEnumActivedV3 =
        serde_json::from_value(serde_json::json!({
            "old_enum": { "enum_status": 0 },
            "new_enum": { "enum_status": 1 }
        }))
        .unwrap();
    assert_eq!(enum_active.new_enum.as_ref().unwrap().enum_status, Some(1));
}

#[test]
fn contact_event_structs_accept_empty_and_null_payloads() {
    let created: P2ContactUserCreatedV3 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(created.object.is_none());

    let updated: P2ContactDepartmentUpdatedV3 = serde_json::from_value(serde_json::json!({
        "object": null,
        "old_object": null
    }))
    .unwrap();
    assert!(updated.object.is_none());
    assert!(updated.old_object.is_none());
}
