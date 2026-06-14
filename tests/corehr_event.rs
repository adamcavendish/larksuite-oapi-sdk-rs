use larksuite_oapi_sdk_rs::events::corehr::{
    P2CommonDataIdUserMappingChangedV1, P2DepartmentDeletedV1, P2DepartmentUpdatedV1,
    P2EmploymentCreatedV1, P2EmploymentUpdatedV1, P2JobChangeUpdatedV1, P2OffboardingUpdatedV1,
    P2OrgRoleAuthorizationUpdatedV1, P2PersonUpdatedV1,
};

#[test]
fn corehr_v1_identity_and_department_events_are_typed() {
    let mapping: P2CommonDataIdUserMappingChangedV1 = serde_json::from_value(serde_json::json!({
        "change_type": "created",
        "id_transform_type": 1,
        "corehr_id": "corehr_1",
        "people_admin_id": "people_1",
        "feishu_id": { "open_id": "ou_user", "user_id": "user_1", "union_id": "on_user" }
    }))
    .unwrap();
    assert_eq!(mapping.change_type.as_deref(), Some("created"));
    assert_eq!(
        mapping.feishu_id.as_ref().unwrap().open_id(),
        Some("ou_user")
    );
    assert_eq!(
        mapping.feishu_id.as_ref().unwrap().user_id(),
        Some("user_1")
    );

    let updated: P2DepartmentUpdatedV1 = serde_json::from_value(serde_json::json!({
        "department_id": "dept_1",
        "field_changes": ["name"]
    }))
    .unwrap();
    assert_eq!(updated.department_id.as_deref(), Some("dept_1"));
    assert_eq!(updated.field_changes, vec!["name"]);

    let deleted: P2DepartmentDeletedV1 = serde_json::from_value(serde_json::json!({
        "department_id": "dept_1",
        "code": "D001"
    }))
    .unwrap();
    assert_eq!(deleted.code.as_deref(), Some("D001"));
}

#[test]
fn corehr_v1_employment_and_job_change_events_are_typed() {
    let created: P2EmploymentCreatedV1 = serde_json::from_value(serde_json::json!({
        "employment_id": "emp_1",
        "target_user_id": { "open_id": "ou_target" }
    }))
    .unwrap();
    assert_eq!(created.employment_id.as_deref(), Some("emp_1"));
    assert_eq!(
        created.target_user_id.as_ref().unwrap().open_id(),
        Some("ou_target")
    );

    let updated: P2EmploymentUpdatedV1 = serde_json::from_value(serde_json::json!({
        "employment_id": "emp_1",
        "target_user_id": { "user_id": "user_1" },
        "field_changes": ["department"]
    }))
    .unwrap();
    assert_eq!(
        updated.target_user_id.as_ref().unwrap().user_id(),
        Some("user_1")
    );
    assert_eq!(updated.field_changes, vec!["department"]);

    let job_change: P2JobChangeUpdatedV1 = serde_json::from_value(serde_json::json!({
        "employment_id": "emp_1",
        "target_user_id": { "union_id": "on_target" },
        "job_change_id": "jc_1",
        "transfer_mode": 1,
        "transfer_type_unique_identifier": "transfer",
        "transfer_reason_unique_identifier": "reason",
        "process_id": "process_1",
        "effective_date": "2026-01-01",
        "status": 3,
        "transfer_key": "key_1"
    }))
    .unwrap();
    assert_eq!(
        job_change.target_user_id.as_ref().unwrap().union_id(),
        Some("on_target")
    );
    assert_eq!(job_change.transfer_key.as_deref(), Some("key_1"));
}

#[test]
fn corehr_v1_offboarding_and_org_role_events_are_typed() {
    let offboarding: P2OffboardingUpdatedV1 = serde_json::from_value(serde_json::json!({
        "employment_id": "emp_1",
        "target_user_id": { "open_id": "ou_target" },
        "offboarding_id": "off_1",
        "process_id": "process_1",
        "status": 2
    }))
    .unwrap();
    assert_eq!(offboarding.offboarding_id.as_deref(), Some("off_1"));
    assert_eq!(
        offboarding.target_user_id.as_ref().unwrap().open_id(),
        Some("ou_target")
    );

    let role: P2OrgRoleAuthorizationUpdatedV1 = serde_json::from_value(serde_json::json!({
        "role_id": "role_1",
        "management_scope_list": [{ "management_dimension": "department", "obj_id": "dept_1" }],
        "employment_id_list": ["emp_1"]
    }))
    .unwrap();
    assert_eq!(role.role_id.as_deref(), Some("role_1"));
    assert_eq!(
        role.management_scope_list[0]
            .management_dimension
            .as_deref(),
        Some("department")
    );
    assert_eq!(role.employment_id_list, vec!["emp_1"]);
}

#[test]
fn corehr_v1_event_structs_accept_empty_and_null_payloads() {
    let person: P2PersonUpdatedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(person.person_id.is_none());
    assert!(person.field_changes.is_empty());

    let employment: P2EmploymentUpdatedV1 = serde_json::from_value(serde_json::json!({
        "target_user_id": null
    }))
    .unwrap();
    assert!(employment.target_user_id.is_none());
}
