use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::resp::{ApiResp, CodeError};
use crate::service::common::{FromV2Response, RestRequest};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeV1ListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

pub struct SearchNodeV1Resp {
    pub api_resp: ApiResp,
    pub code_error: Option<CodeError>,
    pub data: Option<NodeV1ListData>,
}

impl SearchNodeV1Resp {
    pub fn success(&self) -> bool {
        self.api_resp.status_code == 200 && self.code_error.as_ref().is_none_or(|e| e.code == 0)
    }
}

impl FromV2Response<NodeV1ListData> for SearchNodeV1Resp {
    fn from_v2_response(
        api_resp: ApiResp,
        code_error: Option<CodeError>,
        data: Option<NodeV1ListData>,
    ) -> Self {
        Self {
            api_resp,
            code_error,
            data,
        }
    }
}

pub struct SearchNodeV1Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> SearchNodeV1Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

pub struct V1<'a> {
    pub node: NodeV1Resource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            node: NodeV1Resource { config },
        }
    }
}

pub struct NodeV1Resource<'a> {
    config: &'a Config,
}

impl NodeV1Resource<'_> {
    pub async fn search(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<SearchNodeV1Resp, LarkError> {
        let body = serde_json::to_value(body)?;
        self.search_by_query(&SearchNodeV1Query::new(&body), option)
            .await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchNodeV1Query<'_>,
        option: &RequestOption,
    ) -> Result<SearchNodeV1Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/wiki/v1/nodes/search",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<NodeV1ListData, SearchNodeV1Resp>()
        .await
    }
}
