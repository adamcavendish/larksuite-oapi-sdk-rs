use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLike {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionPublicV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_access_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manage_collaborator_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_share_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_switch: Option<bool>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchPermissionPublicV2ReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_access_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_collaborator_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_switch: Option<bool>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileLikeListData {
    #[serde(default)]
    pub files: Vec<FileLike>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(FileLikeListResp, FileLikeListData);
impl_resp!(GetPermissionPublicV2Resp, PermissionPublicV2);
impl_resp!(PatchPermissionPublicV2Resp, PermissionPublicV2);

// ── Resources ──

pub struct FileLikeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListFileLikeQuery<'a> {
    pub file_token: &'a str,
    pub user_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListFileLikeQuery<'a> {
    pub fn new(file_token: &'a str) -> Self {
        Self {
            file_token,
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
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

impl<'a> FileLikeResource<'a> {
    pub async fn list(
        &self,
        file_token: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<FileLikeListResp, LarkError> {
        let query = ListFileLikeQuery::new(file_token)
            .user_id_type(user_id_type)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFileLikeQuery<'_>,
        option: &RequestOption,
    ) -> Result<FileLikeListResp, LarkError> {
        let path = format!("/open-apis/drive/v2/files/{}/likes", query.file_token);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send::<FileLikeListData>()
        .await?;
        Ok(FileLikeListResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct PermissionPublicV2Resource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetPermissionPublicV2Query<'a> {
    pub token: &'a str,
    pub token_type: &'a str,
}

impl<'a> GetPermissionPublicV2Query<'a> {
    pub fn new(token: &'a str, token_type: &'a str) -> Self {
        Self { token, token_type }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchPermissionPublicV2Query<'a> {
    pub token: &'a str,
    pub token_type: &'a str,
    pub body: &'a PatchPermissionPublicV2ReqBody,
}

impl<'a> PatchPermissionPublicV2Query<'a> {
    pub fn new(
        token: &'a str,
        token_type: &'a str,
        body: &'a PatchPermissionPublicV2ReqBody,
    ) -> Self {
        Self {
            token,
            token_type,
            body,
        }
    }
}

impl<'a> PermissionPublicV2Resource<'a> {
    pub async fn get(
        &self,
        token: &str,
        r#type: &str,
        option: &RequestOption,
    ) -> Result<GetPermissionPublicV2Resp, LarkError> {
        let query = GetPermissionPublicV2Query::new(token, r#type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetPermissionPublicV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetPermissionPublicV2Resp, LarkError> {
        let path = format!("/open-apis/drive/v2/permissions/{}/public", query.token);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("type", query.token_type)
        .send::<PermissionPublicV2>()
        .await?;
        Ok(GetPermissionPublicV2Resp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        token: &str,
        r#type: &str,
        body: &PatchPermissionPublicV2ReqBody,
        option: &RequestOption,
    ) -> Result<PatchPermissionPublicV2Resp, LarkError> {
        let query = PatchPermissionPublicV2Query::new(token, r#type, body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchPermissionPublicV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchPermissionPublicV2Resp, LarkError> {
        let path = format!("/open-apis/drive/v2/permissions/{}/public", query.token);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("type", query.token_type)
        .json_body(query.body)?
        .send::<PermissionPublicV2>()
        .await?;
        Ok(PatchPermissionPublicV2Resp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V2<'a> {
    pub file_like: FileLikeResource<'a>,
    pub permission_public: PermissionPublicV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            file_like: FileLikeResource { config },
            permission_public: PermissionPublicV2Resource { config },
        }
    }
}
