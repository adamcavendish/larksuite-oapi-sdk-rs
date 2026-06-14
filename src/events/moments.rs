//! Moments v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsPostCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default)]
    pub category_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsPostDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsReactionCreatedV1 {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsReactionDeletedV1 {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsCommentCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsCommentDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MomentsPostStatisticsUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<PostStatistics>,
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
