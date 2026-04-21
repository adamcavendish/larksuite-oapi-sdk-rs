use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyResp, parse_v2};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_tenant_manager: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Badge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_image: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeGrant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_detail: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_grant_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminDeptStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_active: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_total: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_activation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_video: Option<serde_json::Value>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResetPasswordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBadgeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_image: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBadgeGrantReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grant_all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge: Option<Badge>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeListData {
    #[serde(default)]
    pub items: Vec<Badge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeGrantData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant: Option<BadgeGrant>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeGrantListData {
    #[serde(default)]
    pub items: Vec<BadgeGrant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeptStatListData {
    #[serde(default)]
    pub items: Vec<AdminDeptStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateBadgeResp, BadgeData);
impl_resp!(GetBadgeResp, BadgeData);
impl_resp!(ListBadgeResp, BadgeListData);
impl_resp!(CreateBadgeGrantResp, BadgeGrantData);
impl_resp!(GetBadgeGrantResp, BadgeGrantData);
impl_resp!(ListBadgeGrantResp, BadgeGrantListData);
impl_resp!(GetDeptStatResp, DeptStatListData);

// ── impl_resp_v2! macro ──

impl_resp_v2!(UpdateBadgeResp, BadgeData);
impl_resp_v2!(UpdateBadgeGrantResp, BadgeGrantData);
impl_resp_v2!(ListAdminUserStatResp, serde_json::Value);
impl_resp_v2!(ListAuditInfoResp, serde_json::Value);
impl_resp_v2!(CreateBadgeImageResp, serde_json::Value);
impl_resp_v2!(ListAdminDeptStatResp, serde_json::Value);

// ── Resources ──

pub struct PasswordResource<'a> {
    config: &'a Config,
}

impl<'a> PasswordResource<'a> {
    pub async fn reset(
        &self,
        body: &ResetPasswordReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/admin/v1/password/reset");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct BadgeResource<'a> {
    config: &'a Config,
}

impl<'a> BadgeResource<'a> {
    pub async fn create(
        &self,
        body: &CreateBadgeReqBody,
        option: &RequestOption,
    ) -> Result<CreateBadgeResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/admin/v1/badges");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BadgeData>(self.config, &api_req, option).await?;
        Ok(CreateBadgeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(&self, badge_id: &str, option: &RequestOption) -> Result<GetBadgeResp> {
        let path = format!("/open-apis/admin/v1/badges/{badge_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<BadgeData>(self.config, &api_req, option).await?;
        Ok(GetBadgeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        name: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBadgeResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/admin/v1/badges");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = name {
            api_req.query_params.set("name", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<BadgeListData>(self.config, &api_req, option).await?;
        Ok(ListBadgeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        badge_id: &str,
        body: &CreateBadgeReqBody,
        option: &RequestOption,
    ) -> Result<UpdateBadgeResp> {
        let path = format!("/open-apis/admin/v1/badges/{badge_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BadgeData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateBadgeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct BadgeGrantResource<'a> {
    config: &'a Config,
}

impl<'a> BadgeGrantResource<'a> {
    pub async fn create(
        &self,
        badge_id: &str,
        body: &CreateBadgeGrantReqBody,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateBadgeGrantResp> {
        let path = format!("/open-apis/admin/v1/badges/{badge_id}/grants");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BadgeGrantData>(self.config, &api_req, option).await?;
        Ok(CreateBadgeGrantResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        badge_id: &str,
        grant_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBadgeGrantResp> {
        let path = format!("/open-apis/admin/v1/badges/{badge_id}/grants/{grant_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<BadgeGrantData>(self.config, &api_req, option).await?;
        Ok(GetBadgeGrantResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        badge_id: &str,
        grant_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/admin/v1/badges/{badge_id}/grants/{grant_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
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
        badge_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        name: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBadgeGrantResp> {
        let path = format!("/open-apis/admin/v1/badges/{badge_id}/grants");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = name {
            api_req.query_params.set("name", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<BadgeGrantListData>(self.config, &api_req, option).await?;
        Ok(ListBadgeGrantResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        badge_id: &str,
        grant_id: &str,
        body: &CreateBadgeGrantReqBody,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateBadgeGrantResp> {
        let path = format!("/open-apis/admin/v1/badges/{badge_id}/grants/{grant_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BadgeGrantData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateBadgeGrantResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct AdminDeptStatResource<'a> {
    config: &'a Config,
}

impl<'a> AdminDeptStatResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        department_id_type: &str,
        start_date: &str,
        end_date: &str,
        department_id: &str,
        contains_child_dept: bool,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetDeptStatResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/admin/v1/dept_stats");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("department_id_type", department_id_type);
        api_req.query_params.set("start_date", start_date);
        api_req.query_params.set("end_date", end_date);
        api_req.query_params.set("department_id", department_id);
        api_req
            .query_params
            .set("contains_child_dept", contains_child_dept.to_string());
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DeptStatListData>(self.config, &api_req, option).await?;
        Ok(GetDeptStatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        department_id_type: &str,
        start_date: &str,
        end_date: &str,
        department_id: &str,
        contains_child_dept: bool,
        page_size: Option<i32>,
        page_token: Option<&str>,
        target_geo: Option<&str>,
        with_product_version: Option<bool>,
        option: &RequestOption,
    ) -> Result<ListAdminDeptStatResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/admin/v1/admin_dept_stats");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("department_id_type", department_id_type);
        api_req.query_params.set("start_date", start_date);
        api_req.query_params.set("end_date", end_date);
        api_req.query_params.set("department_id", department_id);
        api_req
            .query_params
            .set("contains_child_dept", contains_child_dept.to_string());
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = target_geo {
            api_req.query_params.set("target_geo", v);
        }
        if let Some(v) = with_product_version {
            api_req
                .query_params
                .set("with_product_version", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAdminDeptStatResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct AdminUserStatResource<'a> {
    config: &'a Config,
}

impl<'a> AdminUserStatResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        start_date: &str,
        end_date: &str,
        department_id: Option<&str>,
        user_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAdminUserStatResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/admin/v1/admin_user_stats");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.query_params.set("start_date", start_date);
        api_req.query_params.set("end_date", end_date);
        if let Some(v) = department_id {
            api_req.query_params.set("department_id", v);
        }
        if let Some(v) = user_id {
            api_req.query_params.set("user_id", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAdminUserStatResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct AuditInfoResource<'a> {
    config: &'a Config,
}

impl<'a> AuditInfoResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        latest: Option<&str>,
        oldest: Option<&str>,
        event_name: Option<&str>,
        operator_type: Option<&str>,
        operator_value: Option<&str>,
        event_module: Option<i32>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAuditInfoResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/admin/v1/audit_infos");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = latest {
            api_req.query_params.set("latest", v);
        }
        if let Some(v) = oldest {
            api_req.query_params.set("oldest", v);
        }
        if let Some(v) = event_name {
            api_req.query_params.set("event_name", v);
        }
        if let Some(v) = operator_type {
            api_req.query_params.set("operator_type", v);
        }
        if let Some(v) = operator_value {
            api_req.query_params.set("operator_value", v);
        }
        if let Some(v) = event_module {
            api_req.query_params.set("event_module", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAuditInfoResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct BadgeImageResource<'a> {
    config: &'a Config,
}

impl<'a> BadgeImageResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateBadgeImageResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/admin/v1/badge_images");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateBadgeImageResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub password: PasswordResource<'a>,
    pub badge: BadgeResource<'a>,
    pub badge_grant: BadgeGrantResource<'a>,
    pub dept_stat: AdminDeptStatResource<'a>,
    pub user_stat: AdminUserStatResource<'a>,
    pub audit_info: AuditInfoResource<'a>,
    pub badge_image: BadgeImageResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            password: PasswordResource { config },
            badge: BadgeResource { config },
            badge_grant: BadgeGrantResource { config },
            dept_stat: AdminDeptStatResource { config },
            user_stat: AdminUserStatResource { config },
            audit_info: AuditInfoResource { config },
            badge_image: BadgeImageResource { config },
        }
    }
}
