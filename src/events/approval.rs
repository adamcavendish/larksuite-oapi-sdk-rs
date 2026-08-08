//! Approval v4 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApprovalEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_group_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_obj: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2ApprovalUpdatedV4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ApprovalEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2InstanceStatusChangedV4Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_user: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2InstanceStatusChangedV4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<P2InstanceStatusChangedV4Data>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2TaskStatusChangedV4Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2TaskStatusChangedV4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<P2TaskStatusChangedV4Data>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_approval_updated_v4 => P2ApprovalUpdatedV4
        : "approval.approval.updated_v4",
    on_p2_approval_instance_status_changed_v4 => P2InstanceStatusChangedV4
        : "approval.instance.status_changed_v4",
    on_p2_approval_task_status_changed_v4 => P2TaskStatusChangedV4
        : "approval.task.status_changed_v4",
}
