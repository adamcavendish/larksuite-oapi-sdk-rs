//! Video Conference (VC) v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MeetingEventUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_role: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}

impl MeetingEventUser {
    pub fn user_id(&self) -> Option<&str> {
        self.id.as_ref().and_then(UserId::user_id)
    }

    pub fn open_id(&self) -> Option<&str> {
        self.id.as_ref().and_then(UserId::open_id)
    }

    pub fn union_id(&self) -> Option<&str> {
        self.id.as_ref().and_then(UserId::union_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MeetingSecuritySetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<UserId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_set_security_contacts_and_group: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MeetingWebinarSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webinar_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MeetingEventMeeting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_user: Option<MeetingEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<MeetingEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_sub_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_setting: Option<MeetingSecuritySetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webinar_setting: Option<MeetingWebinarSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Device {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RoomStatusEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_ids: Option<Vec<UserId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_notice: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_notice: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RoomEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatusEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Vec<Device>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RoomLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_child: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubscribeUserEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubscribeDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApprovalConfigEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_switch: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_condition: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_duration: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<SubscribeUserEvent>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReserveScopeConfigEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_all_users: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_users: Option<Vec<SubscribeUserEvent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_depts: Option<Vec<SubscribeDepartment>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TimeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_cover_child_scope: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_switch: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_in_advance: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opening_hour: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingStartedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingEndedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingJoinedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingLeftV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_reason: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_user: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingAllMeetingStartedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingAllMeetingEndedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingParticipantMeetingEndedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber_ids: Option<Vec<UserId>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingRecordingStartedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingRecordingEndedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingRecordingReadyV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingShareStartedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcMeetingShareEndedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingEventUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcRoomCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<RoomEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcRoomDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<RoomEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcRoomUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<RoomEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcRoomLevelCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level: Option<RoomLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcRoomLevelDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_child: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcRoomLevelUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level: Option<RoomLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcReserveConfigUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_config: Option<ApprovalConfigEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_config: Option<TimeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_scope_config: Option<ReserveScopeConfigEvent>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_vc_meeting_started_v1 => P2VcMeetingStartedV1
        : "vc.meeting.meeting_started_v1",
    on_p2_vc_meeting_ended_v1 => P2VcMeetingEndedV1
        : "vc.meeting.meeting_ended_v1",
    on_p2_vc_meeting_joined_v1 => P2VcMeetingJoinedV1
        : "vc.meeting.join_meeting_v1",
    on_p2_vc_meeting_left_v1 => P2VcMeetingLeftV1
        : "vc.meeting.leave_meeting_v1",
    on_p2_vc_meeting_all_meeting_started_v1 => P2VcMeetingAllMeetingStartedV1
        : "vc.meeting.all_meeting_started_v1",
    on_p2_vc_meeting_all_meeting_ended_v1 => P2VcMeetingAllMeetingEndedV1
        : "vc.meeting.all_meeting_ended_v1",
    on_p2_vc_meeting_participant_meeting_ended_v1 => P2VcMeetingParticipantMeetingEndedV1
        : "vc.meeting.participant_meeting_ended_v1",
    on_p2_vc_meeting_recording_started_v1 => P2VcMeetingRecordingStartedV1
        : "vc.meeting.recording_started_v1",
    on_p2_vc_meeting_recording_ended_v1 => P2VcMeetingRecordingEndedV1
        : "vc.meeting.recording_ended_v1",
    on_p2_vc_meeting_recording_ready_v1 => P2VcMeetingRecordingReadyV1
        : "vc.meeting.recording_ready_v1",
    on_p2_vc_meeting_share_started_v1 => P2VcMeetingShareStartedV1
        : "vc.meeting.share_started_v1",
    on_p2_vc_meeting_share_ended_v1 => P2VcMeetingShareEndedV1
        : "vc.meeting.share_ended_v1",
    on_p2_vc_room_created_v1 => P2VcRoomCreatedV1
        : "vc.room.created_v1",
    on_p2_vc_room_deleted_v1 => P2VcRoomDeletedV1
        : "vc.room.deleted_v1",
    on_p2_vc_room_updated_v1 => P2VcRoomUpdatedV1
        : "vc.room.updated_v1",
    on_p2_vc_room_level_created_v1 => P2VcRoomLevelCreatedV1
        : "vc.room_level.created_v1",
    on_p2_vc_room_level_deleted_v1 => P2VcRoomLevelDeletedV1
        : "vc.room_level.deleted_v1",
    on_p2_vc_room_level_updated_v1 => P2VcRoomLevelUpdatedV1
        : "vc.room_level.updated_v1",
    on_p2_vc_reserve_config_updated_v1 => P2VcReserveConfigUpdatedV1
        : "vc.reserve_config.updated_v1",
}
