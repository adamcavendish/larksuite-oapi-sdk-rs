use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

macro_rules! impl_resp {
    ($name:ident) => {
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<serde_json::Value>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.api_resp.status_code == 200
                    && self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}

impl_resp!(AuthenAccessTokenResp);
impl_resp!(RefreshAuthenAccessTokenResp);
impl_resp!(AuthenUserInfoResp);
impl_resp!(CreateFileResp);

fn parse(
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
    ) -> Result<AuthenAccessTokenResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/authen/v1/access_token");
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<RefreshAuthenAccessTokenResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/authen/v1/refresh_access_token",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::App];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw);
        Ok(RefreshAuthenAccessTokenResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn user_info(&self, option: &RequestOption) -> Result<AuthenUserInfoResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/authen/v1/user_info");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
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
    ) -> Result<CreateFileResp> {
        let path = format!("/open-apis/drive/explorer/v2/file/{folder_token}");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw);
        Ok(CreateFileResp {
            api_resp,
            code_error,
            data,
        })
    }
}
