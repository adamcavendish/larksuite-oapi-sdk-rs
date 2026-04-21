use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SearchDataSourceCreatedV1 {
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SearchDataSourceDeletedV1 {
    #[serde(default)]
    pub data_source_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SearchDataSourceUpdatedV1 {
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    pub name: String,
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
    pub fn on_p2_search_data_source_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2SearchDataSourceCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("search.data_source.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_search_data_source_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2SearchDataSourceDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("search.data_source.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_search_data_source_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2SearchDataSourceUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("search.data_source.updated_v1", wrap_handler(handler))
    }
}
