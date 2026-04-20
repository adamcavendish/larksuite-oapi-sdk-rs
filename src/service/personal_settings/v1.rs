use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_status_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_setting: Option<serde_json::Value>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSystemStatusReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_status: Option<SystemStatus>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchOpenSystemStatusReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<serde_json::Value>>,
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
pub struct SystemStatusData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_status: Option<SystemStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusListData {
    #[serde(default)]
    pub items: Vec<SystemStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchOpenData {
    #[serde(default)]
    pub result_list: Vec<serde_json::Value>,
}

impl_resp!(CreateSystemStatusResp, SystemStatusData);
impl_resp!(ListSystemStatusResp, SystemStatusListData);
impl_resp!(BatchOpenSystemStatusResp, BatchOpenData);

// ── Resources ──

pub struct SystemStatusResource<'a> {
    config: &'a Config,
}

impl<'a> SystemStatusResource<'a> {
    pub async fn create(
        &self,
        body: &CreateSystemStatusReqBody,
        option: &RequestOption,
    ) -> Result<CreateSystemStatusResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/personal_settings/v1/system_statuses",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SystemStatusData>(self.config, &api_req, option).await?;
        Ok(CreateSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        system_status_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/personal_settings/v1/system_statuses/{system_status_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSystemStatusResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/personal_settings/v1/system_statuses",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<SystemStatusListData>(self.config, &api_req, option).await?;
        Ok(ListSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_open(
        &self,
        system_status_id: &str,
        body: &BatchOpenSystemStatusReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchOpenSystemStatusResp> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{system_status_id}/batch_open"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchOpenData>(self.config, &api_req, option).await?;
        Ok(BatchOpenSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_close(
        &self,
        system_status_id: &str,
        body: &BatchOpenSystemStatusReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchOpenSystemStatusResp> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{system_status_id}/batch_close"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchOpenData>(self.config, &api_req, option).await?;
        Ok(BatchOpenSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub system_status: SystemStatusResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            system_status: SystemStatusResource { config },
        }
    }
}
