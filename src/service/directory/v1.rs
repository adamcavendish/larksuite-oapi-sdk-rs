use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyRespV2 as EmptyResp, PageQuery, RestRequest};

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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListDirectoryUserQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListDirectoryUserQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
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

impl<'a> UserResource<'a> {
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListUserResp, LarkError> {
        let query = ListDirectoryUserQuery::new()
            .user_id_type(user_id_type)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDirectoryUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListUserResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/users",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send::<UserListData>()
        .await?;
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateCollaborationRuleQuery<'a> {
    pub body: &'a serde_json::Value,
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> CreateCollaborationRuleQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            target_tenant_key: None,
            tenant_id: None,
        }
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteCollaborationRuleQuery<'a> {
    pub collaboration_rule_id: &'a str,
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> DeleteCollaborationRuleQuery<'a> {
    pub fn new(collaboration_rule_id: &'a str) -> Self {
        Self {
            collaboration_rule_id,
            target_tenant_key: None,
            tenant_id: None,
        }
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListCollaborationRuleQuery<'a> {
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListCollaborationRuleQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateCollaborationRuleQuery<'a> {
    pub collaboration_rule_id: &'a str,
    pub body: &'a serde_json::Value,
    pub target_tenant_key: Option<&'a str>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> UpdateCollaborationRuleQuery<'a> {
    pub fn new(collaboration_rule_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            collaboration_rule_id,
            body,
            target_tenant_key: None,
            tenant_id: None,
        }
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
        self
    }
}

impl CollaborationRuleResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        target_tenant_key: Option<&str>,
        tenant_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCollaborationRuleResp, LarkError> {
        let query = CreateCollaborationRuleQuery::new(body)
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateCollaborationRuleResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/collaboration_rules",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteCollaborationRuleQuery::new(collaboration_rule_id)
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            query.collaboration_rule_id
        );
        let (api_resp, code_error, _) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<ListCollaborationRuleResp, LarkError> {
        let query = ListCollaborationRuleQuery::new()
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCollaborationRuleResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/collaboration_rules",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = UpdateCollaborationRuleQuery::new(collaboration_rule_id, body)
            .target_tenant_key(target_tenant_key)
            .tenant_id(tenant_id);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateCollaborationRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            query.collaboration_rule_id
        );
        let (api_resp, code_error, _) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListCollaborationTenantQuery<'a> {
    pub tenant_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListCollaborationTenantQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
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

impl CollaborationTenantResource<'_> {
    pub async fn list(
        &self,
        tenant_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCollaborationTenantResp, LarkError> {
        let query = ListCollaborationTenantQuery::new()
            .tenant_id(tenant_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCollaborationTenantQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCollaborationTenantResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/collaboration_tenants",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("tenant_id", query.tenant_id)
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListShareEntityQuery<'a> {
    pub target_tenant_key: Option<&'a str>,
    pub target_department_id: Option<&'a str>,
    pub target_group_id: Option<&'a str>,
    pub is_select_subject: Option<bool>,
    pub tenant_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListShareEntityQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target_tenant_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_tenant_key = value.into();
        self
    }

    pub fn target_department_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_department_id = value.into();
        self
    }

    pub fn target_group_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_group_id = value.into();
        self
    }

    pub fn is_select_subject(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_select_subject = value.into();
        self
    }

    pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tenant_id = value.into();
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
    ) -> Result<ListShareEntityResp, LarkError> {
        let query = ListShareEntityQuery::new()
            .target_tenant_key(target_tenant_key)
            .target_department_id(target_department_id)
            .target_group_id(target_group_id)
            .is_select_subject(is_select_subject)
            .tenant_id(tenant_id)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListShareEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListShareEntityResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/directory/v1/share_entities",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("target_tenant_key", query.target_tenant_key)
        .query("target_department_id", query.target_department_id)
        .query("target_group_id", query.target_group_id)
        .query("is_select_subject", query.is_select_subject)
        .query("tenant_id", query.tenant_id)
        .page_query(query.page_query())
        .send_v2::<serde_json::Value>()
        .await?;
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

macro_rules! impl_directory_role_setters {
    () => {
        pub fn employee_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
            self.employee_id_type = value.into();
            self
        }

        pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
            self.department_id_type = value.into();
            self
        }

        pub fn is_admin_role(mut self, value: impl Into<Option<bool>>) -> Self {
            self.is_admin_role = value.into();
            self
        }
    };
}

macro_rules! impl_directory_tenant_setter {
    () => {
        pub fn tenant_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
            self.tenant_id = value.into();
            self
        }
    };
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> CreateDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub is_admin_role: Option<bool>,
    pub employee_id_type: Option<&'a str>,
}

impl<'a> DeleteDepartmentQuery<'a> {
    pub fn new(department_id: &'a str) -> Self {
        Self {
            department_id,
            is_admin_role: None,
            employee_id_type: None,
        }
    }

    pub fn is_admin_role(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_admin_role = value.into();
        self
    }

    pub fn employee_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.employee_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct FilterDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> FilterDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct MgetDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> MgetDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> PatchDepartmentQuery<'a> {
    pub fn new(department_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            department_id,
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchDirectoryDepartmentQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> SearchDirectoryDepartmentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
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
    ) -> Result<CreateDepartmentResp, LarkError> {
        let query = CreateDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteDepartmentQuery::new(department_id)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/departments/{}",
            query.department_id
        );
        let (api_resp, code_error, _) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<FilterDepartmentResp, LarkError> {
        let query = FilterDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.filter_by_query(&query, option).await
    }

    pub async fn filter_by_query(
        &self,
        query: &FilterDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<FilterDepartmentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments/filter",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<MgetDepartmentResp, LarkError> {
        let query = MgetDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role);
        self.mget_by_query(&query, option).await
    }

    pub async fn mget_by_query(
        &self,
        query: &MgetDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<MgetDepartmentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments/mget",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = PatchDepartmentQuery::new(department_id, body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/departments/{}",
            query.department_id
        );
        let (api_resp, code_error, _) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<SearchDepartmentResp, LarkError> {
        let query = SearchDirectoryDepartmentQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchDirectoryDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchDepartmentResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/departments/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateDirectoryEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> CreateDirectoryEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: Option<&'a serde_json::Value>,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> DeleteDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str) -> Self {
        Self {
            employee_id,
            body: None,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    pub fn body(mut self, value: impl Into<Option<&'a serde_json::Value>>) -> Self {
        self.body = value.into();
        self
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct FilterEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> FilterEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct MgetEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> MgetEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> PatchEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            employee_id,
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct RegularDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: Option<&'a serde_json::Value>,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> RegularDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str) -> Self {
        Self {
            employee_id,
            body: None,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    pub fn body(mut self, value: impl Into<Option<&'a serde_json::Value>>) -> Self {
        self.body = value.into();
        self
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ResurrectDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: Option<&'a serde_json::Value>,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> ResurrectDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str) -> Self {
        Self {
            employee_id,
            body: None,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    pub fn body(mut self, value: impl Into<Option<&'a serde_json::Value>>) -> Self {
        self.body = value.into();
        self
    }

    impl_directory_role_setters!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchDirectoryEmployeeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
    pub tenant_id: Option<&'a str>,
}

impl<'a> SearchDirectoryEmployeeQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
            tenant_id: None,
        }
    }

    impl_directory_role_setters!();
    impl_directory_tenant_setter!();
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ToBeResignedDirectoryEmployeeQuery<'a> {
    pub employee_id: &'a str,
    pub body: &'a serde_json::Value,
    pub employee_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub is_admin_role: Option<bool>,
}

impl<'a> ToBeResignedDirectoryEmployeeQuery<'a> {
    pub fn new(employee_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            employee_id,
            body,
            employee_id_type: None,
            department_id_type: None,
            is_admin_role: None,
        }
    }

    impl_directory_role_setters!();
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
    ) -> Result<CreateEmployeeResp, LarkError> {
        let query = CreateDirectoryEmployeeQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateEmployeeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteDirectoryEmployeeQuery::new(employee_id)
            .body(body)
            .employee_id_type(employee_id_type)
            .is_admin_role(is_admin_role)
            .department_id_type(department_id_type);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/directory/v1/employees/{}", query.employee_id);
        let request = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("department_id_type", query.department_id_type);
        let request = if let Some(body) = query.body {
            request.json_body(body)?
        } else {
            request
        };
        let (api_resp, code_error, _) = request.send_v2::<serde_json::Value>().await?;
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
    ) -> Result<FilterEmployeeResp, LarkError> {
        let query = FilterEmployeeQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.filter_by_query(&query, option).await
    }

    pub async fn filter_by_query(
        &self,
        query: &FilterEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<FilterEmployeeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees/filter",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<MgetEmployeeResp, LarkError> {
        let query = MgetEmployeeQuery::new(body)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type);
        self.mget_by_query(&query, option).await
    }

    pub async fn mget_by_query(
        &self,
        query: &MgetEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<MgetEmployeeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees/mget",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = PatchEmployeeQuery::new(employee_id, body)
            .employee_id_type(employee_id_type)
            .is_admin_role(is_admin_role)
            .department_id_type(department_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/directory/v1/employees/{}", query.employee_id);
        let (api_resp, code_error, _) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = RegularDirectoryEmployeeQuery::new(employee_id)
            .body(body)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type);
        self.regular_by_query(&query, option).await
    }

    pub async fn regular_by_query(
        &self,
        query: &RegularDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/employees/{}/regular",
            query.employee_id
        );
        let request = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type);
        let request = if let Some(body) = query.body {
            request.json_body(body)?
        } else {
            request
        };
        let (api_resp, code_error, _) = request.send_v2::<serde_json::Value>().await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = ResurrectDirectoryEmployeeQuery::new(employee_id)
            .body(body)
            .employee_id_type(employee_id_type)
            .is_admin_role(is_admin_role)
            .department_id_type(department_id_type);
        self.resurrect_by_query(&query, option).await
    }

    pub async fn resurrect_by_query(
        &self,
        query: &ResurrectDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/employees/{}/resurrect",
            query.employee_id
        );
        let request = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("department_id_type", query.department_id_type);
        let request = if let Some(body) = query.body {
            request.json_body(body)?
        } else {
            request
        };
        let (api_resp, code_error, _) = request.send_v2::<serde_json::Value>().await?;
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
    ) -> Result<SearchEmployeeResp, LarkError> {
        let query = SearchDirectoryEmployeeQuery::new(body)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type)
            .is_admin_role(is_admin_role)
            .tenant_id(tenant_id);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchEmployeeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/directory/v1/employees/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .query("is_admin_role", query.is_admin_role)
        .query("tenant_id", query.tenant_id)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = ToBeResignedDirectoryEmployeeQuery::new(employee_id, body)
            .is_admin_role(is_admin_role)
            .employee_id_type(employee_id_type)
            .department_id_type(department_id_type);
        self.to_be_resigned_by_query(&query, option).await
    }

    pub async fn to_be_resigned_by_query(
        &self,
        query: &ToBeResignedDirectoryEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/directory/v1/employees/{}/to_be_resigned",
            query.employee_id
        );
        let (api_resp, code_error, _) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("is_admin_role", query.is_admin_role)
        .query("employee_id_type", query.employee_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
