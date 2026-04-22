use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireOfferStatusChangedV1 {
    #[serde(default)]
    pub offer_id: String,
    #[serde(default)]
    pub offer_type: i32,
    #[serde(default)]
    pub schema_id: String,
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub offer_status: i32,
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub talent_id: String,
    #[serde(default)]
    pub creator_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireApplicationStageChangedV1 {
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub stage_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEhrImportTaskImportedV1 {
    #[serde(default)]
    pub task_id: String,
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
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
    pub application_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoAccountCreatedV1 {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub account: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoBackgroundCheckCreatedV1 {
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub background_check: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoBackgroundCheckCanceledV1 {
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub background_check: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEcoExamCreatedV1 {
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub exam: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEhrImportTaskForInternshipOfferImportedV1 {
    #[serde(default)]
    pub task_id: String,
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireReferralAccountAssetsUpdateV1 {
    #[serde(default)]
    pub account: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireTalentDeletedV1 {
    #[serde(default)]
    pub talent_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireTalentTagSubscriptionV1 {
    #[serde(default)]
    pub talent_id: String,
    #[serde(default)]
    pub tag: serde_json::Value,
}

// ── Handler registration helpers ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync + 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => {
                Box::pin(handler(typed)) as Pin<Box<dyn Future<Output = Result<()>> + Send>>
            }
            Err(e) => Box::pin(async move {
                Err(Error::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods (all 17 hire/v1 handlers) ──

macro_rules! hire_v1_handler {
    ($method:ident, $event_key:literal, $payload_type:ty) => {
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload_type) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send + 'static,
        {
            self.on_event($event_key, wrap_handler(handler))
        }
    };
}

impl EventDispatcher {
    hire_v1_handler!(
        on_p2_hire_offer_status_changed_v1,
        "hire.offer.status_changed_v1",
        P2HireOfferStatusChangedV1
    );
    hire_v1_handler!(
        on_p2_hire_application_stage_changed_v1,
        "hire.application.stage_changed_v1",
        P2HireApplicationStageChangedV1
    );
    hire_v1_handler!(
        on_p2_hire_application_deleted_v1,
        "hire.application.deleted_v1",
        P2HireApplicationDeletedV1
    );
    hire_v1_handler!(
        on_p2_hire_ehr_import_task_imported_v1,
        "hire.ehr_import_task.imported_v1",
        P2HireEhrImportTaskImportedV1
    );
    hire_v1_handler!(
        on_p2_hire_ehr_import_task_for_internship_offer_imported_v1,
        "hire.ehr_import_task_for_internship_offer.imported_v1",
        P2HireEhrImportTaskForInternshipOfferImportedV1
    );
    hire_v1_handler!(
        on_p2_hire_job_created_v1,
        "hire.job.created_v1",
        P2HireJobCreatedV1
    );
    hire_v1_handler!(
        on_p2_hire_job_updated_v1,
        "hire.job.updated_v1",
        P2HireJobUpdatedV1
    );
    hire_v1_handler!(
        on_p2_hire_candidate_created_v1,
        "hire.talent.created_v1",
        P2HireCandidateCreatedV1
    );
    hire_v1_handler!(
        on_p2_hire_talent_deleted_v1,
        "hire.talent.deleted_v1",
        P2HireTalentDeletedV1
    );
    hire_v1_handler!(
        on_p2_hire_talent_tag_subscription_v1,
        "hire.talent.tag_subscription_v1",
        P2HireTalentTagSubscriptionV1
    );
    hire_v1_handler!(
        on_p2_hire_interview_created_v1,
        "hire.interview.created_v1",
        P2HireInterviewCreatedV1
    );
    hire_v1_handler!(
        on_p2_hire_interview_updated_v1,
        "hire.interview.updated_v1",
        P2HireInterviewUpdatedV1
    );
    hire_v1_handler!(
        on_p2_hire_eco_account_created_v1,
        "hire.eco_account.created_v1",
        P2HireEcoAccountCreatedV1
    );
    hire_v1_handler!(
        on_p2_hire_eco_background_check_created_v1,
        "hire.eco_background_check.created_v1",
        P2HireEcoBackgroundCheckCreatedV1
    );
    hire_v1_handler!(
        on_p2_hire_eco_background_check_canceled_v1,
        "hire.eco_background_check.canceled_v1",
        P2HireEcoBackgroundCheckCanceledV1
    );
    hire_v1_handler!(
        on_p2_hire_eco_exam_created_v1,
        "hire.eco_exam.created_v1",
        P2HireEcoExamCreatedV1
    );
    hire_v1_handler!(
        on_p2_hire_referral_account_assets_update_v1,
        "hire.referral_account.assets_update_v1",
        P2HireReferralAccountAssetsUpdateV1
    );
}
