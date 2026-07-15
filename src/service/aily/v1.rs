use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AilySession {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_context: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AilyMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AilyRun {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RunError>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAsset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Skill {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AilyMention {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aily_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AilyKnowledgeMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AilyKnowledgeAskProcessData {
    #[serde(default)]
    pub chart_dsls: Vec<String>,
    #[serde(default)]
    pub chunks: Vec<String>,
    #[serde(default)]
    pub sql_data: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AilyKnowledgeFaq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Channel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillGlobalVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetKnowledgeChunkSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separate_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlap: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetImportKnowledgeFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetImportKnowledgeLarkDoc {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_sub_docs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetImportKnowledgeWikiSubDoc {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetImportKnowledgeWiki {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(default)]
    pub sub_docs: Vec<DataAssetImportKnowledgeWikiSubDoc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetImportKnowledgeHelpdesk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetImportKnowledgeSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk_setting: Option<DataAssetKnowledgeChunkSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<DataAssetImportKnowledgeFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lark_doc: Option<DataAssetImportKnowledgeLarkDoc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lark_wiki_space: Option<DataAssetImportKnowledgeWiki>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lark_helpdesk: Option<DataAssetImportKnowledgeHelpdesk>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSessionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSessionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<AilyMention>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRunReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataAssetReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_knowledge_setting: Option<DataAssetImportKnowledgeSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AskKnowledgeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<AilyKnowledgeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_asset_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_asset_tag_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StartSkillReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_variable: Option<SkillGlobalVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<AilySession>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<AilyMessage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageListData {
    #[serde(default)]
    pub messages: Vec<AilyMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<AilyRun>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunListData {
    #[serde(default)]
    pub runs: Vec<AilyRun>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_asset: Option<DataAsset>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetListData {
    #[serde(default)]
    pub items: Vec<DataAsset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadFileData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_info: Option<DataAssetFile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataAssetTagListData {
    #[serde(default)]
    pub items: Vec<DataAssetTag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AskKnowledgeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<AilyKnowledgeMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_data: Option<AilyKnowledgeAskProcessData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faq_result: Option<AilyKnowledgeFaq>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_answer: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<Skill>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillListData {
    #[serde(default)]
    pub skills: Vec<Skill>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartSkillData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl_resp!(CreateSessionResp, SessionData);
impl_resp!(GetSessionResp, SessionData);
impl_resp!(UpdateSessionResp, SessionData);
impl_resp!(CreateMessageResp, MessageData);
impl_resp!(GetMessageResp, MessageData);
impl_resp!(ListMessageResp, MessageListData);
impl_resp!(CreateRunResp, RunData);
impl_resp!(GetRunResp, RunData);
impl_resp!(CancelRunResp, RunData);
impl_resp!(ListRunResp, RunListData);
impl_resp!(CreateDataAssetResp, DataAssetData);
impl_resp!(DeleteDataAssetResp, DataAssetData);
impl_resp!(GetDataAssetResp, DataAssetData);
impl_resp!(ListDataAssetResp, DataAssetListData);
impl_resp!(UploadFileDataAssetResp, UploadFileData);
impl_resp!(ListDataAssetTagResp, DataAssetTagListData);
impl_resp!(AskKnowledgeResp, AskKnowledgeData);
impl_resp!(GetSkillResp, SkillData);
impl_resp!(ListSkillResp, SkillListData);
impl_resp!(StartSkillResp, StartSkillData);

// ── Resources ──

pub struct SessionResource<'a> {
    config: &'a Config,
}

impl<'a> SessionResource<'a> {
    pub async fn create(
        &self,
        body: &CreateSessionReqBody,
        option: &RequestOption,
    ) -> Result<CreateSessionResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/aily/v1/sessions",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<SessionData, CreateSessionResp>()
        .await
    }

    pub async fn delete(
        &self,
        aily_session_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn get(
        &self,
        aily_session_id: &str,
        option: &RequestOption,
    ) -> Result<GetSessionResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_response::<SessionData, GetSessionResp>()
        .await
    }

    pub async fn update(
        &self,
        aily_session_id: &str,
        body: &UpdateSessionReqBody,
        option: &RequestOption,
    ) -> Result<UpdateSessionResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<SessionData, UpdateSessionResp>()
        .await
    }
}

pub struct MessageResource<'a> {
    config: &'a Config,
}

impl<'a> MessageResource<'a> {
    pub async fn create(
        &self,
        aily_session_id: &str,
        body: &CreateMessageReqBody,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/messages");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<MessageData, CreateMessageResp>()
        .await
    }

    pub async fn get(
        &self,
        aily_session_id: &str,
        aily_message_id: &str,
        option: &RequestOption,
    ) -> Result<GetMessageResp, LarkError> {
        let path =
            format!("/open-apis/aily/v1/sessions/{aily_session_id}/messages/{aily_message_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_response::<MessageData, GetMessageResp>()
        .await
    }

    pub async fn list(
        &self,
        aily_session_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMessageResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/messages");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_response::<MessageListData, ListMessageResp>()
        .await
    }
}

pub struct RunResource<'a> {
    config: &'a Config,
}

impl<'a> RunResource<'a> {
    pub async fn cancel(
        &self,
        aily_session_id: &str,
        run_id: &str,
        option: &RequestOption,
    ) -> Result<CancelRunResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/runs/{run_id}/cancel");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_response::<RunData, CancelRunResp>()
        .await
    }

    pub async fn create(
        &self,
        aily_session_id: &str,
        body: &CreateRunReqBody,
        option: &RequestOption,
    ) -> Result<CreateRunResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/runs");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<RunData, CreateRunResp>()
        .await
    }

    pub async fn get(
        &self,
        aily_session_id: &str,
        run_id: &str,
        option: &RequestOption,
    ) -> Result<GetRunResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/runs/{run_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_response::<RunData, GetRunResp>()
        .await
    }

    pub async fn list(
        &self,
        aily_session_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRunResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/runs");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_response::<RunListData, ListRunResp>()
        .await
    }
}

pub struct DataAssetResource<'a> {
    config: &'a Config,
}

impl<'a> DataAssetResource<'a> {
    pub async fn create(
        &self,
        app_id: &str,
        body: &CreateDataAssetReqBody,
        tenant_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("tenant_type", tenant_type)
        .json_body(body)?
        .send_response::<DataAssetData, CreateDataAssetResp>()
        .await
    }

    pub async fn delete(
        &self,
        app_id: &str,
        data_asset_id: &str,
        tenant_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeleteDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets/{data_asset_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("tenant_type", tenant_type)
        .send_response::<DataAssetData, DeleteDataAssetResp>()
        .await
    }

    pub async fn get(
        &self,
        app_id: &str,
        data_asset_id: &str,
        tenant_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets/{data_asset_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("tenant_type", tenant_type)
        .send_response::<DataAssetData, GetDataAssetResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        tenant_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .query("tenant_type", tenant_type)
        .send_response::<DataAssetListData, ListDataAssetResp>()
        .await
    }

    pub async fn upload_file(
        &self,
        app_id: &str,
        tenant_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadFileDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets/upload_file");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("tenant_type", tenant_type)
        .json_body(body)?
        .send_response::<UploadFileData, UploadFileDataAssetResp>()
        .await
    }
}

pub struct DataAssetTagResource<'a> {
    config: &'a Config,
}

impl<'a> DataAssetTagResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        tenant_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDataAssetTagResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_asset_tags");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .query("tenant_type", tenant_type)
        .send_response::<DataAssetTagListData, ListDataAssetTagResp>()
        .await
    }
}

pub struct KnowledgeResource<'a> {
    config: &'a Config,
}

impl<'a> KnowledgeResource<'a> {
    pub async fn ask(
        &self,
        app_id: &str,
        body: &AskKnowledgeReqBody,
        option: &RequestOption,
    ) -> Result<AskKnowledgeResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/knowledges/ask");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<AskKnowledgeData, AskKnowledgeResp>()
        .await
    }
}

pub struct SkillResource<'a> {
    config: &'a Config,
}

impl<'a> SkillResource<'a> {
    pub async fn get(
        &self,
        app_id: &str,
        skill_id: &str,
        option: &RequestOption,
    ) -> Result<GetSkillResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/skills/{skill_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_response::<SkillData, GetSkillResp>()
        .await
    }

    pub async fn list(
        &self,
        app_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSkillResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/skills");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_response::<SkillListData, ListSkillResp>()
        .await
    }

    pub async fn start(
        &self,
        app_id: &str,
        skill_id: &str,
        body: &StartSkillReqBody,
        option: &RequestOption,
    ) -> Result<StartSkillResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/skills/{skill_id}/start");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<StartSkillData, StartSkillResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub session: SessionResource<'a>,
    pub message: MessageResource<'a>,
    pub run: RunResource<'a>,
    pub data_asset: DataAssetResource<'a>,
    pub data_asset_tag: DataAssetTagResource<'a>,
    pub knowledge: KnowledgeResource<'a>,
    pub skill: SkillResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            session: SessionResource { config },
            message: MessageResource { config },
            run: RunResource { config },
            data_asset: DataAssetResource { config },
            data_asset_tag: DataAssetTagResource { config },
            knowledge: KnowledgeResource { config },
            skill: SkillResource { config },
        }
    }
}
