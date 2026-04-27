use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenapiLog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_body: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenapiLogListData {
    #[serde(default)]
    pub items: Vec<OpenapiLog>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListOpenapiLogResp, OpenapiLogListData);
impl_resp!(ListDataOpenapiLogResp, OpenapiLogListData);

// ── Resources ──

pub struct OpenapiLogResource<'a> {
    config: &'a Config,
}

impl<'a> OpenapiLogResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        api_path: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        operator_type: Option<i32>,
        operator_value: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListOpenapiLogResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/security_and_compliance/v1/openapi_logs",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = api_path {
            api_req.query_params.set("api_path", v);
        }
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = operator_type {
            api_req.query_params.set("operator_type", v.to_string());
        }
        if let Some(v) = operator_value {
            api_req.query_params.set("operator_value", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<OpenapiLogListData>(self.config, &api_req, option).await?;
        Ok(ListOpenapiLogResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list_data(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ListDataOpenapiLogResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/security_and_compliance/v1/openapi_logs/list_data",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<OpenapiLogListData>(self.config, &api_req, option).await?;
        Ok(ListDataOpenapiLogResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub openapi_log: OpenapiLogResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            openapi_log: OpenapiLogResource { config },
        }
    }
}
