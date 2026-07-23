use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Generic response data types ───────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ItemData {
    #[serde(flatten)]
    pub data: crate::JsonValue,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListData {
    #[serde(default)]
    pub items: Vec<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

macro_rules! recent_change_data {
    ($name:ident, $ids:ident, $deleted_ids:ident) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            #[serde(default)]
            pub $ids: Vec<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub page_token: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub has_more: Option<bool>,
            #[serde(default)]
            pub $deleted_ids: Vec<String>,
        }
    };
}

recent_change_data!(
    QueryRecentChangeCompanyV2RespData,
    company_ids,
    deleted_company_ids
);
recent_change_data!(
    QueryRecentChangeCostCenterV2RespData,
    cost_center_ids,
    deleted_cost_center_ids
);
recent_change_data!(
    QueryRecentChangeCustomOrgV2RespData,
    custom_org_ids,
    deleted_custom_org_ids
);
recent_change_data!(
    QueryRecentChangeDepartmentV2RespData,
    department_ids,
    deleted_department_ids
);
recent_change_data!(QueryRecentChangeJobV2RespData, job_ids, deleted_job_ids);
recent_change_data!(
    QueryRecentChangeJobFamilyV2RespData,
    job_family_ids,
    deleted_job_family_ids
);
recent_change_data!(
    QueryRecentChangeJobGradeV2RespData,
    job_grade_ids,
    deleted_job_grade_ids
);
recent_change_data!(
    QueryRecentChangeJobLevelV2RespData,
    job_level_ids,
    deleted_job_level_ids
);
recent_change_data!(
    QueryRecentChangeLocationV2RespData,
    location_ids,
    deleted_location_ids
);
recent_change_data!(
    QueryRecentChangePositionV2RespData,
    position_ids,
    deleted_position_ids
);

// ── Response types ─────────────────────────────────────────────────────────────

// approval_groups
impl_resp_v2!(GetApprovalGroupsV2Resp, GetApprovalGroupsV2RespData);
impl_resp_v2!(OpenQueryDeptChangeApprovalGroupsV2Resp, ());
impl_resp_v2!(OpenQueryJobChangeApprovalGroupsV2Resp, ());
impl_resp_v2!(OpenQueryPositionChangeApprovalGroupsV2Resp, ());
// approver
impl_resp_v2!(ListApproverV2Resp, ListData);
// basic_info
impl_resp_v2!(SearchBankV2Resp, ListData);
impl_resp_v2!(SearchBankBranchV2Resp, ListData);
impl_resp_v2!(SearchCityV2Resp, ListData);
impl_resp_v2!(SearchCountryRegionV2Resp, ListData);
impl_resp_v2!(SearchCountryRegionSubdivisionV2Resp, ListData);
impl_resp_v2!(SearchCurrencyV2Resp, ListData);
impl_resp_v2!(SearchDistrictV2Resp, ListData);
impl_resp_v2!(SearchLanguageV2Resp, ListData);
impl_resp_v2!(SearchNationalityV2Resp, ListData);
impl_resp_v2!(SearchTimeZoneV2Resp, ListData);
// bp
impl_resp_v2!(GetBpByDepartmentV2Resp, ());
impl_resp_v2!(ListBpV2Resp, ListData);
// company
impl_resp_v2!(ActiveCompanyV2Resp, ());
impl_resp_v2!(BatchGetCompanyV2Resp, BatchGetCompanyV2RespData);
impl_resp_v2!(
    QueryRecentChangeCompanyV2Resp,
    QueryRecentChangeCompanyV2RespData
);
// contract
impl_resp_v2!(SearchContractV2Resp, ListData);
// cost_allocation
impl_resp_v2!(
    BatchQueryCostAllocationV2Resp,
    BatchQueryCostAllocationV2RespData
);
impl_resp_v2!(
    CreateVersionCostAllocationV2Resp,
    CreateVersionCostAllocationV2RespData
);
impl_resp_v2!(RemoveVersionCostAllocationV2Resp, ());
impl_resp_v2!(UpdateVersionCostAllocationV2Resp, ());
// cost_center
impl_resp_v2!(CreateCostCenterV2Resp, CreateCostCenterV2RespData);
impl_resp_v2!(DeleteCostCenterV2Resp, ());
impl_resp_v2!(PatchCostCenterV2Resp, PatchCostCenterV2RespData);
impl_resp_v2!(
    QueryRecentChangeCostCenterV2Resp,
    QueryRecentChangeCostCenterV2RespData
);
impl_resp_v2!(SearchCostCenterV2Resp, ListData);
impl_resp_v2!(
    CreateCostCenterVersionV2Resp,
    CreateCostCenterVersionV2RespData
);
impl_resp_v2!(DeleteCostCenterVersionV2Resp, ());
impl_resp_v2!(
    PatchCostCenterVersionV2Resp,
    PatchCostCenterVersionV2RespData
);
// custom_org
impl_resp_v2!(ActiveCustomOrgV2Resp, ());
impl_resp_v2!(CreateCustomOrgV2Resp, CreateCustomOrgV2RespData);
impl_resp_v2!(DeleteOrgCustomOrgV2Resp, ());
impl_resp_v2!(PatchCustomOrgV2Resp, ());
impl_resp_v2!(QueryCustomOrgV2Resp, QueryCustomOrgV2RespData);
impl_resp_v2!(
    QueryRecentChangeCustomOrgV2Resp,
    QueryRecentChangeCustomOrgV2RespData
);
impl_resp_v2!(UpdateRuleCustomOrgV2Resp, ());
// default_cost_center
impl_resp_v2!(
    BatchQueryDefaultCostCenterV2Resp,
    BatchQueryDefaultCostCenterV2RespData
);
impl_resp_v2!(
    CreateVersionDefaultCostCenterV2Resp,
    CreateVersionDefaultCostCenterV2RespData
);
impl_resp_v2!(RemoveVersionDefaultCostCenterV2Resp, ());
impl_resp_v2!(UpdateVersionDefaultCostCenterV2Resp, ());
// department
impl_resp_v2!(BatchGetDepartmentV2Resp, BatchGetDepartmentV2RespData);
impl_resp_v2!(DeleteDepartmentV2Resp, ());
impl_resp_v2!(ParentsDepartmentV2Resp, ParentsDepartmentV2RespData);
impl_resp_v2!(PatchDepartmentV2Resp, ());
impl_resp_v2!(
    QueryMultiTimelineDepartmentV2Resp,
    QueryMultiTimelineDepartmentV2RespData
);
impl_resp_v2!(
    QueryOperationLogsDepartmentV2Resp,
    QueryOperationLogsDepartmentV2RespData
);
impl_resp_v2!(
    QueryRecentChangeDepartmentV2Resp,
    QueryRecentChangeDepartmentV2RespData
);
impl_resp_v2!(
    QueryTimelineDepartmentV2Resp,
    QueryTimelineDepartmentV2RespData
);
impl_resp_v2!(SearchDepartmentV2Resp, ListData);
impl_resp_v2!(TreeDepartmentV2Resp, TreeDepartmentV2RespData);
// draft
impl_resp_v2!(GetDraftV2Resp, GetDraftV2RespData);
// employee
impl_resp_v2!(BatchGetEmployeeV2Resp, BatchGetEmployeeV2RespData);
impl_resp_v2!(CreateEmployeeV2Resp, CreateEmployeeV2RespData);
impl_resp_v2!(SearchEmployeeV2Resp, ListData);
// employees_additional_job
impl_resp_v2!(
    BatchEmployeesAdditionalJobV2Resp,
    BatchEmployeesAdditionalJobV2RespData
);
impl_resp_v2!(
    CreateEmployeesAdditionalJobV2Resp,
    CreateEmployeesAdditionalJobV2RespData
);
impl_resp_v2!(DeleteEmployeesAdditionalJobV2Resp, ());
impl_resp_v2!(
    PatchEmployeesAdditionalJobV2Resp,
    PatchEmployeesAdditionalJobV2RespData
);
// employees_bp
impl_resp_v2!(BatchGetEmployeesBpV2Resp, BatchGetEmployeesBpV2RespData);
// employees_international_assignment
impl_resp_v2!(CreateEmployeesIntlAssignmentV2Resp, ());
impl_resp_v2!(DeleteEmployeesIntlAssignmentV2Resp, ());
impl_resp_v2!(ListEmployeesIntlAssignmentV2Resp, ListData);
impl_resp_v2!(PatchEmployeesIntlAssignmentV2Resp, ());
// employees_job_data
impl_resp_v2!(
    BatchGetEmployeesJobDataV2Resp,
    BatchGetEmployeesJobDataV2RespData
);
impl_resp_v2!(QueryEmployeesJobDataV2Resp, QueryEmployeesJobDataV2RespData);
// enum
impl_resp_v2!(SearchEnumV2Resp, ListData);
// job
impl_resp_v2!(BatchGetJobV2Resp, BatchGetJobV2RespData);
impl_resp_v2!(GetJobV2Resp, GetJobV2RespData);
impl_resp_v2!(ListJobV2Resp, ListData);
impl_resp_v2!(QueryMultiTimelineJobV2Resp, QueryMultiTimelineJobV2RespData);
impl_resp_v2!(QueryRecentChangeJobV2Resp, QueryRecentChangeJobV2RespData);
// job_change
impl_resp_v2!(CreateJobChangeV2Resp, CreateJobChangeV2RespData);
impl_resp_v2!(RevokeJobChangeV2Resp, RevokeJobChangeV2RespData);
impl_resp_v2!(SearchJobChangeV2Resp, ListData);
// job_family
impl_resp_v2!(BatchGetJobFamilyV2Resp, BatchGetJobFamilyV2RespData);
impl_resp_v2!(
    QueryMultiTimelineJobFamilyV2Resp,
    QueryMultiTimelineJobFamilyV2RespData
);
impl_resp_v2!(
    QueryRecentChangeJobFamilyV2Resp,
    QueryRecentChangeJobFamilyV2RespData
);
// job_grade
impl_resp_v2!(CreateJobGradeV2Resp, CreateJobGradeV2RespData);
impl_resp_v2!(DeleteJobGradeV2Resp, ());
impl_resp_v2!(PatchJobGradeV2Resp, ());
impl_resp_v2!(QueryJobGradeV2Resp, QueryJobGradeV2RespData);
impl_resp_v2!(
    QueryRecentChangeJobGradeV2Resp,
    QueryRecentChangeJobGradeV2RespData
);
// job_level
impl_resp_v2!(BatchGetJobLevelV2Resp, BatchGetJobLevelV2RespData);
impl_resp_v2!(
    QueryRecentChangeJobLevelV2Resp,
    QueryRecentChangeJobLevelV2RespData
);
// location
impl_resp_v2!(ActiveLocationV2Resp, ());
impl_resp_v2!(BatchGetLocationV2Resp, BatchGetLocationV2RespData);
impl_resp_v2!(PatchLocationV2Resp, ());
impl_resp_v2!(
    QueryRecentChangeLocationV2Resp,
    QueryRecentChangeLocationV2RespData
);
impl_resp_v2!(CreateLocationAddressV2Resp, CreateLocationAddressV2RespData);
impl_resp_v2!(DeleteLocationAddressV2Resp, ());
impl_resp_v2!(PatchLocationAddressV2Resp, ());
// offboarding
impl_resp_v2!(EditOffboardingV2Resp, EditOffboardingV2RespData);
impl_resp_v2!(RevokeOffboardingV2Resp, ());
impl_resp_v2!(SubmitOffboardingV2Resp, ());
// pathway
impl_resp_v2!(ActivePathwayV2Resp, ());
impl_resp_v2!(BatchGetPathwayV2Resp, BatchGetPathwayV2RespData);
impl_resp_v2!(CreatePathwayV2Resp, CreatePathwayV2RespData);
impl_resp_v2!(DeletePathwayV2Resp, ());
impl_resp_v2!(PatchPathwayV2Resp, ());
// person
impl_resp_v2!(CreatePersonV2Resp, CreatePersonV2RespData);
impl_resp_v2!(PatchPersonV2Resp, PatchPersonV2RespData);
// position
impl_resp_v2!(ActivePositionV2Resp, ());
impl_resp_v2!(CreatePositionV2Resp, CreatePositionV2RespData);
impl_resp_v2!(DelPositionV2Resp, ());
impl_resp_v2!(PatchPositionV2Resp, ());
impl_resp_v2!(QueryPositionV2Resp, QueryPositionV2RespData);
impl_resp_v2!(
    QueryRecentChangePositionV2Resp,
    QueryRecentChangePositionV2RespData
);
// pre_hire
impl_resp_v2!(CompletePreHireV2Resp, CompletePreHireV2RespData);
impl_resp_v2!(CreatePreHireV2Resp, CreatePreHireV2RespData);
impl_resp_v2!(DeletePreHireV2Resp, ());
impl_resp_v2!(PatchPreHireV2Resp, PatchPreHireV2RespData);
impl_resp_v2!(QueryPreHireV2Resp, QueryPreHireV2RespData);
impl_resp_v2!(
    RestoreFlowInstancePreHireV2Resp,
    RestoreFlowInstancePreHireV2RespData
);
impl_resp_v2!(SearchPreHireV2Resp, ListData);
impl_resp_v2!(
    TransformOnboardingTaskPreHireV2Resp,
    TransformOnboardingTaskPreHireV2RespData
);
impl_resp_v2!(TransitTaskPreHireV2Resp, TransitTaskPreHireV2RespData);
impl_resp_v2!(
    WithdrawOnboardingPreHireV2Resp,
    WithdrawOnboardingPreHireV2RespData
);
// probation
impl_resp_v2!(EnableDisableAssessmentProbationV2Resp, ());
impl_resp_v2!(SearchProbationV2Resp, ListData);
impl_resp_v2!(SubmitProbationV2Resp, SubmitProbationV2RespData);
impl_resp_v2!(WithdrawProbationV2Resp, ());
impl_resp_v2!(
    CreateProbationAssessmentV2Resp,
    CreateProbationAssessmentV2RespData
);
impl_resp_v2!(DeleteProbationAssessmentV2Resp, ());
impl_resp_v2!(PatchProbationAssessmentV2Resp, ());
// process
impl_resp_v2!(
    FlowVariableDataProcessV2Resp,
    FlowVariableDataProcessV2RespData
);
impl_resp_v2!(GetProcessV2Resp, GetProcessV2RespData);
impl_resp_v2!(ListProcessV2Resp, ListData);
impl_resp_v2!(UpdateProcessApproverV2Resp, UpdateProcessApproverV2RespData);
impl_resp_v2!(UpdateProcessExtraV2Resp, ());
impl_resp_v2!(
    GetProcessFormVariableDataV2Resp,
    GetProcessFormVariableDataV2RespData
);
impl_resp_v2!(UpdateProcessTransferV2Resp, ());
impl_resp_v2!(UpdateProcessRevokeV2Resp, ());
impl_resp_v2!(UpdateProcessWithdrawV2Resp, ());
// report_detail_row
impl_resp_v2!(BatchDeleteReportDetailRowV2Resp, ());
impl_resp_v2!(BatchSaveReportDetailRowV2Resp, ());
// signature_file
impl_resp_v2!(DownloadSignatureFileV2Resp, ());
impl_resp_v2!(ListSignatureFileV2Resp, ListData);
impl_resp_v2!(ListByBizIdSignatureFileV2Resp, ListData);
impl_resp_v2!(QuerySignatureFileV2Resp, QuerySignatureFileV2RespData);
impl_resp_v2!(
    TerminateSignatureFileV2Resp,
    TerminateSignatureFileV2RespData
);
impl_resp_v2!(ListByFileIdSignatureNodeV2Resp, ListData);
impl_resp_v2!(SearchSignatureTemplateV2Resp, ListData);
impl_resp_v2!(ListSignatureTemplateInfoWithThumbnailV2Resp, ListData);
// workforce_plan
impl_resp_v2!(ListWorkforcePlanV2Resp, ListData);
impl_resp_v2!(
    BatchWorkforcePlanDetailV2Resp,
    BatchWorkforcePlanDetailV2RespData
);
impl_resp_v2!(
    BatchV2WorkforcePlanDetailV2Resp,
    BatchV2WorkforcePlanDetailV2RespData
);
impl_resp_v2!(BatchDeleteWorkforcePlanDetailRowV2Resp, ());
impl_resp_v2!(BatchSaveWorkforcePlanDetailRowV2Resp, ());

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetApprovalGroupsV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group: Option<ApprovalGroup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetCompanyV2RespData {
    #[serde(default)]
    pub items: Vec<Company>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchQueryCostAllocationV2RespData {
    #[serde(default)]
    pub items: Vec<EmployeeCostAllocation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateVersionCostAllocationV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_allocation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateCostCenterV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<CostCenter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatchCostCenterV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<CostCenter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateCostCenterVersionV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CostCenterVersion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatchCostCenterVersionV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CostCenterVersion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateCustomOrgV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCustomOrgV2RespData {
    #[serde(default)]
    pub items: Vec<CustomOrg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchQueryDefaultCostCenterV2RespData {
    #[serde(default)]
    pub items: Vec<EmployeeDefaultCostCenter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateVersionDefaultCostCenterV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_tid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetDepartmentV2RespData {
    #[serde(default)]
    pub items: Vec<Department>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ParentsDepartmentV2RespData {
    #[serde(default)]
    pub items: Vec<DepartmentParents>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_export: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMultiTimelineDepartmentV2RespData {
    #[serde(default)]
    pub items: Vec<DepartmentTimeline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryOperationLogsDepartmentV2RespData {
    #[serde(default)]
    pub op_logs: Vec<OrganizationOpLog>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryTimelineDepartmentV2RespData {
    #[serde(default)]
    pub items: Vec<DepartmentTimeline>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TreeDepartmentV2RespData {
    #[serde(default)]
    pub items: Vec<DepartmentTree>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_export: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDraftV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_status: Option<String>,
    #[serde(default)]
    pub process_infos: Vec<ProcessInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetEmployeeV2RespData {
    #[serde(default)]
    pub items: Vec<Employee>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateEmployeeV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchEmployeesAdditionalJobV2RespData {
    #[serde(default)]
    pub items: Vec<EmployeesAdditionalJob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateEmployeesAdditionalJobV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_job: Option<EmployeesAdditionalJobWriteResp>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatchEmployeesAdditionalJobV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_job: Option<EmployeesAdditionalJobWriteResp>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetEmployeesBpV2RespData {
    #[serde(default)]
    pub employment_direct_bps: Vec<EmploymentBp>,
    #[serde(default)]
    pub employment_all_bps: Vec<EmploymentBp>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetEmployeesJobDataV2RespData {
    #[serde(default)]
    pub items: Vec<EmployeeJobData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryEmployeesJobDataV2RespData {
    #[serde(default)]
    pub items: Vec<EmployeeJobData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetJobV2RespData {
    #[serde(default)]
    pub items: Vec<Job>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetJobV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMultiTimelineJobV2RespData {
    #[serde(default)]
    pub items: Vec<JobTimeline>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateJobChangeV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_info: Option<TransferInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_adjust_salary: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default)]
    pub details_of_job_status_change: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_name: Option<EmploymentLookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception_status: Option<String>,
    #[serde(default)]
    pub no_permission_fields: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RevokeJobChangeV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetJobFamilyV2RespData {
    #[serde(default)]
    pub items: Vec<JobFamily>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMultiTimelineJobFamilyV2RespData {
    #[serde(default)]
    pub items: Vec<JobFamilyTimeline>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateJobGradeV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryJobGradeV2RespData {
    #[serde(default)]
    pub items: Vec<JobGrade>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetJobLevelV2RespData {
    #[serde(default)]
    pub items: Vec<JobLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetLocationV2RespData {
    #[serde(default)]
    pub items: Vec<Location>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateLocationAddressV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EditOffboardingV2RespData {
    #[serde(default)]
    pub data: Vec<ObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchGetPathwayV2RespData {
    #[serde(default)]
    pub items: Vec<Pathway>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatePathwayV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatePersonV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatchPersonV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatePositionV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPositionV2RespData {
    #[serde(default)]
    pub items: Vec<Position>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompletePreHireV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatePreHireV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatchPreHireV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPreHireV2RespData {
    #[serde(default)]
    pub items: Vec<PreHire>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RestoreFlowInstancePreHireV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransformOnboardingTaskPreHireV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransitTaskPreHireV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WithdrawOnboardingPreHireV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitProbationV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_info: Option<ProbationInfoForSubmit>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateProbationAssessmentV2RespData {
    #[serde(default)]
    pub assessment_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FlowVariableDataProcessV2RespData {
    #[serde(default)]
    pub field_variable_values: Vec<FieldVariableValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetProcessV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_template_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_definition_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_links: Option<ProcessLink>,
    #[serde(default)]
    pub abstracts: Vec<ProcessAbstractItem>,
    #[serde(default)]
    pub todos: Vec<ProcessTodoItem>,
    #[serde(default)]
    pub cc_list: Vec<ProcessCcItem>,
    #[serde(default)]
    pub done_list: Vec<ProcessDoneItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<i32>,
    #[serde(default)]
    pub system_todos: Vec<ProcessSystemTodoItem>,
    #[serde(default)]
    pub system_done_list: Vec<ProcessSystemDoneItem>,
    #[serde(default)]
    pub comment_infos: Vec<ProcessCommentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_last_completed_correct_process: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_name: Option<DataengineI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateProcessApproverV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetProcessFormVariableDataV2RespData {
    #[serde(default)]
    pub field_variable_values: Vec<FieldVariableValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySignatureFileV2RespData {
    #[serde(default)]
    pub items: Vec<SignatureFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TerminateSignatureFileV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_count: Option<i32>,
    #[serde(default)]
    pub success_file_id_list: Vec<String>,
    #[serde(default)]
    pub fail_file_id_and_reasons: Vec<TerminateSignatureFailIdAndReason>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchWorkforcePlanDetailV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub centralized_reporting_project_id: Option<String>,
    #[serde(default)]
    pub items: Vec<WorkforcePlanDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchV2WorkforcePlanDetailV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub centralized_reporting_project_id: Option<String>,
    #[serde(default)]
    pub items: Vec<WorkforcePlanDetailV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_local_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_western_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line4: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line5: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line6: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line7: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line8: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line9: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line4: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line5: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line6: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line7: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line8: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line9: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default)]
    pub address_type_list: Vec<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_subdivision_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_subdivision_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_subdivision_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_subdivision_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_city_text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApprovalGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group_status_v2: Option<i32>,
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
    pub draft_status: Option<String>,
    #[serde(default)]
    pub department_changes: Vec<String>,
    #[serde(default)]
    pub job_changes: Vec<String>,
    #[serde(default)]
    pub position_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_identification_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default)]
    pub bank_account_usage: Vec<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BasicDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_v2: Option<String>,
    #[serde(default)]
    pub department_name: Vec<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BasicEmployee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_info: Option<BasicPersonInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BasicJobData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BasicPersonInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_local_full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_english_full_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CitizenshipStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default)]
    pub citizenship_status: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_order: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Company {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<HiberarchyCommon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Enum>,
    #[serde(default)]
    pub industry_list: Vec<Enum>,
    #[serde(default)]
    pub legal_representative: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    #[serde(default)]
    pub sub_type_list: Vec<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_company: Option<bool>,
    #[serde(default)]
    pub primary_manager: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<PhoneNumberAndAreaCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<PhoneNumberAndAreaCode>,
    #[serde(default)]
    pub registered_office_address: Vec<I18n>,
    #[serde(default)]
    pub office_address: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_office_address_info: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address_info: Option<Address>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostAllocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub cost_center_rates: Vec<JobDataCostCenter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostCenter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_version_id: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_cost_center_id: Option<String>,
    #[serde(default)]
    pub managers: Vec<String>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_prefer_manual_encoding: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CostCenterVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_cost_center_id: Option<String>,
    #[serde(default)]
    pub managers: Vec<String>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Currency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default)]
    pub country_region_id_list: Vec<String>,
    #[serde(default)]
    pub currency_name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_alpha_3_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomOrg {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
    #[serde(default)]
    pub names: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub manager_ids: Vec<String>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default)]
    pub org_roles: Vec<OrgRole>,
    #[serde(default)]
    pub match_rule_groups: Vec<MatchRules>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataengineI18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultCostCenter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_herit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherit_source: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Department {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default)]
    pub department_name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepartmentLookupName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepartmentParentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default)]
    pub department_name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepartmentParents {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default)]
    pub parent_department_list: Vec<DepartmentParentInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepartmentTimeline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default)]
    pub names: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub descriptions: Vec<I18n>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepartmentTree {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(default)]
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Dependent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id_v2: Option<String>,
    #[serde(default)]
    pub national_id_list: Vec<NationalId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spouses_working_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_covered_by_health_insurance: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_allowed_for_tax_deduction: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<Phone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default)]
    pub birth_certificate_of_child: Vec<File>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_date: Option<String>,
    #[serde(default)]
    pub visas: Vec<Visa>,
    #[serde(default)]
    pub passports: Vec<Passport>,
    #[serde(default)]
    pub citizenship_statuses: Vec<CitizenshipStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DimensionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DimensionInfoData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_info: Option<DimensionInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Education {
    #[serde(default)]
    pub school: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_of_education: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub field_of_study: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school_name: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study_name: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_end_date: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Email {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_usage: Option<Enum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmergencyContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Enum>,
    #[serde(default)]
    pub phone_ist: Vec<Phone>,
    #[serde(default)]
    pub phone_list: Vec<Phone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Employee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prehire_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<EmployeeJobLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway: Option<EmployeePathway>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family: Option<EmployeeJobFamily>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenure: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_employment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_probation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_offboarding: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default)]
    pub work_email_list: Vec<WorkEmail>,
    #[serde(default)]
    pub cost_center_list: Vec<JobDataCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire_employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_info: Option<PersonInfo>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noncompete_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub past_offboarding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_employee_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub times_employed: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recruitment_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_contract_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_expected_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_pay_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<BasicDepartment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager: Option<BasicEmployee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager: Option<BasicEmployee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_international_assignment: Option<InternationalAssignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<Enum>,
    #[serde(default)]
    pub talent_pool_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_org: Option<String>,
    #[serde(default)]
    pub seniority_adjust_information_list: Vec<SeniorityAdjustInformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_direct_bps: Option<EmploymentBp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_all_bps: Option<EmploymentBp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_cpst_plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendance_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_with_headcount_or_not: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_direct_leader: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_dotted_leader: Option<bool>,
    #[serde(default)]
    pub company_talent_pool_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeeCostAllocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub cost_allocations: Vec<EmploymentCostAllocation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeeDefaultCostCenter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub default_cost_centers: Vec<EmploymentDefaultCostCenter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeeJobData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub job_datas: Vec<JobData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeeJobFamily {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeeJobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeePathway {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeesAdditionalJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmployeesAdditionalJobWriteResp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmploymentBp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub hrbp_ids: Vec<String>,
    #[serde(default)]
    pub location_bp_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmploymentCostAllocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub job_data_cost_center_id: Vec<JobDataCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<JobDataId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmploymentDefaultCostCenter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inherit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherit_source: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_created_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmploymentLookupName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Enum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    #[serde(default)]
    pub display: Vec<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldVariableSubVlaue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<FieldVariableValueTo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldVariableValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_name: Option<FieldVariableValueI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_value: Option<FieldVariableValueTo>,
    #[serde(default)]
    pub sub_values: Vec<FieldVariableSubVlaue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldVariableValueI18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldVariableValueTo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<FieldVariableValueI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FieldVariableValueToObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_value: Option<String>,
    #[serde(default)]
    pub record_values: Vec<FieldVariableValueToRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_value: Option<String>,
    #[serde(default)]
    pub list_values: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_value: Option<FieldVariableValueToFile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldVariableValueToFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldVariableValueToObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_api_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldVariableValueToRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_value_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct File {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HiberarchyCommon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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
pub struct InternationalAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_city_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_country_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_reason: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_assignment_process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_assignment_process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_assignment_type: Option<Enum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Job {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub job_title: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default)]
    pub job_family_id_list: Vec<String>,
    #[serde(default)]
    pub job_level_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ObjectFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_job_data: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_start_reason: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_outcome: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager: Option<BasicJobData>,
    #[serde(default)]
    pub dotted_line_managers: Vec<BasicJobData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_direct_manager: Option<BasicJobData>,
    #[serde(default)]
    pub cost_center_rates: Vec<JobDataCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours_v2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_reason: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobDataCostCenter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_rate: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobDataId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobFamily {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selectable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub pathway_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobFamilyTimeline {
    #[serde(default)]
    pub job_family_version_data: Vec<JobFamilyVersionData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobFamilyVersionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_version_id: Option<String>,
    #[serde(default)]
    pub job_family_names: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub descriptions: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selectable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
    #[serde(default)]
    pub pathway_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobGrade {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub names: Vec<I18n>,
    #[serde(default)]
    pub descriptions: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default)]
    pub job_grade: Vec<String>,
    #[serde(default)]
    pub pathway_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_order: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobTimeline {
    #[serde(default)]
    pub job_version_data: Vec<JobVersionData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JobVersionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_version_id: Option<String>,
    #[serde(default)]
    pub job_names: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub descriptions: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub job_titles: Vec<I18n>,
    #[serde(default)]
    pub job_family_ids: Vec<String>,
    #[serde(default)]
    pub job_level_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<HiberarchyCommon>,
    #[serde(default)]
    pub location_usage_list: Vec<Enum>,
    #[serde(default)]
    pub address: Vec<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_language_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LookupName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MatchRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default)]
    pub right_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MatchRules {
    #[serde(default)]
    pub match_rules: Vec<MatchRule>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NationalId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_for_a_long_time: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Nationality {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_2_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_3_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NoticePeriodDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OnboardingTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OperationLogEntityField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<OperationLogEntityFieldExt>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OperationLogEntityFieldExt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    #[serde(default)]
    pub employment_ids: Vec<String>,
    #[serde(default)]
    pub inherit_employment_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrganizationOpLog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(default)]
    pub changes: Vec<OperationLogEntityField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_reason: Option<String>,
    #[serde(default)]
    pub change_reasons: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgdraftDepartmentId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Passport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passport_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Pathway {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub names: Vec<I18n>,
    #[serde(default)]
    pub descriptions: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PersonForCountry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub religion: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ethnicity_race: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PersonInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_local_full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_english_full_name: Option<String>,
    #[serde(default)]
    pub name_list: Vec<PersonName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id_v2: Option<String>,
    #[serde(default)]
    pub additional_nationalities: Vec<Nationality>,
    #[serde(default)]
    pub citizenship_status: Vec<CitizenshipStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub race: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<Enum>,
    #[serde(default)]
    pub phone_list: Vec<Phone>,
    #[serde(default)]
    pub address_list: Vec<Address>,
    #[serde(default)]
    pub email_list: Vec<Email>,
    #[serde(default)]
    pub work_experience_list: Vec<WorkExperienceInfo>,
    #[serde(default)]
    pub education_list: Vec<Education>,
    #[serde(default)]
    pub bank_account_list: Vec<BankAccount>,
    #[serde(default)]
    pub national_id_list: Vec<NationalId>,
    #[serde(default)]
    pub dependent_list: Vec<Dependent>,
    #[serde(default)]
    pub emergency_contact_list: Vec<EmergencyContact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_entered_workforce: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_years: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_image_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_level_of_education: Option<Education>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_degree_of_education: Option<Education>,
    #[serde(default)]
    pub personal_profile: Vec<PersonalProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hukou_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hukou_location: Option<String>,
    #[serde(default)]
    pub political_affiliations: Vec<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_address: Option<String>,
    #[serde(default)]
    pub person_info_chns: Vec<PersonInfoChn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub born_country_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_card_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_martyr_family: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub martyr_card_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_old_alone: Option<bool>,
    #[serde(default)]
    pub resident_taxes: Vec<ResidentTax>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_entry_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub religion: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_years_v2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passport_number: Option<String>,
    #[serde(default)]
    pub former_employer: Vec<I18n>,
    #[serde(default)]
    pub legal_name_v2s: Vec<PersonName>,
    #[serde(default)]
    pub national_id_v2s: Vec<NationalId>,
    #[serde(default)]
    pub visas: Vec<Visa>,
    #[serde(default)]
    pub passports: Vec<Passport>,
    #[serde(default)]
    pub person_for_countries: Vec<PersonForCountry>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PersonInfoChn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hukou_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hukou_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_entry_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
    #[serde(default)]
    pub political_affiliations: Vec<Enum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PersonName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hereditary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_local_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_primary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_middle_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_secondary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_and_western_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_western_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PersonalProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile_type: Option<Enum>,
    #[serde(default)]
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Phone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_area_code: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_usage: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PhoneNumberAndAreaCode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_code: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Position {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub names: Vec<I18n>,
    #[serde(default)]
    pub descriptions: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub job_family_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    pub job_level_id_list: Vec<String>,
    #[serde(default)]
    pub employee_type_id_list: Vec<String>,
    #[serde(default)]
    pub job_grade_id_list: Vec<String>,
    #[serde(default)]
    pub work_location_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_leader_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_key_position: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreHire {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_info: Option<PersonInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_info: Option<PreHireEmploymentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_info: Option<PreHireOnboardingInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_info: Option<PreHireProbationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_info: Option<PreHireContractInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people_fields_json: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreHireAbnormalReason {
    #[serde(default)]
    pub descriptions: Vec<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreHireContractInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_type: Option<String>,
    #[serde(default)]
    pub contract_file_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreHireEmploymentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default)]
    pub cost_center_rates: Vec<JobDataCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_station: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_leader_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_security_city_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_compete_covenant: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire_employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours_v2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspected_rehiring: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_worker: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_manual_updated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_group: Option<PreHirePayGroupInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whether_the_information_is_abnormal: Option<bool>,
    #[serde(default)]
    pub abnormal_reason: Vec<PreHireAbnormalReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_offer_salary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recruitment_project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_package_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_supplier_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_complete_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_order_status: Option<Enum>,
    #[serde(default)]
    pub seniority_adjust_information_list: Vec<PrehireSeniorityAdjustInformationQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_period_positive_voluntary: Option<NoticePeriodDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_period_probation_involuntary: Option<NoticePeriodDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_period_positive_involuntary: Option<NoticePeriodDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_period_probation_voluntary: Option<NoticePeriodDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_manual_updated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_over_due: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_completed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_graduate_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_cost_center: Option<DefaultCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_allocation: Option<CostAllocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reuse_feishu_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reused_feishu_account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_country_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreHireOnboardingInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_hr_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_hr_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recruitment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_sponsored_visa: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<String>,
    #[serde(default)]
    pub onboarding_task_list: Vec<OnboardingTask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_address: Option<Address>,
    #[serde(default)]
    pub flow_name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_method: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawn_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreHirePayGroupInfo {
    #[serde(default)]
    pub name: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreHireProbationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PrehireSeniorityAdjustInformationQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_adjustment: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_adjustment_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons_for_seniority_adjustment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProbationInfoForSubmit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiating_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_review: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub converted_via_bpm: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_assessment_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_assessment_result: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_assessment_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_assessment_grade: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_assessment_comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_assessment_detail: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessAbstractItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<DataengineI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessCcItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessCommentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commentor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commentor_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_msg: Option<String>,
    #[serde(default)]
    pub at_user_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessDoneItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_opinion: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_group_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessSystemDoneItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_opinion: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessSystemTodoItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProcessTodoItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<DataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResidentTax {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year_resident_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resident_status: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_country_region_id: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SeniorityAdjustInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_adjustment_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_adjustment: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons_for_seniority_adjustment: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SignatureFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_id: Option<String>,
    #[serde(default)]
    pub names: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_state: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TerminateSignatureFailIdAndReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TranferEmploymentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_employee_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransferInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager_clean: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_exist: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_draft_department: Option<String>,
    #[serde(default)]
    pub original_department_id_path: Vec<OrgdraftDepartmentId>,
    #[serde(default)]
    pub target_department_id_path: Vec<OrgdraftDepartmentId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_work_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_direct_manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_direct_manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_dotted_manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_dotted_manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_workforce_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_workforce_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_employee_subtype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employee_subtype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_duration_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_duration_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_signing_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_signing_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_contract_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_working_hours_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_hours_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_working_calendar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_calendar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_weekly_working_hours: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_weekly_working_hours: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_work_shift: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_shift: Option<String>,
    #[serde(default)]
    pub original_cost_center_rate: Vec<JobDataCostCenter>,
    #[serde(default)]
    pub target_cost_center_rate: Vec<JobDataCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_allocation_expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_allocation_expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_allocation_effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_allocation_effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_default_cost_center: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_default_cost_center: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_is_default_cost_center_inherited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_is_default_cost_center_inherited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_employment_change: Option<TranferEmploymentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employment_change: Option<TranferEmploymentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_grade: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_grade: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_compensation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_compensation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_draft_position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_social_security_city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_social_security_city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_pathway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_pathway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_transfer_with_workforce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department_name: Option<DepartmentLookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department_name: Option<DepartmentLookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_worklocation_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_worklocation_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_direct_manager_name: Option<EmploymentLookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_direct_manager_name: Option<EmploymentLookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_family_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_family_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_level_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_level_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_workforce_type_name: Option<LookupName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_workforce_type_name: Option<LookupName>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Visa {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visa_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkEmail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_usage: Option<Enum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkExperienceInfo {
    #[serde(default)]
    pub company_organization: Vec<I18n>,
    #[serde(default)]
    pub department: Vec<I18n>,
    #[serde(default)]
    pub job: Vec<I18n>,
    #[serde(default)]
    pub description: Vec<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkforcePlanDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan_detail_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<DimensionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<DimensionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location: Option<DimensionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family: Option<DimensionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<DimensionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<DimensionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<DimensionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_individuals: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_added: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_removed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vacancy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vacancy_including_individuals_to_be_added_and_removed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_rate_including_individuals_to_be_added_and_removed: Option<String>,
    #[serde(default)]
    pub estimated_active_individuals_detail: Vec<WorkforcePlanEaiDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_missing_dimension: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkforcePlanDetailV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan_detail_id: Option<String>,
    #[serde(default)]
    pub dimension_info_datas: Vec<DimensionInfoData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_individuals: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_added: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_removed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vacancy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vacancy_including_individuals_to_be_added_and_removed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_rate_including_individuals_to_be_added_and_removed: Option<String>,
    #[serde(default)]
    pub estimated_active_individuals_details: Vec<WorkforcePlanEaiDetail>,
    #[serde(default)]
    pub multi_period_values: Vec<WorkforcePlanMultiPeriodValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_missing_dimension: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all_zero_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkforcePlanEaiDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_active_individuals: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkforcePlanMultiPeriodValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_added: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_removed: Option<String>,
}
// ── V2 service entry ──────────────────────────────────────────────────────────

pub struct V2<'a> {
    pub approval_groups: ApprovalGroupsV2Resource<'a>,
    pub approver: ApproverV2Resource<'a>,
    pub basic_info: BasicInfoV2Resource<'a>,
    pub bp: BpV2Resource<'a>,
    pub company: CompanyV2Resource<'a>,
    pub contract: ContractV2Resource<'a>,
    pub cost_allocation: CostAllocationV2Resource<'a>,
    pub cost_center: CostCenterV2Resource<'a>,
    pub cost_center_version: CostCenterVersionV2Resource<'a>,
    pub custom_org: CustomOrgV2Resource<'a>,
    pub default_cost_center: DefaultCostCenterV2Resource<'a>,
    pub department: DepartmentV2Resource<'a>,
    pub draft: DraftV2Resource<'a>,
    pub employee: EmployeeV2Resource<'a>,
    pub employees_additional_job: EmployeesAdditionalJobV2Resource<'a>,
    pub employees_bp: EmployeesBpV2Resource<'a>,
    pub employees_international_assignment: EmployeesIntlAssignmentV2Resource<'a>,
    pub employees_job_data: EmployeesJobDataV2Resource<'a>,
    pub enum_resource: EnumV2Resource<'a>,
    pub job: JobV2Resource<'a>,
    pub job_change: JobChangeV2Resource<'a>,
    pub job_family: JobFamilyV2Resource<'a>,
    pub job_grade: JobGradeV2Resource<'a>,
    pub job_level: JobLevelV2Resource<'a>,
    pub location: LocationV2Resource<'a>,
    pub location_address: LocationAddressV2Resource<'a>,
    pub offboarding: OffboardingV2Resource<'a>,
    pub pathway: PathwayV2Resource<'a>,
    pub person: PersonV2Resource<'a>,
    pub position: PositionV2Resource<'a>,
    pub pre_hire: PreHireV2Resource<'a>,
    pub probation: ProbationV2Resource<'a>,
    pub probation_assessment: ProbationAssessmentV2Resource<'a>,
    pub process: ProcessV2Resource<'a>,
    pub process_approver: ProcessApproverV2Resource<'a>,
    pub process_cc: ProcessCcV2Resource<'a>,
    pub process_extra: ProcessExtraV2Resource<'a>,
    pub process_form_variable_data: ProcessFormVariableDataV2Resource<'a>,
    pub process_node: ProcessNodeV2Resource<'a>,
    pub process_status: ProcessStatusV2Resource<'a>,
    pub process_transfer: ProcessTransferV2Resource<'a>,
    pub process_comment_info: ProcessCommentInfoV2Resource<'a>,
    pub process_revoke: ProcessRevokeV2Resource<'a>,
    pub process_withdraw: ProcessWithdrawV2Resource<'a>,
    pub report_detail_row: ReportDetailRowV2Resource<'a>,
    pub signature_file: SignatureFileV2Resource<'a>,
    pub signature_node: SignatureNodeV2Resource<'a>,
    pub signature_template: SignatureTemplateV2Resource<'a>,
    pub signature_template_info_with_thumbnail: SignatureTemplateInfoWithThumbnailV2Resource<'a>,
    pub workforce_plan: WorkforcePlanV2Resource<'a>,
    pub workforce_plan_detail: WorkforcePlanDetailV2Resource<'a>,
    pub workforce_plan_detail_row: WorkforcePlanDetailRowV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            approval_groups: ApprovalGroupsV2Resource { config },
            approver: ApproverV2Resource { config },
            basic_info: BasicInfoV2Resource { config },
            bp: BpV2Resource { config },
            company: CompanyV2Resource { config },
            contract: ContractV2Resource { config },
            cost_allocation: CostAllocationV2Resource { config },
            cost_center: CostCenterV2Resource { config },
            cost_center_version: CostCenterVersionV2Resource { config },
            custom_org: CustomOrgV2Resource { config },
            default_cost_center: DefaultCostCenterV2Resource { config },
            department: DepartmentV2Resource { config },
            draft: DraftV2Resource { config },
            employee: EmployeeV2Resource { config },
            employees_additional_job: EmployeesAdditionalJobV2Resource { config },
            employees_bp: EmployeesBpV2Resource { config },
            employees_international_assignment: EmployeesIntlAssignmentV2Resource { config },
            employees_job_data: EmployeesJobDataV2Resource { config },
            enum_resource: EnumV2Resource { config },
            job: JobV2Resource { config },
            job_change: JobChangeV2Resource { config },
            job_family: JobFamilyV2Resource { config },
            job_grade: JobGradeV2Resource { config },
            job_level: JobLevelV2Resource { config },
            location: LocationV2Resource { config },
            location_address: LocationAddressV2Resource { config },
            offboarding: OffboardingV2Resource { config },
            pathway: PathwayV2Resource { config },
            person: PersonV2Resource { config },
            position: PositionV2Resource { config },
            pre_hire: PreHireV2Resource { config },
            probation: ProbationV2Resource { config },
            probation_assessment: ProbationAssessmentV2Resource { config },
            process: ProcessV2Resource { config },
            process_approver: ProcessApproverV2Resource { config },
            process_cc: ProcessCcV2Resource { config },
            process_extra: ProcessExtraV2Resource { config },
            process_form_variable_data: ProcessFormVariableDataV2Resource { config },
            process_node: ProcessNodeV2Resource { config },
            process_status: ProcessStatusV2Resource { config },
            process_transfer: ProcessTransferV2Resource { config },
            process_comment_info: ProcessCommentInfoV2Resource { config },
            process_revoke: ProcessRevokeV2Resource { config },
            process_withdraw: ProcessWithdrawV2Resource { config },
            report_detail_row: ReportDetailRowV2Resource { config },
            signature_file: SignatureFileV2Resource { config },
            signature_node: SignatureNodeV2Resource { config },
            signature_template: SignatureTemplateV2Resource { config },
            signature_template_info_with_thumbnail: SignatureTemplateInfoWithThumbnailV2Resource {
                config,
            },
            workforce_plan: WorkforcePlanV2Resource { config },
            workforce_plan_detail: WorkforcePlanDetailV2Resource { config },
            workforce_plan_detail_row: WorkforcePlanDetailRowV2Resource { config },
        }
    }
}

// ── Macro for simple post/get/delete/patch methods ────────────────────────────

macro_rules! post_method_with_tokens {
    ($fn_name:ident, $resp:ident, $data:ty, $path:expr, $token_types:expr) => {
        pub async fn $fn_name(
            &self,
            body: impl Serialize,
            option: &RequestOption,
        ) -> Result<$resp, LarkError> {
            RestRequest::new(self.config, http::Method::POST, $path, $token_types, option)
                .json_body(&body)?
                .send_v2_response::<$data, $resp>()
                .await
        }
    };
}

macro_rules! post_method {
    ($fn_name:ident, $resp:ident, $body:ty, $data:ty, $path:expr) => {
        post_method_with_tokens!($fn_name, $resp, $data, $path, vec![AccessTokenType::Tenant]);
    };
    ($fn_name:ident, $resp:ident, $data:ty, $path:expr) => {
        post_method_with_tokens!($fn_name, $resp, $data, $path, vec![AccessTokenType::Tenant]);
    };
}

macro_rules! get_method {
    ($fn_name:ident, $resp:ident, $data:ty, $path:expr) => {
        pub async fn $fn_name(&self, option: &RequestOption) -> Result<$resp, LarkError> {
            RestRequest::new(
                self.config,
                http::Method::GET,
                $path,
                vec![AccessTokenType::Tenant],
                option,
            )
            .send_v2_response::<$data, $resp>()
            .await
        }
    };
}

// ── ApprovalGroups resource ───────────────────────────────────────────────────

pub struct ApprovalGroupsV2Resource<'a> {
    config: &'a Config,
}

impl ApprovalGroupsV2Resource<'_> {
    pub async fn get(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<GetApprovalGroupsV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/approval_groups/{process_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetApprovalGroupsV2RespData, GetApprovalGroupsV2Resp>()
        .await
    }

    post_method!(
        open_query_dept_change_list_by_ids,
        OpenQueryDeptChangeApprovalGroupsV2Resp,
        (),
        "/open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids"
    );
    post_method!(
        open_query_job_change_list_by_ids,
        OpenQueryJobChangeApprovalGroupsV2Resp,
        (),
        "/open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids"
    );
    post_method!(
        open_query_position_change_list_by_ids,
        OpenQueryPositionChangeApprovalGroupsV2Resp,
        (),
        "/open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids"
    );
}

// ── Approver resource ─────────────────────────────────────────────────────────

pub struct ApproverV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListApproverV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListApproverV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl ApproverV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListApproverV2Resp, LarkError> {
        let query = ListApproverV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApproverV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListApproverV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/approvers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListData, ListApproverV2Resp>()
        .await
    }
}

// ── BasicInfo resource ────────────────────────────────────────────────────────

pub struct BasicInfoV2Resource<'a> {
    config: &'a Config,
}

impl BasicInfoV2Resource<'_> {
    post_method!(
        search_banks,
        SearchBankV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/banks/search"
    );
    post_method!(
        search_bank_branches,
        SearchBankBranchV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/bank_branchs/search"
    );
    post_method!(
        search_cities,
        SearchCityV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/cities/search"
    );
    post_method!(
        search_country_regions,
        SearchCountryRegionV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/country_regions/search"
    );
    post_method!(
        search_country_region_subdivisions,
        SearchCountryRegionSubdivisionV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/country_region_subdivisions/search"
    );
    post_method!(
        search_currencies,
        SearchCurrencyV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/currencies/search"
    );
    post_method!(
        search_districts,
        SearchDistrictV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/districts/search"
    );
    post_method!(
        search_languages,
        SearchLanguageV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/languages/search"
    );
    post_method!(
        search_nationalities,
        SearchNationalityV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/nationalities/search"
    );
    post_method!(
        search_time_zones,
        SearchTimeZoneV2Resp,
        ListData,
        "/open-apis/corehr/v2/basic_info/time_zones/search"
    );
}

// ── Bp resource ───────────────────────────────────────────────────────────────

pub struct BpV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListBpV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListBpV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl BpV2Resource<'_> {
    post_method!(
        get_by_department,
        GetBpByDepartmentV2Resp,
        (),
        "/open-apis/corehr/v2/bps/get_by_department"
    );

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBpV2Resp, LarkError> {
        let query = ListBpV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListBpV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListBpV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/bps",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListData, ListBpV2Resp>()
        .await
    }
}

// ── Company resource ──────────────────────────────────────────────────────────

pub struct CompanyV2Resource<'a> {
    config: &'a Config,
}

impl CompanyV2Resource<'_> {
    post_method!(
        active,
        ActiveCompanyV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/companies/active"
    );
    post_method!(
        batch_get,
        BatchGetCompanyV2Resp,
        BatchGetCompanyV2RespData,
        "/open-apis/corehr/v2/companies/batch_get"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeCompanyV2Resp,
        QueryRecentChangeCompanyV2RespData,
        "/open-apis/corehr/v2/companies/query_recent_change"
    );
}

// ── Contract resource ─────────────────────────────────────────────────────────

pub struct ContractV2Resource<'a> {
    config: &'a Config,
}

impl ContractV2Resource<'_> {
    post_method!(
        search,
        SearchContractV2Resp,
        ListData,
        "/open-apis/corehr/v2/contracts/search"
    );
}

// ── CostAllocation resource ───────────────────────────────────────────────────

pub struct CostAllocationV2Resource<'a> {
    config: &'a Config,
}

impl CostAllocationV2Resource<'_> {
    post_method!(
        batch_query,
        BatchQueryCostAllocationV2Resp,
        BatchQueryCostAllocationV2RespData,
        "/open-apis/corehr/v2/cost_allocations/batch_query"
    );
    post_method!(
        create_version,
        CreateVersionCostAllocationV2Resp,
        CreateVersionCostAllocationV2RespData,
        "/open-apis/corehr/v2/cost_allocations/create_version"
    );
    post_method!(
        remove_version,
        RemoveVersionCostAllocationV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/cost_allocations/remove_version"
    );
    post_method!(
        update_version,
        UpdateVersionCostAllocationV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/cost_allocations/update_version"
    );
}

// ── CostCenter resource ───────────────────────────────────────────────────────

pub struct CostCenterV2Resource<'a> {
    config: &'a Config,
}

impl CostCenterV2Resource<'_> {
    post_method!(
        create,
        CreateCostCenterV2Resp,
        CreateCostCenterV2RespData,
        "/open-apis/corehr/v2/cost_centers"
    );

    pub async fn delete(
        &self,
        cost_center_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCostCenterV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteCostCenterV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        cost_center_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchCostCenterV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchCostCenterV2RespData, PatchCostCenterV2Resp>()
        .await
    }

    get_method!(
        query_recent_change,
        QueryRecentChangeCostCenterV2Resp,
        QueryRecentChangeCostCenterV2RespData,
        "/open-apis/corehr/v2/cost_centers/query_recent_change"
    );
    post_method!(
        search,
        SearchCostCenterV2Resp,
        ListData,
        "/open-apis/corehr/v2/cost_centers/search"
    );
}

// ── CostCenterVersion resource ────────────────────────────────────────────────

pub struct CostCenterVersionV2Resource<'a> {
    config: &'a Config,
}

impl CostCenterVersionV2Resource<'_> {
    pub async fn create(
        &self,
        cost_center_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateCostCenterVersionV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}/versions");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateCostCenterVersionV2RespData, CreateCostCenterVersionV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        cost_center_id: &str,
        version_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCostCenterVersionV2Resp, LarkError> {
        let path =
            format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}/versions/{version_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteCostCenterVersionV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        cost_center_id: &str,
        version_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchCostCenterVersionV2Resp, LarkError> {
        let path =
            format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}/versions/{version_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchCostCenterVersionV2RespData, PatchCostCenterVersionV2Resp>()
        .await
    }
}

// ── CustomOrg resource ────────────────────────────────────────────────────────

pub struct CustomOrgV2Resource<'a> {
    config: &'a Config,
}

impl CustomOrgV2Resource<'_> {
    post_method!(
        active,
        ActiveCustomOrgV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/custom_orgs/active"
    );
    post_method!(
        create,
        CreateCustomOrgV2Resp,
        CreateCustomOrgV2RespData,
        "/open-apis/corehr/v2/custom_orgs"
    );
    post_method!(
        delete_org,
        DeleteOrgCustomOrgV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/custom_orgs/delete_org"
    );

    pub async fn patch(
        &self,
        org_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchCustomOrgV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/custom_orgs/{org_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchCustomOrgV2Resp>()
        .await
    }

    post_method!(
        query,
        QueryCustomOrgV2Resp,
        QueryCustomOrgV2RespData,
        "/open-apis/corehr/v2/custom_orgs/query"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeCustomOrgV2Resp,
        QueryRecentChangeCustomOrgV2RespData,
        "/open-apis/corehr/v2/custom_orgs/query_recent_change"
    );
    post_method!(
        update_rule,
        UpdateRuleCustomOrgV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/custom_orgs/update_rule"
    );
}

// ── DefaultCostCenter resource ────────────────────────────────────────────────

pub struct DefaultCostCenterV2Resource<'a> {
    config: &'a Config,
}

impl DefaultCostCenterV2Resource<'_> {
    post_method!(
        batch_query,
        BatchQueryDefaultCostCenterV2Resp,
        BatchQueryDefaultCostCenterV2RespData,
        "/open-apis/corehr/v2/default_cost_centers/batch_query"
    );
    post_method!(
        create_version,
        CreateVersionDefaultCostCenterV2Resp,
        CreateVersionDefaultCostCenterV2RespData,
        "/open-apis/corehr/v2/default_cost_centers/create_version"
    );
    post_method!(
        remove_version,
        RemoveVersionDefaultCostCenterV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/default_cost_centers/remove_version"
    );
    post_method!(
        update_version,
        UpdateVersionDefaultCostCenterV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/default_cost_centers/update_version"
    );
}

// ── Department resource ───────────────────────────────────────────────────────

pub struct DepartmentV2Resource<'a> {
    config: &'a Config,
}

impl DepartmentV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetDepartmentV2Resp,
        BatchGetDepartmentV2RespData,
        "/open-apis/corehr/v2/departments/batch_get"
    );

    pub async fn delete(
        &self,
        department_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDepartmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/departments/{department_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteDepartmentV2Resp>()
        .await
    }

    post_method_with_tokens!(
        parents,
        ParentsDepartmentV2Resp,
        ParentsDepartmentV2RespData,
        "/open-apis/corehr/v2/departments/parents",
        vec![AccessTokenType::Tenant, AccessTokenType::User]
    );

    pub async fn patch(
        &self,
        department_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchDepartmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/departments/{department_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchDepartmentV2Resp>()
        .await
    }

    post_method!(
        query_multi_timeline,
        QueryMultiTimelineDepartmentV2Resp,
        QueryMultiTimelineDepartmentV2RespData,
        "/open-apis/corehr/v2/departments/query_multi_timeline"
    );
    post_method!(
        query_operation_logs,
        QueryOperationLogsDepartmentV2Resp,
        QueryOperationLogsDepartmentV2RespData,
        "/open-apis/corehr/v2/departments/query_operation_logs"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeDepartmentV2Resp,
        QueryRecentChangeDepartmentV2RespData,
        "/open-apis/corehr/v2/departments/query_recent_change"
    );
    post_method!(
        query_timeline,
        QueryTimelineDepartmentV2Resp,
        QueryTimelineDepartmentV2RespData,
        "/open-apis/corehr/v2/departments/query_timeline"
    );
    post_method_with_tokens!(
        search,
        SearchDepartmentV2Resp,
        ListData,
        "/open-apis/corehr/v2/departments/search",
        vec![AccessTokenType::Tenant, AccessTokenType::User]
    );
    post_method_with_tokens!(
        tree,
        TreeDepartmentV2Resp,
        TreeDepartmentV2RespData,
        "/open-apis/corehr/v2/departments/tree",
        vec![AccessTokenType::Tenant, AccessTokenType::User]
    );
}

// ── Draft resource ────────────────────────────────────────────────────────────

pub struct DraftV2Resource<'a> {
    config: &'a Config,
}

impl DraftV2Resource<'_> {
    pub async fn get(
        &self,
        draft_id: &str,
        option: &RequestOption,
    ) -> Result<GetDraftV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/drafts/{draft_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetDraftV2RespData, GetDraftV2Resp>()
        .await
    }
}

// ── Employee resource ─────────────────────────────────────────────────────────

pub struct EmployeeV2Resource<'a> {
    config: &'a Config,
}

impl EmployeeV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetEmployeeV2Resp,
        BatchGetEmployeeV2RespData,
        "/open-apis/corehr/v2/employees/batch_get"
    );
    post_method!(
        create,
        CreateEmployeeV2Resp,
        CreateEmployeeV2RespData,
        "/open-apis/corehr/v2/employees"
    );
    post_method!(
        search,
        SearchEmployeeV2Resp,
        ListData,
        "/open-apis/corehr/v2/employees/search"
    );
}

// ── EmployeesAdditionalJob resource ──────────────────────────────────────────

pub struct EmployeesAdditionalJobV2Resource<'a> {
    config: &'a Config,
}

impl EmployeesAdditionalJobV2Resource<'_> {
    post_method!(
        batch,
        BatchEmployeesAdditionalJobV2Resp,
        BatchEmployeesAdditionalJobV2RespData,
        "/open-apis/corehr/v2/employees/additional_jobs/batch"
    );
    post_method!(
        create,
        CreateEmployeesAdditionalJobV2Resp,
        CreateEmployeesAdditionalJobV2RespData,
        "/open-apis/corehr/v2/employees/additional_jobs"
    );

    pub async fn delete(
        &self,
        additional_job_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmployeesAdditionalJobV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/employees/additional_jobs/{additional_job_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteEmployeesAdditionalJobV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        additional_job_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchEmployeesAdditionalJobV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/employees/additional_jobs/{additional_job_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchEmployeesAdditionalJobV2RespData, PatchEmployeesAdditionalJobV2Resp>()
        .await
    }
}

// ── EmployeesBp resource ──────────────────────────────────────────────────────

pub struct EmployeesBpV2Resource<'a> {
    config: &'a Config,
}

impl EmployeesBpV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetEmployeesBpV2Resp,
        BatchGetEmployeesBpV2RespData,
        "/open-apis/corehr/v2/employees/bps/batch_get"
    );
}

// ── EmployeesIntlAssignment resource ─────────────────────────────────────────

pub struct EmployeesIntlAssignmentV2Resource<'a> {
    config: &'a Config,
}

impl EmployeesIntlAssignmentV2Resource<'_> {
    post_method!(
        create,
        CreateEmployeesIntlAssignmentV2Resp,
        (),
        "/open-apis/corehr/v2/employees/international_assignments"
    );

    pub async fn delete(
        &self,
        international_assignment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmployeesIntlAssignmentV2Resp, LarkError> {
        let path = format!(
            "/open-apis/corehr/v2/employees/international_assignments/{international_assignment_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteEmployeesIntlAssignmentV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEmployeesIntlAssignmentV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/employees/international_assignments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<ListData, ListEmployeesIntlAssignmentV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        international_assignment_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchEmployeesIntlAssignmentV2Resp, LarkError> {
        let path = format!(
            "/open-apis/corehr/v2/employees/international_assignments/{international_assignment_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchEmployeesIntlAssignmentV2Resp>()
        .await
    }
}

// ── EmployeesJobData resource ─────────────────────────────────────────────────

pub struct EmployeesJobDataV2Resource<'a> {
    config: &'a Config,
}

impl EmployeesJobDataV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetEmployeesJobDataV2Resp,
        BatchGetEmployeesJobDataV2RespData,
        "/open-apis/corehr/v2/employees/job_datas/batch_get"
    );
    post_method!(
        query,
        QueryEmployeesJobDataV2Resp,
        QueryEmployeesJobDataV2RespData,
        "/open-apis/corehr/v2/employees/job_datas/query"
    );
}

// ── Enum resource ─────────────────────────────────────────────────────────────

pub struct EnumV2Resource<'a> {
    config: &'a Config,
}

impl EnumV2Resource<'_> {
    post_method!(
        search,
        SearchEnumV2Resp,
        ListData,
        "/open-apis/corehr/v2/enums/search"
    );
}

// ── Job resource ──────────────────────────────────────────────────────────────

pub struct JobV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListJobV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl JobV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetJobV2Resp,
        BatchGetJobV2RespData,
        "/open-apis/corehr/v2/jobs/batch_get"
    );

    pub async fn get(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/jobs/{job_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetJobV2RespData, GetJobV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobV2Resp, LarkError> {
        let query = ListJobV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListJobV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/jobs",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListData, ListJobV2Resp>()
        .await
    }

    post_method!(
        query_multi_timeline,
        QueryMultiTimelineJobV2Resp,
        QueryMultiTimelineJobV2RespData,
        "/open-apis/corehr/v2/jobs/query_multi_timeline"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobV2Resp,
        QueryRecentChangeJobV2RespData,
        "/open-apis/corehr/v2/jobs/query_recent_change"
    );
}

// ── JobChange resource ────────────────────────────────────────────────────────

pub struct JobChangeV2Resource<'a> {
    config: &'a Config,
}

impl JobChangeV2Resource<'_> {
    post_method!(
        create,
        CreateJobChangeV2Resp,
        CreateJobChangeV2RespData,
        "/open-apis/corehr/v2/job_changes"
    );

    pub async fn revoke(
        &self,
        job_change_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<RevokeJobChangeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/job_changes/{job_change_id}/revoke");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<RevokeJobChangeV2RespData, RevokeJobChangeV2Resp>()
        .await
    }

    post_method_with_tokens!(
        search,
        SearchJobChangeV2Resp,
        ListData,
        "/open-apis/corehr/v2/job_changes/search",
        vec![AccessTokenType::Tenant, AccessTokenType::User]
    );
}

// ── JobFamily resource ────────────────────────────────────────────────────────

pub struct JobFamilyV2Resource<'a> {
    config: &'a Config,
}

impl JobFamilyV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetJobFamilyV2Resp,
        BatchGetJobFamilyV2RespData,
        "/open-apis/corehr/v2/job_families/batch_get"
    );
    post_method!(
        query_multi_timeline,
        QueryMultiTimelineJobFamilyV2Resp,
        QueryMultiTimelineJobFamilyV2RespData,
        "/open-apis/corehr/v2/job_families/query_multi_timeline"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobFamilyV2Resp,
        QueryRecentChangeJobFamilyV2RespData,
        "/open-apis/corehr/v2/job_families/query_recent_change"
    );
}

// ── JobGrade resource ─────────────────────────────────────────────────────────

pub struct JobGradeV2Resource<'a> {
    config: &'a Config,
}

impl JobGradeV2Resource<'_> {
    post_method!(
        create,
        CreateJobGradeV2Resp,
        CreateJobGradeV2RespData,
        "/open-apis/corehr/v2/job_grades"
    );

    pub async fn delete(
        &self,
        job_grade_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobGradeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/job_grades/{job_grade_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteJobGradeV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        job_grade_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchJobGradeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/job_grades/{job_grade_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchJobGradeV2Resp>()
        .await
    }

    post_method!(
        query,
        QueryJobGradeV2Resp,
        QueryJobGradeV2RespData,
        "/open-apis/corehr/v2/job_grades/query"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobGradeV2Resp,
        QueryRecentChangeJobGradeV2RespData,
        "/open-apis/corehr/v2/job_grades/query_recent_change"
    );
}

// ── JobLevel resource ─────────────────────────────────────────────────────────

pub struct JobLevelV2Resource<'a> {
    config: &'a Config,
}

impl JobLevelV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetJobLevelV2Resp,
        BatchGetJobLevelV2RespData,
        "/open-apis/corehr/v2/job_levels/batch_get"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobLevelV2Resp,
        QueryRecentChangeJobLevelV2RespData,
        "/open-apis/corehr/v2/job_levels/query_recent_change"
    );
}

// ── Location resource ─────────────────────────────────────────────────────────

pub struct LocationV2Resource<'a> {
    config: &'a Config,
}

impl LocationV2Resource<'_> {
    post_method!(
        active,
        ActiveLocationV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/locations/active"
    );
    post_method!(
        batch_get,
        BatchGetLocationV2Resp,
        BatchGetLocationV2RespData,
        "/open-apis/corehr/v2/locations/batch_get"
    );

    pub async fn patch(
        &self,
        location_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchLocationV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchLocationV2Resp>()
        .await
    }

    get_method!(
        query_recent_change,
        QueryRecentChangeLocationV2Resp,
        QueryRecentChangeLocationV2RespData,
        "/open-apis/corehr/v2/locations/query_recent_change"
    );
}

// ── LocationAddress resource ──────────────────────────────────────────────────

pub struct LocationAddressV2Resource<'a> {
    config: &'a Config,
}

impl LocationAddressV2Resource<'_> {
    pub async fn create(
        &self,
        location_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateLocationAddressV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}/addresses");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateLocationAddressV2RespData, CreateLocationAddressV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        location_id: &str,
        address_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLocationAddressV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}/addresses/{address_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteLocationAddressV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        location_id: &str,
        address_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchLocationAddressV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}/addresses/{address_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchLocationAddressV2Resp>()
        .await
    }
}

// ── Offboarding resource ──────────────────────────────────────────────────────

pub struct OffboardingV2Resource<'a> {
    config: &'a Config,
}

impl OffboardingV2Resource<'_> {
    post_method!(
        edit,
        EditOffboardingV2Resp,
        EditOffboardingV2RespData,
        "/open-apis/corehr/v2/offboardings/edit"
    );
    post_method!(
        revoke,
        RevokeOffboardingV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/offboardings/revoke"
    );
    post_method!(
        submit_v2,
        SubmitOffboardingV2Resp,
        (),
        "/open-apis/corehr/v2/offboardings/submit_v2"
    );
}

// ── Pathway resource ──────────────────────────────────────────────────────────

pub struct PathwayV2Resource<'a> {
    config: &'a Config,
}

impl PathwayV2Resource<'_> {
    post_method!(
        active,
        ActivePathwayV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/pathways/active"
    );
    post_method!(
        batch_get,
        BatchGetPathwayV2Resp,
        BatchGetPathwayV2RespData,
        "/open-apis/corehr/v2/pathways/batch_get"
    );
    post_method!(
        create,
        CreatePathwayV2Resp,
        CreatePathwayV2RespData,
        "/open-apis/corehr/v2/pathways"
    );

    pub async fn delete(
        &self,
        pathway_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePathwayV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pathways/{pathway_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeletePathwayV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        pathway_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchPathwayV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pathways/{pathway_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchPathwayV2Resp>()
        .await
    }
}

// ── Person resource ───────────────────────────────────────────────────────────

pub struct PersonV2Resource<'a> {
    config: &'a Config,
}

impl PersonV2Resource<'_> {
    post_method!(
        create,
        CreatePersonV2Resp,
        CreatePersonV2RespData,
        "/open-apis/corehr/v2/persons"
    );

    pub async fn patch(
        &self,
        person_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchPersonV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/persons/{person_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchPersonV2RespData, PatchPersonV2Resp>()
        .await
    }
}

// ── Position resource ─────────────────────────────────────────────────────────

pub struct PositionV2Resource<'a> {
    config: &'a Config,
}

impl PositionV2Resource<'_> {
    post_method!(
        active,
        ActivePositionV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/positions/active"
    );
    post_method!(
        create,
        CreatePositionV2Resp,
        CreatePositionV2RespData,
        "/open-apis/corehr/v2/positions"
    );
    post_method!(
        del_position,
        DelPositionV2Resp,
        (),
        "/open-apis/corehr/v2/positions/del_position"
    );

    pub async fn patch(
        &self,
        position_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchPositionV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/positions/{position_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchPositionV2Resp>()
        .await
    }

    post_method!(
        query,
        QueryPositionV2Resp,
        QueryPositionV2RespData,
        "/open-apis/corehr/v2/positions/query"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangePositionV2Resp,
        QueryRecentChangePositionV2RespData,
        "/open-apis/corehr/v2/positions/query_recent_change"
    );
}

// ── PreHire resource ──────────────────────────────────────────────────────────

pub struct PreHireV2Resource<'a> {
    config: &'a Config,
}

impl PreHireV2Resource<'_> {
    pub async fn complete(
        &self,
        pre_hire_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CompletePreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}/complete");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CompletePreHireV2RespData, CompletePreHireV2Resp>()
        .await
    }

    post_method!(
        create,
        CreatePreHireV2Resp,
        CreatePreHireV2RespData,
        "/open-apis/corehr/v2/pre_hires"
    );

    pub async fn delete(
        &self,
        pre_hire_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeletePreHireV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        pre_hire_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchPreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchPreHireV2RespData, PatchPreHireV2Resp>()
        .await
    }

    post_method!(
        query,
        QueryPreHireV2Resp,
        QueryPreHireV2RespData,
        "/open-apis/corehr/v2/pre_hires/query"
    );
    post_method!(
        restore_flow_instance,
        RestoreFlowInstancePreHireV2Resp,
        RestoreFlowInstancePreHireV2RespData,
        "/open-apis/corehr/v2/pre_hires/restore_flow_instance"
    );
    post_method!(
        search,
        SearchPreHireV2Resp,
        ListData,
        "/open-apis/corehr/v2/pre_hires/search"
    );
    post_method!(
        transform_onboarding_task,
        TransformOnboardingTaskPreHireV2Resp,
        TransformOnboardingTaskPreHireV2RespData,
        "/open-apis/corehr/v2/pre_hires/transform_onboarding_task"
    );

    pub async fn transit_task(
        &self,
        pre_hire_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<TransitTaskPreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}/transit_task");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<TransitTaskPreHireV2RespData, TransitTaskPreHireV2Resp>()
        .await
    }

    post_method!(
        withdraw_onboarding,
        WithdrawOnboardingPreHireV2Resp,
        WithdrawOnboardingPreHireV2RespData,
        "/open-apis/corehr/v2/pre_hires/withdraw_onboarding"
    );
}

// ── Probation resource ────────────────────────────────────────────────────────

pub struct ProbationV2Resource<'a> {
    config: &'a Config,
}

impl ProbationV2Resource<'_> {
    post_method!(
        enable_disable_assessment,
        EnableDisableAssessmentProbationV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/probation/enable_disable_assessment"
    );
    post_method!(
        search,
        SearchProbationV2Resp,
        ListData,
        "/open-apis/corehr/v2/probation/search"
    );
    post_method!(
        submit,
        SubmitProbationV2Resp,
        SubmitProbationV2RespData,
        "/open-apis/corehr/v2/probation/submit"
    );
    post_method!(
        withdraw,
        WithdrawProbationV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/probation/withdraw"
    );
}

// ── ProbationAssessment resource ──────────────────────────────────────────────

pub struct ProbationAssessmentV2Resource<'a> {
    config: &'a Config,
}

impl ProbationAssessmentV2Resource<'_> {
    post_method!(
        create,
        CreateProbationAssessmentV2Resp,
        CreateProbationAssessmentV2RespData,
        "/open-apis/corehr/v2/probation/assessments"
    );

    pub async fn delete(
        &self,
        assessment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteProbationAssessmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/probation/assessments/{assessment_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteProbationAssessmentV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        assessment_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchProbationAssessmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/probation/assessments/{assessment_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchProbationAssessmentV2Resp>()
        .await
    }
}

// ── Process resource ──────────────────────────────────────────────────────────

pub struct ProcessV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListProcessV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListProcessV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl ProcessV2Resource<'_> {
    pub async fn flow_variable_data(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<FlowVariableDataProcessV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/flow_variable_data");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<FlowVariableDataProcessV2RespData, FlowVariableDataProcessV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<GetProcessV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetProcessV2RespData, GetProcessV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListProcessV2Resp, LarkError> {
        let query = ListProcessV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListProcessV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListProcessV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/processes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListData, ListProcessV2Resp>()
        .await
    }
}

// ── ProcessApprover resource ──────────────────────────────────────────────────

pub struct ProcessApproverV2Resource<'a> {
    config: &'a Config,
}

impl ProcessApproverV2Resource<'_> {
    pub async fn update(
        &self,
        process_id: &str,
        approver_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateProcessApproverV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/approvers/{approver_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<UpdateProcessApproverV2RespData, UpdateProcessApproverV2Resp>()
        .await
    }
}

// ── ProcessExtra resource ─────────────────────────────────────────────────────

pub struct ProcessExtraV2Resource<'a> {
    config: &'a Config,
}

impl ProcessExtraV2Resource<'_> {
    pub async fn update(
        &self,
        process_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateProcessExtraV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/extra");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), UpdateProcessExtraV2Resp>()
        .await
    }
}

// ── ProcessFormVariableData resource ─────────────────────────────────────────

pub struct ProcessFormVariableDataV2Resource<'a> {
    config: &'a Config,
}

impl ProcessFormVariableDataV2Resource<'_> {
    pub async fn get(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<GetProcessFormVariableDataV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/form_variable_data");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetProcessFormVariableDataV2RespData, GetProcessFormVariableDataV2Resp>(
        )
        .await
    }
}

// ── ProcessTransfer resource ──────────────────────────────────────────────────

pub struct ProcessTransferV2Resource<'a> {
    config: &'a Config,
}

impl ProcessTransferV2Resource<'_> {
    pub async fn update(
        &self,
        process_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateProcessTransferV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/transfer");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), UpdateProcessTransferV2Resp>()
        .await
    }
}

// ── ProcessCc resource (placeholder — no methods in Go SDK) ───────────────────

pub struct ProcessCcV2Resource<'a> {
    #[allow(dead_code)]
    config: &'a Config,
}

impl ProcessCcV2Resource<'_> {}

// ── ProcessNode resource (placeholder — no methods in Go SDK) ─────────────────

pub struct ProcessNodeV2Resource<'a> {
    #[allow(dead_code)]
    config: &'a Config,
}

impl ProcessNodeV2Resource<'_> {}

// ── ProcessStatus resource (placeholder — no methods in Go SDK) ───────────────

pub struct ProcessStatusV2Resource<'a> {
    #[allow(dead_code)]
    config: &'a Config,
}

impl ProcessStatusV2Resource<'_> {}

// ── ProcessCommentInfo resource (placeholder — no methods in Go SDK) ──────────

pub struct ProcessCommentInfoV2Resource<'a> {
    #[allow(dead_code)]
    config: &'a Config,
}

impl ProcessCommentInfoV2Resource<'_> {}

// ── ProcessRevoke resource ────────────────────────────────────────────────────

pub struct ProcessRevokeV2Resource<'a> {
    config: &'a Config,
}

impl ProcessRevokeV2Resource<'_> {
    pub async fn update(
        &self,
        process_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateProcessRevokeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/process_revoke/{process_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), UpdateProcessRevokeV2Resp>()
        .await
    }
}

// ── ProcessWithdraw resource ──────────────────────────────────────────────────

pub struct ProcessWithdrawV2Resource<'a> {
    config: &'a Config,
}

impl ProcessWithdrawV2Resource<'_> {
    pub async fn update(
        &self,
        process_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateProcessWithdrawV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/process_withdraw/{process_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), UpdateProcessWithdrawV2Resp>()
        .await
    }
}

// ── ReportDetailRow resource ──────────────────────────────────────────────────

pub struct ReportDetailRowV2Resource<'a> {
    config: &'a Config,
}

impl ReportDetailRowV2Resource<'_> {
    post_method!(
        batch_delete,
        BatchDeleteReportDetailRowV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/report_detail_row/batchDelete"
    );
    post_method!(
        batch_save,
        BatchSaveReportDetailRowV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/report_detail_row/batchSave"
    );
}

// ── SignatureFile resource ────────────────────────────────────────────────────

pub struct SignatureFileV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListSignatureFileV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub signature_file_id: Option<&'a str>,
    pub states: Option<&'a str>,
    pub update_time_start: Option<&'a str>,
    pub update_time_end: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub template_ids: Option<&'a str>,
    pub select_sign_url: Option<bool>,
}

impl<'a> ListSignatureFileV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn signature_file_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.signature_file_id = value.into();
        self
    }

    pub fn states(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.states = value.into();
        self
    }

    pub fn update_time_start(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_time_start = value.into();
        self
    }

    pub fn update_time_end(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_time_end = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn template_ids(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.template_ids = value.into();
        self
    }

    pub fn select_sign_url(mut self, value: impl Into<Option<bool>>) -> Self {
        self.select_sign_url = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListByBizIdSignatureFileV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub biz_process_id: Option<&'a str>,
    pub biz_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub select_sign_url: Option<bool>,
}

impl<'a> ListByBizIdSignatureFileV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn biz_process_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.biz_process_id = value.into();
        self
    }

    pub fn biz_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.biz_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn select_sign_url(mut self, value: impl Into<Option<bool>>) -> Self {
        self.select_sign_url = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl SignatureFileV2Resource<'_> {
    pub async fn download(
        &self,
        signature_file_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<DownloadSignatureFileV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/signature_files/{signature_file_id}/download");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), DownloadSignatureFileV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSignatureFileV2Resp, LarkError> {
        let query = ListSignatureFileV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSignatureFileV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListSignatureFileV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/signature_files",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("signature_file_id", query.signature_file_id)
        .query("states", query.states)
        .query("update_time_start", query.update_time_start)
        .query("update_time_end", query.update_time_end)
        .query("user_id_type", query.user_id_type)
        .query("template_ids", query.template_ids)
        .query("select_sign_url", query.select_sign_url)
        .send_v2_response::<ListData, ListSignatureFileV2Resp>()
        .await
    }

    pub async fn list_by_biz_id(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListByBizIdSignatureFileV2Resp, LarkError> {
        let query = ListByBizIdSignatureFileV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_biz_id_by_query(&query, option).await
    }

    pub async fn list_by_biz_id_by_query(
        &self,
        query: &ListByBizIdSignatureFileV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListByBizIdSignatureFileV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/signature_files/list_by_biz_id",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("biz_process_id", query.biz_process_id)
        .query("biz_type", query.biz_type)
        .query("user_id_type", query.user_id_type)
        .query("select_sign_url", query.select_sign_url)
        .send_v2_response::<ListData, ListByBizIdSignatureFileV2Resp>()
        .await
    }

    post_method!(
        query,
        QuerySignatureFileV2Resp,
        QuerySignatureFileV2RespData,
        "/open-apis/corehr/v2/signature_files/query"
    );
    post_method!(
        terminate,
        TerminateSignatureFileV2Resp,
        TerminateSignatureFileV2RespData,
        "/open-apis/corehr/v2/signature_files/terminate"
    );
}

// ── SignatureNode resource ────────────────────────────────────────────────────

pub struct SignatureNodeV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListByFileIdSignatureNodeV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListByFileIdSignatureNodeV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn file_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.file_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl SignatureNodeV2Resource<'_> {
    pub async fn list_by_file_id(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListByFileIdSignatureNodeV2Resp, LarkError> {
        let query = ListByFileIdSignatureNodeV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_file_id_by_query(&query, option).await
    }

    pub async fn list_by_file_id_by_query(
        &self,
        query: &ListByFileIdSignatureNodeV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListByFileIdSignatureNodeV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/signature_nodes/list_by_file_id",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("file_id", query.file_id)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListData, ListByFileIdSignatureNodeV2Resp>()
        .await
    }
}

// ── SignatureTemplate resource ────────────────────────────────────────────────

pub struct SignatureTemplateV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct SearchSignatureTemplateV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub template_ids: Option<&'a str>,
    pub select_custom_field: Option<bool>,
}

impl<'a> SearchSignatureTemplateV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn template_ids(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.template_ids = value.into();
        self
    }

    pub fn select_custom_field(mut self, value: impl Into<Option<bool>>) -> Self {
        self.select_custom_field = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl SignatureTemplateV2Resource<'_> {
    pub async fn search(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchSignatureTemplateV2Resp, LarkError> {
        let query = SearchSignatureTemplateV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchSignatureTemplateV2Query<'_>,
        option: &RequestOption,
    ) -> Result<SearchSignatureTemplateV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/signature_templates/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("template_ids", query.template_ids)
        .query("select_custom_field", query.select_custom_field)
        .send_v2_response::<ListData, SearchSignatureTemplateV2Resp>()
        .await
    }
}

// ── SignatureTemplateInfoWithThumbnail resource ───────────────────────────────

pub struct SignatureTemplateInfoWithThumbnailV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListSignatureTemplateInfoWithThumbnailV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub name: Option<&'a str>,
    pub category_apiname: Option<&'a str>,
    pub usage_apiname: Option<&'a str>,
    pub active: Option<bool>,
    pub need_region_info: Option<bool>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListSignatureTemplateInfoWithThumbnailV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.name = value.into();
        self
    }

    pub fn category_apiname(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.category_apiname = value.into();
        self
    }

    pub fn usage_apiname(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.usage_apiname = value.into();
        self
    }

    pub fn active(mut self, value: impl Into<Option<bool>>) -> Self {
        self.active = value.into();
        self
    }

    pub fn need_region_info(mut self, value: impl Into<Option<bool>>) -> Self {
        self.need_region_info = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl SignatureTemplateInfoWithThumbnailV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSignatureTemplateInfoWithThumbnailV2Resp, LarkError> {
        let query = ListSignatureTemplateInfoWithThumbnailV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSignatureTemplateInfoWithThumbnailV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListSignatureTemplateInfoWithThumbnailV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/signature_template_info_with_thumbnails",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("name", query.name)
        .query("category_apiname", query.category_apiname)
        .query("usage_apiname", query.usage_apiname)
        .query("active", query.active)
        .query("need_region_info", query.need_region_info)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListData, ListSignatureTemplateInfoWithThumbnailV2Resp>()
        .await
    }
}

// ── WorkforcePlan resource ────────────────────────────────────────────────────

pub struct WorkforcePlanV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListWorkforcePlanV2Query<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub get_all_plan: Option<bool>,
    pub active: Option<bool>,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

impl<'a> ListWorkforcePlanV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn limit(mut self, value: impl Into<Option<i32>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<i32>>) -> Self {
        self.offset = value.into();
        self
    }

    pub fn get_all_plan(mut self, value: impl Into<Option<bool>>) -> Self {
        self.get_all_plan = value.into();
        self
    }

    pub fn active(mut self, value: impl Into<Option<bool>>) -> Self {
        self.active = value.into();
        self
    }

    pub fn start_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_date = value.into();
        self
    }

    pub fn end_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_date = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl WorkforcePlanV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkforcePlanV2Resp, LarkError> {
        let query = ListWorkforcePlanV2Query::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListWorkforcePlanV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListWorkforcePlanV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v2/workforce_plans",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page_query())
        .query("limit", query.limit)
        .query("offset", query.offset)
        .query("get_all_plan", query.get_all_plan)
        .query("active", query.active)
        .query("start_date", query.start_date)
        .query("end_date", query.end_date)
        .send_v2_response::<ListData, ListWorkforcePlanV2Resp>()
        .await
    }
}

// ── WorkforcePlanDetail resource ──────────────────────────────────────────────

pub struct WorkforcePlanDetailV2Resource<'a> {
    config: &'a Config,
}

impl WorkforcePlanDetailV2Resource<'_> {
    post_method_with_tokens!(
        batch,
        BatchWorkforcePlanDetailV2Resp,
        BatchWorkforcePlanDetailV2RespData,
        "/open-apis/corehr/v2/workforce_plan_details/batch",
        vec![AccessTokenType::Tenant, AccessTokenType::User]
    );
    post_method_with_tokens!(
        batch_v2,
        BatchV2WorkforcePlanDetailV2Resp,
        BatchV2WorkforcePlanDetailV2RespData,
        "/open-apis/corehr/v2/workforce_plan_details/batch_v2",
        vec![AccessTokenType::Tenant, AccessTokenType::User]
    );
}

// ── WorkforcePlanDetailRow resource ──────────────────────────────────────────

pub struct WorkforcePlanDetailRowV2Resource<'a> {
    config: &'a Config,
}

impl WorkforcePlanDetailRowV2Resource<'_> {
    post_method!(
        batch_delete,
        BatchDeleteWorkforcePlanDetailRowV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/workforce_plan_detail_row/batchDelete"
    );
    post_method!(
        batch_save,
        BatchSaveWorkforcePlanDetailRowV2Resp,
        crate::JsonValue,
        (),
        "/open-apis/corehr/v2/workforce_plan_detail_row/batchSave"
    );
}
