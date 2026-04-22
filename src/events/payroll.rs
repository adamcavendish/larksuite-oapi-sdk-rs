//! Payroll v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PayrollPaymentActivityApprovedV1 {
    #[serde(default)]
    pub activity_id: String,
    #[serde(default)]
    pub period: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PayrollPaymentActivityStatusChangedV1 {
    #[serde(default)]
    pub activity_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub period: serde_json::Value,
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
    pub fn on_p2_payroll_payment_activity_approved_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2PayrollPaymentActivityApprovedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "payroll.payment_activity.approved_v1",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_payroll_payment_activity_status_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2PayrollPaymentActivityStatusChangedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "payroll.payment_activity.status_changed_v1",
            wrap_handler(handler),
        )
    }
}
