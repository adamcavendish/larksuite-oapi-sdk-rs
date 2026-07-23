//! Meeting Room v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2MeetingRoomCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2MeetingRoomUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2MeetingRoomDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2MeetingRoomStatusChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_meeting_room_created_v1 => P2MeetingRoomCreatedV1
        : "meeting_room.meeting_room.created_v1",
    on_p2_meeting_room_updated_v1 => P2MeetingRoomUpdatedV1
        : "meeting_room.meeting_room.updated_v1",
    on_p2_meeting_room_deleted_v1 => P2MeetingRoomDeletedV1
        : "meeting_room.meeting_room.deleted_v1",
    on_p2_meeting_room_status_changed_v1 => P2MeetingRoomStatusChangedV1
        : "meeting_room.meeting_room.status_changed_v1",
}
