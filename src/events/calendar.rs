//! Calendar v4 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarEventChangedV4 {
    #[serde(default)]
    pub calendar_id: String,
    #[serde(default)]
    pub event_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarAclCreatedV4 {
    #[serde(default)]
    pub acl_id: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub scope: serde_json::Value,
    #[serde(default)]
    pub user_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarAclDeletedV4 {
    #[serde(default)]
    pub acl_id: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub scope: serde_json::Value,
    #[serde(default)]
    pub user_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarChangedV4 {
    #[serde(default)]
    pub calendar_id: String,
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
    pub fn on_p2_calendar_event_changed_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CalendarEventChangedV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("calendar.calendar.event.changed_v4", wrap_handler(handler))
    }

    pub fn on_p2_calendar_acl_created_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CalendarAclCreatedV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("calendar.calendar.acl.created_v4", wrap_handler(handler))
    }

    pub fn on_p2_calendar_acl_deleted_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CalendarAclDeletedV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("calendar.calendar.acl.deleted_v4", wrap_handler(handler))
    }

    pub fn on_p2_calendar_changed_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CalendarChangedV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("calendar.calendar.changed_v4", wrap_handler(handler))
    }
}
