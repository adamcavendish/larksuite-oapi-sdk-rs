use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
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

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

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
}

pub struct JobRequirementResource<'a> {
    config: &'a Config,
}

impl<'a> JobRequirementResource<'a> {
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

fn parse_v2<T: for<'de> serde::Deserialize<'de>>(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<T>,
) -> impl FnOnce() -> (ApiResp, Option<CodeError>, Option<T>) {
    move || {
        let code_error = if raw.code_error.code != 0 {
            Some(raw.code_error)
        } else {
            None
        };
        (api_resp, code_error, raw.data)
    }
}

macro_rules! impl_resp_v2 {
    ($name:ident, $data:ty) => {
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.api_resp.status_code == 200
                    && self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}

impl_resp_v2!(GetEmployeeResp, serde_json::Value);
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
impl_resp_v2!(CreateExternalApplicationResp, serde_json::Value);
impl_resp_v2!(UpdateExternalApplicationResp, serde_json::Value);
impl_resp_v2!(DeleteExternalApplicationResp, ());
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchEmployeeResp {
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetReferralResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── RegistrationSchema resource ──

pub struct RegistrationSchemaResource<'a> {
    config: &'a Config,
}

impl RegistrationSchemaResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListRegistrationSchemaResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/registration_schemas");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
                let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
simple_list_resource!(
    LocationResource,
    ListLocationResp,
    "/open-apis/hire/v1/locations"
);
simple_list_resource!(RoleResource, ListRoleResp, "/open-apis/hire/v1/roles");
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListWebsiteJobPostResp {
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListInterviewerResp {
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
                let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
                let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
                let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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

external_crud_resource!(
    ExternalOfferResource,
    "/open-apis/hire/v1/external_offers",
    CreateExternalOfferResp,
    UpdateExternalOfferResp,
    DeleteExternalOfferResp,
    external_offer_id
);

external_crud_resource!(
    ExternalInterviewResource,
    "/open-apis/hire/v1/external_interviews",
    CreateExternalInterviewResp,
    UpdateExternalInterviewResp,
    DeleteExternalInterviewResp,
    external_interview_id
);

external_crud_resource!(
    ExternalBackgroundCheckResource,
    "/open-apis/hire/v1/external_background_checks",
    CreateExternalBackgroundCheckResp,
    UpdateExternalBackgroundCheckResp,
    DeleteExternalBackgroundCheckResp,
    external_background_check_id
);

// ── Todo resource ──

pub struct TodoResource<'a> {
    config: &'a Config,
}

impl TodoResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListTodoResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v1/todos");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
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
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(UpdateTripartiteAgreementResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

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
        }
    }
}
