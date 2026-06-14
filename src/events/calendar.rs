//! Calendar v4 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
}

impl UserId {
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    pub fn open_id(&self) -> Option<&str> {
        self.open_id.as_deref()
    }

    pub fn union_id(&self) -> Option<&str> {
        self.union_id.as_deref()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AclScopeEvent {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenEventRsvpInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsvp_status: Option<String>,
}

impl OpenEventRsvpInfo {
    pub fn open_id(&self) -> Option<&str> {
        self.from_user_id.as_ref().and_then(UserId::open_id)
    }

    pub fn user_id(&self) -> Option<&str> {
        self.from_user_id.as_ref().and_then(UserId::user_id)
    }

    pub fn union_id(&self) -> Option<&str> {
        self.from_user_id.as_ref().and_then(UserId::union_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarChangedV4 {
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarAclCreatedV4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<AclScopeEvent>,
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarAclDeletedV4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<AclScopeEvent>,
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CalendarEventChangedV4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(default)]
    pub rsvp_infos: Vec<OpenEventRsvpInfo>,
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

macro_rules! calendar_v4_handler {
    ($method:ident, $event_key:literal, $payload_type:ty) => {
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload_type) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
        {
            self.on_event($event_key, wrap_handler(handler))
        }
    };
}

impl EventDispatcher {
    calendar_v4_handler!(
        on_p2_calendar_changed_v4,
        "calendar.calendar.changed_v4",
        P2CalendarChangedV4
    );
    calendar_v4_handler!(
        on_p2_calendar_acl_created_v4,
        "calendar.calendar.acl.created_v4",
        P2CalendarAclCreatedV4
    );
    calendar_v4_handler!(
        on_p2_calendar_acl_deleted_v4,
        "calendar.calendar.acl.deleted_v4",
        P2CalendarAclDeletedV4
    );
    calendar_v4_handler!(
        on_p2_calendar_event_changed_v4,
        "calendar.calendar.event.changed_v4",
        P2CalendarEventChangedV4
    );
}
