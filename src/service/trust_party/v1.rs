use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};
use crate::service::go_v397::GoV397;

impl_resp_v2!(GetCollaborationTenantResp, GetCollaborationTenantRespData);

impl_resp_v2!(ListCollaborationTenantResp, ListCollaborationTenantRespData);

impl_resp_v2!(
    VisibleOrganizationCollaborationTenantResp,
    VisibleOrganizationCollaborationTenantRespData
);

impl_resp_v2!(
    GetCollaborationTenantCollaborationDepartmentResp,
    GetCollaborationTenantCollaborationDepartmentRespData
);

impl_resp_v2!(
    GetCollaborationTenantCollaborationUserResp,
    GetCollaborationTenantCollaborationUserRespData
);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollaborationTenantRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_tenant: Option<CollaborationTenant>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollaborationTenantRespData {
    #[serde(default)]
    pub target_tenant_list: Vec<CollaborationTenant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisibleOrganizationCollaborationTenantRespData {
    #[serde(default)]
    pub collaboration_entity_list: Vec<CollaborationEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollaborationTenantCollaborationDepartmentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department: Option<CollaborationDepartment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollaborationTenantCollaborationUserRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user: Option<CollaborationUser>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvatarInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<I18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default)]
    pub leaders: Vec<CollaborationDepartmentLeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<CollaborationDepartmentId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationDepartmentId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationDepartmentLeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<CollaborationUserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaboration_entity_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_department_name: Option<I18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_user_name: Option<I18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_avatar: Option<AvatarInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_group_name: Option<I18nName>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationTenant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_tenant_name: Option<I18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_short_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_tenant_short_name: Option<I18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_tenant_tag: Option<I18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<AvatarInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<I18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<AvatarInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub department_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(default)]
    pub custom_attrs: Vec<UserCustomAttr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    #[serde(default)]
    pub parent_department_ids: Vec<CollaborationDepartmentId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<CollaborationUserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationUserId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomAttrGenericUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCustomAttr {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<UserCustomAttrValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCustomAttrValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic_user: Option<CustomAttrGenericUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_activated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_exited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unjoin: Option<bool>,
}
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetCollaborationTenantQuery<'a> {
    pub target_tenant_key: &'a str,
}

impl<'a> GetCollaborationTenantQuery<'a> {
    pub fn new(target_tenant_key: &'a str) -> Self {
        Self { target_tenant_key }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListCollaborationTenantQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListCollaborationTenantQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct VisibleOrganizationCollaborationTenantQuery<'a> {
    pub target_tenant_key: &'a str,
    pub page: PageQuery<'a>,
    pub department_id_type: Option<&'a str>,
    pub target_department_id: Option<&'a str>,
    pub group_id_type: Option<&'a str>,
    pub target_group_id: Option<&'a str>,
}

impl<'a> VisibleOrganizationCollaborationTenantQuery<'a> {
    pub fn new(target_tenant_key: &'a str) -> Self {
        Self {
            target_tenant_key,
            page: PageQuery::new(),
            department_id_type: None,
            target_department_id: None,
            group_id_type: None,
            target_group_id: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn target_department_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_department_id = value.into();
        self
    }

    pub fn group_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.group_id_type = value.into();
        self
    }

    pub fn target_group_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_group_id = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetCollaborationTenantCollaborationDepartmentQuery<'a> {
    pub target_tenant_key: &'a str,
    pub target_department_id: &'a str,
    pub target_department_id_type: Option<&'a str>,
}

impl<'a> GetCollaborationTenantCollaborationDepartmentQuery<'a> {
    pub fn new(target_tenant_key: &'a str, target_department_id: &'a str) -> Self {
        Self {
            target_tenant_key,
            target_department_id,
            target_department_id_type: None,
        }
    }

    pub fn target_department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_department_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetCollaborationTenantCollaborationUserQuery<'a> {
    pub target_tenant_key: &'a str,
    pub target_user_id: &'a str,
    pub target_user_id_type: Option<&'a str>,
}

impl<'a> GetCollaborationTenantCollaborationUserQuery<'a> {
    pub fn new(target_tenant_key: &'a str, target_user_id: &'a str) -> Self {
        Self {
            target_tenant_key,
            target_user_id,
            target_user_id_type: None,
        }
    }

    pub fn target_user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_user_id_type = value.into();
        self
    }
}

pub struct CollaborationTenantResource<'a> {
    config: &'a Config,
}

impl<'a> CollaborationTenantResource<'a> {
    /// Get collaboration tenant — GET /open-apis/trust_party/v1/collaboration_tenants/{target_tenant_key}
    pub async fn get(
        &self,
        target_tenant_key: &str,
        option: &RequestOption,
    ) -> Result<GetCollaborationTenantResp, LarkError> {
        self.get_by_query(&GetCollaborationTenantQuery::new(target_tenant_key), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCollaborationTenantQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetCollaborationTenantResp, LarkError> {
        let path = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}",
            query.target_tenant_key
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<GetCollaborationTenantRespData, GetCollaborationTenantResp>()
        .await
    }

    /// List collaboration tenants — GET /open-apis/trust_party/v1/collaboration_tenants
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCollaborationTenantResp, LarkError> {
        let query = ListCollaborationTenantQuery {
            page: PageQuery::from_parts(page_size, page_token),
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCollaborationTenantQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCollaborationTenantResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/trust_party/v1/collaboration_tenants",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .send_v2_response::<ListCollaborationTenantRespData, ListCollaborationTenantResp>()
        .await
    }

    /// Visible organization — GET /open-apis/trust_party/v1/collaboration_tenants/{target_tenant_key}/visible_organization
    pub async fn visible_organization_by_query(
        &self,
        query: &VisibleOrganizationCollaborationTenantQuery<'_>,
        option: &RequestOption,
    ) -> Result<VisibleOrganizationCollaborationTenantResp, LarkError> {
        let path = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}/visible_organization",
            query.target_tenant_key
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("department_id_type", query.department_id_type)
        .query("target_department_id", query.target_department_id)
        .query("group_id_type", query.group_id_type)
        .query("target_group_id", query.target_group_id)
        .send_v2_response::<VisibleOrganizationCollaborationTenantRespData, VisibleOrganizationCollaborationTenantResp>()
        .await
    }
}

pub struct CollaborationTenantCollaborationDepartmentResource<'a> {
    config: &'a Config,
}

impl<'a> CollaborationTenantCollaborationDepartmentResource<'a> {
    /// Get collaboration department — GET /open-apis/trust_party/v1/collaboration_tenants/{target_tenant_key}/collaboration_departments/{target_department_id}
    pub async fn get_by_query(
        &self,
        query: &GetCollaborationTenantCollaborationDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetCollaborationTenantCollaborationDepartmentResp, LarkError> {
        let path = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}/collaboration_departments/{}",
            query.target_tenant_key, query.target_department_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("target_department_id_type", query.target_department_id_type)
        .send_v2_response::<GetCollaborationTenantCollaborationDepartmentRespData, GetCollaborationTenantCollaborationDepartmentResp>()
        .await
    }
}

pub struct CollaborationTenantCollaborationUserResource<'a> {
    config: &'a Config,
}

impl<'a> CollaborationTenantCollaborationUserResource<'a> {
    /// Get collaboration user — GET /open-apis/trust_party/v1/collaboration_tenants/{target_tenant_key}/collaboration_users/{target_user_id}
    pub async fn get_by_query(
        &self,
        query: &GetCollaborationTenantCollaborationUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetCollaborationTenantCollaborationUserResp, LarkError> {
        let path = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}/collaboration_users/{}",
            query.target_tenant_key, query.target_user_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("target_user_id_type", query.target_user_id_type)
        .send_v2_response::<GetCollaborationTenantCollaborationUserRespData, GetCollaborationTenantCollaborationUserResp>()
        .await
    }
}

pub struct V1<'a> {
    pub collaboration_tenant: CollaborationTenantResource<'a>,
    pub collaboration_tenant_collaboration_department:
        CollaborationTenantCollaborationDepartmentResource<'a>,
    pub collaboration_tenant_collaboration_user: CollaborationTenantCollaborationUserResource<'a>,
    config: &'a Config,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            collaboration_tenant: CollaborationTenantResource { config },
            collaboration_tenant_collaboration_department:
                CollaborationTenantCollaborationDepartmentResource { config },
            collaboration_tenant_collaboration_user: CollaborationTenantCollaborationUserResource {
                config,
            },
            config,
        }
    }

    pub fn go_v397(&self) -> GoV397<'a> {
        GoV397::new(self.config)
    }
}
