use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{DownloadResp, EmptyResp, parse_v2};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaikeEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_keys: Option<Vec<Term>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Term>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<EntityStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<DisplayStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Term {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<DisplayStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplayStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_highlight: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_search: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelatedMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oncalls: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dislike_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OuterInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outer_id: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateEntityReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_keys: Option<Vec<Term>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Term>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchEntityReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<BaikeEntity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityListData {
    #[serde(default)]
    pub entities: Vec<BaikeEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchEntityData {
    #[serde(default)]
    pub entities: Vec<BaikeEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateEntityResp, EntityData);
impl_resp!(UpdateEntityResp, EntityData);
impl_resp!(GetEntityResp, EntityData);
impl_resp!(ListEntityResp, EntityListData);
impl_resp!(SearchEntityResp, SearchEntityData);

impl_resp_v2!(ExtractEntityResp, serde_json::Value);
impl_resp_v2!(MatchEntityResp, serde_json::Value);
impl_resp_v2!(ListClassificationResp, serde_json::Value);
impl_resp_v2!(CreateDraftResp, serde_json::Value);
impl_resp_v2!(UpdateDraftResp, serde_json::Value);
impl_resp_v2!(UploadFileResp, serde_json::Value);

// ── Resources ──

pub struct EntityResource<'a> {
    config: &'a Config,
}

impl<'a> EntityResource<'a> {
    pub async fn create(
        &self,
        body: &CreateEntityReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateEntityResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/baike/v1/entities");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<EntityData>(self.config, &api_req, option).await?;
        Ok(CreateEntityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        entity_id: &str,
        body: &CreateEntityReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateEntityResp> {
        let path = format!("/open-apis/baike/v1/entities/{entity_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<EntityData>(self.config, &api_req, option).await?;
        Ok(UpdateEntityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        entity_id: &str,
        provider: Option<&str>,
        outer_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetEntityResp> {
        let path = format!("/open-apis/baike/v1/entities/{entity_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = provider {
            api_req.query_params.set("provider", v);
        }
        if let Some(v) = outer_id {
            api_req.query_params.set("outer_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<EntityData>(self.config, &api_req, option).await?;
        Ok(GetEntityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        provider: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEntityResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/baike/v1/entities");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = provider {
            api_req.query_params.set("provider", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<EntityListData>(self.config, &api_req, option).await?;
        Ok(ListEntityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn search(
        &self,
        body: &SearchEntityReqBody,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchEntityResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/baike/v1/entities/search");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SearchEntityData>(self.config, &api_req, option).await?;
        Ok(SearchEntityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn highlight(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/baike/v1/entities/highlight");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn extract(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExtractEntityResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/baike/v1/entities/extract");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ExtractEntityResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn match_(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<MatchEntityResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/baike/v1/entities/match");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(MatchEntityResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ClassificationResource<'a> {
    config: &'a Config,
}

impl<'a> ClassificationResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListClassificationResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/baike/v1/classifications");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListClassificationResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct DraftResource<'a> {
    config: &'a Config,
}

impl<'a> DraftResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateDraftResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/baike/v1/drafts");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateDraftResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        draft_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateDraftResp> {
        let path = format!("/open-apis/baike/v1/drafts/{draft_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateDraftResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

impl<'a> FileResource<'a> {
    pub async fn download(&self, file_token: &str, option: &RequestOption) -> Result<DownloadResp> {
        let path = format!("/open-apis/baike/v1/files/{file_token}/download");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }

    pub async fn upload(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadFileResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/baike/v1/files/upload");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UploadFileResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub entity: EntityResource<'a>,
    pub classification: ClassificationResource<'a>,
    pub draft: DraftResource<'a>,
    pub file: FileResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            entity: EntityResource { config },
            classification: ClassificationResource { config },
            draft: DraftResource { config },
            file: FileResource { config },
        }
    }
}
