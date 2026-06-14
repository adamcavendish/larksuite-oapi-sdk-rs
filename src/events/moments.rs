//! Moments v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsPostCreatedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub post: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsPostDeletedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub delete_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsReactionCreatedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub reaction_type: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsReactionDeletedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub reaction_type: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsCommentCreatedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub comment: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsCommentDeletedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub comment_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsPostStatisticsUpdatedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub statistics: serde_json::Value,
}

// ── EventDispatcher extension methods (all 7 moments/v1 handlers) ──

event_handlers! {
    on_p2_moments_post_created_v1 => P2MomentsPostCreatedV1
        : "moments.post.created_v1",
    on_p2_moments_post_deleted_v1 => P2MomentsPostDeletedV1
        : "moments.post.deleted_v1",
    on_p2_moments_reaction_created_v1 => P2MomentsReactionCreatedV1
        : "moments.reaction.created_v1",
    on_p2_moments_reaction_deleted_v1 => P2MomentsReactionDeletedV1
        : "moments.reaction.deleted_v1",
    on_p2_moments_comment_created_v1 => P2MomentsCommentCreatedV1
        : "moments.comment.created_v1",
    on_p2_moments_comment_deleted_v1 => P2MomentsCommentDeletedV1
        : "moments.comment.deleted_v1",
    on_p2_moments_post_statistics_updated_v1 => P2MomentsPostStatisticsUpdatedV1
        : "moments.post_statistics.updated_v1",
}
