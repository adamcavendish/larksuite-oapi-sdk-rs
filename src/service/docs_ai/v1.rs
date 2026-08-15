use serde::Serialize;

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{JsonResp, PageQuery, RestRequest};

pub type CreateDocumentResp = JsonResp;
pub type FetchDocumentResp = JsonResp;
pub type UpdateDocumentResp = JsonResp;
pub type ListDocumentHistoryResp = JsonResp;
pub type RevertDocumentHistoryResp = JsonResp;
pub type GetDocumentHistoryRevertStatusResp = JsonResp;

/// Query parameters for listing document history versions.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListDocumentHistoryQuery<'a> {
    pub document_id: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListDocumentHistoryQuery<'a> {
    pub fn new(document_id: &'a str) -> Self {
        Self {
            document_id,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for a document history-revert task.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetDocumentHistoryRevertStatusQuery<'a> {
    pub document_id: &'a str,
    pub task_id: &'a str,
}

impl<'a> GetDocumentHistoryRevertStatusQuery<'a> {
    pub fn new(document_id: &'a str, task_id: &'a str) -> Self {
        Self {
            document_id,
            task_id,
        }
    }
}

/// Docs AI v1 operations proven by the official Lark CLI's Open Platform client.
///
/// This module deliberately stays separate from legacy `docs/v1`, structured
/// `docx/v1`, and OCR-oriented `document_ai/v1`. The Docs AI request and
/// response schemas are not present in the pinned Go SDK catalog, so callers
/// pass serializable JSON payloads and receive [`JsonResp`] values unchanged.
pub struct V1<'a> {
    pub document: DocumentResource<'a>,
    pub history: DocumentHistoryResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            document: DocumentResource { config },
            history: DocumentHistoryResource { config },
        }
    }
}

/// Create, fetch, and update Docs AI documents.
pub struct DocumentResource<'a> {
    config: &'a Config,
}

impl DocumentResource<'_> {
    /// Creates a document from the upstream JSON request body.
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateDocumentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/docs_ai/v1/documents",
            supported_access_tokens(),
            option,
        )
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Fetches document content with the upstream JSON request body.
    ///
    /// For example, callers may provide `format`, `read_option`,
    /// `export_option`, and the CLI-proven `extra_param` comment options.
    pub async fn fetch(
        &self,
        document_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<FetchDocumentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/docs_ai/v1/documents/:document_id/fetch",
            supported_access_tokens(),
            option,
        )
        .path_param("document_id", document_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Updates document content with the upstream JSON request body.
    ///
    /// Commands such as `block_replace` and `block_delete` may carry stable
    /// outer fields such as `start_block_id` and `end_block_id`; the SDK leaves
    /// command-specific validation to the Open Platform service.
    pub async fn update(
        &self,
        document_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateDocumentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::PUT,
            "/open-apis/docs_ai/v1/documents/:document_id",
            supported_access_tokens(),
            option,
        )
        .path_param("document_id", document_id)
        .json_body(&body)?
        .send_json()
        .await
    }
}

/// List, revert, and inspect Docs AI document history.
pub struct DocumentHistoryResource<'a> {
    config: &'a Config,
}

impl DocumentHistoryResource<'_> {
    /// Lists document history versions.
    pub async fn list(
        &self,
        query: &ListDocumentHistoryQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDocumentHistoryResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/docs_ai/v1/documents/:document_id/histories",
            supported_access_tokens(),
            option,
        )
        .path_param("document_id", query.document_id)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Starts a document history revert with the upstream JSON request body.
    ///
    /// The CLI-proven body contains `history_version_id` and optional
    /// `wait_timeout_ms`; the dynamic payload preserves future service fields.
    pub async fn revert(
        &self,
        document_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<RevertDocumentHistoryResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/docs_ai/v1/documents/:document_id/history/revert",
            supported_access_tokens(),
            option,
        )
        .path_param("document_id", document_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Gets the status of a document history-revert task.
    pub async fn revert_status(
        &self,
        query: &GetDocumentHistoryRevertStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDocumentHistoryRevertStatusResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/docs_ai/v1/documents/:document_id/history/revert_status",
            supported_access_tokens(),
            option,
        )
        .path_param("document_id", query.document_id)
        .query("task_id", query.task_id)
        .send_json()
        .await
    }
}

fn supported_access_tokens() -> Vec<AccessTokenType> {
    vec![AccessTokenType::User, AccessTokenType::Tenant]
}
