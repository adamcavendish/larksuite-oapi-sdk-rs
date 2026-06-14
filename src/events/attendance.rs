//! Attendance v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2AttendanceUserTaskUpdatedV1 {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub shifts: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2AttendanceUserApprovalCreatedV1 {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub date: i32,
    #[serde(default)]
    pub outs: Vec<serde_json::Value>,
    #[serde(default)]
    pub leaves: Vec<serde_json::Value>,
    #[serde(default)]
    pub overtime_works: Vec<serde_json::Value>,
    #[serde(default)]
    pub time_off_approvals: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2AttendanceUserFlowCreatedV1 {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub creator_id: String,
    #[serde(default)]
    pub location_name: String,
    #[serde(default)]
    pub check_time: String,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub record_id: String,
    #[serde(default)]
    pub longitude: f64,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub ssid: String,
    #[serde(default)]
    pub bssid: String,
    #[serde(default)]
    pub check_result: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2AttendanceRemedyApplyUpdatedV1 {
    #[serde(default)]
    pub remedy_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub apply_time: String,
    #[serde(default)]
    pub status: i32,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_attendance_user_task_updated_v1 => P2AttendanceUserTaskUpdatedV1
        : "attendance.user_task.updated_v1",
    on_p2_attendance_user_approval_created_v1 => P2AttendanceUserApprovalCreatedV1
        : "attendance.user_approval.created_v1",
    on_p2_attendance_user_flow_created_v1 => P2AttendanceUserFlowCreatedV1
        : "attendance.user_flow.created_v1",
    on_p2_attendance_remedy_apply_updated_v1 => P2AttendanceRemedyApplyUpdatedV1
        : "attendance.remedy_apply.updated_v1",
}
