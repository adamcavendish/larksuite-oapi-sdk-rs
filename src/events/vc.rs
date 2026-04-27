//! Video Conference (VC) v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
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
pub struct P2VcMeetingAllMeetingStartedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingAllMeetingEndedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingRecordingStartedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingRecordingEndedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingRecordingReadyV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub duration: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingShareStartedV1 {
    #[serde(default)]
    pub meeting: serde_json::Value,
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcMeetingShareEndedV1 {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcRoomLevelCreatedV1 {
    #[serde(default)]
    pub room_level: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcRoomLevelDeletedV1 {
    #[serde(default)]
    pub room_level: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcRoomLevelUpdatedV1 {
    #[serde(default)]
    pub room_level: serde_json::Value,
    #[serde(default)]
    pub old_room_level: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2VcReserveConfigUpdatedV1 {
    #[serde(default)]
    pub reserve_config: serde_json::Value,
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

// ── EventDispatcher extension methods (all 18 vc/v1 handlers) ──

macro_rules! vc_v1_handler {
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
    vc_v1_handler!(
        on_p2_vc_meeting_started_v1,
        "vc.meeting.meeting_started_v1",
        P2VcMeetingStartedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_ended_v1,
        "vc.meeting.meeting_ended_v1",
        P2VcMeetingEndedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_joined_v1,
        "vc.meeting.join_meeting_v1",
        P2VcMeetingJoinedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_left_v1,
        "vc.meeting.leave_meeting_v1",
        P2VcMeetingLeftV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_all_meeting_started_v1,
        "vc.meeting.all_meeting_started_v1",
        P2VcMeetingAllMeetingStartedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_all_meeting_ended_v1,
        "vc.meeting.all_meeting_ended_v1",
        P2VcMeetingAllMeetingEndedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_recording_started_v1,
        "vc.meeting.recording_started_v1",
        P2VcMeetingRecordingStartedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_recording_ended_v1,
        "vc.meeting.recording_ended_v1",
        P2VcMeetingRecordingEndedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_recording_ready_v1,
        "vc.meeting.recording_ready_v1",
        P2VcMeetingRecordingReadyV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_share_started_v1,
        "vc.meeting.share_started_v1",
        P2VcMeetingShareStartedV1
    );
    vc_v1_handler!(
        on_p2_vc_meeting_share_ended_v1,
        "vc.meeting.share_ended_v1",
        P2VcMeetingShareEndedV1
    );
    vc_v1_handler!(
        on_p2_vc_room_created_v1,
        "vc.room.created_v1",
        P2VcRoomCreatedV1
    );
    vc_v1_handler!(
        on_p2_vc_room_deleted_v1,
        "vc.room.deleted_v1",
        P2VcRoomDeletedV1
    );
    vc_v1_handler!(
        on_p2_vc_room_updated_v1,
        "vc.room.updated_v1",
        P2VcRoomUpdatedV1
    );
    vc_v1_handler!(
        on_p2_vc_room_level_created_v1,
        "vc.room_level.created_v1",
        P2VcRoomLevelCreatedV1
    );
    vc_v1_handler!(
        on_p2_vc_room_level_deleted_v1,
        "vc.room_level.deleted_v1",
        P2VcRoomLevelDeletedV1
    );
    vc_v1_handler!(
        on_p2_vc_room_level_updated_v1,
        "vc.room_level.updated_v1",
        P2VcRoomLevelUpdatedV1
    );
    vc_v1_handler!(
        on_p2_vc_reserve_config_updated_v1,
        "vc.reserve_config.updated_v1",
        P2VcReserveConfigUpdatedV1
    );
}
