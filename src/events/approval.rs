//! Approval v4 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApprovalInstanceCreatedV4 {
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub approval_code: String,
    #[serde(default)]
    pub type_: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApprovalTaskCreatedV4 {
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub approval_code: String,
    #[serde(default)]
    pub task_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApprovalCcCreatedV4 {
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub approval_code: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub read_status: String,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
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
    pub fn on_p2_approval_instance_created_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApprovalInstanceCreatedV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "approval.approval.instance.created_v4",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_approval_task_created_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApprovalTaskCreatedV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("approval.approval.task.created_v4", wrap_handler(handler))
    }

    pub fn on_p2_approval_cc_created_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApprovalCcCreatedV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("approval.approval.cc.created_v4", wrap_handler(handler))
    }
}
