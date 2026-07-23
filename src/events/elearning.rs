//! Elearning v2 event handlers.

use serde::{Deserialize, Serialize};

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CourseRegistrationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub course_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learner: Option<EventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enroll_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enroll_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning_state: Option<i64>,
    #[serde(default)]
    pub compulsory_lesson_ids: Vec<String>,
    #[serde(default)]
    pub learned_compulsory_lesson_ids: Vec<String>,
    #[serde(default)]
    pub optional_lesson_ids: Vec<String>,
    #[serde(default)]
    pub learned_optional_lesson_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CourseRegistrationDeletedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub course_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learner: Option<EventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CourseRegistrationCreatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<CourseRegistrationData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CourseRegistrationDeletedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<CourseRegistrationDeletedData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CourseRegistrationUpdatedV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<CourseRegistrationData>,
}

event_handlers! {
    on_p2_elearning_course_registration_created_v2 => P2CourseRegistrationCreatedV2
        : "elearning.course_registration.created_v2",
    on_p2_elearning_course_registration_deleted_v2 => P2CourseRegistrationDeletedV2
        : "elearning.course_registration.deleted_v2",
    on_p2_elearning_course_registration_updated_v2 => P2CourseRegistrationUpdatedV2
        : "elearning.course_registration.updated_v2",
}
