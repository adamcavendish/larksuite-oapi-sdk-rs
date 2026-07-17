use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

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

// -- Query parameter types --

#[derive(Debug, Clone, Copy, Default)]
pub struct ListOpenapiLogQuery<'a> {
    pub api_path: Option<&'a str>,
    pub start_time: Option<&'a str>,
    pub end_time: Option<&'a str>,
    pub operator_type: Option<i32>,
    pub operator_value: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListOpenapiLogQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_path(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.api_path = value.into();
        self
    }

    pub fn start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_time = value.into();
        self
    }

    pub fn end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_time = value.into();
        self
    }

    pub fn operator_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.operator_type = value.into();
        self
    }

    pub fn operator_value(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_value = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

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
        let query = ListOpenapiLogQuery::new()
            .api_path(api_path)
            .start_time(start_time)
            .end_time(end_time)
            .operator_type(operator_type)
            .operator_value(operator_value)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListOpenapiLogQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOpenapiLogResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/security_and_compliance/v1/openapi_logs",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("api_path", query.api_path)
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("operator_type", query.operator_type)
        .query("operator_value", query.operator_value)
        .page_query(query.page)
        .send_response::<OpenapiLogListData, ListOpenapiLogResp>()
        .await
    }

    pub async fn list_data(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<ListDataOpenapiLogResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/security_and_compliance/v1/openapi_logs/list_data",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<OpenapiLogListData, ListDataOpenapiLogResp>()
        .await
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
