use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
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

impl<'a> FileLikeResource<'a> {
    pub async fn list(
        &self,
        file_token: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<FileLikeListResp> {
        let path = format!("/open-apis/drive/v2/files/{file_token}/likes");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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
            transport::request_typed::<FileLikeListData>(self.config, &api_req, option).await?;
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
    ) -> Result<GetPermissionPublicV2Resp> {
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
    ) -> Result<PatchPermissionPublicV2Resp> {
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
