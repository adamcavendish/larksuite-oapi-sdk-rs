use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenAccessTokenRespBody {
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub en_name: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub avatar_thumb: String,
    #[serde(default)]
    pub avatar_middle: String,
    #[serde(default)]
    pub avatar_big: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub union_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub enterprise_email: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub mobile: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub refresh_expires_in: i64,
    #[serde(default)]
    pub refresh_token: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RefreshAuthenAccessTokenRespBody {
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub en_name: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub avatar_thumb: String,
    #[serde(default)]
    pub avatar_middle: String,
    #[serde(default)]
    pub avatar_big: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub union_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub mobile: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub refresh_expires_in: i64,
    #[serde(default)]
    pub refresh_token: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenUserInfoRespBody {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub en_name: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub avatar_thumb: String,
    #[serde(default)]
    pub avatar_middle: String,
    #[serde(default)]
    pub avatar_big: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub union_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub enterprise_email: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub mobile: String,
    #[serde(default)]
    pub tenant_key: String,
}

impl_resp!(AuthenAccessTokenResp, AuthenAccessTokenRespBody);
impl_resp!(
    RefreshAuthenAccessTokenResp,
    RefreshAuthenAccessTokenRespBody
);
impl_resp!(AuthenUserInfoResp, AuthenUserInfoRespBody);
impl_resp!(CreateFileResp);

fn parse<T: for<'de> Deserialize<'de>>(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<T>,
) -> (ApiResp, CodeError, Option<T>) {
    (api_resp, raw.code_error, raw.data)
}

fn parse_untyped(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<serde_json::Value>,
) -> (ApiResp, Option<CodeError>, Option<serde_json::Value>) {
    let code_error = if raw.code_error.code != 0 {
        Some(raw.code_error)
    } else {
        None
    };
    (api_resp, code_error, raw.data)
}

pub struct ExtService<'a> {
    pub authen: AuthenExtResource<'a>,
    pub drive_explorer: DriveExplorerExtResource<'a>,
}

impl<'a> ExtService<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            authen: AuthenExtResource { config },
            drive_explorer: DriveExplorerExtResource { config },
        }
    }
}

pub struct AuthenExtResource<'a> {
    config: &'a Config,
}

impl AuthenExtResource<'_> {
    pub async fn access_token(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<AuthenAccessTokenResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/authen/v1/access_token");
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<AuthenAccessTokenRespBody>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse(api_resp, raw);
        Ok(AuthenAccessTokenResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn refresh_access_token(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RefreshAuthenAccessTokenResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/authen/v1/refresh_access_token",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<RefreshAuthenAccessTokenRespBody>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        let (api_resp, code_error, data) = parse(api_resp, raw);
        Ok(RefreshAuthenAccessTokenResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn user_info(&self, option: &RequestOption) -> Result<AuthenUserInfoResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/authen/v1/user_info");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<AuthenUserInfoRespBody>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse(api_resp, raw);
        Ok(AuthenUserInfoResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct DriveExplorerExtResource<'a> {
    config: &'a Config,
}

impl DriveExplorerExtResource<'_> {
    pub async fn create_file(
        &self,
        folder_token: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateFileResp, LarkError> {
        let path = format!("/open-apis/drive/explorer/v2/file/{folder_token}");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_untyped(api_resp, raw);
        Ok(CreateFileResp {
            api_resp,
            code_error,
            data,
        })
    }
}
