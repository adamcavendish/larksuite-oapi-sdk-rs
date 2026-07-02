use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Calendar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_third_party: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventAttendee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsvp_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_organizer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_members: Option<Vec<AttendeeChatMember>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub third_party_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_customization: Option<Vec<AttendeeResourceCustomization>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttendeeChatMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsvp_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_organizer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttendeeResourceCustomization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<AttendeeResourceCustomizationOption>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttendeeResourceCustomizationOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub others_content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalendarEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer_calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_notification: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vchat: Option<Vchat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendee_ability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_busy_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<EventLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_exception: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_organizer: Option<EventOrganizer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendee>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more_attendee: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vchat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vc_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_settings: Option<MeetingSettings>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_meeting_permission: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_screen_sharing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_attendee_unmute: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remind_before_event_min: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_waiting_room: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_record: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reminder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventOrganizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalendarAcl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acl_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<AclScope>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AclScope {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeZone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_abbr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_dst: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dst_offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utc_offset: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExchangeBinding {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_binding_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalendarSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_pull_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_add_bot: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FreeBusyEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeoffEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoff_event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Instance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_exception: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer_calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vchat: Option<Vchat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendee_ability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_busy_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<EventLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_organizer: Option<EventOrganizer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendee>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCalendar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Calendar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Freebusy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsvp_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFreebusy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freebusy_items: Option<Vec<Freebusy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventSearchFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_ids: Option<Vec<String>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCalendarReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchCalendarReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vchat: Option<Vchat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_ability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_busy_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EventLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendee>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vchat: Option<Vchat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_ability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_busy_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EventLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendee>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAttendeeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_start_time_admin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enable_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_operator_edit_call_permission: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAttendeeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_start_time_admin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enable_admin: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAclReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AclScope>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateExchangeBindingReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchCalendarSettingReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_pull_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_add_bot: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListFreeBusyReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MgetCalendarReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchCalendarReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PrimarysCalendarReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ReplyEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsvp_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventSearchFilter>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchFreeBusyReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_external_calendar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_busy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_rsvp_status: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GenerateCaldavConfReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTimeoffEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalendarListData {
    #[serde(default)]
    pub calendar_list: Vec<Calendar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalendarEventData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<CalendarEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalendarEventListData {
    #[serde(default)]
    pub items: Vec<CalendarEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttendeeListData {
    #[serde(default)]
    pub items: Vec<EventAttendee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AclListData {
    #[serde(default)]
    pub acls: Vec<CalendarAcl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeZoneListData {
    #[serde(default)]
    pub timezone_list: Vec<TimeZone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FreeBusyData {
    #[serde(default)]
    pub free_busy: std::collections::HashMap<String, Vec<FreeBusyEvent>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MgetCalendarData {
    #[serde(default)]
    pub calendars: Vec<Calendar>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchCalendarData {
    #[serde(default)]
    pub items: Vec<Calendar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscribeCalendarData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Calendar>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrimarysCalendarData {
    #[serde(default)]
    pub calendars: Vec<UserCalendar>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceViewData {
    #[serde(default)]
    pub items: Vec<Instance>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstancesData {
    #[serde(default)]
    pub items: Vec<Instance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchEventData {
    #[serde(default)]
    pub items: Vec<CalendarEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberListData {
    #[serde(default)]
    pub items: Vec<AttendeeChatMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingChatData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applink: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingMinuteData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchFreeBusyData {
    #[serde(default)]
    pub freebusy_lists: Vec<UserFreebusy>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CaldavConfData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeoffEventData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoff_event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl_resp!(CreateCalendarResp, Calendar);
impl_resp!(GetCalendarResp, Calendar);
impl_resp!(PatchCalendarResp, Calendar);
impl_resp!(ListCalendarResp, CalendarListData);
impl_resp!(CreateEventResp, CalendarEventData);
impl_resp!(GetEventResp, CalendarEventData);
impl_resp!(PatchEventResp, CalendarEventData);
impl_resp!(ListEventResp, CalendarEventListData);
impl_resp!(CreateAttendeeResp, AttendeeListData);
impl_resp!(ListAttendeeResp, AttendeeListData);
impl_resp!(CreateAclResp, CalendarAcl);
impl_resp!(ListAclResp, AclListData);
impl_resp!(ListTimeZoneResp, TimeZoneListData);
impl_resp!(CreateExchangeBindingResp, ExchangeBinding);
impl_resp!(GetExchangeBindingResp, ExchangeBinding);
impl_resp!(GetCalendarSettingResp, CalendarSetting);
impl_resp!(PatchCalendarSettingResp, CalendarSetting);
impl_resp!(ListFreeBusyResp, FreeBusyData);
impl_resp!(MgetCalendarResp, MgetCalendarData);
impl_resp!(SearchCalendarResp, SearchCalendarData);
impl_resp!(SubscribeCalendarResp, SubscribeCalendarData);
impl_resp!(PrimarysCalendarResp, PrimarysCalendarData);
impl_resp!(InstanceViewResp, InstanceViewData);
impl_resp!(InstancesResp, InstancesData);
impl_resp!(ReplyEventResp, serde_json::Value);
impl_resp!(SearchEventResp, SearchEventData);
impl_resp!(ListChatMemberResp, ChatMemberListData);
impl_resp!(CreateMeetingChatResp, MeetingChatData);
impl_resp!(CreateMeetingMinuteResp, MeetingMinuteData);
impl_resp!(BatchFreeBusyResp, BatchFreeBusyData);
impl_resp!(GenerateCaldavConfResp, CaldavConfData);
impl_resp!(CreateTimeoffEventResp, TimeoffEventData);

// ── Resources ──

pub struct CalendarResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetCalendarQuery<'a> {
    pub calendar_id: &'a str,
}

impl<'a> GetCalendarQuery<'a> {
    pub fn new(calendar_id: &'a str) -> Self {
        Self { calendar_id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListCalendarQuery<'a> {
    pub page: PageQuery<'a>,
    pub sync_token: Option<&'a str>,
}

impl<'a> ListCalendarQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn sync_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.sync_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchCalendarQuery<'a> {
    pub body: &'a SearchCalendarReqBody,
    pub page: PageQuery<'a>,
}

impl<'a> SearchCalendarQuery<'a> {
    pub fn new(body: &'a SearchCalendarReqBody) -> Self {
        Self {
            body,
            page: PageQuery::new(),
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

impl<'a> CalendarResource<'a> {
    pub async fn create(
        &self,
        body: &CreateCalendarReqBody,
        option: &RequestOption,
    ) -> Result<CreateCalendarResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/calendars",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<Calendar, CreateCalendarResp>()
        .await
    }

    pub async fn get(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<GetCalendarResp, LarkError> {
        let query = GetCalendarQuery::new(calendar_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCalendarQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetCalendarResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{}", query.calendar_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<Calendar, GetCalendarResp>()
        .await
    }

    pub async fn patch(
        &self,
        calendar_id: &str,
        body: &PatchCalendarReqBody,
        option: &RequestOption,
    ) -> Result<PatchCalendarResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<Calendar, PatchCalendarResp>()
        .await
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        sync_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCalendarResp, LarkError> {
        let query = ListCalendarQuery {
            page: PageQuery::from_parts(page_size, page_token),
            sync_token,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCalendarQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCalendarResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/calendar/v4/calendars",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("sync_token", query.sync_token)
        .send_response::<CalendarListData, ListCalendarResp>()
        .await
    }

    pub async fn primary(
        &self,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetCalendarResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/primary",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_response::<Calendar, GetCalendarResp>()
        .await
    }

    pub async fn mget(
        &self,
        body: &MgetCalendarReqBody,
        option: &RequestOption,
    ) -> Result<MgetCalendarResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/mget",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<MgetCalendarData, MgetCalendarResp>()
        .await
    }

    pub async fn primarys(
        &self,
        body: &PrimarysCalendarReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PrimarysCalendarResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/primarys",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<PrimarysCalendarData, PrimarysCalendarResp>()
        .await
    }

    pub async fn search(
        &self,
        body: &SearchCalendarReqBody,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<SearchCalendarResp, LarkError> {
        let query = SearchCalendarQuery {
            body,
            page: PageQuery::from_parts(page_size, page_token),
        };
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchCalendarQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchCalendarResp, LarkError> {
        let request = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/search",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .json_body(query.body)?;
        let (api_resp, raw) = request.send::<SearchCalendarData>().await?;
        Ok(SearchCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn subscribe(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<SubscribeCalendarResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/subscribe");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_response::<SubscribeCalendarData, SubscribeCalendarResp>()
        .await
    }

    pub async fn subscription(&self, option: &RequestOption) -> Result<EmptyResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/subscription",
            vec![AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscribe(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/unsubscribe");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscription(&self, option: &RequestOption) -> Result<EmptyResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/unsubscription",
            vec![AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct CalendarEventResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetEventQuery<'a> {
    pub calendar_id: &'a str,
    pub event_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetEventQuery<'a> {
    pub fn new(calendar_id: &'a str, event_id: &'a str) -> Self {
        Self {
            calendar_id,
            event_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListEventQuery<'a> {
    pub calendar_id: &'a str,
    pub page: PageQuery<'a>,
    pub anchor_time: Option<&'a str>,
    pub sync_token: Option<&'a str>,
    pub start_time: Option<&'a str>,
    pub end_time: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListEventQuery<'a> {
    pub fn new(calendar_id: &'a str) -> Self {
        Self {
            calendar_id,
            page: PageQuery::new(),
            anchor_time: None,
            sync_token: None,
            start_time: None,
            end_time: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn anchor_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.anchor_time = value.into();
        self
    }

    pub fn sync_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.sync_token = value.into();
        self
    }

    pub fn start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_time = value.into();
        self
    }

    pub fn end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_time = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct InstanceViewEventQuery<'a> {
    pub calendar_id: &'a str,
    pub start_time: Option<&'a str>,
    pub end_time: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> InstanceViewEventQuery<'a> {
    pub fn new(calendar_id: &'a str) -> Self {
        Self {
            calendar_id,
            start_time: None,
            end_time: None,
            user_id_type: None,
        }
    }

    pub fn start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_time = value.into();
        self
    }

    pub fn end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_time = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct InstancesEventQuery<'a> {
    pub calendar_id: &'a str,
    pub event_id: &'a str,
    pub page: PageQuery<'a>,
    pub start_time: Option<&'a str>,
    pub end_time: Option<&'a str>,
}

impl<'a> InstancesEventQuery<'a> {
    pub fn new(calendar_id: &'a str, event_id: &'a str) -> Self {
        Self {
            calendar_id,
            event_id,
            page: PageQuery::new(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_time = value.into();
        self
    }

    pub fn end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_time = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchEventQuery<'a> {
    pub calendar_id: &'a str,
    pub body: &'a SearchEventReqBody,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SearchEventQuery<'a> {
    pub fn new(calendar_id: &'a str, body: &'a SearchEventReqBody) -> Self {
        Self {
            calendar_id,
            body,
            page: PageQuery::new(),
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> CalendarEventResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        body: &CreateEventReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateEventResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<CalendarEventData, CreateEventResp>()
        .await
    }

    pub async fn get(
        &self,
        calendar_id: &str,
        event_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetEventResp, LarkError> {
        let query = GetEventQuery::new(calendar_id, event_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetEventQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetEventResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/events/{}",
            query.calendar_id, query.event_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<CalendarEventData, GetEventResp>()
        .await
    }

    pub async fn patch(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &PatchEventReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchEventResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<CalendarEventData, PatchEventResp>()
        .await
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        event_id: &str,
        need_notification: Option<bool>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("need_notification", need_notification)
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        calendar_id: &str,
        page_size: Option<i32>,
        anchor_time: Option<&str>,
        page_token: Option<&str>,
        sync_token: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEventResp, LarkError> {
        let query = ListEventQuery {
            calendar_id,
            page: PageQuery::from_parts(page_size, page_token),
            anchor_time,
            sync_token,
            start_time,
            end_time,
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEventQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEventResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/events",
            query.calendar_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("anchor_time", query.anchor_time)
        .query("sync_token", query.sync_token)
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("user_id_type", query.user_id_type)
        .send_response::<CalendarEventListData, ListEventResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn instance_view(
        &self,
        calendar_id: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<InstanceViewResp, LarkError> {
        let query = InstanceViewEventQuery::new(calendar_id)
            .start_time(start_time)
            .end_time(end_time)
            .user_id_type(user_id_type);
        self.instance_view_by_query(&query, option).await
    }

    pub async fn instance_view_by_query(
        &self,
        query: &InstanceViewEventQuery<'_>,
        option: &RequestOption,
    ) -> Result<InstanceViewResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/events/instance_view",
            query.calendar_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("user_id_type", query.user_id_type)
        .send_response::<InstanceViewData, InstanceViewResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn instances(
        &self,
        calendar_id: &str,
        event_id: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<InstancesResp, LarkError> {
        let query = InstancesEventQuery {
            calendar_id,
            event_id,
            page: PageQuery::from_parts(page_size, page_token),
            start_time,
            end_time,
        };
        self.instances_by_query(&query, option).await
    }

    pub async fn instances_by_query(
        &self,
        query: &InstancesEventQuery<'_>,
        option: &RequestOption,
    ) -> Result<InstancesResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/events/{}/instances",
            query.calendar_id, query.event_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .page_query(query.page)
        .send_response::<InstancesData, InstancesResp>()
        .await
    }

    pub async fn reply(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &ReplyEventReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn search(
        &self,
        calendar_id: &str,
        body: &SearchEventReqBody,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<SearchEventResp, LarkError> {
        let query = SearchEventQuery {
            calendar_id,
            body,
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
        };
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchEventQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchEventResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/events/search",
            query.calendar_id
        );
        let request = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .json_body(query.body)?;
        let (api_resp, raw) = request.send::<SearchEventData>().await?;
        Ok(SearchEventResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn subscription(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/subscription");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscription(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/unsubscription");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct CalendarAttendeeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListAttendeeQuery<'a> {
    pub calendar_id: &'a str,
    pub event_id: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListAttendeeQuery<'a> {
    pub fn new(calendar_id: &'a str, event_id: &'a str) -> Self {
        Self {
            calendar_id,
            event_id,
            page: PageQuery::new(),
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> CalendarAttendeeResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &CreateAttendeeReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateAttendeeResp, LarkError> {
        let path =
            format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/attendees");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<AttendeeListData, CreateAttendeeResp>()
        .await
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &DeleteAttendeeReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/attendees/batch_delete"
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        calendar_id: &str,
        event_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAttendeeResp, LarkError> {
        let query = ListAttendeeQuery {
            calendar_id,
            event_id,
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAttendeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAttendeeResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/events/{}/attendees",
            query.calendar_id, query.event_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_response::<AttendeeListData, ListAttendeeResp>()
        .await
    }
}

pub struct CalendarAclResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListAclQuery<'a> {
    pub calendar_id: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListAclQuery<'a> {
    pub fn new(calendar_id: &'a str) -> Self {
        Self {
            calendar_id,
            page: PageQuery::new(),
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> CalendarAclResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        body: &CreateAclReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateAclResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<CalendarAcl, CreateAclResp>()
        .await
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        acl_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/{acl_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        calendar_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListAclResp, LarkError> {
        let query = ListAclQuery {
            calendar_id,
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAclQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAclResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/acls",
            query.calendar_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_response::<AclListData, ListAclResp>()
        .await
    }

    pub async fn subscription(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/subscription");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscription(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/unsubscription");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct TimeZoneResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListTimeZoneQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListTimeZoneQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

impl<'a> TimeZoneResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTimeZoneResp, LarkError> {
        let query = ListTimeZoneQuery {
            page: PageQuery::from_parts(page_size, page_token),
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTimeZoneQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTimeZoneResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/calendar/v4/timezones",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .send_response::<TimeZoneListData, ListTimeZoneResp>()
        .await
    }
}

pub struct ExchangeBindingResource<'a> {
    config: &'a Config,
}

impl<'a> ExchangeBindingResource<'a> {
    pub async fn create(
        &self,
        body: &CreateExchangeBindingReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateExchangeBindingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/exchange_bindings",
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<ExchangeBinding, CreateExchangeBindingResp>()
        .await
    }

    pub async fn get(
        &self,
        exchange_binding_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetExchangeBindingResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/exchange_bindings/{exchange_binding_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_response::<ExchangeBinding, GetExchangeBindingResp>()
        .await
    }

    pub async fn delete(
        &self,
        exchange_binding_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/exchange_bindings/{exchange_binding_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct CalendarSettingResource<'a> {
    config: &'a Config,
}

impl<'a> CalendarSettingResource<'a> {
    pub async fn get(&self, option: &RequestOption) -> Result<GetCalendarSettingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/calendar/v4/settings",
            vec![AccessTokenType::User],
            option,
        )
        .send_response::<CalendarSetting, GetCalendarSettingResp>()
        .await
    }

    pub async fn patch(
        &self,
        body: &PatchCalendarSettingReqBody,
        option: &RequestOption,
    ) -> Result<PatchCalendarSettingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/calendar/v4/settings",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<CalendarSetting, PatchCalendarSettingResp>()
        .await
    }

    pub async fn generate_caldav_conf(
        &self,
        body: &GenerateCaldavConfReqBody,
        option: &RequestOption,
    ) -> Result<GenerateCaldavConfResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/settings/generate_caldav_conf",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<CaldavConfData, GenerateCaldavConfResp>()
        .await
    }
}

pub struct FreeBusyResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListFreeBusyQuery<'a> {
    pub body: &'a ListFreeBusyReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListFreeBusyQuery<'a> {
    pub fn new(body: &'a ListFreeBusyReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchFreeBusyQuery<'a> {
    pub body: &'a BatchFreeBusyReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchFreeBusyQuery<'a> {
    pub fn new(body: &'a BatchFreeBusyReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> FreeBusyResource<'a> {
    pub async fn list(
        &self,
        body: &ListFreeBusyReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFreeBusyResp, LarkError> {
        let query = ListFreeBusyQuery::new(body).user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFreeBusyQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFreeBusyResp, LarkError> {
        let request = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/freebusy/list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?;
        let (api_resp, raw) = request.send::<FreeBusyData>().await?;
        Ok(ListFreeBusyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch(
        &self,
        body: &BatchFreeBusyReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchFreeBusyResp, LarkError> {
        let query = BatchFreeBusyQuery::new(body).user_id_type(user_id_type);
        self.batch_by_query(&query, option).await
    }

    pub async fn batch_by_query(
        &self,
        query: &BatchFreeBusyQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchFreeBusyResp, LarkError> {
        let request = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/freebusy/batch",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?;
        let (api_resp, raw) = request.send::<BatchFreeBusyData>().await?;
        Ok(BatchFreeBusyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CalendarEventAttendeeChatMemberResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListChatMemberQuery<'a> {
    pub calendar_id: &'a str,
    pub event_id: &'a str,
    pub attendee_id: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListChatMemberQuery<'a> {
    pub fn new(calendar_id: &'a str, event_id: &'a str, attendee_id: &'a str) -> Self {
        Self {
            calendar_id,
            event_id,
            attendee_id,
            page: PageQuery::new(),
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> CalendarEventAttendeeChatMemberResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        calendar_id: &str,
        event_id: &str,
        attendee_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListChatMemberResp, LarkError> {
        let query = ListChatMemberQuery {
            calendar_id,
            event_id,
            attendee_id,
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListChatMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListChatMemberResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{}/events/{}/attendees/{}/chat_members",
            query.calendar_id, query.event_id, query.attendee_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_response::<ChatMemberListData, ListChatMemberResp>()
        .await
    }
}

pub struct CalendarEventMeetingChatResource<'a> {
    config: &'a Config,
}

impl<'a> CalendarEventMeetingChatResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        event_id: &str,
        option: &RequestOption,
    ) -> Result<CreateMeetingChatResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/meeting_chat"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<MeetingChatData, CreateMeetingChatResp>()
        .await
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        event_id: &str,
        meeting_chat_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/meeting_chat"
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("meeting_chat_id", meeting_chat_id)
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct CalendarEventMeetingMinuteResource<'a> {
    config: &'a Config,
}

impl<'a> CalendarEventMeetingMinuteResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        event_id: &str,
        option: &RequestOption,
    ) -> Result<CreateMeetingMinuteResp, LarkError> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/meeting_minute"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<MeetingMinuteData, CreateMeetingMinuteResp>()
        .await
    }
}

pub struct TimeoffEventResource<'a> {
    config: &'a Config,
}

impl<'a> TimeoffEventResource<'a> {
    pub async fn create(
        &self,
        body: &CreateTimeoffEventReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTimeoffEventResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/calendar/v4/timeoff_events",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<TimeoffEventData, CreateTimeoffEventResp>()
        .await
    }

    pub async fn delete(
        &self,
        timeoff_event_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/calendar/v4/timeoff_events/{timeoff_event_id}");
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<serde_json::Value>()
        .await?;

        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── Version struct ──

pub struct V4<'a> {
    pub calendar: CalendarResource<'a>,
    pub event: CalendarEventResource<'a>,
    pub attendee: CalendarAttendeeResource<'a>,
    pub acl: CalendarAclResource<'a>,
    pub timezone: TimeZoneResource<'a>,
    pub exchange_binding: ExchangeBindingResource<'a>,
    pub setting: CalendarSettingResource<'a>,
    pub freebusy: FreeBusyResource<'a>,
    pub attendee_chat_member: CalendarEventAttendeeChatMemberResource<'a>,
    pub meeting_chat: CalendarEventMeetingChatResource<'a>,
    pub meeting_minute: CalendarEventMeetingMinuteResource<'a>,
    pub timeoff_event: TimeoffEventResource<'a>,
}

impl<'a> V4<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            calendar: CalendarResource { config },
            event: CalendarEventResource { config },
            attendee: CalendarAttendeeResource { config },
            acl: CalendarAclResource { config },
            timezone: TimeZoneResource { config },
            exchange_binding: ExchangeBindingResource { config },
            setting: CalendarSettingResource { config },
            freebusy: FreeBusyResource { config },
            attendee_chat_member: CalendarEventAttendeeChatMemberResource { config },
            meeting_chat: CalendarEventMeetingChatResource { config },
            meeting_minute: CalendarEventMeetingMinuteResource { config },
            timeoff_event: TimeoffEventResource { config },
        }
    }
}
