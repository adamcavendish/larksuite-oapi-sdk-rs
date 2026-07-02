use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocsDocument {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocsMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_modify_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<DocsDocument>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_metas: Option<Vec<DocsMeta>>,
}

impl_resp!(GetDocumentResp, DocumentData);
impl_resp!(BatchGetDocumentMetaResp, MetaData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

impl_resp!(GetContentResp, ContentData);

#[derive(Debug, Clone, Copy)]
pub struct GetDocumentQuery<'a> {
    pub document_id: &'a str,
}

impl<'a> GetDocumentQuery<'a> {
    pub fn new(document_id: &'a str) -> Self {
        Self { document_id }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetContentQuery<'a> {
    pub doc_token: &'a str,
    pub doc_type: Option<&'a str>,
    pub content_type: Option<&'a str>,
    pub lang: Option<&'a str>,
}

impl<'a> GetContentQuery<'a> {
    pub fn new(doc_token: &'a str) -> Self {
        Self {
            doc_token,
            doc_type: None,
            content_type: None,
            lang: None,
        }
    }

    pub fn doc_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.doc_type = value.into();
        self
    }

    pub fn content_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.content_type = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

// ── Resources ──

pub struct DocumentResource<'a> {
    config: &'a Config,
}

impl<'a> DocumentResource<'a> {
    pub async fn get(
        &self,
        document_id: &str,
        option: &RequestOption,
    ) -> Result<GetDocumentResp, LarkError> {
        self.get_by_query(&GetDocumentQuery::new(document_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDocumentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDocumentResp, LarkError> {
        let path = format!("/open-apis/docs/v1/documents/{}", query.document_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<DocumentData>()
        .await?;
        Ok(GetDocumentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ContentResource<'a> {
    config: &'a Config,
}

impl<'a> ContentResource<'a> {
    pub async fn get(
        &self,
        doc_token: &str,
        doc_type: Option<&str>,
        content_type: Option<&str>,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetContentResp, LarkError> {
        let query = GetContentQuery::new(doc_token)
            .doc_type(doc_type)
            .content_type(content_type)
            .lang(lang);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetContentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetContentResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/docs/v1/content",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("doc_token", query.doc_token)
        .query("doc_type", query.doc_type)
        .query("content_type", query.content_type)
        .query("lang", query.lang)
        .send::<ContentData>()
        .await?;
        Ok(GetContentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub document: DocumentResource<'a>,
    pub content: ContentResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            document: DocumentResource { config },
            content: ContentResource { config },
        }
    }
}
