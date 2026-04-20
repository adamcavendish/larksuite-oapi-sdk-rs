use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingStartedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingEndedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingJoinedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingLeftV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcRoomCreatedV1 {
    #[serde(default)]
    pub room: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcRoomDeletedV1 {
    #[serde(default)]
    pub room: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcRoomUpdatedV1 {
    #[serde(default)]
    pub room: serde_json::Value,
    #[serde(default)]
    pub old_room: serde_json::Value,
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
    pub fn on_p2_vc_meeting_started_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2VcMeetingStartedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("vc.meeting.meeting_started_v1", wrap_handler(handler))
    }

    pub fn on_p2_vc_meeting_ended_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2VcMeetingEndedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("vc.meeting.meeting_ended_v1", wrap_handler(handler))
    }

    pub fn on_p2_vc_meeting_joined_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2VcMeetingJoinedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("vc.meeting.participant_joined_v1", wrap_handler(handler))
    }

    pub fn on_p2_vc_meeting_left_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2VcMeetingLeftV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("vc.meeting.participant_left_v1", wrap_handler(handler))
    }

    pub fn on_p2_vc_room_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2VcRoomCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("vc.room.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_vc_room_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2VcRoomDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("vc.room.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_vc_room_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2VcRoomUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("vc.room.updated_v1", wrap_handler(handler))
    }
}
