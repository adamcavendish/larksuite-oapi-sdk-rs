//! OKR v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2OkrOkrUpdatedV1 {
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub period_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub objectives: Vec<serde_json::Value>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_okr_okr_updated_v1 => P2OkrOkrUpdatedV1
        : "okr.okr.updated_v1",
}
