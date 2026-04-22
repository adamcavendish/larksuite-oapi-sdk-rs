//! Moments v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

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

// ── Handler registration helpers ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync + 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => {
                Box::pin(handler(typed)) as Pin<Box<dyn Future<Output = Result<()>> + Send>>
            }
            Err(e) => Box::pin(async move {
                Err(Error::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods (all 7 moments/v1 handlers) ──

macro_rules! moments_v1_handler {
    ($method:ident, $event_key:literal, $payload_type:ty) => {
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload_type) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send + 'static,
        {
            self.on_event($event_key, wrap_handler(handler))
        }
    };
}

impl EventDispatcher {
    moments_v1_handler!(
        on_p2_moments_post_created_v1,
        "moments.post.created_v1",
        P2MomentsPostCreatedV1
    );
    moments_v1_handler!(
        on_p2_moments_post_deleted_v1,
        "moments.post.deleted_v1",
        P2MomentsPostDeletedV1
    );
    moments_v1_handler!(
        on_p2_moments_reaction_created_v1,
        "moments.reaction.created_v1",
        P2MomentsReactionCreatedV1
    );
    moments_v1_handler!(
        on_p2_moments_reaction_deleted_v1,
        "moments.reaction.deleted_v1",
        P2MomentsReactionDeletedV1
    );
    moments_v1_handler!(
        on_p2_moments_comment_created_v1,
        "moments.comment.created_v1",
        P2MomentsCommentCreatedV1
    );
    moments_v1_handler!(
        on_p2_moments_comment_deleted_v1,
        "moments.comment.deleted_v1",
        P2MomentsCommentDeletedV1
    );
    moments_v1_handler!(
        on_p2_moments_post_statistics_updated_v1,
        "moments.post_statistics.updated_v1",
        P2MomentsPostStatisticsUpdatedV1
    );
}
