//! Security and Compliance v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2 {
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub device_sn: String,
    #[serde(default)]
    pub apply_type: i32,
    #[serde(default)]
    pub apply_time: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2 {
    #[serde(default)]
    pub user_id: serde_json::Value,
    #[serde(default)]
    pub device_sn: String,
    #[serde(default)]
    pub event_type: i32,
    #[serde(default)]
    pub event_time: i64,
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
    pub fn on_p2_security_and_compliance_device_apply_record_device_apply_event_v2<F, Fut>(
        self,
        handler: F,
    ) -> Self
    where
        F: Fn(P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2) -> Fut
            + Send
            + Sync
            + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "security_and_compliance.device_apply_record.device_apply_event_v2",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_security_and_compliance_device_record_device_change_event_v2<F, Fut>(
        self,
        handler: F,
    ) -> Self
    where
        F: Fn(P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2) -> Fut
            + Send
            + Sync
            + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "security_and_compliance.device_record.device_change_event_v2",
            wrap_handler(handler),
        )
    }
}
