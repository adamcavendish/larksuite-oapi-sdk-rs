use base64::Engine as _;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::{
    AccessTokenType, DEVICE_AUTHORIZATION_URL_PATH, FEISHU_BASE_URL, FEISHU_OAUTH_BASE_URL,
    LARK_BASE_URL, LARK_OAUTH_BASE_URL, TOKEN_REVOCATION_URL_PATH, USER_OAUTH_TOKEN_URL_PATH,
};
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::RestRequest;
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateAccessTokenQuery<'a> {
    pub body: &'a CreateAccessTokenReqBody,
}

impl<'a> CreateAccessTokenQuery<'a> {
    pub fn new(body: &'a CreateAccessTokenReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateOidcAccessTokenQuery<'a> {
    pub body: &'a CreateOidcAccessTokenReqBody,
}

impl<'a> CreateOidcAccessTokenQuery<'a> {
    pub fn new(body: &'a CreateOidcAccessTokenReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateOidcRefreshAccessTokenQuery<'a> {
    pub body: &'a CreateOidcRefreshAccessTokenReqBody,
}

impl<'a> CreateOidcRefreshAccessTokenQuery<'a> {
    pub fn new(body: &'a CreateOidcRefreshAccessTokenReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateRefreshAccessTokenQuery<'a> {
    pub body: &'a CreateRefreshAccessTokenReqBody,
}

impl<'a> CreateRefreshAccessTokenQuery<'a> {
    pub fn new(body: &'a CreateRefreshAccessTokenReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct GetUserInfoQuery;

impl GetUserInfoQuery {
    pub fn new() -> Self {
        Self
    }
}

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
        let query = CreateAccessTokenQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateAccessTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/authen/v1/access_token",
            vec![AccessTokenType::App],
            option,
        )
        .json_body(query.body)?
        .send_response::<UserAccessTokenInfo, CreateAccessTokenResp>()
        .await
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
        let query = CreateOidcAccessTokenQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateOidcAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateOidcAccessTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/authen/v1/oidc/access_token",
            vec![AccessTokenType::App],
            option,
        )
        .json_body(query.body)?
        .send_response::<TokenInfo, CreateOidcAccessTokenResp>()
        .await
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
        let query = CreateOidcRefreshAccessTokenQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateOidcRefreshAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateOidcRefreshAccessTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/authen/v1/oidc/refresh_access_token",
            vec![AccessTokenType::App],
            option,
        )
        .json_body(query.body)?
        .send_response::<TokenInfo, CreateOidcRefreshAccessTokenResp>()
        .await
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
        let query = CreateRefreshAccessTokenQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateRefreshAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateRefreshAccessTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/authen/v1/refresh_access_token",
            vec![AccessTokenType::App],
            option,
        )
        .json_body(query.body)?
        .send_response::<UserAccessTokenInfo, CreateRefreshAccessTokenResp>()
        .await
    }
}

// ── OAuth 2.0 access token client (authorization_code + refresh_token) ──
//
// This layer uses the authen v2 token endpoint for authorization-code and
// refresh-token exchanges, plus Accounts-host device authorization and revocation.

/// Successful OAuth token response data.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
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

/// Device authorization details returned by Feishu or Lark.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceAuthorization {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// OAuth token response wrapping the raw HTTP response and parsed data.
#[non_exhaustive]
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
/// Provides user OAuth authorization-code, refresh-token, device authorization,
/// and token revocation flows.
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
        _scope: Option<&str>,
        option: &RequestOption,
    ) -> Result<AccessTokenResp, LarkError> {
        let body = OAuthTokenRequestBody {
            grant_type: "authorization_code".to_string(),
            code: Some(code.to_string()),
            redirect_uri: redirect_uri.map(|s| s.to_string()),
            code_verifier: code_verifier.map(|s| s.to_string()),
            ..Default::default()
        };
        self.do_oauth_request(&body, OAuthTokenRequestEncoding::Json, option)
            .await
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
        self.do_oauth_request(&body, OAuthTokenRequestEncoding::Json, option)
            .await
    }

    /// Starts an OAuth 2.0 device authorization flow.
    ///
    /// Present `verification_uri_complete` to the user when available, then call
    /// [`Self::poll_device_token`] to wait for approval.
    pub async fn request_device_authorization(
        &self,
        scope: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeviceAuthorization, LarkError> {
        let app_secret = self.require_app_secret()?;
        let mut request_option = option.clone();
        let basic_auth = base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{app_secret}", self.config.app_id));
        request_option
            .headers
            .get_or_insert_with(http::HeaderMap::new)
            .insert(
                http::header::AUTHORIZATION,
                http::HeaderValue::from_str(&format!("Basic {basic_auth}"))
                    .map_err(|err| LarkError::IllegalParam(err.to_string()))?,
            );

        let mut fields = vec![("client_id".to_owned(), self.config.app_id.clone())];
        if let Some(scope) = scope.filter(|scope| !scope.is_empty()) {
            fields.push(("scope".to_owned(), scope.to_owned()));
        }
        let request_url = format!(
            "{}{}",
            oauth_accounts_base_url(&self.config.base_url),
            DEVICE_AUTHORIZATION_URL_PATH
        );
        let mut api_req = ApiReq::new(http::Method::POST, request_url);
        api_req.supported_access_token_types = vec![AccessTokenType::None];
        api_req.body = Some(ReqBody::UrlEncoded(fields));

        let api_resp =
            transport::raw_send_absolute_url(self.config, &api_req, &request_option, None).await?;
        let response: DeviceAuthorizationResponse = serde_json::from_slice(&api_resp.raw_body)
            .map_err(|err| LarkError::Token(err.to_string()))?;
        if !(200..300).contains(&api_resp.status_code) || response.error.is_failure() {
            return Err(oauth_response_error(
                "request device authorization",
                &response.error,
            ));
        }
        if response.device_code.is_empty()
            || response.user_code.is_empty()
            || response.verification_uri.is_empty()
        {
            return Err(LarkError::Token(
                "device authorization response is missing required fields".to_owned(),
            ));
        }
        if response.expires_in == 0 {
            return Err(LarkError::Token(
                "device authorization response has an invalid expires_in".to_owned(),
            ));
        }

        Ok(DeviceAuthorization {
            device_code: response.device_code,
            user_code: response.user_code,
            verification_uri_complete: if response.verification_uri_complete.is_empty() {
                response.verification_uri.clone()
            } else {
                response.verification_uri_complete
            },
            verification_uri: response.verification_uri,
            expires_in: response.expires_in,
            interval: response.interval.max(1),
        })
    }

    /// Polls until the device flow is approved, denied, or expires.
    pub async fn poll_device_token(
        &self,
        device: &DeviceAuthorization,
        option: &RequestOption,
    ) -> Result<AccessTokenResp, LarkError> {
        let deadline =
            tokio::time::Instant::now() + std::time::Duration::from_secs(device.expires_in);
        let mut interval = std::time::Duration::from_secs(device.interval.max(1));

        loop {
            tokio::time::sleep(interval).await;
            if tokio::time::Instant::now() >= deadline {
                return Err(LarkError::Token("device authorization expired".to_owned()));
            }

            let body = OAuthTokenRequestBody {
                grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_owned(),
                device_code: Some(device.device_code.clone()),
                ..Default::default()
            };
            match self
                .do_oauth_request(&body, OAuthTokenRequestEncoding::UrlEncoded, option)
                .await
            {
                Ok(token) => return Ok(token),
                Err(error) => {
                    let message = error.to_string();
                    if message.contains("authorization_pending") {
                        continue;
                    }
                    if message.contains("slow_down") {
                        interval += std::time::Duration::from_secs(5);
                        continue;
                    }
                    return Err(error);
                }
            }
        }
    }

    /// Revokes an OAuth access or refresh token.
    pub async fn revoke_token(
        &self,
        token: &str,
        token_type_hint: Option<&str>,
        option: &RequestOption,
    ) -> Result<(), LarkError> {
        let app_secret = self.require_app_secret()?;
        if token.is_empty() {
            return Err(LarkError::IllegalParam(
                "token must not be empty".to_owned(),
            ));
        }

        let mut fields = vec![
            ("client_id".to_owned(), self.config.app_id.clone()),
            ("client_secret".to_owned(), app_secret.to_owned()),
            ("token".to_owned(), token.to_owned()),
        ];
        if let Some(token_type_hint) = token_type_hint.filter(|hint| !hint.is_empty()) {
            fields.push(("token_type_hint".to_owned(), token_type_hint.to_owned()));
        }
        let request_url = format!(
            "{}{}",
            oauth_accounts_base_url(&self.config.base_url),
            TOKEN_REVOCATION_URL_PATH
        );
        let mut api_req = ApiReq::new(http::Method::POST, request_url);
        api_req.supported_access_token_types = vec![AccessTokenType::None];
        api_req.body = Some(ReqBody::UrlEncoded(fields));

        let api_resp =
            transport::raw_send_absolute_url(self.config, &api_req, option, None).await?;
        if (200..300).contains(&api_resp.status_code)
            && api_resp
                .raw_body
                .iter()
                .all(|byte| byte.is_ascii_whitespace())
        {
            return Ok(());
        }

        let response: OAuthErrorResponse = serde_json::from_slice(&api_resp.raw_body)
            .map_err(|err| LarkError::Token(err.to_string()))?;
        if (200..300).contains(&api_resp.status_code) && !response.is_failure() {
            return Ok(());
        }
        Err(oauth_response_error("revoke OAuth token", &response))
    }

    async fn do_oauth_request(
        &self,
        body: &OAuthTokenRequestBody,
        encoding: OAuthTokenRequestEncoding,
        option: &RequestOption,
    ) -> Result<AccessTokenResp, LarkError> {
        let request_url = format!(
            "{}{}",
            self.config.base_url.trim_end_matches('/'),
            USER_OAUTH_TOKEN_URL_PATH
        );
        let mut request_option = option.clone();

        let mut body = body.clone();
        body.client_id = Some(self.config.app_id.clone());
        body.client_secret = Some(self.require_app_secret()?.to_owned());

        let headers = request_option
            .headers
            .get_or_insert_with(http::HeaderMap::new);
        headers.insert(
            http::header::CONTENT_TYPE,
            match encoding {
                OAuthTokenRequestEncoding::Json => {
                    http::HeaderValue::from_static("application/json; charset=utf-8")
                }
                OAuthTokenRequestEncoding::UrlEncoded => {
                    http::HeaderValue::from_static("application/x-www-form-urlencoded")
                }
            },
        );

        let mut api_req = ApiReq::new(http::Method::POST, &request_url);
        api_req.supported_access_token_types = vec![AccessTokenType::None];
        api_req.body = Some(match encoding {
            OAuthTokenRequestEncoding::Json => ReqBody::json(&body)?,
            OAuthTokenRequestEncoding::UrlEncoded => ReqBody::UrlEncoded(body.urlencoded_fields()),
        });

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

    fn require_app_secret(&self) -> Result<&str, LarkError> {
        if self.config.app_secret.is_empty() {
            return Err(LarkError::IllegalParam(
                "AppSecret must be configured for user OAuth access-token APIs".to_owned(),
            ));
        }
        Ok(&self.config.app_secret)
    }
}

#[derive(Debug, Clone, Copy)]
enum OAuthTokenRequestEncoding {
    Json,
    UrlEncoded,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
struct OAuthTokenRequestBody {
    grant_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    device_code: Option<String>,
}

impl OAuthTokenRequestBody {
    fn urlencoded_fields(&self) -> Vec<(String, String)> {
        [
            ("grant_type", Some(self.grant_type.as_str())),
            ("client_id", self.client_id.as_deref()),
            ("client_secret", self.client_secret.as_deref()),
            ("code", self.code.as_deref()),
            ("redirect_uri", self.redirect_uri.as_deref()),
            ("code_verifier", self.code_verifier.as_deref()),
            ("scope", self.scope.as_deref()),
            ("refresh_token", self.refresh_token.as_deref()),
            ("device_code", self.device_code.as_deref()),
        ]
        .into_iter()
        .filter_map(|(key, value)| value.map(|value| (key.to_owned(), value.to_owned())))
        .collect()
    }
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

#[derive(Debug, Default, Deserialize)]
struct OAuthErrorResponse {
    #[serde(default)]
    code: i64,
    #[serde(default)]
    msg: String,
    #[serde(default)]
    error: String,
    #[serde(default)]
    error_description: String,
}

impl OAuthErrorResponse {
    fn is_failure(&self) -> bool {
        self.code != 0 || !self.error.is_empty()
    }

    fn message(&self) -> &str {
        if !self.error_description.is_empty() {
            &self.error_description
        } else if !self.error.is_empty() {
            &self.error
        } else {
            &self.msg
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct DeviceAuthorizationResponse {
    #[serde(flatten)]
    error: OAuthErrorResponse,
    #[serde(default)]
    device_code: String,
    #[serde(default)]
    user_code: String,
    #[serde(default)]
    verification_uri: String,
    #[serde(default)]
    verification_uri_complete: String,
    #[serde(default)]
    expires_in: u64,
    #[serde(default)]
    interval: u64,
}

fn oauth_accounts_base_url(base_url: &str) -> &str {
    match base_url.trim_end_matches('/') {
        FEISHU_BASE_URL => FEISHU_OAUTH_BASE_URL,
        LARK_BASE_URL => LARK_OAUTH_BASE_URL,
        custom_base_url => custom_base_url,
    }
}

fn oauth_response_error(operation: &str, response: &OAuthErrorResponse) -> LarkError {
    LarkError::Api(Box::new(crate::resp::CodeError {
        code: response.code,
        msg: format!("{operation}: {}", response.message()),
        ..Default::default()
    }))
}

pub struct UserInfoResource<'a> {
    config: &'a Config,
}

impl<'a> UserInfoResource<'a> {
    /// Get current user info (requires user_access_token).
    pub async fn get(&self, option: &RequestOption) -> Result<GetUserInfoResp, LarkError> {
        let query = GetUserInfoQuery::new();
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        _query: &GetUserInfoQuery,
        option: &RequestOption,
    ) -> Result<GetUserInfoResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/authen/v1/user_info",
            vec![AccessTokenType::User],
            option,
        )
        .send_response::<UserInfo, GetUserInfoResp>()
        .await
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

#[cfg(test)]
mod device_oauth_tests {
    use super::*;

    #[test]
    fn resolves_accounts_endpoint_for_feishu_lark_and_custom_bases() {
        assert_eq!(
            oauth_accounts_base_url(FEISHU_BASE_URL),
            FEISHU_OAUTH_BASE_URL
        );
        assert_eq!(oauth_accounts_base_url(LARK_BASE_URL), LARK_OAUTH_BASE_URL);
        assert_eq!(
            oauth_accounts_base_url("https://oauth.example.test/"),
            "https://oauth.example.test"
        );
    }
}
