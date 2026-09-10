//! Video Conference (VC) v1 event handlers.

use serde::ser::SerializeMap;
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

/// A meeting participant reported by a VC bot webhook.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(from = "MeetingAgentEventUserWire")]
#[non_exhaustive]
pub struct MeetingAgentEventUser {
    /// Legacy string-valued wire ID. Mutually exclusive with `structured_id`.
    pub id: Option<String>,
    /// Structured wire ID used by newer webhooks; preserves all ID namespaces.
    /// Serialized under `id`, never as a separate `structured_id` wire field.
    pub structured_id: Option<UserId>,
    pub user_type: Option<i32>,
    pub user_role: Option<i32>,
    pub user_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MeetingAgentEventUserId {
    Legacy(String),
    Structured(UserId),
}

#[derive(Deserialize)]
struct MeetingAgentEventUserWire {
    id: Option<MeetingAgentEventUserId>,
    user_type: Option<i32>,
    user_role: Option<i32>,
    user_name: Option<String>,
}

impl From<MeetingAgentEventUserWire> for MeetingAgentEventUser {
    fn from(wire: MeetingAgentEventUserWire) -> Self {
        let (id, structured_id) = match wire.id {
            Some(MeetingAgentEventUserId::Legacy(id)) => (Some(id), None),
            Some(MeetingAgentEventUserId::Structured(id)) => (None, Some(id)),
            None => (None, None),
        };
        Self {
            id,
            structured_id,
            user_type: wire.user_type,
            user_role: wire.user_role,
            user_name: wire.user_name,
        }
    }
}

impl Serialize for MeetingAgentEventUser {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.id.is_some() && self.structured_id.is_some() {
            return Err(serde::ser::Error::custom(
                "VC bot user cannot have both id and structured_id",
            ));
        }
        let mut map = serializer.serialize_map(None)?;
        if let Some(id) = &self.id {
            map.serialize_entry("id", id)?;
        }
        if let Some(id) = &self.structured_id {
            map.serialize_entry("id", id)?;
        }
        if let Some(user_type) = self.user_type {
            map.serialize_entry("user_type", &user_type)?;
        }
        if let Some(user_role) = self.user_role {
            map.serialize_entry("user_role", &user_role)?;
        }
        if let Some(user_name) = &self.user_name {
            map.serialize_entry("user_name", user_name)?;
        }
        map.end()
    }
}

/// The meeting information supplied to VC bot webhooks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MeetingAgentEventMeeting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_user: Option<MeetingAgentEventUser>,
}

/// A shared document reported by a VC bot activity webhook.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShareDoc {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// A change to the currently focused document comment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommentFocus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focused: Option<bool>,
}

/// A change to the currently viewed document section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SectionLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_titles: Option<Vec<String>>,
}

/// A preview action for an element in a shared document.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ElementPreview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ParticipantJoinedItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ParticipantLeftItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_reason: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TranscriptItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_ms: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time_ms: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sentence_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatMessageItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MagicShareStartedItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_doc: Option<ShareDoc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// `share_started` or `share_detected`; absence means `share_started`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MagicShareEndedItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DocumentContextChangedItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_doc: Option<ShareDoc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_focus: Option<CommentFocus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_location: Option<SectionLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_preview: Option<ElementPreview>,
}

/// A countdown state change delivered to a VC bot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CountdownItem {
    /// SET, PROLONG, END_IN_ADVANCE, CLOSE, ENDED, or REMIND.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_play_audio_at_end: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders_before_end_in_second: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub countdown_set_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remain_minutes: Option<i32>,
}

/// One activity record delivered to a VC bot while it is in a meeting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MeetingActivityItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingAgentEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_joined_items: Option<Vec<ParticipantJoinedItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_left_items: Option<Vec<ParticipantLeftItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript_received_items: Option<Vec<TranscriptItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_received_items: Option<Vec<ChatMessageItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magic_share_started_items: Option<Vec<MagicShareStartedItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magic_share_ended_items: Option<Vec<MagicShareEndedItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_context_changed_items: Option<Vec<DocumentContextChangedItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub countdown_items: Option<Vec<CountdownItem>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcBotMeetingActivityV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_activity_items: Option<Vec<MeetingActivityItem>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcBotMeetingEndedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingAgentEventMeeting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcBotMeetingInvitedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingAgentEventMeeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inviter: Option<MeetingAgentEventUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcBotMeetingStartedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<MeetingAgentEventMeeting>,
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
pub struct P2VcNoteGeneratedV1Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber_ids: Option<Vec<UserId>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2VcNoteGeneratedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<P2VcNoteGeneratedV1Data>,
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
    on_p2_vc_bot_meeting_activity_v1 => P2VcBotMeetingActivityV1
        : "vc.bot.meeting_activity_v1",
    on_p2_vc_bot_meeting_ended_v1 => P2VcBotMeetingEndedV1
        : "vc.bot.meeting_ended_v1",
    on_p2_vc_bot_meeting_invited_v1 => P2VcBotMeetingInvitedV1
        : "vc.bot.meeting_invited_v1",
    on_p2_vc_bot_meeting_started_v1 => P2VcBotMeetingStartedV1
        : "vc.bot.meeting_started_v1",
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
    on_p2_vc_note_generated_v1 => P2VcNoteGeneratedV1
        : "vc.note.generated_v1",
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
