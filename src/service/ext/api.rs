use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

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
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/authen/v1/access_token",
            vec![AccessTokenType::App],
            option,
        )
        .json_body(&body)?
        .send::<AuthenAccessTokenRespBody>()
        .await?;
        Ok(AuthenAccessTokenResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn refresh_access_token(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RefreshAuthenAccessTokenResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/authen/v1/refresh_access_token",
            vec![AccessTokenType::App],
            option,
        )
        .json_body(&body)?
        .send::<RefreshAuthenAccessTokenRespBody>()
        .await?;
        Ok(RefreshAuthenAccessTokenResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn user_info(&self, option: &RequestOption) -> Result<AuthenUserInfoResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/authen/v1/user_info",
            vec![AccessTokenType::User],
            option,
        )
        .send::<AuthenUserInfoRespBody>()
        .await?;
        Ok(AuthenUserInfoResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateFileResp {
            api_resp,
            code_error,
            data,
        })
    }
}
