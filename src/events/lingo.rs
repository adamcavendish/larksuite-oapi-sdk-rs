//! Lingo (Terminology) v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2LingoConceptCreatedV1 {
    #[serde(default)]
    pub entity: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2LingoConceptUpdatedV1 {
    #[serde(default)]
    pub entity: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_lingo_concept_created_v1 => P2LingoConceptCreatedV1
        : "lingo.draft.updated_v1",
    on_p2_lingo_concept_updated_v1 => P2LingoConceptUpdatedV1
        : "lingo.entity.updated_v1",
}
