use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::service::common::parse_v2;
use crate::transport;

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
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchNodeV1Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/wiki/v1/nodes/search");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<NodeV1ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchNodeV1Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
