use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{DownloadResp, EmptyResp, PageQuery, RestRequest};

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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateEntityQuery<'a> {
    pub body: &'a CreateEntityReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateEntityQuery<'a> {
    pub fn new(body: &'a CreateEntityReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UpdateEntityQuery<'a> {
    pub entity_id: &'a str,
    pub body: &'a CreateEntityReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateEntityQuery<'a> {
    pub fn new(entity_id: &'a str, body: &'a CreateEntityReqBody) -> Self {
        Self {
            entity_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetEntityQuery<'a> {
    pub entity_id: &'a str,
    pub provider: Option<&'a str>,
    pub outer_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetEntityQuery<'a> {
    pub fn new(entity_id: &'a str) -> Self {
        Self {
            entity_id,
            provider: None,
            outer_id: None,
            user_id_type: None,
        }
    }

    pub fn provider(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.provider = value.into();
        self
    }

    pub fn outer_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.outer_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListEntityQuery<'a> {
    pub page: PageQuery<'a>,
    pub provider: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListEntityQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }

    pub fn provider(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.provider = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SearchEntityQuery<'a> {
    pub body: &'a SearchEntityReqBody,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SearchEntityQuery<'a> {
    pub fn new(body: &'a SearchEntityReqBody) -> Self {
        Self {
            body,
            page: PageQuery::default(),
            user_id_type: None,
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct HighlightEntityQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> HighlightEntityQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ExtractEntityQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> ExtractEntityQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct MatchEntityQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> MatchEntityQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListClassificationQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListClassificationQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

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
    ) -> Result<CreateEntityResp, LarkError> {
        let query = CreateEntityQuery::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateEntityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/baike/v1/entities",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<EntityData, CreateEntityResp>()
        .await
    }

    pub async fn update(
        &self,
        entity_id: &str,
        body: &CreateEntityReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateEntityResp, LarkError> {
        let query = UpdateEntityQuery::new(entity_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateEntityResp, LarkError> {
        let path = format!("/open-apis/baike/v1/entities/{}", query.entity_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<EntityData, UpdateEntityResp>()
        .await
    }

    pub async fn get(
        &self,
        entity_id: &str,
        provider: Option<&str>,
        outer_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetEntityResp, LarkError> {
        let query = GetEntityQuery::new(entity_id)
            .provider(provider)
            .outer_id(outer_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetEntityResp, LarkError> {
        let path = format!("/open-apis/baike/v1/entities/{}", query.entity_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("provider", query.provider)
        .query("outer_id", query.outer_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<EntityData, GetEntityResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        provider: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEntityResp, LarkError> {
        let query = ListEntityQuery::new()
            .page(PageQuery::from_parts(page_size, page_token))
            .provider(provider)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEntityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/baike/v1/entities",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("provider", query.provider)
        .query("user_id_type", query.user_id_type)
        .send_response::<EntityListData, ListEntityResp>()
        .await
    }

    pub async fn search(
        &self,
        body: &SearchEntityReqBody,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchEntityResp, LarkError> {
        let query = SearchEntityQuery::new(body)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchEntityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/baike/v1/entities/search",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<SearchEntityData, SearchEntityResp>()
        .await
    }

    pub async fn highlight(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = HighlightEntityQuery::new(&body);
        self.highlight_by_query(&query, option).await
    }

    pub async fn highlight_by_query(
        &self,
        query: &HighlightEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/baike/v1/entities/highlight",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn extract(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExtractEntityResp, LarkError> {
        let query = ExtractEntityQuery::new(&body);
        self.extract_by_query(&query, option).await
    }

    pub async fn extract_by_query(
        &self,
        query: &ExtractEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<ExtractEntityResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/baike/v1/entities/extract",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<MatchEntityResp, LarkError> {
        let query = MatchEntityQuery::new(&body);
        self.match_by_query(&query, option).await
    }

    pub async fn match_by_query(
        &self,
        query: &MatchEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<MatchEntityResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/baike/v1/entities/match",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<ListClassificationResp, LarkError> {
        let query =
            ListClassificationQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListClassificationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListClassificationResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/baike/v1/classifications",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateDraftQuery<'a> {
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateDraftQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateDraftQuery<'a> {
    pub draft_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateDraftQuery<'a> {
    pub fn new(draft_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            draft_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> DraftResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateDraftResp, LarkError> {
        let query = CreateDraftQuery::new(&body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDraftQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDraftResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/baike/v1/drafts",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
    ) -> Result<UpdateDraftResp, LarkError> {
        let query = UpdateDraftQuery::new(draft_id, &body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateDraftQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateDraftResp, LarkError> {
        let path = format!("/open-apis/baike/v1/drafts/{}", query.draft_id);
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UploadFileQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> UploadFileQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

impl<'a> FileResource<'a> {
    pub async fn download(
        &self,
        file_token: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/baike/v1/files/{file_token}/download");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .download()
        .await
    }

    pub async fn upload(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadFileResp, LarkError> {
        let query = UploadFileQuery::new(&body);
        self.upload_by_query(&query, option).await
    }

    pub async fn upload_by_query(
        &self,
        query: &UploadFileQuery<'_>,
        option: &RequestOption,
    ) -> Result<UploadFileResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/baike/v1/files/upload",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
