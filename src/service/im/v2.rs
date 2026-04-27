use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::parse_v2;
use crate::transport;

// ── Response data types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppFeedCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_feed_card: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BizEntityTagRelationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_entity_tag_relation: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
}

// ── Response types ─────────────────────────────────────────────────────────────

impl_resp_v2!(CreateAppFeedCardV2Resp, AppFeedCardData);
impl_resp_v2!(DeleteAppFeedCardBatchV2Resp, ());
impl_resp_v2!(UpdateAppFeedCardBatchV2Resp, ());
impl_resp_v2!(CreateBizEntityTagRelationV2Resp, BizEntityTagRelationData);
impl_resp_v2!(GetBizEntityTagRelationV2Resp, BizEntityTagRelationData);
impl_resp_v2!(UpdateBizEntityTagRelationV2Resp, BizEntityTagRelationData);
impl_resp_v2!(UpdateChatButtonV2Resp, ());
impl_resp_v2!(BotTimeSensitiveFeedCardV2Resp, ());
impl_resp_v2!(PatchFeedCardV2Resp, ());
impl_resp_v2!(CreateTagV2Resp, TagData);
impl_resp_v2!(PatchTagV2Resp, TagData);
impl_resp_v2!(BatchUpdateUrlPreviewV2Resp, ());

// ── V2 service entry ──────────────────────────────────────────────────────────

pub struct V2<'a> {
    pub app_feed_card: AppFeedCardV2Resource<'a>,
    pub app_feed_card_batch: AppFeedCardBatchV2Resource<'a>,
    pub biz_entity_tag_relation: BizEntityTagRelationV2Resource<'a>,
    pub chat_button: ChatButtonV2Resource<'a>,
    pub feed_card: FeedCardV2Resource<'a>,
    pub tag: TagV2Resource<'a>,
    pub url_preview: UrlPreviewV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app_feed_card: AppFeedCardV2Resource { config },
            app_feed_card_batch: AppFeedCardBatchV2Resource { config },
            biz_entity_tag_relation: BizEntityTagRelationV2Resource { config },
            chat_button: ChatButtonV2Resource { config },
            feed_card: FeedCardV2Resource { config },
            tag: TagV2Resource { config },
            url_preview: UrlPreviewV2Resource { config },
        }
    }
}

// ── AppFeedCard resource ──────────────────────────────────────────────────────

pub struct AppFeedCardV2Resource<'a> {
    config: &'a Config,
}

impl AppFeedCardV2Resource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateAppFeedCardV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v2/app_feed_card");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<AppFeedCardData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateAppFeedCardV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── AppFeedCardBatch resource ─────────────────────────────────────────────────

pub struct AppFeedCardBatchV2Resource<'a> {
    config: &'a Config,
}

impl AppFeedCardBatchV2Resource<'_> {
    pub async fn delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<DeleteAppFeedCardBatchV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::DELETE, "/open-apis/im/v2/app_feed_card/batch");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteAppFeedCardBatchV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateAppFeedCardBatchV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::PUT, "/open-apis/im/v2/app_feed_card/batch");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateAppFeedCardBatchV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── BizEntityTagRelation resource ─────────────────────────────────────────────

pub struct BizEntityTagRelationV2Resource<'a> {
    config: &'a Config,
}

impl BizEntityTagRelationV2Resource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateBizEntityTagRelationV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/im/v2/biz_entity_tag_relation",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<BizEntityTagRelationData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateBizEntityTagRelationV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        option: &RequestOption,
    ) -> Result<GetBizEntityTagRelationV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/im/v2/biz_entity_tag_relation",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<BizEntityTagRelationData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetBizEntityTagRelationV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateBizEntityTagRelationV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::PUT,
            "/open-apis/im/v2/biz_entity_tag_relation",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<BizEntityTagRelationData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateBizEntityTagRelationV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ChatButton resource ───────────────────────────────────────────────────────

pub struct ChatButtonV2Resource<'a> {
    config: &'a Config,
}

impl ChatButtonV2Resource<'_> {
    pub async fn update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateChatButtonV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::PUT, "/open-apis/im/v2/chat_button");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateChatButtonV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── FeedCard resource ─────────────────────────────────────────────────────────

pub struct FeedCardV2Resource<'a> {
    config: &'a Config,
}

impl FeedCardV2Resource<'_> {
    pub async fn bot_time_sensitive(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BotTimeSensitiveFeedCardV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::PATCH,
            "/open-apis/im/v2/feed_cards/bot_time_sentive",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BotTimeSensitiveFeedCardV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        feed_card_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchFeedCardV2Resp, LarkError> {
        let path = format!("/open-apis/im/v2/feed_cards/{feed_card_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchFeedCardV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Tag resource ──────────────────────────────────────────────────────────────

pub struct TagV2Resource<'a> {
    config: &'a Config,
}

impl TagV2Resource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateTagV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v2/tags");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<TagData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateTagV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        tag_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchTagV2Resp, LarkError> {
        let path = format!("/open-apis/im/v2/tags/{tag_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<TagData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchTagV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UrlPreview resource ───────────────────────────────────────────────────────

pub struct UrlPreviewV2Resource<'a> {
    config: &'a Config,
}

impl UrlPreviewV2Resource<'_> {
    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateUrlPreviewV2Resp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/im/v2/url_previews/batch_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchUpdateUrlPreviewV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
