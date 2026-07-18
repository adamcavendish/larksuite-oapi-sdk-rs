use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Room {
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
    pub room_status: Option<RoomStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Vec<Device>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_waiting_room: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_allow_call_in: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Device {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enrolled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_signage: Option<RoomDigitalSignage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_box_digital_signage: Option<RoomDigitalSignage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meeting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_user: Option<MeetingUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count_accumulated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<Participant>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability: Option<MeetingAbility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Participant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_host: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cohost: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_avg: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_meeting: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_meeting_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_meeting_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_participant_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_meeting_per_day: Option<Vec<crate::JsonValue>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRoomReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_room_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRoomReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_room_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Vec<Device>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SetRoomConfigReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_room_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_display_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_digital_signage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_signage: Option<RoomDigitalSignage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_room_box_digital_signage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_box_digital_signage: Option<RoomDigitalSignage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_room_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MgetRoomReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchRoomReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_room_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_level_name: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessCodeRoomConfigReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_day: Option<i32>,
}

pub type SetCheckboardAccessCodeRoomConfigReqBody = AccessCodeRoomConfigReqBody;
pub type SetRoomAccessCodeRoomConfigReqBody = AccessCodeRoomConfigReqBody;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingListExportReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParticipantListExportReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParticipantQualityListExportReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceReservationListExportReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_exclude: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteMeetingReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitees: Option<Vec<MeetingUser>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KickoutMeetingReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kickout_users: Option<Vec<MeetingUser>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetHostMeetingReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_user: Option<MeetingUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_host_user: Option<MeetingUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordingPermissionObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPermissionMeetingRecordingReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_objects: Option<Vec<RecordingPermissionObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartMeetingRecordingReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplyReserveReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_settings: Option<ReserveMeetingSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateReserveReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_settings: Option<ReserveMeetingSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchReserveConfigReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_config: Option<ApprovalConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_config: Option<TimeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_scope_config: Option<ReserveScopeConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchReserveConfigAdminReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_admin_config: Option<ReserveAdminConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchReserveConfigDisableInformReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_inform: Option<DisableInformConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchReserveConfigFormReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_form_config: Option<ReserveFormConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DelRoomLevelReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_child: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MgetRoomLevelReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_ids: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomListData {
    #[serde(default)]
    pub rooms: Vec<Room>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomConfigData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_config: Option<RoomConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingListData {
    #[serde(default)]
    pub meeting_briefs: Vec<Meeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParticipantListData {
    #[serde(default)]
    pub participants: Vec<Participant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingReportData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_report: Option<MeetingReport>,
}

impl_resp!(CreateRoomResp, RoomData);
impl_resp!(GetRoomResp, RoomData);
impl_resp!(ListRoomResp, RoomListData);
impl_resp!(GetRoomConfigResp, RoomConfigData);
impl_resp!(GetMeetingResp, MeetingData);
impl_resp!(ListMeetingResp, MeetingListData);
impl_resp!(ListParticipantResp, ParticipantListData);
impl_resp!(GetMeetingReportResp, MeetingReportData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MgetRoomRespData {
    #[serde(default)]
    pub items: Vec<ResponseRoom>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchRoomRespData {
    #[serde(default)]
    pub rooms: Vec<ResponseRoom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCheckboardAccessCodeRoomConfigRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetRoomAccessCodeRoomConfigRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryRoomConfigRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_signage: Option<RoomDigitalSignage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_box_digital_signage: Option<RoomDigitalSignage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<ResponseRoomStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<Alert>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetExportRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_msg: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMeetingRecordingRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recording: Option<MeetingRecording>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplyReserveRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve: Option<Reserve>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_correction_check_info: Option<ReserveCorrectionCheckInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReserveRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve: Option<Reserve>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetActiveMeetingReserveRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<ResponseMeeting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateReserveRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve: Option<Reserve>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_correction_check_info: Option<ReserveCorrectionCheckInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveScopeReserveConfigRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_config: Option<ApprovalConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_config: Option<TimeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_scope_config: Option<ReserveScopeConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReserveConfigAdminRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_admin_config: Option<ReserveAdminConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReserveConfigDisableInformRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_inform: Option<DisableInformConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReserveConfigFormRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_form_config: Option<ReserveFormConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRoomLevelRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level: Option<RoomLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRoomLevelRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level: Option<RoomLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRoomLevelRespData {
    #[serde(default)]
    pub items: Vec<RoomLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MgetRoomLevelRespData {
    #[serde(default)]
    pub items: Vec<RoomLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchRoomLevelRespData {
    #[serde(default)]
    pub level_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetScopeConfigRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_config: Option<ScopeConfig>,
    #[serde(default)]
    pub origin_configs: Vec<ScopeConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMeetingListRespData {
    #[serde(default)]
    pub meeting_list: Vec<MeetingInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetParticipantListRespData {
    #[serde(default)]
    pub participants: Vec<ResponseParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetParticipantQualityListRespData {
    #[serde(default)]
    pub participant_quality_list: Vec<ParticipantQuality>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetResourceReservationListRespData {
    #[serde(default)]
    pub room_reservation_list: Vec<RoomMeetingReservation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Alert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitor_target: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_strategy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_level: Option<i32>,
    #[serde(default)]
    pub contacts: Vec<Contact>,
    #[serde(default, rename = "notifyMethods")]
    pub notify_methods: Vec<i32>,
    #[serde(default, rename = "alertRule", skip_serializing_if = "Option::is_none")]
    pub alert_rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recover_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitor_target_room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitor_target_room_mac: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_switch: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_condition: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_duration: Option<f64>,
    #[serde(default)]
    pub approvers: Vec<SubscribeUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Conditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_key: Option<String>,
    #[serde(default)]
    pub option_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_fill: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub options: Vec<Options>,
    #[serde(default)]
    pub conditions: Vec<Conditions>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseDevice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisableInformConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_cover_child_scope: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_inform: Option<bool>,
    #[serde(default)]
    pub informed_users: Vec<SubscribeUser>,
    #[serde(default)]
    pub informed_depts: Vec<SubscribeDepartment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseMeeting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_user: Option<ResponseMeetingUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_connect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count_accumulated: Option<String>,
    #[serde(default)]
    pub participants: Vec<MeetingParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability: Option<MeetingAbility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingAbility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_video: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_audio: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_share_screen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_follow_screen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_recording: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_pstn: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_participants: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_devices: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sharing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recording: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone: Option<bool>,
    #[serde(default)]
    pub reserved_rooms: Vec<ReservedRoom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_related_document: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_note: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_subtype: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_webinar_viewers: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingParticipant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_join_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_leave_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_meeting_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_host: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cohost: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingRecording {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseMeetingUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Options {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_other: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseParticipant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_rtc_proxy: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub microphone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sharing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_meeting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webinar_user_role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParticipantQuality {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<QualityNetwork>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<QualityAudio>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<QualityVideoSharing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_sharing: Option<QualityVideoSharing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_usage: Option<QualityCpuUsage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PstnSipInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_address: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityAudio {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mic_input_volume: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker_volume: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter_sent: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityCpuUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_avg_cpu_usage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_max_cpu_usage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_avg_cpu_usage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_max_cpu_usage: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityNetwork {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_delay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packet_loss_avg_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packet_loss_max_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packet_loss_avg_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packet_loss_max_sent: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityVideoSharing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_resolution_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framerate_received: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_resolution_sent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framerate_sent: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reserve {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_settings: Option<ReserveMeetingSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveActionPermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(default)]
    pub permission_checkers: Vec<ReservePermissionChecker>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveAdminConfig {
    #[serde(default)]
    pub depts: Vec<SubscribeDepartment>,
    #[serde(default)]
    pub users: Vec<SubscribeUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveAssignHost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveCallSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callee: Option<ReserveCallee>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveCallee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pstn_sip_info: Option<PstnSipInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveCorrectionCheckInfo {
    #[serde(default)]
    pub invalid_host_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveFormConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_cover_child_scope: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserve_form: Option<bool>,
    #[serde(default)]
    pub notified_users: Vec<SubscribeUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notified_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<i32>,
    #[serde(default)]
    pub custom_list: Vec<CustomList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveMeetingSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default)]
    pub action_permissions: Vec<ReserveActionPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_initial_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_connect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_setting: Option<ReserveCallSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_record: Option<bool>,
    #[serde(default)]
    pub assign_host_list: Vec<ReserveAssignHost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReservePermissionChecker {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_field: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_mode: Option<i32>,
    #[serde(default)]
    pub check_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReserveScopeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_cover_child_scope: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_all_users: Option<i32>,
    #[serde(default)]
    pub allow_users: Vec<SubscribeUser>,
    #[serde(default)]
    pub allow_depts: Vec<SubscribeDepartment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReservedRoom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseRoom {
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
    #[serde(default)]
    pub path: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<ResponseRoomStatus>,
    #[serde(default)]
    pub device: Vec<ResponseDevice>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseRoomConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_signage: Option<RoomDigitalSignage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_box_digital_signage: Option<RoomDigitalSignage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<ResponseRoomStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomDigitalSignage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub if_cover_child_scope: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_display: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_display: Option<i32>,
    #[serde(default)]
    pub materials: Vec<RoomDigitalSignageMaterial>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomDigitalSignageMaterial {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub path: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_child: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomMeetingReservation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserver_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_of_reserver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guests_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reservation_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_check_in_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_release_early: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub releasing_person: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub releasing_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseRoomStatus {
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
    #[serde(default)]
    pub contact_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_notice: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_notice: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScopeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_config: Option<ResponseRoomConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscribeDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscribeUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
// ── Resources ──

pub struct RoomResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateRoomQuery<'a> {
    pub body: &'a CreateRoomReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateRoomQuery<'a> {
    pub fn new(body: &'a CreateRoomReqBody) -> Self {
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetRoomQuery<'a> {
    pub room_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetRoomQuery<'a> {
    pub fn new(room_id: &'a str) -> Self {
        Self {
            room_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UpdateRoomQuery<'a> {
    pub room_id: &'a str,
    pub body: &'a UpdateRoomReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateRoomQuery<'a> {
    pub fn new(room_id: &'a str, body: &'a UpdateRoomReqBody) -> Self {
        Self {
            room_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListRoomQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub room_level_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListRoomQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn room_level_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.room_level_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MgetRoomQuery<'a> {
    pub body: &'a MgetRoomReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> MgetRoomQuery<'a> {
    pub fn new(body: &'a MgetRoomReqBody) -> Self {
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SearchRoomQuery<'a> {
    pub body: &'a SearchRoomReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SearchRoomQuery<'a> {
    pub fn new(body: &'a SearchRoomReqBody) -> Self {
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

impl<'a> RoomResource<'a> {
    pub async fn create(
        &self,
        body: &CreateRoomReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateRoomResp, LarkError> {
        let query = CreateRoomQuery::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateRoomQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateRoomResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/rooms",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<RoomData, CreateRoomResp>()
        .await
    }

    pub async fn get(
        &self,
        room_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRoomResp, LarkError> {
        let query = GetRoomQuery::new(room_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetRoomQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetRoomResp, LarkError> {
        let path = format!("/open-apis/vc/v1/rooms/{}", query.room_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<RoomData, GetRoomResp>()
        .await
    }

    pub async fn update(
        &self,
        room_id: &str,
        body: &UpdateRoomReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = UpdateRoomQuery::new(room_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateRoomQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/rooms/{}", query.room_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_empty()
        .await
    }

    pub async fn delete(
        &self,
        room_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/rooms/{room_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        room_level_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRoomResp, LarkError> {
        let query = ListRoomQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .room_level_id(room_level_id)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListRoomQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListRoomResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/rooms",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("room_level_id", query.room_level_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<RoomListData, ListRoomResp>()
        .await
    }

    pub async fn mget(
        &self,
        body: MgetRoomReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<MgetRoomResp, LarkError> {
        let query = MgetRoomQuery::new(&body).user_id_type(user_id_type);
        self.mget_by_query(&query, option).await
    }

    pub async fn mget_by_query(
        &self,
        query: &MgetRoomQuery<'_>,
        option: &RequestOption,
    ) -> Result<MgetRoomResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/rooms/mget",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<MgetRoomRespData, MgetRoomResp>()
        .await
    }

    pub async fn search(
        &self,
        body: SearchRoomReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchRoomResp, LarkError> {
        let query = SearchRoomQuery::new(&body).user_id_type(user_id_type);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchRoomQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchRoomResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/rooms/search",
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<SearchRoomRespData, SearchRoomResp>()
        .await
    }
}

pub struct RoomConfigResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SetRoomConfigQuery<'a> {
    pub body: &'a SetRoomConfigReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SetRoomConfigQuery<'a> {
    pub fn new(body: &'a SetRoomConfigReqBody) -> Self {
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct RoomConfigQuery<'a> {
    pub scope: i32,
    pub country_id: Option<&'a str>,
    pub district_id: Option<&'a str>,
    pub building_id: Option<&'a str>,
    pub floor_name: Option<&'a str>,
    pub room_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> RoomConfigQuery<'a> {
    pub fn new(scope: i32) -> Self {
        Self {
            scope,
            country_id: None,
            district_id: None,
            building_id: None,
            floor_name: None,
            room_id: None,
            user_id_type: None,
        }
    }

    pub fn country_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.country_id = value.into();
        self
    }

    pub fn district_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.district_id = value.into();
        self
    }

    pub fn building_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.building_id = value.into();
        self
    }

    pub fn floor_name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.floor_name = value.into();
        self
    }

    pub fn room_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.room_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> RoomConfigResource<'a> {
    pub async fn get_by_query(
        &self,
        query: &RoomConfigQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetRoomConfigResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/room_configs/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("scope", query.scope)
        .query("country_id", query.country_id)
        .query("district_id", query.district_id)
        .query("building_id", query.building_id)
        .query("floor_name", query.floor_name)
        .query("room_id", query.room_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<RoomConfigData, GetRoomConfigResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        scope: i32,
        country_id: Option<&str>,
        district_id: Option<&str>,
        building_id: Option<&str>,
        floor_name: Option<&str>,
        room_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRoomConfigResp, LarkError> {
        let query = RoomConfigQuery::new(scope)
            .country_id(country_id)
            .district_id(district_id)
            .building_id(building_id)
            .floor_name(floor_name)
            .room_id(room_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn set(
        &self,
        body: &SetRoomConfigReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = SetRoomConfigQuery::new(body).user_id_type(user_id_type);
        self.set_by_query(&query, option).await
    }

    pub async fn set_by_query(
        &self,
        query: &SetRoomConfigQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/room_configs/set",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_empty()
        .await
    }

    pub async fn set_checkboard_access_code(
        &self,
        body: SetCheckboardAccessCodeRoomConfigReqBody,
        option: &RequestOption,
    ) -> Result<SetCheckboardAccessCodeRoomConfigResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/room_configs/set_checkboard_access_code",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<SetCheckboardAccessCodeRoomConfigRespData, SetCheckboardAccessCodeRoomConfigResp>()
        .await
    }

    pub async fn set_room_access_code(
        &self,
        body: SetRoomAccessCodeRoomConfigReqBody,
        option: &RequestOption,
    ) -> Result<SetRoomAccessCodeRoomConfigResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/room_configs/set_room_access_code",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<SetRoomAccessCodeRoomConfigRespData, SetRoomAccessCodeRoomConfigResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn query(
        &self,
        scope: i32,
        country_id: Option<&str>,
        district_id: Option<&str>,
        building_id: Option<&str>,
        floor_name: Option<&str>,
        room_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryRoomConfigResp, LarkError> {
        let query = RoomConfigQuery::new(scope)
            .country_id(country_id)
            .district_id(district_id)
            .building_id(building_id)
            .floor_name(floor_name)
            .room_id(room_id)
            .user_id_type(user_id_type);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &RoomConfigQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryRoomConfigResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/room_configs/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("scope", query.scope)
        .query("country_id", query.country_id)
        .query("district_id", query.district_id)
        .query("building_id", query.building_id)
        .query("floor_name", query.floor_name)
        .query("room_id", query.room_id)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<QueryRoomConfigRespData, QueryRoomConfigResp>()
        .await
    }
}

pub struct MeetingResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListMeetingByNoQuery<'a> {
    pub meeting_no: &'a str,
    pub start_time: &'a str,
    pub end_time: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListMeetingByNoQuery<'a> {
    pub fn new(meeting_no: &'a str, start_time: &'a str, end_time: &'a str) -> Self {
        Self {
            meeting_no,
            start_time,
            end_time,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> MeetingResource<'a> {
    pub async fn get(
        &self,
        meeting_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_response::<MeetingData, GetMeetingResp>()
        .await
    }

    pub async fn invite(
        &self,
        meeting_id: &str,
        body: InviteMeetingReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/invite");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_empty()
        .await
    }

    pub async fn kickout(
        &self,
        meeting_id: &str,
        body: KickoutMeetingReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/kickout");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_empty()
        .await
    }

    pub async fn set_host(
        &self,
        meeting_id: &str,
        body: SetHostMeetingReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/set_host");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_empty()
        .await
    }

    pub async fn end(
        &self,
        meeting_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/end");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list_by_no(
        &self,
        meeting_no: &str,
        start_time: &str,
        end_time: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMeetingResp, LarkError> {
        let query = ListMeetingByNoQuery::new(meeting_no, start_time, end_time)
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_no_query(&query, option).await
    }

    pub async fn list_by_no_query(
        &self,
        query: &ListMeetingByNoQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListMeetingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/meetings/list_by_no",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("meeting_no", query.meeting_no)
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .send_response::<MeetingListData, ListMeetingResp>()
        .await
    }
}

pub struct ParticipantResource<'a> {
    config: &'a Config,
}

impl<'a> ParticipantResource<'a> {
    pub async fn list(
        &self,
        meeting_id: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListParticipantResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/participants");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_response::<ParticipantListData, ListParticipantResp>()
        .await
    }
}

pub struct ReportResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetTopUserReportQuery<'a> {
    pub start_time: &'a str,
    pub end_time: &'a str,
    pub limit: i32,
    pub order_by: i32,
    pub meeting_type: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTopUserReportQuery<'a> {
    pub fn new(start_time: &'a str, end_time: &'a str, limit: i32, order_by: i32) -> Self {
        Self {
            start_time,
            end_time,
            limit,
            order_by,
            meeting_type: None,
            user_id_type: None,
        }
    }

    pub fn meeting_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.meeting_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ReportResource<'a> {
    pub async fn get_daily(
        &self,
        start_time: &str,
        end_time: &str,
        meeting_type: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingReportResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/reports/get_daily",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("start_time", start_time)
        .query("end_time", end_time)
        .query("meeting_type", meeting_type)
        .query("user_id_type", user_id_type)
        .send_response::<MeetingReportData, GetMeetingReportResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_top_user(
        &self,
        start_time: &str,
        end_time: &str,
        limit: i32,
        order_by: i32,
        meeting_type: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = GetTopUserReportQuery::new(start_time, end_time, limit, order_by)
            .meeting_type(meeting_type)
            .user_id_type(user_id_type);
        self.get_top_user_by_query(&query, option).await
    }

    pub async fn get_top_user_by_query(
        &self,
        query: &GetTopUserReportQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/reports/get_top_user",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("limit", query.limit)
        .query("order_by", query.order_by)
        .query("meeting_type", query.meeting_type)
        .query("user_id_type", query.user_id_type)
        .send_empty()
        .await
    }
}

// ── impl_resp_v2! macro ──

impl_resp_v2!(MgetRoomResp, MgetRoomRespData);
impl_resp_v2!(SearchRoomResp, SearchRoomRespData);
impl_resp_v2!(
    SetCheckboardAccessCodeRoomConfigResp,
    SetCheckboardAccessCodeRoomConfigRespData
);
impl_resp_v2!(
    SetRoomAccessCodeRoomConfigResp,
    SetRoomAccessCodeRoomConfigRespData
);
impl_resp_v2!(QueryRoomConfigResp, QueryRoomConfigRespData);

// ── Alert response types ──

impl_resp_v2!(ListAlertResp, ListAlertRespData);

// ── Export response types ──

impl_resp_v2!(ExportMeetingListResp, ());
impl_resp_v2!(ExportParticipantListResp, ());
impl_resp_v2!(ExportParticipantQualityListResp, ());
impl_resp_v2!(ExportResourceReservationListResp, ());
impl_resp_v2!(GetExportResp, GetExportRespData);
impl_resp_v2!(DownloadExportResp, ());

// ── MeetingRecording response types ──

impl_resp_v2!(GetMeetingRecordingResp, GetMeetingRecordingRespData);
impl_resp_v2!(SetPermissionMeetingRecordingResp, ());
impl_resp_v2!(StartMeetingRecordingResp, ());
impl_resp_v2!(StopMeetingRecordingResp, ());

// ── Reserve response types ──

impl_resp_v2!(ApplyReserveResp, ApplyReserveRespData);
impl_resp_v2!(DeleteReserveResp, ());
impl_resp_v2!(GetReserveResp, GetReserveRespData);
impl_resp_v2!(GetActiveMeetingReserveResp, GetActiveMeetingReserveRespData);
impl_resp_v2!(UpdateReserveResp, UpdateReserveRespData);

// ── ReserveConfig response types ──

impl_resp_v2!(PatchReserveConfigResp, ());
impl_resp_v2!(
    ReserveScopeReserveConfigResp,
    ReserveScopeReserveConfigRespData
);
impl_resp_v2!(GetReserveConfigAdminResp, GetReserveConfigAdminRespData);
impl_resp_v2!(PatchReserveConfigAdminResp, ());
impl_resp_v2!(
    GetReserveConfigDisableInformResp,
    GetReserveConfigDisableInformRespData
);
impl_resp_v2!(PatchReserveConfigDisableInformResp, ());
impl_resp_v2!(GetReserveConfigFormResp, GetReserveConfigFormRespData);
impl_resp_v2!(PatchReserveConfigFormResp, ());

// ── RoomLevel response types ──

impl_resp_v2!(CreateRoomLevelResp, CreateRoomLevelRespData);
impl_resp_v2!(DelRoomLevelResp, ());
impl_resp_v2!(GetRoomLevelResp, GetRoomLevelRespData);
impl_resp_v2!(ListRoomLevelResp, ListRoomLevelRespData);
impl_resp_v2!(MgetRoomLevelResp, MgetRoomLevelRespData);
impl_resp_v2!(PatchRoomLevelResp, ());
impl_resp_v2!(SearchRoomLevelResp, SearchRoomLevelRespData);

// ── ScopeConfig response types ──

impl_resp_v2!(CreateScopeConfigResp, ());
impl_resp_v2!(GetScopeConfigResp, GetScopeConfigRespData);

// ── MeetingList / ParticipantList / ParticipantQualityList / ResourceReservationList response types ──

impl_resp_v2!(GetMeetingListResp, GetMeetingListRespData);
impl_resp_v2!(GetParticipantListResp, GetParticipantListRespData);
impl_resp_v2!(
    GetParticipantQualityListResp,
    GetParticipantQualityListRespData
);
impl_resp_v2!(
    GetResourceReservationListResp,
    GetResourceReservationListRespData
);

// ── AlertResource ──

pub struct AlertResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListAlertQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub start_time: Option<&'a str>,
    pub end_time: Option<&'a str>,
    pub query_type: Option<i32>,
    pub query_value: Option<&'a str>,
}

impl<'a> ListAlertQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }

    pub fn start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_time = value.into();
        self
    }

    pub fn end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_time = value.into();
        self
    }

    pub fn query_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.query_type = value.into();
        self
    }

    pub fn query_value(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.query_value = value.into();
        self
    }
}

impl<'a> AlertResource<'a> {
    /// GET /open-apis/vc/v1/alerts — 获取告警记录
    pub async fn list_by_query(
        &self,
        query: &ListAlertQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAlertResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/alerts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("query_type", query.query_type)
        .query("query_value", query.query_value)
        .send_v2_response::<ListAlertRespData, ListAlertResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        query_type: Option<i32>,
        query_value: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAlertResp, LarkError> {
        let query = ListAlertQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .start_time(start_time)
            .end_time(end_time)
            .query_type(query_type)
            .query_value(query_value);
        self.list_by_query(&query, option).await
    }
}

// ── ExportResource ──

pub struct ExportResource<'a> {
    config: &'a Config,
}

impl<'a> ExportResource<'a> {
    /// POST /open-apis/vc/v1/exports/meeting_list — 导出会议明细
    pub async fn meeting_list(
        &self,
        body: MeetingListExportReqBody,
        option: &RequestOption,
    ) -> Result<ExportMeetingListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/exports/meeting_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), ExportMeetingListResp>()
        .await
    }

    /// POST /open-apis/vc/v1/exports/participant_list — 导出参会人明细
    pub async fn participant_list(
        &self,
        body: ParticipantListExportReqBody,
        option: &RequestOption,
    ) -> Result<ExportParticipantListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/exports/participant_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), ExportParticipantListResp>()
        .await
    }

    /// POST /open-apis/vc/v1/exports/participant_quality_list — 导出参会人会议质量数据
    pub async fn participant_quality_list(
        &self,
        body: ParticipantQualityListExportReqBody,
        option: &RequestOption,
    ) -> Result<ExportParticipantQualityListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/exports/participant_quality_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), ExportParticipantQualityListResp>()
        .await
    }

    /// POST /open-apis/vc/v1/exports/resource_reservation_list — 导出会议室预定数据
    pub async fn resource_reservation_list(
        &self,
        body: ResourceReservationListExportReqBody,
        option: &RequestOption,
    ) -> Result<ExportResourceReservationListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/exports/resource_reservation_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), ExportResourceReservationListResp>()
        .await
    }

    /// GET /open-apis/vc/v1/exports/:task_id — 查询导出任务结果
    pub async fn get(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<GetExportResp, LarkError> {
        let path = format!("/open-apis/vc/v1/exports/{task_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<GetExportRespData, GetExportResp>()
        .await
    }

    /// GET /open-apis/vc/v1/exports/download — 下载导出文件
    pub async fn download(
        &self,
        file_token: &str,
        option: &RequestOption,
    ) -> Result<DownloadExportResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/exports/download",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("file_token", file_token)
        .send_v2_response::<(), DownloadExportResp>()
        .await
    }
}

// ── MeetingRecordingResource ──

pub struct MeetingRecordingResource<'a> {
    config: &'a Config,
}

impl<'a> MeetingRecordingResource<'a> {
    /// GET /open-apis/vc/v1/meetings/:meeting_id/recording — 获取录制文件
    pub async fn get(
        &self,
        meeting_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_v2_response::<GetMeetingRecordingRespData, GetMeetingRecordingResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/set_permission — 授权录制文件
    pub async fn set_permission(
        &self,
        meeting_id: &str,
        body: SetPermissionMeetingRecordingReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SetPermissionMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/set_permission");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_v2_response::<(), SetPermissionMeetingRecordingResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/start — 开始录制
    pub async fn start(
        &self,
        meeting_id: &str,
        body: StartMeetingRecordingReqBody,
        option: &RequestOption,
    ) -> Result<StartMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/start");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), StartMeetingRecordingResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/stop — 停止录制
    pub async fn stop(
        &self,
        meeting_id: &str,
        option: &RequestOption,
    ) -> Result<StopMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/stop");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), StopMeetingRecordingResp>()
        .await
    }
}

// ── ReserveResource ──

pub struct ReserveResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ApplyReserveQuery<'a> {
    pub body: &'a ApplyReserveReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ApplyReserveQuery<'a> {
    pub fn new(body: &'a ApplyReserveReqBody) -> Self {
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetReserveQuery<'a> {
    pub reserve_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetReserveQuery<'a> {
    pub fn new(reserve_id: &'a str) -> Self {
        Self {
            reserve_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetActiveMeetingReserveQuery<'a> {
    pub reserve_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetActiveMeetingReserveQuery<'a> {
    pub fn new(reserve_id: &'a str) -> Self {
        Self {
            reserve_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UpdateReserveQuery<'a> {
    pub reserve_id: &'a str,
    pub body: &'a UpdateReserveReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateReserveQuery<'a> {
    pub fn new(reserve_id: &'a str, body: &'a UpdateReserveReqBody) -> Self {
        Self {
            reserve_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ReserveResource<'a> {
    /// POST /open-apis/vc/v1/reserves/apply — 预约会议
    pub async fn apply(
        &self,
        body: ApplyReserveReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ApplyReserveResp, LarkError> {
        let query = ApplyReserveQuery::new(&body).user_id_type(user_id_type);
        self.apply_by_query(&query, option).await
    }

    pub async fn apply_by_query(
        &self,
        query: &ApplyReserveQuery<'_>,
        option: &RequestOption,
    ) -> Result<ApplyReserveResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/reserves/apply",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<ApplyReserveRespData, ApplyReserveResp>()
        .await
    }

    /// DELETE /open-apis/vc/v1/reserves/:reserve_id — 删除预约
    pub async fn delete(
        &self,
        reserve_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteReserveResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserves/{reserve_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), DeleteReserveResp>()
        .await
    }

    /// GET /open-apis/vc/v1/reserves/:reserve_id — 获取预约
    pub async fn get(
        &self,
        reserve_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveResp, LarkError> {
        let query = GetReserveQuery::new(reserve_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetReserveQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetReserveResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserves/{}", query.reserve_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetReserveRespData, GetReserveResp>()
        .await
    }

    /// GET /open-apis/vc/v1/reserves/:reserve_id/get_active_meeting — 获取活跃会议
    pub async fn get_active_meeting(
        &self,
        reserve_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetActiveMeetingReserveResp, LarkError> {
        let query = GetActiveMeetingReserveQuery::new(reserve_id).user_id_type(user_id_type);
        self.get_active_meeting_by_query(&query, option).await
    }

    pub async fn get_active_meeting_by_query(
        &self,
        query: &GetActiveMeetingReserveQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetActiveMeetingReserveResp, LarkError> {
        let path = format!(
            "/open-apis/vc/v1/reserves/{}/get_active_meeting",
            query.reserve_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetActiveMeetingReserveRespData, GetActiveMeetingReserveResp>()
        .await
    }

    /// PUT /open-apis/vc/v1/reserves/:reserve_id — 更新预约
    pub async fn update(
        &self,
        reserve_id: &str,
        body: UpdateReserveReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateReserveResp, LarkError> {
        let query = UpdateReserveQuery::new(reserve_id, &body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateReserveQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateReserveResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserves/{}", query.reserve_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<UpdateReserveRespData, UpdateReserveResp>()
        .await
    }
}

// ── ReserveConfigResource ──

pub struct ReserveConfigResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/reserve_scope — 获取会议室预定范围
    pub async fn reserve_scope(
        &self,
        scope_type: i32,
        scope_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ReserveScopeReserveConfigResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/reserve_configs/reserve_scope",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("scope_type", scope_type)
        .query("scope_id", scope_id)
        .query("user_id_type", user_id_type)
        .send_v2_response::<ReserveScopeReserveConfigRespData, ReserveScopeReserveConfigResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id — 更新会议室预定范围
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: PatchReserveConfigReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_v2_response::<(), PatchReserveConfigResp>()
        .await
    }
}

// ── ReserveConfigAdminResource ──

pub struct ReserveConfigAdminResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigAdminResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin
    pub async fn get(
        &self,
        reserve_config_id: &str,
        scope_type: i32,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveConfigAdminResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/admin");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("scope_type", scope_type)
        .query("user_id_type", user_id_type)
        .send_v2_response::<GetReserveConfigAdminRespData, GetReserveConfigAdminResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: PatchReserveConfigAdminReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigAdminResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/admin");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_v2_response::<(), PatchReserveConfigAdminResp>()
        .await
    }
}

// ── ReserveConfigDisableInformResource ──

pub struct ReserveConfigDisableInformResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigDisableInformResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
    pub async fn get(
        &self,
        reserve_config_id: &str,
        scope_type: i32,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveConfigDisableInformResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/disable_inform");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("scope_type", scope_type)
        .query("user_id_type", user_id_type)
        .send_v2_response::<GetReserveConfigDisableInformRespData, GetReserveConfigDisableInformResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: PatchReserveConfigDisableInformReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigDisableInformResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/disable_inform");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_v2_response::<(), PatchReserveConfigDisableInformResp>()
        .await
    }
}

// ── ReserveConfigFormResource ──

pub struct ReserveConfigFormResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigFormResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id/form
    pub async fn get(
        &self,
        reserve_config_id: &str,
        scope_type: i32,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveConfigFormResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/form");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("scope_type", scope_type)
        .query("user_id_type", user_id_type)
        .send_v2_response::<GetReserveConfigFormRespData, GetReserveConfigFormResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/form
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: PatchReserveConfigFormReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigFormResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/form");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_v2_response::<(), PatchReserveConfigFormResp>()
        .await
    }
}

// ── RoomLevelResource ──

pub struct RoomLevelResource<'a> {
    config: &'a Config,
}

impl<'a> RoomLevelResource<'a> {
    /// POST /open-apis/vc/v1/room_levels — 创建会议室层级
    pub async fn create(
        &self,
        body: RoomLevel,
        option: &RequestOption,
    ) -> Result<CreateRoomLevelResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/room_levels",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<CreateRoomLevelRespData, CreateRoomLevelResp>()
        .await
    }

    /// POST /open-apis/vc/v1/room_levels/del — 删除会议室层级
    pub async fn del(
        &self,
        body: DelRoomLevelReqBody,
        option: &RequestOption,
    ) -> Result<DelRoomLevelResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/room_levels/del",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), DelRoomLevelResp>()
        .await
    }

    /// GET /open-apis/vc/v1/room_levels/:room_level_id — 查询会议室层级详情
    pub async fn get(
        &self,
        room_level_id: &str,
        option: &RequestOption,
    ) -> Result<GetRoomLevelResp, LarkError> {
        let path = format!("/open-apis/vc/v1/room_levels/{room_level_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetRoomLevelRespData, GetRoomLevelResp>()
        .await
    }

    /// GET /open-apis/vc/v1/room_levels — 查询会议室层级列表
    pub async fn list(
        &self,
        room_level_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRoomLevelResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/room_levels",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("room_level_id", room_level_id)
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<ListRoomLevelRespData, ListRoomLevelResp>()
        .await
    }

    /// POST /open-apis/vc/v1/room_levels/mget — 批量查询会议室层级详情
    pub async fn mget(
        &self,
        body: MgetRoomLevelReqBody,
        option: &RequestOption,
    ) -> Result<MgetRoomLevelResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/room_levels/mget",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<MgetRoomLevelRespData, MgetRoomLevelResp>()
        .await
    }

    /// PATCH /open-apis/vc/v1/room_levels/:room_level_id — 更新会议室层级
    pub async fn patch(
        &self,
        room_level_id: &str,
        body: RoomLevel,
        option: &RequestOption,
    ) -> Result<PatchRoomLevelResp, LarkError> {
        let path = format!("/open-apis/vc/v1/room_levels/{room_level_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<(), PatchRoomLevelResp>()
        .await
    }

    /// GET /open-apis/vc/v1/room_levels/search — 搜索会议室层级
    pub async fn search(
        &self,
        custom_level_ids: &str,
        option: &RequestOption,
    ) -> Result<SearchRoomLevelResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/room_levels/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("custom_level_ids", custom_level_ids)
        .send_v2_response::<SearchRoomLevelRespData, SearchRoomLevelResp>()
        .await
    }
}

// ── ScopeConfigResource ──

pub struct ScopeConfigResource<'a> {
    config: &'a Config,
}

impl<'a> ScopeConfigResource<'a> {
    /// POST /open-apis/vc/v1/scope_config — 设置会议室配置
    pub async fn create(
        &self,
        body: ScopeConfig,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateScopeConfigResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/vc/v1/scope_config",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(&body)?
        .send_v2_response::<(), CreateScopeConfigResp>()
        .await
    }

    /// GET /open-apis/vc/v1/scope_config — 查询会议室配置
    pub async fn get(
        &self,
        scope_type: i32,
        scope_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetScopeConfigResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/scope_config",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("scope_type", scope_type)
        .query("scope_id", scope_id)
        .query("user_id_type", user_id_type)
        .send_v2_response::<GetScopeConfigRespData, GetScopeConfigResp>()
        .await
    }
}

// ── MeetingListResource ──

pub struct MeetingListResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetMeetingListQuery<'a> {
    pub start_time: &'a str,
    pub end_time: &'a str,
    pub meeting_status: Option<i32>,
    pub meeting_no: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub room_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetMeetingListQuery<'a> {
    pub fn new(start_time: &'a str, end_time: &'a str) -> Self {
        Self {
            start_time,
            end_time,
            meeting_status: None,
            meeting_no: None,
            user_id: None,
            room_id: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn meeting_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.meeting_status = value.into();
        self
    }

    pub fn meeting_no(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.meeting_no = value.into();
        self
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn room_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.room_id = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> MeetingListResource<'a> {
    /// GET /open-apis/vc/v1/meeting_list
    pub async fn get_by_query(
        &self,
        query: &GetMeetingListQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMeetingListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/meeting_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("meeting_status", query.meeting_status)
        .query("meeting_no", query.meeting_no)
        .query("user_id", query.user_id)
        .query("room_id", query.room_id)
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetMeetingListRespData, GetMeetingListResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        start_time: &str,
        end_time: &str,
        meeting_status: Option<i32>,
        meeting_no: Option<&str>,
        user_id: Option<&str>,
        room_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingListResp, LarkError> {
        let query = GetMeetingListQuery::new(start_time, end_time)
            .meeting_status(meeting_status)
            .meeting_no(meeting_no)
            .user_id(user_id)
            .room_id(room_id)
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }
}

// ── ParticipantListResource ──

pub struct ParticipantListResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetParticipantListQuery<'a> {
    pub meeting_start_time: &'a str,
    pub meeting_end_time: &'a str,
    pub meeting_no: &'a str,
    pub meeting_status: Option<i32>,
    pub user_id: Option<&'a str>,
    pub room_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetParticipantListQuery<'a> {
    pub fn new(
        meeting_start_time: &'a str,
        meeting_end_time: &'a str,
        meeting_no: &'a str,
    ) -> Self {
        Self {
            meeting_start_time,
            meeting_end_time,
            meeting_no,
            meeting_status: None,
            user_id: None,
            room_id: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn meeting_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.meeting_status = value.into();
        self
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn room_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.room_id = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ParticipantListResource<'a> {
    /// GET /open-apis/vc/v1/participant_list
    pub async fn get_by_query(
        &self,
        query: &GetParticipantListQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetParticipantListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/participant_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("meeting_start_time", query.meeting_start_time)
        .query("meeting_end_time", query.meeting_end_time)
        .query("meeting_no", query.meeting_no)
        .query("meeting_status", query.meeting_status)
        .query("user_id", query.user_id)
        .query("room_id", query.room_id)
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetParticipantListRespData, GetParticipantListResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        meeting_start_time: &str,
        meeting_end_time: &str,
        meeting_status: Option<i32>,
        meeting_no: &str,
        user_id: Option<&str>,
        room_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetParticipantListResp, LarkError> {
        let query = GetParticipantListQuery::new(meeting_start_time, meeting_end_time, meeting_no)
            .meeting_status(meeting_status)
            .user_id(user_id)
            .room_id(room_id)
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }
}

// ── ParticipantQualityListResource ──

pub struct ParticipantQualityListResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetParticipantQualityListQuery<'a> {
    pub meeting_start_time: &'a str,
    pub meeting_end_time: &'a str,
    pub meeting_no: &'a str,
    pub join_time: &'a str,
    pub user_id: Option<&'a str>,
    pub room_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetParticipantQualityListQuery<'a> {
    pub fn new(
        meeting_start_time: &'a str,
        meeting_end_time: &'a str,
        meeting_no: &'a str,
        join_time: &'a str,
    ) -> Self {
        Self {
            meeting_start_time,
            meeting_end_time,
            meeting_no,
            join_time,
            user_id: None,
            room_id: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn room_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.room_id = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ParticipantQualityListResource<'a> {
    /// GET /open-apis/vc/v1/participant_quality_list
    pub async fn get_by_query(
        &self,
        query: &GetParticipantQualityListQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetParticipantQualityListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/participant_quality_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("meeting_start_time", query.meeting_start_time)
        .query("meeting_end_time", query.meeting_end_time)
        .query("meeting_no", query.meeting_no)
        .query("join_time", query.join_time)
        .query("user_id", query.user_id)
        .query("room_id", query.room_id)
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetParticipantQualityListRespData, GetParticipantQualityListResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        meeting_start_time: &str,
        meeting_end_time: &str,
        meeting_no: &str,
        join_time: &str,
        user_id: Option<&str>,
        room_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetParticipantQualityListResp, LarkError> {
        let query = GetParticipantQualityListQuery::new(
            meeting_start_time,
            meeting_end_time,
            meeting_no,
            join_time,
        )
        .user_id(user_id)
        .room_id(room_id)
        .page_size(page_size)
        .page_token(page_token)
        .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }
}

// ── ResourceReservationListResource ──

pub struct ResourceReservationListResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetResourceReservationListQuery<'a> {
    pub room_level_id: &'a str,
    pub start_time: &'a str,
    pub end_time: &'a str,
    pub has_video: Option<bool>,
    pub room_ids: Option<&'a str>,
    pub is_exclude: Option<bool>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> GetResourceReservationListQuery<'a> {
    pub fn new(room_level_id: &'a str, start_time: &'a str, end_time: &'a str) -> Self {
        Self {
            room_level_id,
            start_time,
            end_time,
            has_video: None,
            room_ids: None,
            is_exclude: None,
            page_size: None,
            page_token: None,
        }
    }

    pub fn has_video(mut self, value: impl Into<Option<bool>>) -> Self {
        self.has_video = value.into();
        self
    }

    pub fn room_ids(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.room_ids = value.into();
        self
    }

    pub fn is_exclude(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_exclude = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> ResourceReservationListResource<'a> {
    /// GET /open-apis/vc/v1/resource_reservation_list
    pub async fn get_by_query(
        &self,
        query: &GetResourceReservationListQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetResourceReservationListResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/vc/v1/resource_reservation_list",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("room_level_id", query.room_level_id)
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("has_video", query.has_video)
        .query("room_ids", query.room_ids)
        .query("is_exclude", query.is_exclude)
        .page_query(query.page_query())
        .send_v2_response::<GetResourceReservationListRespData, GetResourceReservationListResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        room_level_id: &str,
        has_video: Option<bool>,
        start_time: &str,
        end_time: &str,
        room_ids: Option<&str>,
        is_exclude: Option<bool>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetResourceReservationListResp, LarkError> {
        let query = GetResourceReservationListQuery::new(room_level_id, start_time, end_time)
            .has_video(has_video)
            .room_ids(room_ids)
            .is_exclude(is_exclude)
            .page_size(page_size)
            .page_token(page_token);
        self.get_by_query(&query, option).await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub room: RoomResource<'a>,
    pub room_config: RoomConfigResource<'a>,
    pub meeting: MeetingResource<'a>,
    pub participant: ParticipantResource<'a>,
    pub report: ReportResource<'a>,
    pub alert: AlertResource<'a>,
    pub export: ExportResource<'a>,
    pub meeting_recording: MeetingRecordingResource<'a>,
    pub reserve: ReserveResource<'a>,
    pub reserve_config: ReserveConfigResource<'a>,
    pub reserve_config_admin: ReserveConfigAdminResource<'a>,
    pub reserve_config_disable_inform: ReserveConfigDisableInformResource<'a>,
    pub reserve_config_form: ReserveConfigFormResource<'a>,
    pub room_level: RoomLevelResource<'a>,
    pub scope_config: ScopeConfigResource<'a>,
    pub meeting_list: MeetingListResource<'a>,
    pub participant_list: ParticipantListResource<'a>,
    pub participant_quality_list: ParticipantQualityListResource<'a>,
    pub resource_reservation_list: ResourceReservationListResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            room: RoomResource { config },
            room_config: RoomConfigResource { config },
            meeting: MeetingResource { config },
            participant: ParticipantResource { config },
            report: ReportResource { config },
            alert: AlertResource { config },
            export: ExportResource { config },
            meeting_recording: MeetingRecordingResource { config },
            reserve: ReserveResource { config },
            reserve_config: ReserveConfigResource { config },
            reserve_config_admin: ReserveConfigAdminResource { config },
            reserve_config_disable_inform: ReserveConfigDisableInformResource { config },
            reserve_config_form: ReserveConfigFormResource { config },
            room_level: RoomLevelResource { config },
            scope_config: ScopeConfigResource { config },
            meeting_list: MeetingListResource { config },
            participant_list: ParticipantListResource { config },
            participant_quality_list: ParticipantQualityListResource { config },
            resource_reservation_list: ResourceReservationListResource { config },
        }
    }
}
