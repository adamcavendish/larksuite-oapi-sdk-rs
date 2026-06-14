//! Hire v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcoAccountCustomFieldEventData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcoBackgroundCheckCreateEventMobile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcoBackgroundCheckCreateEventCandidateInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<EcoBackgroundCheckCreateEventMobile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcoBackgroundCheckCreateEventContactInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<EcoBackgroundCheckCreateEventMobile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcoBackgroundCheckCreateEventCustomKv {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcoExamCreateEventMobile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcoExamCreateEventCandidateInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<EcoExamCreateEventMobile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cash {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BonusAmount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_bonus: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<Cash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_bonus: Option<Vec<Cash>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Assets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed_bonus: Option<BonusAmount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_bonus: Option<BonusAmount>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationStageInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireOfferStatusChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireApplicationStageChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEhrImportTaskImportedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ehr_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ehr_requirement_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ehr_department: Option<DepartmentId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireJobCreatedV1 {
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub job: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireJobUpdatedV1 {
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub job: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub updated_fields: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireCandidateCreatedV1 {
    #[serde(default)]
    pub candidate_id: String,
    #[serde(default)]
    pub candidate: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireInterviewCreatedV1 {
    #[serde(default)]
    pub interview_id: String,
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub interview: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireInterviewUpdatedV1 {
    #[serde(default)]
    pub interview_id: String,
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub interview: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub updated_fields: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireApplicationDeletedV1 {
    #[serde(default)]
    pub application_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoAccountCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(default)]
    pub usage_list: Vec<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_field_list: Option<Vec<EcoAccountCustomFieldEventData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoBackgroundCheckCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(default)]
    pub additional_item_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate_info: Option<EcoBackgroundCheckCreateEventCandidateInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_contact_info: Option<EcoBackgroundCheckCreateEventContactInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_field_list: Option<Vec<EcoBackgroundCheckCreateEventCustomKv>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoBackgroundCheckCanceledV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_check_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoExamCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exam_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paper_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate_info: Option<EcoExamCreateEventCandidateInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEhrImportTaskForInternshipOfferImportedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_onboard_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ehr_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ehr_department: Option<DepartmentId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireReferralAccountAssetsUpdateV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Assets>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireTalentDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireTalentTagSubscriptionV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<TalentTag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_stage: Option<ApplicationStageInfo>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_hire_offer_status_changed_v1 => P2HireOfferStatusChangedV1
        : "hire.offer.status_changed_v1",
    on_p2_hire_application_stage_changed_v1 => P2HireApplicationStageChangedV1
        : "hire.application.stage_changed_v1",
    on_p2_hire_application_deleted_v1 => P2HireApplicationDeletedV1
        : "hire.application.deleted_v1",
    on_p2_hire_ehr_import_task_imported_v1 => P2HireEhrImportTaskImportedV1
        : "hire.ehr_import_task.imported_v1",
    on_p2_hire_ehr_import_task_for_internship_offer_imported_v1 => P2HireEhrImportTaskForInternshipOfferImportedV1
        : "hire.ehr_import_task_for_internship_offer.imported_v1",
    on_p2_hire_job_created_v1 => P2HireJobCreatedV1
        : "hire.job.created_v1",
    on_p2_hire_job_updated_v1 => P2HireJobUpdatedV1
        : "hire.job.updated_v1",
    on_p2_hire_candidate_created_v1 => P2HireCandidateCreatedV1
        : "hire.talent.created_v1",
    on_p2_hire_talent_deleted_v1 => P2HireTalentDeletedV1
        : "hire.talent.deleted_v1",
    on_p2_hire_talent_tag_subscription_v1 => P2HireTalentTagSubscriptionV1
        : "hire.talent.tag_subscription_v1",
    on_p2_hire_interview_created_v1 => P2HireInterviewCreatedV1
        : "hire.interview.created_v1",
    on_p2_hire_interview_updated_v1 => P2HireInterviewUpdatedV1
        : "hire.interview.updated_v1",
    on_p2_hire_eco_account_created_v1 => P2HireEcoAccountCreatedV1
        : "hire.eco_account.created_v1",
    on_p2_hire_eco_background_check_created_v1 => P2HireEcoBackgroundCheckCreatedV1
        : "hire.eco_background_check.created_v1",
    on_p2_hire_eco_background_check_canceled_v1 => P2HireEcoBackgroundCheckCanceledV1
        : "hire.eco_background_check.canceled_v1",
    on_p2_hire_eco_exam_created_v1 => P2HireEcoExamCreatedV1
        : "hire.eco_exam.created_v1",
    on_p2_hire_referral_account_assets_update_v1 => P2HireReferralAccountAssetsUpdateV1
        : "hire.referral_account.assets_update_v1",
}
