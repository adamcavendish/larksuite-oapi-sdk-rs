//! CoreHR v2 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrganizationDomainEventData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agg_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agg_entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agg_entity_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opt_type: Option<i32>,
    #[serde(default)]
    pub field_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeeDomainEventData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agg_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agg_entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opt_type: Option<i32>,
    #[serde(default)]
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OnboardingTaskChange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OnboardingFlowChange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct I18nV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OnboardingFlow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ApprovalGroupsUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjust_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group_status_v2: Option<i32>,
}

macro_rules! id_event {
    ($name:ident, $field:ident) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub $field: Option<String>,
        }
    };
}

macro_rules! id_field_changes_event {
    ($name:ident, $field:ident) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub $field: Option<String>,
            #[serde(default)]
            pub field_changes: Vec<String>,
        }
    };
}

id_event!(P2CompanyCreatedV2, company_id);
id_event!(P2CompanyDeletedV2, company_id);
id_event!(P2CostCenterCreatedV2, cost_center_id);
id_event!(P2CostCenterDeletedV2, cost_center_id);
id_event!(P2DepartmentCreatedV2, department_id);
id_event!(P2JobFamilyCreatedV2, job_family_id);
id_event!(P2JobFamilyDeletedV2, job_family_id);
id_event!(P2JobGradeCreatedV2, job_grade_id);
id_event!(P2JobGradeDeletedV2, job_grade_id);
id_event!(P2JobLevelCreatedV2, job_level_id);
id_event!(P2JobLevelDeletedV2, job_level_id);
id_event!(P2LocationCreatedV2, location_id);
id_event!(P2LocationDeletedV2, location_id);
id_event!(P2PathwayCreatedV2, pathway_id);
id_event!(P2PathwayDeletedV2, pathway_id);
id_event!(P2PositionCreatedV2, position_id);
id_event!(P2PositionDeletedV2, position_id);

id_field_changes_event!(P2CostCenterUpdatedV2, cost_center_id);
id_field_changes_event!(P2DepartmentUpdatedV2, department_id);
id_field_changes_event!(P2JobFamilyUpdatedV2, job_family_id);
id_field_changes_event!(P2JobGradeUpdatedV2, job_grade_id);
id_field_changes_event!(P2JobLevelUpdatedV2, job_level_id);
id_field_changes_event!(P2PathwayUpdatedV2, pathway_id);
id_field_changes_event!(P2PositionUpdatedV2, position_id);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CompanyUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default)]
    pub field_changes: Vec<String>,
    #[serde(default)]
    pub sub_events: Vec<OrganizationDomainEventData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2LocationUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default)]
    pub field_changes: Vec<String>,
    #[serde(default)]
    pub sub_events: Vec<OrganizationDomainEventData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CustomOrgCreatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CustomOrgDeletedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CustomOrgUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(default)]
    pub field_changes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2EmployeeDomainEventV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_event_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opt_scene: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opt_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opt_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opt_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub data: Vec<EmployeeDomainEventData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2JobChangeStatusUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_key: Option<String>,
    #[serde(default)]
    pub details_of_job_status_change: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2JobChangeUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<i32>,
    #[serde(default)]
    pub updated_fields: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2OffboardingChecklistUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2OffboardingStatusUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2OffboardingUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_info_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(default)]
    pub updated_fields: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2PreHireOnboardingTaskChangedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    #[serde(default)]
    pub onboarding_task_changes: Vec<OnboardingTaskChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_flow_change: Option<OnboardingFlowChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_flow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<OnboardingFlow>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ProbationUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ProcessUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ProcessApproverUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id_str: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ProcessCcUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ProcessNodeUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ProcessStatusUpdateV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ProcessCommentInfoUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2SignatureFileStatusUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_process_id: Option<String>,
}

event_handlers! {
    on_p2_corehr_approval_groups_updated_v2 => P2ApprovalGroupsUpdatedV2
        : "corehr.approval_groups.updated_v2",
    on_p2_corehr_company_created_v2 => P2CompanyCreatedV2
        : "corehr.company.created_v2",
    on_p2_corehr_company_deleted_v2 => P2CompanyDeletedV2
        : "corehr.company.deleted_v2",
    on_p2_corehr_company_updated_v2 => P2CompanyUpdatedV2
        : "corehr.company.updated_v2",
    on_p2_corehr_cost_center_created_v2 => P2CostCenterCreatedV2
        : "corehr.cost_center.created_v2",
    on_p2_corehr_cost_center_deleted_v2 => P2CostCenterDeletedV2
        : "corehr.cost_center.deleted_v2",
    on_p2_corehr_cost_center_updated_v2 => P2CostCenterUpdatedV2
        : "corehr.cost_center.updated_v2",
    on_p2_corehr_custom_org_created_v2 => P2CustomOrgCreatedV2
        : "corehr.custom_org.created_v2",
    on_p2_corehr_custom_org_deleted_v2 => P2CustomOrgDeletedV2
        : "corehr.custom_org.deleted_v2",
    on_p2_corehr_custom_org_updated_v2 => P2CustomOrgUpdatedV2
        : "corehr.custom_org.updated_v2",
    on_p2_corehr_department_created_v2 => P2DepartmentCreatedV2
        : "corehr.department.created_v2",
    on_p2_corehr_department_updated_v2 => P2DepartmentUpdatedV2
        : "corehr.department.updated_v2",
    on_p2_corehr_employee_domain_event_v2 => P2EmployeeDomainEventV2
        : "corehr.employee.domain_event_v2",
    on_p2_corehr_job_change_status_updated_v2 => P2JobChangeStatusUpdatedV2
        : "corehr.job_change.status_updated_v2",
    on_p2_corehr_job_change_updated_v2 => P2JobChangeUpdatedV2
        : "corehr.job_change.updated_v2",
    on_p2_corehr_job_family_created_v2 => P2JobFamilyCreatedV2
        : "corehr.job_family.created_v2",
    on_p2_corehr_job_family_deleted_v2 => P2JobFamilyDeletedV2
        : "corehr.job_family.deleted_v2",
    on_p2_corehr_job_family_updated_v2 => P2JobFamilyUpdatedV2
        : "corehr.job_family.updated_v2",
    on_p2_corehr_job_grade_created_v2 => P2JobGradeCreatedV2
        : "corehr.job_grade.created_v2",
    on_p2_corehr_job_grade_deleted_v2 => P2JobGradeDeletedV2
        : "corehr.job_grade.deleted_v2",
    on_p2_corehr_job_grade_updated_v2 => P2JobGradeUpdatedV2
        : "corehr.job_grade.updated_v2",
    on_p2_corehr_job_level_created_v2 => P2JobLevelCreatedV2
        : "corehr.job_level.created_v2",
    on_p2_corehr_job_level_deleted_v2 => P2JobLevelDeletedV2
        : "corehr.job_level.deleted_v2",
    on_p2_corehr_job_level_updated_v2 => P2JobLevelUpdatedV2
        : "corehr.job_level.updated_v2",
    on_p2_corehr_location_created_v2 => P2LocationCreatedV2
        : "corehr.location.created_v2",
    on_p2_corehr_location_deleted_v2 => P2LocationDeletedV2
        : "corehr.location.deleted_v2",
    on_p2_corehr_location_updated_v2 => P2LocationUpdatedV2
        : "corehr.location.updated_v2",
    on_p2_corehr_offboarding_checklist_updated_v2 => P2OffboardingChecklistUpdatedV2
        : "corehr.offboarding.checklist_updated_v2",
    on_p2_corehr_offboarding_status_updated_v2 => P2OffboardingStatusUpdatedV2
        : "corehr.offboarding.status_updated_v2",
    on_p2_corehr_offboarding_updated_v2 => P2OffboardingUpdatedV2
        : "corehr.offboarding.updated_v2",
    on_p2_corehr_pathway_created_v2 => P2PathwayCreatedV2
        : "corehr.pathway.created_v2",
    on_p2_corehr_pathway_deleted_v2 => P2PathwayDeletedV2
        : "corehr.pathway.deleted_v2",
    on_p2_corehr_pathway_updated_v2 => P2PathwayUpdatedV2
        : "corehr.pathway.updated_v2",
    on_p2_corehr_position_created_v2 => P2PositionCreatedV2
        : "corehr.position.created_v2",
    on_p2_corehr_position_deleted_v2 => P2PositionDeletedV2
        : "corehr.position.deleted_v2",
    on_p2_corehr_position_updated_v2 => P2PositionUpdatedV2
        : "corehr.position.updated_v2",
    on_p2_corehr_pre_hire_onboarding_task_changed_v2 => P2PreHireOnboardingTaskChangedV2
        : "corehr.pre_hire.onboarding_task_changed_v2",
    on_p2_corehr_probation_updated_v2 => P2ProbationUpdatedV2
        : "corehr.probation.updated_v2",
    on_p2_corehr_process_approver_updated_v2 => P2ProcessApproverUpdatedV2
        : "corehr.process.approver.updated_v2",
    on_p2_corehr_process_cc_updated_v2 => P2ProcessCcUpdatedV2
        : "corehr.process.cc.updated_v2",
    on_p2_corehr_process_node_updated_v2 => P2ProcessNodeUpdatedV2
        : "corehr.process.node.updated_v2",
    on_p2_corehr_process_status_update_v2 => P2ProcessStatusUpdateV2
        : "corehr.process.status.update_v2",
    on_p2_corehr_process_updated_v2 => P2ProcessUpdatedV2
        : "corehr.process.updated_v2",
    on_p2_corehr_process_comment_info_updated_v2 => P2ProcessCommentInfoUpdatedV2
        : "corehr.process_comment_info.updated_v2",
    on_p2_corehr_signature_file_status_updated_v2 => P2SignatureFileStatusUpdatedV2
        : "corehr.signature_file.status_updated_v2",
}
