use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
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
    pub i18n_summary: Option<crate::JsonValue>,
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
    pub i18n_summary: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<crate::JsonValue>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BlockData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<Block>,
}

impl_resp!(CreateBlockResp, BlockData);
impl_resp!(GetBlockResp, BlockData);

#[derive(Debug, Clone, Copy)]
pub struct CreateBlockQuery<'a> {
    pub body: &'a CreateBlockReqBody,
}

impl<'a> CreateBlockQuery<'a> {
    pub fn new(body: &'a CreateBlockReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetBlockQuery<'a> {
    pub block_id: &'a str,
}

impl<'a> GetBlockQuery<'a> {
    pub fn new(block_id: &'a str) -> Self {
        Self { block_id }
    }
}

// ── Resources ──

pub struct BlockResource<'a> {
    config: &'a Config,
}

impl<'a> BlockResource<'a> {
    pub async fn create(
        &self,
        body: &CreateBlockReqBody,
        option: &RequestOption,
    ) -> Result<CreateBlockResp, LarkError> {
        self.create_by_query(&CreateBlockQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateBlockResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/block/v2/blocks",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<BlockData, CreateBlockResp>()
        .await
    }

    pub async fn get(
        &self,
        block_id: &str,
        option: &RequestOption,
    ) -> Result<GetBlockResp, LarkError> {
        self.get_by_query(&GetBlockQuery::new(block_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBlockResp, LarkError> {
        let path = format!("/open-apis/block/v2/blocks/{}", query.block_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<BlockData, GetBlockResp>()
        .await
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
