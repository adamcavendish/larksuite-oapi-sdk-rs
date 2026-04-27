use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserAccessTokenInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_thumb: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_middle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_big: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_thumb: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_middle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_big: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOidcAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOidcRefreshAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRefreshAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

// ── Response wrappers ──

impl_resp!(CreateAccessTokenResp, UserAccessTokenInfo);
impl_resp!(CreateOidcAccessTokenResp, TokenInfo);
impl_resp!(CreateOidcRefreshAccessTokenResp, TokenInfo);
impl_resp!(CreateRefreshAccessTokenResp, UserAccessTokenInfo);
impl_resp!(GetUserInfoResp, UserInfo);

// ── Resources ──

pub struct AccessTokenResource<'a> {
    config: &'a Config,
}

impl<'a> AccessTokenResource<'a> {
    /// Exchange login pre-auth code for user_access_token (legacy path).
    pub async fn create(
        &self,
        body: &CreateAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<CreateAccessTokenResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/authen/v1/access_token");
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UserAccessTokenInfo>(self.config, &api_req, option).await?;
        Ok(CreateAccessTokenResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct OidcAccessTokenResource<'a> {
    config: &'a Config,
}

impl<'a> OidcAccessTokenResource<'a> {
    /// Exchange login pre-auth code for user_access_token (OIDC path).
    pub async fn create(
        &self,
        body: &CreateOidcAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<CreateOidcAccessTokenResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/authen/v1/oidc/access_token");
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TokenInfo>(self.config, &api_req, option).await?;
        Ok(CreateOidcAccessTokenResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct OidcRefreshAccessTokenResource<'a> {
    config: &'a Config,
}

impl<'a> OidcRefreshAccessTokenResource<'a> {
    /// Refresh user_access_token using refresh_token (OIDC path).
    pub async fn create(
        &self,
        body: &CreateOidcRefreshAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<CreateOidcRefreshAccessTokenResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/authen/v1/oidc/refresh_access_token",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TokenInfo>(self.config, &api_req, option).await?;
        Ok(CreateOidcRefreshAccessTokenResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct RefreshAccessTokenResource<'a> {
    config: &'a Config,
}

impl<'a> RefreshAccessTokenResource<'a> {
    /// Refresh user_access_token (legacy path).
    pub async fn create(
        &self,
        body: &CreateRefreshAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<CreateRefreshAccessTokenResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/authen/v1/refresh_access_token",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UserAccessTokenInfo>(self.config, &api_req, option).await?;
        Ok(CreateRefreshAccessTokenResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct UserInfoResource<'a> {
    config: &'a Config,
}

impl<'a> UserInfoResource<'a> {
    /// Get current user info (requires user_access_token).
    pub async fn get(&self, option: &RequestOption) -> Result<GetUserInfoResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/authen/v1/user_info");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<UserInfo>(self.config, &api_req, option).await?;
        Ok(GetUserInfoResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub access_token: AccessTokenResource<'a>,
    pub oidc_access_token: OidcAccessTokenResource<'a>,
    pub oidc_refresh_access_token: OidcRefreshAccessTokenResource<'a>,
    pub refresh_access_token: RefreshAccessTokenResource<'a>,
    pub user_info: UserInfoResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            access_token: AccessTokenResource { config },
            oidc_access_token: OidcAccessTokenResource { config },
            oidc_refresh_access_token: OidcRefreshAccessTokenResource { config },
            refresh_access_token: RefreshAccessTokenResource { config },
            user_info: UserInfoResource { config },
        }
    }
}
