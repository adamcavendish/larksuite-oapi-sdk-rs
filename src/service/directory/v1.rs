use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyRespV2 as EmptyResp, parse_v2};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectoryUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserListData {
    #[serde(default)]
    pub items: Vec<DirectoryUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListUserResp, UserListData);

impl_resp_v2!(CreateCollaborationRuleResp, serde_json::Value);
impl_resp_v2!(ListCollaborationRuleResp, serde_json::Value);
impl_resp_v2!(ListCollaborationTenantResp, serde_json::Value);
impl_resp_v2!(ListShareEntityResp, serde_json::Value);
impl_resp_v2!(CreateDepartmentResp, serde_json::Value);
impl_resp_v2!(FilterDepartmentResp, serde_json::Value);
impl_resp_v2!(MgetDepartmentResp, serde_json::Value);
impl_resp_v2!(SearchDepartmentResp, serde_json::Value);
impl_resp_v2!(CreateEmployeeResp, serde_json::Value);
impl_resp_v2!(FilterEmployeeResp, serde_json::Value);
impl_resp_v2!(MgetEmployeeResp, serde_json::Value);
impl_resp_v2!(SearchEmployeeResp, serde_json::Value);

// ── Resources ──

pub struct UserResource<'a> {
    config: &'a Config,
}

impl<'a> UserResource<'a> {
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListUserResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/directory/v1/users");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<UserListData>(self.config, &api_req, option).await?;
        Ok(ListUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── CollaborationRule resource ──

pub struct CollaborationRuleResource<'a> {
    config: &'a Config,
}

impl CollaborationRuleResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCollaborationRuleResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/directory/v1/collaboration_rules",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = target_tenant_key {
            api_req.query_params.set("target_tenant_key", v);
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateCollaborationRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        collaboration_rule_id: &str,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/collaboration_rules/{collaboration_rule_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = target_tenant_key {
            api_req.query_params.set("target_tenant_key", v);
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn list(
        &self,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCollaborationRuleResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/directory/v1/collaboration_rules",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = target_tenant_key {
            api_req.query_params.set("target_tenant_key", v);
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
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
        Ok(ListCollaborationRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        collaboration_rule_id: &str,
        body: &serde_json::Value,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/collaboration_rules/{collaboration_rule_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = target_tenant_key {
            api_req.query_params.set("target_tenant_key", v);
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }
}

// ── CollaborationTenant resource ──

pub struct CollaborationTenantResource<'a> {
    config: &'a Config,
}

impl CollaborationTenantResource<'_> {
    pub async fn list(
        &self,
        tenant_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCollaborationTenantResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/directory/v1/collaboration_tenants",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
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
        Ok(ListCollaborationTenantResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ShareEntity resource ──

pub struct ShareEntityResource<'a> {
    config: &'a Config,
}

impl ShareEntityResource<'_> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        target_tenant_key: Option<&str>,
        target_department_id: Option<&str>,
        target_group_id: Option<&str>,
        is_select_subject: Option<bool>,
        tenant_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListShareEntityResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/directory/v1/share_entities");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = target_tenant_key {
            api_req.query_params.set("target_tenant_key", v);
        }
        if let Some(v) = target_department_id {
            api_req.query_params.set("target_department_id", v);
        }
        if let Some(v) = target_group_id {
            api_req.query_params.set("target_group_id", v);
        }
        if let Some(v) = is_select_subject {
            api_req.query_params.set("is_select_subject", v.to_string());
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
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
        Ok(ListShareEntityResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Department resource ──

pub struct DepartmentResource<'a> {
    config: &'a Config,
}

impl DepartmentResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/directory/v1/departments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        department_id: &str,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn filter(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<FilterDepartmentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/directory/v1/departments/filter",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(FilterDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn mget(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        option: &RequestOption,
    ) -> Result<MgetDepartmentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/directory/v1/departments/mget",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(MgetDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        department_id: &str,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn search(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchDepartmentResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/directory/v1/departments/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Employee resource ──

pub struct EmployeeResource<'a> {
    config: &'a Config,
}

impl EmployeeResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateEmployeeResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/directory/v1/employees");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        employee_id: &str,
        body: Option<&serde_json::Value>,
        employee_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/employees/{employee_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn filter(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<FilterEmployeeResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/directory/v1/employees/filter",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(FilterEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn mget(
        &self,
        body: &serde_json::Value,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<MgetEmployeeResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/directory/v1/employees/mget");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(MgetEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        employee_id: &str,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/employees/{employee_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn regular(
        &self,
        employee_id: &str,
        body: Option<&serde_json::Value>,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/employees/{employee_id}/regular");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn resurrect(
        &self,
        employee_id: &str,
        body: Option<&serde_json::Value>,
        employee_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/employees/{employee_id}/resurrect");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn search(
        &self,
        body: &serde_json::Value,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        is_admin_role: Option<bool>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchEmployeeResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/directory/v1/employees/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = tenant_id {
            api_req.query_params.set("tenant_id", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchEmployeeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn to_be_resigned(
        &self,
        employee_id: &str,
        body: &serde_json::Value,
        is_admin_role: Option<bool>,
        employee_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/directory/v1/employees/{employee_id}/to_be_resigned");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = is_admin_role {
            api_req.query_params.set("is_admin_role", v.to_string());
        }
        if let Some(v) = employee_id_type {
            api_req.query_params.set("employee_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub user: UserResource<'a>,
    pub collaboration_rule: CollaborationRuleResource<'a>,
    pub collaboration_tenant: CollaborationTenantResource<'a>,
    pub share_entity: ShareEntityResource<'a>,
    pub department: DepartmentResource<'a>,
    pub employee: EmployeeResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            user: UserResource { config },
            collaboration_rule: CollaborationRuleResource { config },
            collaboration_tenant: CollaborationTenantResource { config },
            share_entity: ShareEntityResource { config },
            department: DepartmentResource { config },
            employee: EmployeeResource { config },
        }
    }
}
