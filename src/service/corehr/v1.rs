use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Shared sub-types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnumValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_initiator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_initiating_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_finish_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Enum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OffboardingChecklist {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_finish_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_process_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OffboardingData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OffboardingInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrbp_id: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_offboarding_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_reason: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_reason_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_block_list: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain_account: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_insurance_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provident_fund_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_noncompete_agreement: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noncompete_agreement_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noncompete_agreement_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noncompete_agreement_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noncompete_agreement_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_type: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_attendance_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_transfer_with_workforce: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Offboarding {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiating_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_info: Option<OffboardingInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_checklist: Option<OffboardingChecklist>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
}

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Employee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub race_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_contract_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_expected_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_rate: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason: Option<EnumValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_manager: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Company {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nature: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<Vec<EnumValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_representative: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<Vec<EnumValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_company: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_manager: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_office_address: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<Vec<I18nText>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_usage: Option<Vec<EnumValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_language_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Currency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_alpha_3_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkingHoursType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendance_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_for_new_job: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment: Option<Employee>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeListData {
    #[serde(default)]
    pub items: Vec<Employee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<Department>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentListData {
    #[serde(default)]
    pub items: Vec<Department>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevelData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<JobLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevelListData {
    #[serde(default)]
    pub items: Vec<JobLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompanyData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompanyListData {
    #[serde(default)]
    pub items: Vec<Company>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationListData {
    #[serde(default)]
    pub items: Vec<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurrencyListData {
    #[serde(default)]
    pub items: Vec<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkingHoursTypeListData {
    #[serde(default)]
    pub items: Vec<WorkingHoursType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(GetEmployeeResp, EmployeeData);
impl_resp!(ListEmployeeResp, EmployeeListData);
impl_resp!(GetDepartmentResp, DepartmentData);
impl_resp!(ListDepartmentResp, DepartmentListData);
impl_resp!(GetJobLevelResp, JobLevelData);
impl_resp!(ListJobLevelResp, JobLevelListData);
impl_resp!(GetCompanyResp, CompanyData);
impl_resp!(ListCompanyResp, CompanyListData);
impl_resp!(GetLocationResp, LocationData);
impl_resp!(ListLocationResp, LocationListData);
impl_resp!(ListCurrencyResp, CurrencyListData);
impl_resp!(ListWorkingHoursTypeResp, WorkingHoursTypeListData);

// ── Resources ──

pub struct EmployeeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetEmployeeQuery<'a> {
    pub employment_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetEmployeeQuery<'a> {
    pub fn new(employment_id: &'a str) -> Self {
        Self {
            employment_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListEmployeeQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListEmployeeQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
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

impl<'a> EmployeeResource<'a> {
    pub async fn get(
        &self,
        employment_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetEmployeeResp, LarkError> {
        let query = GetEmployeeQuery::new(employment_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetEmployeeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employments/{}", query.employment_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<EmployeeData>()
        .await?;
        Ok(GetEmployeeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEmployeeResp, LarkError> {
        let query = ListEmployeeQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEmployeeResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/employments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send::<EmployeeListData>()
        .await?;
        Ok(ListEmployeeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct DepartmentResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetDepartmentQuery<'a> {
    pub fn new(department_id: &'a str) -> Self {
        Self {
            department_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListDepartmentQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListDepartmentQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
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

impl<'a> DepartmentResource<'a> {
    pub async fn get(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetDepartmentResp, LarkError> {
        let query = GetDepartmentQuery::new(department_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDepartmentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/departments/{}", query.department_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<DepartmentData>()
        .await?;
        Ok(GetDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDepartmentResp, LarkError> {
        let query = ListDepartmentQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDepartmentResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/departments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send::<DepartmentListData>()
        .await?;
        Ok(ListDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct JobLevelResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobLevelQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListJobLevelQuery<'a> {
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

impl<'a> JobLevelResource<'a> {
    pub async fn get(
        &self,
        job_level_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobLevelResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<JobLevelData>()
        .await?;
        Ok(GetJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobLevelResp, LarkError> {
        let query = ListJobLevelQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobLevelQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobLevelResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/job_levels",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send::<JobLevelListData>()
        .await?;
        Ok(ListJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CompanyResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListCompanyQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListCompanyQuery<'a> {
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

impl<'a> CompanyResource<'a> {
    pub async fn get(
        &self,
        company_id: &str,
        option: &RequestOption,
    ) -> Result<GetCompanyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<CompanyData>()
        .await?;
        Ok(GetCompanyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCompanyResp, LarkError> {
        let query = ListCompanyQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCompanyQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCompanyResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/companies",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send::<CompanyListData>()
        .await?;
        Ok(ListCompanyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateCompanyResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/companies",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateCompanyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        company_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCompanyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteCompanyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        company_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchCompanyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchCompanyResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct LocationResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListLocationQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListLocationQuery<'a> {
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

impl<'a> LocationResource<'a> {
    pub async fn get(
        &self,
        location_id: &str,
        option: &RequestOption,
    ) -> Result<GetLocationResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/locations/{location_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<LocationData>()
        .await?;
        Ok(GetLocationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListLocationResp, LarkError> {
        let query = ListLocationQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListLocationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListLocationResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/locations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send::<LocationListData>()
        .await?;
        Ok(ListLocationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CurrencyResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListCurrencyQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListCurrencyQuery<'a> {
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

impl<'a> CurrencyResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCurrencyResp, LarkError> {
        let query = ListCurrencyQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCurrencyQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCurrencyResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/currencies",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send::<CurrencyListData>()
        .await?;
        Ok(ListCurrencyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct WorkingHoursTypeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListWorkingHoursTypeQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListWorkingHoursTypeQuery<'a> {
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

impl<'a> WorkingHoursTypeResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkingHoursTypeResp, LarkError> {
        let query = ListWorkingHoursTypeQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListWorkingHoursTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListWorkingHoursTypeResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/working_hours_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send::<WorkingHoursTypeListData>()
        .await?;
        Ok(ListWorkingHoursTypeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Helpers for newer resources (use Option<CodeError> pattern) ──

// ── Response types for new resources ──

impl_resp_v2!(SearchAssignedUserResp, serde_json::Value);
impl_resp_v2!(AddRoleAssignAuthorizationResp, serde_json::Value);
impl_resp_v2!(GetByParamAuthorizationResp, serde_json::Value);
impl_resp_v2!(QueryAuthorizationResp, serde_json::Value);
impl_resp_v2!(RemoveRoleAssignAuthorizationResp, serde_json::Value);
impl_resp_v2!(UpdateRoleAssignAuthorizationResp, serde_json::Value);
impl_resp_v2!(ConvertCommonDataIdResp, serde_json::Value);
impl_resp_v2!(AddEnumOptionCommonDataMetaDataResp, serde_json::Value);
impl_resp_v2!(EditEnumOptionCommonDataMetaDataResp, serde_json::Value);
impl_resp_v2!(MatchCompensationStandardResp, serde_json::Value);
impl_resp_v2!(CreateContractResp, serde_json::Value);
impl_resp_v2!(DeleteContractResp, ());
impl_resp_v2!(GetContractResp, serde_json::Value);
impl_resp_v2!(ListContractResp, serde_json::Value);
impl_resp_v2!(PatchContractResp, serde_json::Value);
impl_resp_v2!(GetCountryRegionResp, serde_json::Value);
impl_resp_v2!(ListCountryRegionResp, serde_json::Value);
impl_resp_v2!(GetCurrencyResp, serde_json::Value);
impl_resp_v2!(GetByParamCustomFieldResp, serde_json::Value);
impl_resp_v2!(ListObjectApiNameCustomFieldResp, serde_json::Value);
impl_resp_v2!(QueryCustomFieldResp, serde_json::Value);
impl_resp_v2!(CreateDepartmentResp, serde_json::Value);
impl_resp_v2!(DeleteDepartmentResp, ());
impl_resp_v2!(PatchDepartmentResp, serde_json::Value);
impl_resp_v2!(CreateEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(DeleteEmployeeTypeResp, ());
impl_resp_v2!(GetEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(ListEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(PatchEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(CreateEmploymentResp, serde_json::Value);
impl_resp_v2!(DeleteEmploymentResp, ());
impl_resp_v2!(PatchEmploymentResp, serde_json::Value);
impl_resp_v2!(GetFileResp, serde_json::Value);
impl_resp_v2!(CreateJobResp, serde_json::Value);
impl_resp_v2!(DeleteJobResp, ());
impl_resp_v2!(GetJobResp, serde_json::Value);
impl_resp_v2!(ListJobResp, serde_json::Value);
impl_resp_v2!(PatchJobResp, serde_json::Value);
impl_resp_v2!(CreateJobChangeResp, serde_json::Value);
impl_resp_v2!(CreateJobDataResp, serde_json::Value);
impl_resp_v2!(DeleteJobDataResp, ());
impl_resp_v2!(GetJobDataResp, serde_json::Value);
impl_resp_v2!(ListJobDataResp, serde_json::Value);
impl_resp_v2!(PatchJobDataResp, serde_json::Value);
impl_resp_v2!(CreateJobFamilyResp, serde_json::Value);
impl_resp_v2!(DeleteJobFamilyResp, ());
impl_resp_v2!(GetJobFamilyResp, serde_json::Value);
impl_resp_v2!(ListJobFamilyResp, serde_json::Value);
impl_resp_v2!(PatchJobFamilyResp, serde_json::Value);
impl_resp_v2!(CreateJobLevelResp, serde_json::Value);
impl_resp_v2!(DeleteJobLevelResp, ());
impl_resp_v2!(PatchJobLevelResp, serde_json::Value);
impl_resp_v2!(CalendarByScopeLeaveResp, serde_json::Value);
impl_resp_v2!(LeaveBalancesLeaveResp, serde_json::Value);
impl_resp_v2!(LeaveRequestHistoryLeaveResp, serde_json::Value);
impl_resp_v2!(LeaveTypesLeaveResp, serde_json::Value);
impl_resp_v2!(WorkCalendarLeaveResp, serde_json::Value);
impl_resp_v2!(WorkCalendarDateLeaveResp, serde_json::Value);
impl_resp_v2!(CreateLeaveGrantingRecordResp, serde_json::Value);
impl_resp_v2!(DeleteLeaveGrantingRecordResp, ());
impl_resp_v2!(CreateLocationResp, serde_json::Value);
impl_resp_v2!(DeleteLocationResp, ());
impl_resp_v2!(CreateNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(DeleteNationalIdTypeResp, ());
impl_resp_v2!(GetNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(ListNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(PatchNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(QueryOffboardingResp, serde_json::Value);
impl_resp_v2!(SearchOffboardingResp, SearchOffboardingRespData);
impl_resp_v2!(SubmitOffboardingResp, serde_json::Value);
impl_resp_v2!(CreatePersonResp, serde_json::Value);
impl_resp_v2!(DeletePersonResp, ());
impl_resp_v2!(GetPersonResp, serde_json::Value);
impl_resp_v2!(PatchPersonResp, serde_json::Value);
impl_resp_v2!(UploadPersonResp, serde_json::Value);
impl_resp_v2!(DeletePreHireResp, ());
impl_resp_v2!(GetPreHireResp, serde_json::Value);
impl_resp_v2!(ListPreHireResp, serde_json::Value);
impl_resp_v2!(PatchPreHireResp, serde_json::Value);
impl_resp_v2!(GetProcessFormVariableDataResp, serde_json::Value);
impl_resp_v2!(ListSecurityGroupResp, serde_json::Value);
impl_resp_v2!(QuerySecurityGroupResp, serde_json::Value);
impl_resp_v2!(GetSubdivisionResp, serde_json::Value);
impl_resp_v2!(ListSubdivisionResp, serde_json::Value);
impl_resp_v2!(GetSubregionResp, serde_json::Value);
impl_resp_v2!(ListSubregionResp, serde_json::Value);
impl_resp_v2!(QueryTransferReasonResp, serde_json::Value);
impl_resp_v2!(QueryTransferTypeResp, serde_json::Value);
impl_resp_v2!(CreateWorkingHoursTypeResp, serde_json::Value);
impl_resp_v2!(DeleteWorkingHoursTypeResp, ());
impl_resp_v2!(GetWorkingHoursTypeResp, serde_json::Value);
impl_resp_v2!(PatchWorkingHoursTypeResp, serde_json::Value);
impl_resp_v2!(CreateCompanyResp, serde_json::Value);
impl_resp_v2!(DeleteCompanyResp, ());
impl_resp_v2!(PatchCompanyResp, serde_json::Value);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOffboardingRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Offboarding>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone)]
struct PageIteratorState<T> {
    next_page_token: Option<String>,
    request_page_token: Option<String>,
    items: VecDeque<T>,
    started: bool,
    exhausted: bool,
    limit: Option<usize>,
    emitted: usize,
}

impl<T> Default for PageIteratorState<T> {
    fn default() -> Self {
        Self {
            next_page_token: None,
            request_page_token: None,
            items: VecDeque::new(),
            started: false,
            exhausted: false,
            limit: None,
            emitted: 0,
        }
    }
}

impl<T> PageIteratorState<T> {
    fn limit(mut self, limit: usize) -> Self {
        self.limit = (limit > 0).then_some(limit);
        self
    }

    fn with_page_token(mut self, page_token: Option<String>) -> Self {
        self.request_page_token = page_token;
        self.started = self.request_page_token.is_some();
        self.exhausted = false;
        self
    }

    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn page_token_for_request(&self) -> Option<&str> {
        if self.started {
            self.request_page_token
                .as_deref()
                .or(self.next_page_token.as_deref())
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.limit.is_some_and(|limit| self.emitted >= limit) {
            return None;
        }
        let item = self.items.pop_front()?;
        self.emitted += 1;
        Some(item)
    }

    fn should_fetch(&self) -> bool {
        self.limit.is_none_or(|limit| self.emitted < limit)
            && self.items.is_empty()
            && !self.exhausted
            && (!self.started
                || self.request_page_token.is_some()
                || self.next_page_token.is_some())
    }

    fn accept_page(
        &mut self,
        items: Option<Vec<T>>,
        page_token: Option<String>,
        has_more: Option<bool>,
    ) {
        let prev_page_token = self.next_page_token.clone();
        self.started = true;
        self.request_page_token = None;
        self.items = items.unwrap_or_default().into();
        self.next_page_token = if self.items.is_empty() {
            prev_page_token
        } else {
            page_token
        };
        self.exhausted =
            self.items.is_empty() || !has_more.unwrap_or(false) || self.next_page_token.is_none();
    }
}

// ── New Resource Structs ──

pub struct AssignedUserResource<'a> {
    config: &'a Config,
}

impl<'a> AssignedUserResource<'a> {
    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchAssignedUserResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/assigned_users/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchAssignedUserResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct AuthorizationResource<'a> {
    config: &'a Config,
}

impl<'a> AuthorizationResource<'a> {
    pub async fn add_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddRoleAssignAuthorizationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/add_role_assign",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(AddRoleAssignAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_by_param(
        &self,
        option: &RequestOption,
    ) -> Result<GetByParamAuthorizationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/authorizations/get_by_param",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetByParamAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryAuthorizationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/authorizations/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveRoleAssignAuthorizationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/remove_role_assign",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(RemoveRoleAssignAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateRoleAssignAuthorizationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/update_role_assign",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateRoleAssignAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CommonDataIdResource<'a> {
    config: &'a Config,
}

impl<'a> CommonDataIdResource<'a> {
    pub async fn convert(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ConvertCommonDataIdResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/id/convert",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ConvertCommonDataIdResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CommonDataMetaDataResource<'a> {
    config: &'a Config,
}

impl<'a> CommonDataMetaDataResource<'a> {
    pub async fn add_enum_option(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddEnumOptionCommonDataMetaDataResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/meta_data/add_enum_option",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(AddEnumOptionCommonDataMetaDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn edit_enum_option(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EditEnumOptionCommonDataMetaDataResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/meta_data/edit_enum_option",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(EditEnumOptionCommonDataMetaDataResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CompensationStandardResource<'a> {
    config: &'a Config,
}

impl<'a> CompensationStandardResource<'a> {
    pub async fn match_standard(
        &self,
        option: &RequestOption,
    ) -> Result<MatchCompensationStandardResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/compensation_standards/match",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(MatchCompensationStandardResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ContractResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListContractQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListContractQuery<'a> {
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

impl<'a> ContractResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateContractResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/contracts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateContractResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        contract_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteContractResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteContractResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        contract_id: &str,
        option: &RequestOption,
    ) -> Result<GetContractResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetContractResp {
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
    ) -> Result<ListContractResp, LarkError> {
        let query = ListContractQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListContractQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListContractResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/contracts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListContractResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        contract_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchContractResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchContractResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CountryRegionResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListCountryRegionQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListCountryRegionQuery<'a> {
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

impl<'a> CountryRegionResource<'a> {
    pub async fn get(
        &self,
        country_region_id: &str,
        option: &RequestOption,
    ) -> Result<GetCountryRegionResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/country_regions/{country_region_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetCountryRegionResp {
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
    ) -> Result<ListCountryRegionResp, LarkError> {
        let query = ListCountryRegionQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCountryRegionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCountryRegionResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/country_regions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListCountryRegionResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CustomFieldResource<'a> {
    config: &'a Config,
}

impl<'a> CustomFieldResource<'a> {
    pub async fn get_by_param(
        &self,
        option: &RequestOption,
    ) -> Result<GetByParamCustomFieldResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/get_by_param",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetByParamCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list_object_api_name(
        &self,
        option: &RequestOption,
    ) -> Result<ListObjectApiNameCustomFieldResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/list_object_api_name",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListObjectApiNameCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryCustomFieldResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct EmployeeTypeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListEmployeeTypeQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListEmployeeTypeQuery<'a> {
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

impl<'a> EmployeeTypeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEmployeeTypeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/employee_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        employee_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmployeeTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        employee_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetEmployeeTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetEmployeeTypeResp {
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
    ) -> Result<ListEmployeeTypeResp, LarkError> {
        let query = ListEmployeeTypeQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEmployeeTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEmployeeTypeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/employee_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        employee_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmployeeTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct EmploymentResource<'a> {
    config: &'a Config,
}

impl<'a> EmploymentResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEmploymentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/employments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateEmploymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        employment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmploymentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employments/{employment_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteEmploymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        employment_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmploymentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employments/{employment_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchEmploymentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

impl<'a> FileResource<'a> {
    pub async fn get(&self, id: &str, option: &RequestOption) -> Result<GetFileResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/files/{id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetFileResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListJobQuery<'a> {
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

impl<'a> JobResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/jobs",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, job_id: &str, option: &RequestOption) -> Result<GetJobResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetJobResp {
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
    ) -> Result<ListJobResp, LarkError> {
        let query = ListJobQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/jobs",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchJobResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobChangeResource<'a> {
    config: &'a Config,
}

impl<'a> JobChangeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobChangeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_changes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateJobChangeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobDataResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobDataQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListJobDataQuery<'a> {
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

impl<'a> JobDataResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobDataResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_datas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_data_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobDataResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        job_data_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobDataResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetJobDataResp {
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
    ) -> Result<ListJobDataResp, LarkError> {
        let query = ListJobDataQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobDataResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/job_datas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_data_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobDataResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobFamilyResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobFamilyQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListJobFamilyQuery<'a> {
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

impl<'a> JobFamilyResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobFamilyResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_families",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobFamilyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobFamilyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetJobFamilyResp {
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
    ) -> Result<ListJobFamilyResp, LarkError> {
        let query = ListJobFamilyQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobFamilyQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobFamilyResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/job_families",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_family_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobFamilyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct LeaveResource<'a> {
    config: &'a Config,
}

impl<'a> LeaveResource<'a> {
    pub async fn calendar_by_scope(
        &self,
        option: &RequestOption,
    ) -> Result<CalendarByScopeLeaveResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/calendar_by_scope",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CalendarByScopeLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn leave_balances(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveBalancesLeaveResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_balances",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(LeaveBalancesLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn leave_request_history(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveRequestHistoryLeaveResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_request_history",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(LeaveRequestHistoryLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn leave_types(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveTypesLeaveResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(LeaveTypesLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn work_calendar(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WorkCalendarLeaveResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/leaves/work_calendar",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(WorkCalendarLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn work_calendar_date(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WorkCalendarDateLeaveResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/leaves/work_calendar_date",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(WorkCalendarDateLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct LeaveGrantingRecordResource<'a> {
    config: &'a Config,
}

impl<'a> LeaveGrantingRecordResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateLeaveGrantingRecordResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/leave_granting_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateLeaveGrantingRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        leave_granting_record_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLeaveGrantingRecordResp, LarkError> {
        let path =
            format!("/open-apis/corehr/v1/leave_granting_records/{leave_granting_record_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteLeaveGrantingRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct NationalIdTypeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListNationalIdTypeQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListNationalIdTypeQuery<'a> {
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

impl<'a> NationalIdTypeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateNationalIdTypeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/national_id_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        national_id_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteNationalIdTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        national_id_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetNationalIdTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetNationalIdTypeResp {
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
    ) -> Result<ListNationalIdTypeResp, LarkError> {
        let query = ListNationalIdTypeQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListNationalIdTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListNationalIdTypeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/national_id_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        national_id_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchNationalIdTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct OffboardingResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct SearchOffboardingQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SearchOffboardingQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
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

#[derive(Debug, Clone)]
pub struct SearchOffboardingIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<Offboarding>,
    body: serde_json::Value,
    page_size: Option<i32>,
    user_id_type: Option<String>,
}

impl<'a> SearchOffboardingIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.state = self.state.with_page_token(Some(page_token.into()));
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<Offboarding>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = OffboardingResource {
            config: self.config,
        };
        let resp = resource
            .search_page(
                self.page_size,
                self.state.page_token_for_request(),
                self.user_id_type.as_deref(),
                self.body.clone(),
                option,
            )
            .await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(data.items, data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

impl<'a> OffboardingResource<'a> {
    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryOffboardingResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryOffboardingResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchOffboardingResp, LarkError> {
        self.search_page(None, None, None, body, option).await
    }

    pub async fn search_page(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchOffboardingResp, LarkError> {
        let query = SearchOffboardingQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.search_by_query(&query, body, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchOffboardingQuery<'_>,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchOffboardingResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .json_body(&body)?
        .send_v2::<SearchOffboardingRespData>()
        .await?;
        Ok(SearchOffboardingResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub fn search_by_iterator(
        &self,
        body: serde_json::Value,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
    ) -> SearchOffboardingIterator<'a> {
        SearchOffboardingIterator {
            config: self.config,
            state: PageIteratorState::default(),
            body,
            page_size,
            user_id_type: user_id_type.map(|v| v.to_owned()),
        }
    }

    pub async fn submit(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SubmitOffboardingResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/submit",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SubmitOffboardingResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct PersonResource<'a> {
    config: &'a Config,
}

impl<'a> PersonResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreatePersonResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/persons",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreatePersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        person_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePersonResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeletePersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        person_id: &str,
        option: &RequestOption,
    ) -> Result<GetPersonResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetPersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        person_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPersonResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchPersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn upload(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadPersonResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/persons/upload",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UploadPersonResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct PreHireResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListPreHireQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListPreHireQuery<'a> {
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

impl<'a> PreHireResource<'a> {
    pub async fn delete(
        &self,
        pre_hire_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePreHireResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeletePreHireResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        pre_hire_id: &str,
        option: &RequestOption,
    ) -> Result<GetPreHireResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetPreHireResp {
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
    ) -> Result<ListPreHireResp, LarkError> {
        let query = ListPreHireQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPreHireQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPreHireResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/pre_hires",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListPreHireResp {
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
    ) -> Result<PatchPreHireResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchPreHireResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ProcessFormVariableDataResource<'a> {
    config: &'a Config,
}

impl<'a> ProcessFormVariableDataResource<'a> {
    pub async fn get(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<GetProcessFormVariableDataResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/processes/{process_id}/form_variable_data");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetProcessFormVariableDataResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct SecurityGroupResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListSecurityGroupQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListSecurityGroupQuery<'a> {
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

impl<'a> SecurityGroupResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSecurityGroupResp, LarkError> {
        let query = ListSecurityGroupQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSecurityGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListSecurityGroupResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/security_groups",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListSecurityGroupResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QuerySecurityGroupResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/security_groups/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QuerySecurityGroupResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct SubdivisionResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListSubdivisionQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListSubdivisionQuery<'a> {
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

impl<'a> SubdivisionResource<'a> {
    pub async fn get(
        &self,
        subdivision_id: &str,
        option: &RequestOption,
    ) -> Result<GetSubdivisionResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/subdivisions/{subdivision_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetSubdivisionResp {
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
    ) -> Result<ListSubdivisionResp, LarkError> {
        let query = ListSubdivisionQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSubdivisionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListSubdivisionResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/subdivisions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListSubdivisionResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct SubregionResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListSubregionQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListSubregionQuery<'a> {
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

impl<'a> SubregionResource<'a> {
    pub async fn get(
        &self,
        subregion_id: &str,
        option: &RequestOption,
    ) -> Result<GetSubregionResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/subregions/{subregion_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetSubregionResp {
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
    ) -> Result<ListSubregionResp, LarkError> {
        let query = ListSubregionQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSubregionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListSubregionResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/subregions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListSubregionResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct TransferReasonResource<'a> {
    config: &'a Config,
}

impl<'a> TransferReasonResource<'a> {
    pub async fn query(
        &self,
        option: &RequestOption,
    ) -> Result<QueryTransferReasonResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/transfer_reasons/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryTransferReasonResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct TransferTypeResource<'a> {
    config: &'a Config,
}

impl<'a> TransferTypeResource<'a> {
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTransferTypeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/transfer_types/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryTransferTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Methods added to existing resources (JobLevel, Currency, Location, WorkingHoursType) ──
// These are added via new impl blocks on the existing structs.

impl<'a> JobLevelResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobLevelResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_levels",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateJobLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_level_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobLevelResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteJobLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_level_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobLevelResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchJobLevelResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> CurrencyResource<'a> {
    pub async fn get(
        &self,
        currency_id: &str,
        option: &RequestOption,
    ) -> Result<GetCurrencyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/currencies/{currency_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetCurrencyResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> LocationResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateLocationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/locations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateLocationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        location_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLocationResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/locations/{location_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteLocationResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> DepartmentResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/departments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        department_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDepartmentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/departments/{department_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        department_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchDepartmentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/departments/{department_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> WorkingHoursTypeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateWorkingHoursTypeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/working_hours_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        working_hours_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteWorkingHoursTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        working_hours_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetWorkingHoursTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        working_hours_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchWorkingHoursTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub employee: EmployeeResource<'a>,
    pub department: DepartmentResource<'a>,
    pub job_level: JobLevelResource<'a>,
    pub company: CompanyResource<'a>,
    pub location: LocationResource<'a>,
    pub currency: CurrencyResource<'a>,
    pub working_hours_type: WorkingHoursTypeResource<'a>,
    pub assigned_user: AssignedUserResource<'a>,
    pub authorization: AuthorizationResource<'a>,
    pub common_data_id: CommonDataIdResource<'a>,
    pub common_data_meta_data: CommonDataMetaDataResource<'a>,
    pub compensation_standard: CompensationStandardResource<'a>,
    pub contract: ContractResource<'a>,
    pub country_region: CountryRegionResource<'a>,
    pub custom_field: CustomFieldResource<'a>,
    pub employee_type: EmployeeTypeResource<'a>,
    pub employment: EmploymentResource<'a>,
    pub file: FileResource<'a>,
    pub job: JobResource<'a>,
    pub job_change: JobChangeResource<'a>,
    pub job_data: JobDataResource<'a>,
    pub job_family: JobFamilyResource<'a>,
    pub leave: LeaveResource<'a>,
    pub leave_granting_record: LeaveGrantingRecordResource<'a>,
    pub national_id_type: NationalIdTypeResource<'a>,
    pub offboarding: OffboardingResource<'a>,
    pub person: PersonResource<'a>,
    pub pre_hire: PreHireResource<'a>,
    pub process_form_variable_data: ProcessFormVariableDataResource<'a>,
    pub security_group: SecurityGroupResource<'a>,
    pub subdivision: SubdivisionResource<'a>,
    pub subregion: SubregionResource<'a>,
    pub transfer_reason: TransferReasonResource<'a>,
    pub transfer_type: TransferTypeResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            employee: EmployeeResource { config },
            department: DepartmentResource { config },
            job_level: JobLevelResource { config },
            company: CompanyResource { config },
            location: LocationResource { config },
            currency: CurrencyResource { config },
            working_hours_type: WorkingHoursTypeResource { config },
            assigned_user: AssignedUserResource { config },
            authorization: AuthorizationResource { config },
            common_data_id: CommonDataIdResource { config },
            common_data_meta_data: CommonDataMetaDataResource { config },
            compensation_standard: CompensationStandardResource { config },
            contract: ContractResource { config },
            country_region: CountryRegionResource { config },
            custom_field: CustomFieldResource { config },
            employee_type: EmployeeTypeResource { config },
            employment: EmploymentResource { config },
            file: FileResource { config },
            job: JobResource { config },
            job_change: JobChangeResource { config },
            job_data: JobDataResource { config },
            job_family: JobFamilyResource { config },
            leave: LeaveResource { config },
            leave_granting_record: LeaveGrantingRecordResource { config },
            national_id_type: NationalIdTypeResource { config },
            offboarding: OffboardingResource { config },
            person: PersonResource { config },
            pre_hire: PreHireResource { config },
            process_form_variable_data: ProcessFormVariableDataResource { config },
            security_group: SecurityGroupResource { config },
            subdivision: SubdivisionResource { config },
            subregion: SubregionResource { config },
            transfer_reason: TransferReasonResource { config },
            transfer_type: TransferTypeResource { config },
        }
    }
}
