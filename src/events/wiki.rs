//! Wiki v2 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2WikiSpaceCreatedV1 {
    #[serde(default)]
    pub space: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2WikiSpaceMemberAddedV1 {
    #[serde(default)]
    pub space_id: String,
    #[serde(default)]
    pub member: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2WikiSpaceMemberDeletedV1 {
    #[serde(default)]
    pub space_id: String,
    #[serde(default)]
    pub member: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2WikiNodeCreatedV1 {
    #[serde(default)]
    pub space_id: String,
    #[serde(default)]
    pub node_token: String,
    #[serde(default)]
    pub obj_type: String,
    #[serde(default)]
    pub parent_node_token: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2WikiNodeDeletedV1 {
    #[serde(default)]
    pub space_id: String,
    #[serde(default)]
    pub node_token: String,
    #[serde(default)]
    pub obj_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2WikiNodeMovedV1 {
    #[serde(default)]
    pub space_id: String,
    #[serde(default)]
    pub node_token: String,
    #[serde(default)]
    pub obj_type: String,
    #[serde(default)]
    pub parent_node_token: String,
    #[serde(default)]
    pub old_parent_node_token: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_wiki_space_created_v1 => P2WikiSpaceCreatedV1
        : "wiki.space.created_v1",
    on_p2_wiki_space_member_added_v1 => P2WikiSpaceMemberAddedV1
        : "wiki.space.member_added_v1",
    on_p2_wiki_space_member_deleted_v1 => P2WikiSpaceMemberDeletedV1
        : "wiki.space.member_deleted_v1",
    on_p2_wiki_node_created_v1 => P2WikiNodeCreatedV1
        : "wiki.node.created_v1",
    on_p2_wiki_node_deleted_v1 => P2WikiNodeDeletedV1
        : "wiki.node.deleted_v1",
    on_p2_wiki_node_moved_v1 => P2WikiNodeMovedV1
        : "wiki.node.moved_v1",
}
