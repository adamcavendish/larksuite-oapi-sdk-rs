use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

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
    pub conclusion: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_score: Option<serde_json::Value>,
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<JobData>()
        .await?;
        Ok(GetJobResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<JobListData>()
        .await?;
        Ok(ListJobResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn close(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CloseJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/close");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CloseJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn combined_create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedCreateJobResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/jobs/combined_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CombinedCreateJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn combined_update(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedUpdateJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/combined_update");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CombinedUpdateJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn config(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<ConfigJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/config");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ConfigJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_detail(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<GetDetailJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/get_detail");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetDetailJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn open(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<OpenJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/open");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(OpenJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn recruiter(
        &self,
        job_id: &str,
        option: &RequestOption,
    ) -> Result<RecruiterJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/recruiter");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(RecruiterJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update_config(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateConfigJobResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/update_config");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateConfigJobResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send::<TalentData>()
        .await?;
        Ok(CreateTalentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<TalentData>()
        .await?;
        Ok(GetTalentResp {
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<TalentListData>()
        .await?;
        Ok(ListTalentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn add_to_folder(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddToFolderTalentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/add_to_folder",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(AddToFolderTalentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_get_id(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchGetIdTalentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/batch_get_id",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchGetIdTalentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn combined_create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedCreateTalentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/combined_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CombinedCreateTalentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn combined_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CombinedUpdateTalentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/combined_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CombinedUpdateTalentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn onboard_status(
        &self,
        talent_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<OnboardStatusTalentResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/onboard_status");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(OnboardStatusTalentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_to_folder(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveToFolderTalentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talents/remove_to_folder",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(RemoveToFolderTalentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn tag(
        &self,
        talent_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<TagTalentResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/tag");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(TagTalentResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<ApplicationData>()
        .await?;
        Ok(CreateApplicationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<ApplicationData>()
        .await?;
        Ok(GetApplicationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn terminate(
        &self,
        application_id: &str,
        body: &TerminateApplicationReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/terminate");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn transfer_stage(
        &self,
        application_id: &str,
        body: &TransferStageReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/transfer_stage");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<ApplicationListData>()
        .await?;
        Ok(ListApplicationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn cancel_onboard(
        &self,
        application_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CancelOnboardApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/cancel_onboard");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CancelOnboardApplicationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_detail(
        &self,
        application_id: &str,
        option: &RequestOption,
    ) -> Result<GetDetailApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/get_detail");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetDetailApplicationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn recover(
        &self,
        application_id: &str,
        option: &RequestOption,
    ) -> Result<RecoverApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/recover");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(RecoverApplicationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn transfer_onboard(
        &self,
        application_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<TransferOnboardApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/transfer_onboard");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(TransferOnboardApplicationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn offer(
        &self,
        application_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<OfferApplicationResp, LarkError> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/offer");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(OfferApplicationResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<InterviewListData>()
        .await?;
        Ok(ListInterviewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get_by_talent(
        &self,
        option: &RequestOption,
    ) -> Result<GetByTalentInterviewResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interviews/get_by_talent",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetByTalentInterviewResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/offers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send::<OfferData>()
        .await?;
        Ok(CreateOfferResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        offer_id: &str,
        body: &UpdateOfferReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateOfferResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send::<OfferData>()
        .await?;
        Ok(UpdateOfferResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<GetOfferData>()
        .await?;
        Ok(GetOfferResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn offer_status(
        &self,
        offer_id: &str,
        body: &OfferStatusReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}/offer_status");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<OfferListData>()
        .await?;
        Ok(ListOfferResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn intern_offer_status(
        &self,
        offer_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<InternOfferStatusResp, LarkError> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}/intern_offer_status");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(InternOfferStatusResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/job_requirements",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .query("department_id_type", department_id_type)
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateJobRequirementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_requirement_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobRequirementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/job_requirements/{job_requirement_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteJobRequirementResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .query("department_id_type", department_id_type)
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateJobRequirementResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, raw) = RestRequest::new(
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
        .send::<JobRequirementListData>()
        .await?;
        Ok(ListJobRequirementResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list_by_id(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ListByIdJobRequirementResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/job_requirements/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListByIdJobRequirementResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<AttachmentData>()
        .await?;
        Ok(GetAttachmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn preview(
        &self,
        attachment_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/attachments/{attachment_id}/preview");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateAttachmentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/attachments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateAttachmentResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<OfferSchemaData>()
        .await?;
        Ok(GetOfferSchemaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Helpers for newer resources (use Option<CodeError> pattern) ──

impl_resp_v2!(GetEmployeeResp, serde_json::Value);
impl_resp_v2!(GetByApplicationEmployeeResp, serde_json::Value);
impl_resp_v2!(PatchEmployeeResp, serde_json::Value);
impl_resp_v2!(ListEvaluationResp, serde_json::Value);
impl_resp_v2!(ListNoteResp, serde_json::Value);
impl_resp_v2!(CreateNoteResp, serde_json::Value);
impl_resp_v2!(GetNoteResp, serde_json::Value);
impl_resp_v2!(PatchNoteResp, serde_json::Value);
impl_resp_v2!(DeleteNoteResp, ());
impl_resp_v2!(ListQuestionnaireResp, serde_json::Value);
impl_resp_v2!(GetReferralResp, serde_json::Value);
impl_resp_v2!(ListRegistrationSchemaResp, serde_json::Value);
impl_resp_v2!(ListResumeSourceResp, serde_json::Value);
impl_resp_v2!(ListJobFunctionResp, serde_json::Value);
impl_resp_v2!(ListJobTypeResp, serde_json::Value);
impl_resp_v2!(ListJobProcessResp, serde_json::Value);
impl_resp_v2!(ListLocationResp, serde_json::Value);
impl_resp_v2!(ListRoleResp, serde_json::Value);
impl_resp_v2!(GetRoleResp, serde_json::Value);
impl_resp_v2!(ListSubjectResp, serde_json::Value);
impl_resp_v2!(ListTalentFolderResp, serde_json::Value);
impl_resp_v2!(ListTerminationReasonResp, serde_json::Value);
impl_resp_v2!(ListUserRoleResp, serde_json::Value);
impl_resp_v2!(ListWebsiteResp, serde_json::Value);
impl_resp_v2!(ListWebsiteJobPostResp, serde_json::Value);
impl_resp_v2!(GetWebsiteJobPostResp, serde_json::Value);
impl_resp_v2!(ListInterviewRecordResp, serde_json::Value);
impl_resp_v2!(GetInterviewRecordResp, serde_json::Value);
impl_resp_v2!(ListInterviewerResp, serde_json::Value);
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
impl_resp_v2!(ListTodoResp, serde_json::Value);
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
impl_resp_v2!(SearchWebsiteJobPostResp, serde_json::Value);

// ── Additional response types ──

impl_resp_v2!(OfferApplicationResp, serde_json::Value);
impl_resp_v2!(ListApplicationInterviewResp2, serde_json::Value);
impl_resp_v2!(PatchEhrImportTaskResp, serde_json::Value);
impl_resp_v2!(ListEvaluationTaskResp, serde_json::Value);
impl_resp_v2!(CreateExamResp, serde_json::Value);
impl_resp_v2!(ListExamMarkingTaskResp, serde_json::Value);
impl_resp_v2!(CreateExternalInterviewAssessmentResp, serde_json::Value);
impl_resp_v2!(PatchExternalInterviewAssessmentResp, serde_json::Value);
impl_resp_v2!(CreateExternalReferralRewardResp, serde_json::Value);
impl_resp_v2!(DeleteExternalReferralRewardResp, ());
impl_resp_v2!(ListInterviewFeedbackFormResp, serde_json::Value);
impl_resp_v2!(GetInterviewRecordAttachmentResp, serde_json::Value);
impl_resp_v2!(ListInterviewRegistrationSchemaResp, serde_json::Value);
impl_resp_v2!(ListInterviewRoundTypeResp, serde_json::Value);
impl_resp_v2!(ListInterviewTaskResp, serde_json::Value);
impl_resp_v2!(GetJobManagerResp, serde_json::Value);
impl_resp_v2!(ListJobRequirementSchemaResp, serde_json::Value);
impl_resp_v2!(ListJobSchemaResp, serde_json::Value);
impl_resp_v2!(GetMinutesResp, serde_json::Value);
impl_resp_v2!(GetOfferApplicationFormResp, serde_json::Value);
impl_resp_v2!(ListOfferApplicationFormResp, serde_json::Value);
impl_resp_v2!(ListOfferApprovalTemplateResp, serde_json::Value);
impl_resp_v2!(UpdateOfferCustomFieldResp, serde_json::Value);
impl_resp_v2!(ListPortalApplySchemaResp, serde_json::Value);
impl_resp_v2!(GetReferralWebsiteJobPostResp, serde_json::Value);
impl_resp_v2!(ListReferralWebsiteJobPostResp, serde_json::Value);
impl_resp_v2!(CreateTalentExternalInfoResp, serde_json::Value);
impl_resp_v2!(UpdateTalentExternalInfoResp, serde_json::Value);
impl_resp_v2!(ListTalentTagResp, serde_json::Value);
impl_resp_v2!(CreateWebsiteChannelResp, serde_json::Value);
impl_resp_v2!(DeleteWebsiteChannelResp, ());
impl_resp_v2!(ListWebsiteChannelResp, serde_json::Value);
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        employee_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmployeeResp, LarkError> {
        let path = format!("/open-apis/hire/v1/employees/{employee_id}");
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
        Ok(PatchEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_by_application(
        &self,
        option: &RequestOption,
    ) -> Result<GetByApplicationEmployeeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/employees/get_by_application",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetByApplicationEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Evaluation resource ──

pub struct EvaluationResource<'a> {
    config: &'a Config,
}

impl EvaluationResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListEvaluationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/evaluations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListEvaluationResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/notes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateNoteResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        note_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteNoteResp, LarkError> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteNoteResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        note_id: &str,
        option: &RequestOption,
    ) -> Result<GetNoteResp, LarkError> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetNoteResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListNoteResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/notes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListNoteResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        note_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchNoteResp, LarkError> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
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
        Ok(PatchNoteResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Questionnaire resource ──

pub struct QuestionnaireResource<'a> {
    config: &'a Config,
}

impl QuestionnaireResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListQuestionnaireResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/questionnaires",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListQuestionnaireResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/referrals/get_by_application",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetReferralResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchReferralResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referrals/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchReferralResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/registration_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListRegistrationSchemaResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ResumeSource resource ──

pub struct ResumeSourceResource<'a> {
    config: &'a Config,
}

impl ResumeSourceResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListResumeSourceResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/resume_sources",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListResumeSourceResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Simple list-only resources ──

macro_rules! simple_list_resource {
    ($struct_name:ident, $resp:ident, $path:literal) => {
        pub struct $struct_name<'a> {
            config: &'a Config,
        }
        impl $struct_name<'_> {
            pub async fn list(&self, option: &RequestOption) -> Result<$resp, LarkError> {
                let (api_resp, code_error, data) = RestRequest::new(
                    self.config,
                    http::Method::GET,
                    $path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .send_v2::<serde_json::Value>()
                .await?;
                Ok($resp {
                    api_resp,
                    code_error,
                    data,
                })
            }
        }
    };
}

simple_list_resource!(
    JobFunctionResource,
    ListJobFunctionResp,
    "/open-apis/hire/v1/job_functions"
);
simple_list_resource!(
    JobTypeResource,
    ListJobTypeResp,
    "/open-apis/hire/v1/job_types"
);
simple_list_resource!(
    JobProcessResource,
    ListJobProcessResp,
    "/open-apis/hire/v1/job_processes"
);
pub struct LocationResource<'a> {
    config: &'a Config,
}

impl LocationResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListLocationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/locations",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListLocationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryLocationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/locations/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryLocationResp {
            api_resp,
            code_error,
            data,
        })
    }
}
pub struct RoleResource<'a> {
    config: &'a Config,
}

impl RoleResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListRoleResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/roles",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListRoleResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        role_id: &str,
        option: &RequestOption,
    ) -> Result<GetRoleResp, LarkError> {
        let path = format!("/open-apis/hire/v1/roles/{role_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetRoleResp {
            api_resp,
            code_error,
            data,
        })
    }
}
simple_list_resource!(
    SubjectResource,
    ListSubjectResp,
    "/open-apis/hire/v1/subjects"
);
simple_list_resource!(
    TalentFolderResource,
    ListTalentFolderResp,
    "/open-apis/hire/v1/talent_folders"
);
simple_list_resource!(
    TerminationReasonResource,
    ListTerminationReasonResp,
    "/open-apis/hire/v1/termination_reasons"
);
simple_list_resource!(
    UserRoleResource,
    ListUserRoleResp,
    "/open-apis/hire/v1/user_roles"
);
simple_list_resource!(
    WebsiteResource,
    ListWebsiteResp,
    "/open-apis/hire/v1/websites"
);

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
        let path = format!("/open-apis/hire/v1/websites/{website_id}/job_posts/{job_post_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetWebsiteJobPostResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        website_id: &str,
        option: &RequestOption,
    ) -> Result<ListWebsiteJobPostResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/job_posts");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListWebsiteJobPostResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(
        &self,
        website_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchWebsiteJobPostResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/job_posts/search");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchWebsiteJobPostResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetInterviewRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListInterviewRecordResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListInterviewRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Interviewer resource ──

pub struct InterviewerResource<'a> {
    config: &'a Config,
}

impl InterviewerResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListInterviewerResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interviewers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListInterviewerResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        interviewer_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchInterviewerResp, LarkError> {
        let path = format!("/open-apis/hire/v1/interviewers/{interviewer_id}");
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
        Ok(PatchInterviewerResp {
            api_resp,
            code_error,
            data,
        })
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
                let (api_resp, code_error, data) = RestRequest::new(
                    self.config,
                    http::Method::POST,
                    $base_path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .json_body(&body)?
                .send_v2::<serde_json::Value>()
                .await?;
                Ok($create_resp {
                    api_resp,
                    code_error,
                    data,
                })
            }

            pub async fn update(
                &self,
                $id_param: &str,
                body: serde_json::Value,
                option: &RequestOption,
            ) -> Result<$update_resp, LarkError> {
                let path = format!("{}/{}", $base_path, $id_param);
                let (api_resp, code_error, data) = RestRequest::new(
                    self.config,
                    http::Method::PUT,
                    path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .json_body(&body)?
                .send_v2::<serde_json::Value>()
                .await?;
                Ok($update_resp {
                    api_resp,
                    code_error,
                    data,
                })
            }

            pub async fn delete(
                &self,
                $id_param: &str,
                option: &RequestOption,
            ) -> Result<$delete_resp, LarkError> {
                let path = format!("{}/{}", $base_path, $id_param);
                let (api_resp, code_error, data) = RestRequest::new(
                    self.config,
                    http::Method::DELETE,
                    path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .send_v2::<()>()
                .await?;
                Ok($delete_resp {
                    api_resp,
                    code_error,
                    data,
                })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/external_applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListExternalApplicationResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_offers/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchQueryExternalOfferResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_interviews/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchQueryExternalInterviewResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_background_checks/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchQueryExternalBackgroundCheckResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Todo resource ──

pub struct TodoResource<'a> {
    config: &'a Config,
}

impl TodoResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListTodoResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/todos",
            vec![AccessTokenType::User],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListTodoResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/tripartite_agreements",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateTripartiteAgreementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        tripartite_agreement_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteTripartiteAgreementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/tripartite_agreements/{tripartite_agreement_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteTripartiteAgreementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        option: &RequestOption,
    ) -> Result<ListTripartiteAgreementResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/tripartite_agreements",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListTripartiteAgreementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        tripartite_agreement_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateTripartiteAgreementResp, LarkError> {
        let path = format!("/open-apis/hire/v1/tripartite_agreements/{tripartite_agreement_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateTripartiteAgreementResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PublishAdvertisementResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchQueryAgencyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_agency_account(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<GetAgencyAccountResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/get_agency_account",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetAgencyAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn operate_agency_account(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<OperateAgencyAccountResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/operate_agency_account",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(OperateAgencyAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn protect(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ProtectAgencyResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/protect",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ProtectAgencyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn protect_search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ProtectSearchAgencyResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/agencies/protection_period/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ProtectSearchAgencyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryAgencyResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/agencies/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryAgencyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        agency_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAgencyResp, LarkError> {
        let path = format!("/open-apis/hire/v1/agencies/{agency_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetAgencyResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/background_check_orders/batch_query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchQueryBackgroundCheckOrderResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        option: &RequestOption,
    ) -> Result<ListBackgroundCheckOrderResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/background_check_orders",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListBackgroundCheckOrderResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/applications/diversity_inclusions/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchDiversityInclusionResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_account_custom_fields",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateEcoAccountCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoAccountCustomFieldResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_account_custom_fields/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchDeleteEcoAccountCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoAccountCustomFieldResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_account_custom_fields/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchUpdateEcoAccountCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/cancel",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CancelEcoBackgroundCheckResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update_progress(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateProgressEcoBackgroundCheckResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/update_progress",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateProgressEcoBackgroundCheckResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update_result(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateResultEcoBackgroundCheckResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/update_result",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateResultEcoBackgroundCheckResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_custom_fields",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateEcoBackgroundCheckCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoBackgroundCheckCustomFieldResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchDeleteEcoBackgroundCheckCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoBackgroundCheckCustomFieldResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchUpdateEcoBackgroundCheckCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_packages",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateEcoBackgroundCheckPackageResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoBackgroundCheckPackageResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_packages/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchDeleteEcoBackgroundCheckPackageResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoBackgroundCheckPackageResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_background_check_packages/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchUpdateEcoBackgroundCheckPackageResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(LoginInfoEcoExamResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update_result(
        &self,
        exam_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateResultEcoExamResp, LarkError> {
        let path = format!("/open-apis/hire/v1/eco_exams/{exam_id}/update_result");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateResultEcoExamResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_exam_papers",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateEcoExamPaperResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteEcoExamPaperResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/eco_exam_papers/batch_delete",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchDeleteEcoExamPaperResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateEcoExamPaperResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_exam_papers/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchUpdateEcoExamPaperResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchUpdateJobManagerResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        job_id: &str,
        manager_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetJobManagerResp, LarkError> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/managers/{manager_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetJobManagerResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/job_publish_records/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchJobPublishRecordResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referral_account",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateReferralAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn deactivate(
        &self,
        referral_account_id: &str,
        option: &RequestOption,
    ) -> Result<DeactivateReferralAccountResp, LarkError> {
        let path = format!("/open-apis/hire/v1/referral_account/{referral_account_id}/deactivate");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(DeactivateReferralAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn enable(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EnableReferralAccountResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referral_account/enable",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(EnableReferralAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_account_assets(
        &self,
        option: &RequestOption,
    ) -> Result<GetAccountAssetsReferralAccountResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/referral_account/get_account_assets",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetAccountAssetsReferralAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn reconciliation(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ReconciliationReferralAccountResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/referral_account/reconciliation",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ReconciliationReferralAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn withdraw(
        &self,
        referral_account_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WithdrawReferralAccountResp, LarkError> {
        let path = format!("/open-apis/hire/v1/referral_account/{referral_account_id}/withdraw");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(WithdrawReferralAccountResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talent_blocklist/change_talent_block",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ChangeTalentBlockResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── TalentObject resource ──

pub struct TalentObjectResource<'a> {
    config: &'a Config,
}

impl TalentObjectResource<'_> {
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTalentObjectResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talent_objects/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryTalentObjectResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/talent_operation_logs/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchTalentOperationLogResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchChangeTalentPoolResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn move_talent(
        &self,
        talent_pool_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<MoveTalentTalentPoolResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talent_pools/{talent_pool_id}/talent_relationship");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(MoveTalentTalentPoolResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(&self, option: &RequestOption) -> Result<SearchTalentPoolResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talent_pools",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchTalentPoolResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/tests/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(SearchTestResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateByAttachmentWebsiteDeliveryResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn create_by_resume(
        &self,
        website_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateByResumeWebsiteDeliveryResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_resume");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateByResumeWebsiteDeliveryResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .query("job_level_id_type", query.job_level_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListApplicationInterviewResp2 {
            api_resp,
            code_error,
            data,
        })
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
        Ok(PatchEhrImportTaskResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
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
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListEvaluationTaskResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/exams",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateExamResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
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
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListExamMarkingTaskResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_interview_assessments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateExternalInterviewAssessmentResp {
            api_resp,
            code_error,
            data,
        })
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
        Ok(PatchExternalInterviewAssessmentResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/hire/v1/external_referral_rewards",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateExternalReferralRewardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        external_referral_reward_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteExternalReferralRewardResp, LarkError> {
        let path =
            format!("/open-apis/hire/v1/external_referral_rewards/{external_referral_reward_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteExternalReferralRewardResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── InterviewFeedbackForm resource ──

pub struct InterviewFeedbackFormResource<'a> {
    config: &'a Config,
}

impl InterviewFeedbackFormResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewFeedbackFormResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_feedback_forms",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListInterviewFeedbackFormResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_records/attachments",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetInterviewRecordAttachmentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── InterviewRegistrationSchema resource ──

pub struct InterviewRegistrationSchemaResource<'a> {
    config: &'a Config,
}

impl InterviewRegistrationSchemaResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewRegistrationSchemaResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_registration_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListInterviewRegistrationSchemaResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── InterviewRoundType resource ──

pub struct InterviewRoundTypeResource<'a> {
    config: &'a Config,
}

impl InterviewRoundTypeResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewRoundTypeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/interview_round_types",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListInterviewRoundTypeResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
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
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListInterviewTaskResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_requirement_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListJobRequirementSchemaResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/job_schemas",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListJobSchemaResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Minutes resource ──

pub struct MinutesResource<'a> {
    config: &'a Config,
}

impl MinutesResource<'_> {
    pub async fn get(&self, option: &RequestOption) -> Result<GetMinutesResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/minutes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetMinutesResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetOfferApplicationFormResp {
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
    ) -> Result<ListOfferApplicationFormResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/offer_application_forms",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListOfferApplicationFormResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/offer_approval_templates",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListOfferApprovalTemplateResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateOfferCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/portal_apply_schemas",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListPortalApplySchemaResp {
            api_resp,
            code_error,
            data,
        })
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
        let path = format!("/open-apis/hire/v1/referral_websites/job_posts/{job_post_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetReferralWebsiteJobPostResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListReferralWebsiteJobPostResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/referral_websites/job_posts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListReferralWebsiteJobPostResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateTalentExternalInfoResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        talent_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateTalentExternalInfoResp, LarkError> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/external_info");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateTalentExternalInfoResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v1/talent_tags",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListTalentTagResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateWebsiteChannelResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        website_id: &str,
        channel_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteWebsiteChannelResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels/{channel_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteWebsiteChannelResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        website_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWebsiteChannelResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListWebsiteChannelResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        website_id: &str,
        channel_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateWebsiteChannelResp, LarkError> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels/{channel_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateWebsiteChannelResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetWebsiteDeliveryTaskResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateWebsiteSiteUserResp {
            api_resp,
            code_error,
            data,
        })
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
