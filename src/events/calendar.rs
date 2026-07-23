//! Calendar v4 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AclScopeEvent {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct P2CalendarChangedV4 {
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_calendar_changed_v4 => P2CalendarChangedV4
        : "calendar.calendar.changed_v4",
    on_p2_calendar_acl_created_v4 => P2CalendarAclCreatedV4
        : "calendar.calendar.acl.created_v4",
    on_p2_calendar_acl_deleted_v4 => P2CalendarAclDeletedV4
        : "calendar.calendar.acl.deleted_v4",
    on_p2_calendar_event_changed_v4 => P2CalendarEventChangedV4
        : "calendar.calendar.event.changed_v4",
}
