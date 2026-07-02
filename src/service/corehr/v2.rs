use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

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
            RestRequest::new(
                self.config,
                http::Method::POST,
                $path,
                vec![AccessTokenType::Tenant],
                option,
            )
            .json_body(&body)?
            .send_v2_response::<$data, $resp>()
            .await
        }
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
        .send_v2_response::<serde_json::Value, GetApprovalGroupsV2Resp>()
        .await
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
        serde_json::Value,
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchCostCenterV2Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateCostCenterVersionV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchCostCenterVersionV2Resp>()
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
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchCustomOrgV2Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchDepartmentV2Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetDraftV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchEmployeesAdditionalJobV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchEmployeesIntlAssignmentV2Resp>()
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
        serde_json::Value,
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
        .send_v2_response::<serde_json::Value, GetJobV2Resp>()
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, RevokeJobChangeV2Resp>()
        .await
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchJobGradeV2Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchLocationV2Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateLocationAddressV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchLocationAddressV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchPathwayV2Resp>()
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
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchPersonV2Resp>()
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
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchPositionV2Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CompletePreHireV2Resp>()
        .await
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchPreHireV2Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, TransitTaskPreHireV2Resp>()
        .await
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, PatchProbationAssessmentV2Resp>()
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
        .send_v2_response::<serde_json::Value, FlowVariableDataProcessV2Resp>()
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
        .send_v2_response::<serde_json::Value, GetProcessV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, UpdateProcessApproverV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, UpdateProcessExtraV2Resp>()
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
        .send_v2_response::<serde_json::Value, GetProcessFormVariableDataV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, UpdateProcessTransferV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, UpdateProcessRevokeV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, UpdateProcessWithdrawV2Resp>()
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
        body: serde_json::Value,
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
        .send_v2_response::<serde_json::Value, DownloadSignatureFileV2Resp>()
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
