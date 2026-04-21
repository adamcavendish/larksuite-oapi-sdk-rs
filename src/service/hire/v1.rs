use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyResp, parse_v2};
use crate::transport;

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

impl<'a> JobResource<'a> {
    pub async fn get(
        &self,
        job_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<JobData>(self.config, &api_req, option).await?;
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
    ) -> Result<ListJobResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/jobs");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = update_start_time {
            api_req.query_params.set("update_start_time", v);
        }
        if let Some(v) = update_end_time {
            api_req.query_params.set("update_end_time", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<JobListData>(self.config, &api_req, option).await?;
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
    ) -> Result<CloseJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/close");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CombinedCreateJobResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/jobs/combined_create",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CombinedUpdateJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/combined_update");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CombinedUpdateJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn config(&self, job_id: &str, option: &RequestOption) -> Result<ConfigJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/config");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetDetailJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/get_detail");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<OpenJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/open");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<RecruiterJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/recruiter");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateConfigJobResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/update_config");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl<'a> TalentResource<'a> {
    pub async fn create(
        &self,
        body: &CreateTalentReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTalentResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/talents");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TalentData>(self.config, &api_req, option).await?;
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
    ) -> Result<GetTalentResp> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TalentData>(self.config, &api_req, option).await?;
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
    ) -> Result<ListTalentResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/talents");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TalentListData>(self.config, &api_req, option).await?;
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
    ) -> Result<AddToFolderTalentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/talents/add_to_folder",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchGetIdTalentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/talents/batch_get_id",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CombinedCreateTalentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/talents/combined_create",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CombinedUpdateTalentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/talents/combined_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<OnboardStatusTalentResp> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/onboard_status");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<RemoveToFolderTalentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/talents/remove_to_folder",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<TagTalentResp> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/tag");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl<'a> ApplicationResource<'a> {
    pub async fn create(
        &self,
        body: &CreateApplicationReqBody,
        option: &RequestOption,
    ) -> Result<CreateApplicationResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/applications");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ApplicationData>(self.config, &api_req, option).await?;
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
    ) -> Result<GetApplicationResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ApplicationData>(self.config, &api_req, option).await?;
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
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/terminate");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/transfer_stage");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<ListApplicationResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/applications");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = job_id {
            api_req.query_params.set("job_id", v);
        }
        if let Some(v) = stage_id {
            api_req.query_params.set("stage_id", v);
        }
        if let Some(v) = talent_id {
            api_req.query_params.set("talent_id", v);
        }
        if let Some(v) = active_status {
            api_req.query_params.set("active_status", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ApplicationListData>(self.config, &api_req, option).await?;
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
    ) -> Result<CancelOnboardApplicationResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/cancel_onboard");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetDetailApplicationResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/get_detail");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<RecoverApplicationResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/recover");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<TransferOnboardApplicationResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/transfer_onboard");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<OfferApplicationResp> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/offer");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListInterviewResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/interviews");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = application_id {
            api_req.query_params.set("application_id", v);
        }
        if let Some(v) = interview_id {
            api_req.query_params.set("interview_id", v);
        }
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<InterviewListData>(self.config, &api_req, option).await?;
        Ok(ListInterviewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get_by_talent(&self, option: &RequestOption) -> Result<GetByTalentInterviewResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/interviews/get_by_talent",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl<'a> OfferResource<'a> {
    pub async fn create(
        &self,
        body: &CreateOfferReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateOfferResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/offers");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<OfferData>(self.config, &api_req, option).await?;
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
    ) -> Result<UpdateOfferResp> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<OfferData>(self.config, &api_req, option).await?;
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
    ) -> Result<GetOfferResp> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GetOfferData>(self.config, &api_req, option).await?;
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
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}/offer_status");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<ListOfferResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/offers");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = application_id {
            api_req.query_params.set("application_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<OfferListData>(self.config, &api_req, option).await?;
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
    ) -> Result<InternOfferStatusResp> {
        let path = format!("/open-apis/hire/v1/offers/{offer_id}/intern_offer_status");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl<'a> JobRequirementResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateJobRequirementResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/job_requirements");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<DeleteJobRequirementResp> {
        let path = format!("/open-apis/hire/v1/job_requirements/{job_requirement_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateJobRequirementResp> {
        let path = format!("/open-apis/hire/v1/job_requirements/{job_requirement_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListJobRequirementResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/job_requirements");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = job_id {
            api_req.query_params.set("job_id", v);
        }
        if let Some(v) = create_time_begin {
            api_req.query_params.set("create_time_begin", v);
        }
        if let Some(v) = create_time_end {
            api_req.query_params.set("create_time_end", v);
        }
        if let Some(v) = update_time_begin {
            api_req.query_params.set("update_time_begin", v);
        }
        if let Some(v) = update_time_end {
            api_req.query_params.set("update_time_end", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<JobRequirementListData>(self.config, &api_req, option)
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
    ) -> Result<ListByIdJobRequirementResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/job_requirements/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetAttachmentResp> {
        let path = format!("/open-apis/hire/v1/attachments/{attachment_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<AttachmentData>(self.config, &api_req, option).await?;
        Ok(GetAttachmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn preview(&self, attachment_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/hire/v1/attachments/{attachment_id}/preview");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateAttachmentResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/attachments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetOfferSchemaResp> {
        let path = format!("/open-apis/hire/v1/offer_schemas/{offer_schema_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<OfferSchemaData>(self.config, &api_req, option).await?;
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

// ── New response types (Phase 10) ──

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
    pub async fn get(&self, employee_id: &str, option: &RequestOption) -> Result<GetEmployeeResp> {
        let path = format!("/open-apis/hire/v1/employees/{employee_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<PatchEmployeeResp> {
        let path = format!("/open-apis/hire/v1/employees/{employee_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_by_application(
        &self,
        option: &RequestOption,
    ) -> Result<GetByApplicationEmployeeResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/employees/get_by_application",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListEvaluationResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/evaluations");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateNoteResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/notes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateNoteResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(&self, note_id: &str, option: &RequestOption) -> Result<DeleteNoteResp> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteNoteResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, note_id: &str, option: &RequestOption) -> Result<GetNoteResp> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetNoteResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListNoteResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/notes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<PatchNoteResp> {
        let path = format!("/open-apis/hire/v1/notes/{note_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListQuestionnaireResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/questionnaires");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn get_by_application(&self, option: &RequestOption) -> Result<GetReferralResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/referrals/get_by_application",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<SearchReferralResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/referrals/search");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListRegistrationSchemaResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/registration_schemas");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListResumeSourceResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/resume_sources");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
            pub async fn list(&self, option: &RequestOption) -> Result<$resp> {
                let mut api_req = ApiReq::new(http::Method::GET, $path);
                api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
                let (api_resp, raw) =
                    transport::request_typed::<serde_json::Value>(self.config, &api_req, option)
                        .await?;
                let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListLocationResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/locations");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<QueryLocationResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/locations/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListRoleResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/roles");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListRoleResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, role_id: &str, option: &RequestOption) -> Result<GetRoleResp> {
        let path = format!("/open-apis/hire/v1/roles/{role_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetWebsiteJobPostResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/job_posts/{job_post_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListWebsiteJobPostResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/job_posts");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<SearchWebsiteJobPostResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/job_posts/search");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetInterviewRecordResp> {
        let path = format!("/open-apis/hire/v1/interview_records/{interview_record_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetInterviewRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListInterviewRecordResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/interview_records");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListInterviewerResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/interviewers");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<PatchInterviewerResp> {
        let path = format!("/open-apis/hire/v1/interviewers/{interviewer_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
            ) -> Result<$create_resp> {
                let mut api_req = ApiReq::new(http::Method::POST, $base_path);
                api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
                api_req.body = Some(ReqBody::json(&body)?);
                let (api_resp, raw) =
                    transport::request_typed::<serde_json::Value>(self.config, &api_req, option)
                        .await?;
                let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
            ) -> Result<$update_resp> {
                let path = format!("{}/{}", $base_path, $id_param);
                let mut api_req = ApiReq::new(http::Method::PUT, &path);
                api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
                api_req.body = Some(ReqBody::json(&body)?);
                let (api_resp, raw) =
                    transport::request_typed::<serde_json::Value>(self.config, &api_req, option)
                        .await?;
                let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
            ) -> Result<$delete_resp> {
                let path = format!("{}/{}", $base_path, $id_param);
                let mut api_req = ApiReq::new(http::Method::DELETE, &path);
                api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
                let (api_resp, raw) =
                    transport::request_typed::<()>(self.config, &api_req, option).await?;
                let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListExternalApplicationResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/external_applications",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchQueryExternalOfferResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/external_offers/batch_query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchQueryExternalInterviewResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/external_interviews/batch_query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchQueryExternalBackgroundCheckResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/external_background_checks/batch_query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn list(&self, option: &RequestOption) -> Result<ListTodoResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/todos");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateTripartiteAgreementResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/tripartite_agreements",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<DeleteTripartiteAgreementResp> {
        let path = format!("/open-apis/hire/v1/tripartite_agreements/{tripartite_agreement_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteTripartiteAgreementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListTripartiteAgreementResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/tripartite_agreements",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateTripartiteAgreementResp> {
        let path = format!("/open-apis/hire/v1/tripartite_agreements/{tripartite_agreement_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<PublishAdvertisementResp> {
        let path = format!("/open-apis/hire/v1/advertisements/{advertisement_id}/publish");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchQueryAgencyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/agencies/batch_query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetAgencyAccountResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/agencies/get_agency_account",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<OperateAgencyAccountResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/agencies/operate_agency_account",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ProtectAgencyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/agencies/protect");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ProtectSearchAgencyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/agencies/protection_period/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ProtectSearchAgencyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryAgencyResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/agencies/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetAgencyResp> {
        let path = format!("/open-apis/hire/v1/agencies/{agency_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchQueryBackgroundCheckOrderResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/background_check_orders/batch_query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchQueryBackgroundCheckOrderResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListBackgroundCheckOrderResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/background_check_orders",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<SearchDiversityInclusionResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/applications/diversity_inclusions/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateEcoAccountCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_account_custom_fields",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchDeleteEcoAccountCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_account_custom_fields/batch_delete",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchUpdateEcoAccountCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_account_custom_fields/batch_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CancelEcoBackgroundCheckResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/cancel",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateProgressEcoBackgroundCheckResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/update_progress",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateResultEcoBackgroundCheckResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_checks/update_result",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateEcoBackgroundCheckCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_custom_fields",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchDeleteEcoBackgroundCheckCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchUpdateEcoBackgroundCheckCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateEcoBackgroundCheckPackageResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_packages",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchDeleteEcoBackgroundCheckPackageResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_background_check_packages/batch_delete",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchUpdateEcoBackgroundCheckPackageResp> {
        let mut api_req = ApiReq::new(
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_background_check_packages/batch_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<LoginInfoEcoExamResp> {
        let path = format!("/open-apis/hire/v1/eco_exams/{exam_id}/login_info");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateResultEcoExamResp> {
        let path = format!("/open-apis/hire/v1/eco_exams/{exam_id}/update_result");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateEcoExamPaperResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/eco_exam_papers");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchDeleteEcoExamPaperResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/eco_exam_papers/batch_delete",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchUpdateEcoExamPaperResp> {
        let mut api_req = ApiReq::new(
            http::Method::PATCH,
            "/open-apis/hire/v1/eco_exam_papers/batch_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchUpdateJobManagerResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/managers/batch_update");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetJobManagerResp> {
        let path = format!("/open-apis/hire/v1/jobs/{job_id}/managers/{manager_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<SearchJobPublishRecordResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/job_publish_records/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateReferralAccountResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/referral_account");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<DeactivateReferralAccountResp> {
        let path = format!("/open-apis/hire/v1/referral_account/{referral_account_id}/deactivate");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<EnableReferralAccountResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/referral_account/enable",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(EnableReferralAccountResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_account_assets(
        &self,
        option: &RequestOption,
    ) -> Result<GetAccountAssetsReferralAccountResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/referral_account/get_account_assets",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ReconciliationReferralAccountResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/referral_account/reconciliation",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<WithdrawReferralAccountResp> {
        let path = format!("/open-apis/hire/v1/referral_account/{referral_account_id}/withdraw");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ChangeTalentBlockResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/talent_blocklist/change_talent_block",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTalentObjectResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/talent_objects/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<SearchTalentOperationLogResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/talent_operation_logs/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<BatchChangeTalentPoolResp> {
        let path =
            format!("/open-apis/hire/v1/talent_pools/{talent_pool_id}/batch_change_talent_pool");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<MoveTalentTalentPoolResp> {
        let path = format!("/open-apis/hire/v1/talent_pools/{talent_pool_id}/talent_relationship");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(MoveTalentTalentPoolResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(&self, option: &RequestOption) -> Result<SearchTalentPoolResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/talent_pools");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<SearchTestResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/tests/search");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateByAttachmentWebsiteDeliveryResp> {
        let path =
            format!("/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_attachment");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateByResumeWebsiteDeliveryResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_resume");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl ApplicationInterviewResource<'_> {
    pub async fn list(
        &self,
        application_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListApplicationInterviewResp2> {
        let path = format!("/open-apis/hire/v1/applications/{application_id}/interviews");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<PatchEhrImportTaskResp> {
        let path = format!("/open-apis/hire/v1/ehr_import_tasks/{ehr_import_task_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl EvaluationTaskResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEvaluationTaskResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/evaluation_tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateExamResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/hire/v1/exams");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl ExamMarkingTaskResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListExamMarkingTaskResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/exam_marking_tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateExternalInterviewAssessmentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/external_interview_assessments",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<PatchExternalInterviewAssessmentResp> {
        let path = format!(
            "/open-apis/hire/v1/external_interview_assessments/{external_interview_assessment_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateExternalReferralRewardResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/hire/v1/external_referral_rewards",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<DeleteExternalReferralRewardResp> {
        let path =
            format!("/open-apis/hire/v1/external_referral_rewards/{external_referral_reward_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListInterviewFeedbackFormResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/interview_feedback_forms",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn get(&self, option: &RequestOption) -> Result<GetInterviewRecordAttachmentResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/interview_records/attachments",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListInterviewRegistrationSchemaResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/interview_registration_schemas",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListInterviewRoundTypeResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/interview_round_types",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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

impl InterviewTaskResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewTaskResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/interview_tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListJobRequirementSchemaResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/job_requirement_schemas",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListJobSchemaResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/job_schemas");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    pub async fn get(&self, option: &RequestOption) -> Result<GetMinutesResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/minutes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetOfferApplicationFormResp> {
        let path =
            format!("/open-apis/hire/v1/offer_application_forms/{offer_application_form_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListOfferApplicationFormResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/offer_application_forms",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListOfferApprovalTemplateResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/offer_approval_templates",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateOfferCustomFieldResp> {
        let path = format!("/open-apis/hire/v1/offer_custom_fields/{offer_custom_field_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListPortalApplySchemaResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/portal_apply_schemas");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetReferralWebsiteJobPostResp> {
        let path = format!("/open-apis/hire/v1/referral_websites/job_posts/{job_post_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListReferralWebsiteJobPostResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/hire/v1/referral_websites/job_posts",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateTalentExternalInfoResp> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/external_info");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateTalentExternalInfoResp> {
        let path = format!("/open-apis/hire/v1/talents/{talent_id}/external_info");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListTalentTagResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/talent_tags");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateWebsiteChannelResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<DeleteWebsiteChannelResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels/{channel_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListWebsiteChannelResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<UpdateWebsiteChannelResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/channels/{channel_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<GetWebsiteDeliveryTaskResp> {
        let path =
            format!("/open-apis/hire/v1/websites/{website_id}/delivery_tasks/{delivery_task_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<CreateWebsiteSiteUserResp> {
        let path = format!("/open-apis/hire/v1/websites/{website_id}/site_users");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
