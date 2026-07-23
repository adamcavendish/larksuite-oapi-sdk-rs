//! Approval v4 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

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

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_approval_updated_v4 => P2ApprovalUpdatedV4
        : "approval.approval.updated_v4",
}
