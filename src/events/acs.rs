//! Access Control System (ACS) v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2AccessRecordCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_clock_in: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_door_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2AcsUserUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_uploaded: Option<bool>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_acs_access_record_created_v1 => P2AccessRecordCreatedV1
        : "acs.access_record.created_v1",
    on_p2_acs_user_updated_v1 => P2AcsUserUpdatedV1
        : "acs.user.updated_v1",
}
