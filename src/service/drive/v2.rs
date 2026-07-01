use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, RequestOption};
use crate::service::common::RestRequest;
use crate::transport;

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
        .query("page_size", query.page_size)
        .query("page_token", query.page_token)
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

impl<'a> PermissionPublicV2Resource<'a> {
    pub async fn get(
        &self,
        token: &str,
        r#type: &str,
        option: &RequestOption,
    ) -> Result<GetPermissionPublicV2Resp, LarkError> {
        let path = format!("/open-apis/drive/v2/permissions/{token}/public");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", r#type);
        let (api_resp, raw) =
            transport::request_typed::<PermissionPublicV2>(self.config, &api_req, option).await?;
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
        use crate::req::ReqBody;
        let path = format!("/open-apis/drive/v2/permissions/{token}/public");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", r#type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<PermissionPublicV2>(self.config, &api_req, option).await?;
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
