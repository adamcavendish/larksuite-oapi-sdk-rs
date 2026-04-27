//! Wiki v2 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

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

// ── Handler registration helpers ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
+ Send
+ Sync
+ 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => Box::pin(handler(typed))
                as Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>,
            Err(e) => Box::pin(async move {
                Err(LarkError::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods ──

impl EventDispatcher {
    pub fn on_p2_wiki_space_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2WikiSpaceCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("wiki.space.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_wiki_space_member_added_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2WikiSpaceMemberAddedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("wiki.space.member_added_v1", wrap_handler(handler))
    }

    pub fn on_p2_wiki_space_member_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2WikiSpaceMemberDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("wiki.space.member_deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_wiki_node_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2WikiNodeCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("wiki.node.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_wiki_node_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2WikiNodeDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("wiki.node.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_wiki_node_moved_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2WikiNodeMovedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("wiki.node.moved_v1", wrap_handler(handler))
    }
}
