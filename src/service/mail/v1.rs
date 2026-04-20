use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

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
pub struct BatchCreatePublicMailboxMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PublicMailboxMember>>,
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
pub struct PublicMailboxMemberListData {
    #[serde(default)]
    pub items: Vec<PublicMailboxMember>,
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

// ── Resources ──

pub struct MailgroupResource<'a> {
    config: &'a Config,
}

impl<'a> MailgroupResource<'a> {
    pub async fn create(
        &self,
        body: &CreateMailgroupReqBody,
        option: &RequestOption,
    ) -> Result<CreateMailgroupResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/mail/v1/mailgroups");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MailgroupData>(self.config, &api_req, option).await?;
        Ok(CreateMailgroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        mailgroup_id: &str,
        option: &RequestOption,
    ) -> Result<GetMailgroupResp> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<MailgroupData>(self.config, &api_req, option).await?;
        Ok(GetMailgroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, mailgroup_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}");
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
        manager_user_id: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListMailgroupResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/mail/v1/mailgroups");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = manager_user_id {
            api_req.query_params.set("manager_user_id", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<MailgroupListData>(self.config, &api_req, option).await?;
        Ok(ListMailgroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<GetMailgroupMemberResp> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MailgroupMemberData>(self.config, &api_req, option).await?;
        Ok(GetMailgroupMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        mailgroup_id: &str,
        member_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/members/{member_id}");
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
        mailgroup_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListMailgroupMemberResp> {
        let path = format!("/open-apis/mail/v1/mailgroups/{mailgroup_id}/members");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<MailgroupMemberListData>(self.config, &api_req, option)
                .await?;
        Ok(ListMailgroupMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreatePublicMailboxResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/mail/v1/public_mailboxes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<PublicMailboxData>(self.config, &api_req, option).await?;
        Ok(CreatePublicMailboxResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        public_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<GetPublicMailboxResp> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<PublicMailboxData>(self.config, &api_req, option).await?;
        Ok(GetPublicMailboxResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        public_mailbox_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}");
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
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListPublicMailboxResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/mail/v1/public_mailboxes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<PublicMailboxListData>(self.config, &api_req, option)
                .await?;
        Ok(ListPublicMailboxResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(
        &self,
        public_mailbox_id: &str,
        member_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/members/{member_id}");
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
        public_mailbox_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListPublicMailboxMemberResp> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{public_mailbox_id}/members");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<PublicMailboxMemberListData>(self.config, &api_req, option)
                .await?;
        Ok(ListPublicMailboxMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub mailgroup: MailgroupResource<'a>,
    pub mailgroup_member: MailgroupMemberResource<'a>,
    pub public_mailbox: PublicMailboxResource<'a>,
    pub public_mailbox_member: PublicMailboxMemberResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            mailgroup: MailgroupResource { config },
            mailgroup_member: MailgroupMemberResource { config },
            public_mailbox: PublicMailboxResource { config },
            public_mailbox_member: PublicMailboxMemberResource { config },
        }
    }
}
