use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::parse_v2;
use crate::transport;

// ── Generic response data types ───────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemData {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

// ── Response types ─────────────────────────────────────────────────────────────

// approval_groups
impl_resp_v2!(GetApprovalGroupsV2Resp, serde_json::Value);
impl_resp_v2!(OpenQueryDeptChangeApprovalGroupsV2Resp, serde_json::Value);
impl_resp_v2!(OpenQueryJobChangeApprovalGroupsV2Resp, serde_json::Value);
impl_resp_v2!(
    OpenQueryPositionChangeApprovalGroupsV2Resp,
    serde_json::Value
);
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
impl_resp_v2!(GetBpByDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(ListBpV2Resp, ListData);
// company
impl_resp_v2!(ActiveCompanyV2Resp, serde_json::Value);
impl_resp_v2!(BatchGetCompanyV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeCompanyV2Resp, serde_json::Value);
// contract
impl_resp_v2!(SearchContractV2Resp, ListData);
// cost_allocation
impl_resp_v2!(BatchQueryCostAllocationV2Resp, serde_json::Value);
impl_resp_v2!(CreateVersionCostAllocationV2Resp, serde_json::Value);
impl_resp_v2!(RemoveVersionCostAllocationV2Resp, serde_json::Value);
impl_resp_v2!(UpdateVersionCostAllocationV2Resp, serde_json::Value);
// cost_center
impl_resp_v2!(CreateCostCenterV2Resp, serde_json::Value);
impl_resp_v2!(DeleteCostCenterV2Resp, ());
impl_resp_v2!(PatchCostCenterV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeCostCenterV2Resp, serde_json::Value);
impl_resp_v2!(SearchCostCenterV2Resp, ListData);
impl_resp_v2!(CreateCostCenterVersionV2Resp, serde_json::Value);
impl_resp_v2!(DeleteCostCenterVersionV2Resp, ());
impl_resp_v2!(PatchCostCenterVersionV2Resp, serde_json::Value);
// custom_org
impl_resp_v2!(ActiveCustomOrgV2Resp, serde_json::Value);
impl_resp_v2!(CreateCustomOrgV2Resp, serde_json::Value);
impl_resp_v2!(DeleteOrgCustomOrgV2Resp, serde_json::Value);
impl_resp_v2!(PatchCustomOrgV2Resp, serde_json::Value);
impl_resp_v2!(QueryCustomOrgV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeCustomOrgV2Resp, serde_json::Value);
impl_resp_v2!(UpdateRuleCustomOrgV2Resp, serde_json::Value);
// default_cost_center
impl_resp_v2!(BatchQueryDefaultCostCenterV2Resp, serde_json::Value);
impl_resp_v2!(CreateVersionDefaultCostCenterV2Resp, serde_json::Value);
impl_resp_v2!(RemoveVersionDefaultCostCenterV2Resp, serde_json::Value);
impl_resp_v2!(UpdateVersionDefaultCostCenterV2Resp, serde_json::Value);
// department
impl_resp_v2!(BatchGetDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(DeleteDepartmentV2Resp, ());
impl_resp_v2!(ParentsDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(PatchDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(QueryMultiTimelineDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(QueryOperationLogsDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(QueryTimelineDepartmentV2Resp, serde_json::Value);
impl_resp_v2!(SearchDepartmentV2Resp, ListData);
impl_resp_v2!(TreeDepartmentV2Resp, serde_json::Value);
// draft
impl_resp_v2!(GetDraftV2Resp, serde_json::Value);
// employee
impl_resp_v2!(BatchGetEmployeeV2Resp, serde_json::Value);
impl_resp_v2!(CreateEmployeeV2Resp, serde_json::Value);
impl_resp_v2!(SearchEmployeeV2Resp, ListData);
// employees_additional_job
impl_resp_v2!(BatchEmployeesAdditionalJobV2Resp, serde_json::Value);
impl_resp_v2!(CreateEmployeesAdditionalJobV2Resp, serde_json::Value);
impl_resp_v2!(DeleteEmployeesAdditionalJobV2Resp, ());
impl_resp_v2!(PatchEmployeesAdditionalJobV2Resp, serde_json::Value);
// employees_bp
impl_resp_v2!(BatchGetEmployeesBpV2Resp, serde_json::Value);
// employees_international_assignment
impl_resp_v2!(CreateEmployeesIntlAssignmentV2Resp, serde_json::Value);
impl_resp_v2!(DeleteEmployeesIntlAssignmentV2Resp, ());
impl_resp_v2!(ListEmployeesIntlAssignmentV2Resp, ListData);
impl_resp_v2!(PatchEmployeesIntlAssignmentV2Resp, serde_json::Value);
// employees_job_data
impl_resp_v2!(BatchGetEmployeesJobDataV2Resp, serde_json::Value);
impl_resp_v2!(QueryEmployeesJobDataV2Resp, serde_json::Value);
// enum
impl_resp_v2!(SearchEnumV2Resp, ListData);
// job
impl_resp_v2!(BatchGetJobV2Resp, serde_json::Value);
impl_resp_v2!(GetJobV2Resp, serde_json::Value);
impl_resp_v2!(ListJobV2Resp, ListData);
impl_resp_v2!(QueryMultiTimelineJobV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeJobV2Resp, serde_json::Value);
// job_change
impl_resp_v2!(CreateJobChangeV2Resp, serde_json::Value);
impl_resp_v2!(RevokeJobChangeV2Resp, serde_json::Value);
impl_resp_v2!(SearchJobChangeV2Resp, ListData);
// job_family
impl_resp_v2!(BatchGetJobFamilyV2Resp, serde_json::Value);
impl_resp_v2!(QueryMultiTimelineJobFamilyV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeJobFamilyV2Resp, serde_json::Value);
// job_grade
impl_resp_v2!(CreateJobGradeV2Resp, serde_json::Value);
impl_resp_v2!(DeleteJobGradeV2Resp, ());
impl_resp_v2!(PatchJobGradeV2Resp, serde_json::Value);
impl_resp_v2!(QueryJobGradeV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeJobGradeV2Resp, serde_json::Value);
// job_level
impl_resp_v2!(BatchGetJobLevelV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeJobLevelV2Resp, serde_json::Value);
// location
impl_resp_v2!(ActiveLocationV2Resp, serde_json::Value);
impl_resp_v2!(BatchGetLocationV2Resp, serde_json::Value);
impl_resp_v2!(PatchLocationV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangeLocationV2Resp, serde_json::Value);
impl_resp_v2!(CreateLocationAddressV2Resp, serde_json::Value);
impl_resp_v2!(DeleteLocationAddressV2Resp, ());
impl_resp_v2!(PatchLocationAddressV2Resp, serde_json::Value);
// offboarding
impl_resp_v2!(EditOffboardingV2Resp, serde_json::Value);
impl_resp_v2!(RevokeOffboardingV2Resp, serde_json::Value);
impl_resp_v2!(SubmitOffboardingV2Resp, serde_json::Value);
// pathway
impl_resp_v2!(ActivePathwayV2Resp, serde_json::Value);
impl_resp_v2!(BatchGetPathwayV2Resp, serde_json::Value);
impl_resp_v2!(CreatePathwayV2Resp, serde_json::Value);
impl_resp_v2!(DeletePathwayV2Resp, ());
impl_resp_v2!(PatchPathwayV2Resp, serde_json::Value);
// person
impl_resp_v2!(CreatePersonV2Resp, serde_json::Value);
impl_resp_v2!(PatchPersonV2Resp, serde_json::Value);
// position
impl_resp_v2!(ActivePositionV2Resp, serde_json::Value);
impl_resp_v2!(CreatePositionV2Resp, serde_json::Value);
impl_resp_v2!(DelPositionV2Resp, serde_json::Value);
impl_resp_v2!(PatchPositionV2Resp, serde_json::Value);
impl_resp_v2!(QueryPositionV2Resp, serde_json::Value);
impl_resp_v2!(QueryRecentChangePositionV2Resp, serde_json::Value);
// pre_hire
impl_resp_v2!(CompletePreHireV2Resp, serde_json::Value);
impl_resp_v2!(CreatePreHireV2Resp, serde_json::Value);
impl_resp_v2!(DeletePreHireV2Resp, ());
impl_resp_v2!(PatchPreHireV2Resp, serde_json::Value);
impl_resp_v2!(QueryPreHireV2Resp, serde_json::Value);
impl_resp_v2!(RestoreFlowInstancePreHireV2Resp, serde_json::Value);
impl_resp_v2!(SearchPreHireV2Resp, ListData);
impl_resp_v2!(TransformOnboardingTaskPreHireV2Resp, serde_json::Value);
impl_resp_v2!(TransitTaskPreHireV2Resp, serde_json::Value);
impl_resp_v2!(WithdrawOnboardingPreHireV2Resp, serde_json::Value);
// probation
impl_resp_v2!(EnableDisableAssessmentProbationV2Resp, serde_json::Value);
impl_resp_v2!(SearchProbationV2Resp, ListData);
impl_resp_v2!(SubmitProbationV2Resp, serde_json::Value);
impl_resp_v2!(WithdrawProbationV2Resp, serde_json::Value);
impl_resp_v2!(CreateProbationAssessmentV2Resp, serde_json::Value);
impl_resp_v2!(DeleteProbationAssessmentV2Resp, ());
impl_resp_v2!(PatchProbationAssessmentV2Resp, serde_json::Value);
// process
impl_resp_v2!(FlowVariableDataProcessV2Resp, serde_json::Value);
impl_resp_v2!(GetProcessV2Resp, serde_json::Value);
impl_resp_v2!(ListProcessV2Resp, ListData);
impl_resp_v2!(UpdateProcessApproverV2Resp, serde_json::Value);
impl_resp_v2!(UpdateProcessExtraV2Resp, serde_json::Value);
impl_resp_v2!(GetProcessFormVariableDataV2Resp, serde_json::Value);
impl_resp_v2!(UpdateProcessTransferV2Resp, serde_json::Value);
impl_resp_v2!(UpdateProcessRevokeV2Resp, serde_json::Value);
impl_resp_v2!(UpdateProcessWithdrawV2Resp, serde_json::Value);
// report_detail_row
impl_resp_v2!(BatchDeleteReportDetailRowV2Resp, serde_json::Value);
impl_resp_v2!(BatchSaveReportDetailRowV2Resp, serde_json::Value);
// signature_file
impl_resp_v2!(DownloadSignatureFileV2Resp, serde_json::Value);
impl_resp_v2!(ListSignatureFileV2Resp, ListData);
impl_resp_v2!(ListByBizIdSignatureFileV2Resp, ListData);
impl_resp_v2!(QuerySignatureFileV2Resp, serde_json::Value);
impl_resp_v2!(TerminateSignatureFileV2Resp, serde_json::Value);
impl_resp_v2!(ListByFileIdSignatureNodeV2Resp, ListData);
impl_resp_v2!(SearchSignatureTemplateV2Resp, ListData);
impl_resp_v2!(ListSignatureTemplateInfoWithThumbnailV2Resp, ListData);
// workforce_plan
impl_resp_v2!(ListWorkforcePlanV2Resp, ListData);
impl_resp_v2!(BatchWorkforcePlanDetailV2Resp, serde_json::Value);
impl_resp_v2!(BatchV2WorkforcePlanDetailV2Resp, serde_json::Value);
impl_resp_v2!(BatchDeleteWorkforcePlanDetailRowV2Resp, serde_json::Value);
impl_resp_v2!(BatchSaveWorkforcePlanDetailRowV2Resp, serde_json::Value);

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

macro_rules! post_method {
    ($fn_name:ident, $resp:ident, $data:ty, $path:expr) => {
        pub async fn $fn_name(
            &self,
            body: serde_json::Value,
            option: &RequestOption,
        ) -> Result<$resp, LarkError> {
            let mut api_req = ApiReq::new(http::Method::POST, $path);
            api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
            api_req.body = Some(ReqBody::json(&body)?);
            let (api_resp, raw) =
                transport::request_typed::<$data>(self.config, &api_req, option).await?;
            let (api_resp, code_error, data) = parse_v2(api_resp, raw);
            Ok($resp {
                api_resp,
                code_error,
                data,
            })
        }
    };
}

macro_rules! get_method {
    ($fn_name:ident, $resp:ident, $data:ty, $path:expr) => {
        pub async fn $fn_name(&self, option: &RequestOption) -> Result<$resp, LarkError> {
            let mut api_req = ApiReq::new(http::Method::GET, $path);
            api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
            let (api_resp, raw) =
                transport::request_typed::<$data>(self.config, &api_req, option).await?;
            let (api_resp, code_error, data) = parse_v2(api_resp, raw);
            Ok($resp {
                api_resp,
                code_error,
                data,
            })
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
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetApprovalGroupsV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        open_query_dept_change_list_by_ids,
        OpenQueryDeptChangeApprovalGroupsV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids"
    );
    post_method!(
        open_query_job_change_list_by_ids,
        OpenQueryJobChangeApprovalGroupsV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids"
    );
    post_method!(
        open_query_position_change_list_by_ids,
        OpenQueryPositionChangeApprovalGroupsV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids"
    );
}

// ── Approver resource ─────────────────────────────────────────────────────────

pub struct ApproverV2Resource<'a> {
    config: &'a Config,
}

impl ApproverV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListApproverV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v2/approvers");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListApproverV2Resp {
            api_resp,
            code_error,
            data,
        })
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

impl BpV2Resource<'_> {
    post_method!(
        get_by_department,
        GetBpByDepartmentV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/bps/get_by_department"
    );

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBpV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v2/bps");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListBpV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/companies/active"
    );
    post_method!(
        batch_get,
        BatchGetCompanyV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/companies/batch_get"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeCompanyV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/cost_allocations/batch_query"
    );
    post_method!(
        create_version,
        CreateVersionCostAllocationV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/cost_allocations/create_version"
    );
    post_method!(
        remove_version,
        RemoveVersionCostAllocationV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/cost_allocations/remove_version"
    );
    post_method!(
        update_version,
        UpdateVersionCostAllocationV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/cost_centers"
    );

    pub async fn delete(
        &self,
        cost_center_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCostCenterV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteCostCenterV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        cost_center_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchCostCenterV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchCostCenterV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    get_method!(
        query_recent_change,
        QueryRecentChangeCostCenterV2Resp,
        serde_json::Value,
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateCostCenterVersionV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}/versions");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateCostCenterVersionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        cost_center_id: &str,
        version_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCostCenterVersionV2Resp, LarkError> {
        let path =
            format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}/versions/{version_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteCostCenterVersionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        cost_center_id: &str,
        version_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchCostCenterVersionV2Resp, LarkError> {
        let path =
            format!("/open-apis/corehr/v2/cost_centers/{cost_center_id}/versions/{version_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchCostCenterVersionV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/custom_orgs/active"
    );
    post_method!(
        create,
        CreateCustomOrgV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/custom_orgs"
    );
    post_method!(
        delete_org,
        DeleteOrgCustomOrgV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/custom_orgs/delete_org"
    );

    pub async fn patch(
        &self,
        org_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchCustomOrgV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/custom_orgs/{org_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchCustomOrgV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        query,
        QueryCustomOrgV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/custom_orgs/query"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeCustomOrgV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/custom_orgs/query_recent_change"
    );
    post_method!(
        update_rule,
        UpdateRuleCustomOrgV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/default_cost_centers/batch_query"
    );
    post_method!(
        create_version,
        CreateVersionDefaultCostCenterV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/default_cost_centers/create_version"
    );
    post_method!(
        remove_version,
        RemoveVersionDefaultCostCenterV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/default_cost_centers/remove_version"
    );
    post_method!(
        update_version,
        UpdateVersionDefaultCostCenterV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/departments/batch_get"
    );

    pub async fn delete(
        &self,
        department_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDepartmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteDepartmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        parents,
        ParentsDepartmentV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/departments/parents"
    );

    pub async fn patch(
        &self,
        department_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchDepartmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchDepartmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        query_multi_timeline,
        QueryMultiTimelineDepartmentV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/departments/query_multi_timeline"
    );
    post_method!(
        query_operation_logs,
        QueryOperationLogsDepartmentV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/departments/query_operation_logs"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeDepartmentV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/departments/query_recent_change"
    );
    post_method!(
        query_timeline,
        QueryTimelineDepartmentV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/departments/query_timeline"
    );
    post_method!(
        search,
        SearchDepartmentV2Resp,
        ListData,
        "/open-apis/corehr/v2/departments/search"
    );
    post_method!(
        tree,
        TreeDepartmentV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/departments/tree"
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
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetDraftV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/employees/batch_get"
    );
    post_method!(
        create,
        CreateEmployeeV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/employees/additional_jobs/batch"
    );
    post_method!(
        create,
        CreateEmployeesAdditionalJobV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/employees/additional_jobs"
    );

    pub async fn delete(
        &self,
        additional_job_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmployeesAdditionalJobV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/employees/additional_jobs/{additional_job_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteEmployeesAdditionalJobV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        additional_job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmployeesAdditionalJobV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/employees/additional_jobs/{additional_job_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchEmployeesAdditionalJobV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
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
        serde_json::Value,
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
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteEmployeesIntlAssignmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEmployeesIntlAssignmentV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v2/employees/international_assignments",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListEmployeesIntlAssignmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        international_assignment_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmployeesIntlAssignmentV2Resp, LarkError> {
        let path = format!(
            "/open-apis/corehr/v2/employees/international_assignments/{international_assignment_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchEmployeesIntlAssignmentV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/employees/job_datas/batch_get"
    );
    post_method!(
        query,
        QueryEmployeesJobDataV2Resp,
        serde_json::Value,
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

impl JobV2Resource<'_> {
    post_method!(
        batch_get,
        BatchGetJobV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/jobs/batch_get"
    );

    pub async fn get(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/jobs/{job_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetJobV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v2/jobs");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListJobV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        query_multi_timeline,
        QueryMultiTimelineJobV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/jobs/query_multi_timeline"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/job_changes"
    );

    pub async fn revoke(
        &self,
        job_change_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RevokeJobChangeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/job_changes/{job_change_id}/revoke");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RevokeJobChangeV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        search,
        SearchJobChangeV2Resp,
        ListData,
        "/open-apis/corehr/v2/job_changes/search"
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
        serde_json::Value,
        "/open-apis/corehr/v2/job_families/batch_get"
    );
    post_method!(
        query_multi_timeline,
        QueryMultiTimelineJobFamilyV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/job_families/query_multi_timeline"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobFamilyV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/job_grades"
    );

    pub async fn delete(
        &self,
        job_grade_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobGradeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/job_grades/{job_grade_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteJobGradeV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_grade_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobGradeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/job_grades/{job_grade_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchJobGradeV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        query,
        QueryJobGradeV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/job_grades/query"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobGradeV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/job_levels/batch_get"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangeJobLevelV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/locations/active"
    );
    post_method!(
        batch_get,
        BatchGetLocationV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/locations/batch_get"
    );

    pub async fn patch(
        &self,
        location_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchLocationV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchLocationV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    get_method!(
        query_recent_change,
        QueryRecentChangeLocationV2Resp,
        serde_json::Value,
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateLocationAddressV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}/addresses");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateLocationAddressV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        location_id: &str,
        address_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLocationAddressV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}/addresses/{address_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteLocationAddressV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        location_id: &str,
        address_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchLocationAddressV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/locations/{location_id}/addresses/{address_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchLocationAddressV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/offboardings/edit"
    );
    post_method!(
        revoke,
        RevokeOffboardingV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/offboardings/revoke"
    );
    post_method!(
        submit_v2,
        SubmitOffboardingV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/pathways/active"
    );
    post_method!(
        batch_get,
        BatchGetPathwayV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/pathways/batch_get"
    );
    post_method!(
        create,
        CreatePathwayV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/pathways"
    );

    pub async fn delete(
        &self,
        pathway_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePathwayV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pathways/{pathway_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeletePathwayV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        pathway_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPathwayV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pathways/{pathway_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchPathwayV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/persons"
    );

    pub async fn patch(
        &self,
        person_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPersonV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/persons/{person_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchPersonV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/positions/active"
    );
    post_method!(
        create,
        CreatePositionV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/positions"
    );
    post_method!(
        del_position,
        DelPositionV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/positions/del_position"
    );

    pub async fn patch(
        &self,
        position_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPositionV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/positions/{position_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchPositionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        query,
        QueryPositionV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/positions/query"
    );
    get_method!(
        query_recent_change,
        QueryRecentChangePositionV2Resp,
        serde_json::Value,
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CompletePreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}/complete");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CompletePreHireV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        create,
        CreatePreHireV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/pre_hires"
    );

    pub async fn delete(
        &self,
        pre_hire_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeletePreHireV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        pre_hire_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchPreHireV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        query,
        QueryPreHireV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/pre_hires/query"
    );
    post_method!(
        restore_flow_instance,
        RestoreFlowInstancePreHireV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/pre_hires/transform_onboarding_task"
    );

    pub async fn transit_task(
        &self,
        pre_hire_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<TransitTaskPreHireV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/pre_hires/{pre_hire_id}/transit_task");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(TransitTaskPreHireV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        withdraw_onboarding,
        WithdrawOnboardingPreHireV2Resp,
        serde_json::Value,
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
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/probation/submit"
    );
    post_method!(
        withdraw,
        WithdrawProbationV2Resp,
        serde_json::Value,
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
        serde_json::Value,
        "/open-apis/corehr/v2/probation/assessments"
    );

    pub async fn delete(
        &self,
        assessment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteProbationAssessmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/probation/assessments/{assessment_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteProbationAssessmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        assessment_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchProbationAssessmentV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/probation/assessments/{assessment_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchProbationAssessmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Process resource ──────────────────────────────────────────────────────────

pub struct ProcessV2Resource<'a> {
    config: &'a Config,
}

impl ProcessV2Resource<'_> {
    pub async fn flow_variable_data(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<FlowVariableDataProcessV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/flow_variable_data");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(FlowVariableDataProcessV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<GetProcessV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetProcessV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListProcessV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v2/processes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListProcessV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateProcessApproverV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/approvers/{approver_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateProcessApproverV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateProcessExtraV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/extra");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateProcessExtraV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetProcessFormVariableDataV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateProcessTransferV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/processes/{process_id}/transfer");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateProcessTransferV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateProcessRevokeV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/process_revoke/{process_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateProcessRevokeV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateProcessWithdrawV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/process_withdraw/{process_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateProcessWithdrawV2Resp {
            api_resp,
            code_error,
            data,
        })
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
        serde_json::Value,
        "/open-apis/corehr/v2/report_detail_row/batchDelete"
    );
    post_method!(
        batch_save,
        BatchSaveReportDetailRowV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/report_detail_row/batchSave"
    );
}

// ── SignatureFile resource ────────────────────────────────────────────────────

pub struct SignatureFileV2Resource<'a> {
    config: &'a Config,
}

impl SignatureFileV2Resource<'_> {
    pub async fn download(
        &self,
        signature_file_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<DownloadSignatureFileV2Resp, LarkError> {
        let path = format!("/open-apis/corehr/v2/signature_files/{signature_file_id}/download");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DownloadSignatureFileV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSignatureFileV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v2/signature_files");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListSignatureFileV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list_by_biz_id(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListByBizIdSignatureFileV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v2/signature_files/list_by_biz_id",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListByBizIdSignatureFileV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    post_method!(
        query,
        QuerySignatureFileV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/signature_files/query"
    );
    post_method!(
        terminate,
        TerminateSignatureFileV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/signature_files/terminate"
    );
}

// ── SignatureNode resource ────────────────────────────────────────────────────

pub struct SignatureNodeV2Resource<'a> {
    config: &'a Config,
}

impl SignatureNodeV2Resource<'_> {
    pub async fn list_by_file_id(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListByFileIdSignatureNodeV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v2/signature_nodes/list_by_file_id",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListByFileIdSignatureNodeV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── SignatureTemplate resource ────────────────────────────────────────────────

pub struct SignatureTemplateV2Resource<'a> {
    config: &'a Config,
}

impl SignatureTemplateV2Resource<'_> {
    pub async fn search(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchSignatureTemplateV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v2/signature_templates/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchSignatureTemplateV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── SignatureTemplateInfoWithThumbnail resource ───────────────────────────────

pub struct SignatureTemplateInfoWithThumbnailV2Resource<'a> {
    config: &'a Config,
}

impl SignatureTemplateInfoWithThumbnailV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSignatureTemplateInfoWithThumbnailV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v2/signature_template_info_with_thumbnails",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListSignatureTemplateInfoWithThumbnailV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── WorkforcePlan resource ────────────────────────────────────────────────────

pub struct WorkforcePlanV2Resource<'a> {
    config: &'a Config,
}

impl WorkforcePlanV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkforcePlanV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v2/workforce_plans");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListWorkforcePlanV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── WorkforcePlanDetail resource ──────────────────────────────────────────────

pub struct WorkforcePlanDetailV2Resource<'a> {
    config: &'a Config,
}

impl WorkforcePlanDetailV2Resource<'_> {
    post_method!(
        batch,
        BatchWorkforcePlanDetailV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/workforce_plan_details/batch"
    );
    post_method!(
        batch_v2,
        BatchV2WorkforcePlanDetailV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/workforce_plan_details/batch_v2"
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
        serde_json::Value,
        "/open-apis/corehr/v2/workforce_plan_detail_row/batchDelete"
    );
    post_method!(
        batch_save,
        BatchSaveWorkforcePlanDetailRowV2Resp,
        serde_json::Value,
        "/open-apis/corehr/v2/workforce_plan_detail_row/batchSave"
    );
}
