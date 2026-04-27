//! Baike (Enterprise Encyclopedia) v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2BaikeConceptCreatedV1 {
    #[serde(default)]
    pub entity: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2BaikeConceptUpdatedV1 {
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
    pub fn on_p2_baike_concept_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2BaikeConceptCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("baike.draft.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_baike_concept_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2BaikeConceptUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("baike.entity.updated_v1", wrap_handler(handler))
    }
}
