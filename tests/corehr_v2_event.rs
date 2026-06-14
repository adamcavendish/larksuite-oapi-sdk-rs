use larksuite_oapi_sdk_rs::events::corehr_v2::{
    P2ApprovalGroupsUpdatedV2, P2CompanyUpdatedV2, P2EmployeeDomainEventV2,
    P2JobChangeStatusUpdatedV2, P2JobChangeUpdatedV2, P2OffboardingChecklistUpdatedV2,
    P2OffboardingUpdatedV2, P2PreHireOnboardingTaskChangedV2, P2ProcessApproverUpdatedV2,
    P2ProcessNodeUpdatedV2, P2SignatureFileStatusUpdatedV2,
};

#[test]
fn corehr_v2_org_and_approval_events_are_typed() {
    let approval: P2ApprovalGroupsUpdatedV2 = serde_json::from_value(serde_json::json!({
        "approval_group_id": "ag_1",
        "process_id": "process_1",
        "approval_group_status": 1,
        "topic": "Transfer",
        "approval_group_status_v2": 2
    }))
    .unwrap();
    assert_eq!(approval.approval_group_id.as_deref(), Some("ag_1"));
    assert_eq!(approval.approval_group_status_v2, Some(2));

    let company: P2CompanyUpdatedV2 = serde_json::from_value(serde_json::json!({
        "company_id": "company_1",
        "field_changes": ["name"],
        "sub_events": [{
            "id": "sub_1",
            "entity": "company",
            "agg_entity": "tenant",
            "agg_entity_id": "tenant_1",
            "agg_entity_field": "companies",
            "opt_type": 2,
            "field_changes": ["name"]
        }]
    }))
    .unwrap();
    assert_eq!(company.company_id.as_deref(), Some("company_1"));
    assert_eq!(company.sub_events[0].entity.as_deref(), Some("company"));
    assert_eq!(company.sub_events[0].field_changes, vec!["name"]);
}

#[test]
fn corehr_v2_employee_domain_and_job_change_events_are_typed() {
    let domain: P2EmployeeDomainEventV2 = serde_json::from_value(serde_json::json!({
        "event_type": 1,
        "sub_event_type": 2,
        "operator_user_id": "ou_operator",
        "employment_id": "emp_1",
        "data": [{
            "id": "row_1",
            "entity": "employment",
            "agg_entity": "person",
            "agg_entity_id": "person_1",
            "opt_type": 2,
            "fields": ["department"]
        }]
    }))
    .unwrap();
    assert_eq!(domain.event_type, Some(1));
    assert_eq!(domain.data[0].fields, vec!["department"]);

    let status: P2JobChangeStatusUpdatedV2 = serde_json::from_value(serde_json::json!({
        "employment_id": "emp_1",
        "target_user_id": { "open_id": "ou_target" },
        "job_change_id": "jc_1",
        "transfer_mode": 1,
        "status": 3,
        "original_status": 2,
        "details_of_job_status_change": ["approved"]
    }))
    .unwrap();
    assert_eq!(
        status.target_user_id.as_ref().unwrap().open_id(),
        Some("ou_target")
    );
    assert_eq!(status.original_status, Some(2));
    assert_eq!(status.details_of_job_status_change, vec!["approved"]);

    let updated: P2JobChangeUpdatedV2 = serde_json::from_value(serde_json::json!({
        "employment_id": "emp_1",
        "tenant_id": "tenant_1",
        "operator": "operator_1",
        "updated_time": "1710000000000",
        "job_change_id": "jc_1",
        "updated_fields": ["status"],
        "transform_type": "transfer"
    }))
    .unwrap();
    assert_eq!(updated.operator.as_deref(), Some("operator_1"));
    assert_eq!(updated.updated_fields, vec!["status"]);
}

#[test]
fn corehr_v2_offboarding_and_pre_hire_events_are_typed() {
    let checklist: P2OffboardingChecklistUpdatedV2 = serde_json::from_value(serde_json::json!({
        "employment_id": "emp_1",
        "target_user_id": { "user_id": "user_1" },
        "offboarding_id": "off_1",
        "checklist_process_id": "check_1",
        "checklist_status": 4
    }))
    .unwrap();
    assert_eq!(
        checklist.target_user_id.as_ref().unwrap().user_id(),
        Some("user_1")
    );
    assert_eq!(checklist.checklist_status, Some(4));

    let offboarding: P2OffboardingUpdatedV2 = serde_json::from_value(serde_json::json!({
        "tenant_id": "tenant_1",
        "offboarding_info_id": "off_info_1",
        "process_id": "process_1",
        "checklist_process_id": "check_1",
        "employment_id": "emp_1",
        "operator": "operator_1",
        "status": 1,
        "checklist_status": 2,
        "updated_time": "1710000000000",
        "updated_fields": ["status"],
        "target_user_id": { "union_id": "on_user" }
    }))
    .unwrap();
    assert_eq!(
        offboarding.target_user_id.as_ref().unwrap().union_id(),
        Some("on_user")
    );
    assert_eq!(offboarding.updated_fields, vec!["status"]);

    let pre_hire: P2PreHireOnboardingTaskChangedV2 = serde_json::from_value(serde_json::json!({
        "tenant_id": "tenant_1",
        "pre_hire_id": "pre_1",
        "onboarding_task_changes": [{ "after_status": "done", "task_code": "task" }],
        "onboarding_flow_change": { "after_status": "running" },
        "onboarding_flow_id": "flow_1",
        "flow_info": { "id": "flow_1", "name": { "zh_cn": "流程", "en_us": "Flow" } }
    }))
    .unwrap();
    assert_eq!(
        pre_hire.onboarding_task_changes[0].task_code.as_deref(),
        Some("task")
    );
    assert_eq!(
        pre_hire
            .flow_info
            .as_ref()
            .unwrap()
            .name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Flow")
    );
}

#[test]
fn corehr_v2_process_events_are_typed() {
    let approver: P2ProcessApproverUpdatedV2 = serde_json::from_value(serde_json::json!({
        "process_id": "process_1",
        "approver_id": "approver_1",
        "type": 1,
        "status": 2,
        "biz_type": "corehr",
        "flow_definition_id": "flow_def_1",
        "node_definition_id": "node_def_1",
        "node_id": "123",
        "node_id_str": "node_123"
    }))
    .unwrap();
    assert_eq!(approver.type_, Some(1));
    assert_eq!(approver.node_id_str.as_deref(), Some("node_123"));

    let node: P2ProcessNodeUpdatedV2 = serde_json::from_value(serde_json::json!({
        "flow_definition_id": "flow_def_1",
        "node_definition_id": "node_def_1",
        "process_id": "process_1",
        "process_node_id": "process_node_1",
        "node_type": 1,
        "node_status": 2,
        "biz_type": "corehr"
    }))
    .unwrap();
    assert_eq!(node.process_node_id.as_deref(), Some("process_node_1"));
    assert_eq!(node.node_status, Some(2));

    let signature: P2SignatureFileStatusUpdatedV2 = serde_json::from_value(serde_json::json!({
        "signature_file_id": "file_1",
        "before_status": "pending",
        "after_status": "signed",
        "biz_process_id": "process_1"
    }))
    .unwrap();
    assert_eq!(signature.after_status.as_deref(), Some("signed"));
}

#[test]
fn corehr_v2_event_structs_accept_empty_and_null_payloads() {
    let company: P2CompanyUpdatedV2 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(company.company_id.is_none());
    assert!(company.sub_events.is_empty());

    let pre_hire: P2PreHireOnboardingTaskChangedV2 = serde_json::from_value(serde_json::json!({
        "onboarding_flow_change": null,
        "flow_info": null
    }))
    .unwrap();
    assert!(pre_hire.onboarding_flow_change.is_none());
    assert!(pre_hire.flow_info.is_none());
}
