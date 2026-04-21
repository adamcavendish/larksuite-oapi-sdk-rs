use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

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

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

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

impl<'a> CalendarResource<'a> {
    pub async fn create(
        &self,
        body: &CreateCalendarReqBody,
        option: &RequestOption,
    ) -> Result<CreateCalendarResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/calendar/v4/calendars");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<Calendar>(self.config, &api_req, option).await?;
        Ok(CreateCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(&self, calendar_id: &str, option: &RequestOption) -> Result<GetCalendarResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<Calendar>(self.config, &api_req, option).await?;
        Ok(GetCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        calendar_id: &str,
        body: &PatchCalendarReqBody,
        option: &RequestOption,
    ) -> Result<PatchCalendarResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<Calendar>(self.config, &api_req, option).await?;
        Ok(PatchCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, calendar_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<ListCalendarResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/calendar/v4/calendars");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = sync_token {
            api_req.query_params.set("sync_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CalendarListData>(self.config, &api_req, option).await?;
        Ok(ListCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn primary(
        &self,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetCalendarResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/primary",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<Calendar>(self.config, &api_req, option).await?;
        Ok(GetCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn mget(
        &self,
        body: &MgetCalendarReqBody,
        option: &RequestOption,
    ) -> Result<MgetCalendarResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/calendar/v4/calendars/mget");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MgetCalendarData>(self.config, &api_req, option).await?;
        Ok(MgetCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn primarys(
        &self,
        body: &PrimarysCalendarReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PrimarysCalendarResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/primarys",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<PrimarysCalendarData>(self.config, &api_req, option).await?;
        Ok(PrimarysCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn search(
        &self,
        body: &SearchCalendarReqBody,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<SearchCalendarResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SearchCalendarData>(self.config, &api_req, option).await?;
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
    ) -> Result<SubscribeCalendarResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/subscribe");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<SubscribeCalendarData>(self.config, &api_req, option)
                .await?;
        Ok(SubscribeCalendarResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn subscription(&self, option: &RequestOption) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/subscription",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscribe(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/unsubscribe");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscription(&self, option: &RequestOption) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/calendars/unsubscription",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct CalendarEventResource<'a> {
    config: &'a Config,
}

impl<'a> CalendarEventResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        body: &CreateEventReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateEventResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CalendarEventData>(self.config, &api_req, option).await?;
        Ok(CreateEventResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        calendar_id: &str,
        event_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetEventResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CalendarEventData>(self.config, &api_req, option).await?;
        Ok(GetEventResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &PatchEventReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchEventResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CalendarEventData>(self.config, &api_req, option).await?;
        Ok(PatchEventResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        event_id: &str,
        need_notification: Option<bool>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = need_notification {
            api_req.query_params.set("need_notification", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<ListEventResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = anchor_time {
            api_req.query_params.set("anchor_time", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = sync_token {
            api_req.query_params.set("sync_token", v);
        }
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CalendarEventListData>(self.config, &api_req, option)
                .await?;
        Ok(ListEventResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn instance_view(
        &self,
        calendar_id: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<InstanceViewResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/instance_view");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<InstanceViewData>(self.config, &api_req, option).await?;
        Ok(InstanceViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<InstancesResp> {
        let path =
            format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/instances");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<InstancesData>(self.config, &api_req, option).await?;
        Ok(InstancesResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn reply(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &ReplyEventReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<SearchEventResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/search");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SearchEventData>(self.config, &api_req, option).await?;
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
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/subscription");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscription(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/unsubscription");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct CalendarAttendeeResource<'a> {
    config: &'a Config,
}

impl<'a> CalendarAttendeeResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &CreateAttendeeReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateAttendeeResp> {
        let path =
            format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/attendees");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<AttendeeListData>(self.config, &api_req, option).await?;
        Ok(CreateAttendeeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        event_id: &str,
        body: &DeleteAttendeeReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/attendees/batch_delete"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<ListAttendeeResp> {
        let path =
            format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/attendees");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AttendeeListData>(self.config, &api_req, option).await?;
        Ok(ListAttendeeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CalendarAclResource<'a> {
    config: &'a Config,
}

impl<'a> CalendarAclResource<'a> {
    pub async fn create(
        &self,
        calendar_id: &str,
        body: &CreateAclReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateAclResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CalendarAcl>(self.config, &api_req, option).await?;
        Ok(CreateAclResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        acl_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/{acl_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<ListAclResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<AclListData>(self.config, &api_req, option).await?;
        Ok(ListAclResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn subscription(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/subscription");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscription(
        &self,
        calendar_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/unsubscription");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct TimeZoneResource<'a> {
    config: &'a Config,
}

impl<'a> TimeZoneResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTimeZoneResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/calendar/v4/timezones");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TimeZoneListData>(self.config, &api_req, option).await?;
        Ok(ListTimeZoneResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreateExchangeBindingResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/exchange_bindings",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ExchangeBinding>(self.config, &api_req, option).await?;
        Ok(CreateExchangeBindingResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        exchange_binding_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetExchangeBindingResp> {
        let path = format!("/open-apis/calendar/v4/exchange_bindings/{exchange_binding_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ExchangeBinding>(self.config, &api_req, option).await?;
        Ok(GetExchangeBindingResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        exchange_binding_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/exchange_bindings/{exchange_binding_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    pub async fn get(&self, option: &RequestOption) -> Result<GetCalendarSettingResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/calendar/v4/settings");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<CalendarSetting>(self.config, &api_req, option).await?;
        Ok(GetCalendarSettingResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        body: &PatchCalendarSettingReqBody,
        option: &RequestOption,
    ) -> Result<PatchCalendarSettingResp> {
        let mut api_req = ApiReq::new(http::Method::PATCH, "/open-apis/calendar/v4/settings");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CalendarSetting>(self.config, &api_req, option).await?;
        Ok(PatchCalendarSettingResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn generate_caldav_conf(
        &self,
        body: &GenerateCaldavConfReqBody,
        option: &RequestOption,
    ) -> Result<GenerateCaldavConfResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/settings/generate_caldav_conf",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CaldavConfData>(self.config, &api_req, option).await?;
        Ok(GenerateCaldavConfResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct FreeBusyResource<'a> {
    config: &'a Config,
}

impl<'a> FreeBusyResource<'a> {
    pub async fn list(
        &self,
        body: &ListFreeBusyReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFreeBusyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/calendar/v4/freebusy/batch_get",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FreeBusyData>(self.config, &api_req, option).await?;
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
    ) -> Result<BatchFreeBusyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/calendar/v4/freebusy/batch");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchFreeBusyData>(self.config, &api_req, option).await?;
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
    ) -> Result<ListChatMemberResp> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/attendees/{attendee_id}/chat_members"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ChatMemberListData>(self.config, &api_req, option).await?;
        Ok(ListChatMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreateMeetingChatResp> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/meeting_chat"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<MeetingChatData>(self.config, &api_req, option).await?;
        Ok(CreateMeetingChatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        calendar_id: &str,
        event_id: &str,
        meeting_chat_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/meeting_chat"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = meeting_chat_id {
            api_req.query_params.set("meeting_chat_id", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<CreateMeetingMinuteResp> {
        let path = format!(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/meeting_minute"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<MeetingMinuteData>(self.config, &api_req, option).await?;
        Ok(CreateMeetingMinuteResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreateTimeoffEventResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/calendar/v4/timeoff_events");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TimeoffEventData>(self.config, &api_req, option).await?;
        Ok(CreateTimeoffEventResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        timeoff_event_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/calendar/v4/timeoff_events/{timeoff_event_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
