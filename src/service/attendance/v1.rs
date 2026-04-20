use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

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
    pub locations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_day_shift_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_punch_cfg: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_punch_special_days: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_punch_special_days: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_day_no_punch_as_lack: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_now: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_period_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy_period_custom_date: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_type_config: Option<serde_json::Value>,
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
    pub flexible_rule: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_off: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub punch_time_rule: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_lack: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_early: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_lack: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub break_time_rule: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_serious_late: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_punch_shift_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overtime_rule: Option<Vec<serde_json::Value>>,
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
    pub check_in_record: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out_record: Option<serde_json::Value>,
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
    pub reason: Option<Vec<serde_json::Value>>,
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
    pub punch_time_rule: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub break_time_rule: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_rule: Option<Vec<serde_json::Value>>,
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
pub struct GroupData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<AttendanceGroup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupListData {
    #[serde(default)]
    pub group_list: Vec<serde_json::Value>,
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

// ── Resources ──

pub struct GroupResource<'a> {
    config: &'a Config,
}

impl<'a> GroupResource<'a> {
    pub async fn create(
        &self,
        body: &CreateOrUpdateGroupReqBody,
        employee_type: &str,
        dept_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateGroupResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/attendance/v1/groups");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        if let Some(v) = dept_type {
            api_req.query_params.set("dept_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<GroupData>(self.config, &api_req, option).await?;
        Ok(CreateGroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        group_id: &str,
        employee_type: &str,
        dept_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetGroupResp> {
        let path = format!("/open-apis/attendance/v1/groups/{group_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        if let Some(v) = dept_type {
            api_req.query_params.set("dept_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GroupData>(self.config, &api_req, option).await?;
        Ok(GetGroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, group_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/attendance/v1/groups/{group_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
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
        option: &RequestOption,
    ) -> Result<ListGroupResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/attendance/v1/groups");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GroupListData>(self.config, &api_req, option).await?;
        Ok(ListGroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ShiftResource<'a> {
    config: &'a Config,
}

impl<'a> ShiftResource<'a> {
    pub async fn create(
        &self,
        body: &CreateShiftReqBody,
        option: &RequestOption,
    ) -> Result<CreateShiftResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/attendance/v1/shifts");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ShiftData>(self.config, &api_req, option).await?;
        Ok(CreateShiftResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(&self, shift_id: &str, option: &RequestOption) -> Result<GetShiftResp> {
        let path = format!("/open-apis/attendance/v1/shifts/{shift_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<ShiftData>(self.config, &api_req, option).await?;
        Ok(GetShiftResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, shift_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/attendance/v1/shifts/{shift_id}");
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

pub struct UserSettingResource<'a> {
    config: &'a Config,
}

impl<'a> UserSettingResource<'a> {
    pub async fn batch_get(
        &self,
        body: &BatchGetUserSettingReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<BatchGetUserSettingResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_settings/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UserSettingListData>(self.config, &api_req, option).await?;
        Ok(BatchGetUserSettingResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct RecordResource<'a> {
    config: &'a Config,
}

impl<'a> RecordResource<'a> {
    pub async fn query(
        &self,
        body: &QueryRecordReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryRecordResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_tasks/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RecordListData>(self.config, &api_req, option).await?;
        Ok(QueryRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ApprovalInfoResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalInfoResource<'a> {
    pub async fn get(
        &self,
        body: &GetApprovalInfoReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<GetApprovalInfoResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_approvals/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ApprovalInfoListData>(self.config, &api_req, option).await?;
        Ok(GetApprovalInfoResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct LeaveAccrualResource<'a> {
    config: &'a Config,
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
    ) -> Result<ListLeaveAccrualResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/attendance/v1/leave_accrual_record",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employment_id", employment_id);
        if let Some(v) = leave_type_id {
            api_req.query_params.set("leave_type_id", v);
        }
        if let Some(v) = accrual_date_from {
            api_req.query_params.set("accrual_date_from", v);
        }
        if let Some(v) = accrual_date_to {
            api_req.query_params.set("accrual_date_to", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<LeaveAccrualListData>(self.config, &api_req, option).await?;
        Ok(ListLeaveAccrualResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub group: GroupResource<'a>,
    pub shift: ShiftResource<'a>,
    pub user_setting: UserSettingResource<'a>,
    pub record: RecordResource<'a>,
    pub approval_info: ApprovalInfoResource<'a>,
    pub leave_accrual: LeaveAccrualResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            group: GroupResource { config },
            shift: ShiftResource { config },
            user_setting: UserSettingResource { config },
            record: RecordResource { config },
            approval_info: ApprovalInfoResource { config },
            leave_accrual: LeaveAccrualResource { config },
        }
    }
}
