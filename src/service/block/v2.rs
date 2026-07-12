use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Entity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_meta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_summay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_preview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
}

impl_resp!(CreateEntityResp, EntityData);
impl_resp!(UpdateEntityResp, EntityData);
impl_resp!(CreateMessageResp, MessageData);

#[derive(Debug, Clone, Copy)]
pub struct CreateEntityQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreateEntityQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateEntityQuery<'a> {
    pub block_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateEntityQuery<'a> {
    pub fn new(block_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { block_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateMessageQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreateMessageQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

// ── Resources ──

pub struct EntityResource<'a> {
    config: &'a Config,
}

impl<'a> EntityResource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEntityResp, LarkError> {
        self.create_by_query(&CreateEntityQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateEntityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/block/v2/entities",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<EntityData, CreateEntityResp>()
        .await
    }

    pub async fn update(
        &self,
        block_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateEntityResp, LarkError> {
        self.update_by_query(&UpdateEntityQuery::new(block_id, body), option)
            .await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateEntityResp, LarkError> {
        let path = format!("/open-apis/block/v2/entities/{}", query.block_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<EntityData, UpdateEntityResp>()
        .await
    }
}

pub struct MessageResource<'a> {
    config: &'a Config,
}

impl<'a> MessageResource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        self.create_by_query(&CreateMessageQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateMessageQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/block/v2/message",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<MessageData, CreateMessageResp>()
        .await
    }
}

// ── Version struct ──

pub struct V2<'a> {
    pub entity: EntityResource<'a>,
    pub message: MessageResource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            entity: EntityResource { config },
            message: MessageResource { config },
        }
    }
}
