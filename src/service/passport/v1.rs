use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionListData {
    #[serde(default)]
    pub session_list: Vec<SessionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(QuerySessionResp, SessionListData);

// ── Resources ──

pub struct SessionResource<'a> {
    config: &'a Config,
}

impl<'a> SessionResource<'a> {
    pub async fn query(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QuerySessionResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/passport/v1/sessions/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id", user_id);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<SessionListData>(self.config, &api_req, option).await?;
        Ok(QuerySessionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn logout(
        &self,
        idp_credential_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::DELETE, "/open-apis/passport/v1/sessions");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("idp_credential_id", idp_credential_id);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub session: SessionResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            session: SessionResource { config },
        }
    }
}
