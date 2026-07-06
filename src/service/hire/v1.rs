use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{
    EmptyResp, PageIteratorState, PageQuery, RestRequest, impl_page_iterator_controls,
};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Job {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recruitment_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_salary: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_salary: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headcount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_light_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_function: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<IdNameObject>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdNameObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeNameObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommonSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_customized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children_list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegistrationSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenarios: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<CommonSchema>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFunction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobTypeInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Role {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_of_application: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_of_application: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_business_management_scope: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socail_permission_collection: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campus_permission_collection: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Website {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_type_list: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_channel_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommonAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<CodeNameObject>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebsiteJobPostCustomizedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<WebsiteJobPostCustomizedValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebsiteJobPostCustomizedValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<WebsiteJobPostCustomizedOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_list: Option<Vec<WebsiteJobPostCustomizedOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_range: Option<WebsiteJobPostCustomizedTimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebsiteJobPostCustomizedOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebsiteJobPostCustomizedTimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PortalJobPost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_expire_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_recruitment_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_job_level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_job_level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<CommonAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headcount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_light_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<WebsiteJobPostCustomizedData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_function: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<CommonAddress>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebsiteJobPost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_expire_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_recruitment_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_job_level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_job_level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<CommonAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headcount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_light_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<WebsiteJobPostCustomizedData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_function: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<CommonAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_sequence_info: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_major_list: Option<Vec<IdNameObject>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebsiteChannelInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<IdNameObject>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentFolderForList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminationReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referral_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_used_as_evaluation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TodoCommon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Todo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<TodoCommon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<TodoCommon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exam: Option<TodoCommon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview: Option<TodoCommon>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_management_scopes: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvaluationTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExamMarkingTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Interviewer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewFeedbackForm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_calculation_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRegistrationSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_used_as_interview: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_list: Option<Vec<CommonSchema>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRoundType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_assessment_template_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobProcess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_list: Option<Vec<JobProcessStage>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobProcessStage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobRequirementSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_list: Option<Vec<CommonSchema>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_list: Option<Vec<CommonSchema>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferApplyForm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferApplyFormInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<OfferApplyFormSchema>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferApplyFormSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_list: Option<Vec<OfferApplyFormModuleInfo>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferApplyFormModuleInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_customized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_list: Option<Vec<OfferApplyFormObjectInfo>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferApplyFormObjectInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_customized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_approve: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_sensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferApprovalTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_list: Option<Vec<Department>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Questionnaire {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub questionnaire_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_answers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Employee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_conversion_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_conversion_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overboard_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overboard_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_requirement_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_employment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Evaluation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MentionEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Note {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_mentioned_user: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_entity_list: Option<Vec<MentionEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEmployeeRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee: Option<Employee>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetByApplicationEmployeeRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee: Option<Employee>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEvaluationRespData {
    #[serde(default)]
    pub items: Vec<Evaluation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetNoteRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Note>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListNoteRespData {
    #[serde(default)]
    pub items: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInterviewRecordRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_record: Option<InterviewRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInterviewRecordRespData {
    #[serde(default)]
    pub items: Vec<InterviewRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRegistrationSchemaRespData {
    #[serde(default)]
    pub items: Vec<RegistrationSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListResumeSourceRespData {
    #[serde(default)]
    pub items: Vec<ResumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobFunctionRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<JobFunction>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobTypeRespData {
    #[serde(default)]
    pub items: Vec<JobTypeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobProcessRespData {
    #[serde(default)]
    pub items: Vec<JobProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListLocationRespData {
    #[serde(default)]
    pub items: Vec<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRoleRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleDetail>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRoleRespData {
    #[serde(default)]
    pub items: Vec<Role>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListWebsiteRespData {
    #[serde(default)]
    pub items: Vec<Website>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebsiteJobPostRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_post: Option<WebsiteJobPost>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListWebsiteJobPostRespData {
    #[serde(default)]
    pub items: Vec<WebsiteJobPost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchWebsiteJobPostRespData {
    #[serde(default)]
    pub items: Vec<WebsiteJobPost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReferralWebsiteJobPostRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_post: Option<PortalJobPost>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListReferralWebsiteJobPostRespData {
    #[serde(default)]
    pub items: Vec<PortalJobPost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPortalApplySchemaRespData {
    #[serde(default)]
    pub items: Vec<RegistrationSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListWebsiteChannelRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub website_channel_list: Vec<WebsiteChannelInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListQuestionnaireRespData {
    #[serde(default)]
    pub items: Vec<Questionnaire>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSubjectRespData {
    #[serde(default)]
    pub items: Vec<Subject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTalentFolderRespData {
    #[serde(default)]
    pub items: Vec<TalentFolderForList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTerminationReasonRespData {
    #[serde(default)]
    pub items: Vec<TerminationReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTodoRespData {
    #[serde(default)]
    pub items: Vec<Todo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUserRoleRespData {
    #[serde(default)]
    pub items: Vec<UserRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobRequirementSchemaRespData {
    #[serde(default)]
    pub items: Vec<JobRequirementSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobSchemaRespData {
    #[serde(default)]
    pub items: Vec<JobSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetOfferApplicationFormRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_apply_form: Option<OfferApplyFormInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListOfferApplicationFormRespData {
    #[serde(default)]
    pub items: Vec<OfferApplyForm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListOfferApprovalTemplateRespData {
    #[serde(default)]
    pub items: Vec<OfferApprovalTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTalentTagRespData {
    #[serde(default)]
    pub items: Vec<TalentTag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEvaluationTaskRespData {
    #[serde(default)]
    pub items: Vec<EvaluationTask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListExamMarkingTaskRespData {
    #[serde(default)]
    pub items: Vec<ExamMarkingTask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInterviewTaskRespData {
    #[serde(default)]
    pub items: Vec<InterviewTask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationInterviewRespData {
    #[serde(default)]
    pub items: Vec<Interview>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInterviewerRespData {
    #[serde(default)]
    pub items: Vec<Interviewer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInterviewFeedbackFormRespData {
    #[serde(default)]
    pub items: Vec<InterviewFeedbackForm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInterviewRegistrationSchemaRespData {
    #[serde(default)]
    pub items: Vec<InterviewRegistrationSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInterviewRoundTypeRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default)]
    pub items: Vec<InterviewRoundType>,
}

macro_rules! hire_catalog_page_query {
    ($name:ident) => {
        #[derive(Debug, Clone, Default)]
        #[non_exhaustive]
        pub struct $name<'a> {
            pub page_size: Option<i32>,
            pub page_token: Option<&'a str>,
        }

        impl<'a> $name<'a> {
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
    };
}

hire_catalog_page_query!(ListRegistrationSchemaQuery);
hire_catalog_page_query!(ListResumeSourceQuery);
hire_catalog_page_query!(ListJobFunctionQuery);
hire_catalog_page_query!(ListJobTypeQuery);
hire_catalog_page_query!(ListJobProcessQuery);
hire_catalog_page_query!(ListLocationQuery);
hire_catalog_page_query!(ListRoleQuery);
hire_catalog_page_query!(ListWebsiteQuery);
hire_catalog_page_query!(ListPortalApplySchemaQuery);
hire_catalog_page_query!(ListJobRequirementSchemaQuery);
hire_catalog_page_query!(ListOfferApplicationFormQuery);
hire_catalog_page_query!(ListTalentTagQuery);

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobSchemaQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub scenario: Option<i32>,
}

impl<'a> ListJobSchemaQuery<'a> {
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

    pub fn scenario(mut self, value: impl Into<Option<i32>>) -> Self {
        self.scenario = value.into();
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListQuestionnaireQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub application_id: Option<&'a str>,
    pub interview_id: Option<&'a str>,
    pub update_start_time: Option<&'a str>,
    pub update_end_time: Option<&'a str>,
}

impl<'a> ListQuestionnaireQuery<'a> {
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

    pub fn application_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.application_id = value.into();
        self
    }

    pub fn interview_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.interview_id = value.into();
        self
    }

    pub fn update_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_start_time = value.into();
        self
    }

    pub fn update_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_end_time = value.into();
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListOfferApprovalTemplateQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> ListOfferApprovalTemplateQuery<'a> {
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

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct GetByApplicationEmployeeQuery<'a> {
    pub application_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
    pub job_family_id_type: Option<&'a str>,
    pub employee_type_id_type: Option<&'a str>,
}

impl<'a> GetByApplicationEmployeeQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn application_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.application_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub fn job_family_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_family_id_type = value.into();
        self
    }

    pub fn employee_type_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.employee_type_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListEvaluationQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub application_id: Option<&'a str>,
    pub update_start_time: Option<&'a str>,
    pub update_end_time: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListEvaluationQuery<'a> {
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

    pub fn application_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.application_id = value.into();
        self
    }

    pub fn update_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_start_time = value.into();
        self
    }

    pub fn update_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_end_time = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListNoteQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub talent_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListNoteQuery<'a> {
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

    pub fn talent_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.talent_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListInterviewRecordQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub ids: Option<&'a [&'a str]>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListInterviewRecordQuery<'a> {
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

    pub fn ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.ids = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
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
#[non_exhaustive]
pub struct GetWebsiteJobPostQuery<'a> {
    pub website_id: &'a str,
    pub job_post_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
}

impl<'a> GetWebsiteJobPostQuery<'a> {
    pub fn new(website_id: &'a str, job_post_id: &'a str) -> Self {
        Self {
            website_id,
            job_post_id,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListWebsiteJobPostQuery<'a> {
    pub website_id: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
    pub update_start_time: Option<&'a str>,
    pub update_end_time: Option<&'a str>,
    pub create_start_time: Option<&'a str>,
    pub create_end_time: Option<&'a str>,
}

impl<'a> ListWebsiteJobPostQuery<'a> {
    pub fn new(website_id: &'a str) -> Self {
        Self {
            website_id,
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
            update_start_time: None,
            update_end_time: None,
            create_start_time: None,
            create_end_time: None,
        }
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub fn update_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_start_time = value.into();
        self
    }

    pub fn update_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_end_time = value.into();
        self
    }

    pub fn create_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.create_start_time = value.into();
        self
    }

    pub fn create_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.create_end_time = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SearchWebsiteJobPostQuery<'a> {
    pub website_id: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
}

impl<'a> SearchWebsiteJobPostQuery<'a> {
    pub fn new(website_id: &'a str) -> Self {
        Self {
            website_id,
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
        }
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetReferralWebsiteJobPostQuery<'a> {
    pub job_post_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
}

impl<'a> GetReferralWebsiteJobPostQuery<'a> {
    pub fn new(job_post_id: &'a str) -> Self {
        Self {
            job_post_id,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListReferralWebsiteJobPostQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub process_type: Option<i32>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
}

impl<'a> ListReferralWebsiteJobPostQuery<'a> {
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

    pub fn process_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.process_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListWebsiteChannelQuery<'a> {
    pub website_id: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListWebsiteChannelQuery<'a> {
    pub fn new(website_id: &'a str) -> Self {
        Self {
            website_id,
            page_size: None,
            page_token: None,
        }
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
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListSubjectQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub subject_ids: Option<&'a [&'a str]>,
}

impl<'a> ListSubjectQuery<'a> {
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn subject_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.subject_ids = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListTalentFolderQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListTalentFolderQuery<'a> {
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

hire_catalog_page_query!(ListTerminationReasonQuery);

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListTodoQuery<'a> {
    pub page_size: Option<&'a str>,
    pub page_token: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub type_: Option<&'a str>,
}

impl<'a> ListTodoQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn type_(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.type_ = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListUserRoleQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub role_id: Option<&'a str>,
    pub update_start_time: Option<&'a str>,
    pub update_end_time: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListUserRoleQuery<'a> {
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

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn role_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.role_id = value.into();
        self
    }

    pub fn update_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_start_time = value.into();
        self
    }

    pub fn update_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_end_time = value.into();
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

macro_rules! hire_catalog_iterator {
    ($iter:ident, $item:ty, $resource:ident, $query:ident) => {
        #[derive(Debug, Clone)]
        pub struct $iter<'a> {
            config: &'a Config,
            state: PageIteratorState<$item>,
            page_size: Option<i32>,
        }

        impl_page_iterator_controls!($iter);

        impl $iter<'_> {
            pub async fn next(
                &mut self,
                option: &RequestOption,
            ) -> Result<Option<$item>, LarkError> {
                if let Some(item) = self.state.pop() {
                    return Ok(Some(item));
                }
                if !self.state.should_fetch() {
                    return Ok(None);
                }

                let query = $query::new()
                    .page_size(self.page_size)
                    .page_token(self.state.page_token_for_request());
                let resource = $resource {
                    config: self.config,
                };
                let resp = resource.list_by_query(&query, option).await?;
                let data = resp.data.unwrap_or_default();
                self.state
                    .accept_page(Some(data.items), data.page_token, data.has_more);
                Ok(self.state.pop())
            }
        }
    };
}

hire_catalog_iterator!(
    ListRegistrationSchemaIterator,
    RegistrationSchema,
    RegistrationSchemaResource,
    ListRegistrationSchemaQuery
);
hire_catalog_iterator!(
    ListResumeSourceIterator,
    ResumeSource,
    ResumeSourceResource,
    ListResumeSourceQuery
);
hire_catalog_iterator!(
    ListJobFunctionIterator,
    JobFunction,
    JobFunctionResource,
    ListJobFunctionQuery
);
hire_catalog_iterator!(
    ListJobTypeIterator,
    JobTypeInfo,
    JobTypeResource,
    ListJobTypeQuery
);
hire_catalog_iterator!(
    ListLocationIterator,
    Location,
    LocationResource,
    ListLocationQuery
);
hire_catalog_iterator!(ListRoleIterator, Role, RoleResource, ListRoleQuery);
hire_catalog_iterator!(
    ListWebsiteIterator,
    Website,
    WebsiteResource,
    ListWebsiteQuery
);
hire_catalog_iterator!(
    ListPortalApplySchemaIterator,
    RegistrationSchema,
    PortalApplySchemaResource,
    ListPortalApplySchemaQuery
);
hire_catalog_iterator!(
    ListTalentTagIterator,
    TalentTag,
    TalentTagResource,
    ListTalentTagQuery
);

#[derive(Debug, Clone)]
pub struct ListEvaluationIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<Evaluation>,
    page_size: Option<i32>,
    application_id: Option<String>,
    update_start_time: Option<String>,
    update_end_time: Option<String>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(ListEvaluationIterator);

impl ListEvaluationIterator<'_> {
    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<Evaluation>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListEvaluationQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .application_id(self.application_id.as_deref())
            .update_start_time(self.update_start_time.as_deref())
            .update_end_time(self.update_end_time.as_deref())
            .user_id_type(self.user_id_type.as_deref());
        let resource = EvaluationResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListInterviewRecordIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<InterviewRecord>,
    page_size: Option<i32>,
    ids: Option<Vec<String>>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(ListInterviewRecordIterator);

impl ListInterviewRecordIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<InterviewRecord>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let ids = self
            .ids
            .as_ref()
            .map(|values| values.iter().map(String::as_str).collect::<Vec<_>>());
        let query = ListInterviewRecordQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .ids(ids.as_deref())
            .user_id_type(self.user_id_type.as_deref());
        let resource = InterviewRecordResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListWebsiteJobPostIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<WebsiteJobPost>,
    website_id: String,
    page_size: Option<i32>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    job_level_id_type: Option<String>,
    update_start_time: Option<String>,
    update_end_time: Option<String>,
    create_start_time: Option<String>,
    create_end_time: Option<String>,
}

impl_page_iterator_controls!(ListWebsiteJobPostIterator);

impl ListWebsiteJobPostIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<WebsiteJobPost>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListWebsiteJobPostQuery::new(&self.website_id)
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .user_id_type(self.user_id_type.as_deref())
            .department_id_type(self.department_id_type.as_deref())
            .job_level_id_type(self.job_level_id_type.as_deref())
            .update_start_time(self.update_start_time.as_deref())
            .update_end_time(self.update_end_time.as_deref())
            .create_start_time(self.create_start_time.as_deref())
            .create_end_time(self.create_end_time.as_deref());
        let resource = WebsiteJobPostResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct SearchWebsiteJobPostIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<WebsiteJobPost>,
    website_id: String,
    body: serde_json::Value,
    page_size: Option<i32>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    job_level_id_type: Option<String>,
}

impl_page_iterator_controls!(SearchWebsiteJobPostIterator);

impl SearchWebsiteJobPostIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<WebsiteJobPost>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = SearchWebsiteJobPostQuery::new(&self.website_id)
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .user_id_type(self.user_id_type.as_deref())
            .department_id_type(self.department_id_type.as_deref())
            .job_level_id_type(self.job_level_id_type.as_deref());
        let resource = WebsiteJobPostResource {
            config: self.config,
        };
        let resp = resource
            .search_by_query(&query, self.body.clone(), option)
            .await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListReferralWebsiteJobPostIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<PortalJobPost>,
    page_size: Option<i32>,
    process_type: Option<i32>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    job_level_id_type: Option<String>,
}

impl_page_iterator_controls!(ListReferralWebsiteJobPostIterator);

impl ListReferralWebsiteJobPostIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<PortalJobPost>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListReferralWebsiteJobPostQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .process_type(self.process_type)
            .user_id_type(self.user_id_type.as_deref())
            .department_id_type(self.department_id_type.as_deref())
            .job_level_id_type(self.job_level_id_type.as_deref());
        let resource = ReferralWebsiteJobPostResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListTalentFolderIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<TalentFolderForList>,
    page_size: Option<i32>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(ListTalentFolderIterator);

impl ListTalentFolderIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<TalentFolderForList>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListTalentFolderQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .user_id_type(self.user_id_type.as_deref());
        let resource = TalentFolderResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListTerminationReasonIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<TerminationReason>,
    page_size: Option<i32>,
}

impl_page_iterator_controls!(ListTerminationReasonIterator);

impl ListTerminationReasonIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<TerminationReason>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListTerminationReasonQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request());
        let resource = TerminationReasonResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListTodoIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<Todo>,
    page_size: Option<String>,
    user_id: Option<String>,
    user_id_type: Option<String>,
    type_: Option<String>,
}

impl_page_iterator_controls!(ListTodoIterator);

impl ListTodoIterator<'_> {
    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<Todo>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListTodoQuery::new()
            .page_size(self.page_size.as_deref())
            .page_token(self.state.page_token_for_request())
            .user_id(self.user_id.as_deref())
            .user_id_type(self.user_id_type.as_deref())
            .type_(self.type_.as_deref());
        let resource = TodoResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListUserRoleIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<UserRole>,
    page_size: Option<i32>,
    user_id: Option<String>,
    role_id: Option<String>,
    update_start_time: Option<String>,
    update_end_time: Option<String>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(ListUserRoleIterator);

impl ListUserRoleIterator<'_> {
    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<UserRole>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListUserRoleQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .user_id(self.user_id.as_deref())
            .role_id(self.role_id.as_deref())
            .update_start_time(self.update_start_time.as_deref())
            .update_end_time(self.update_end_time.as_deref())
            .user_id_type(self.user_id_type.as_deref());
        let resource = UserRoleResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

macro_rules! hire_task_iterator {
    ($iter:ident, $item:ty, $resource:ident, $query:ident) => {
        #[derive(Debug, Clone)]
        pub struct $iter<'a> {
            config: &'a Config,
            state: PageIteratorState<$item>,
            page_size: Option<i32>,
            user_id: Option<String>,
            activity_status: Option<i32>,
            user_id_type: Option<String>,
        }

        impl_page_iterator_controls!($iter);

        impl $iter<'_> {
            pub async fn next(
                &mut self,
                option: &RequestOption,
            ) -> Result<Option<$item>, LarkError> {
                if let Some(item) = self.state.pop() {
                    return Ok(Some(item));
                }
                if !self.state.should_fetch() {
                    return Ok(None);
                }

                let query = $query::new()
                    .page_size(self.page_size)
                    .page_token(self.state.page_token_for_request())
                    .user_id(self.user_id.as_deref())
                    .activity_status(self.activity_status)
                    .user_id_type(self.user_id_type.as_deref());
                let resource = $resource {
                    config: self.config,
                };
                let resp = resource.list_by_query(&query, option).await?;
                let data = resp.data.unwrap_or_default();
                self.state
                    .accept_page(Some(data.items), data.page_token, data.has_more);
                Ok(self.state.pop())
            }
        }
    };
}

hire_task_iterator!(
    ListEvaluationTaskIterator,
    EvaluationTask,
    EvaluationTaskResource,
    ListEvaluationTaskQuery
);
hire_task_iterator!(
    ListExamMarkingTaskIterator,
    ExamMarkingTask,
    ExamMarkingTaskResource,
    ListExamMarkingTaskQuery
);
hire_task_iterator!(
    ListInterviewTaskIterator,
    InterviewTask,
    InterviewTaskResource,
    ListInterviewTaskQuery
);

#[derive(Debug, Clone)]
pub struct ListInterviewFeedbackFormIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<InterviewFeedbackForm>,
    page_size: Option<i32>,
    interview_feedback_form_ids: Vec<String>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(ListInterviewFeedbackFormIterator);

impl ListInterviewFeedbackFormIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<InterviewFeedbackForm>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let ids: Vec<&str> = self
            .interview_feedback_form_ids
            .iter()
            .map(String::as_str)
            .collect();
        let query = ListInterviewFeedbackFormQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .interview_feedback_form_ids((!ids.is_empty()).then_some(ids.as_slice()))
            .user_id_type(self.user_id_type.as_deref());
        let resource = InterviewFeedbackFormResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListInterviewRegistrationSchemaIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<InterviewRegistrationSchema>,
    page_size: Option<i32>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(ListInterviewRegistrationSchemaIterator);

impl ListInterviewRegistrationSchemaIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<InterviewRegistrationSchema>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let query = ListInterviewRegistrationSchemaQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .user_id_type(self.user_id_type.as_deref());
        let resource = InterviewRegistrationSchemaResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct ListInterviewerIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<Interviewer>,
    page_size: Option<i32>,
    user_ids: Vec<String>,
    verify_status: Option<i32>,
    earliest_update_time: Option<String>,
    latest_update_time: Option<String>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(ListInterviewerIterator);

impl ListInterviewerIterator<'_> {
    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<Interviewer>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let user_ids: Vec<&str> = self.user_ids.iter().map(String::as_str).collect();
        let query = ListInterviewerQuery::new()
            .page_size(self.page_size)
            .page_token(self.state.page_token_for_request())
            .user_ids((!user_ids.is_empty()).then_some(user_ids.as_slice()))
            .verify_status(self.verify_status)
            .earliest_update_time(self.earliest_update_time.as_deref())
            .latest_update_time(self.latest_update_time.as_deref())
            .user_id_type(self.user_id_type.as_deref());
        let resource = InterviewerResource {
            config: self.config,
        };
        let resp = resource.list_by_query(&query, option).await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(Some(data.items), data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referral_record_list: Option<Vec<TalentReferralInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentReferralInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referral_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Talent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_in_agency_period: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_onboarded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<TalentBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<TalentEducation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub career_list: Option<Vec<TalentCareer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_list: Option<Vec<TalentProject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Vec<TalentWork>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_list: Option<Vec<TalentAward>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_list: Option<Vec<TalentLanguage>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sns_list: Option<Vec<TalentSns>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_source_list: Option<Vec<TalentResumeSource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_registration_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_attachment_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_degree: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentBasicInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience_years: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_city: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hometown_city: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_city_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_home_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentEducation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub academic_ranking: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentCareer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub career_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentWork {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentAward {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentLanguage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentSns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sns_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentResumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_source: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_website: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referral_record: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminate_reason: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminate_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Interview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub round: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_record_list: Option<Vec<InterviewRecord>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_submit_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_modify_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_submit_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_score: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessment_score: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_question_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interviewer: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_quality_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_assessment_list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Offer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<OfferBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salary_plan: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customize_info_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salary_currency: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_expire_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_city_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<IdNameObject>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferBasicInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_city: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_requirement: Option<IdNameObject>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobRequirement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_progress: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recruitment_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_function: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_salary: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_salary: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_approved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<IdNameObject>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecruitmentConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_apply_schema: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_process_conf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rec_process_info: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessment_template: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_list: Option<Vec<serde_json::Value>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateApplicationReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_record: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TerminateApplicationReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reason_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reason_note: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TransferStageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOfferReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<OfferBasicInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_plan: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customize_info_list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateOfferReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<OfferBasicInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_plan: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customize_info_list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct OfferStatusReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_reason_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_reason_notes: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTalentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent: Option<Talent>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobData {
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
pub struct TalentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent: Option<Talent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentListData {
    #[serde(default)]
    pub items: Vec<Talent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationListData {
    #[serde(default)]
    pub items: Vec<Application>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewListData {
    #[serde(default)]
    pub items: Vec<Interview>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<OfferBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salary_plan: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customize_info_list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetOfferData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<Offer>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferListData {
    #[serde(default)]
    pub items: Vec<Offer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobRequirementListData {
    #[serde(default)]
    pub items: Vec<JobRequirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferSchemaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_schema: Option<OfferSchema>,
}

impl_resp!(GetJobResp, JobData);
impl_resp!(ListJobResp, JobListData);
impl_resp!(GetTalentResp, TalentData);
impl_resp!(CreateTalentResp, TalentData);
impl_resp!(ListTalentResp, TalentListData);
impl_resp!(CreateApplicationResp, ApplicationData);
impl_resp!(GetApplicationResp, ApplicationData);
impl_resp!(ListApplicationResp, ApplicationListData);
impl_resp!(ListInterviewResp, InterviewListData);
impl_resp!(CreateOfferResp, OfferData);
impl_resp!(UpdateOfferResp, OfferData);
impl_resp!(GetOfferResp, GetOfferData);
impl_resp!(ListOfferResp, OfferListData);
impl_resp!(ListJobRequirementResp, JobRequirementListData);
impl_resp!(GetAttachmentResp, AttachmentData);
impl_resp!(GetOfferSchemaResp, OfferSchemaData);

// ── Resources ──

pub struct JobResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetJobQuery<'a> {
    pub job_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
    pub job_family_id_type: Option<&'a str>,
}

impl<'a> GetJobQuery<'a> {
    pub fn new(job_id: &'a str) -> Self {
        Self {
            job_id,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
            job_family_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub fn job_family_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_family_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub update_start_time: Option<&'a str>,
    pub update_end_time: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
    pub job_family_id_type: Option<&'a str>,
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

    pub fn update_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_start_time = value.into();
        self
    }

    pub fn update_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_end_time = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub fn job_family_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_family_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> JobResource<'a> {
    pub async fn get(
        &self,
        job_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetJobResp, LarkError> {
        let query = GetJobQuery::new(job_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetJobQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{}", query.job_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .query("job_family_id_type", query.job_family_id_type)
        .send_response::<JobData, GetJobResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        update_start_time: Option<&str>,
        update_end_time: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobResp, LarkError> {
        let query = ListJobQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .update_start_time(update_start_time)
            .update_end_time(update_end_time)
            .user_id_type(user_id_type);
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
            "/open-apis/hire/v1/jobs",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("update_start_time", query.update_start_time)
        .query("update_end_time", query.update_end_time)
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .query("job_family_id_type", query.job_family_id_type)
        .send_response::<JobListData, ListJobResp>()
        .await
    }

    pub async fn close(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CloseJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/close");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CloseJobResp>()
        .await
    }

    pub async fn combined_create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedCreateJobResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/jobs/combined_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CombinedCreateJobResp>()
        .await
    }

    pub async fn combined_update(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedUpdateJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/combined_update");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CombinedUpdateJobResp>()
        .await
    }

    pub async fn config(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<ConfigJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/config");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, ConfigJobResp>()
        .await
    }

    pub async fn get_detail(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<GetDetailJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/get_detail");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetDetailJobResp>()
        .await
    }

    pub async fn open(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<OpenJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/open");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, OpenJobResp>()
        .await
    }

    pub async fn recruiter(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<RecruiterJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/recruiter");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, RecruiterJobResp>()
        .await
    }

    pub async fn update_config(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateConfigJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/update_config");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateConfigJobResp>()
        .await
    }
}

pub struct TalentResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetTalentQuery<'a> {
    pub talent_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTalentQuery<'a> {
    pub fn new(talent_id: &'a str) -> Self {
        Self {
            talent_id,
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
pub struct ListTalentQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub keyword: Option<&'a str>,
    pub update_start_time: Option<&'a str>,
    pub update_end_time: Option<&'a str>,
    pub sort_by: Option<i32>,
    pub query_option: Option<&'a str>,
}

impl<'a> ListTalentQuery<'a> {
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn keyword(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.keyword = value.into();
        self
    }

    pub fn update_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_start_time = value.into();
        self
    }

    pub fn update_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_end_time = value.into();
        self
    }

    pub fn sort_by(mut self, value: impl Into<Option<i32>>) -> Self {
        self.sort_by = value.into();
        self
    }

    pub fn query_option(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.query_option = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> TalentResource<'a> {
    pub async fn create(
        &self,
        body: &CreateTalentReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTalentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<TalentData, CreateTalentResp>()
        .await
    }

    pub async fn get(
        &self,
        talent_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTalentResp, LarkError> {
        let query = GetTalentQuery::new(talent_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTalentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetTalentResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{}", query.talent_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<TalentData, GetTalentResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTalentResp, LarkError> {
        let query = ListTalentQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTalentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTalentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talents",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .query("keyword", query.keyword)
        .query("update_start_time", query.update_start_time)
        .query("update_end_time", query.update_end_time)
        .query("sort_by", query.sort_by)
        .query("query_option", query.query_option)
        .send_response::<TalentListData, ListTalentResp>()
        .await
    }

    pub async fn add_to_folder(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddToFolderTalentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/add_to_folder",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, AddToFolderTalentResp>()
        .await
    }

    pub async fn batch_get_id(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchGetIdTalentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/batch_get_id",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchGetIdTalentResp>()
        .await
    }

    pub async fn combined_create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedCreateTalentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/combined_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CombinedCreateTalentResp>()
        .await
    }

    pub async fn combined_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedUpdateTalentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/combined_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CombinedUpdateTalentResp>()
        .await
    }

    pub async fn onboard_status(
        &self,
        talent_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<OnboardStatusTalentResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/onboard_status");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, OnboardStatusTalentResp>()
        .await
    }

    pub async fn remove_to_folder(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveToFolderTalentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/remove_to_folder",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, RemoveToFolderTalentResp>()
        .await
    }

    pub async fn tag(
        &self,
        talent_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<TagTalentResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/tag");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, TagTalentResp>()
        .await
    }
}

pub struct ApplicationResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetApplicationQuery<'a> {
    pub application_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApplicationQuery<'a> {
    pub fn new(application_id: &'a str) -> Self {
        Self {
            application_id,
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
pub struct ListApplicationQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub job_id: Option<&'a str>,
    pub stage_id: Option<&'a str>,
    pub talent_id: Option<&'a str>,
    pub active_status: Option<i32>,
    pub user_id_type: Option<&'a str>,
    pub process_id: Option<&'a str>,
    pub update_start_time: Option<&'a str>,
    pub update_end_time: Option<&'a str>,
}

impl<'a> ListApplicationQuery<'a> {
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

    pub fn job_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_id = value.into();
        self
    }

    pub fn stage_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.stage_id = value.into();
        self
    }

    pub fn talent_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.talent_id = value.into();
        self
    }

    pub fn active_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.active_status = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn process_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.process_id = value.into();
        self
    }

    pub fn update_start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_start_time = value.into();
        self
    }

    pub fn update_end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_end_time = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> ApplicationResource<'a> {
    pub async fn create(
        &self,
        body: &CreateApplicationReqBody,
        option: &RequestOption,
    ) -> Result<CreateApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<ApplicationData, CreateApplicationResp>()
        .await
    }

    pub async fn get(
        &self,
        application_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationResp, LarkError> {
        let query = GetApplicationQuery::new(application_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{}", query.application_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<ApplicationData, GetApplicationResp>()
        .await
    }

    pub async fn terminate(
        &self,
        application_id: &str,
        body: &TerminateApplicationReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/terminate");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn transfer_stage(
        &self,
        application_id: &str,
        body: &TransferStageReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/transfer_stage");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        job_id: Option<&str>,
        stage_id: Option<&str>,
        talent_id: Option<&str>,
        active_status: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListApplicationResp, LarkError> {
        let query = ListApplicationQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .job_id(job_id)
            .stage_id(stage_id)
            .talent_id(talent_id)
            .active_status(active_status)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("job_id", query.job_id)
        .query("stage_id", query.stage_id)
        .query("talent_id", query.talent_id)
        .query("active_status", query.active_status)
        .query("user_id_type", query.user_id_type)
        .query("process_id", query.process_id)
        .query("update_start_time", query.update_start_time)
        .query("update_end_time", query.update_end_time)
        .send_response::<ApplicationListData, ListApplicationResp>()
        .await
    }

    pub async fn cancel_onboard(
        &self,
        application_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CancelOnboardApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/cancel_onboard");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CancelOnboardApplicationResp>()
        .await
    }

    pub async fn get_detail(
        &self,
        application_id: &str,
        option: &RequestOption,
    ) -> Result<GetDetailApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/get_detail");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetDetailApplicationResp>()
        .await
    }

    pub async fn recover(
        &self,
        application_id: &str,
        option: &RequestOption,
    ) -> Result<RecoverApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/recover");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, RecoverApplicationResp>()
        .await
    }

    pub async fn transfer_onboard(
        &self,
        application_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<TransferOnboardApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/transfer_onboard");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, TransferOnboardApplicationResp>()
        .await
    }

    pub async fn offer(
        &self,
        application_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<OfferApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/offer");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2_response::<serde_json::Value, OfferApplicationResp>()
        .await
    }
}

pub struct InterviewResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListInterviewQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub application_id: Option<&'a str>,
    pub interview_id: Option<&'a str>,
    pub start_time: Option<&'a str>,
    pub end_time: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
}

impl<'a> ListInterviewQuery<'a> {
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

    pub fn application_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.application_id = value.into();
        self
    }

    pub fn interview_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.interview_id = value.into();
        self
    }

    pub fn start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_time = value.into();
        self
    }

    pub fn end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_time = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> InterviewResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        application_id: Option<&str>,
        interview_id: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewResp, LarkError> {
        let query = ListInterviewQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .application_id(application_id)
            .interview_id(interview_id)
            .start_time(start_time)
            .end_time(end_time)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interviews",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("application_id", query.application_id)
        .query("interview_id", query.interview_id)
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("user_id_type", query.user_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .send_response::<InterviewListData, ListInterviewResp>()
        .await
    }

    pub async fn get_by_talent(
        &self,
        option: &RequestOption,
    ) -> Result<GetByTalentInterviewResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interviews/get_by_talent",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetByTalentInterviewResp>()
        .await
    }
}

pub struct OfferResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetOfferQuery<'a> {
    pub offer_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
    pub job_family_id_type: Option<&'a str>,
    pub employee_type_id_type: Option<&'a str>,
}

impl<'a> GetOfferQuery<'a> {
    pub fn new(offer_id: &'a str) -> Self {
        Self {
            offer_id,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
            job_family_id_type: None,
            employee_type_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub fn job_family_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_family_id_type = value.into();
        self
    }

    pub fn employee_type_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.employee_type_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListOfferQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub application_id: Option<&'a str>,
    pub talent_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub employee_type_id_type: Option<&'a str>,
}

impl<'a> ListOfferQuery<'a> {
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

    pub fn application_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.application_id = value.into();
        self
    }

    pub fn talent_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.talent_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn employee_type_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.employee_type_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> OfferResource<'a> {
    pub async fn create(
        &self,
        body: &CreateOfferReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateOfferResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/offers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<OfferData, CreateOfferResp>()
        .await
    }

    pub async fn update(
        &self,
        offer_id: &str,
        body: &UpdateOfferReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateOfferResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<OfferData, UpdateOfferResp>()
        .await
    }

    pub async fn get(
        &self,
        offer_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetOfferResp, LarkError> {
        let query = GetOfferQuery::new(offer_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetOfferQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetOfferResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offers/{}", query.offer_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .query("job_family_id_type", query.job_family_id_type)
        .query("employee_type_id_type", query.employee_type_id_type)
        .send_response::<GetOfferData, GetOfferResp>()
        .await
    }

    pub async fn offer_status(
        &self,
        offer_id: &str,
        body: &OfferStatusReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}/offer_status");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        application_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListOfferResp, LarkError> {
        let query = ListOfferQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .application_id(application_id)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListOfferQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOfferResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/offers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("application_id", query.application_id)
        .query("talent_id", query.talent_id)
        .query("user_id_type", query.user_id_type)
        .query("employee_type_id_type", query.employee_type_id_type)
        .send_response::<OfferListData, ListOfferResp>()
        .await
    }

    pub async fn intern_offer_status(
        &self,
        offer_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<InternOfferStatusResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}/intern_offer_status");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, InternOfferStatusResp>()
        .await
    }
}

pub struct JobRequirementResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListJobRequirementQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub job_id: Option<&'a str>,
    pub create_time_begin: Option<&'a str>,
    pub create_time_end: Option<&'a str>,
    pub update_time_begin: Option<&'a str>,
    pub update_time_end: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
    pub job_family_id_type: Option<&'a str>,
    pub employee_type_id_type: Option<&'a str>,
}

impl<'a> ListJobRequirementQuery<'a> {
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

    pub fn job_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_id = value.into();
        self
    }

    pub fn create_time_begin(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.create_time_begin = value.into();
        self
    }

    pub fn create_time_end(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.create_time_end = value.into();
        self
    }

    pub fn update_time_begin(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_time_begin = value.into();
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

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub fn job_family_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_family_id_type = value.into();
        self
    }

    pub fn employee_type_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.employee_type_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> JobRequirementResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateJobRequirementResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/job_requirements",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .query("department_id_type", department_id_type)
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateJobRequirementResp>()
        .await
    }

    pub async fn delete(
        &self,
        job_requirement_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobRequirementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/job_requirements/{job_requirement_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteJobRequirementResp>()
        .await
    }

    pub async fn update(
        &self,
        job_requirement_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateJobRequirementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/job_requirements/{job_requirement_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .query("department_id_type", department_id_type)
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateJobRequirementResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        job_id: Option<&str>,
        create_time_begin: Option<&str>,
        create_time_end: Option<&str>,
        update_time_begin: Option<&str>,
        update_time_end: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobRequirementResp, LarkError> {
        let query = ListJobRequirementQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .job_id(job_id)
            .create_time_begin(create_time_begin)
            .create_time_end(create_time_end)
            .update_time_begin(update_time_begin)
            .update_time_end(update_time_end)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobRequirementQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobRequirementResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_requirements",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("job_id", query.job_id)
        .query("create_time_begin", query.create_time_begin)
        .query("create_time_end", query.create_time_end)
        .query("update_time_begin", query.update_time_begin)
        .query("update_time_end", query.update_time_end)
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .query("job_family_id_type", query.job_family_id_type)
        .query("employee_type_id_type", query.employee_type_id_type)
        .send_response::<JobRequirementListData, ListJobRequirementResp>()
        .await
    }

    pub async fn list_by_id(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ListByIdJobRequirementResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/job_requirements/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, ListByIdJobRequirementResp>()
        .await
    }
}

pub struct AttachmentResource<'a> {
    config: &'a Config,
}

impl<'a> AttachmentResource<'a> {
    pub async fn get(
        &self,
        attachment_id: &str,
        option: &RequestOption,
    ) -> Result<GetAttachmentResp, LarkError> {
        let path = format!("/open-apis/hire/v1/attachments/{attachment_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<AttachmentData, GetAttachmentResp>()
        .await
    }

    pub async fn preview(
        &self,
        attachment_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/attachments/{attachment_id}/preview");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateAttachmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/attachments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateAttachmentResp>()
        .await
    }
}

pub struct OfferSchemaResource<'a> {
    config: &'a Config,
}

impl<'a> OfferSchemaResource<'a> {
    pub async fn get(
        &self,
        offer_schema_id: &str,
        option: &RequestOption,
    ) -> Result<GetOfferSchemaResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offer_schemas/{offer_schema_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<OfferSchemaData, GetOfferSchemaResp>()
        .await
    }
}

// ── Helpers for newer resources (use Option<CodeError> pattern) ──

impl_resp_v2!(GetEmployeeResp, GetEmployeeRespData);
impl_resp_v2!(
    GetByApplicationEmployeeResp,
    GetByApplicationEmployeeRespData
);
impl_resp_v2!(PatchEmployeeResp, serde_json::Value);
impl_resp_v2!(ListEvaluationResp, ListEvaluationRespData);
impl_resp_v2!(ListNoteResp, ListNoteRespData);
impl_resp_v2!(CreateNoteResp, serde_json::Value);
impl_resp_v2!(GetNoteResp, GetNoteRespData);
impl_resp_v2!(PatchNoteResp, serde_json::Value);
impl_resp_v2!(DeleteNoteResp, ());
impl_resp_v2!(ListQuestionnaireResp, ListQuestionnaireRespData);
impl_resp_v2!(GetReferralResp, serde_json::Value);
impl_resp_v2!(ListRegistrationSchemaResp, ListRegistrationSchemaRespData);
impl_resp_v2!(ListResumeSourceResp, ListResumeSourceRespData);
impl_resp_v2!(ListJobFunctionResp, ListJobFunctionRespData);
impl_resp_v2!(ListJobTypeResp, ListJobTypeRespData);
impl_resp_v2!(ListJobProcessResp, ListJobProcessRespData);
impl_resp_v2!(ListLocationResp, ListLocationRespData);
impl_resp_v2!(ListRoleResp, ListRoleRespData);
impl_resp_v2!(GetRoleResp, GetRoleRespData);
impl_resp_v2!(ListSubjectResp, ListSubjectRespData);
impl_resp_v2!(ListTalentFolderResp, ListTalentFolderRespData);
impl_resp_v2!(ListTerminationReasonResp, ListTerminationReasonRespData);
impl_resp_v2!(ListUserRoleResp, ListUserRoleRespData);
impl_resp_v2!(ListWebsiteResp, ListWebsiteRespData);
impl_resp_v2!(ListWebsiteJobPostResp, ListWebsiteJobPostRespData);
impl_resp_v2!(GetWebsiteJobPostResp, GetWebsiteJobPostRespData);
impl_resp_v2!(ListInterviewRecordResp, ListInterviewRecordRespData);
impl_resp_v2!(GetInterviewRecordResp, GetInterviewRecordRespData);
impl_resp_v2!(ListInterviewerResp, ListInterviewerRespData);
impl_resp_v2!(PatchInterviewerResp, serde_json::Value);
impl_resp_v2!(CreateExternalApplicationResp, serde_json::Value);
impl_resp_v2!(UpdateExternalApplicationResp, serde_json::Value);
impl_resp_v2!(DeleteExternalApplicationResp, ());
impl_resp_v2!(ListExternalApplicationResp, serde_json::Value);
impl_resp_v2!(CreateExternalOfferResp, serde_json::Value);
impl_resp_v2!(UpdateExternalOfferResp, serde_json::Value);
impl_resp_v2!(DeleteExternalOfferResp, ());
impl_resp_v2!(CreateExternalInterviewResp, serde_json::Value);
impl_resp_v2!(UpdateExternalInterviewResp, serde_json::Value);
impl_resp_v2!(DeleteExternalInterviewResp, ());
impl_resp_v2!(CreateExternalBackgroundCheckResp, serde_json::Value);
impl_resp_v2!(UpdateExternalBackgroundCheckResp, serde_json::Value);
impl_resp_v2!(DeleteExternalBackgroundCheckResp, ());
impl_resp_v2!(ListTodoResp, ListTodoRespData);
impl_resp_v2!(CreateTripartiteAgreementResp, serde_json::Value);
impl_resp_v2!(UpdateTripartiteAgreementResp, serde_json::Value);
impl_resp_v2!(DeleteTripartiteAgreementResp, ());
impl_resp_v2!(ListTripartiteAgreementResp, serde_json::Value);

// ── New response types for missing methods ──

impl_resp_v2!(PublishAdvertisementResp, serde_json::Value);
impl_resp_v2!(BatchQueryAgencyResp, serde_json::Value);
impl_resp_v2!(GetAgencyAccountResp, serde_json::Value);
impl_resp_v2!(OperateAgencyAccountResp, serde_json::Value);
impl_resp_v2!(ProtectAgencyResp, serde_json::Value);
impl_resp_v2!(ProtectSearchAgencyResp, serde_json::Value);
impl_resp_v2!(QueryAgencyResp, serde_json::Value);
impl_resp_v2!(CancelOnboardApplicationResp, serde_json::Value);
impl_resp_v2!(GetDetailApplicationResp, serde_json::Value);
impl_resp_v2!(RecoverApplicationResp, serde_json::Value);
impl_resp_v2!(TransferOnboardApplicationResp, serde_json::Value);
impl_resp_v2!(BatchQueryBackgroundCheckOrderResp, serde_json::Value);
impl_resp_v2!(ListBackgroundCheckOrderResp, serde_json::Value);
impl_resp_v2!(SearchDiversityInclusionResp, serde_json::Value);
impl_resp_v2!(BatchDeleteEcoAccountCustomFieldResp, serde_json::Value);
impl_resp_v2!(BatchUpdateEcoAccountCustomFieldResp, serde_json::Value);
impl_resp_v2!(CancelEcoBackgroundCheckResp, serde_json::Value);
impl_resp_v2!(UpdateProgressEcoBackgroundCheckResp, serde_json::Value);
impl_resp_v2!(UpdateResultEcoBackgroundCheckResp, serde_json::Value);
impl_resp_v2!(
    BatchDeleteEcoBackgroundCheckCustomFieldResp,
    serde_json::Value
);
impl_resp_v2!(
    BatchUpdateEcoBackgroundCheckCustomFieldResp,
    serde_json::Value
);
impl_resp_v2!(BatchDeleteEcoBackgroundCheckPackageResp, serde_json::Value);
impl_resp_v2!(BatchUpdateEcoBackgroundCheckPackageResp, serde_json::Value);
impl_resp_v2!(LoginInfoEcoExamResp, serde_json::Value);
impl_resp_v2!(UpdateResultEcoExamResp, serde_json::Value);
impl_resp_v2!(BatchDeleteEcoExamPaperResp, serde_json::Value);
impl_resp_v2!(BatchUpdateEcoExamPaperResp, serde_json::Value);
impl_resp_v2!(BatchQueryExternalBackgroundCheckResp, serde_json::Value);
impl_resp_v2!(BatchQueryExternalInterviewResp, serde_json::Value);
impl_resp_v2!(BatchQueryExternalOfferResp, serde_json::Value);
impl_resp_v2!(GetByTalentInterviewResp, serde_json::Value);
impl_resp_v2!(CloseJobResp, serde_json::Value);
impl_resp_v2!(CombinedCreateJobResp, serde_json::Value);
impl_resp_v2!(CombinedUpdateJobResp, serde_json::Value);
impl_resp_v2!(ConfigJobResp, serde_json::Value);
impl_resp_v2!(GetDetailJobResp, serde_json::Value);
impl_resp_v2!(OpenJobResp, serde_json::Value);
impl_resp_v2!(RecruiterJobResp, serde_json::Value);
impl_resp_v2!(UpdateConfigJobResp, serde_json::Value);
impl_resp_v2!(BatchUpdateJobManagerResp, serde_json::Value);
impl_resp_v2!(SearchJobPublishRecordResp, serde_json::Value);
impl_resp_v2!(ListByIdJobRequirementResp, serde_json::Value);
impl_resp_v2!(QueryLocationResp, serde_json::Value);
impl_resp_v2!(InternOfferStatusResp, serde_json::Value);
impl_resp_v2!(SearchReferralResp, serde_json::Value);
impl_resp_v2!(DeactivateReferralAccountResp, serde_json::Value);
impl_resp_v2!(EnableReferralAccountResp, serde_json::Value);
impl_resp_v2!(GetAccountAssetsReferralAccountResp, serde_json::Value);
impl_resp_v2!(ReconciliationReferralAccountResp, serde_json::Value);
impl_resp_v2!(WithdrawReferralAccountResp, serde_json::Value);
impl_resp_v2!(AddToFolderTalentResp, serde_json::Value);
impl_resp_v2!(BatchGetIdTalentResp, serde_json::Value);
impl_resp_v2!(CombinedCreateTalentResp, serde_json::Value);
impl_resp_v2!(CombinedUpdateTalentResp, serde_json::Value);
impl_resp_v2!(OnboardStatusTalentResp, serde_json::Value);
impl_resp_v2!(RemoveToFolderTalentResp, serde_json::Value);
impl_resp_v2!(TagTalentResp, serde_json::Value);
impl_resp_v2!(ChangeTalentBlockResp, serde_json::Value);
impl_resp_v2!(QueryTalentObjectResp, serde_json::Value);
impl_resp_v2!(SearchTalentOperationLogResp, serde_json::Value);
impl_resp_v2!(BatchChangeTalentPoolResp, serde_json::Value);
impl_resp_v2!(MoveTalentTalentPoolResp, serde_json::Value);
impl_resp_v2!(SearchTalentPoolResp, serde_json::Value);
impl_resp_v2!(SearchTestResp, serde_json::Value);
impl_resp_v2!(CreateByAttachmentWebsiteDeliveryResp, serde_json::Value);
impl_resp_v2!(CreateByResumeWebsiteDeliveryResp, serde_json::Value);
impl_resp_v2!(SearchWebsiteJobPostResp, SearchWebsiteJobPostRespData);

// ── Additional response types ──

impl_resp_v2!(OfferApplicationResp, serde_json::Value);
impl_resp_v2!(
    ListApplicationInterviewResp2,
    ListApplicationInterviewRespData
);
impl_resp_v2!(PatchEhrImportTaskResp, serde_json::Value);
impl_resp_v2!(ListEvaluationTaskResp, ListEvaluationTaskRespData);
impl_resp_v2!(CreateExamResp, serde_json::Value);
impl_resp_v2!(ListExamMarkingTaskResp, ListExamMarkingTaskRespData);
impl_resp_v2!(CreateExternalInterviewAssessmentResp, serde_json::Value);
impl_resp_v2!(PatchExternalInterviewAssessmentResp, serde_json::Value);
impl_resp_v2!(CreateExternalReferralRewardResp, serde_json::Value);
impl_resp_v2!(DeleteExternalReferralRewardResp, ());
impl_resp_v2!(
    ListInterviewFeedbackFormResp,
    ListInterviewFeedbackFormRespData
);
impl_resp_v2!(GetInterviewRecordAttachmentResp, serde_json::Value);
impl_resp_v2!(
    ListInterviewRegistrationSchemaResp,
    ListInterviewRegistrationSchemaRespData
);
impl_resp_v2!(ListInterviewRoundTypeResp, ListInterviewRoundTypeRespData);
impl_resp_v2!(ListInterviewTaskResp, ListInterviewTaskRespData);
impl_resp_v2!(GetJobManagerResp, serde_json::Value);
impl_resp_v2!(ListJobRequirementSchemaResp, ListJobRequirementSchemaRespData);
impl_resp_v2!(ListJobSchemaResp, ListJobSchemaRespData);
impl_resp_v2!(GetMinutesResp, serde_json::Value);
impl_resp_v2!(GetOfferApplicationFormResp, GetOfferApplicationFormRespData);
impl_resp_v2!(ListOfferApplicationFormResp, ListOfferApplicationFormRespData);
impl_resp_v2!(ListOfferApprovalTemplateResp, ListOfferApprovalTemplateRespData);
impl_resp_v2!(UpdateOfferCustomFieldResp, serde_json::Value);
impl_resp_v2!(ListPortalApplySchemaResp, ListPortalApplySchemaRespData);
impl_resp_v2!(
    GetReferralWebsiteJobPostResp,
    GetReferralWebsiteJobPostRespData
);
impl_resp_v2!(
    ListReferralWebsiteJobPostResp,
    ListReferralWebsiteJobPostRespData
);
impl_resp_v2!(CreateTalentExternalInfoResp, serde_json::Value);
impl_resp_v2!(UpdateTalentExternalInfoResp, serde_json::Value);
impl_resp_v2!(ListTalentTagResp, ListTalentTagRespData);
impl_resp_v2!(CreateWebsiteChannelResp, serde_json::Value);
impl_resp_v2!(DeleteWebsiteChannelResp, ());
impl_resp_v2!(ListWebsiteChannelResp, ListWebsiteChannelRespData);
impl_resp_v2!(UpdateWebsiteChannelResp, serde_json::Value);
impl_resp_v2!(GetWebsiteDeliveryTaskResp, serde_json::Value);
impl_resp_v2!(CreateWebsiteSiteUserResp, serde_json::Value);

// ── New response types (Phase 11 — missing methods) ──

impl_resp_v2!(GetAgencyResp, serde_json::Value);
impl_resp_v2!(CreateAttachmentResp, serde_json::Value);
impl_resp_v2!(CreateEcoAccountCustomFieldResp, serde_json::Value);
impl_resp_v2!(CreateEcoBackgroundCheckCustomFieldResp, serde_json::Value);
impl_resp_v2!(CreateEcoBackgroundCheckPackageResp, serde_json::Value);
impl_resp_v2!(CreateEcoExamPaperResp, serde_json::Value);
impl_resp_v2!(CreateJobRequirementResp, serde_json::Value);
impl_resp_v2!(DeleteJobRequirementResp, ());
impl_resp_v2!(UpdateJobRequirementResp, serde_json::Value);
impl_resp_v2!(CreateReferralAccountResp, serde_json::Value);

// ── Employee resource ──

pub struct EmployeeResource<'a> {
    config: &'a Config,
}

impl EmployeeResource<'_> {
    pub async fn get(
        &self,
        employee_id: &str,
        option: &RequestOption,
    ) -> Result<GetEmployeeResp, LarkError> {
        let path = format!("/open-apis/hire/v1/employees/{employee_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetEmployeeRespData, GetEmployeeResp>()
        .await
    }

    pub async fn patch(
        &self,
        employee_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmployeeResp, LarkError> {
        let path = format!("/open-apis/hire/v1/employees/{employee_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchEmployeeResp>()
        .await
    }

    pub async fn get_by_application(
        &self,
        option: &RequestOption,
    ) -> Result<GetByApplicationEmployeeResp, LarkError> {
        self.get_by_application_query(&GetByApplicationEmployeeQuery::new(), option)
            .await
    }

    pub async fn get_by_application_query(
        &self,
        query: &GetByApplicationEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetByApplicationEmployeeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/employees/get_by_application",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("application_id", query.application_id)
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .query("job_family_id_type", query.job_family_id_type)
        .query("employee_type_id_type", query.employee_type_id_type)
        .send_v2_response::<GetByApplicationEmployeeRespData, GetByApplicationEmployeeResp>()
        .await
    }
}

// ── Evaluation resource ──

pub struct EvaluationResource<'a> {
    config: &'a Config,
}

impl EvaluationResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListEvaluationResp, LarkError> {
        self.list_by_query(&ListEvaluationQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEvaluationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEvaluationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/evaluations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("application_id", query.application_id)
        .query("update_start_time", query.update_start_time)
        .query("update_end_time", query.update_end_time)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListEvaluationRespData, ListEvaluationResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListEvaluationIterator<'_> {
        let query = ListEvaluationQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListEvaluationQuery<'_>,
    ) -> ListEvaluationIterator<'_> {
        ListEvaluationIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            application_id: query.application_id.map(ToOwned::to_owned),
            update_start_time: query.update_start_time.map(ToOwned::to_owned),
            update_end_time: query.update_end_time.map(ToOwned::to_owned),
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── Note resource ──

pub struct NoteResource<'a> {
    config: &'a Config,
}

impl NoteResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateNoteResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/notes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateNoteResp>()
        .await
    }

    pub async fn delete(
        &self,
        note_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteNoteResp, LarkError> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteNoteResp>()
        .await
    }

    pub async fn get(
        &self,
        note_id: &str,
        option: &RequestOption,
    ) -> Result<GetNoteResp, LarkError> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetNoteRespData, GetNoteResp>()
        .await
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListNoteResp, LarkError> {
        self.list_by_query(&ListNoteQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListNoteQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListNoteResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/notes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("talent_id", query.talent_id)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListNoteRespData, ListNoteResp>()
        .await
    }

    pub async fn patch(
        &self,
        note_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchNoteResp, LarkError> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchNoteResp>()
        .await
    }
}

// ── Questionnaire resource ──

pub struct QuestionnaireResource<'a> {
    config: &'a Config,
}

impl QuestionnaireResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListQuestionnaireResp, LarkError> {
        self.list_by_query(&ListQuestionnaireQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListQuestionnaireQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListQuestionnaireResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/questionnaires",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("application_id", query.application_id)
        .query("interview_id", query.interview_id)
        .query("update_start_time", query.update_start_time)
        .query("update_end_time", query.update_end_time)
        .send_v2_response::<ListQuestionnaireRespData, ListQuestionnaireResp>()
        .await
    }
}

// ── Referral resource ──

pub struct ReferralResource<'a> {
    config: &'a Config,
}

impl ReferralResource<'_> {
    pub async fn get_by_application(
        &self,
        option: &RequestOption,
    ) -> Result<GetReferralResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/referrals/get_by_application",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetReferralResp>()
        .await
    }

    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchReferralResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referrals/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, SearchReferralResp>()
        .await
    }
}

pub struct RegistrationSchemaResource<'a> {
    config: &'a Config,
}

impl RegistrationSchemaResource<'_> {
    pub async fn list(
        &self,
        option: &RequestOption,
    ) -> Result<ListRegistrationSchemaResp, LarkError> {
        self.list_by_query(&ListRegistrationSchemaQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListRegistrationSchemaQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListRegistrationSchemaResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/registration_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListRegistrationSchemaRespData, ListRegistrationSchemaResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListRegistrationSchemaIterator<'_> {
        let query = ListRegistrationSchemaQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListRegistrationSchemaQuery<'_>,
    ) -> ListRegistrationSchemaIterator<'_> {
        ListRegistrationSchemaIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

// ── ResumeSource resource ──

pub struct ResumeSourceResource<'a> {
    config: &'a Config,
}

impl ResumeSourceResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListResumeSourceResp, LarkError> {
        self.list_by_query(&ListResumeSourceQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListResumeSourceQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListResumeSourceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/resume_sources",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListResumeSourceRespData, ListResumeSourceResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListResumeSourceIterator<'_> {
        let query = ListResumeSourceQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListResumeSourceQuery<'_>,
    ) -> ListResumeSourceIterator<'_> {
        ListResumeSourceIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

pub struct JobFunctionResource<'a> {
    config: &'a Config,
}

impl JobFunctionResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListJobFunctionResp, LarkError> {
        self.list_by_query(&ListJobFunctionQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobFunctionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobFunctionResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_functions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListJobFunctionRespData, ListJobFunctionResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListJobFunctionIterator<'_> {
        let query = ListJobFunctionQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListJobFunctionQuery<'_>,
    ) -> ListJobFunctionIterator<'_> {
        ListJobFunctionIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

pub struct JobTypeResource<'a> {
    config: &'a Config,
}

impl JobTypeResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListJobTypeResp, LarkError> {
        self.list_by_query(&ListJobTypeQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobTypeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListJobTypeRespData, ListJobTypeResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListJobTypeIterator<'_> {
        let query = ListJobTypeQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(&self, query: &ListJobTypeQuery<'_>) -> ListJobTypeIterator<'_> {
        ListJobTypeIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

pub struct JobProcessResource<'a> {
    config: &'a Config,
}

impl JobProcessResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListJobProcessResp, LarkError> {
        self.list_by_query(&ListJobProcessQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobProcessQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobProcessResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_processes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListJobProcessRespData, ListJobProcessResp>()
        .await
    }
}
pub struct LocationResource<'a> {
    config: &'a Config,
}

impl LocationResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListLocationResp, LarkError> {
        self.list_by_query(&ListLocationQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListLocationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListLocationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/locations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListLocationRespData, ListLocationResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListLocationIterator<'_> {
        let query = ListLocationQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListLocationQuery<'_>,
    ) -> ListLocationIterator<'_> {
        ListLocationIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryLocationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/locations/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QueryLocationResp>()
        .await
    }
}
pub struct RoleResource<'a> {
    config: &'a Config,
}

impl RoleResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListRoleResp, LarkError> {
        self.list_by_query(&ListRoleQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListRoleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListRoleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/roles",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListRoleRespData, ListRoleResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListRoleIterator<'_> {
        let query = ListRoleQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(&self, query: &ListRoleQuery<'_>) -> ListRoleIterator<'_> {
        ListRoleIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }

    pub async fn get(
        &self,
        role_id: &str,
        option: &RequestOption,
    ) -> Result<GetRoleResp, LarkError> {
        let path = format!("/open-apis/hire/v1/roles/{role_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetRoleRespData, GetRoleResp>()
        .await
    }
}
pub struct SubjectResource<'a> {
    config: &'a Config,
}

impl SubjectResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListSubjectResp, LarkError> {
        self.list_by_query(&ListSubjectQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSubjectQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListSubjectResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/subjects",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .query_values(
            "subject_ids",
            query.subject_ids.map(|ids| ids.iter().copied()),
        )
        .send_v2_response::<ListSubjectRespData, ListSubjectResp>()
        .await
    }
}

pub struct TalentFolderResource<'a> {
    config: &'a Config,
}

impl TalentFolderResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListTalentFolderResp, LarkError> {
        self.list_by_query(&ListTalentFolderQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTalentFolderQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTalentFolderResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talent_folders",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListTalentFolderRespData, ListTalentFolderResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
    ) -> ListTalentFolderIterator<'_> {
        let query = ListTalentFolderQuery::new()
            .page_size(page_size)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListTalentFolderQuery<'_>,
    ) -> ListTalentFolderIterator<'_> {
        ListTalentFolderIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

pub struct TerminationReasonResource<'a> {
    config: &'a Config,
}

impl TerminationReasonResource<'_> {
    pub async fn list(
        &self,
        option: &RequestOption,
    ) -> Result<ListTerminationReasonResp, LarkError> {
        self.list_by_query(&ListTerminationReasonQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTerminationReasonQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTerminationReasonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/termination_reasons",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListTerminationReasonRespData, ListTerminationReasonResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListTerminationReasonIterator<'_> {
        let query = ListTerminationReasonQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListTerminationReasonQuery<'_>,
    ) -> ListTerminationReasonIterator<'_> {
        ListTerminationReasonIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

pub struct UserRoleResource<'a> {
    config: &'a Config,
}

impl UserRoleResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListUserRoleResp, LarkError> {
        self.list_by_query(&ListUserRoleQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListUserRoleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListUserRoleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/user_roles",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id", query.user_id)
        .query("role_id", query.role_id)
        .query("update_start_time", query.update_start_time)
        .query("update_end_time", query.update_end_time)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListUserRoleRespData, ListUserRoleResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        user_id: Option<&str>,
        role_id: Option<&str>,
        update_start_time: Option<&str>,
        update_end_time: Option<&str>,
        user_id_type: Option<&str>,
    ) -> ListUserRoleIterator<'_> {
        let query = ListUserRoleQuery::new()
            .page_size(page_size)
            .user_id(user_id)
            .role_id(role_id)
            .update_start_time(update_start_time)
            .update_end_time(update_end_time)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListUserRoleQuery<'_>,
    ) -> ListUserRoleIterator<'_> {
        ListUserRoleIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            user_id: query.user_id.map(ToOwned::to_owned),
            role_id: query.role_id.map(ToOwned::to_owned),
            update_start_time: query.update_start_time.map(ToOwned::to_owned),
            update_end_time: query.update_end_time.map(ToOwned::to_owned),
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}
pub struct WebsiteResource<'a> {
    config: &'a Config,
}

impl WebsiteResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListWebsiteResp, LarkError> {
        self.list_by_query(&ListWebsiteQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListWebsiteQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListWebsiteResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/websites",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListWebsiteRespData, ListWebsiteResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListWebsiteIterator<'_> {
        let query = ListWebsiteQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(&self, query: &ListWebsiteQuery<'_>) -> ListWebsiteIterator<'_> {
        ListWebsiteIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

// ── WebsiteJobPost resource ──

pub struct WebsiteJobPostResource<'a> {
    config: &'a Config,
}

impl WebsiteJobPostResource<'_> {
    pub async fn get(
        &self,
        website_id: &str,
        job_post_id: &str,
        option: &RequestOption,
    ) -> Result<GetWebsiteJobPostResp, LarkError> {
        let query = GetWebsiteJobPostQuery::new(website_id, job_post_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetWebsiteJobPostQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetWebsiteJobPostResp, LarkError> {
        let path = format!(
            "/open-apis/hire/v1/websites/{}/job_posts/{}",
            query.website_id, query.job_post_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .send_v2_response::<GetWebsiteJobPostRespData, GetWebsiteJobPostResp>()
        .await
    }

    pub async fn list(
        &self,
        website_id: &str,
        option: &RequestOption,
    ) -> Result<ListWebsiteJobPostResp, LarkError> {
        let query = ListWebsiteJobPostQuery::new(website_id);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListWebsiteJobPostQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListWebsiteJobPostResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{}/job_posts", query.website_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .query("update_start_time", query.update_start_time)
        .query("update_end_time", query.update_end_time)
        .query("create_start_time", query.create_start_time)
        .query("create_end_time", query.create_end_time)
        .send_v2_response::<ListWebsiteJobPostRespData, ListWebsiteJobPostResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub fn list_by_iterator(
        &self,
        website_id: &str,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        job_level_id_type: Option<&str>,
        update_start_time: Option<&str>,
        update_end_time: Option<&str>,
        create_start_time: Option<&str>,
        create_end_time: Option<&str>,
    ) -> ListWebsiteJobPostIterator<'_> {
        let query = ListWebsiteJobPostQuery::new(website_id)
            .page_size(page_size)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .job_level_id_type(job_level_id_type)
            .update_start_time(update_start_time)
            .update_end_time(update_end_time)
            .create_start_time(create_start_time)
            .create_end_time(create_end_time);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListWebsiteJobPostQuery<'_>,
    ) -> ListWebsiteJobPostIterator<'_> {
        ListWebsiteJobPostIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            website_id: query.website_id.to_owned(),
            page_size: query.page_size,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
            department_id_type: query.department_id_type.map(ToOwned::to_owned),
            job_level_id_type: query.job_level_id_type.map(ToOwned::to_owned),
            update_start_time: query.update_start_time.map(ToOwned::to_owned),
            update_end_time: query.update_end_time.map(ToOwned::to_owned),
            create_start_time: query.create_start_time.map(ToOwned::to_owned),
            create_end_time: query.create_end_time.map(ToOwned::to_owned),
        }
    }

    pub async fn search(
        &self,
        website_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchWebsiteJobPostResp, LarkError> {
        let query = SearchWebsiteJobPostQuery::new(website_id);
        self.search_by_query(&query, body, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchWebsiteJobPostQuery<'_>,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchWebsiteJobPostResp, LarkError> {
        let path = format!(
            "/open-apis/hire/v1/websites/{}/job_posts/search",
            query.website_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .json_body(&body)?
        .send_v2_response::<SearchWebsiteJobPostRespData, SearchWebsiteJobPostResp>()
        .await
    }

    pub fn search_by_iterator(
        &self,
        website_id: &str,
        body: serde_json::Value,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        job_level_id_type: Option<&str>,
    ) -> SearchWebsiteJobPostIterator<'_> {
        let query = SearchWebsiteJobPostQuery::new(website_id)
            .page_size(page_size)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .job_level_id_type(job_level_id_type);
        self.search_iterator_by_query(&query, body)
    }

    pub fn search_iterator_by_query(
        &self,
        query: &SearchWebsiteJobPostQuery<'_>,
        body: serde_json::Value,
    ) -> SearchWebsiteJobPostIterator<'_> {
        SearchWebsiteJobPostIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            website_id: query.website_id.to_owned(),
            body,
            page_size: query.page_size,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
            department_id_type: query.department_id_type.map(ToOwned::to_owned),
            job_level_id_type: query.job_level_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── InterviewRecord resource ──

pub struct InterviewRecordResource<'a> {
    config: &'a Config,
}

impl InterviewRecordResource<'_> {
    pub async fn get(
        &self,
        interview_record_id: &str,
        option: &RequestOption,
    ) -> Result<GetInterviewRecordResp, LarkError> {
        let path = format!("/open-apis/hire/v1/interview_records/{interview_record_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetInterviewRecordRespData, GetInterviewRecordResp>()
        .await
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListInterviewRecordResp, LarkError> {
        self.list_by_query(&ListInterviewRecordQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewRecordResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query_values("ids", query.ids)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListInterviewRecordRespData, ListInterviewRecordResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListInterviewRecordIterator<'_> {
        let query = ListInterviewRecordQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListInterviewRecordQuery<'_>,
    ) -> ListInterviewRecordIterator<'_> {
        ListInterviewRecordIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            ids: query
                .ids
                .map(|values| values.iter().map(|value| (*value).to_owned()).collect()),
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── Interviewer resource ──

pub struct InterviewerResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListInterviewerQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_ids: Option<&'a [&'a str]>,
    pub verify_status: Option<i32>,
    pub earliest_update_time: Option<&'a str>,
    pub latest_update_time: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListInterviewerQuery<'a> {
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

    pub fn user_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.user_ids = value.into();
        self
    }

    pub fn verify_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.verify_status = value.into();
        self
    }

    pub fn earliest_update_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.earliest_update_time = value.into();
        self
    }

    pub fn latest_update_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.latest_update_time = value.into();
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

impl InterviewerResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListInterviewerResp, LarkError> {
        self.list_by_query(&ListInterviewerQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewerQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewerResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interviewers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query_values("user_ids", query.user_ids.map(|ids| ids.iter().copied()))
        .query("verify_status", query.verify_status)
        .query("earliest_update_time", query.earliest_update_time)
        .query("latest_update_time", query.latest_update_time)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListInterviewerRespData, ListInterviewerResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        user_ids: Option<&[&str]>,
        verify_status: Option<i32>,
        earliest_update_time: Option<&str>,
        latest_update_time: Option<&str>,
        user_id_type: Option<&str>,
    ) -> ListInterviewerIterator<'_> {
        let query = ListInterviewerQuery::new()
            .page_size(page_size)
            .user_ids(user_ids)
            .verify_status(verify_status)
            .earliest_update_time(earliest_update_time)
            .latest_update_time(latest_update_time)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListInterviewerQuery<'_>,
    ) -> ListInterviewerIterator<'_> {
        ListInterviewerIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            user_ids: query
                .user_ids
                .map(|ids| ids.iter().map(|id| (*id).to_owned()).collect())
                .unwrap_or_default(),
            verify_status: query.verify_status,
            earliest_update_time: query.earliest_update_time.map(ToOwned::to_owned),
            latest_update_time: query.latest_update_time.map(ToOwned::to_owned),
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }

    pub async fn patch(
        &self,
        interviewer_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchInterviewerResp, LarkError> {
        let path = format!("/open-apis/hire/v1/interviewers/{interviewer_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchInterviewerResp>()
        .await
    }
}

// ── External resources (CRUD) ──

macro_rules! external_crud_resource {
    ($struct_name:ident, $base_path:literal,
     $create_resp:ident, $update_resp:ident, $delete_resp:ident, $id_param:ident) => {
        pub struct $struct_name<'a> {
            config: &'a Config,
        }
        impl $struct_name<'_> {
            pub async fn create(
                &self,
                body: serde_json::Value,
                option: &RequestOption,
            ) -> Result<$create_resp, LarkError> {
                RestRequest::new(
                    self.config,
                    http::Method::POST,
                    $base_path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .json_body(&body)?
                .send_v2_response::<serde_json::Value, $create_resp>()
                .await
            }

            pub async fn update(
                &self,
                $id_param: &str,
                body: serde_json::Value,
                option: &RequestOption,
            ) -> Result<$update_resp, LarkError> {
                let path = format!("{}/{}", $base_path, $id_param);
                RestRequest::new(
                    self.config,
                    http::Method::PUT,
                    path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .json_body(&body)?
                .send_v2_response::<serde_json::Value, $update_resp>()
                .await
            }

            pub async fn delete(
                &self,
                $id_param: &str,
                option: &RequestOption,
            ) -> Result<$delete_resp, LarkError> {
                let path = format!("{}/{}", $base_path, $id_param);
                RestRequest::new(
                    self.config,
                    http::Method::DELETE,
                    path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .send_v2_response::<(), $delete_resp>()
                .await
            }
        }
    };
}

external_crud_resource!(
    ExternalApplicationResource,
    "/open-apis/hire/v1/external_applications",
    CreateExternalApplicationResp,
    UpdateExternalApplicationResp,
    DeleteExternalApplicationResp,
    external_application_id
);

impl ExternalApplicationResource<'_> {
    pub async fn list(
        &self,
        option: &RequestOption,
    ) -> Result<ListExternalApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/external_applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, ListExternalApplicationResp>()
        .await
    }
}

external_crud_resource!(
    ExternalOfferResource,
    "/open-apis/hire/v1/external_offers",
    CreateExternalOfferResp,
    UpdateExternalOfferResp,
    DeleteExternalOfferResp,
    external_offer_id
);

impl ExternalOfferResource<'_> {
    pub async fn batch_query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchQueryExternalOfferResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_offers/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchQueryExternalOfferResp>()
        .await
    }
}

external_crud_resource!(
    ExternalInterviewResource,
    "/open-apis/hire/v1/external_interviews",
    CreateExternalInterviewResp,
    UpdateExternalInterviewResp,
    DeleteExternalInterviewResp,
    external_interview_id
);

impl ExternalInterviewResource<'_> {
    pub async fn batch_query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchQueryExternalInterviewResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_interviews/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchQueryExternalInterviewResp>()
        .await
    }
}

external_crud_resource!(
    ExternalBackgroundCheckResource,
    "/open-apis/hire/v1/external_background_checks",
    CreateExternalBackgroundCheckResp,
    UpdateExternalBackgroundCheckResp,
    DeleteExternalBackgroundCheckResp,
    external_background_check_id
);

impl ExternalBackgroundCheckResource<'_> {
    pub async fn batch_query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchQueryExternalBackgroundCheckResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_background_checks/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchQueryExternalBackgroundCheckResp>()
        .await
    }
}

// ── Todo resource ──

pub struct TodoResource<'a> {
    config: &'a Config,
}

impl TodoResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListTodoResp, LarkError> {
        self.list_by_query(&ListTodoQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTodoQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTodoResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/todos",
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", query.page_size)
        .query("page_token", query.page_token)
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .query("type", query.type_)
        .send_v2_response::<ListTodoRespData, ListTodoResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<&str>,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        type_: Option<&str>,
    ) -> ListTodoIterator<'_> {
        let query = ListTodoQuery::new()
            .page_size(page_size)
            .user_id(user_id)
            .user_id_type(user_id_type)
            .type_(type_);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(&self, query: &ListTodoQuery<'_>) -> ListTodoIterator<'_> {
        ListTodoIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size.map(ToOwned::to_owned),
            user_id: query.user_id.map(ToOwned::to_owned),
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
            type_: query.type_.map(ToOwned::to_owned),
        }
    }
}

// ── TripartiteAgreement resource ──

pub struct TripartiteAgreementResource<'a> {
    config: &'a Config,
}

impl TripartiteAgreementResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateTripartiteAgreementResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/tripartite_agreements",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateTripartiteAgreementResp>()
        .await
    }

    pub async fn delete(
        &self,
        tripartite_agreement_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteTripartiteAgreementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/tripartite_agreements/{tripartite_agreement_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteTripartiteAgreementResp>()
        .await
    }

    pub async fn list(
        &self,
        option: &RequestOption,
    ) -> Result<ListTripartiteAgreementResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/tripartite_agreements",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, ListTripartiteAgreementResp>()
        .await
    }

    pub async fn update(
        &self,
        tripartite_agreement_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateTripartiteAgreementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/tripartite_agreements/{tripartite_agreement_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateTripartiteAgreementResp>()
        .await
    }
}

// ── Advertisement resource ──

pub struct AdvertisementResource<'a> {
    config: &'a Config,
}

impl AdvertisementResource<'_> {
    pub async fn publish(
        &self,
        advertisement_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PublishAdvertisementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/advertisements/{advertisement_id}/publish");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PublishAdvertisementResp>()
        .await
    }
}

// ── Agency resource ──

pub struct AgencyResource<'a> {
    config: &'a Config,
}

impl AgencyResource<'_> {
    pub async fn batch_query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchQueryAgencyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchQueryAgencyResp>()
        .await
    }

    pub async fn get_agency_account(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<GetAgencyAccountResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/get_agency_account",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, GetAgencyAccountResp>()
        .await
    }

    pub async fn operate_agency_account(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<OperateAgencyAccountResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/operate_agency_account",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, OperateAgencyAccountResp>()
        .await
    }

    pub async fn protect(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ProtectAgencyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/protect",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, ProtectAgencyResp>()
        .await
    }

    pub async fn protect_search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ProtectSearchAgencyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/protection_period/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, ProtectSearchAgencyResp>()
        .await
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryAgencyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/agencies/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, QueryAgencyResp>()
        .await
    }

    pub async fn get(
        &self,
        agency_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAgencyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/agencies/{agency_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2_response::<serde_json::Value, GetAgencyResp>()
        .await
    }
}

// ── BackgroundCheckOrder resource ──

pub struct BackgroundCheckOrderResource<'a> {
    config: &'a Config,
}

impl BackgroundCheckOrderResource<'_> {
    pub async fn batch_query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchQueryBackgroundCheckOrderResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/background_check_orders/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchQueryBackgroundCheckOrderResp>()
        .await
    }

    pub async fn list(
        &self,
        option: &RequestOption,
    ) -> Result<ListBackgroundCheckOrderResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/background_check_orders",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, ListBackgroundCheckOrderResp>()
        .await
    }
}

// ── DiversityInclusion resource ──

pub struct DiversityInclusionResource<'a> {
    config: &'a Config,
}

impl DiversityInclusionResource<'_> {
    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchDiversityInclusionResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/applications/diversity_inclusions/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, SearchDiversityInclusionResp>()
        .await
    }
}

// ── EcoAccountCustomField resource ──

pub struct EcoAccountCustomFieldResource<'a> {
    config: &'a Config,
}

impl EcoAccountCustomFieldResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEcoAccountCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_account_custom_fields",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateEcoAccountCustomFieldResp>()
        .await
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoAccountCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_account_custom_fields/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchDeleteEcoAccountCustomFieldResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoAccountCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_account_custom_fields/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchUpdateEcoAccountCustomFieldResp>()
        .await
    }
}

// ── EcoBackgroundCheck resource ──

pub struct EcoBackgroundCheckResource<'a> {
    config: &'a Config,
}

impl EcoBackgroundCheckResource<'_> {
    pub async fn cancel(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CancelEcoBackgroundCheckResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/cancel",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CancelEcoBackgroundCheckResp>()
        .await
    }

    pub async fn update_progress(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateProgressEcoBackgroundCheckResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/update_progress",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateProgressEcoBackgroundCheckResp>()
        .await
    }

    pub async fn update_result(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateResultEcoBackgroundCheckResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/update_result",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateResultEcoBackgroundCheckResp>()
        .await
    }
}

// ── EcoBackgroundCheckCustomField resource ──

pub struct EcoBackgroundCheckCustomFieldResource<'a> {
    config: &'a Config,
}

impl EcoBackgroundCheckCustomFieldResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEcoBackgroundCheckCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_custom_fields",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateEcoBackgroundCheckCustomFieldResp>()
        .await
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoBackgroundCheckCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchDeleteEcoBackgroundCheckCustomFieldResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoBackgroundCheckCustomFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchUpdateEcoBackgroundCheckCustomFieldResp>()
        .await
    }
}

// ── EcoBackgroundCheckPackage resource ──

pub struct EcoBackgroundCheckPackageResource<'a> {
    config: &'a Config,
}

impl EcoBackgroundCheckPackageResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEcoBackgroundCheckPackageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_packages",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateEcoBackgroundCheckPackageResp>()
        .await
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoBackgroundCheckPackageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_packages/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchDeleteEcoBackgroundCheckPackageResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoBackgroundCheckPackageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_background_check_packages/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchUpdateEcoBackgroundCheckPackageResp>()
        .await
    }
}

// ── EcoExam resource ──

pub struct EcoExamResource<'a> {
    config: &'a Config,
}

impl EcoExamResource<'_> {
    pub async fn login_info(
        &self,
        exam_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<LoginInfoEcoExamResp, LarkError> {
        let path = format!("/open-apis/hire/v1/eco_exams/{exam_id}/login_info");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, LoginInfoEcoExamResp>()
        .await
    }

    pub async fn update_result(
        &self,
        exam_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateResultEcoExamResp, LarkError> {
        let path = format!("/open-apis/hire/v1/eco_exams/{exam_id}/update_result");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateResultEcoExamResp>()
        .await
    }
}

// ── EcoExamPaper resource ──

pub struct EcoExamPaperResource<'a> {
    config: &'a Config,
}

impl EcoExamPaperResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEcoExamPaperResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_exam_papers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateEcoExamPaperResp>()
        .await
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoExamPaperResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_exam_papers/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchDeleteEcoExamPaperResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoExamPaperResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_exam_papers/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchUpdateEcoExamPaperResp>()
        .await
    }
}

// ── JobManager resource ──

pub struct JobManagerResource<'a> {
    config: &'a Config,
}

impl JobManagerResource<'_> {
    pub async fn batch_update(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateJobManagerResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/managers/batch_update");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchUpdateJobManagerResp>()
        .await
    }

    pub async fn get(
        &self,
        job_id: &str,
        manager_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetJobManagerResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/managers/{manager_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2_response::<serde_json::Value, GetJobManagerResp>()
        .await
    }
}

// ── JobPublishRecord resource ──

pub struct JobPublishRecordResource<'a> {
    config: &'a Config,
}

impl JobPublishRecordResource<'_> {
    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchJobPublishRecordResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/job_publish_records/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, SearchJobPublishRecordResp>()
        .await
    }
}

// ── ReferralAccount resource ──

pub struct ReferralAccountResource<'a> {
    config: &'a Config,
}

impl ReferralAccountResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateReferralAccountResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referral_account",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateReferralAccountResp>()
        .await
    }

    pub async fn deactivate(
        &self,
        referral_account_id: &str,
        option: &RequestOption,
    ) -> Result<DeactivateReferralAccountResp, LarkError> {
        let path = format!("/open-apis/hire/v1/referral_account/{referral_account_id}/deactivate");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, DeactivateReferralAccountResp>()
        .await
    }

    pub async fn enable(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EnableReferralAccountResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referral_account/enable",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, EnableReferralAccountResp>()
        .await
    }

    pub async fn get_account_assets(
        &self,
        option: &RequestOption,
    ) -> Result<GetAccountAssetsReferralAccountResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/referral_account/get_account_assets",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetAccountAssetsReferralAccountResp>()
        .await
    }

    pub async fn reconciliation(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ReconciliationReferralAccountResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referral_account/reconciliation",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, ReconciliationReferralAccountResp>()
        .await
    }

    pub async fn withdraw(
        &self,
        referral_account_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WithdrawReferralAccountResp, LarkError> {
        let path = format!("/open-apis/hire/v1/referral_account/{referral_account_id}/withdraw");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, WithdrawReferralAccountResp>()
        .await
    }
}

// ── TalentBlocklist resource ──

pub struct TalentBlocklistResource<'a> {
    config: &'a Config,
}

impl TalentBlocklistResource<'_> {
    pub async fn change_talent_block(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ChangeTalentBlockResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talent_blocklist/change_talent_block",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, ChangeTalentBlockResp>()
        .await
    }
}

// ── TalentObject resource ──

pub struct TalentObjectResource<'a> {
    config: &'a Config,
}

impl TalentObjectResource<'_> {
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTalentObjectResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talent_objects/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, QueryTalentObjectResp>()
        .await
    }
}

// ── TalentOperationLog resource ──

pub struct TalentOperationLogResource<'a> {
    config: &'a Config,
}

impl TalentOperationLogResource<'_> {
    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchTalentOperationLogResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talent_operation_logs/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, SearchTalentOperationLogResp>()
        .await
    }
}

// ── TalentPool resource ──

pub struct TalentPoolResource<'a> {
    config: &'a Config,
}

impl TalentPoolResource<'_> {
    pub async fn batch_change_talent_pool(
        &self,
        talent_pool_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchChangeTalentPoolResp, LarkError> {
        let path =
            format!("/open-apis/hire/v1/talent_pools/{talent_pool_id}/batch_change_talent_pool");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchChangeTalentPoolResp>()
        .await
    }

    pub async fn move_talent(
        &self,
        talent_pool_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<MoveTalentTalentPoolResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talent_pools/{talent_pool_id}/talent_relationship");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, MoveTalentTalentPoolResp>()
        .await
    }

    pub async fn search(&self, option: &RequestOption) -> Result<SearchTalentPoolResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talent_pools",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, SearchTalentPoolResp>()
        .await
    }
}

// ── Test resource ──

pub struct TestResource<'a> {
    config: &'a Config,
}

impl TestResource<'_> {
    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchTestResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/tests/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, SearchTestResp>()
        .await
    }
}

// ── WebsiteDelivery resource ──

pub struct WebsiteDeliveryResource<'a> {
    config: &'a Config,
}

impl WebsiteDeliveryResource<'_> {
    pub async fn create_by_attachment(
        &self,
        website_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateByAttachmentWebsiteDeliveryResp, LarkError> {
        let path =
            format!("/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_attachment");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateByAttachmentWebsiteDeliveryResp>()
        .await
    }

    pub async fn create_by_resume(
        &self,
        website_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateByResumeWebsiteDeliveryResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_resume");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateByResumeWebsiteDeliveryResp>()
        .await
    }
}

// ── ApplicationInterview resource ──

pub struct ApplicationInterviewResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListApplicationInterviewQuery<'a> {
    pub application_id: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub job_level_id_type: Option<&'a str>,
}

impl<'a> ListApplicationInterviewQuery<'a> {
    pub fn new(application_id: &'a str) -> Self {
        Self {
            application_id,
            page_size: None,
            page_token: None,
            user_id_type: None,
            job_level_id_type: None,
        }
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn job_level_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.job_level_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl ApplicationInterviewResource<'_> {
    pub async fn list(
        &self,
        application_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListApplicationInterviewResp2, LarkError> {
        let query = ListApplicationInterviewQuery::new(application_id)
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationInterviewQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationInterviewResp2, LarkError> {
        let path = format!(
            "/open-apis/hire/v1/applications/{}/interviews",
            query.application_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .send_v2_response::<ListApplicationInterviewRespData, ListApplicationInterviewResp2>()
        .await
    }
}

// ── EhrImportTask resource ──

pub struct EhrImportTaskResource<'a> {
    config: &'a Config,
}

impl EhrImportTaskResource<'_> {
    pub async fn patch(
        &self,
        ehr_import_task_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEhrImportTaskResp, LarkError> {
        let path = format!("/open-apis/hire/v1/ehr_import_tasks/{ehr_import_task_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchEhrImportTaskResp>()
        .await
    }
}

// ── EvaluationTask resource ──

pub struct EvaluationTaskResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListEvaluationTaskQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub activity_status: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListEvaluationTaskQuery<'a> {
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

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn activity_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.activity_status = value.into();
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

impl EvaluationTaskResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEvaluationTaskResp, LarkError> {
        let query = ListEvaluationTaskQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEvaluationTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEvaluationTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/evaluation_tasks",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id", query.user_id)
        .query("activity_status", query.activity_status)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListEvaluationTaskRespData, ListEvaluationTaskResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        user_id: Option<&str>,
        activity_status: Option<i32>,
        user_id_type: Option<&str>,
    ) -> ListEvaluationTaskIterator<'_> {
        let query = ListEvaluationTaskQuery::new()
            .page_size(page_size)
            .user_id(user_id)
            .activity_status(activity_status)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListEvaluationTaskQuery<'_>,
    ) -> ListEvaluationTaskIterator<'_> {
        ListEvaluationTaskIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            user_id: query.user_id.map(ToOwned::to_owned),
            activity_status: query.activity_status,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── Exam resource ──

pub struct ExamResource<'a> {
    config: &'a Config,
}

impl ExamResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateExamResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/exams",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateExamResp>()
        .await
    }
}

// ── ExamMarkingTask resource ──

pub struct ExamMarkingTaskResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListExamMarkingTaskQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub activity_status: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListExamMarkingTaskQuery<'a> {
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

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn activity_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.activity_status = value.into();
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

impl ExamMarkingTaskResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListExamMarkingTaskResp, LarkError> {
        let query = ListExamMarkingTaskQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListExamMarkingTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListExamMarkingTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/exam_marking_tasks",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id", query.user_id)
        .query("activity_status", query.activity_status)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListExamMarkingTaskRespData, ListExamMarkingTaskResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        user_id: Option<&str>,
        activity_status: Option<i32>,
        user_id_type: Option<&str>,
    ) -> ListExamMarkingTaskIterator<'_> {
        let query = ListExamMarkingTaskQuery::new()
            .page_size(page_size)
            .user_id(user_id)
            .activity_status(activity_status)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListExamMarkingTaskQuery<'_>,
    ) -> ListExamMarkingTaskIterator<'_> {
        ListExamMarkingTaskIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            user_id: query.user_id.map(ToOwned::to_owned),
            activity_status: query.activity_status,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── ExternalInterviewAssessment resource ──

pub struct ExternalInterviewAssessmentResource<'a> {
    config: &'a Config,
}

impl ExternalInterviewAssessmentResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateExternalInterviewAssessmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_interview_assessments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateExternalInterviewAssessmentResp>()
        .await
    }

    pub async fn patch(
        &self,
        external_interview_assessment_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchExternalInterviewAssessmentResp, LarkError> {
        let path = format!(
            "/open-apis/hire/v1/external_interview_assessments/{external_interview_assessment_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, PatchExternalInterviewAssessmentResp>()
        .await
    }
}

// ── ExternalReferralReward resource ──

pub struct ExternalReferralRewardResource<'a> {
    config: &'a Config,
}

impl ExternalReferralRewardResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateExternalReferralRewardResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_referral_rewards",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateExternalReferralRewardResp>()
        .await
    }

    pub async fn delete(
        &self,
        external_referral_reward_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteExternalReferralRewardResp, LarkError> {
        let path =
            format!("/open-apis/hire/v1/external_referral_rewards/{external_referral_reward_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteExternalReferralRewardResp>()
        .await
    }
}

// ── InterviewFeedbackForm resource ──

pub struct InterviewFeedbackFormResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListInterviewFeedbackFormQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub interview_feedback_form_ids: Option<&'a [&'a str]>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListInterviewFeedbackFormQuery<'a> {
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

    pub fn interview_feedback_form_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.interview_feedback_form_ids = value.into();
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

impl InterviewFeedbackFormResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewFeedbackFormResp, LarkError> {
        let query = ListInterviewFeedbackFormQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewFeedbackFormQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewFeedbackFormResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_feedback_forms",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query_values(
            "interview_feedback_form_ids",
            query
                .interview_feedback_form_ids
                .map(|ids| ids.iter().copied()),
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListInterviewFeedbackFormRespData, ListInterviewFeedbackFormResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        interview_feedback_form_ids: Option<&[&str]>,
        user_id_type: Option<&str>,
    ) -> ListInterviewFeedbackFormIterator<'_> {
        let query = ListInterviewFeedbackFormQuery::new()
            .page_size(page_size)
            .interview_feedback_form_ids(interview_feedback_form_ids)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListInterviewFeedbackFormQuery<'_>,
    ) -> ListInterviewFeedbackFormIterator<'_> {
        ListInterviewFeedbackFormIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            interview_feedback_form_ids: query
                .interview_feedback_form_ids
                .map(|ids| ids.iter().map(|id| (*id).to_owned()).collect())
                .unwrap_or_default(),
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── InterviewRecordAttachment resource ──

pub struct InterviewRecordAttachmentResource<'a> {
    config: &'a Config,
}

impl InterviewRecordAttachmentResource<'_> {
    pub async fn get(
        &self,
        option: &RequestOption,
    ) -> Result<GetInterviewRecordAttachmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_records/attachments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetInterviewRecordAttachmentResp>()
        .await
    }
}

// ── InterviewRegistrationSchema resource ──

pub struct InterviewRegistrationSchemaResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListInterviewRegistrationSchemaQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListInterviewRegistrationSchemaQuery<'a> {
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl InterviewRegistrationSchemaResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewRegistrationSchemaResp, LarkError> {
        let query = ListInterviewRegistrationSchemaQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewRegistrationSchemaQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewRegistrationSchemaResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_registration_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<
            ListInterviewRegistrationSchemaRespData,
            ListInterviewRegistrationSchemaResp,
        >()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
    ) -> ListInterviewRegistrationSchemaIterator<'_> {
        let query = ListInterviewRegistrationSchemaQuery::new()
            .page_size(page_size)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListInterviewRegistrationSchemaQuery<'_>,
    ) -> ListInterviewRegistrationSchemaIterator<'_> {
        ListInterviewRegistrationSchemaIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── InterviewRoundType resource ──

pub struct InterviewRoundTypeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListInterviewRoundTypeQuery {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub process_type: Option<i32>,
}

impl ListInterviewRoundTypeQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<String>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn process_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.process_type = value.into();
        self
    }
}

impl InterviewRoundTypeResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewRoundTypeResp, LarkError> {
        let query = ListInterviewRoundTypeQuery::new()
            .page_size(page_size)
            .page_token(page_token.map(ToOwned::to_owned));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewRoundTypeQuery,
        option: &RequestOption,
    ) -> Result<ListInterviewRoundTypeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_round_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", query.page_size)
        .query("page_token", query.page_token.as_deref())
        .query("process_type", query.process_type)
        .send_v2_response::<ListInterviewRoundTypeRespData, ListInterviewRoundTypeResp>()
        .await
    }
}

// ── InterviewTask resource ──

pub struct InterviewTaskResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListInterviewTaskQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub activity_status: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListInterviewTaskQuery<'a> {
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

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn activity_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.activity_status = value.into();
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

impl InterviewTaskResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewTaskResp, LarkError> {
        let query = ListInterviewTaskQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_tasks",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id", query.user_id)
        .query("activity_status", query.activity_status)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListInterviewTaskRespData, ListInterviewTaskResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        user_id: Option<&str>,
        activity_status: Option<i32>,
        user_id_type: Option<&str>,
    ) -> ListInterviewTaskIterator<'_> {
        let query = ListInterviewTaskQuery::new()
            .page_size(page_size)
            .user_id(user_id)
            .activity_status(activity_status)
            .user_id_type(user_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListInterviewTaskQuery<'_>,
    ) -> ListInterviewTaskIterator<'_> {
        ListInterviewTaskIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            user_id: query.user_id.map(ToOwned::to_owned),
            activity_status: query.activity_status,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── JobRequirementSchema resource ──

pub struct JobRequirementSchemaResource<'a> {
    config: &'a Config,
}

impl JobRequirementSchemaResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobRequirementSchemaResp, LarkError> {
        let query = ListJobRequirementSchemaQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobRequirementSchemaQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobRequirementSchemaResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_requirement_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListJobRequirementSchemaRespData, ListJobRequirementSchemaResp>()
        .await
    }
}

// ── JobSchema resource ──

pub struct JobSchemaResource<'a> {
    config: &'a Config,
}

impl JobSchemaResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobSchemaResp, LarkError> {
        let query = ListJobSchemaQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListJobSchemaQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListJobSchemaResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("scenario", query.scenario)
        .send_v2_response::<ListJobSchemaRespData, ListJobSchemaResp>()
        .await
    }
}

// ── Minutes resource ──

pub struct MinutesResource<'a> {
    config: &'a Config,
}

impl MinutesResource<'_> {
    pub async fn get(&self, option: &RequestOption) -> Result<GetMinutesResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/minutes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetMinutesResp>()
        .await
    }
}

// ── OfferApplicationForm resource ──

pub struct OfferApplicationFormResource<'a> {
    config: &'a Config,
}

impl OfferApplicationFormResource<'_> {
    pub async fn get(
        &self,
        offer_application_form_id: &str,
        option: &RequestOption,
    ) -> Result<GetOfferApplicationFormResp, LarkError> {
        let path =
            format!("/open-apis/hire/v1/offer_application_forms/{offer_application_form_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetOfferApplicationFormRespData, GetOfferApplicationFormResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListOfferApplicationFormResp, LarkError> {
        let query = ListOfferApplicationFormQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListOfferApplicationFormQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOfferApplicationFormResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/offer_application_forms",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListOfferApplicationFormRespData, ListOfferApplicationFormResp>()
        .await
    }
}

// ── OfferApprovalTemplate resource ──

pub struct OfferApprovalTemplateResource<'a> {
    config: &'a Config,
}

impl OfferApprovalTemplateResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListOfferApprovalTemplateResp, LarkError> {
        let query = ListOfferApprovalTemplateQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListOfferApprovalTemplateQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOfferApprovalTemplateResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/offer_approval_templates",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("department_id_type", query.department_id_type)
        .send_v2_response::<ListOfferApprovalTemplateRespData, ListOfferApprovalTemplateResp>()
        .await
    }
}

// ── OfferCustomField resource ──

pub struct OfferCustomFieldResource<'a> {
    config: &'a Config,
}

impl OfferCustomFieldResource<'_> {
    pub async fn update(
        &self,
        offer_custom_field_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateOfferCustomFieldResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offer_custom_fields/{offer_custom_field_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateOfferCustomFieldResp>()
        .await
    }
}

// ── PortalApplySchema resource ──

pub struct PortalApplySchemaResource<'a> {
    config: &'a Config,
}

impl PortalApplySchemaResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPortalApplySchemaResp, LarkError> {
        let query = ListPortalApplySchemaQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPortalApplySchemaQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPortalApplySchemaResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/portal_apply_schemas",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListPortalApplySchemaRespData, ListPortalApplySchemaResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListPortalApplySchemaIterator<'_> {
        let query = ListPortalApplySchemaQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListPortalApplySchemaQuery<'_>,
    ) -> ListPortalApplySchemaIterator<'_> {
        ListPortalApplySchemaIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

// ── ReferralWebsiteJobPost resource ──

pub struct ReferralWebsiteJobPostResource<'a> {
    config: &'a Config,
}

impl ReferralWebsiteJobPostResource<'_> {
    pub async fn get(
        &self,
        job_post_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReferralWebsiteJobPostResp, LarkError> {
        let query = GetReferralWebsiteJobPostQuery::new(job_post_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetReferralWebsiteJobPostQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetReferralWebsiteJobPostResp, LarkError> {
        let path = format!(
            "/open-apis/hire/v1/referral_websites/job_posts/{}",
            query.job_post_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .send_v2_response::<GetReferralWebsiteJobPostRespData, GetReferralWebsiteJobPostResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListReferralWebsiteJobPostResp, LarkError> {
        let query = ListReferralWebsiteJobPostQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListReferralWebsiteJobPostQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListReferralWebsiteJobPostResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/referral_websites/job_posts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("process_type", query.process_type)
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .send_v2_response::<ListReferralWebsiteJobPostRespData, ListReferralWebsiteJobPostResp>()
        .await
    }

    pub fn list_by_iterator(
        &self,
        page_size: Option<i32>,
        process_type: Option<i32>,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        job_level_id_type: Option<&str>,
    ) -> ListReferralWebsiteJobPostIterator<'_> {
        let query = ListReferralWebsiteJobPostQuery::new()
            .page_size(page_size)
            .process_type(process_type)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .job_level_id_type(job_level_id_type);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListReferralWebsiteJobPostQuery<'_>,
    ) -> ListReferralWebsiteJobPostIterator<'_> {
        ListReferralWebsiteJobPostIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
            process_type: query.process_type,
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
            department_id_type: query.department_id_type.map(ToOwned::to_owned),
            job_level_id_type: query.job_level_id_type.map(ToOwned::to_owned),
        }
    }
}

// ── TalentExternalInfo resource ──

pub struct TalentExternalInfoResource<'a> {
    config: &'a Config,
}

impl TalentExternalInfoResource<'_> {
    pub async fn create(
        &self,
        talent_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateTalentExternalInfoResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/external_info");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateTalentExternalInfoResp>()
        .await
    }

    pub async fn update(
        &self,
        talent_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateTalentExternalInfoResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/external_info");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateTalentExternalInfoResp>()
        .await
    }
}

// ── TalentTag resource ──

pub struct TalentTagResource<'a> {
    config: &'a Config,
}

impl TalentTagResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTalentTagResp, LarkError> {
        let query = ListTalentTagQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTalentTagQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTalentTagResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talent_tags",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListTalentTagRespData, ListTalentTagResp>()
        .await
    }

    pub fn list_by_iterator(&self, page_size: Option<i32>) -> ListTalentTagIterator<'_> {
        let query = ListTalentTagQuery::new().page_size(page_size);
        self.list_iterator_by_query(&query)
    }

    pub fn list_iterator_by_query(
        &self,
        query: &ListTalentTagQuery<'_>,
    ) -> ListTalentTagIterator<'_> {
        ListTalentTagIterator {
            config: self.config,
            state: PageIteratorState::default()
                .with_page_token(query.page_token.map(ToOwned::to_owned)),
            page_size: query.page_size,
        }
    }
}

// ── WebsiteChannel resource ──

pub struct WebsiteChannelResource<'a> {
    config: &'a Config,
}

impl WebsiteChannelResource<'_> {
    pub async fn create(
        &self,
        website_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateWebsiteChannelResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateWebsiteChannelResp>()
        .await
    }

    pub async fn delete(
        &self,
        website_id: &str,
        channel_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteWebsiteChannelResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels/{channel_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteWebsiteChannelResp>()
        .await
    }

    pub async fn list(
        &self,
        website_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWebsiteChannelResp, LarkError> {
        let query = ListWebsiteChannelQuery::new(website_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListWebsiteChannelQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListWebsiteChannelResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{}/channels", query.website_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", query.page_size)
        .query("page_token", query.page_token)
        .send_v2_response::<ListWebsiteChannelRespData, ListWebsiteChannelResp>()
        .await
    }

    pub async fn update(
        &self,
        website_id: &str,
        channel_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateWebsiteChannelResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels/{channel_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, UpdateWebsiteChannelResp>()
        .await
    }
}

// ── WebsiteDeliveryTask resource ──

pub struct WebsiteDeliveryTaskResource<'a> {
    config: &'a Config,
}

impl WebsiteDeliveryTaskResource<'_> {
    pub async fn get(
        &self,
        website_id: &str,
        delivery_task_id: &str,
        option: &RequestOption,
    ) -> Result<GetWebsiteDeliveryTaskResp, LarkError> {
        let path =
            format!("/open-apis/hire/v1/websites/{website_id}/delivery_tasks/{delivery_task_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, GetWebsiteDeliveryTaskResp>()
        .await
    }
}

// ── WebsiteSiteUser resource ──

pub struct WebsiteSiteUserResource<'a> {
    config: &'a Config,
}

impl WebsiteSiteUserResource<'_> {
    pub async fn create(
        &self,
        website_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateWebsiteSiteUserResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/site_users");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateWebsiteSiteUserResp>()
        .await
    }
}

// ── EhrImportTaskForInternshipOffer resource (placeholder — no methods in Go SDK) ──

pub struct EhrImportTaskForInternshipOfferResource<'a> {
    #[allow(dead_code)]
    config: &'a Config,
}

pub struct V1<'a> {
    pub job: JobResource<'a>,
    pub talent: TalentResource<'a>,
    pub application: ApplicationResource<'a>,
    pub interview: InterviewResource<'a>,
    pub offer: OfferResource<'a>,
    pub job_requirement: JobRequirementResource<'a>,
    pub attachment: AttachmentResource<'a>,
    pub offer_schema: OfferSchemaResource<'a>,
    pub employee: EmployeeResource<'a>,
    pub evaluation: EvaluationResource<'a>,
    pub note: NoteResource<'a>,
    pub questionnaire: QuestionnaireResource<'a>,
    pub referral: ReferralResource<'a>,
    pub registration_schema: RegistrationSchemaResource<'a>,
    pub resume_source: ResumeSourceResource<'a>,
    pub job_function: JobFunctionResource<'a>,
    pub job_type: JobTypeResource<'a>,
    pub job_process: JobProcessResource<'a>,
    pub location: LocationResource<'a>,
    pub role: RoleResource<'a>,
    pub subject: SubjectResource<'a>,
    pub talent_folder: TalentFolderResource<'a>,
    pub termination_reason: TerminationReasonResource<'a>,
    pub user_role: UserRoleResource<'a>,
    pub website: WebsiteResource<'a>,
    pub website_job_post: WebsiteJobPostResource<'a>,
    pub interview_record: InterviewRecordResource<'a>,
    pub interviewer: InterviewerResource<'a>,
    pub external_application: ExternalApplicationResource<'a>,
    pub external_offer: ExternalOfferResource<'a>,
    pub external_interview: ExternalInterviewResource<'a>,
    pub external_background_check: ExternalBackgroundCheckResource<'a>,
    pub todo: TodoResource<'a>,
    pub tripartite_agreement: TripartiteAgreementResource<'a>,
    pub advertisement: AdvertisementResource<'a>,
    pub agency: AgencyResource<'a>,
    pub background_check_order: BackgroundCheckOrderResource<'a>,
    pub diversity_inclusion: DiversityInclusionResource<'a>,
    pub eco_account_custom_field: EcoAccountCustomFieldResource<'a>,
    pub eco_background_check: EcoBackgroundCheckResource<'a>,
    pub eco_background_check_custom_field: EcoBackgroundCheckCustomFieldResource<'a>,
    pub eco_background_check_package: EcoBackgroundCheckPackageResource<'a>,
    pub eco_exam: EcoExamResource<'a>,
    pub eco_exam_paper: EcoExamPaperResource<'a>,
    pub job_manager: JobManagerResource<'a>,
    pub job_publish_record: JobPublishRecordResource<'a>,
    pub referral_account: ReferralAccountResource<'a>,
    pub talent_blocklist: TalentBlocklistResource<'a>,
    pub talent_object: TalentObjectResource<'a>,
    pub talent_operation_log: TalentOperationLogResource<'a>,
    pub talent_pool: TalentPoolResource<'a>,
    pub test: TestResource<'a>,
    pub website_delivery: WebsiteDeliveryResource<'a>,
    pub application_interview: ApplicationInterviewResource<'a>,
    pub ehr_import_task: EhrImportTaskResource<'a>,
    pub ehr_import_task_for_internship_offer: EhrImportTaskForInternshipOfferResource<'a>,
    pub evaluation_task: EvaluationTaskResource<'a>,
    pub exam: ExamResource<'a>,
    pub exam_marking_task: ExamMarkingTaskResource<'a>,
    pub external_interview_assessment: ExternalInterviewAssessmentResource<'a>,
    pub external_referral_reward: ExternalReferralRewardResource<'a>,
    pub interview_feedback_form: InterviewFeedbackFormResource<'a>,
    pub interview_record_attachment: InterviewRecordAttachmentResource<'a>,
    pub interview_registration_schema: InterviewRegistrationSchemaResource<'a>,
    pub interview_round_type: InterviewRoundTypeResource<'a>,
    pub interview_task: InterviewTaskResource<'a>,
    pub job_requirement_schema: JobRequirementSchemaResource<'a>,
    pub job_schema: JobSchemaResource<'a>,
    pub minutes: MinutesResource<'a>,
    pub offer_application_form: OfferApplicationFormResource<'a>,
    pub offer_approval_template: OfferApprovalTemplateResource<'a>,
    pub offer_custom_field: OfferCustomFieldResource<'a>,
    pub portal_apply_schema: PortalApplySchemaResource<'a>,
    pub referral_website_job_post: ReferralWebsiteJobPostResource<'a>,
    pub talent_external_info: TalentExternalInfoResource<'a>,
    pub talent_tag: TalentTagResource<'a>,
    pub website_channel: WebsiteChannelResource<'a>,
    pub website_delivery_task: WebsiteDeliveryTaskResource<'a>,
    pub website_site_user: WebsiteSiteUserResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            job: JobResource { config },
            talent: TalentResource { config },
            application: ApplicationResource { config },
            interview: InterviewResource { config },
            offer: OfferResource { config },
            job_requirement: JobRequirementResource { config },
            attachment: AttachmentResource { config },
            offer_schema: OfferSchemaResource { config },
            employee: EmployeeResource { config },
            evaluation: EvaluationResource { config },
            note: NoteResource { config },
            questionnaire: QuestionnaireResource { config },
            referral: ReferralResource { config },
            registration_schema: RegistrationSchemaResource { config },
            resume_source: ResumeSourceResource { config },
            job_function: JobFunctionResource { config },
            job_type: JobTypeResource { config },
            job_process: JobProcessResource { config },
            location: LocationResource { config },
            role: RoleResource { config },
            subject: SubjectResource { config },
            talent_folder: TalentFolderResource { config },
            termination_reason: TerminationReasonResource { config },
            user_role: UserRoleResource { config },
            website: WebsiteResource { config },
            website_job_post: WebsiteJobPostResource { config },
            interview_record: InterviewRecordResource { config },
            interviewer: InterviewerResource { config },
            external_application: ExternalApplicationResource { config },
            external_offer: ExternalOfferResource { config },
            external_interview: ExternalInterviewResource { config },
            external_background_check: ExternalBackgroundCheckResource { config },
            todo: TodoResource { config },
            tripartite_agreement: TripartiteAgreementResource { config },
            advertisement: AdvertisementResource { config },
            agency: AgencyResource { config },
            background_check_order: BackgroundCheckOrderResource { config },
            diversity_inclusion: DiversityInclusionResource { config },
            eco_account_custom_field: EcoAccountCustomFieldResource { config },
            eco_background_check: EcoBackgroundCheckResource { config },
            eco_background_check_custom_field: EcoBackgroundCheckCustomFieldResource { config },
            eco_background_check_package: EcoBackgroundCheckPackageResource { config },
            eco_exam: EcoExamResource { config },
            eco_exam_paper: EcoExamPaperResource { config },
            job_manager: JobManagerResource { config },
            job_publish_record: JobPublishRecordResource { config },
            referral_account: ReferralAccountResource { config },
            talent_blocklist: TalentBlocklistResource { config },
            talent_object: TalentObjectResource { config },
            talent_operation_log: TalentOperationLogResource { config },
            talent_pool: TalentPoolResource { config },
            test: TestResource { config },
            website_delivery: WebsiteDeliveryResource { config },
            application_interview: ApplicationInterviewResource { config },
            ehr_import_task: EhrImportTaskResource { config },
            ehr_import_task_for_internship_offer: EhrImportTaskForInternshipOfferResource {
                config,
            },
            evaluation_task: EvaluationTaskResource { config },
            exam: ExamResource { config },
            exam_marking_task: ExamMarkingTaskResource { config },
            external_interview_assessment: ExternalInterviewAssessmentResource { config },
            external_referral_reward: ExternalReferralRewardResource { config },
            interview_feedback_form: InterviewFeedbackFormResource { config },
            interview_record_attachment: InterviewRecordAttachmentResource { config },
            interview_registration_schema: InterviewRegistrationSchemaResource { config },
            interview_round_type: InterviewRoundTypeResource { config },
            interview_task: InterviewTaskResource { config },
            job_requirement_schema: JobRequirementSchemaResource { config },
            job_schema: JobSchemaResource { config },
            minutes: MinutesResource { config },
            offer_application_form: OfferApplicationFormResource { config },
            offer_approval_template: OfferApprovalTemplateResource { config },
            offer_custom_field: OfferCustomFieldResource { config },
            portal_apply_schema: PortalApplySchemaResource { config },
            referral_website_job_post: ReferralWebsiteJobPostResource { config },
            talent_external_info: TalentExternalInfoResource { config },
            talent_tag: TalentTagResource { config },
            website_channel: WebsiteChannelResource { config },
            website_delivery_task: WebsiteDeliveryTaskResource { config },
            website_site_user: WebsiteSiteUserResource { config },
        }
    }
}
