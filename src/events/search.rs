//! Search v2 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SearchDataSourceCreatedV1 {
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SearchDataSourceDeletedV1 {
    #[serde(default)]
    pub data_source_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SearchDataSourceUpdatedV1 {
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    pub name: String,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_search_data_source_created_v1 => P2SearchDataSourceCreatedV1
        : "search.data_source.created_v1",
    on_p2_search_data_source_deleted_v1 => P2SearchDataSourceDeletedV1
        : "search.data_source.deleted_v1",
    on_p2_search_data_source_updated_v1 => P2SearchDataSourceUpdatedV1
        : "search.data_source.updated_v1",
}
