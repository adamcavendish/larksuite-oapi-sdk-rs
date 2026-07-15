use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageIteratorState, PageQuery, RestRequest};

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
    pub name: Option<CorehrName>,
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
pub struct CorehrIdInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorehrName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorehrEnumFieldOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CorehrName>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConvertCommonDataIdRespData {
    #[serde(default)]
    pub items: Vec<CorehrIdInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddEnumOptionCommonDataMetaDataRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_field_api_name: Option<String>,
    #[serde(default)]
    pub enum_field_options: Vec<CorehrEnumFieldOption>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditEnumOptionCommonDataMetaDataRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_field_api_name: Option<String>,
    #[serde(default)]
    pub enum_field_options: Vec<CorehrEnumFieldOption>,
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
    pub phone_list: Option<Vec<Phone>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<Address>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_list: Option<Vec<Email>>,
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
    pub cost_center_rate: Option<Vec<CorehrSupportCostCenterItem>>,
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
    pub hiberarchy_common: Option<HiberarchyCommon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_manager: Option<Vec<I18nText>>,
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
    pub hiberarchy_common: Option<HiberarchyCommon>,
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
    pub hiberarchy_common: Option<HiberarchyCommon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_usage: Option<Vec<EnumValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<Address>>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorehrEnum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18n>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryRegion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_3_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_2_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorehrCurrency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_alpha_3_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NationalIdType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_rule_description: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<CorehrEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subdivision {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdivision_type: Option<CorehrEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subregion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdivision_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superior_subregion_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_transfer_reason_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_employee_type: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Job {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_prefer_manual_encoding: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorehrSupportCostCenterItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
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
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_start_reason: Option<CorehrEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_outcome: Option<CorehrEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_direct_manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_rate: Option<Vec<CorehrSupportCostCenterItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours_v2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<CorehrEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<CorehrEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_reason: Option<CorehrEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFamily {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selectable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18n>>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkingHoursTypeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type: Option<WorkingHoursType>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryRegionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region: Option<CountryRegion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryRegionListData {
    #[serde(default)]
    pub items: Vec<CountryRegion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorehrCurrencyData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<CorehrCurrency>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NationalIdTypeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_type: Option<NationalIdType>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NationalIdTypeListData {
    #[serde(default)]
    pub items: Vec<NationalIdType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubdivisionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdivision: Option<Subdivision>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubdivisionListData {
    #[serde(default)]
    pub items: Vec<Subdivision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubregionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subregion: Option<Subregion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubregionListData {
    #[serde(default)]
    pub items: Vec<Subregion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferReasonListData {
    #[serde(default)]
    pub items: Vec<TransferReason>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferTypeListData {
    #[serde(default)]
    pub items: Vec<TransferType>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeTypeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<EmployeeType>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeTypeListData {
    #[serde(default)]
    pub items: Vec<EmployeeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobResponseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobListData {
    #[serde(default)]
    pub items: Vec<Job>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobDataResponseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data: Option<JobData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobDataListData {
    #[serde(default)]
    pub items: Vec<JobData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFamilyData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family: Option<JobFamily>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFamilyListData {
    #[serde(default)]
    pub items: Vec<JobFamily>,
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

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchAssignedUserRespData {
    #[serde(default)]
    pub items: Vec<RoleAuthorization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddRoleAssignAuthorizationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetByParamAuthorizationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_authorization: Option<RoleAuthorization>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryAuthorizationRespData {
    #[serde(default)]
    pub items: Vec<RoleAuthorization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRoleAssignAuthorizationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRoleAssignAuthorizationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchCompensationStandardRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<CpstGrade>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateContractRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetContractRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListContractRespData {
    #[serde(default)]
    pub items: Vec<Contract>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchContractRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetByParamCustomFieldRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<CustomField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListObjectApiNameCustomFieldRespData {
    #[serde(default)]
    pub items: Vec<Object>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryCustomFieldRespData {
    #[serde(default)]
    pub items: Vec<CustomField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDepartmentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<DepartmentCreate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateEmploymentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment: Option<EmploymentCreate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchEmploymentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment: Option<Employment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateJobChangeRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalendarByScopeLeaveRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_wk_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveBalancesLeaveRespData {
    #[serde(default)]
    pub employment_leave_balance_list: Vec<EmploymentLeaveBalance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveRequestHistoryLeaveRespData {
    #[serde(default)]
    pub leave_request_list: Vec<LeaveRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveTypesLeaveRespData {
    #[serde(default)]
    pub leave_type_list: Vec<LeaveType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkCalendarLeaveRespData {
    #[serde(default)]
    pub work_calendars: Vec<WorkCalendarDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkCalendarDateLeaveRespData {
    #[serde(default)]
    pub calendar_dates: Vec<WkCalendarDate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLeaveGrantingRecordRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_granting_record: Option<LeaveGrantingRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryOffboardingRespData {
    #[serde(default)]
    pub items: Vec<OffboardingReason>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitOffboardingRespData {
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
    pub add_block_list: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePersonRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPersonRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchPersonRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadPersonRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPreHireRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire: Option<PreHire>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPreHireRespData {
    #[serde(default)]
    pub items: Vec<PreHireQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchPreHireRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire: Option<PreHire>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetProcessFormVariableDataRespData {
    #[serde(default)]
    pub field_variable_values: Vec<FormFieldVariable>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSecurityGroupRespData {
    #[serde(default)]
    pub items: Vec<SecurityGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuerySecurityGroupRespData {
    #[serde(default)]
    pub hrbp_list: Vec<Hrbp>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_local_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_western_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    pub address_type_list: Vec<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentFieldSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<ResponseEnum>,
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
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default)]
    pub bank_account_usage: Vec<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BpmDataengineI18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculatedFieldSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommonSchemaConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_field_setting: Option<TextFieldSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_field_setting: Option<NumberFieldSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_field_setting: Option<EnumFieldSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_field_setting: Option<LookupFieldSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_field_setting: Option<DateTimeFieldSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_field_setting: Option<AttachmentFieldSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_field_setting: Option<ImageFieldSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculated_field_setting: Option<CalculatedFieldSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommonSchemaOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_party_company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_status: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_times: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CpstBandWidth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CpstCurrency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CpstI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CpstGrade {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_standard_value: Option<CpstGradeStandardValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<CpstCurrency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CpstI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CpstGradeStandardValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_object: Option<ReferenceObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_type: Option<CpstStandardType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub band_width: Option<CpstBandWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CpstI18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CpstStandardType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_schema_config: Option<CommonSchemaConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseCustomFieldData {
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
pub struct CustomName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTimeFieldSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<HiberarchyCommon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_prefer_manual_encoding: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dependent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id_v2: Option<String>,
    #[serde(default)]
    pub national_id_list: Vec<NationalId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spouses_working_status: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_covered_by_health_insurance: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_allowed_for_tax_deduction: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Education {
    #[serde(default)]
    pub school: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_of_education: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub field_of_study: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school_name: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study_name: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_end_date: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Email {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_usage: Option<ResponseEnum>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmergencyContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<ResponseEnum>,
    #[serde(default)]
    pub phone_ist: Vec<Phone>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Employment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prehire_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenure: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_probation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_employment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_worker: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<ResponseEnum>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default)]
    pub work_email_list: Vec<Email>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_offboarding: Option<ResponseEnum>,
    #[serde(default)]
    pub cost_center_list: Vec<JobDataCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire_employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<ResponseEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmploymentCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prehire_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenure: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_probation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_employment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<ResponseEnum>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default)]
    pub work_email_list: Vec<Email>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_offboarding: Option<ResponseEnum>,
    #[serde(default)]
    pub cost_center_list: Vec<JobDataCostCenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rehire_employment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmploymentLeaveBalance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub employment_name: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<String>,
    #[serde(default)]
    pub leave_balance_list: Vec<LeaveBalance>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseEnum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    #[serde(default)]
    pub display: Vec<ResponseI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnumFieldSetting {
    #[serde(default)]
    pub enum_field_option_list: Vec<CommonSchemaOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<FilterRuleValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<FilterRuleValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_value_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterExpression {
    #[serde(default)]
    pub conditions: Vec<FilterCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logic_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterRuleValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_name: Option<BpmDataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_value: Option<FormVariableValueInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableBoolValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableDateValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableDatetimeValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableDepartmentValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableEmploymentValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableEnumValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<BpmDataengineI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<BpmDataengineI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableFileValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableI18nValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BpmDataengineI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableListObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<FormFieldVariableTextValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<FormFieldVariableNumberValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_value: Option<FormFieldVariableDateValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_value: Option<FormFieldVariableEmploymentValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<FormFieldVariableDatetimeValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<FormFieldVariableEnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<FormFieldVariableNullValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<FormFieldVariableBoolValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_value: Option<FormFieldVariableDepartmentValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_value: Option<FormFieldVariableFileValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<FormFieldVariableI18nValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FormFieldVariableObjectValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_value: Option<FormFieldVariableRecordValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableListValue {
    #[serde(default)]
    pub values: Vec<FormFieldVariableListObject>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableNullValue {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableNumberValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableObjectValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_api_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableRecordValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<FormFieldVariableRecordValueExample>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableRecordValueExample {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region: Option<FormVariableValueInfoExample>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldVariableTextValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormVariableValueInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<FormFieldVariableTextValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<FormFieldVariableNumberValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_value: Option<FormFieldVariableDateValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_value: Option<FormFieldVariableEmploymentValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<FormFieldVariableDatetimeValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<FormFieldVariableEnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<FormFieldVariableNullValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<FormFieldVariableBoolValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_value: Option<FormFieldVariableDepartmentValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_value: Option<FormFieldVariableFileValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<FormFieldVariableI18nValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FormFieldVariableObjectValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_value: Option<FormFieldVariableListValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_value: Option<FormFieldVariableRecordValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormVariableValueInfoExample {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FormFieldVariableObjectValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HiberarchyCommon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub name: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub description: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Hrbp {
    #[serde(default)]
    pub employment_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseI18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImageFieldSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_style: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobDataCostCenter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveBalance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default)]
    pub leave_type_name: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_cycles_left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub this_cycle_total: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub this_cycle_taken: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_balance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration_unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_cycle_accrual: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_in_current_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taken: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taken_history_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_balance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taken_current_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_granted: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_balance_excluding_under_approval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_quantity_under_approval: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveExtendItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveGrantingRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_by: Option<i32>,
    #[serde(default)]
    pub reason: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveProcessInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_apply_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub employment_name: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default)]
    pub leave_type_name: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration_unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_request_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deducted: Option<bool>,
    #[serde(default)]
    pub details: Vec<LeaveRequestDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    #[serde(default)]
    pub leave_process_id: Vec<String>,
    #[serde(default)]
    pub leave_correct_process_id: Vec<String>,
    #[serde(default)]
    pub leave_cancel_process_id: Vec<String>,
    #[serde(default)]
    pub leave_return_process_id: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wd_paid_type: Option<i32>,
    #[serde(default)]
    pub leave_correct_process_info: Vec<LeaveProcessInfo>,
    #[serde(default)]
    pub workday_extend_infos: Vec<LeaveExtendItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_tag_conf: Option<LeaveTagConf>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveRequestDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_duration_unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveSubtype {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default)]
    pub leave_type_name: Vec<ResponseI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveTagConf {
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default)]
    pub leave_type_name: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default)]
    pub leave_subtype_list: Vec<LeaveSubtype>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LookupFieldSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_obj_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManagementScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Name {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NumberFieldSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_field_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub round_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_total_places: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Object {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseObjectFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OffboardingReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_unique_identifier: Option<String>,
    #[serde(default)]
    pub name: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_offboarding_reason_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrgTruncation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<SecurityGroup>,
    #[serde(default)]
    pub assigned_organization_list: Vec<Vec<AssignedOrganization>>,
    #[serde(default)]
    pub grantor_rule_list: Vec<PermissionSecurityGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssignedOrganization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_name: Option<Name>,
    #[serde(default)]
    pub org_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionSecurityGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_dimension: Option<RuleDimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<FilterExpression>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Person {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub name_list: Vec<PersonName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub race: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<ResponseEnum>,
    #[serde(default)]
    pub phone_list: Vec<Phone>,
    #[serde(default)]
    pub address_list: Vec<Address>,
    #[serde(default)]
    pub email_list: Vec<Email>,
    #[serde(default)]
    pub work_experience_list: Vec<WorkExperience>,
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
    pub profile_image_id: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default)]
    pub resident_tax_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_level_of_education: Option<Education>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_degree_of_education: Option<Education>,
    #[serde(default)]
    pub personal_profile: Vec<PersonalProfile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name_type: Option<ResponseEnum>,
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
    pub social: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<ResponseEnum>,
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
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile_type: Option<ResponseEnum>,
    #[serde(default)]
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Phone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_area_code: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_usage: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreHire {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default)]
    pub cost_center_rate: Vec<SupportCostCenterItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<ResponseEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreHireQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<ResponseEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<ResponseEnum>,
    #[serde(default)]
    pub cost_center_rate: Vec<SupportCostCenterItem>,
    #[serde(default)]
    pub work_email_list: Vec<Email>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleAuthorization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default)]
    pub permission_detail_list: Vec<PermissionDetail>,
    #[serde(default)]
    pub management_scope_list: Vec<ManagementScope>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleDimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<Name>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default)]
    pub org_truncation: Vec<OrgTruncation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupportCostCenterItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextFieldSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multilingual: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_url_type: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranferEmploymentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_employee_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseCustomFieldData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    pub original_cost_center_rate: Vec<SupportCostCenterItem>,
    #[serde(default)]
    pub target_cost_center_rate: Vec<SupportCostCenterItem>,
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
    pub original_pathway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_pathway: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WkCalendarDate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WkCalendarI18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkCalendarDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_name: Option<WkCalendarI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkExperience {
    #[serde(default)]
    pub company_organization: Vec<ResponseI18n>,
    #[serde(default)]
    pub department: Vec<ResponseI18n>,
    #[serde(default)]
    pub job: Vec<ResponseI18n>,
    #[serde(default)]
    pub description: Vec<ResponseI18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<ResponseObjectFieldData>,
}
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<EmployeeData, GetEmployeeResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/employments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send_response::<EmployeeListData, ListEmployeeResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<DepartmentData, GetDepartmentResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/departments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send_response::<DepartmentListData, ListDepartmentResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<JobLevelData, GetJobLevelResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/job_levels",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_response::<JobLevelListData, ListJobLevelResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<CompanyData, GetCompanyResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/companies",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_response::<CompanyListData, ListCompanyResp>()
        .await
    }

    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateCompanyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/companies",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CompanyData, CreateCompanyResp>()
        .await
    }

    pub async fn delete(
        &self,
        company_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCompanyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteCompanyResp>()
        .await
    }

    pub async fn patch(
        &self,
        company_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchCompanyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CompanyData, PatchCompanyResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<LocationData, GetLocationResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/locations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_response::<LocationListData, ListLocationResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/currencies",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_response::<CurrencyListData, ListCurrencyResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/working_hours_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_response::<WorkingHoursTypeListData, ListWorkingHoursTypeResp>()
        .await
    }
}

// ── Helpers for newer resources (use Option<CodeError> pattern) ──

// ── Response types for new resources ──

impl_resp_v2!(SearchAssignedUserResp, SearchAssignedUserRespData);
impl_resp_v2!(
    AddRoleAssignAuthorizationResp,
    AddRoleAssignAuthorizationRespData
);
impl_resp_v2!(GetByParamAuthorizationResp, GetByParamAuthorizationRespData);
impl_resp_v2!(QueryAuthorizationResp, QueryAuthorizationRespData);
impl_resp_v2!(
    RemoveRoleAssignAuthorizationResp,
    RemoveRoleAssignAuthorizationRespData
);
impl_resp_v2!(
    UpdateRoleAssignAuthorizationResp,
    UpdateRoleAssignAuthorizationRespData
);
impl_resp_v2!(ConvertCommonDataIdResp, ConvertCommonDataIdRespData);
impl_resp_v2!(
    AddEnumOptionCommonDataMetaDataResp,
    AddEnumOptionCommonDataMetaDataRespData
);
impl_resp_v2!(
    EditEnumOptionCommonDataMetaDataResp,
    EditEnumOptionCommonDataMetaDataRespData
);
impl_resp_v2!(
    MatchCompensationStandardResp,
    MatchCompensationStandardRespData
);
impl_resp_v2!(CreateContractResp, CreateContractRespData);
impl_resp_v2!(DeleteContractResp, ());
impl_resp_v2!(GetContractResp, GetContractRespData);
impl_resp_v2!(ListContractResp, ListContractRespData);
impl_resp_v2!(PatchContractResp, PatchContractRespData);
impl_resp_v2!(GetCountryRegionResp, CountryRegionData);
impl_resp_v2!(ListCountryRegionResp, CountryRegionListData);
impl_resp_v2!(GetCurrencyResp, CorehrCurrencyData);
impl_resp_v2!(GetByParamCustomFieldResp, GetByParamCustomFieldRespData);
impl_resp_v2!(
    ListObjectApiNameCustomFieldResp,
    ListObjectApiNameCustomFieldRespData
);
impl_resp_v2!(QueryCustomFieldResp, QueryCustomFieldRespData);
impl_resp_v2!(CreateDepartmentResp, CreateDepartmentRespData);
impl_resp_v2!(DeleteDepartmentResp, ());
impl_resp_v2!(PatchDepartmentResp, DepartmentData);
impl_resp_v2!(CreateEmployeeTypeResp, EmployeeTypeData);
impl_resp_v2!(DeleteEmployeeTypeResp, ());
impl_resp_v2!(GetEmployeeTypeResp, EmployeeTypeData);
impl_resp_v2!(ListEmployeeTypeResp, EmployeeTypeListData);
impl_resp_v2!(PatchEmployeeTypeResp, EmployeeTypeData);
impl_resp_v2!(CreateEmploymentResp, CreateEmploymentRespData);
impl_resp_v2!(DeleteEmploymentResp, ());
impl_resp_v2!(PatchEmploymentResp, PatchEmploymentRespData);
impl_resp_v2!(GetFileResp, ());
impl_resp_v2!(CreateJobResp, JobResponseData);
impl_resp_v2!(DeleteJobResp, ());
impl_resp_v2!(GetJobResp, JobResponseData);
impl_resp_v2!(ListJobResp, JobListData);
impl_resp_v2!(PatchJobResp, JobResponseData);
impl_resp_v2!(CreateJobChangeResp, CreateJobChangeRespData);
impl_resp_v2!(CreateJobDataResp, JobDataResponseData);
impl_resp_v2!(DeleteJobDataResp, ());
impl_resp_v2!(GetJobDataResp, JobDataResponseData);
impl_resp_v2!(ListJobDataResp, JobDataListData);
impl_resp_v2!(PatchJobDataResp, JobDataResponseData);
impl_resp_v2!(CreateJobFamilyResp, JobFamilyData);
impl_resp_v2!(DeleteJobFamilyResp, ());
impl_resp_v2!(GetJobFamilyResp, JobFamilyData);
impl_resp_v2!(ListJobFamilyResp, JobFamilyListData);
impl_resp_v2!(PatchJobFamilyResp, JobFamilyData);
impl_resp_v2!(CreateJobLevelResp, JobLevelData);
impl_resp_v2!(DeleteJobLevelResp, ());
impl_resp_v2!(PatchJobLevelResp, JobLevelData);
impl_resp_v2!(CalendarByScopeLeaveResp, CalendarByScopeLeaveRespData);
impl_resp_v2!(LeaveBalancesLeaveResp, LeaveBalancesLeaveRespData);
impl_resp_v2!(
    LeaveRequestHistoryLeaveResp,
    LeaveRequestHistoryLeaveRespData
);
impl_resp_v2!(LeaveTypesLeaveResp, LeaveTypesLeaveRespData);
impl_resp_v2!(WorkCalendarLeaveResp, WorkCalendarLeaveRespData);
impl_resp_v2!(WorkCalendarDateLeaveResp, WorkCalendarDateLeaveRespData);
impl_resp_v2!(
    CreateLeaveGrantingRecordResp,
    CreateLeaveGrantingRecordRespData
);
impl_resp_v2!(DeleteLeaveGrantingRecordResp, ());
impl_resp_v2!(CreateLocationResp, LocationData);
impl_resp_v2!(DeleteLocationResp, ());
impl_resp_v2!(CreateNationalIdTypeResp, NationalIdTypeData);
impl_resp_v2!(DeleteNationalIdTypeResp, ());
impl_resp_v2!(GetNationalIdTypeResp, NationalIdTypeData);
impl_resp_v2!(ListNationalIdTypeResp, NationalIdTypeListData);
impl_resp_v2!(PatchNationalIdTypeResp, NationalIdTypeData);
impl_resp_v2!(QueryOffboardingResp, QueryOffboardingRespData);
impl_resp_v2!(SearchOffboardingResp, SearchOffboardingRespData);
impl_resp_v2!(SubmitOffboardingResp, SubmitOffboardingRespData);
impl_resp_v2!(CreatePersonResp, CreatePersonRespData);
impl_resp_v2!(DeletePersonResp, ());
impl_resp_v2!(GetPersonResp, GetPersonRespData);
impl_resp_v2!(PatchPersonResp, PatchPersonRespData);
impl_resp_v2!(UploadPersonResp, UploadPersonRespData);
impl_resp_v2!(DeletePreHireResp, ());
impl_resp_v2!(GetPreHireResp, GetPreHireRespData);
impl_resp_v2!(ListPreHireResp, ListPreHireRespData);
impl_resp_v2!(PatchPreHireResp, PatchPreHireRespData);
impl_resp_v2!(
    GetProcessFormVariableDataResp,
    GetProcessFormVariableDataRespData
);
impl_resp_v2!(ListSecurityGroupResp, ListSecurityGroupRespData);
impl_resp_v2!(QuerySecurityGroupResp, QuerySecurityGroupRespData);
impl_resp_v2!(GetSubdivisionResp, SubdivisionData);
impl_resp_v2!(ListSubdivisionResp, SubdivisionListData);
impl_resp_v2!(GetSubregionResp, SubregionData);
impl_resp_v2!(ListSubregionResp, SubregionListData);
impl_resp_v2!(QueryTransferReasonResp, TransferReasonListData);
impl_resp_v2!(QueryTransferTypeResp, TransferTypeListData);
impl_resp_v2!(CreateWorkingHoursTypeResp, WorkingHoursTypeData);
impl_resp_v2!(DeleteWorkingHoursTypeResp, ());
impl_resp_v2!(GetWorkingHoursTypeResp, WorkingHoursTypeData);
impl_resp_v2!(PatchWorkingHoursTypeResp, WorkingHoursTypeData);
impl_resp_v2!(CreateCompanyResp, CompanyData);
impl_resp_v2!(DeleteCompanyResp, ());
impl_resp_v2!(PatchCompanyResp, CompanyData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOffboardingRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Offboarding>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/assigned_users/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<SearchAssignedUserRespData, SearchAssignedUserResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/add_role_assign",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<AddRoleAssignAuthorizationRespData, AddRoleAssignAuthorizationResp>()
        .await
    }

    pub async fn get_by_param(
        &self,
        option: &RequestOption,
    ) -> Result<GetByParamAuthorizationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/authorizations/get_by_param",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetByParamAuthorizationRespData, GetByParamAuthorizationResp>()
        .await
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryAuthorizationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/authorizations/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<QueryAuthorizationRespData, QueryAuthorizationResp>()
        .await
    }

    pub async fn remove_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveRoleAssignAuthorizationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/remove_role_assign",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<RemoveRoleAssignAuthorizationRespData, RemoveRoleAssignAuthorizationResp>()
        .await
    }

    pub async fn update_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateRoleAssignAuthorizationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/update_role_assign",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<UpdateRoleAssignAuthorizationRespData, UpdateRoleAssignAuthorizationResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/id/convert",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<ConvertCommonDataIdRespData, ConvertCommonDataIdResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/meta_data/add_enum_option",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<AddEnumOptionCommonDataMetaDataRespData, AddEnumOptionCommonDataMetaDataResp>()
        .await
    }

    pub async fn edit_enum_option(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EditEnumOptionCommonDataMetaDataResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/meta_data/edit_enum_option",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<EditEnumOptionCommonDataMetaDataRespData, EditEnumOptionCommonDataMetaDataResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/compensation_standards/match",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<MatchCompensationStandardRespData, MatchCompensationStandardResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/contracts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateContractRespData, CreateContractResp>()
        .await
    }

    pub async fn delete(
        &self,
        contract_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteContractResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteContractResp>()
        .await
    }

    pub async fn get(
        &self,
        contract_id: &str,
        option: &RequestOption,
    ) -> Result<GetContractResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetContractRespData, GetContractResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/contracts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListContractRespData, ListContractResp>()
        .await
    }

    pub async fn patch(
        &self,
        contract_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchContractResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchContractRespData, PatchContractResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<CountryRegionData, GetCountryRegionResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/country_regions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<CountryRegionListData, ListCountryRegionResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/get_by_param",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetByParamCustomFieldRespData, GetByParamCustomFieldResp>()
        .await
    }

    pub async fn list_object_api_name(
        &self,
        option: &RequestOption,
    ) -> Result<ListObjectApiNameCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/list_object_api_name",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<ListObjectApiNameCustomFieldRespData, ListObjectApiNameCustomFieldResp>(
        )
        .await
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<QueryCustomFieldRespData, QueryCustomFieldResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/employee_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<EmployeeTypeData, CreateEmployeeTypeResp>()
        .await
    }

    pub async fn delete(
        &self,
        employee_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmployeeTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteEmployeeTypeResp>()
        .await
    }

    pub async fn get(
        &self,
        employee_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetEmployeeTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<EmployeeTypeData, GetEmployeeTypeResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/employee_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<EmployeeTypeListData, ListEmployeeTypeResp>()
        .await
    }

    pub async fn patch(
        &self,
        employee_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmployeeTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<EmployeeTypeData, PatchEmployeeTypeResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/employments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateEmploymentRespData, CreateEmploymentResp>()
        .await
    }

    pub async fn delete(
        &self,
        employment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmploymentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employments/{employment_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteEmploymentResp>()
        .await
    }

    pub async fn patch(
        &self,
        employment_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmploymentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/employments/{employment_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchEmploymentRespData, PatchEmploymentResp>()
        .await
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

impl<'a> FileResource<'a> {
    pub async fn get(&self, id: &str, option: &RequestOption) -> Result<GetFileResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/files/{id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), GetFileResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/jobs",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobResponseData, CreateJobResp>()
        .await
    }

    pub async fn delete(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteJobResp>()
        .await
    }

    pub async fn get(&self, job_id: &str, option: &RequestOption) -> Result<GetJobResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<JobResponseData, GetJobResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/jobs",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<JobListData, ListJobResp>()
        .await
    }

    pub async fn patch(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobResponseData, PatchJobResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_changes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateJobChangeRespData, CreateJobChangeResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_datas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobDataResponseData, CreateJobDataResp>()
        .await
    }

    pub async fn delete(
        &self,
        job_data_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobDataResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteJobDataResp>()
        .await
    }

    pub async fn get(
        &self,
        job_data_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobDataResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<JobDataResponseData, GetJobDataResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/job_datas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<JobDataListData, ListJobDataResp>()
        .await
    }

    pub async fn patch(
        &self,
        job_data_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobDataResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobDataResponseData, PatchJobDataResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_families",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobFamilyData, CreateJobFamilyResp>()
        .await
    }

    pub async fn delete(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobFamilyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteJobFamilyResp>()
        .await
    }

    pub async fn get(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobFamilyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<JobFamilyData, GetJobFamilyResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/job_families",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<JobFamilyListData, ListJobFamilyResp>()
        .await
    }

    pub async fn patch(
        &self,
        job_family_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobFamilyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobFamilyData, PatchJobFamilyResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/calendar_by_scope",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<CalendarByScopeLeaveRespData, CalendarByScopeLeaveResp>()
        .await
    }

    pub async fn leave_balances(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveBalancesLeaveResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_balances",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<LeaveBalancesLeaveRespData, LeaveBalancesLeaveResp>()
        .await
    }

    pub async fn leave_request_history(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveRequestHistoryLeaveResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_request_history",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<LeaveRequestHistoryLeaveRespData, LeaveRequestHistoryLeaveResp>()
        .await
    }

    pub async fn leave_types(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveTypesLeaveResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<LeaveTypesLeaveRespData, LeaveTypesLeaveResp>()
        .await
    }

    pub async fn work_calendar(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WorkCalendarLeaveResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/leaves/work_calendar",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<WorkCalendarLeaveRespData, WorkCalendarLeaveResp>()
        .await
    }

    pub async fn work_calendar_date(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WorkCalendarDateLeaveResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/leaves/work_calendar_date",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<WorkCalendarDateLeaveRespData, WorkCalendarDateLeaveResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/leave_granting_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateLeaveGrantingRecordRespData, CreateLeaveGrantingRecordResp>()
        .await
    }

    pub async fn delete(
        &self,
        leave_granting_record_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLeaveGrantingRecordResp, LarkError> {
        let path =
            format!("/open-apis/corehr/v1/leave_granting_records/{leave_granting_record_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteLeaveGrantingRecordResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/national_id_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<NationalIdTypeData, CreateNationalIdTypeResp>()
        .await
    }

    pub async fn delete(
        &self,
        national_id_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteNationalIdTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteNationalIdTypeResp>()
        .await
    }

    pub async fn get(
        &self,
        national_id_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetNationalIdTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<NationalIdTypeData, GetNationalIdTypeResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/national_id_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<NationalIdTypeListData, ListNationalIdTypeResp>()
        .await
    }

    pub async fn patch(
        &self,
        national_id_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchNationalIdTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<NationalIdTypeData, PatchNationalIdTypeResp>()
        .await
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
        let resp = Box::pin(resource.search_page(
            self.page_size,
            self.state.page_token_for_request(),
            self.user_id_type.as_deref(),
            self.body.clone(),
            option,
        ))
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<QueryOffboardingRespData, QueryOffboardingResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .json_body(&body)?
        .send_v2_response::<SearchOffboardingRespData, SearchOffboardingResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/submit",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<SubmitOffboardingRespData, SubmitOffboardingResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/persons",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreatePersonRespData, CreatePersonResp>()
        .await
    }

    pub async fn delete(
        &self,
        person_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePersonResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeletePersonResp>()
        .await
    }

    pub async fn get(
        &self,
        person_id: &str,
        option: &RequestOption,
    ) -> Result<GetPersonResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetPersonRespData, GetPersonResp>()
        .await
    }

    pub async fn patch(
        &self,
        person_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPersonResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchPersonRespData, PatchPersonResp>()
        .await
    }

    pub async fn upload(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadPersonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/persons/upload",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<UploadPersonRespData, UploadPersonResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeletePreHireResp>()
        .await
    }

    pub async fn get(
        &self,
        pre_hire_id: &str,
        option: &RequestOption,
    ) -> Result<GetPreHireResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetPreHireRespData, GetPreHireResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/pre_hires",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListPreHireRespData, ListPreHireResp>()
        .await
    }

    pub async fn patch(
        &self,
        pre_hire_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPreHireResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<PatchPreHireRespData, PatchPreHireResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetProcessFormVariableDataRespData, GetProcessFormVariableDataResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/security_groups",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListSecurityGroupRespData, ListSecurityGroupResp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QuerySecurityGroupResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/security_groups/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<QuerySecurityGroupRespData, QuerySecurityGroupResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<SubdivisionData, GetSubdivisionResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/subdivisions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<SubdivisionListData, ListSubdivisionResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<SubregionData, GetSubregionResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/subregions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<SubregionListData, ListSubregionResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/transfer_reasons/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<TransferReasonListData, QueryTransferReasonResp>()
        .await
    }
}

pub struct TransferTypeResource<'a> {
    config: &'a Config,
}

impl<'a> TransferTypeResource<'a> {
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTransferTypeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/corehr/v1/transfer_types/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<TransferTypeListData, QueryTransferTypeResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/job_levels",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobLevelData, CreateJobLevelResp>()
        .await
    }

    pub async fn delete(
        &self,
        job_level_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobLevelResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteJobLevelResp>()
        .await
    }

    pub async fn patch(
        &self,
        job_level_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobLevelResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<JobLevelData, PatchJobLevelResp>()
        .await
    }
}

impl<'a> CurrencyResource<'a> {
    pub async fn get(
        &self,
        currency_id: &str,
        option: &RequestOption,
    ) -> Result<GetCurrencyResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/currencies/{currency_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<CorehrCurrencyData, GetCurrencyResp>()
        .await
    }
}

impl<'a> LocationResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateLocationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/locations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<LocationData, CreateLocationResp>()
        .await
    }

    pub async fn delete(
        &self,
        location_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLocationResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/locations/{location_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteLocationResp>()
        .await
    }
}

impl<'a> DepartmentResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/departments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateDepartmentRespData, CreateDepartmentResp>()
        .await
    }

    pub async fn delete(
        &self,
        department_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDepartmentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/departments/{department_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteDepartmentResp>()
        .await
    }

    pub async fn patch(
        &self,
        department_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchDepartmentResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/departments/{department_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<DepartmentData, PatchDepartmentResp>()
        .await
    }
}

impl<'a> WorkingHoursTypeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateWorkingHoursTypeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/corehr/v1/working_hours_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<WorkingHoursTypeData, CreateWorkingHoursTypeResp>()
        .await
    }

    pub async fn delete(
        &self,
        working_hours_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteWorkingHoursTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteWorkingHoursTypeResp>()
        .await
    }

    pub async fn get(
        &self,
        working_hours_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetWorkingHoursTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<WorkingHoursTypeData, GetWorkingHoursTypeResp>()
        .await
    }

    pub async fn patch(
        &self,
        working_hours_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchWorkingHoursTypeResp, LarkError> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<WorkingHoursTypeData, PatchWorkingHoursTypeResp>()
        .await
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
