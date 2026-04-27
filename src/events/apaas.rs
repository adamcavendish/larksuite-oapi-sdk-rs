//! aPaaS (Application Platform) v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApaasWorkspaceRecordChangeV1 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub record_ids: Vec<String>,
    #[serde(default)]
    pub object_api_name: String,
}

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

impl EventDispatcher {
    pub fn on_p2_apaas_workspace_record_change_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApaasWorkspaceRecordChangeV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("apaas.workspace.record_change_v1", wrap_handler(handler))
    }
}
