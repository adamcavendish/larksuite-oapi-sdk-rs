//! Compensation v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CompensationArchiveChangedV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub archive_id: String,
    #[serde(default)]
    pub effective_date: String,
}

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

impl EventDispatcher {
    pub fn on_p2_compensation_archive_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CompensationArchiveChangedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("compensation.archive.changed_v1", wrap_handler(handler))
    }
}
