//! Lingo (Terminology) v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

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
    pub fn on_p2_lingo_concept_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2LingoConceptCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("lingo.draft.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_lingo_concept_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2LingoConceptUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("lingo.entity.updated_v1", wrap_handler(handler))
    }
}
