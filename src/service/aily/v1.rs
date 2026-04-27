use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::EmptyResp;
use crate::transport;

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
    pub channel_context: Option<serde_json::Value>,
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
    pub content: Option<serde_json::Value>,
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
    pub error: Option<serde_json::Value>,
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
    pub description: Option<serde_json::Value>,
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
    pub name: Option<serde_json::Value>,
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
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSessionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_context: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
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
    pub content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRunReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataAssetReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_knowledge_setting: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AskKnowledgeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_asset_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_asset_tag_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StartSkillReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_variable: Option<serde_json::Value>,
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
    pub file_info: Option<serde_json::Value>,
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
    pub message: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faq_result: Option<serde_json::Value>,
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
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/aily/v1/sessions");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SessionData>(self.config, &api_req, option).await?;
        Ok(CreateSessionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        aily_session_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        aily_session_id: &str,
        option: &RequestOption,
    ) -> Result<GetSessionResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<SessionData>(self.config, &api_req, option).await?;
        Ok(GetSessionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        aily_session_id: &str,
        body: &UpdateSessionReqBody,
        option: &RequestOption,
    ) -> Result<UpdateSessionResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SessionData>(self.config, &api_req, option).await?;
        Ok(UpdateSessionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MessageData>(self.config, &api_req, option).await?;
        Ok(CreateMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        aily_session_id: &str,
        aily_message_id: &str,
        option: &RequestOption,
    ) -> Result<GetMessageResp, LarkError> {
        let path =
            format!("/open-apis/aily/v1/sessions/{aily_session_id}/messages/{aily_message_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<MessageData>(self.config, &api_req, option).await?;
        Ok(GetMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        aily_session_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMessageResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/messages");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<MessageListData>(self.config, &api_req, option).await?;
        Ok(ListMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<RunData>(self.config, &api_req, option).await?;
        Ok(CancelRunResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create(
        &self,
        aily_session_id: &str,
        body: &CreateRunReqBody,
        option: &RequestOption,
    ) -> Result<CreateRunResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/runs");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RunData>(self.config, &api_req, option).await?;
        Ok(CreateRunResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        aily_session_id: &str,
        run_id: &str,
        option: &RequestOption,
    ) -> Result<GetRunResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/runs/{run_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<RunData>(self.config, &api_req, option).await?;
        Ok(GetRunResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        aily_session_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRunResp, LarkError> {
        let path = format!("/open-apis/aily/v1/sessions/{aily_session_id}/runs");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RunListData>(self.config, &api_req, option).await?;
        Ok(ListRunResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        if let Some(v) = tenant_type {
            api_req.query_params.set("tenant_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DataAssetData>(self.config, &api_req, option).await?;
        Ok(CreateDataAssetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_id: &str,
        data_asset_id: &str,
        tenant_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeleteDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets/{data_asset_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = tenant_type {
            api_req.query_params.set("tenant_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DataAssetData>(self.config, &api_req, option).await?;
        Ok(DeleteDataAssetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        app_id: &str,
        data_asset_id: &str,
        tenant_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets/{data_asset_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = tenant_type {
            api_req.query_params.set("tenant_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DataAssetData>(self.config, &api_req, option).await?;
        Ok(GetDataAssetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = tenant_type {
            api_req.query_params.set("tenant_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DataAssetListData>(self.config, &api_req, option).await?;
        Ok(ListDataAssetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn upload_file(
        &self,
        app_id: &str,
        tenant_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadFileDataAssetResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/data_assets/upload_file");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        if let Some(v) = tenant_type {
            api_req.query_params.set("tenant_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<UploadFileData>(self.config, &api_req, option).await?;
        Ok(UploadFileDataAssetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = tenant_type {
            api_req.query_params.set("tenant_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DataAssetTagListData>(self.config, &api_req, option).await?;
        Ok(ListDataAssetTagResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<AskKnowledgeData>(self.config, &api_req, option).await?;
        Ok(AskKnowledgeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<SkillData>(self.config, &api_req, option).await?;
        Ok(GetSkillResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        app_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSkillResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/skills");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<SkillListData>(self.config, &api_req, option).await?;
        Ok(ListSkillResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn start(
        &self,
        app_id: &str,
        skill_id: &str,
        body: &StartSkillReqBody,
        option: &RequestOption,
    ) -> Result<StartSkillResp, LarkError> {
        let path = format!("/open-apis/aily/v1/apps/{app_id}/skills/{skill_id}/start");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<StartSkillData>(self.config, &api_req, option).await?;
        Ok(StartSkillResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
