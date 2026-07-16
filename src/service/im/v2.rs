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

// ── Mutation request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAppFeedCardText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAppFeedCardUrl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAppFeedCardButton {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_url: Option<OpenAppFeedCardUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<OpenAppFeedCardText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub button_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAppFeedCardButtons {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<OpenAppFeedCardButton>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenFeedStatusLabel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAppFeedLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppFeedNotify {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_notify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_sound_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_custom_sound: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAppFeedCard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_label: Option<OpenFeedStatusLabel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buttons: Option<OpenAppFeedCardButtons>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<OpenAppFeedLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_sensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify: Option<AppFeedNotify>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAppFeedCardV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_feed_card: Option<OpenAppFeedCard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOpenAppFeedCardDeleter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteAppFeedCardBatchV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed_cards: Option<Vec<UserOpenAppFeedCardDeleter>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOpenAppFeedCardUpdater {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_feed_card: Option<OpenAppFeedCard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAppFeedCardBatchV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed_cards: Option<Vec<UserOpenAppFeedCardUpdater>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BizEntityTagRelationV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_biz_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatButtonV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buttons: Option<OpenAppFeedCardButtons>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotTimeSensitiveFeedCardV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_sensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchFeedCardV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_sensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTagV2Input {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<TagI18nName>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTagV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_tag: Option<CreateTagV2Input>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchTagV2Input {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<TagI18nName>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchTagV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_tag: Option<PatchTagV2Input>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateUrlPreviewV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_tokens: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_ids: Option<Vec<String>>,
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
    pub body: &'a CreateAppFeedCardV2ReqBody,
}

impl<'a> CreateAppFeedCardV2Query<'a> {
    pub fn new(body: &'a CreateAppFeedCardV2ReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteAppFeedCardBatchV2Query<'a> {
    pub body: &'a DeleteAppFeedCardBatchV2ReqBody,
}

impl<'a> DeleteAppFeedCardBatchV2Query<'a> {
    pub fn new(body: &'a DeleteAppFeedCardBatchV2ReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateAppFeedCardBatchV2Query<'a> {
    pub body: &'a UpdateAppFeedCardBatchV2ReqBody,
}

impl<'a> UpdateAppFeedCardBatchV2Query<'a> {
    pub fn new(body: &'a UpdateAppFeedCardBatchV2ReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateBizEntityTagRelationV2Query<'a> {
    pub body: &'a BizEntityTagRelationV2ReqBody,
}

impl<'a> CreateBizEntityTagRelationV2Query<'a> {
    pub fn new(body: &'a BizEntityTagRelationV2ReqBody) -> Self {
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
    pub body: &'a BizEntityTagRelationV2ReqBody,
}

impl<'a> UpdateBizEntityTagRelationV2Query<'a> {
    pub fn new(body: &'a BizEntityTagRelationV2ReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateChatButtonV2Query<'a> {
    pub body: &'a UpdateChatButtonV2ReqBody,
}

impl<'a> UpdateChatButtonV2Query<'a> {
    pub fn new(body: &'a UpdateChatButtonV2ReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BotTimeSensitiveFeedCardV2Query<'a> {
    pub body: &'a BotTimeSensitiveFeedCardV2ReqBody,
}

impl<'a> BotTimeSensitiveFeedCardV2Query<'a> {
    pub fn new(body: &'a BotTimeSensitiveFeedCardV2ReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchFeedCardV2Query<'a> {
    pub feed_card_id: &'a str,
    pub body: &'a PatchFeedCardV2ReqBody,
}

impl<'a> PatchFeedCardV2Query<'a> {
    pub fn new(feed_card_id: &'a str, body: &'a PatchFeedCardV2ReqBody) -> Self {
        Self { feed_card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateTagV2Query<'a> {
    pub body: &'a CreateTagV2ReqBody,
}

impl<'a> CreateTagV2Query<'a> {
    pub fn new(body: &'a CreateTagV2ReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchTagV2Query<'a> {
    pub tag_id: &'a str,
    pub body: &'a PatchTagV2ReqBody,
}

impl<'a> PatchTagV2Query<'a> {
    pub fn new(tag_id: &'a str, body: &'a PatchTagV2ReqBody) -> Self {
        Self { tag_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchUpdateUrlPreviewV2Query<'a> {
    pub body: &'a BatchUpdateUrlPreviewV2ReqBody,
}

impl<'a> BatchUpdateUrlPreviewV2Query<'a> {
    pub fn new(body: &'a BatchUpdateUrlPreviewV2ReqBody) -> Self {
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
        body: CreateAppFeedCardV2ReqBody,
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
        body: DeleteAppFeedCardBatchV2ReqBody,
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
        body: UpdateAppFeedCardBatchV2ReqBody,
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
        body: BizEntityTagRelationV2ReqBody,
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
        body: BizEntityTagRelationV2ReqBody,
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
        body: UpdateChatButtonV2ReqBody,
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
        body: BotTimeSensitiveFeedCardV2ReqBody,
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
        body: PatchFeedCardV2ReqBody,
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
        body: CreateTagV2ReqBody,
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
        body: PatchTagV2ReqBody,
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
        body: BatchUpdateUrlPreviewV2ReqBody,
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
