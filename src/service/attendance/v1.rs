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

    pub async fn list_user(
        &self,
        group_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListUserGroupResp> {
        let path = format!("/open-apis/attendance/v1/groups/{group_id}/list_user");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListUserGroupResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchGroupResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/attendance/v1/groups/search");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchGroupResp {
            api_resp,
            code_error,
            data,
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

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListShiftResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/attendance/v1/shifts");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListShiftResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryShiftResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/attendance/v1/shifts/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryShiftResp {
            api_resp,
            code_error,
            data,
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

// ── Helpers for v2 pattern (Option<CodeError>) ──

fn parse_v2<T: for<'de> serde::Deserialize<'de>>(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<T>,
) -> (ApiResp, Option<CodeError>, Option<T>) {
    if raw.code_error.code != 0 {
        (api_resp, Some(raw.code_error), None)
    } else {
        (api_resp, None, raw.data)
    }
}

macro_rules! impl_resp_v2 {
    ($name:ident, $data:ty) => {
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}

// ── Response types for new resources ──

impl_resp_v2!(UploadReportArchiveRuleResp, serde_json::Value);
impl_resp_v2!(UserStatsFieldsQueryArchiveRuleResp, serde_json::Value);
impl_resp_v2!(PatchLeaveAccrualRecordResp, serde_json::Value);
impl_resp_v2!(PatchLeaveEmployExpireRecordResp, serde_json::Value);
impl_resp_v2!(CreateUserApprovalResp, serde_json::Value);
impl_resp_v2!(QueryUserApprovalResp, serde_json::Value);
impl_resp_v2!(BatchCreateUserDailyShiftResp, serde_json::Value);
impl_resp_v2!(BatchCreateTempUserDailyShiftResp, serde_json::Value);
impl_resp_v2!(QueryUserDailyShiftResp, serde_json::Value);
impl_resp_v2!(BatchCreateUserFlowResp, serde_json::Value);
impl_resp_v2!(BatchDelUserFlowResp, serde_json::Value);
impl_resp_v2!(GetUserFlowResp, serde_json::Value);
impl_resp_v2!(QueryUserFlowResp, serde_json::Value);
impl_resp_v2!(ModifyUserSettingResp, serde_json::Value);
impl_resp_v2!(QueryUserSettingResp, serde_json::Value);
impl_resp_v2!(QueryUserStatsDataResp, serde_json::Value);
impl_resp_v2!(QueryUserStatsFieldResp, serde_json::Value);
impl_resp_v2!(QueryUserStatsViewResp, serde_json::Value);
impl_resp_v2!(UpdateUserStatsViewResp, serde_json::Value);
impl_resp_v2!(QueryUserTaskResp, serde_json::Value);
impl_resp_v2!(CreateUserTaskRemedyResp, serde_json::Value);
impl_resp_v2!(QueryUserTaskRemedyResp, serde_json::Value);
impl_resp_v2!(QueryUserAllowedRemedysUserTaskRemedyResp, serde_json::Value);
impl_resp_v2!(ProcessApprovalInfoResp, serde_json::Value);
impl_resp_v2!(DelReportArchiveRuleResp, serde_json::Value);
impl_resp_v2!(ListArchiveRuleResp, serde_json::Value);
impl_resp_v2!(UploadFileResp, serde_json::Value);
impl_resp_v2!(ListUserGroupResp, serde_json::Value);
impl_resp_v2!(SearchGroupResp, serde_json::Value);
impl_resp_v2!(GetLeaveEmployExpireRecordResp, serde_json::Value);
impl_resp_v2!(ListShiftResp, serde_json::Value);
impl_resp_v2!(QueryShiftResp, serde_json::Value);

#[derive(Debug, Clone)]
pub struct DownloadResp {
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub data: Vec<u8>,
}

// ── Request body types for new resources ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadReportArchiveRuleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_report_datas: Option<Vec<serde_json::Value>>,
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
    pub reason: Option<Vec<serde_json::Value>>,
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
    pub user_approval: Option<serde_json::Value>,
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
    pub user_daily_shifts: Option<Vec<serde_json::Value>>,
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
    pub flow_records: Option<Vec<serde_json::Value>>,
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
    pub user_setting: Option<serde_json::Value>,
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
    pub view: Option<serde_json::Value>,
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

impl<'a> ArchiveRuleResource<'a> {
    pub async fn upload_report(
        &self,
        body: &UploadReportArchiveRuleReqBody,
        option: &RequestOption,
    ) -> Result<UploadReportArchiveRuleResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/archive_rule/upload_report",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UploadReportArchiveRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn user_stats_fields_query(
        &self,
        body: &UserStatsFieldsQueryArchiveRuleReqBody,
        option: &RequestOption,
    ) -> Result<UserStatsFieldsQueryArchiveRuleResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/archive_rule/user_stats_fields_query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UserStatsFieldsQueryArchiveRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn del_report(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<DelReportArchiveRuleResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/archive_rule/del_report",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DelReportArchiveRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListArchiveRuleResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/attendance/v1/archive_rule");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListArchiveRuleResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct LeaveAccrualRecordResource<'a> {
    config: &'a Config,
}

impl<'a> LeaveAccrualRecordResource<'a> {
    pub async fn patch(
        &self,
        leave_id: &str,
        body: &PatchLeaveAccrualRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchLeaveAccrualRecordResp> {
        let path = format!("/open-apis/attendance/v1/leave_accrual_record/{leave_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchLeaveAccrualRecordResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<PatchLeaveEmployExpireRecordResp> {
        let path = format!("/open-apis/attendance/v1/leave_employ_expire_records/{leave_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchLeaveEmployExpireRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        leave_id: &str,
        option: &RequestOption,
    ) -> Result<GetLeaveEmployExpireRecordResp> {
        let path = format!("/open-apis/attendance/v1/leave_employ_expire_records/{leave_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetLeaveEmployExpireRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApprovalInfoProcessResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalInfoProcessResource<'a> {
    pub async fn process(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ProcessApprovalInfoResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/approval_infos/process",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ProcessApprovalInfoResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

impl<'a> FileResource<'a> {
    pub async fn download(&self, file_id: &str, option: &RequestOption) -> Result<DownloadResp> {
        let path = format!("/open-apis/attendance/v1/files/{file_id}/download");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }

    pub async fn upload(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadFileResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/attendance/v1/files/upload");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UploadFileResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<CreateUserApprovalResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_approvals",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateUserApprovalResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: &QueryUserApprovalReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserApprovalResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_approvals/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserApprovalResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<BatchCreateUserDailyShiftResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_daily_shifts/batch_create",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateUserDailyShiftResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_create_temp(
        &self,
        body: &serde_json::Value,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<BatchCreateTempUserDailyShiftResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_daily_shifts/batch_create_temp",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateTempUserDailyShiftResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: &QueryUserDailyShiftReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserDailyShiftResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_daily_shifts/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserDailyShiftResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<BatchCreateUserFlowResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_flows/batch_create",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateUserFlowResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_del(
        &self,
        body: &BatchDelUserFlowReqBody,
        option: &RequestOption,
    ) -> Result<BatchDelUserFlowResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_flows/batch_del",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchDelUserFlowResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        user_flow_id: &str,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<GetUserFlowResp> {
        let path = format!("/open-apis/attendance/v1/user_flows/{user_flow_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetUserFlowResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: &QueryUserFlowReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserFlowResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_flows/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserFlowResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<ModifyUserSettingResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_settings/modify",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ModifyUserSettingResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: &QueryUserSettingReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserSettingResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/attendance/v1/user_settings/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserSettingResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<QueryUserStatsDataResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_stats_datas/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserStatsDataResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<QueryUserStatsFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_stats_fields/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserStatsFieldResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<QueryUserStatsViewResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_stats_views/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserStatsViewResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        user_stats_view_id: &str,
        body: &UpdateUserStatsViewReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<UpdateUserStatsViewResp> {
        let path = format!("/open-apis/attendance/v1/user_stats_views/{user_stats_view_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateUserStatsViewResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<QueryUserTaskResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_tasks/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserTaskResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<CreateUserTaskRemedyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_task_remedys",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateUserTaskRemedyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: &QueryUserTaskRemedyReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserTaskRemedyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_task_remedys/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserTaskRemedyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query_user_allowed_remedys(
        &self,
        body: &QueryUserAllowedRemedysReqBody,
        employee_type: &str,
        option: &RequestOption,
    ) -> Result<QueryUserAllowedRemedysUserTaskRemedyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("employee_type", employee_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserAllowedRemedysUserTaskRemedyResp {
            api_resp,
            code_error,
            data,
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
