use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Block {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_summary: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBlockReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_summary: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<Block>,
}

impl_resp!(CreateBlockResp, BlockData);
impl_resp!(GetBlockResp, BlockData);

// ── Resources ──

pub struct BlockResource<'a> {
    config: &'a Config,
}

impl<'a> BlockResource<'a> {
    pub async fn create(
        &self,
        body: &CreateBlockReqBody,
        option: &RequestOption,
    ) -> Result<CreateBlockResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/block/v2/blocks");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BlockData>(self.config, &api_req, option).await?;
        Ok(CreateBlockResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(&self, block_id: &str, option: &RequestOption) -> Result<GetBlockResp> {
        let path = format!("/open-apis/block/v2/blocks/{block_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<BlockData>(self.config, &api_req, option).await?;
        Ok(GetBlockResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub block: BlockResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            block: BlockResource { config },
        }
    }
}
