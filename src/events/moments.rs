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
pub struct P2MomentsCommentCreatedV1 {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub comment: serde_json::Value,
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

// ── EventDispatcher extension methods ──

impl EventDispatcher {
    pub fn on_p2_moments_post_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MomentsPostCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("moments.post.published_v1", wrap_handler(handler))
    }

    pub fn on_p2_moments_post_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MomentsPostDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("moments.post.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_moments_reaction_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MomentsReactionCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("moments.post.reacted_v1", wrap_handler(handler))
    }

    pub fn on_p2_moments_comment_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MomentsCommentCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("moments.post.comment_replied_v1", wrap_handler(handler))
    }
}
