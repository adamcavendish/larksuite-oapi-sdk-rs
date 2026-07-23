use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{FormDataField, RequestOption};
use crate::service::common::{DownloadResp, EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LingoEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_keys: Option<Vec<LingoTerm>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<LingoTerm>>,
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
    pub statistics: Option<Statistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<DisplayStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<Vec<Classification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<BaikeImage>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LingoTerm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<DisplayStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClassificationFilter {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLingoEntityReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_keys: Option<Vec<LingoTerm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<LingoTerm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<Vec<Classification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<BaikeImage>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchLingoEntityReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_filter: Option<ClassificationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<LingoEntity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityListData {
    #[serde(default)]
    pub entities: Vec<LingoEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateEntityResp, EntityData);
impl_resp!(UpdateEntityResp, EntityData);
impl_resp!(GetEntityResp, EntityData);
impl_resp!(ListEntityResp, EntityListData);
impl_resp!(SearchEntityResp, EntityListData);

impl_resp_v2!(HighlightEntityResp, HighlightEntityRespData);
impl_resp_v2!(MatchEntityResp, MatchEntityRespData);
impl_resp_v2!(ListClassificationResp, ListClassificationRespData);
impl_resp_v2!(CreateDraftResp, CreateDraftRespData);
impl_resp_v2!(UpdateDraftResp, UpdateDraftRespData);
impl_resp_v2!(UploadFileResp, UploadFileRespData);
impl_resp_v2!(ListRepoResp, ListRepoRespData);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateEntityQuery<'a> {
    pub body: &'a CreateLingoEntityReqBody,
    pub repo_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateEntityQuery<'a> {
    pub fn new(body: &'a CreateLingoEntityReqBody) -> Self {
        Self {
            body,
            repo_id: None,
            user_id_type: None,
        }
    }

    pub fn repo_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.repo_id = value.into();
        self
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
    pub body: &'a CreateLingoEntityReqBody,
    pub repo_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateEntityQuery<'a> {
    pub fn new(entity_id: &'a str, body: &'a CreateLingoEntityReqBody) -> Self {
        Self {
            entity_id,
            body,
            repo_id: None,
            user_id_type: None,
        }
    }

    pub fn repo_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.repo_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DeleteEntityQuery<'a> {
    pub entity_id: &'a str,
    pub provider: Option<&'a str>,
    pub outer_id: Option<&'a str>,
}

impl<'a> DeleteEntityQuery<'a> {
    pub fn new(entity_id: &'a str) -> Self {
        Self {
            entity_id,
            provider: None,
            outer_id: None,
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
    pub repo_id: Option<&'a str>,
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

    pub fn repo_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.repo_id = value.into();
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
    pub body: &'a SearchLingoEntityReqBody,
    pub page: PageQuery<'a>,
    pub repo_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SearchEntityQuery<'a> {
    pub fn new(body: &'a SearchLingoEntityReqBody) -> Self {
        Self {
            body,
            page: PageQuery::default(),
            repo_id: None,
            user_id_type: None,
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }

    pub fn repo_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.repo_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct HighlightEntityQuery<'a> {
    pub body: &'a crate::JsonValue,
}

impl<'a> HighlightEntityQuery<'a> {
    pub fn new(body: &'a crate::JsonValue) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct MatchEntityQuery<'a> {
    pub body: &'a crate::JsonValue,
}

impl<'a> MatchEntityQuery<'a> {
    pub fn new(body: &'a crate::JsonValue) -> Self {
        Self { body }
    }
}

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HighlightEntityRespData {
    #[serde(default)]
    pub phrases: Vec<Phrase>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MatchEntityRespData {
    #[serde(default)]
    pub results: Vec<MatchInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListClassificationRespData {
    #[serde(default)]
    pub items: Vec<Classification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateDraftRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateDraftRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UploadFileRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListRepoRespData {
    #[serde(default)]
    pub items: Vec<Repo>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Abbreviation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BaikeImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Classification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub father_id: Option<String>,
    #[serde(default)]
    pub i18n_names: Vec<I18nClsName>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DisplayStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_highlight: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_search: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Draft {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Entity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub main_keys: Vec<Term>,
    #[serde(default)]
    pub full_names: Vec<Term>,
    #[serde(default)]
    pub aliases: Vec<Term>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updater: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    #[serde(default)]
    pub i18n_descs: Vec<I18nEntryDesc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct I18nClsName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct I18nEntryDesc {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MatchInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OuterInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Phrase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub entity_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Referer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RelatedMeta {
    #[serde(default)]
    pub users: Vec<Referer>,
    #[serde(default)]
    pub chats: Vec<Referer>,
    #[serde(default)]
    pub docs: Vec<Referer>,
    #[serde(default)]
    pub oncalls: Vec<Referer>,
    #[serde(default)]
    pub links: Vec<Referer>,
    #[serde(default)]
    pub abbreviations: Vec<Abbreviation>,
    #[serde(default)]
    pub classifications: Vec<Classification>,
    #[serde(default)]
    pub images: Vec<BaikeImage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Span {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Statistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dislike_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Term {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<DisplayStatus>,
}
// ── Resources ──

pub struct EntityResource<'a> {
    config: &'a Config,
}

impl<'a> EntityResource<'a> {
    pub async fn create(
        &self,
        body: &CreateLingoEntityReqBody,
        repo_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateEntityResp, LarkError> {
        let query = CreateEntityQuery::new(body)
            .repo_id(repo_id)
            .user_id_type(user_id_type);
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
            "/open-apis/lingo/v1/entities",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("repo_id", query.repo_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<EntityData, CreateEntityResp>()
        .await
    }

    pub async fn update(
        &self,
        entity_id: &str,
        body: &CreateLingoEntityReqBody,
        repo_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateEntityResp, LarkError> {
        let query = UpdateEntityQuery::new(entity_id, body)
            .repo_id(repo_id)
            .user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateEntityResp, LarkError> {
        let path = format!("/open-apis/lingo/v1/entities/{}", query.entity_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("repo_id", query.repo_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<EntityData, UpdateEntityResp>()
        .await
    }

    pub async fn delete(
        &self,
        entity_id: &str,
        provider: Option<&str>,
        outer_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteEntityQuery::new(entity_id)
            .provider(provider)
            .outer_id(outer_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/lingo/v1/entities/{}", query.entity_id);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("provider", query.provider)
        .query("outer_id", query.outer_id)
        .send_empty()
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
        let path = format!("/open-apis/lingo/v1/entities/{}", query.entity_id);
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
        repo_id: Option<&str>,
        provider: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEntityResp, LarkError> {
        let query = ListEntityQuery::new()
            .page(PageQuery::from_parts(page_size, page_token))
            .repo_id(repo_id)
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
            "/open-apis/lingo/v1/entities",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("repo_id", query.repo_id)
        .query("provider", query.provider)
        .query("user_id_type", query.user_id_type)
        .send_response::<EntityListData, ListEntityResp>()
        .await
    }

    pub async fn search(
        &self,
        body: &SearchLingoEntityReqBody,
        repo_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchEntityResp, LarkError> {
        let query = SearchEntityQuery::new(body)
            .repo_id(repo_id)
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
            "/open-apis/lingo/v1/entities/search",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("repo_id", query.repo_id)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<EntityListData, SearchEntityResp>()
        .await
    }

    pub async fn highlight(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<HighlightEntityResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = HighlightEntityQuery::new(&body);
        self.highlight_by_query(&query, option).await
    }

    pub async fn highlight_by_query(
        &self,
        query: &HighlightEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<HighlightEntityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/lingo/v1/entities/highlight",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<HighlightEntityRespData, HighlightEntityResp>()
        .await
    }

    pub async fn match_(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<MatchEntityResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = MatchEntityQuery::new(&body);
        self.match_by_query(&query, option).await
    }

    pub async fn match_by_query(
        &self,
        query: &MatchEntityQuery<'_>,
        option: &RequestOption,
    ) -> Result<MatchEntityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/lingo/v1/entities/match",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<MatchEntityRespData, MatchEntityResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/lingo/v1/classifications",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .send_v2_response::<ListClassificationRespData, ListClassificationResp>()
        .await
    }
}

pub struct DraftResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateDraftQuery<'a> {
    pub body: &'a crate::JsonValue,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateDraftQuery<'a> {
    pub fn new(body: &'a crate::JsonValue) -> Self {
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
    pub body: &'a crate::JsonValue,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateDraftQuery<'a> {
    pub fn new(draft_id: &'a str, body: &'a crate::JsonValue) -> Self {
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
        body: impl Serialize,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateDraftResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = CreateDraftQuery::new(&body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDraftQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDraftResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/lingo/v1/drafts",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CreateDraftRespData, CreateDraftResp>()
        .await
    }

    pub async fn update(
        &self,
        draft_id: &str,
        body: impl Serialize,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateDraftResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = UpdateDraftQuery::new(draft_id, &body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateDraftQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateDraftResp, LarkError> {
        let path = format!("/open-apis/lingo/v1/drafts/{}", query.draft_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<UpdateDraftRespData, UpdateDraftResp>()
        .await
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UploadFileQuery<'a> {
    pub body: &'a [FormDataField],
}

impl<'a> UploadFileQuery<'a> {
    pub fn new(body: &'a [FormDataField]) -> Self {
        Self { body }
    }
}

impl<'a> FileResource<'a> {
    pub async fn download(
        &self,
        file_token: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/lingo/v1/files/{file_token}/download");
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
        body: Vec<FormDataField>,
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/lingo/v1/files/upload",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .form_body(query.body.to_vec())
        .send_v2_response::<UploadFileRespData, UploadFileResp>()
        .await
    }
}

pub struct RepoResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListRepoQuery;

impl ListRepoQuery {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> RepoResource<'a> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListRepoResp, LarkError> {
        let query = ListRepoQuery::new();
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        _query: &ListRepoQuery,
        option: &RequestOption,
    ) -> Result<ListRepoResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/lingo/v1/repos",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<ListRepoRespData, ListRepoResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub entity: EntityResource<'a>,
    pub classification: ClassificationResource<'a>,
    pub draft: DraftResource<'a>,
    pub file: FileResource<'a>,
    pub repo: RepoResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            entity: EntityResource { config },
            classification: ClassificationResource { config },
            draft: DraftResource { config },
            file: FileResource { config },
            repo: RepoResource { config },
        }
    }
}
