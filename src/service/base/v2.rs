use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(CreateAppRoleV2Resp, AppRoleData);
impl_resp_v2!(ListAppRoleV2Resp, AppRoleListData);
impl_resp_v2!(UpdateAppRoleV2Resp, AppRoleData);

fn parse<T: for<'de> serde::Deserialize<'de>>(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<T>,
) -> impl FnOnce() -> (ApiResp, Option<CodeError>, Option<T>) {
    move || {
        let code_error = if raw.code_error.code != 0 {
            Some(raw.code_error)
        } else {
            None
        };
        (api_resp, code_error, raw.data)
    }
}

pub struct V2<'a> {
    pub app_role: AppRoleV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app_role: AppRoleV2Resource { config },
        }
    }
}

pub struct AppRoleV2Resource<'a> {
    config: &'a Config,
}

impl AppRoleV2Resource<'_> {
    pub async fn create(
        &self,
        app_token: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateAppRoleV2Resp> {
        let path = format!("/open-apis/base/v2/apps/{app_token}/roles");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<AppRoleData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(CreateAppRoleV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppRoleV2Resp> {
        let path = format!("/open-apis/base/v2/apps/{app_token}/roles");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AppRoleListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(ListAppRoleV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        app_token: &str,
        role_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateAppRoleV2Resp> {
        let path = format!("/open-apis/base/v2/apps/{app_token}/roles/{role_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<AppRoleData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(UpdateAppRoleV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
