//! Bitable (Multidimensional Table) v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2BitableFieldChangedV1 {
    #[serde(default)]
    pub app_token: String,
    #[serde(default)]
    pub table_id: String,
    #[serde(default)]
    pub field_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub action_type: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2BitableRecordChangedV1 {
    #[serde(default)]
    pub app_token: String,
    #[serde(default)]
    pub table_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub action_type: String,
    #[serde(default)]
    pub record_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2BitableTableChangedV1 {
    #[serde(default)]
    pub app_token: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub action_type: String,
    #[serde(default)]
    pub table_id: String,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_bitable_field_changed_v1 => P2BitableFieldChangedV1
        : "bitable.bitable.field_updated_v1",
    on_p2_bitable_record_changed_v1 => P2BitableRecordChangedV1
        : "bitable.bitable.record_updated_v1",
    on_p2_bitable_table_changed_v1 => P2BitableTableChangedV1
        : "bitable.bitable.table_updated_v1",
}
