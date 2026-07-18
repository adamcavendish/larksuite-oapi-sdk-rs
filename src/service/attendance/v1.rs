use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{DownloadResp, EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gps_range: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FreeClockSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_mode: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_internal_hhmm: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FreePunchCfg {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_day: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_day_no_punch_as_lack: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_hours_demand: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_clock_setting: Option<FreeClockSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PunchSpecialDateShift {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_day: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttendanceGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind_dept_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub except_dept_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind_user_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub except_user_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_leader_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_group_leader_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_out_punch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_punch_need_approval: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_punch_need_remark: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_punch_need_photo: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_punch_allowed_hide_addr: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_pc_punch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_remedy_punch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_punch_approval: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_open_days: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixshift_effect_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_day_shift_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_punch_cfg: Option<FreePunchCfg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_punch_special_days: Option<Vec<PunchSpecialDateShift>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_punch_special_days: Option<Vec<PunchSpecialDateShift>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_day_no_punch_as_lack: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_now: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_period_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_period_custom_date: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_type_config: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_cumulative_time: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_over_time: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Shift {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_times: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_rule: Option<Vec<FlexibleRule>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_off: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_time_rule: Option<Vec<PunchTimeRule>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_lack: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_early: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_lack: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub break_time_rule: Option<Vec<RestRule>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_serious_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_punch_shift_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overtime_rule: Option<Vec<OvertimeRule>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overtime_rest_time_need_pump: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overtime_spec_work_type_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_key_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_in_face_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_in_face_key_update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveReportData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_datas: Option<Vec<ArchiveFieldData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttendanceRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_record: Option<UserFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out_record: Option<UserFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveAccrualRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<LangText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_type: Option<i32>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOrUpdateGroupReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<AttendanceGroup>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateShiftReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_times: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_time_rule: Option<Vec<PunchTimeRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub break_time_rule: Option<Vec<RestRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_rule: Option<Vec<OvertimeRule>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchGetUserSettingReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_from: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_to: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_overtime_result: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetApprovalInfoReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_field_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_overtime_result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<AttendanceGroup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupListData {
    #[serde(default)]
    pub group_list: Vec<AttendanceGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShiftData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift: Option<Shift>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSettingListData {
    #[serde(default)]
    pub user_settings: Vec<UserSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordListData {
    #[serde(default)]
    pub user_datas: Vec<AttendanceRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalid_user_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_user_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalInfoListData {
    #[serde(default)]
    pub user_datas: Vec<ApprovalInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveAccrualListData {
    #[serde(default)]
    pub records: Vec<LeaveAccrualRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateGroupResp, GroupData);
impl_resp!(GetGroupResp, GroupData);
impl_resp!(ListGroupResp, GroupListData);
impl_resp!(CreateShiftResp, ShiftData);
impl_resp!(GetShiftResp, ShiftData);
impl_resp!(BatchGetUserSettingResp, UserSettingListData);
impl_resp!(QueryRecordResp, RecordListData);
impl_resp!(GetApprovalInfoResp, ApprovalInfoListData);
impl_resp!(ListLeaveAccrualResp, LeaveAccrualListData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadReportArchiveRuleRespData {
    #[serde(default)]
    pub invalid_code: Vec<String>,
    #[serde(default)]
    pub invalid_member_id: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatsFieldsQueryArchiveRuleRespData {
    #[serde(default)]
    pub archive_report_fields: Vec<ArchiveField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchLeaveAccrualRecordRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<ResponseLeaveAccrualRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateUserApprovalRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_approval: Option<UserApproval>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserApprovalRespData {
    #[serde(default)]
    pub user_approvals: Vec<UserApproval>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateUserDailyShiftRespData {
    #[serde(default)]
    pub user_daily_shifts: Vec<UserDailyShift>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateTempUserDailyShiftRespData {
    #[serde(default)]
    pub user_tmp_daily_shifts: Vec<UserTmpDailyShift>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserDailyShiftRespData {
    #[serde(default)]
    pub user_daily_shifts: Vec<UserDailyShift>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateUserFlowRespData {
    #[serde(default)]
    pub flow_records: Vec<UserFlow>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDelUserFlowRespData {
    #[serde(default)]
    pub success_record_ids: Vec<String>,
    #[serde(default)]
    pub fail_record_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserFlowRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_field: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_wifi: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default)]
    pub photo_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserFlowRespData {
    #[serde(default)]
    pub user_flow_results: Vec<UserFlow>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyUserSettingRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<ResponseUserSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserSettingRespData {
    #[serde(default)]
    pub user_settings: Vec<ResponseUserSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserStatsDataRespData {
    #[serde(default)]
    pub user_datas: Vec<UserStatsData>,
    #[serde(default)]
    pub invalid_user_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserStatsFieldRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_stats_field: Option<UserStatsField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserStatsViewRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<UserStatsView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserStatsViewRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<UserStatsView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserTaskRespData {
    #[serde(default)]
    pub user_task_results: Vec<UserTask>,
    #[serde(default)]
    pub invalid_user_ids: Vec<String>,
    #[serde(default)]
    pub unauthorized_user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateUserTaskRemedyRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_remedy: Option<UserTaskRemedy>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserTaskRemedyRespData {
    #[serde(default)]
    pub user_remedys: Vec<UserTaskRemedy>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserAllowedRemedysUserTaskRemedyRespData {
    #[serde(default)]
    pub user_allowed_remedys: Vec<UserAllowedRemedy>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessApprovalInfoRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_info: Option<ResponseApprovalInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArchiveRuleRespData {
    #[serde(default)]
    pub items: Vec<ArchiveReportMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadFileRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUserGroupRespData {
    #[serde(default)]
    pub users: Vec<UserBase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchGroupRespData {
    #[serde(default)]
    pub group_list: Vec<GroupMeta>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLeaveEmployExpireRecordRespData {
    #[serde(default)]
    pub records: Vec<LeaveEmployExpireRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListShiftRespData {
    #[serde(default)]
    pub shift_list: Vec<ResponseShift>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryShiftRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_times: Option<i32>,
    #[serde(default)]
    pub sub_shift_leader_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    #[serde(default)]
    pub flexible_rule: Vec<FlexibleRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_off: Option<bool>,
    #[serde(default)]
    pub punch_time_rule: Vec<PunchTimeRule>,
    #[serde(default)]
    pub late_off_late_on_rule: Vec<LateOffLateOnRule>,
    #[serde(default)]
    pub rest_time_rule: Vec<RestRule>,
    #[serde(default)]
    pub overtime_rule: Vec<OvertimeRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_type: Option<i32>,
    #[serde(default)]
    pub overtime_rest_time_rule: Vec<RestRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_serious_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_middle_time_rule: Option<ShiftMiddleTimeRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_attendance_time_config: Option<ShiftAttendanceTimeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_off_late_on_setting: Option<LateOffLateOnSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub rest_time_flexible_configs: Vec<RestTimeFlexibleConfig>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseApprovalInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub upper_titles: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveReportMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_name: Option<I18nMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_rule_name: Option<I18nMap>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChildField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChildItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Field {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub child_fields: Vec<ChildField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlexibleRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_early_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_late_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nMap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nNames {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Item {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub child_items: Vec<ChildItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LangText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LateOffLateOnRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_off_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_on_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LateOffLateOnSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_off_base_on_time_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_on_base_on_time_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseLeaveAccrualRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_by: Option<i32>,
    #[serde(default)]
    pub reason: Vec<LangText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveEmployExpireRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_granting_quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default)]
    pub reason: Vec<LangText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_update_by_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_sub_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OvertimeRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_overtime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_overtime: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PunchTimeRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_lack: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_advance_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_early: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_lack: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_delay_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_serious_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_on: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_off: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PunchTimeSimpleRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegionPlace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RestRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_begin_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RestTimeFlexibleConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_flexible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_mins: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseShift {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_times: Option<i32>,
    #[serde(default)]
    pub sub_shift_leader_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    #[serde(default)]
    pub flexible_rule: Vec<FlexibleRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_off: Option<bool>,
    #[serde(default)]
    pub punch_time_rule: Vec<PunchTimeRule>,
    #[serde(default)]
    pub late_off_late_on_rule: Vec<LateOffLateOnRule>,
    #[serde(default)]
    pub rest_time_rule: Vec<RestRule>,
    #[serde(default)]
    pub overtime_rule: Vec<OvertimeRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_type: Option<i32>,
    #[serde(default)]
    pub overtime_rest_time_rule: Vec<RestRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_serious_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_middle_time_rule: Option<ShiftMiddleTimeRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_attendance_time_config: Option<ShiftAttendanceTimeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_off_late_on_setting: Option<LateOffLateOnSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub rest_time_flexible_configs: Vec<RestTimeFlexibleConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShiftAttendanceTimeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendance_time: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_attendance_time: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_attendance_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShiftMiddleTimeRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_time_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_middle_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_record: Option<UserFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out_record: Option<UserFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_result_supplement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out_result_supplement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in_shift_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out_shift_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_shift_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_stamp: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time_stamp: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeRangeList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
    #[serde(default)]
    pub time_ranges: Vec<TimeRange>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserAllowedRemedy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_date: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_free_punch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_no: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_punch_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserApproval {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default)]
    pub outs: Vec<UserOut>,
    #[serde(default)]
    pub leaves: Vec<UserLeave>,
    #[serde(default)]
    pub overtime_works: Vec<UserOvertimeWork>,
    #[serde(default)]
    pub trips: Vec<UserTrip>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub department_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDailyShift {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_no: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_clear_schedule: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFlow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_field: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_wifi: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default)]
    pub photo_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserLeave {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uniq_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<I18nNames>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    #[serde(default)]
    pub leave_detail_range_objs: Vec<TimeRangeList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOut {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uniq_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<I18nNames>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    #[serde(default)]
    pub correct_process_id: Vec<String>,
    #[serde(default)]
    pub cancel_process_id: Vec<String>,
    #[serde(default)]
    pub process_id: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOvertimeWork {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    #[serde(default)]
    pub correct_process_id: Vec<String>,
    #[serde(default)]
    pub cancel_process_id: Vec<String>,
    #[serde(default)]
    pub process_id: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseUserSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_key_update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatsData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub datas: Vec<UserStatsDataCell>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatsDataCell {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default)]
    pub features: Vec<UserStatsDataFeature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_num: Option<UserStatsDataDuration>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatsDataDuration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_day: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_hour: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatsDataFeature {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatsField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatsView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(default)]
    pub records: Vec<TaskResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTaskRemedy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_date: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_no: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTmpDailyShift {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    #[serde(default)]
    pub punch_time_simple_rules: Vec<PunchTimeSimpleRule>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTrip {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    #[serde(default)]
    pub correct_process_id: Vec<String>,
    #[serde(default)]
    pub cancel_process_id: Vec<String>,
    #[serde(default)]
    pub process_id: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure: Option<RegionPlace>,
    #[serde(default)]
    pub destinations: Vec<RegionPlace>,
    #[serde(default)]
    pub transportation: Vec<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trip_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}
// ── Resources ──

pub struct GroupResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateGroupQuery<'a> {
    pub body: &'a CreateOrUpdateGroupReqBody,
    pub employee_type: &'a str,
    pub dept_type: Option<&'a str>,
}

impl<'a> CreateGroupQuery<'a> {
    pub fn new(body: &'a CreateOrUpdateGroupReqBody, employee_type: &'a str) -> Self {
        Self {
            body,
            employee_type,
            dept_type: None,
        }
    }

    pub fn dept_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.dept_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetGroupQuery<'a> {
    pub group_id: &'a str,
    pub employee_type: &'a str,
    pub dept_type: Option<&'a str>,
}

impl<'a> GetGroupQuery<'a> {
    pub fn new(group_id: &'a str, employee_type: &'a str) -> Self {
        Self {
            group_id,
            employee_type,
            dept_type: None,
        }
    }

    pub fn dept_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.dept_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListGroupQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListGroupQuery<'a> {
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
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListUserGroupQuery<'a> {
    pub group_id: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListUserGroupQuery<'a> {
    pub fn new(group_id: &'a str) -> Self {
        Self {
            group_id,
            page_size: None,
            page_token: None,
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
}

impl<'a> GroupResource<'a> {
    pub async fn create(
        &self,
        body: &CreateOrUpdateGroupReqBody,
        employee_type: &str,
        dept_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateGroupResp, LarkError> {
        let query = CreateGroupQuery::new(body, employee_type).dept_type(dept_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateGroupResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/groups",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", query.employee_type)
        .query("dept_type", query.dept_type)
        .json_body(query.body)?
        .send_response::<GroupData, CreateGroupResp>()
        .await
    }

    pub async fn get(
        &self,
        group_id: &str,
        employee_type: &str,
        dept_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetGroupResp, LarkError> {
        let query = GetGroupQuery::new(group_id, employee_type).dept_type(dept_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetGroupResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/groups/{}", query.group_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", query.employee_type)
        .query("dept_type", query.dept_type)
        .send_response::<GroupData, GetGroupResp>()
        .await
    }

    pub async fn delete(
        &self,
        group_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/groups/{group_id}");
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
        option: &RequestOption,
    ) -> Result<ListGroupResp, LarkError> {
        let query = ListGroupQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListGroupResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/attendance/v1/groups",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_response::<GroupListData, ListGroupResp>()
        .await
    }

    pub async fn list_user(
        &self,
        group_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListUserGroupResp, LarkError> {
        let query = ListUserGroupQuery::new(group_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_user_by_query(&query, option).await
    }

    pub async fn list_user_by_query(
        &self,
        query: &ListUserGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListUserGroupResp, LarkError> {
        let path = format!(
            "/open-apis/attendance/v1/groups/{}/list_user",
            query.group_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListUserGroupRespData, ListUserGroupResp>()
        .await
    }

    pub async fn search(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<SearchGroupResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/groups/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<SearchGroupRespData, SearchGroupResp>()
        .await
    }
}

pub struct ShiftResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListShiftQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListShiftQuery<'a> {
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
}

impl<'a> ShiftResource<'a> {
    pub async fn create(
        &self,
        body: &CreateShiftReqBody,
        option: &RequestOption,
    ) -> Result<CreateShiftResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/shifts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<ShiftData, CreateShiftResp>()
        .await
    }

    pub async fn get(
        &self,
        shift_id: &str,
        option: &RequestOption,
    ) -> Result<GetShiftResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/shifts/{shift_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<ShiftData, GetShiftResp>()
        .await
    }

    pub async fn delete(
        &self,
        shift_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/shifts/{shift_id}");
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
        option: &RequestOption,
    ) -> Result<ListShiftResp, LarkError> {
        let query = ListShiftQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListShiftQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListShiftResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/attendance/v1/shifts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListShiftRespData, ListShiftResp>()
        .await
    }

    pub async fn query(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<QueryShiftResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/shifts/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<QueryShiftRespData, QueryShiftResp>()
        .await
    }
}

pub struct UserSettingResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct BatchGetUserSettingQuery<'a> {
    pub body: &'a BatchGetUserSettingReqBody,
    pub employee_type: &'a str,
}

impl<'a> BatchGetUserSettingQuery<'a> {
    pub fn new(body: &'a BatchGetUserSettingReqBody, employee_type: &'a str) -> Self {
        Self {
            body,
            employee_type,
        }
    }
}

impl<'a> UserSettingResource<'a> {
    pub async fn batch_get(
        &self,
        body: &BatchGetUserSettingReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<BatchGetUserSettingResp, LarkError> {
        let query = BatchGetUserSettingQuery::new(body, employee_type);
        self.batch_get_by_query(&query, option).await
    }

    pub async fn batch_get_by_query(
        &self,
        query: &BatchGetUserSettingQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchGetUserSettingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/attendance/v1/user_settings/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", query.employee_type)
        .json_body(query.body)?
        .send_response::<UserSettingListData, BatchGetUserSettingResp>()
        .await
    }
}

pub struct RecordResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct QueryRecordQuery<'a> {
    pub body: &'a QueryRecordReqBody,
    pub employee_type: &'a str,
}

impl<'a> QueryRecordQuery<'a> {
    pub fn new(body: &'a QueryRecordReqBody, employee_type: &'a str) -> Self {
        Self {
            body,
            employee_type,
        }
    }
}

impl<'a> RecordResource<'a> {
    pub async fn query(
        &self,
        body: &QueryRecordReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryRecordResp, LarkError> {
        let query = QueryRecordQuery::new(body, employee_type);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryRecordResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_tasks/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", query.employee_type)
        .json_body(query.body)?
        .send_response::<RecordListData, QueryRecordResp>()
        .await
    }
}

pub struct ApprovalInfoResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetApprovalInfoQuery<'a> {
    pub body: &'a GetApprovalInfoReqBody,
    pub employee_type: &'a str,
}

impl<'a> GetApprovalInfoQuery<'a> {
    pub fn new(body: &'a GetApprovalInfoReqBody, employee_type: &'a str) -> Self {
        Self {
            body,
            employee_type,
        }
    }
}

impl<'a> ApprovalInfoResource<'a> {
    pub async fn get(
        &self,
        body: &GetApprovalInfoReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<GetApprovalInfoResp, LarkError> {
        let query = GetApprovalInfoQuery::new(body, employee_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApprovalInfoQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApprovalInfoResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_approvals/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", query.employee_type)
        .json_body(query.body)?
        .send_response::<ApprovalInfoListData, GetApprovalInfoResp>()
        .await
    }
}

pub struct LeaveAccrualResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListLeaveAccrualQuery<'a> {
    pub employment_id: &'a str,
    pub leave_type_id: Option<&'a str>,
    pub accrual_date_from: Option<&'a str>,
    pub accrual_date_to: Option<&'a str>,
    pub page_token: Option<&'a str>,
    pub page_size: Option<i32>,
}

impl<'a> ListLeaveAccrualQuery<'a> {
    pub fn new(employment_id: &'a str) -> Self {
        Self {
            employment_id,
            leave_type_id: None,
            accrual_date_from: None,
            accrual_date_to: None,
            page_token: None,
            page_size: None,
        }
    }

    pub fn leave_type_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.leave_type_id = value.into();
        self
    }

    pub fn accrual_date_from(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.accrual_date_from = value.into();
        self
    }

    pub fn accrual_date_to(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.accrual_date_to = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
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

impl<'a> LeaveAccrualResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        employment_id: &str,
        leave_type_id: Option<&str>,
        accrual_date_from: Option<&str>,
        accrual_date_to: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListLeaveAccrualResp, LarkError> {
        let query = ListLeaveAccrualQuery::new(employment_id)
            .leave_type_id(leave_type_id)
            .accrual_date_from(accrual_date_from)
            .accrual_date_to(accrual_date_to)
            .page_token(page_token)
            .page_size(page_size);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListLeaveAccrualQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListLeaveAccrualResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/attendance/v1/leave_accrual_record",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employment_id", query.employment_id)
        .query("leave_type_id", query.leave_type_id)
        .query("accrual_date_from", query.accrual_date_from)
        .query("accrual_date_to", query.accrual_date_to)
        .page_query(query.page_query())
        .send_response::<LeaveAccrualListData, ListLeaveAccrualResp>()
        .await
    }
}

// ── Response types for new resources ──

impl_resp_v2!(UploadReportArchiveRuleResp, UploadReportArchiveRuleRespData);
impl_resp_v2!(
    UserStatsFieldsQueryArchiveRuleResp,
    UserStatsFieldsQueryArchiveRuleRespData
);
impl_resp_v2!(PatchLeaveAccrualRecordResp, PatchLeaveAccrualRecordRespData);
impl_resp_v2!(PatchLeaveEmployExpireRecordResp, ());
impl_resp_v2!(CreateUserApprovalResp, CreateUserApprovalRespData);
impl_resp_v2!(QueryUserApprovalResp, QueryUserApprovalRespData);
impl_resp_v2!(
    BatchCreateUserDailyShiftResp,
    BatchCreateUserDailyShiftRespData
);
impl_resp_v2!(
    BatchCreateTempUserDailyShiftResp,
    BatchCreateTempUserDailyShiftRespData
);
impl_resp_v2!(QueryUserDailyShiftResp, QueryUserDailyShiftRespData);
impl_resp_v2!(BatchCreateUserFlowResp, BatchCreateUserFlowRespData);
impl_resp_v2!(BatchDelUserFlowResp, BatchDelUserFlowRespData);
impl_resp_v2!(GetUserFlowResp, GetUserFlowRespData);
impl_resp_v2!(QueryUserFlowResp, QueryUserFlowRespData);
impl_resp_v2!(ModifyUserSettingResp, ModifyUserSettingRespData);
impl_resp_v2!(QueryUserSettingResp, QueryUserSettingRespData);
impl_resp_v2!(QueryUserStatsDataResp, QueryUserStatsDataRespData);
impl_resp_v2!(QueryUserStatsFieldResp, QueryUserStatsFieldRespData);
impl_resp_v2!(QueryUserStatsViewResp, QueryUserStatsViewRespData);
impl_resp_v2!(UpdateUserStatsViewResp, UpdateUserStatsViewRespData);
impl_resp_v2!(QueryUserTaskResp, QueryUserTaskRespData);
impl_resp_v2!(CreateUserTaskRemedyResp, CreateUserTaskRemedyRespData);
impl_resp_v2!(QueryUserTaskRemedyResp, QueryUserTaskRemedyRespData);
impl_resp_v2!(
    QueryUserAllowedRemedysUserTaskRemedyResp,
    QueryUserAllowedRemedysUserTaskRemedyRespData
);
impl_resp_v2!(ProcessApprovalInfoResp, ProcessApprovalInfoRespData);
impl_resp_v2!(DelReportArchiveRuleResp, ());
impl_resp_v2!(ListArchiveRuleResp, ListArchiveRuleRespData);
impl_resp_v2!(UploadFileResp, UploadFileRespData);
impl_resp_v2!(ListUserGroupResp, ListUserGroupRespData);
impl_resp_v2!(SearchGroupResp, SearchGroupRespData);
impl_resp_v2!(
    GetLeaveEmployExpireRecordResp,
    GetLeaveEmployExpireRecordRespData
);
impl_resp_v2!(ListShiftResp, ListShiftRespData);
impl_resp_v2!(QueryShiftResp, QueryShiftRespData);

// ── Request body types for new resources ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadReportArchiveRuleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_report_datas: Option<Vec<ArchiveReportData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UserStatsFieldsQueryArchiveRuleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchLeaveAccrualRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_granting_record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<LangText>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchLeaveEmployExpireRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUserApprovalReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_approval: Option<UserApproval>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserApprovalReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_from: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_to: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_to: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateUserDailyShiftReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_daily_shifts: Option<Vec<UserDailyShift>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserDailyShiftReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_from: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_to: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateUserFlowReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_records: Option<Vec<UserFlow>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDelUserFlowReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserFlowReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_to: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyUserSettingReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<UserSetting>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserSettingReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserStatsDataReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_group_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserStatsFieldReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserStatsViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserStatsViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<UserStatsView>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_from: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_to: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_overtime_result: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUserTaskRemedyReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_date: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_no: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserTaskRemedyReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserAllowedRemedysReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_date: Option<i32>,
}

// ── New resource structs ──

pub struct ArchiveRuleResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListArchiveRuleQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListArchiveRuleQuery<'a> {
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
}

impl<'a> ArchiveRuleResource<'a> {
    pub async fn upload_report(
        &self,
        body: &UploadReportArchiveRuleReqBody,
        option: &RequestOption,
    ) -> Result<UploadReportArchiveRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/archive_rule/upload_report",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<UploadReportArchiveRuleRespData, UploadReportArchiveRuleResp>()
        .await
    }

    pub async fn user_stats_fields_query(
        &self,
        body: &UserStatsFieldsQueryArchiveRuleReqBody,
        option: &RequestOption,
    ) -> Result<UserStatsFieldsQueryArchiveRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/archive_rule/user_stats_fields_query",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2_response::<UserStatsFieldsQueryArchiveRuleRespData, UserStatsFieldsQueryArchiveRuleResp>()
        .await
    }

    pub async fn del_report(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<DelReportArchiveRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/archive_rule/del_report",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), DelReportArchiveRuleResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListArchiveRuleResp, LarkError> {
        let query = ListArchiveRuleQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListArchiveRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListArchiveRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/attendance/v1/archive_rule",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page_query())
        .send_v2_response::<ListArchiveRuleRespData, ListArchiveRuleResp>()
        .await
    }
}

pub struct LeaveAccrualRecordResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PatchLeaveAccrualRecordQuery<'a> {
    pub leave_id: &'a str,
    pub body: &'a PatchLeaveAccrualRecordReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchLeaveAccrualRecordQuery<'a> {
    pub fn new(leave_id: &'a str, body: &'a PatchLeaveAccrualRecordReqBody) -> Self {
        Self {
            leave_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> LeaveAccrualRecordResource<'a> {
    pub async fn patch(
        &self,
        leave_id: &str,
        body: &PatchLeaveAccrualRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchLeaveAccrualRecordResp, LarkError> {
        let query = PatchLeaveAccrualRecordQuery::new(leave_id, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchLeaveAccrualRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchLeaveAccrualRecordResp, LarkError> {
        let path = format!(
            "/open-apis/attendance/v1/leave_accrual_record/{}",
            query.leave_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<PatchLeaveAccrualRecordRespData, PatchLeaveAccrualRecordResp>()
        .await
    }
}

pub struct LeaveEmployExpireRecordResource<'a> {
    config: &'a Config,
}

impl<'a> LeaveEmployExpireRecordResource<'a> {
    pub async fn patch(
        &self,
        leave_id: &str,
        body: &PatchLeaveEmployExpireRecordReqBody,
        option: &RequestOption,
    ) -> Result<PatchLeaveEmployExpireRecordResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/leave_employ_expire_records/{leave_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), PatchLeaveEmployExpireRecordResp>()
        .await
    }

    pub async fn get(
        &self,
        leave_id: &str,
        option: &RequestOption,
    ) -> Result<GetLeaveEmployExpireRecordResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/leave_employ_expire_records/{leave_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<GetLeaveEmployExpireRecordRespData, GetLeaveEmployExpireRecordResp>()
        .await
    }
}

pub struct ApprovalInfoProcessResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalInfoProcessResource<'a> {
    pub async fn process(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<ProcessApprovalInfoResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/approval_infos/process",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<ProcessApprovalInfoRespData, ProcessApprovalInfoResp>()
        .await
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

impl<'a> FileResource<'a> {
    pub async fn download(
        &self,
        file_id: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/files/{file_id}/download");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .download()
        .await
    }

    pub async fn upload(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<UploadFileResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/files/upload",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<UploadFileRespData, UploadFileResp>()
        .await
    }
}

pub struct UserApprovalResource<'a> {
    config: &'a Config,
}

impl<'a> UserApprovalResource<'a> {
    pub async fn create(
        &self,
        body: &CreateUserApprovalReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<CreateUserApprovalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_approvals",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<CreateUserApprovalRespData, CreateUserApprovalResp>()
        .await
    }

    pub async fn query(
        &self,
        body: &QueryUserApprovalReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserApprovalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_approvals/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserApprovalRespData, QueryUserApprovalResp>()
        .await
    }
}

pub struct UserDailyShiftResource<'a> {
    config: &'a Config,
}

impl<'a> UserDailyShiftResource<'a> {
    pub async fn batch_create(
        &self,
        body: &BatchCreateUserDailyShiftReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<BatchCreateUserDailyShiftResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_daily_shifts/batch_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<BatchCreateUserDailyShiftRespData, BatchCreateUserDailyShiftResp>()
        .await
    }

    pub async fn batch_create_temp(
        &self,
        body: &impl Serialize,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<BatchCreateTempUserDailyShiftResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_daily_shifts/batch_create_temp",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<BatchCreateTempUserDailyShiftRespData, BatchCreateTempUserDailyShiftResp>()
        .await
    }

    pub async fn query(
        &self,
        body: &QueryUserDailyShiftReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserDailyShiftResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_daily_shifts/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserDailyShiftRespData, QueryUserDailyShiftResp>()
        .await
    }
}

pub struct UserFlowResource<'a> {
    config: &'a Config,
}

impl<'a> UserFlowResource<'a> {
    pub async fn batch_create(
        &self,
        body: &BatchCreateUserFlowReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<BatchCreateUserFlowResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_flows/batch_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<BatchCreateUserFlowRespData, BatchCreateUserFlowResp>()
        .await
    }

    pub async fn batch_del(
        &self,
        body: &BatchDelUserFlowReqBody,
        option: &RequestOption,
    ) -> Result<BatchDelUserFlowResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_flows/batch_del",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<BatchDelUserFlowRespData, BatchDelUserFlowResp>()
        .await
    }

    pub async fn get(
        &self,
        user_flow_id: &str,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<GetUserFlowResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/user_flows/{user_flow_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .send_v2_response::<GetUserFlowRespData, GetUserFlowResp>()
        .await
    }

    pub async fn query(
        &self,
        body: &QueryUserFlowReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserFlowResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_flows/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserFlowRespData, QueryUserFlowResp>()
        .await
    }
}

pub struct UserSettingResource2<'a> {
    config: &'a Config,
}

impl<'a> UserSettingResource2<'a> {
    pub async fn modify(
        &self,
        body: &ModifyUserSettingReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<ModifyUserSettingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_settings/modify",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<ModifyUserSettingRespData, ModifyUserSettingResp>()
        .await
    }

    pub async fn query(
        &self,
        body: &QueryUserSettingReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserSettingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/attendance/v1/user_settings/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserSettingRespData, QueryUserSettingResp>()
        .await
    }
}

pub struct UserStatsDataResource<'a> {
    config: &'a Config,
}

impl<'a> UserStatsDataResource<'a> {
    pub async fn query(
        &self,
        body: &QueryUserStatsDataReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserStatsDataResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_stats_datas/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserStatsDataRespData, QueryUserStatsDataResp>()
        .await
    }
}

pub struct UserStatsFieldResource<'a> {
    config: &'a Config,
}

impl<'a> UserStatsFieldResource<'a> {
    pub async fn query(
        &self,
        body: &QueryUserStatsFieldReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserStatsFieldResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_stats_fields/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserStatsFieldRespData, QueryUserStatsFieldResp>()
        .await
    }
}

pub struct UserStatsViewResource<'a> {
    config: &'a Config,
}

impl<'a> UserStatsViewResource<'a> {
    pub async fn query(
        &self,
        body: &QueryUserStatsViewReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserStatsViewResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_stats_views/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserStatsViewRespData, QueryUserStatsViewResp>()
        .await
    }

    pub async fn update(
        &self,
        user_stats_view_id: &str,
        body: &UpdateUserStatsViewReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<UpdateUserStatsViewResp, LarkError> {
        let path = format!("/open-apis/attendance/v1/user_stats_views/{user_stats_view_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<UpdateUserStatsViewRespData, UpdateUserStatsViewResp>()
        .await
    }
}

pub struct UserTaskResource<'a> {
    config: &'a Config,
}

impl<'a> UserTaskResource<'a> {
    pub async fn query(
        &self,
        body: &QueryUserTaskReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_tasks/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserTaskRespData, QueryUserTaskResp>()
        .await
    }
}

pub struct UserTaskRemedyResource<'a> {
    config: &'a Config,
}

impl<'a> UserTaskRemedyResource<'a> {
    pub async fn create(
        &self,
        body: &CreateUserTaskRemedyReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<CreateUserTaskRemedyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_task_remedys",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<CreateUserTaskRemedyRespData, CreateUserTaskRemedyResp>()
        .await
    }

    pub async fn query(
        &self,
        body: &QueryUserTaskRemedyReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserTaskRemedyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_task_remedys/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserTaskRemedyRespData, QueryUserTaskRemedyResp>()
        .await
    }

    pub async fn query_user_allowed_remedys(
        &self,
        body: &QueryUserAllowedRemedysReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserAllowedRemedysUserTaskRemedyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("employee_type", employee_type)
        .json_body(body)?
        .send_v2_response::<QueryUserAllowedRemedysUserTaskRemedyRespData, QueryUserAllowedRemedysUserTaskRemedyResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub group: GroupResource<'a>,
    pub shift: ShiftResource<'a>,
    pub user_setting: UserSettingResource<'a>,
    pub record: RecordResource<'a>,
    pub approval_info: ApprovalInfoResource<'a>,
    pub approval_info_process: ApprovalInfoProcessResource<'a>,
    pub leave_accrual: LeaveAccrualResource<'a>,
    pub archive_rule: ArchiveRuleResource<'a>,
    pub file: FileResource<'a>,
    pub leave_accrual_record: LeaveAccrualRecordResource<'a>,
    pub leave_employ_expire_record: LeaveEmployExpireRecordResource<'a>,
    pub user_approval: UserApprovalResource<'a>,
    pub user_daily_shift: UserDailyShiftResource<'a>,
    pub user_flow: UserFlowResource<'a>,
    pub user_setting2: UserSettingResource2<'a>,
    pub user_stats_data: UserStatsDataResource<'a>,
    pub user_stats_field: UserStatsFieldResource<'a>,
    pub user_stats_view: UserStatsViewResource<'a>,
    pub user_task: UserTaskResource<'a>,
    pub user_task_remedy: UserTaskRemedyResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            group: GroupResource { config },
            shift: ShiftResource { config },
            user_setting: UserSettingResource { config },
            record: RecordResource { config },
            approval_info: ApprovalInfoResource { config },
            approval_info_process: ApprovalInfoProcessResource { config },
            leave_accrual: LeaveAccrualResource { config },
            archive_rule: ArchiveRuleResource { config },
            file: FileResource { config },
            leave_accrual_record: LeaveAccrualRecordResource { config },
            leave_employ_expire_record: LeaveEmployExpireRecordResource { config },
            user_approval: UserApprovalResource { config },
            user_daily_shift: UserDailyShiftResource { config },
            user_flow: UserFlowResource { config },
            user_setting2: UserSettingResource2 { config },
            user_stats_data: UserStatsDataResource { config },
            user_stats_field: UserStatsFieldResource { config },
            user_stats_view: UserStatsViewResource { config },
            user_task: UserTaskResource { config },
            user_task_remedy: UserTaskRemedyResource { config },
        }
    }
}
