use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::{AccessTokenType, HEADER_X_TARGET_SERVICE};
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::token;
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

// ── OAuth 2.0 access token client (authorization_code + refresh_token) ──
//
// This layer sits on top of the OAuth /oauth/v3/token endpoint (the same one used
// by the internal JWT bearer tenant-token flow). It is the recommended API for
// authorization-code and refresh-token exchanges, and supports client-assertion
// mode (JWT bearer) in place of app_secret.

/// Successful OAuth token response data.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token_expires_in: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// OAuth token response wrapping the raw HTTP response and parsed data.
pub struct AccessTokenResp {
    pub api_resp: crate::resp::ApiResp,
    pub data: Option<AccessTokenRespData>,
}

impl AccessTokenResp {
    #[must_use]
    pub fn success(&self) -> bool {
        self.api_resp.status_code == 200
    }
}

/// Error returned when the OAuth token endpoint responds with a non-success
/// status or a non-zero `code` / non-empty `error` field.
#[derive(Debug)]
pub struct AccessTokenError {
    pub api_resp: Option<crate::resp::ApiResp>,
    pub code: i64,
    pub error_type: String,
    pub error_description: String,
}

impl std::fmt::Display for AccessTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = if !self.error_description.is_empty() {
            &self.error_description
        } else if !self.error_type.is_empty() {
            &self.error_type
        } else {
            "access token request failed"
        };
        if let Some(ref api_resp) = self.api_resp {
            write!(
                f,
                "statusCode:{}, code:{}, msg:{}",
                api_resp.status_code, self.code, msg
            )
        } else {
            write!(f, "code:{}, msg:{}", self.code, msg)
        }
    }
}

impl std::error::Error for AccessTokenError {}

/// OAuth 2.0 access token client.
///
/// Provides authorization-code and refresh-token flows against the
/// `/oauth/v3/token` endpoint, with optional client-assertion (JWT bearer)
/// support.
pub struct AccessToken<'a> {
    config: &'a Config,
}

impl<'a> AccessToken<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// Exchange an authorization code for a user access token.
    pub async fn retrieve_by_authorization_code(
        &self,
        code: &str,
        redirect_uri: Option<&str>,
        code_verifier: Option<&str>,
        scope: Option<&str>,
        option: &RequestOption,
    ) -> Result<AccessTokenResp, LarkError> {
        let body = OAuthTokenRequestBody {
            grant_type: "authorization_code".to_string(),
            code: Some(code.to_string()),
            redirect_uri: redirect_uri.map(|s| s.to_string()),
            code_verifier: code_verifier.map(|s| s.to_string()),
            scope: scope.map(|s| s.to_string()),
            ..Default::default()
        };
        self.do_oauth_request(&body, option).await
    }

    /// Refresh a user access token.
    pub async fn refresh(
        &self,
        refresh_token: &str,
        scope: Option<&str>,
        option: &RequestOption,
    ) -> Result<AccessTokenResp, LarkError> {
        let body = OAuthTokenRequestBody {
            grant_type: "refresh_token".to_string(),
            refresh_token: Some(refresh_token.to_string()),
            scope: scope.map(|s| s.to_string()),
            ..Default::default()
        };
        self.do_oauth_request(&body, option).await
    }

    async fn do_oauth_request(
        &self,
        body: &OAuthTokenRequestBody,
        option: &RequestOption,
    ) -> Result<AccessTokenResp, LarkError> {
        let oauth_base_url = token::resolve_oauth_base_url(self.config);
        let mut request_url = format!(
            "{}{}",
            oauth_base_url.trim_end_matches('/'),
            crate::constants::OAUTH_TOKEN_URL_PATH
        );
        let mut request_option = option.clone();

        let mut body = body.clone();
        body.client_id = Some(self.config.app_id.clone());

        if let Some(provider) = self.config.client_assertion_provider() {
            let aud = token::extract_aud_from_url(&oauth_base_url)?;
            let assertion = provider.retrieve_token(&aud).await.map_err(|e| {
                LarkError::ClientAssertion(format!("retrieve client assertion token failed: {e}"))
            })?;
            if assertion.value.is_empty() {
                return Err(LarkError::ClientAssertion(
                    "client assertion token is empty".to_string(),
                ));
            }
            body.client_assertion_type =
                Some(crate::constants::CLIENT_ASSERTION_TYPE_JWT_BEARER.to_string());
            if let Some(ref target) = assertion.target_info {
                let target_service = if target.target_service.contains("://") {
                    target.target_service.clone()
                } else {
                    format!("https://{}", target.target_service)
                };
                request_url = format!(
                    "{}{}{}",
                    target_service.trim_end_matches('/'),
                    target.target_prefix,
                    crate::constants::OAUTH_TOKEN_URL_PATH
                );
                let headers = request_option
                    .headers
                    .get_or_insert_with(http::HeaderMap::new);
                headers.insert(
                    HEADER_X_TARGET_SERVICE,
                    http::HeaderValue::from_str(&aud).map_err(|e| {
                        LarkError::ClientAssertion(format!("invalid target service header: {e}"))
                    })?,
                );
            }
            body.client_assertion = Some(assertion.value);
        } else if !self.config.app_secret.is_empty() {
            body.client_secret = Some(self.config.app_secret.clone());
        } else {
            return Err(LarkError::ClientAssertion(
                "AppSecret and ClientAssertionProvider cannot both be empty for AccessToken APIs"
                    .to_string(),
            ));
        }

        let mut api_req = ApiReq::new(http::Method::POST, &request_url);
        api_req.supported_access_token_types = vec![AccessTokenType::None];
        api_req.body = Some(ReqBody::json(&body)?);

        let api_resp =
            transport::raw_send_absolute_url(self.config, &api_req, &request_option, None).await?;

        let resp_body: OAuthTokenResponseBody =
            serde_json::from_slice(&api_resp.raw_body).unwrap_or_default();

        if api_resp.status_code != 200 || resp_body.code != 0 || !resp_body.error.is_empty() {
            let desc = if !resp_body.error_description.is_empty() {
                resp_body.error_description.clone()
            } else if !resp_body.error.is_empty() {
                resp_body.error.clone()
            } else {
                String::new()
            };
            return Err(LarkError::Api(Box::new(crate::resp::CodeError {
                code: resp_body.code,
                msg: desc,
                ..Default::default()
            })));
        }

        if resp_body.access_token.is_empty() {
            return Err(LarkError::Api(Box::new(crate::resp::CodeError {
                code: resp_body.code,
                msg: "access_token is empty".to_string(),
                ..Default::default()
            })));
        }

        Ok(AccessTokenResp {
            api_resp,
            data: Some(AccessTokenRespData {
                access_token: Some(resp_body.access_token).filter(|s| !s.is_empty()),
                token_type: Some(resp_body.token_type).filter(|s| !s.is_empty()),
                expires_in: if resp_body.expires_in > 0 {
                    Some(resp_body.expires_in)
                } else {
                    None
                },
                refresh_token: Some(resp_body.refresh_token).filter(|s| !s.is_empty()),
                refresh_token_expires_in: if resp_body.refresh_token_expires_in > 0 {
                    Some(resp_body.refresh_token_expires_in)
                } else {
                    None
                },
                scope: Some(resp_body.scope).filter(|s| !s.is_empty()),
            }),
        })
    }
}

#[derive(Debug, Clone, Default, serde::Serialize)]
struct OAuthTokenRequestBody {
    grant_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_assertion_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_assertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_verifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
struct OAuthTokenResponseBody {
    #[serde(default)]
    code: i64,
    #[serde(default)]
    error: String,
    #[serde(default)]
    error_description: String,
    #[serde(default)]
    access_token: String,
    #[serde(default)]
    token_type: String,
    #[serde(default)]
    expires_in: i64,
    #[serde(default)]
    refresh_token: String,
    #[serde(default)]
    refresh_token_expires_in: i64,
    #[serde(default)]
    scope: String,
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
    pub oauth: AccessToken<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            oauth: AccessToken { config },
            access_token: AccessTokenResource { config },
            oidc_access_token: OidcAccessTokenResource { config },
            oidc_refresh_access_token: OidcRefreshAccessTokenResource { config },
            refresh_access_token: RefreshAccessTokenResource { config },
            user_info: UserInfoResource { config },
        }
    }
}
