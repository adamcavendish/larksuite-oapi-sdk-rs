//! Performance v2 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PerformanceStageTaskCreatedV1 {
    #[serde(default)]
    pub activity_id: String,
    #[serde(default)]
    pub stage_task: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PerformanceStageTaskUpdatedV1 {
    #[serde(default)]
    pub activity_id: String,
    #[serde(default)]
    pub stage_task: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub changed_fields: Vec<String>,
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
    pub fn on_p2_performance_stage_task_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2PerformanceStageTaskCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("performance.stage_task.created_v2", wrap_handler(handler))
    }

    pub fn on_p2_performance_stage_task_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2PerformanceStageTaskUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("performance.stage_task.updated_v2", wrap_handler(handler))
    }
}
