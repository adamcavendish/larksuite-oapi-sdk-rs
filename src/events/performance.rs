//! Performance v2 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StageChange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_stage_role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewDataChange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(default)]
    pub stage_changes: Vec<StageChange>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ReviewDataChangedV2 {
    #[serde(default)]
    pub items: Vec<ReviewDataChange>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2StageTaskOpenResultV2 {
    #[serde(default)]
    pub items: Vec<OpenResult>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_performance_review_data_changed_v2 => P2ReviewDataChangedV2
        : "performance.review_data.changed_v2",
    on_p2_performance_stage_task_open_result_v2 => P2StageTaskOpenResultV2
        : "performance.stage_task.open_result_v2",
}
