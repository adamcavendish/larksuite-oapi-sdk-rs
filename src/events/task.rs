//! Task v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Shared sub-types ──

pub use crate::events::common::{UserId, UserIdList};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2TaskUpdateTenantV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<UserIdList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2TaskUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2TaskCommentUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<i32>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_task_update_tenant_v1 => P2TaskUpdateTenantV1
        : "task.task.update_tenant_v1",
    on_p2_task_updated_v1 => P2TaskUpdatedV1
        : "task.task.updated_v1",
    on_p2_task_comment_updated_v1 => P2TaskCommentUpdatedV1
        : "task.task.comment.updated_v1",
}
