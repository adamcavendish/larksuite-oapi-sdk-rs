use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireOfferStatusChangedV1 {
    #[serde(default)]
    pub offer_id: String,
    #[serde(default)]
    pub offer_type: i32,
    #[serde(default)]
    pub schema_id: String,
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub offer_status: i32,
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub talent_id: String,
    #[serde(default)]
    pub creator_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireApplicationStageChangedV1 {
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub stage_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HireEhrImportTaskImportedV1 {
    #[serde(default)]
    pub task_id: String,
    #[serde(default)]
    pub application_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
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
    pub fn on_p2_hire_offer_status_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2HireOfferStatusChangedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("hire.offer.status_changed_v1", wrap_handler(handler))
    }

    pub fn on_p2_hire_application_stage_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2HireApplicationStageChangedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("hire.application.stage_changed_v1", wrap_handler(handler))
    }

    pub fn on_p2_hire_ehr_import_task_imported_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2HireEhrImportTaskImportedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("hire.ehr_import_task.imported_v1", wrap_handler(handler))
    }
}
