//! Minutes v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GeneratedSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_entity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2MinuteGeneratedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute_source: Option<GeneratedSource>,
    #[serde(default)]
    pub subscriber_ids: Vec<UserId>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_minutes_minute_generated_v1 => P2MinuteGeneratedV1
        : "minutes.minute.generated_v1",
}
