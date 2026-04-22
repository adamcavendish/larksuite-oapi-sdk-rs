//! Attendance v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

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
    pub fn on_p2_attendance_user_task_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2AttendanceUserTaskUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("attendance.user_task.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_attendance_user_approval_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2AttendanceUserApprovalCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("attendance.user_approval.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_attendance_user_flow_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2AttendanceUserFlowCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("attendance.user_flow.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_attendance_remedy_apply_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2AttendanceRemedyApplyUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("attendance.remedy_apply.updated_v1", wrap_handler(handler))
    }
}
