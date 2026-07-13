use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyRespV2 as EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectoryUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserListData {
    #[serde(default)]
    pub items: Vec<DirectoryUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListUserResp, UserListData);

impl_resp_v2!(CreateCollaborationRuleResp, CreateCollaborationRuleRespData);
impl_resp_v2!(ListCollaborationRuleResp, ListCollaborationRuleRespData);
impl_resp_v2!(ListCollaborationTenantResp, ListCollaborationTenantRespData);
impl_resp_v2!(ListShareEntityResp, ());
impl_resp_v2!(CreateDepartmentResp, CreateDepartmentRespData);
impl_resp_v2!(FilterDepartmentResp, FilterDepartmentRespData);
impl_resp_v2!(MgetDepartmentResp, MgetDepartmentRespData);
impl_resp_v2!(SearchDepartmentResp, SearchDepartmentRespData);
impl_resp_v2!(CreateEmployeeResp, CreateEmployeeRespData);
impl_resp_v2!(FilterEmployeeResp, FilterEmployeeRespData);
impl_resp_v2!(MgetEmployeeResp, MgetEmployeeRespData);
impl_resp_v2!(SearchEmployeeResp, SearchEmployeeRespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCollaborationRuleRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollaborationRuleRespData {
    #[serde(default)]
    pub items: Vec<CollaborationRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollaborationTenantRespData {
    #[serde(default)]
    pub items: Vec<CollaborationTenant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDepartmentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterDepartmentRespData {
    #[serde(default)]
    pub departments: Vec<Department>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_response: Option<PageResponse>,
    #[serde(default)]
    pub abnormals: Vec<AbnormalRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MgetDepartmentRespData {
    #[serde(default)]
    pub departments: Vec<Department>,
    #[serde(default)]
    pub abnormals: Vec<AbnormalRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchDepartmentRespData {
    #[serde(default)]
    pub departments: Vec<Department>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_response: Option<PageResponse>,
    #[serde(default)]
    pub abnormals: Vec<AbnormalRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateEmployeeRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterEmployeeRespData {
    #[serde(default)]
    pub employees: Vec<EmployeeEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_response: Option<PageResponse>,
    #[serde(default)]
    pub abnormals: Vec<AbnormalRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MgetEmployeeRespData {
    #[serde(default)]
    pub employees: Vec<EmployeeEntity>,
    #[serde(default)]
    pub abnormals: Vec<AbnormalRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchEmployeeRespData {
    #[serde(default)]
    pub employees: Vec<EmployeeEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_response: Option<PageResponse>,
    #[serde(default)]
    pub abnormals: Vec<AbnormalRecord>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AbnormalRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_error: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<std::collections::HashMap<String, i32>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_bank_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_card_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_main: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_field_value: Option<std::collections::HashMap<String, CustomFieldValue>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<CollaborationRuleEntities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_is_valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<CollaborationRuleEntities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_is_valid: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationRuleEntities {
    #[serde(default)]
    pub open_user_ids: Vec<String>,
    #[serde(default)]
    pub open_department_ids: Vec<String>,
    #[serde(default)]
    pub open_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationTenant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_name: Option<I18nText>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Company {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_value: Option<UrlValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<EnumValue>,
    #[serde(default)]
    pub user_values: Vec<UserValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_value: Option<PhoneValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_count: Option<DepartmentCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_child: Option<bool>,
    #[serde(default)]
    pub leaders: Vec<DepartmentLeader>,
    #[serde(default)]
    pub hrbps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_weight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    #[serde(default)]
    pub custom_field_values: Vec<CustomFieldValue>,
    #[serde(default)]
    pub department_paths: Vec<String>,
    #[serde(default)]
    pub department_path_infos: Vec<DepartmentBaseInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_dimension: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentBaseInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<I18nText>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentCount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_members_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_members_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_members_count_exclude_leaders: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_departments_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_departments_count: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentLeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dependent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_list: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spouses_working_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_covered_by_health_insurance: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_allowed_for_tax_deduction: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Education {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_of_education: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school_enum: Option<WuKongEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study_enum: Option<WuKongEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmergencyContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeBaseEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub residential_address: Option<String>,
    #[serde(default)]
    pub contact_addresses: Vec<ContactAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Tenant>,
    #[serde(default)]
    pub departments: Vec<Department>,
    #[serde(default)]
    pub employee_order_in_departments: Vec<UserDepartmentSortInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_department_path: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_department_path_name: Option<std::collections::HashMap<String, I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<Image>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_image_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_set_custom_background_image: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
    #[serde(default)]
    pub dotted_line_leader_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default)]
    pub enterprise_email_aliases: Vec<String>,
    #[serde(default)]
    pub custom_field_values: Vec<CustomFieldValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_flag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_status: Option<WorkStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_been_resurrected: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub during_resign_cooling_off_period: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_set_custom_avatar: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_registered: Option<bool>,
    #[serde(default)]
    pub department_path_infos: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resign_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resigned_user_department_path: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resigned_user_department_path_name: Option<std::collections::HashMap<String, I18nText>>,
    #[serde(default)]
    pub resigned_departments: Vec<Department>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_name: Option<String>,
    #[serde(default)]
    pub subscription_ids: Vec<i64>,
    #[serde(default)]
    pub virtual_org_infos: Vec<UserVirtualOrgInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_forbidden_delete_employee: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeEducationAndWorkExperienceEntity {
    #[serde(default)]
    pub educations: Vec<Education>,
    #[serde(default)]
    pub work_experiences: Vec<WorkExperience>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub former_employer: Option<String>,
    #[serde(default)]
    pub personal_profiles: Vec<PersonalProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_of_id_portrait_side: Option<Attachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_of_id_emblem_side: Option<Attachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_photo: Option<Attachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diploma: Option<Attachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graduation_certificate: Option<Attachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_of_merit: Option<Attachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_paperwork: Option<Attachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_level_of_education_entity: Option<Education>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_degree_of_education_entity: Option<Education>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_info: Option<EmployeeBaseEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_info: Option<EmployeeWorkEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_and_social_relationship: Option<EmployeePersonalAndSocialRelationshipEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_and_work_experience: Option<EmployeeEducationAndWorkExperienceEntity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeePersonalAndSocialRelationshipEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_place: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub politics_status: Option<String>,
    #[serde(default)]
    pub bank_accounts: Vec<BankAccount>,
    #[serde(default)]
    pub certificates: Vec<Certificate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_residence_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_security_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provident_fund_number: Option<String>,
    #[serde(default)]
    pub emergency_contacts: Vec<EmergencyContact>,
    #[serde(default)]
    pub dependents: Vec<Dependent>,
    #[serde(default)]
    pub resident_taxs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub religion: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeWorkEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_country_or_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_place: Option<Place>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_station: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staff_status: Option<i32>,
    #[serde(default)]
    pub positions: Vec<Position>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<JobTitle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<JobLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family: Option<JobFamily>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_work_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_join_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regularization_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_lable: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convert_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_subject: Option<Company>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labor_contract_signing_times: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resign_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resign_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resign_remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resign_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_entry_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_entry_remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recruitment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seniority_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnumValue {
    #[serde(default)]
    pub enum_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<std::collections::HashMap<String, OptionValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Image {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImageLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFamily {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobTitle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default)]
    pub job_family_ids: Vec<String>,
    #[serde(default)]
    pub job_level_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Name {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_py: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub another_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<I18nText>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_value: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Place {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(default)]
    pub usages: Vec<String>,
    #[serde(default)]
    pub address_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_address_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Position {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_position_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_main_position: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tenant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_user_info: Option<UserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_modify_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_geo_full_domains: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dissolve_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_info: Option<TenantTraceInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_domain_be_modified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staff_size: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantTraceInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salesforce_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_email: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inviter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitation_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captcha_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reg_params: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UrlValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_text: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcurl: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDepartmentSortInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_weight_in_deparment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_weight_among_deparments: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<I18nText>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserValue {
    #[serde(default)]
    pub ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserVirtualOrgInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub departments: Vec<Department>,
    #[serde(default)]
    pub department_path_base_infos: Vec<DepartmentBaseInfo>,
    #[serde(default)]
    pub employee_order_in_departments: Vec<UserDepartmentSortInfo>,
    #[serde(default)]
    pub leaders: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkExperience {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WuKongEnum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
}
// ── Resources ──

pub struct UserResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListDirectoryUserQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListDirectoryUserQuery<'a> {
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

impl<'a> UserResource<'a> {
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListUserResp, LarkError> {
        let query = ListDirectoryUserQuery::new()
            .user_id_type(user_id_type)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDirectoryUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListUserResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/users",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send_response::<UserListData, ListUserResp>()
        .await
    }
}

// ── CollaborationRule resource ──

pub struct CollaborationRuleResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateCollaborationRuleQuery<'a> {
    pub body: &'a serde_json::Value,
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> CreateCollaborationRuleQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            target_tenant_key: None,
            tenant_id: None,
        }
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteCollaborationRuleQuery<'a> {
    pub collaboration_rule_id: &'a str,
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> DeleteCollaborationRuleQuery<'a> {
    pub fn new(collaboration_rule_id: &'a str) -> Self {
        Self {
            collaboration_rule_id,
            target_tenant_key: None,
            tenant_id: None,
        }
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListCollaborationRuleQuery<'a> {
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListCollaborationRuleQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateCollaborationRuleQuery<'a> {
    pub collaboration_rule_id: &'a str,
    pub body: &'a serde_json::Value,
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> UpdateCollaborationRuleQuery<'a> {
    pub fn new(collaboration_rule_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            collaboration_rule_id,
            body,
            target_tenant_key: None,
            tenant_id: None,
        }
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
        self
    }
}

impl CollaborationRuleResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCollaborationRuleResp, LarkError> {
        let query = CreateCollaborationRuleQuery::new(body)
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateCollaborationRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/collaboration_rules",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<CreateCollaborationRuleRespData, CreateCollaborationRuleResp>()
        .await
    }

    pub async fn delete(
        &self,
        collaboration_rule_id: &str,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteCollaborationRuleQuery::new(collaboration_rule_id)
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            query.collaboration_rule_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .send_v2_response::<serde_json::Value, EmptyResp>()
        .await
    }

    pub async fn list(
        &self,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCollaborationRuleResp, LarkError> {
        let query = ListCollaborationRuleQuery::new()
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCollaborationRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/collaboration_rules",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .page_query(query.page_query())
        .send_v2_response::<ListCollaborationRuleRespData, ListCollaborationRuleResp>()
        .await
    }

    pub async fn update(
        &self,
        collaboration_rule_id: &str,
        body: &serde_json::Value,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = UpdateCollaborationRuleQuery::new(collaboration_rule_id, body)
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            query.collaboration_rule_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<serde_json::Value, EmptyResp>()
        .await
    }
}

// ── CollaborationTenant resource ──

pub struct CollaborationTenantResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListCollaborationTenantQuery<'a> {
    pub tenant_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListCollaborationTenantQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
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

impl CollaborationTenantResource<'_> {
    pub async fn list(
        &self,
        tenant_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCollaborationTenantResp, LarkError> {
        let query = ListCollaborationTenantQuery::new()
            .tenant_id(tenant_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCollaborationTenantQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCollaborationTenantResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/collaboration_tenants",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("tenant_id", query.tenant_id)
        .page_query(query.page_query())
        .send_v2_response::<ListCollaborationTenantRespData, ListCollaborationTenantResp>()
        .await
    }
}

// ── ShareEntity resource ──

pub struct ShareEntityResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListShareEntityQuery<'a> {
    pub target_tenant_key: Option<&'a str>,
    pub target_department_id: Option<&'a str>,
    pub target_group_id: Option<&'a str>,
    pub is_select_subject: Option<bool>,
    pub tenant_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListShareEntityQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn target_department_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_department_id = value.into();
        self
    }

    pub fn target_group_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_group_id = value.into();
        self
    }

    pub fn is_select_subject(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_select_subject = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
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

impl ShareEntityResource<'_> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        target_tenant_key: Option<&str>,
        target_department_id: Option<&str>,
        target_group_id: Option<&str>,
        is_select_subject: Option<bool>,
        tenant_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListShareEntityResp, LarkError> {
        let query = ListShareEntityQuery::new()
            .target_tenant_key(target_tenant_key)
            .target_department_id(target_department_id)
            .target_group_id(target_group_id)
            .is_select_subject(is_select_subject)
            .tenant_id(tenant_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListShareEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListShareEntityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/share_entities",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("target_department_id", query.target_department_id)
        .query("target_group_id", query.target_group_id)
        .query("is_select_subject", query.is_select_subject)
        .query("tenant_id", query.tenant_id)
        .page_query(query.page_query())
        .send_v2_response::<(), ListShareEntityResp>()
        .await
    }
}

// ── Department resource ──

pub struct DepartmentResource<'a> {
    config: &'a Config,
}

macro_rules! impl_directory_role_setters {
    () => {
        pub fn employee_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
            self.employee_id_type = value.into();
            self
        }

        pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
            self.department_id_type = value.into();
            self
        }

        pub fn is_admin_role(mut self, value: impl Into<Option<bool>>) -> Self {
            self.is_admin_role = value.into();
            self
        }
    };
}

macro_rules! impl_directory_tenant_setter {
    () => {
        pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
            self.tenant_id = value.into();
            self
        }
    };
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> CreateDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub is_admin_role: Option<bool>,
    pub employee_id_type: Option<&'a str>,
}

impl<'a> DeleteDepartmentQuery<'a> {
    pub fn new(department_id: &'a str) -> Self {
        Self {
            department_id,
            is_admin_role: None,
            employee_id_type: None,
        }
    }

    pub fn is_admin_role(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_admin_role = value.into();
        self
    }

    pub fn employee_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.employee_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct FilterDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> FilterDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct MgetDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> MgetDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> PatchDepartmentQuery<'a> {
    pub fn new(department_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            department_id,
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchDirectoryDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> SearchDirectoryDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

impl DepartmentResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp, LarkError> {
        let query = CreateDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<CreateDepartmentRespData, CreateDepartmentResp>()
        .await
    }

    pub async fn delete(
        &self,
        department_id: &str,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteDepartmentQuery::new(department_id)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/departments/{}",
            query.department_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .send_v2_response::<serde_json::Value, EmptyResp>()
        .await
    }

    pub async fn filter(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<FilterDepartmentResp, LarkError> {
        let query = FilterDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.filter_by_query(&query, option).await
    }

    pub async fn filter_by_query(
        &self,
        query: &FilterDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<FilterDepartmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments/filter",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<FilterDepartmentRespData, FilterDepartmentResp>()
        .await
    }

    pub async fn mget(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        option: &RequestOption,
    ) -> Result<MgetDepartmentResp, LarkError> {
        let query = MgetDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role);
        self.mget_by_query(&query, option).await
    }

    pub async fn mget_by_query(
        &self,
        query: &MgetDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<MgetDepartmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments/mget",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .json_body(query.body)?
        .send_v2_response::<MgetDepartmentRespData, MgetDepartmentResp>()
        .await
    }

    pub async fn patch(
        &self,
        department_id: &str,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = PatchDepartmentQuery::new(department_id, body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/departments/{}",
            query.department_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .json_body(query.body)?
        .send_v2_response::<serde_json::Value, EmptyResp>()
        .await
    }

    pub async fn search(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchDepartmentResp, LarkError> {
        let query = SearchDirectoryDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchDirectoryDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchDepartmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<SearchDepartmentRespData, SearchDepartmentResp>()
        .await
    }
}

// ── Employee resource ──

pub struct EmployeeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateDirectoryEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> CreateDirectoryEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: Option<&'a serde_json::Value>,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> DeleteDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str) -> Self {
        Self {
            employee_id,
            body: None,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    pub fn body(mut self, value: impl Into<Option<&'a serde_json::Value>>) -> Self {
        self.body = value.into();
        self
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct FilterEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> FilterEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct MgetEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> MgetEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> PatchEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            employee_id,
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct RegularDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: Option<&'a serde_json::Value>,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> RegularDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str) -> Self {
        Self {
            employee_id,
            body: None,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    pub fn body(mut self, value: impl Into<Option<&'a serde_json::Value>>) -> Self {
        self.body = value.into();
        self
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ResurrectDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: Option<&'a serde_json::Value>,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> ResurrectDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str) -> Self {
        Self {
            employee_id,
            body: None,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    pub fn body(mut self, value: impl Into<Option<&'a serde_json::Value>>) -> Self {
        self.body = value.into();
        self
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchDirectoryEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> SearchDirectoryEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ToBeResignedDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> ToBeResignedDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            employee_id,
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

impl EmployeeResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateEmployeeResp, LarkError> {
        let query = CreateDirectoryEmployeeQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateEmployeeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<CreateEmployeeRespData, CreateEmployeeResp>()
        .await
    }

    pub async fn delete(
        &self,
        employee_id: &str,
        body: Option<&serde_json::Value>,
        employee_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteDirectoryEmployeeQuery::new(employee_id)
            .body(body)
            .employee_id_type(employee_id_type)
            .is_admin_role(is_admin_role)
            .department_id_type(department_id_type);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/directory/v1/employees/{}", query.employee_id);
        let request = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("department_id_type", query.department_id_type);
        let request = if let Some(body) = query.body {
            request.json_body(body)?
        } else {
            request
        };
        request
            .send_v2_response::<serde_json::Value, EmptyResp>()
            .await
    }

    pub async fn filter(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<FilterEmployeeResp, LarkError> {
        let query = FilterEmployeeQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.filter_by_query(&query, option).await
    }

    pub async fn filter_by_query(
        &self,
        query: &FilterEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<FilterEmployeeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees/filter",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<FilterEmployeeRespData, FilterEmployeeResp>()
        .await
    }

    pub async fn mget(
        &self,
        body: &serde_json::Value,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<MgetEmployeeResp, LarkError> {
        let query = MgetEmployeeQuery::new(body)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type);
        self.mget_by_query(&query, option).await
    }

    pub async fn mget_by_query(
        &self,
        query: &MgetEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<MgetEmployeeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees/mget",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<MgetEmployeeRespData, MgetEmployeeResp>()
        .await
    }

    pub async fn patch(
        &self,
        employee_id: &str,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = PatchEmployeeQuery::new(employee_id, body)
            .employee_id_type(employee_id_type)
            .is_admin_role(is_admin_role)
            .department_id_type(department_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/directory/v1/employees/{}", query.employee_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<serde_json::Value, EmptyResp>()
        .await
    }

    pub async fn regular(
        &self,
        employee_id: &str,
        body: Option<&serde_json::Value>,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = RegularDirectoryEmployeeQuery::new(employee_id)
            .body(body)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type);
        self.regular_by_query(&query, option).await
    }

    pub async fn regular_by_query(
        &self,
        query: &RegularDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/employees/{}/regular",
            query.employee_id
        );
        let request = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type);
        let request = if let Some(body) = query.body {
            request.json_body(body)?
        } else {
            request
        };
        request
            .send_v2_response::<serde_json::Value, EmptyResp>()
            .await
    }

    pub async fn resurrect(
        &self,
        employee_id: &str,
        body: Option<&serde_json::Value>,
        employee_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = ResurrectDirectoryEmployeeQuery::new(employee_id)
            .body(body)
            .employee_id_type(employee_id_type)
            .is_admin_role(is_admin_role)
            .department_id_type(department_id_type);
        self.resurrect_by_query(&query, option).await
    }

    pub async fn resurrect_by_query(
        &self,
        query: &ResurrectDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/employees/{}/resurrect",
            query.employee_id
        );
        let request = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("department_id_type", query.department_id_type);
        let request = if let Some(body) = query.body {
            request.json_body(body)?
        } else {
            request
        };
        request
            .send_v2_response::<serde_json::Value, EmptyResp>()
            .await
    }

    pub async fn search(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchEmployeeResp, LarkError> {
        let query = SearchDirectoryEmployeeQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchEmployeeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2_response::<SearchEmployeeRespData, SearchEmployeeResp>()
        .await
    }

    pub async fn to_be_resigned(
        &self,
        employee_id: &str,
        body: &serde_json::Value,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = ToBeResignedDirectoryEmployeeQuery::new(employee_id, body)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type);
        self.to_be_resigned_by_query(&query, option).await
    }

    pub async fn to_be_resigned_by_query(
        &self,
        query: &ToBeResignedDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/employees/{}/to_be_resigned",
            query.employee_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<serde_json::Value, EmptyResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub user: UserResource<'a>,
    pub collaboration_rule: CollaborationRuleResource<'a>,
    pub collaboration_tenant: CollaborationTenantResource<'a>,
    pub share_entity: ShareEntityResource<'a>,
    pub department: DepartmentResource<'a>,
    pub employee: EmployeeResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            user: UserResource { config },
            collaboration_rule: CollaborationRuleResource { config },
            collaboration_tenant: CollaborationTenantResource { config },
            share_entity: ShareEntityResource { config },
            department: DepartmentResource { config },
            employee: EmployeeResource { config },
        }
    }
}
