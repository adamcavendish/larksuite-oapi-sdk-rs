use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Response data types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppFeedCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_cards: Option<Vec<FailedAppFeedCard>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FailedAppFeedCard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BizEntityTagRelationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_info_with_bind_versions: Option<Vec<TagInfoWithBindVersion>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagInfoWithBindVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_info: Option<TagInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTagData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_tag_fail_reason: Option<TagFailureReason>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchTagData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_info: Option<TagInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_tag_fail_reason: Option<TagFailureReason>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<TagI18nName>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagI18nName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagFailureReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duplicate_id: Option<String>,
}

// ── Response types ─────────────────────────────────────────────────────────────

impl_resp_v2!(CreateAppFeedCardV2Resp, AppFeedCardData);
impl_resp_v2!(DeleteAppFeedCardBatchV2Resp, ());
impl_resp_v2!(UpdateAppFeedCardBatchV2Resp, ());
impl_resp_v2!(CreateBizEntityTagRelationV2Resp, ());
impl_resp_v2!(GetBizEntityTagRelationV2Resp, BizEntityTagRelationData);
impl_resp_v2!(UpdateBizEntityTagRelationV2Resp, ());
impl_resp_v2!(UpdateChatButtonV2Resp, ());
impl_resp_v2!(BotTimeSensitiveFeedCardV2Resp, ());
impl_resp_v2!(PatchFeedCardV2Resp, ());
impl_resp_v2!(CreateTagV2Resp, CreateTagData);
impl_resp_v2!(PatchTagV2Resp, PatchTagData);
impl_resp_v2!(BatchUpdateUrlPreviewV2Resp, ());

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateAppFeedCardV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreateAppFeedCardV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteAppFeedCardBatchV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> DeleteAppFeedCardBatchV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateAppFeedCardBatchV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateAppFeedCardBatchV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateBizEntityTagRelationV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreateBizEntityTagRelationV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct GetBizEntityTagRelationV2Query;

impl GetBizEntityTagRelationV2Query {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateBizEntityTagRelationV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateBizEntityTagRelationV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateChatButtonV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateChatButtonV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BotTimeSensitiveFeedCardV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> BotTimeSensitiveFeedCardV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchFeedCardV2Query<'a> {
    pub feed_card_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> PatchFeedCardV2Query<'a> {
    pub fn new(feed_card_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { feed_card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateTagV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreateTagV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchTagV2Query<'a> {
    pub tag_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> PatchTagV2Query<'a> {
    pub fn new(tag_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { tag_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchUpdateUrlPreviewV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> BatchUpdateUrlPreviewV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

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
        let query = CreateAppFeedCardV2Query::new(&body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateAppFeedCardV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateAppFeedCardV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/im/v2/app_feed_card",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<AppFeedCardData, CreateAppFeedCardV2Resp>()
        .await
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
        let query = DeleteAppFeedCardBatchV2Query::new(&body);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteAppFeedCardBatchV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteAppFeedCardBatchV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/im/v2/app_feed_card/batch",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), DeleteAppFeedCardBatchV2Resp>()
        .await
    }

    pub async fn update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateAppFeedCardBatchV2Resp, LarkError> {
        let query = UpdateAppFeedCardBatchV2Query::new(&body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateAppFeedCardBatchV2Query<'_>,
        option: &RequestOption,
    ) -> Result<UpdateAppFeedCardBatchV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PUT,
            "/open-apis/im/v2/app_feed_card/batch",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateAppFeedCardBatchV2Resp>()
        .await
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
        let query = CreateBizEntityTagRelationV2Query::new(&body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateBizEntityTagRelationV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateBizEntityTagRelationV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/im/v2/biz_entity_tag_relation",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), CreateBizEntityTagRelationV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        option: &RequestOption,
    ) -> Result<GetBizEntityTagRelationV2Resp, LarkError> {
        self.get_by_query(&GetBizEntityTagRelationV2Query::new(), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        _query: &GetBizEntityTagRelationV2Query,
        option: &RequestOption,
    ) -> Result<GetBizEntityTagRelationV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/im/v2/biz_entity_tag_relation",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<BizEntityTagRelationData, GetBizEntityTagRelationV2Resp>()
        .await
    }

    pub async fn update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateBizEntityTagRelationV2Resp, LarkError> {
        let query = UpdateBizEntityTagRelationV2Query::new(&body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateBizEntityTagRelationV2Query<'_>,
        option: &RequestOption,
    ) -> Result<UpdateBizEntityTagRelationV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PUT,
            "/open-apis/im/v2/biz_entity_tag_relation",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateBizEntityTagRelationV2Resp>()
        .await
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
        let query = UpdateChatButtonV2Query::new(&body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateChatButtonV2Query<'_>,
        option: &RequestOption,
    ) -> Result<UpdateChatButtonV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PUT,
            "/open-apis/im/v2/chat_button",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateChatButtonV2Resp>()
        .await
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
        let query = BotTimeSensitiveFeedCardV2Query::new(&body);
        self.bot_time_sensitive_by_query(&query, option).await
    }

    pub async fn bot_time_sensitive_by_query(
        &self,
        query: &BotTimeSensitiveFeedCardV2Query<'_>,
        option: &RequestOption,
    ) -> Result<BotTimeSensitiveFeedCardV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/im/v2/feed_cards/bot_time_sentive",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), BotTimeSensitiveFeedCardV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        feed_card_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchFeedCardV2Resp, LarkError> {
        let query = PatchFeedCardV2Query::new(feed_card_id, &body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchFeedCardV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchFeedCardV2Resp, LarkError> {
        let path = format!("/open-apis/im/v2/feed_cards/{}", query.feed_card_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), PatchFeedCardV2Resp>()
        .await
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
        let query = CreateTagV2Query::new(&body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateTagV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateTagV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/im/v2/tags",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CreateTagData, CreateTagV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        tag_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchTagV2Resp, LarkError> {
        let query = PatchTagV2Query::new(tag_id, &body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchTagV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchTagV2Resp, LarkError> {
        let path = format!("/open-apis/im/v2/tags/{}", query.tag_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<PatchTagData, PatchTagV2Resp>()
        .await
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
        let query = BatchUpdateUrlPreviewV2Query::new(&body);
        self.batch_update_by_query(&query, option).await
    }

    pub async fn batch_update_by_query(
        &self,
        query: &BatchUpdateUrlPreviewV2Query<'_>,
        option: &RequestOption,
    ) -> Result<BatchUpdateUrlPreviewV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/im/v2/url_previews/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), BatchUpdateUrlPreviewV2Resp>()
        .await
    }
}
