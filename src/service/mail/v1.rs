use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mailgroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailgroup_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_members_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_external_member: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_all_company_member: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub who_can_send_mail: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicMailbox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_mailbox_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicMailboxMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMailgroupReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_send_mail: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateMailgroupMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MailgroupMember>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePublicMailboxReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeleteMailgroupMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreatePublicMailboxMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PublicMailboxMember>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeletePublicMailboxMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMailgroupAliasReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateMailgroupManagerReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailgroup_manager_list: Option<Vec<MailgroupManager>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeleteMailgroupManagerReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailgroup_manager_list: Option<Vec<MailgroupManager>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateMailgroupPermissionMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MailgroupPermissionMember>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeleteMailgroupPermissionMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_member_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePublicMailboxAliasReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryUserReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUserMailboxAliasReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_alias: Option<String>,
}

// ── Domain types for alias / manager / permission_member resources ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAlias {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupManager {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupPermissionMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailgroup: Option<Mailgroup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupListData {
    #[serde(default)]
    pub items: Vec<Mailgroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupMemberData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<MailgroupMember>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupMemberListData {
    #[serde(default)]
    pub items: Vec<MailgroupMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicMailboxData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_mailbox: Option<PublicMailbox>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicMailboxListData {
    #[serde(default)]
    pub items: Vec<PublicMailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicMailboxMemberData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<PublicMailboxMember>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicMailboxMemberListData {
    #[serde(default)]
    pub items: Vec<PublicMailboxMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupPermissionMemberData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_member: Option<MailgroupPermissionMember>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupPermissionMemberListData {
    #[serde(default)]
    pub items: Vec<MailgroupPermissionMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailgroupManagerListData {
    #[serde(default)]
    pub items: Vec<MailgroupManager>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateMailgroupResp, MailgroupData);
impl_resp!(GetMailgroupResp, MailgroupData);
impl_resp!(ListMailgroupResp, MailgroupListData);
impl_resp!(GetMailgroupMemberResp, MailgroupMemberData);
impl_resp!(ListMailgroupMemberResp, MailgroupMemberListData);
impl_resp!(CreatePublicMailboxResp, PublicMailboxData);
impl_resp!(GetPublicMailboxResp, PublicMailboxData);
impl_resp!(ListPublicMailboxResp, PublicMailboxListData);
impl_resp!(ListPublicMailboxMemberResp, PublicMailboxMemberListData);
impl_resp!(GetPublicMailboxMemberResp, PublicMailboxMemberData);
impl_resp!(
    GetMailgroupPermissionMemberResp,
    MailgroupPermissionMemberData
);
impl_resp!(
    ListMailgroupPermissionMemberResp,
    MailgroupPermissionMemberListData
);
impl_resp!(ListMailgroupManagerResp, MailgroupManagerListData);

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListMailgroupQuery<'a> {
    pub manager_user_id: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListMailgroupQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn manager_user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.manager_user_id = value.into();
        self
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateMailgroupQuery<'a> {
    pub body: &'a CreateMailgroupReqBody,
}

impl<'a> CreateMailgroupQuery<'a> {
    pub fn new(body: &'a CreateMailgroupReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetMailgroupQuery<'a> {
    pub mailgroup_id: &'a str,
}

impl<'a> GetMailgroupQuery<'a> {
    pub fn new(mailgroup_id: &'a str) -> Self {
        Self { mailgroup_id }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MailgroupBodyQuery<'a> {
    pub mailgroup_id: &'a str,
    pub body: &'a CreateMailgroupReqBody,
}

impl<'a> MailgroupBodyQuery<'a> {
    pub fn new(mailgroup_id: &'a str, body: &'a CreateMailgroupReqBody) -> Self {
        Self { mailgroup_id, body }
    }
}

pub type PatchMailgroupQuery<'a> = MailgroupBodyQuery<'a>;
pub type UpdateMailgroupQuery<'a> = MailgroupBodyQuery<'a>;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListMailgroupMemberQuery<'a> {
    pub mailgroup_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListMailgroupMemberQuery<'a> {
    pub fn new(mailgroup_id: &'a str) -> Self {
        Self {
            mailgroup_id,
            user_id_type: None,
            page: PageQuery::default(),
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateMailgroupMemberQuery<'a> {
    pub mailgroup_id: &'a str,
    pub body: &'a MailgroupMember,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateMailgroupMemberQuery<'a> {
    pub fn new(mailgroup_id: &'a str, body: &'a MailgroupMember) -> Self {
        Self {
            mailgroup_id,
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
pub struct BatchCreateMailgroupMemberQuery<'a> {
    pub mailgroup_id: &'a str,
    pub body: &'a BatchCreateMailgroupMemberReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchCreateMailgroupMemberQuery<'a> {
    pub fn new(mailgroup_id: &'a str, body: &'a BatchCreateMailgroupMemberReqBody) -> Self {
        Self {
            mailgroup_id,
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
pub struct BatchDeleteMailgroupMemberQuery<'a> {
    pub mailgroup_id: &'a str,
    pub body: &'a BatchDeleteMailgroupMemberReqBody,
}

impl<'a> BatchDeleteMailgroupMemberQuery<'a> {
    pub fn new(mailgroup_id: &'a str, body: &'a BatchDeleteMailgroupMemberReqBody) -> Self {
        Self { mailgroup_id, body }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetMailgroupMemberQuery<'a> {
    pub mailgroup_id: &'a str,
    pub member_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetMailgroupMemberQuery<'a> {
    pub fn new(mailgroup_id: &'a str, member_id: &'a str) -> Self {
        Self {
            mailgroup_id,
            member_id,
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
pub struct ListPublicMailboxQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListPublicMailboxQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreatePublicMailboxQuery<'a> {
    pub body: &'a CreatePublicMailboxReqBody,
}

impl<'a> CreatePublicMailboxQuery<'a> {
    pub fn new(body: &'a CreatePublicMailboxReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetPublicMailboxQuery<'a> {
    pub public_mailbox_id: &'a str,
}

impl<'a> GetPublicMailboxQuery<'a> {
    pub fn new(public_mailbox_id: &'a str) -> Self {
        Self { public_mailbox_id }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PublicMailboxBodyQuery<'a> {
    pub public_mailbox_id: &'a str,
    pub body: &'a CreatePublicMailboxReqBody,
}

impl<'a> PublicMailboxBodyQuery<'a> {
    pub fn new(public_mailbox_id: &'a str, body: &'a CreatePublicMailboxReqBody) -> Self {
        Self {
            public_mailbox_id,
            body,
        }
    }
}

pub type PatchPublicMailboxQuery<'a> = PublicMailboxBodyQuery<'a>;
pub type UpdatePublicMailboxQuery<'a> = PublicMailboxBodyQuery<'a>;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListPublicMailboxMemberQuery<'a> {
    pub public_mailbox_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListPublicMailboxMemberQuery<'a> {
    pub fn new(public_mailbox_id: &'a str) -> Self {
        Self {
            public_mailbox_id,
            user_id_type: None,
            page: PageQuery::default(),
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreatePublicMailboxMemberQuery<'a> {
    pub public_mailbox_id: &'a str,
    pub body: &'a PublicMailboxMember,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreatePublicMailboxMemberQuery<'a> {
    pub fn new(public_mailbox_id: &'a str, body: &'a PublicMailboxMember) -> Self {
        Self {
            public_mailbox_id,
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
pub struct BatchCreatePublicMailboxMemberQuery<'a> {
    pub public_mailbox_id: &'a str,
    pub body: &'a BatchCreatePublicMailboxMemberReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchCreatePublicMailboxMemberQuery<'a> {
    pub fn new(
        public_mailbox_id: &'a str,
        body: &'a BatchCreatePublicMailboxMemberReqBody,
    ) -> Self {
        Self {
            public_mailbox_id,
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
pub struct BatchDeletePublicMailboxMemberQuery<'a> {
    pub public_mailbox_id: &'a str,
    pub body: &'a BatchDeletePublicMailboxMemberReqBody,
}

impl<'a> BatchDeletePublicMailboxMemberQuery<'a> {
    pub fn new(
        public_mailbox_id: &'a str,
        body: &'a BatchDeletePublicMailboxMemberReqBody,
    ) -> Self {
        Self {
            public_mailbox_id,
            body,
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetPublicMailboxMemberQuery<'a> {
    pub public_mailbox_id: &'a str,
    pub member_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetPublicMailboxMemberQuery<'a> {
    pub fn new(public_mailbox_id: &'a str, member_id: &'a str) -> Self {
        Self {
            public_mailbox_id,
            member_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

// ── Resources ──

pub struct MailgroupResource<'a> {
    config: &'a Config,
}

impl<'a> MailgroupResource<'a> {
    pub async fn create(
        &self,
        body: &CreateMailgroupReqBody,
        option: &RequestOption,
    ) -> Result<CreateMailgroupResp, LarkError> {
        let query = CreateMailgroupQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateMailgroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateMailgroupResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/mail/v1/mailgroups",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<MailgroupData, CreateMailgroupResp>()
        .await
    }

    pub async fn get(
        &self,
        mailgroup_id: &str,
        option: &RequestOption,
    ) -> Result<GetMailgroupResp, LarkError> {
        let query = GetMailgroupQuery::new(mailgroup_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetMailgroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMailgroupResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}", query.mailgroup_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<MailgroupData, GetMailgroupResp>()
        .await
    }

    pub async fn delete(
        &self,
        mailgroup_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}");
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
        manager_user_id: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListMailgroupResp, LarkError> {
        let query = ListMailgroupQuery::new()
            .manager_user_id(manager_user_id)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListMailgroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListMailgroupResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/mail/v1/mailgroups",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("manager_user_id", query.manager_user_id)
        .page_query(query.page)
        .send_response::<MailgroupListData, ListMailgroupResp>()
        .await
    }

    /// PATCH /open-apis/mail/v1/mailgroups/:mailgroup_id
    pub async fn patch(
        &self,
        mailgroup_id: &str,
        body: &CreateMailgroupReqBody,
        option: &RequestOption,
    ) -> Result<GetMailgroupResp, LarkError> {
        let query = PatchMailgroupQuery::new(mailgroup_id, body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchMailgroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMailgroupResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}", query.mailgroup_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<MailgroupData, GetMailgroupResp>()
        .await
    }

    /// PUT /open-apis/mail/v1/mailgroups/:mailgroup_id
    pub async fn update(
        &self,
        mailgroup_id: &str,
        body: &CreateMailgroupReqBody,
        option: &RequestOption,
    ) -> Result<GetMailgroupResp, LarkError> {
        let query = UpdateMailgroupQuery::new(mailgroup_id, body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateMailgroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMailgroupResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}", query.mailgroup_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<MailgroupData, GetMailgroupResp>()
        .await
    }
}

pub struct MailgroupMemberResource<'a> {
    config: &'a Config,
}

impl<'a> MailgroupMemberResource<'a> {
    pub async fn create(
        &self,
        mailgroup_id: &str,
        body: &MailgroupMember,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMailgroupMemberResp, LarkError> {
        let query = CreateMailgroupMemberQuery::new(mailgroup_id, body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateMailgroupMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMailgroupMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members",
            query.mailgroup_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<MailgroupMemberData, GetMailgroupMemberResp>()
        .await
    }

    pub async fn delete(
        &self,
        mailgroup_id: &str,
        member_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/members/{member_id}");
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
        mailgroup_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListMailgroupMemberResp, LarkError> {
        let query = ListMailgroupMemberQuery::new(mailgroup_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListMailgroupMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListMailgroupMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members",
            query.mailgroup_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_response::<MailgroupMemberListData, ListMailgroupMemberResp>()
        .await
    }

    /// POST /open-apis/mail/v1/mailgroups/:mailgroup_id/members/batch_create
    pub async fn batch_create(
        &self,
        mailgroup_id: &str,
        body: &BatchCreateMailgroupMemberReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchCreateMailgroupMemberResp, LarkError> {
        let query =
            BatchCreateMailgroupMemberQuery::new(mailgroup_id, body).user_id_type(user_id_type);
        self.batch_create_by_query(&query, option).await
    }

    pub async fn batch_create_by_query(
        &self,
        query: &BatchCreateMailgroupMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchCreateMailgroupMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/batch_create",
            query.mailgroup_id
        );
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchCreateMailgroupMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/mailgroups/:mailgroup_id/members/batch_delete
    pub async fn batch_delete(
        &self,
        mailgroup_id: &str,
        body: &BatchDeleteMailgroupMemberReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = BatchDeleteMailgroupMemberQuery::new(mailgroup_id, body);
        self.batch_delete_by_query(&query, option).await
    }

    pub async fn batch_delete_by_query(
        &self,
        query: &BatchDeleteMailgroupMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/batch_delete",
            query.mailgroup_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_empty()
        .await
    }

    /// GET /open-apis/mail/v1/mailgroups/:mailgroup_id/members/:member_id
    pub async fn get(
        &self,
        mailgroup_id: &str,
        member_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMailgroupMemberResp, LarkError> {
        let query =
            GetMailgroupMemberQuery::new(mailgroup_id, member_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetMailgroupMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMailgroupMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/{}",
            query.mailgroup_id, query.member_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<MailgroupMemberData, GetMailgroupMemberResp>()
        .await
    }
}

pub struct PublicMailboxResource<'a> {
    config: &'a Config,
}

impl<'a> PublicMailboxResource<'a> {
    pub async fn create(
        &self,
        body: &CreatePublicMailboxReqBody,
        option: &RequestOption,
    ) -> Result<CreatePublicMailboxResp, LarkError> {
        let query = CreatePublicMailboxQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreatePublicMailboxQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreatePublicMailboxResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/mail/v1/public_mailboxes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<PublicMailboxData, CreatePublicMailboxResp>()
        .await
    }

    pub async fn get(
        &self,
        public_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxResp, LarkError> {
        let query = GetPublicMailboxQuery::new(public_mailbox_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetPublicMailboxQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}",
            query.public_mailbox_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<PublicMailboxData, GetPublicMailboxResp>()
        .await
    }

    pub async fn delete(
        &self,
        public_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}");
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
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListPublicMailboxResp, LarkError> {
        let query =
            ListPublicMailboxQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPublicMailboxQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPublicMailboxResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/mail/v1/public_mailboxes",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .send_response::<PublicMailboxListData, ListPublicMailboxResp>()
        .await
    }

    /// PATCH /open-apis/mail/v1/public_mailboxes/:public_mailbox_id
    pub async fn patch(
        &self,
        public_mailbox_id: &str,
        body: &CreatePublicMailboxReqBody,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxResp, LarkError> {
        let query = PatchPublicMailboxQuery::new(public_mailbox_id, body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchPublicMailboxQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}",
            query.public_mailbox_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<PublicMailboxData, GetPublicMailboxResp>()
        .await
    }

    /// DELETE /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/remove_to_recycle_bin
    pub async fn remove_to_recycle_bin(
        &self,
        public_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/remove_to_recycle_bin"
        );
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

    /// PUT /open-apis/mail/v1/public_mailboxes/:public_mailbox_id
    pub async fn update(
        &self,
        public_mailbox_id: &str,
        body: &CreatePublicMailboxReqBody,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxResp, LarkError> {
        let query = UpdatePublicMailboxQuery::new(public_mailbox_id, body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdatePublicMailboxQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}",
            query.public_mailbox_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<PublicMailboxData, GetPublicMailboxResp>()
        .await
    }
}

pub struct PublicMailboxMemberResource<'a> {
    config: &'a Config,
}

impl<'a> PublicMailboxMemberResource<'a> {
    pub async fn create(
        &self,
        public_mailbox_id: &str,
        body: &PublicMailboxMember,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query =
            CreatePublicMailboxMemberQuery::new(public_mailbox_id, body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreatePublicMailboxMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members",
            query.public_mailbox_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
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
        public_mailbox_id: &str,
        member_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/members/{member_id}");
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
        public_mailbox_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListPublicMailboxMemberResp, LarkError> {
        let query = ListPublicMailboxMemberQuery::new(public_mailbox_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPublicMailboxMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPublicMailboxMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members",
            query.public_mailbox_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_response::<PublicMailboxMemberListData, ListPublicMailboxMemberResp>()
        .await
    }

    /// POST /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/batch_create
    pub async fn batch_create(
        &self,
        public_mailbox_id: &str,
        body: &BatchCreatePublicMailboxMemberReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchCreatePublicMailboxMemberResp, LarkError> {
        let query = BatchCreatePublicMailboxMemberQuery::new(public_mailbox_id, body)
            .user_id_type(user_id_type);
        self.batch_create_by_query(&query, option).await
    }

    pub async fn batch_create_by_query(
        &self,
        query: &BatchCreatePublicMailboxMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchCreatePublicMailboxMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members/batch_create",
            query.public_mailbox_id
        );
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchCreatePublicMailboxMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/batch_delete
    pub async fn batch_delete(
        &self,
        public_mailbox_id: &str,
        body: &BatchDeletePublicMailboxMemberReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = BatchDeletePublicMailboxMemberQuery::new(public_mailbox_id, body);
        self.batch_delete_by_query(&query, option).await
    }

    pub async fn batch_delete_by_query(
        &self,
        query: &BatchDeletePublicMailboxMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members/batch_delete",
            query.public_mailbox_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_empty()
        .await
    }

    /// POST /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/clear
    pub async fn clear(
        &self,
        public_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/members/clear");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_empty()
        .await
    }

    /// GET /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/:member_id
    pub async fn get(
        &self,
        public_mailbox_id: &str,
        member_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxMemberResp, LarkError> {
        let query = GetPublicMailboxMemberQuery::new(public_mailbox_id, member_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetPublicMailboxMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members/{}",
            query.public_mailbox_id, query.member_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<PublicMailboxMemberData, GetPublicMailboxMemberResp>()
        .await
    }
}

// ── Helpers for newer resources (use Option<CodeError> pattern) ──

// ── Domain types for new resources ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentDownloadUrlItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MailAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<MailAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<MailAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_from: Option<MailAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_state: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_plain_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Folder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_message_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_thread_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleConditionItem {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RuleConditionItem>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleActionItem {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RuleActionItem>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<RuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_the_rest_of_rules: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enable: Option<bool>,
}

// ── Request body types for new resources ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct SendUserMailboxMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<MailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<MailAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_plain_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedupe_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_from: Option<MailAddress>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ReorderUserMailboxRuleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<Vec<String>>,
}

// ── Response types for new resources ──

impl_resp_v2!(GetUserMailboxMessageResp, serde_json::Value);
impl_resp_v2!(GetByCardUserMailboxMessageResp, serde_json::Value);
impl_resp_v2!(ListUserMailboxMessageResp, serde_json::Value);
impl_resp_v2!(SendUserMailboxMessageResp, serde_json::Value);
impl_resp_v2!(
    DownloadUrlUserMailboxMessageAttachmentResp,
    serde_json::Value
);
impl_resp_v2!(CreateUserMailboxFolderResp, serde_json::Value);
impl_resp_v2!(DeleteUserMailboxFolderResp, ());
impl_resp_v2!(ListUserMailboxFolderResp, serde_json::Value);
impl_resp_v2!(PatchUserMailboxFolderResp, serde_json::Value);
impl_resp_v2!(CreateUserMailboxMailContactResp, serde_json::Value);
impl_resp_v2!(DeleteUserMailboxMailContactResp, ());
impl_resp_v2!(ListUserMailboxMailContactResp, serde_json::Value);
impl_resp_v2!(PatchUserMailboxMailContactResp, serde_json::Value);
impl_resp_v2!(CreateUserMailboxRuleResp, serde_json::Value);
impl_resp_v2!(DeleteUserMailboxRuleResp, ());
impl_resp_v2!(ListUserMailboxRuleResp, serde_json::Value);
impl_resp_v2!(ReorderUserMailboxRuleResp, ());
impl_resp_v2!(UpdateUserMailboxRuleResp, serde_json::Value);
impl_resp_v2!(BatchCreateMailgroupMemberResp, serde_json::Value);
impl_resp_v2!(BatchCreatePublicMailboxMemberResp, serde_json::Value);
impl_resp_v2!(CreateMailgroupAliasResp, serde_json::Value);
impl_resp_v2!(ListMailgroupAliasResp, serde_json::Value);
impl_resp_v2!(BatchCreateMailgroupManagerResp, serde_json::Value);
impl_resp_v2!(CreateMailgroupPermissionMemberResp, serde_json::Value);
impl_resp_v2!(BatchCreateMailgroupPermissionMemberResp, serde_json::Value);
impl_resp_v2!(CreatePublicMailboxAliasResp, serde_json::Value);
impl_resp_v2!(ListPublicMailboxAliasResp, serde_json::Value);
impl_resp_v2!(QueryUserResp, serde_json::Value);
impl_resp_v2!(DeleteUserMailboxResp, ());
impl_resp_v2!(CreateUserMailboxAliasResp, serde_json::Value);
impl_resp_v2!(ListUserMailboxAliasResp, serde_json::Value);
impl_resp_v2!(DeleteUserMailboxAliasResp, ());
impl_resp_v2!(SubscribeUserMailboxEventResp, serde_json::Value);
impl_resp_v2!(SubscriptionUserMailboxEventResp, serde_json::Value);
impl_resp_v2!(UnsubscribeUserMailboxEventResp, ());

// ── UserMailboxMessage resource ──

pub struct UserMailboxMessageResource<'a> {
    config: &'a Config,
}

impl UserMailboxMessageResource<'_> {
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id
    pub async fn get(
        &self,
        user_mailbox_id: &str,
        message_id: &str,
        option: &RequestOption,
    ) -> Result<GetUserMailboxMessageResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(GetUserMailboxMessageResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/get_by_card
    pub async fn get_by_card(
        &self,
        user_mailbox_id: &str,
        card_id: Option<&str>,
        owner_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetByCardUserMailboxMessageResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/get_by_card");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("card_id", card_id)
        .query("owner_id", owner_id)
        .query("user_id_type", user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(GetByCardUserMailboxMessageResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        folder_id: Option<&str>,
        only_unread: Option<bool>,
        option: &RequestOption,
    ) -> Result<ListUserMailboxMessageResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .query("folder_id", folder_id)
        .query("only_unread", only_unread)
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(ListUserMailboxMessageResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/send
    pub async fn send(
        &self,
        user_mailbox_id: &str,
        body: &SendUserMailboxMessageReqBody,
        option: &RequestOption,
    ) -> Result<SendUserMailboxMessageResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/send");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(SendUserMailboxMessageResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserMailboxMessageAttachment resource ──

pub struct UserMailboxMessageAttachmentResource<'a> {
    config: &'a Config,
}

impl UserMailboxMessageAttachmentResource<'_> {
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/attachments/download_url
    pub async fn download_url(
        &self,
        user_mailbox_id: &str,
        message_id: &str,
        attachment_ids: &[&str],
        option: &RequestOption,
    ) -> Result<DownloadUrlUserMailboxMessageAttachmentResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}/attachments/download_url"
        );
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query_values("attachment_ids", Some(attachment_ids.iter().copied()))
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(DownloadUrlUserMailboxMessageAttachmentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserMailboxFolder resource ──

pub struct UserMailboxFolderResource<'a> {
    config: &'a Config,
}

impl UserMailboxFolderResource<'_> {
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders
    pub async fn create(
        &self,
        user_mailbox_id: &str,
        body: &Folder,
        option: &RequestOption,
    ) -> Result<CreateUserMailboxFolderResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(CreateUserMailboxFolderResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        folder_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteUserMailboxFolderResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2::<()>()
        .await?;

        Ok(DeleteUserMailboxFolderResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        folder_type: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListUserMailboxFolderResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("folder_type", folder_type)
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(ListUserMailboxFolderResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id
    pub async fn patch(
        &self,
        user_mailbox_id: &str,
        folder_id: &str,
        body: &Folder,
        option: &RequestOption,
    ) -> Result<PatchUserMailboxFolderResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(PatchUserMailboxFolderResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserMailboxMailContact resource ──

pub struct UserMailboxMailContactResource<'a> {
    config: &'a Config,
}

impl UserMailboxMailContactResource<'_> {
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts
    pub async fn create(
        &self,
        user_mailbox_id: &str,
        body: &MailContact,
        option: &RequestOption,
    ) -> Result<CreateUserMailboxMailContactResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(CreateUserMailboxMailContactResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts/:mail_contact_id
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        mail_contact_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteUserMailboxMailContactResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{mail_contact_id}"
        );
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2::<()>()
        .await?;

        Ok(DeleteUserMailboxMailContactResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListUserMailboxMailContactResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(ListUserMailboxMailContactResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts/:mail_contact_id
    pub async fn patch(
        &self,
        user_mailbox_id: &str,
        mail_contact_id: &str,
        body: &MailContact,
        option: &RequestOption,
    ) -> Result<PatchUserMailboxMailContactResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{mail_contact_id}"
        );
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(PatchUserMailboxMailContactResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserMailboxRule resource ──

pub struct UserMailboxRuleResource<'a> {
    config: &'a Config,
}

impl UserMailboxRuleResource<'_> {
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules
    pub async fn create(
        &self,
        user_mailbox_id: &str,
        body: &Rule,
        option: &RequestOption,
    ) -> Result<CreateUserMailboxRuleResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(CreateUserMailboxRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/:rule_id
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        rule_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteUserMailboxRuleResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2::<()>()
        .await?;

        Ok(DeleteUserMailboxRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<ListUserMailboxRuleResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(ListUserMailboxRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/reorder
    pub async fn reorder(
        &self,
        user_mailbox_id: &str,
        body: &ReorderUserMailboxRuleReqBody,
        option: &RequestOption,
    ) -> Result<ReorderUserMailboxRuleResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/reorder");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<()>()
        .await?;

        Ok(ReorderUserMailboxRuleResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PUT /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/:rule_id
    pub async fn update(
        &self,
        user_mailbox_id: &str,
        rule_id: &str,
        body: &Rule,
        option: &RequestOption,
    ) -> Result<UpdateUserMailboxRuleResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(UpdateUserMailboxRuleResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── MailgroupAlias resource ──

pub struct MailgroupAliasResource<'a> {
    config: &'a Config,
}

impl MailgroupAliasResource<'_> {
    /// POST /open-apis/mail/v1/mailgroups/:mailgroup_id/aliases
    pub async fn create(
        &self,
        mailgroup_id: &str,
        body: &CreateMailgroupAliasReqBody,
        option: &RequestOption,
    ) -> Result<CreateMailgroupAliasResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/aliases");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(CreateMailgroupAliasResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/mailgroups/:mailgroup_id/aliases/:alias_id
    pub async fn delete(
        &self,
        mailgroup_id: &str,
        alias_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/aliases/{alias_id}");
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

    /// GET /open-apis/mail/v1/mailgroups/:mailgroup_id/aliases
    pub async fn list(
        &self,
        mailgroup_id: &str,
        option: &RequestOption,
    ) -> Result<ListMailgroupAliasResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/aliases");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(ListMailgroupAliasResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── MailgroupManager resource ──

pub struct MailgroupManagerResource<'a> {
    config: &'a Config,
}

impl MailgroupManagerResource<'_> {
    /// POST /open-apis/mail/v1/mailgroups/:mailgroup_id/managers/batch_create
    pub async fn batch_create(
        &self,
        mailgroup_id: &str,
        body: &BatchCreateMailgroupManagerReqBody,
        option: &RequestOption,
    ) -> Result<BatchCreateMailgroupManagerResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_create");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(BatchCreateMailgroupManagerResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/mail/v1/mailgroups/:mailgroup_id/managers/batch_delete
    pub async fn batch_delete(
        &self,
        mailgroup_id: &str,
        body: &BatchDeleteMailgroupManagerReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_delete");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    /// GET /open-apis/mail/v1/mailgroups/:mailgroup_id/managers
    pub async fn list(
        &self,
        mailgroup_id: &str,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListMailgroupManagerResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_token", page_token)
        .query("page_size", page_size)
        .send_response::<MailgroupManagerListData, ListMailgroupManagerResp>()
        .await
    }
}

// ── MailgroupPermissionMember resource ──

pub struct MailgroupPermissionMemberResource<'a> {
    config: &'a Config,
}

impl MailgroupPermissionMemberResource<'_> {
    /// POST /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members
    pub async fn create(
        &self,
        mailgroup_id: &str,
        body: &MailgroupPermissionMember,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateMailgroupPermissionMemberResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/permission_members");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(CreateMailgroupPermissionMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/:permission_member_id
    pub async fn delete(
        &self,
        mailgroup_id: &str,
        permission_member_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{mailgroup_id}/permission_members/{permission_member_id}"
        );
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

    /// GET /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/:permission_member_id
    pub async fn get(
        &self,
        mailgroup_id: &str,
        permission_member_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMailgroupPermissionMemberResp, LarkError> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{mailgroup_id}/permission_members/{permission_member_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_response::<MailgroupPermissionMemberData, GetMailgroupPermissionMemberResp>()
        .await
    }

    /// GET /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members
    pub async fn list(
        &self,
        mailgroup_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListMailgroupPermissionMemberResp, LarkError> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/permission_members");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .query("page_token", page_token)
        .query("page_size", page_size)
        .send_response::<MailgroupPermissionMemberListData, ListMailgroupPermissionMemberResp>()
        .await
    }

    /// POST /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/batch_create
    pub async fn batch_create(
        &self,
        mailgroup_id: &str,
        body: &BatchCreateMailgroupPermissionMemberReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchCreateMailgroupPermissionMemberResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/permission_members/batch_create");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchCreateMailgroupPermissionMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/batch_delete
    pub async fn batch_delete(
        &self,
        mailgroup_id: &str,
        body: &BatchDeleteMailgroupPermissionMemberReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/permission_members/batch_delete");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }
}

// ── PublicMailboxAlias resource ──

pub struct PublicMailboxAliasResource<'a> {
    config: &'a Config,
}

impl PublicMailboxAliasResource<'_> {
    /// POST /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases
    pub async fn create(
        &self,
        public_mailbox_id: &str,
        body: &CreatePublicMailboxAliasReqBody,
        option: &RequestOption,
    ) -> Result<CreatePublicMailboxAliasResp, LarkError> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/aliases");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(CreatePublicMailboxAliasResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases/:alias_id
    pub async fn delete(
        &self,
        public_mailbox_id: &str,
        alias_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/aliases/{alias_id}");
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

    /// GET /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases
    pub async fn list(
        &self,
        public_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<ListPublicMailboxAliasResp, LarkError> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/aliases");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(ListPublicMailboxAliasResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── User resource ──

pub struct UserResource<'a> {
    config: &'a Config,
}

impl UserResource<'_> {
    /// POST /open-apis/mail/v1/users/query
    pub async fn query(
        &self,
        body: &QueryUserReqBody,
        option: &RequestOption,
    ) -> Result<QueryUserResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/mail/v1/users/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(QueryUserResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserMailbox resource ──

pub struct UserMailboxResource<'a> {
    config: &'a Config,
}

impl UserMailboxResource<'_> {
    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        transfer_mailbox: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeleteUserMailboxResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("transfer_mailbox", transfer_mailbox)
        .send_v2::<()>()
        .await?;

        Ok(DeleteUserMailboxResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserMailboxAlias resource ──

pub struct UserMailboxAliasResource<'a> {
    config: &'a Config,
}

impl UserMailboxAliasResource<'_> {
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases
    pub async fn create(
        &self,
        user_mailbox_id: &str,
        body: &CreateUserMailboxAliasReqBody,
        option: &RequestOption,
    ) -> Result<CreateUserMailboxAliasResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/aliases");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(CreateUserMailboxAliasResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases/:alias_id
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        alias_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteUserMailboxAliasResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/aliases/{alias_id}");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;

        Ok(DeleteUserMailboxAliasResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<ListUserMailboxAliasResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/aliases");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(ListUserMailboxAliasResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserMailboxEvent resource ──

pub struct UserMailboxEventResource<'a> {
    config: &'a Config,
}

impl UserMailboxEventResource<'_> {
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/subscribe
    pub async fn subscribe(
        &self,
        user_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<SubscribeUserMailboxEventResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/event/subscribe");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(SubscribeUserMailboxEventResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/subscription
    pub async fn subscription(
        &self,
        user_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<SubscriptionUserMailboxEventResp, LarkError> {
        let path =
            format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/event/subscription");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;

        Ok(SubscriptionUserMailboxEventResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/unsubscribe
    pub async fn unsubscribe(
        &self,
        user_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<UnsubscribeUserMailboxEventResp, LarkError> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/event/unsubscribe");
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_v2::<()>()
        .await?;

        Ok(UnsubscribeUserMailboxEventResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub mailgroup: MailgroupResource<'a>,
    pub mailgroup_alias: MailgroupAliasResource<'a>,
    pub mailgroup_manager: MailgroupManagerResource<'a>,
    pub mailgroup_member: MailgroupMemberResource<'a>,
    pub mailgroup_permission_member: MailgroupPermissionMemberResource<'a>,
    pub public_mailbox: PublicMailboxResource<'a>,
    pub public_mailbox_alias: PublicMailboxAliasResource<'a>,
    pub public_mailbox_member: PublicMailboxMemberResource<'a>,
    pub user: UserResource<'a>,
    pub user_mailbox: UserMailboxResource<'a>,
    pub user_mailbox_alias: UserMailboxAliasResource<'a>,
    pub user_mailbox_event: UserMailboxEventResource<'a>,
    pub user_mailbox_message: UserMailboxMessageResource<'a>,
    pub user_mailbox_message_attachment: UserMailboxMessageAttachmentResource<'a>,
    pub user_mailbox_folder: UserMailboxFolderResource<'a>,
    pub user_mailbox_mail_contact: UserMailboxMailContactResource<'a>,
    pub user_mailbox_rule: UserMailboxRuleResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            mailgroup: MailgroupResource { config },
            mailgroup_alias: MailgroupAliasResource { config },
            mailgroup_manager: MailgroupManagerResource { config },
            mailgroup_member: MailgroupMemberResource { config },
            mailgroup_permission_member: MailgroupPermissionMemberResource { config },
            public_mailbox: PublicMailboxResource { config },
            public_mailbox_alias: PublicMailboxAliasResource { config },
            public_mailbox_member: PublicMailboxMemberResource { config },
            user: UserResource { config },
            user_mailbox: UserMailboxResource { config },
            user_mailbox_alias: UserMailboxAliasResource { config },
            user_mailbox_event: UserMailboxEventResource { config },
            user_mailbox_message: UserMailboxMessageResource { config },
            user_mailbox_message_attachment: UserMailboxMessageAttachmentResource { config },
            user_mailbox_folder: UserMailboxFolderResource { config },
            user_mailbox_mail_contact: UserMailboxMailContactResource { config },
            user_mailbox_rule: UserMailboxRuleResource { config },
        }
    }
}
