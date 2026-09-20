//! Spark application, storage and database-sync APIs.
//!
#![doc = include_str!("../../../docs/spark-app-export.md")]

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{FormDataField, ReqBody, RequestOption};
use crate::service::common::{DownloadResp, DownloadStreamResp, JsonResp, PageQuery, RestRequest};
use crate::service::go_compatibility::{GoCompatibility, GoCompatibilityEndpoint};
use serde::Serialize;

pub type CreateAppResp = JsonResp;
pub type GetAppVisibilityAppResp = JsonResp;
pub type IconAppResp = JsonResp;
pub type ListAppResp = JsonResp;
pub type PatchAppResp = JsonResp;
pub type SqlCommandsAppResp = JsonResp;
pub type GetDbQuotaResp = JsonResp;
pub type UpdateAppVisibilityAppResp = JsonResp;
pub type UploadHtmlCodeAndReleaseAppResp = JsonResp;
pub type GetEnumDetailAppEnumResp = JsonResp;
pub type GetEnumListAppEnumResp = JsonResp;
pub type UploadAppStorageResp = JsonResp;
pub type UploadCompleteAppStorageResp = JsonResp;
pub type UploadInitializeAppStorageResp = JsonResp;
pub type UploadPartAppStorageResp = JsonResp;
pub type ListFilesAppStorageResp = JsonResp;
pub type GetFileAppStorageResp = JsonResp;
pub type SignFileAppStorageResp = JsonResp;
pub type GetFileQuotaAppStorageResp = JsonResp;
pub type PreUploadFileAppStorageResp = JsonResp;
pub type UploadFileCallbackAppStorageResp = JsonResp;
pub type BatchRemoveFilesAppStorageResp = JsonResp;
pub type BatchUpdateTableRecordsAppTableResp = JsonResp;
pub type DeleteTableRecordsAppTableResp = JsonResp;
pub type GetTableDetailAppTableResp = JsonResp;
pub type GetTableListAppTableResp = JsonResp;
pub type GetTableRecordListAppTableResp = JsonResp;
pub type PatchTableRecordsAppTableResp = JsonResp;
pub type PostTableRecordsAppTableResp = JsonResp;
pub type GetViewRecordListAppViewResp = JsonResp;
pub type IdConvertDirectoryUserResp = JsonResp;
pub type CreateDbSyncResp = JsonResp;
pub type ListDbSyncResp = JsonResp;
pub type GetDbSyncResp = JsonResp;
pub type UpdateDbSyncResp = JsonResp;
pub type EnableDbSyncResp = JsonResp;
pub type DisableDbSyncResp = JsonResp;
pub type DeleteDbSyncResp = JsonResp;

const EMPTY_PARAMS: [(&str, &str); 0] = [];

/// Query parameters for Spark database usage and quota.
///
/// Leave `env` unset to let the platform select the app's environment.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetDbQuotaQuery<'a> {
    pub app_id: &'a str,
    pub env: Option<&'a str>,
}

impl<'a> GetDbQuotaQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self { app_id, env: None }
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }
}

/// Exactly one app locator for source export. Empty locators are rejected locally.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportAppLocator<'a> {
    AppId(&'a str),
    MetaToken(&'a str),
}

/// Export failures retain HTTP metadata and a bounded response-body prefix.
#[derive(Debug, thiserror::Error)]
pub enum ExportAppError {
    #[error(transparent)]
    Request(#[from] LarkError),
    #[error("Spark app export returned a non-archive response (HTTP {status})", status = .api_resp.status_code)]
    InvalidResponse {
        /// At most 4 KiB of the response body is retained in `raw_body`.
        api_resp: Box<crate::resp::ApiResp>,
        /// Platform error, when the bounded body is a complete error envelope.
        code_error: Option<Box<crate::resp::CodeError>>,
    },
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct SparkPageQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> SparkPageQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(page_size) = self.page.page_size {
            pairs.push(("page_size", page_size.to_string()));
        }
        if let Some(page_token) = self.page.page_token {
            pairs.push(("page_token", page_token.to_string()));
        }
        pairs
    }
}

/// Query parameters for listing Base-to-database sync tasks.
///
/// The sync configuration itself remains open-ended JSON because the upstream
/// CLI is the current contract source and the API is not yet present in the
/// pinned Go SDK catalog.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListDbSyncQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub mode: Option<&'a str>,
    pub status: Option<&'a str>,
    pub table_name: Option<&'a str>,
    pub env: Option<&'a str>,
}

impl<'a> ListDbSyncQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            page: PageQuery::new(),
            mode: None,
            status: None,
            table_name: None,
            env: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn mode(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.mode = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn table_name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.table_name = value.into();
        self
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DownloadAppStorageQuery<'a> {
    pub app_id: &'a str,
    pub file_key: Option<&'a str>,
    pub file_url: Option<&'a str>,
}

/// Query parameters for listing files in a Spark app's storage.
///
/// Timestamps are forwarded in the platform's RFC 3339 format. The SDK does
/// not interpret relative dates or normalize them to a local timezone.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListFilesAppStorageQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub name: Option<&'a str>,
    pub path: Option<&'a str>,
    pub content_type: Option<&'a str>,
    pub size_gt: Option<i64>,
    pub size_lt: Option<i64>,
    pub uploaded_since: Option<&'a str>,
    pub uploaded_until: Option<&'a str>,
}

impl<'a> ListFilesAppStorageQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            ..Default::default()
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.name = value.into();
        self
    }

    pub fn path(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.path = value.into();
        self
    }

    pub fn content_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.content_type = value.into();
        self
    }

    pub fn size_gt(mut self, value: impl Into<Option<i64>>) -> Self {
        self.size_gt = value.into();
        self
    }

    pub fn size_lt(mut self, value: impl Into<Option<i64>>) -> Self {
        self.size_lt = value.into();
        self
    }

    pub fn uploaded_since(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.uploaded_since = value.into();
        self
    }

    pub fn uploaded_until(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.uploaded_until = value.into();
        self
    }
}

/// Query parameters for looking up one Spark storage file by its remote path.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetFileAppStorageQuery<'a> {
    pub app_id: &'a str,
    pub path: &'a str,
}

impl<'a> GetFileAppStorageQuery<'a> {
    pub fn new(app_id: &'a str, path: &'a str) -> Self {
        Self { app_id, path }
    }
}

impl<'a> DownloadAppStorageQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            file_key: None,
            file_url: None,
        }
    }

    pub fn file_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.file_key = value.into();
        self
    }

    pub fn file_url(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.file_url = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SparkRecordQuery<'a> {
    pub app_id: &'a str,
    pub table_name: &'a str,
    pub page: PageQuery<'a>,
    pub select: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub order: Option<&'a str>,
    pub env: Option<&'a str>,
    pub user_identifier_type: Option<&'a str>,
}

impl<'a> SparkRecordQuery<'a> {
    pub fn new(app_id: &'a str, table_name: &'a str) -> Self {
        Self {
            app_id,
            table_name,
            page: PageQuery::new(),
            select: None,
            filter: None,
            order: None,
            env: None,
            user_identifier_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn select(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.select = value.into();
        self
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn order(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.order = value.into();
        self
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }

    pub fn user_identifier_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_identifier_type = value.into();
        self
    }

    fn path_pairs(&self) -> [(&'static str, &'a str); 2] {
        [("app_id", self.app_id), ("table_name", self.table_name)]
    }

    fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(page_size) = self.page.page_size {
            pairs.push(("page_size", page_size.to_string()));
        }
        if let Some(page_token) = self.page.page_token {
            pairs.push(("page_token", page_token.to_string()));
        }
        if let Some(select) = self.select {
            pairs.push(("select", select.to_string()));
        }
        if let Some(filter) = self.filter {
            pairs.push(("filter", filter.to_string()));
        }
        if let Some(order) = self.order {
            pairs.push(("order", order.to_string()));
        }
        if let Some(env) = self.env {
            pairs.push(("env", env.to_string()));
        }
        if let Some(user_identifier_type) = self.user_identifier_type {
            pairs.push(("user_identifier_type", user_identifier_type.to_string()));
        }
        pairs
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SparkTableMutationQuery<'a> {
    pub app_id: &'a str,
    pub table_name: &'a str,
    pub body: &'a crate::JsonValue,
    pub filter: Option<&'a str>,
    pub env: Option<&'a str>,
    pub columns: Option<&'a str>,
    pub on_conflict: Option<&'a str>,
    pub user_identifier_type: Option<&'a str>,
}

impl<'a> SparkTableMutationQuery<'a> {
    pub fn new(app_id: &'a str, table_name: &'a str, body: &'a crate::JsonValue) -> Self {
        Self {
            app_id,
            table_name,
            body,
            filter: None,
            env: None,
            columns: None,
            on_conflict: None,
            user_identifier_type: None,
        }
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }

    pub fn columns(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.columns = value.into();
        self
    }

    pub fn on_conflict(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.on_conflict = value.into();
        self
    }

    pub fn user_identifier_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_identifier_type = value.into();
        self
    }

    fn path_pairs(&self) -> [(&'static str, &'a str); 2] {
        [("app_id", self.app_id), ("table_name", self.table_name)]
    }

    fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(filter) = self.filter {
            pairs.push(("filter", filter.to_string()));
        }
        if let Some(env) = self.env {
            pairs.push(("env", env.to_string()));
        }
        if let Some(columns) = self.columns {
            pairs.push(("columns", columns.to_string()));
        }
        if let Some(on_conflict) = self.on_conflict {
            pairs.push(("on_conflict", on_conflict.to_string()));
        }
        if let Some(user_identifier_type) = self.user_identifier_type {
            pairs.push(("user_identifier_type", user_identifier_type.to_string()));
        }
        pairs
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SparkViewRecordQuery<'a> {
    pub app_id: &'a str,
    pub view_name: &'a str,
    pub page: PageQuery<'a>,
    pub select: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub order: Option<&'a str>,
    pub env: Option<&'a str>,
    pub user_identifier_type: Option<&'a str>,
}

impl<'a> SparkViewRecordQuery<'a> {
    pub fn new(app_id: &'a str, view_name: &'a str) -> Self {
        Self {
            app_id,
            view_name,
            page: PageQuery::new(),
            select: None,
            filter: None,
            order: None,
            env: None,
            user_identifier_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn select(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.select = value.into();
        self
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn order(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.order = value.into();
        self
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }

    pub fn user_identifier_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_identifier_type = value.into();
        self
    }

    fn path_pairs(&self) -> [(&'static str, &'a str); 2] {
        [("app_id", self.app_id), ("view_name", self.view_name)]
    }

    fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(page_size) = self.page.page_size {
            pairs.push(("page_size", page_size.to_string()));
        }
        if let Some(page_token) = self.page.page_token {
            pairs.push(("page_token", page_token.to_string()));
        }
        if let Some(select) = self.select {
            pairs.push(("select", select.to_string()));
        }
        if let Some(filter) = self.filter {
            pairs.push(("filter", filter.to_string()));
        }
        if let Some(order) = self.order {
            pairs.push(("order", order.to_string()));
        }
        if let Some(env) = self.env {
            pairs.push(("env", env.to_string()));
        }
        if let Some(user_identifier_type) = self.user_identifier_type {
            pairs.push(("user_identifier_type", user_identifier_type.to_string()));
        }
        pairs
    }
}

pub struct AppResource<'a> {
    config: &'a Config,
}

/// Base-to-database sync task operations.
///
/// All endpoints currently require a user access token. Request and response
/// schemas are deliberately represented as serializable/generic JSON until an
/// authoritative OpenAPI schema is published by an SDK catalog.
pub struct DbSyncResource<'a> {
    config: &'a Config,
}

impl<'a> DbSyncResource<'a> {
    pub async fn create(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateDbSyncResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/db/sync_create",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    pub async fn list(
        &self,
        query: &ListDbSyncQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDbSyncResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/db/sync_list",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", query.app_id)
        .page_query(query.page)
        .query("mode", query.mode)
        .query("status", query.status)
        .query("table", query.table_name)
        .query("env", query.env)
        .send_json()
        .await
    }

    pub async fn get(
        &self,
        app_id: &str,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<GetDbSyncResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/db/sync_task",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .query("task_id", task_id)
        .send_json()
        .await
    }

    pub async fn update(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateDbSyncResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        RestRequest::new(
            self.config,
            http::Method::PUT,
            "/open-apis/spark/v1/apps/:app_id/db/sync_update",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    pub async fn enable(
        &self,
        app_id: &str,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<EnableDbSyncResp, LarkError> {
        let body = crate::JsonValue::from_serializable(serde_json::json!({ "task_id": task_id }))?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/db/sync_enable",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    pub async fn disable(
        &self,
        app_id: &str,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<DisableDbSyncResp, LarkError> {
        let body = crate::JsonValue::from_serializable(serde_json::json!({ "task_id": task_id }))?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/db/sync_disable",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    pub async fn delete(
        &self,
        app_id: &str,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDbSyncResp, LarkError> {
        let body = crate::JsonValue::from_serializable(serde_json::json!({ "task_id": task_id }))?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/db/sync_del",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(&body)?
        .send_json()
        .await
    }
}

impl<'a> AppResource<'a> {
    /// Stream app source using user credentials with `spark:app:read`.
    ///
    /// Only 2xx responses marked `application/zip`, `application/octet-stream`,
    /// `application/x-zip-compressed`, `binary/octet-stream`, or
    /// `application/force-download` are accepted (case-insensitive, ignoring
    /// parameters). Other bodies, including those with no Content-Type, are read
    /// up to 4 KiB and returned as [`ExportAppError::InvalidResponse`].
    /// This validates response metadata, not ZIP contents. No files are created.
    pub async fn export(
        &self,
        locator: ExportAppLocator<'_>,
        option: &RequestOption,
    ) -> Result<DownloadStreamResp, ExportAppError> {
        let (ExportAppLocator::AppId(value) | ExportAppLocator::MetaToken(value)) = locator;
        if value.trim().is_empty() {
            return Err(
                LarkError::IllegalParam("app export locator must not be empty".into()).into(),
            );
        }
        if option
            .user_access_token
            .as_deref()
            .is_none_or(|token| token.trim().is_empty())
        {
            return Err(
                LarkError::IllegalParam("app export requires a user access token".into()).into(),
            );
        }
        const ERROR_LIMIT: usize = 4096;
        let mut response = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/export",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(&locator)?
        .download_stream_with_error_limit(Some(ERROR_LIMIT))
        .await?;
        let content_type = response
            .api_resp
            .header
            .get(http::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .split(';')
            .next()
            .unwrap_or("")
            .trim();
        if (200..300).contains(&response.api_resp.status_code)
            && (content_type.eq_ignore_ascii_case("application/zip")
                || content_type.eq_ignore_ascii_case("application/octet-stream")
                || content_type.eq_ignore_ascii_case("application/x-zip-compressed")
                || content_type.eq_ignore_ascii_case("binary/octet-stream")
                || content_type.eq_ignore_ascii_case("application/force-download"))
        {
            return Ok(response);
        }
        let mut body = Vec::new();
        while body.len() < ERROR_LIMIT {
            let Some(chunk) = response.body.next_chunk().await? else {
                break;
            };
            body.extend_from_slice(&chunk[..chunk.len().min(ERROR_LIMIT - body.len())]);
        }
        let code_error = serde_json::from_slice::<crate::resp::CodeError>(&body)
            .ok()
            .filter(|error| !error.success())
            .map(Box::new);
        response.api_resp.raw_body = body;
        Err(ExportAppError::InvalidResponse {
            api_resp: Box::new(response.api_resp),
            code_error,
        })
    }

    pub async fn create(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateAppResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PostSparkV1Apps,
                EMPTY_PARAMS,
                EMPTY_PARAMS,
                Some(&body),
                option,
            )
            .await
    }

    pub async fn get_app_visibility(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<GetAppVisibilityAppResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1AppsByAppIdAccessScope,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn icon(
        &self,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<IconAppResp, LarkError> {
        GoCompatibility::new(self.config)
            .request(
                GoCompatibilityEndpoint::PostSparkV1Icon,
                EMPTY_PARAMS,
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }

    pub async fn list(
        &self,
        query: &SparkPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAppResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1Apps,
                EMPTY_PARAMS,
                query.as_pairs(),
                None,
                option,
            )
            .await
    }

    pub async fn patch(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchAppResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PatchSparkV1AppsByAppId,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(&body),
                option,
            )
            .await
    }

    /// Read database usage and quota with a user token and `spark:app:read`.
    ///
    /// An unset environment is omitted from the request for server-side
    /// selection. The response preserves platform fields, including zero quotas
    /// and unknown fields, without CLI-style rounding or projection.
    pub async fn get_db_quota(
        &self,
        query: &GetDbQuotaQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDbQuotaResp, LarkError> {
        if option
            .user_access_token
            .as_deref()
            .is_none_or(|token| token.trim().is_empty())
        {
            return Err(LarkError::IllegalParam(
                "Spark database quota requires a user access token".into(),
            ));
        }
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/db/quota",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", query.app_id)
        .query("env", query.env)
        .send_json()
        .await
    }

    pub async fn sql_commands(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<SqlCommandsAppResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PostSparkV1AppsByAppIdSqlCommands,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(&body),
                option,
            )
            .await
    }

    pub async fn update_app_visibility(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateAppVisibilityAppResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PutSparkV1AppsByAppIdAccessScope,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(&body),
                option,
            )
            .await
    }

    pub async fn upload_html_code_and_release(
        &self,
        app_id: &str,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<UploadHtmlCodeAndReleaseAppResp, LarkError> {
        GoCompatibility::new(self.config)
            .request(
                GoCompatibilityEndpoint::PostSparkV1AppsByAppIdUploadAndReleaseHtmlCode,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }
}

pub struct AppEnumResource<'a> {
    config: &'a Config,
}

impl<'a> AppEnumResource<'a> {
    pub async fn get_enum_detail(
        &self,
        app_id: &str,
        enum_name: &str,
        option: &RequestOption,
    ) -> Result<GetEnumDetailAppEnumResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1AppsByAppIdEnumsByEnumName,
                [("app_id", app_id), ("enum_name", enum_name)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn get_enum_list(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<GetEnumListAppEnumResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1AppsByAppIdEnums,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }
}

pub struct AppStorageResource<'a> {
    config: &'a Config,
}

impl<'a> AppStorageResource<'a> {
    /// List files stored by a Spark app.
    pub async fn list_files(
        &self,
        query: &ListFilesAppStorageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFilesAppStorageResp, LarkError> {
        require_storage_user_access_token(option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/storage/file_list",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", query.app_id)
        .page_query(query.page)
        .query("name", query.name)
        .query("path", query.path)
        .query("type", query.content_type)
        .query("size_gt", query.size_gt)
        .query("size_lt", query.size_lt)
        .query("uploaded_since", query.uploaded_since)
        .query("uploaded_until", query.uploaded_until)
        .send_json()
        .await
    }

    /// Get one Spark storage file's metadata by its remote path.
    pub async fn get_file(
        &self,
        query: &GetFileAppStorageQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetFileAppStorageResp, LarkError> {
        require_storage_user_access_token(option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/storage/file",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", query.app_id)
        .query("path", query.path)
        .send_json()
        .await
    }

    /// Create a temporary signed download URL for a storage file.
    pub async fn sign_file(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<SignFileAppStorageResp, LarkError> {
        require_storage_user_access_token(option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/storage/file_sign",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(body)?
        .send_json()
        .await
    }

    /// Get the app's file-storage usage and quota reported by the platform.
    pub async fn get_file_quota(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<GetFileQuotaAppStorageResp, LarkError> {
        require_storage_user_access_token(option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/storage/file_quota",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .send_json()
        .await
    }

    /// Request a presigned upload URL and its upload ID.
    ///
    /// Upload the bytes to the returned external URL yourself, then pass its
    /// returned ETag to [`Self::upload_file_callback`]. The SDK intentionally
    /// does not forward Lark credentials to that URL.
    pub async fn pre_upload_file(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<PreUploadFileAppStorageResp, LarkError> {
        require_storage_user_access_token(option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/storage/file_pre_upload",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(body)?
        .send_json()
        .await
    }

    /// Register a presigned upload using its platform-issued upload ID and ETag.
    pub async fn upload_file_callback(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<UploadFileCallbackAppStorageResp, LarkError> {
        require_storage_user_access_token(option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/storage/file_upload_callback",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(body)?
        .send_json()
        .await
    }

    /// Delete storage files by path.
    ///
    /// Successful transport only means the batch endpoint accepted the request;
    /// inspect the platform's per-item results in `data` for partial failures.
    pub async fn batch_remove_files(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchRemoveFilesAppStorageResp, LarkError> {
        require_storage_user_access_token(option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/storage/file_batch_remove",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .json_body(body)?
        .send_json()
        .await
    }

    pub async fn download_by_query(
        &self,
        query: &DownloadAppStorageQuery<'_>,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/spark/v1/apps/{}/storage", query.app_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("file_key", query.file_key)
        .query("file_url", query.file_url)
        .download()
        .await
    }

    pub async fn upload(
        &self,
        app_id: &str,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<UploadAppStorageResp, LarkError> {
        GoCompatibility::new(self.config)
            .request(
                GoCompatibilityEndpoint::PostSparkV1AppsByAppIdStorageUpload,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }

    pub async fn upload_complete(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<UploadCompleteAppStorageResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PostSparkV1AppsByAppIdStorageUploadComplete,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(&body),
                option,
            )
            .await
    }

    pub async fn upload_initialize(
        &self,
        app_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<UploadInitializeAppStorageResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PostSparkV1AppsByAppIdStorageUploadInitialize,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(&body),
                option,
            )
            .await
    }

    pub async fn upload_part(
        &self,
        app_id: &str,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<UploadPartAppStorageResp, LarkError> {
        GoCompatibility::new(self.config)
            .request(
                GoCompatibilityEndpoint::PostSparkV1AppsByAppIdStorageUploadPart,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }
}

fn require_storage_user_access_token(option: &RequestOption) -> Result<(), LarkError> {
    if option
        .user_access_token
        .as_deref()
        .is_none_or(|token| token.trim().is_empty())
    {
        return Err(LarkError::IllegalParam(
            "Spark storage management requires a user access token".into(),
        ));
    }
    Ok(())
}

pub struct AppTableResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableResource<'a> {
    pub async fn batch_update_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchUpdateTableRecordsAppTableResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PatchSparkV1AppsByAppIdTablesByTableNameRecordsBatchUpdate,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }

    pub async fn delete_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<DeleteTableRecordsAppTableResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::DeleteSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }

    pub async fn get_table_detail(
        &self,
        app_id: &str,
        table_name: &str,
        option: &RequestOption,
    ) -> Result<GetTableDetailAppTableResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1AppsByAppIdTablesByTableName,
                [("app_id", app_id), ("table_name", table_name)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn get_table_list(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<GetTableListAppTableResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1AppsByAppIdTables,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn get_table_record_list(
        &self,
        query: &SparkRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetTableRecordListAppTableResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                None,
                option,
            )
            .await
    }

    pub async fn patch_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchTableRecordsAppTableResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PatchSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }

    pub async fn post_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<PostTableRecordsAppTableResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PostSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }
}

pub struct AppViewResource<'a> {
    config: &'a Config,
}

impl<'a> AppViewResource<'a> {
    pub async fn get_view_record_list(
        &self,
        app_id: &str,
        view_name: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetViewRecordListAppViewResp, LarkError> {
        let query = SparkViewRecordQuery::new(app_id, view_name)
            .page_size(page_size)
            .page_token(page_token);
        self.get_view_record_list_by_query(&query, option).await
    }

    pub async fn get_view_record_list_by_query(
        &self,
        query: &SparkViewRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetViewRecordListAppViewResp, LarkError> {
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::GetSparkV1AppsByAppIdViewsByViewNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                None,
                option,
            )
            .await
    }
}

pub struct DirectoryUserResource<'a> {
    config: &'a Config,
}

impl<'a> DirectoryUserResource<'a> {
    pub async fn id_convert(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<IdConvertDirectoryUserResp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        GoCompatibility::new(self.config)
            .request_json(
                GoCompatibilityEndpoint::PostSparkV1DirectoryUserIdConvert,
                EMPTY_PARAMS,
                EMPTY_PARAMS,
                Some(&body),
                option,
            )
            .await
    }
}

pub struct V1<'a> {
    pub app: AppResource<'a>,
    pub db_sync: DbSyncResource<'a>,
    pub app_enum: AppEnumResource<'a>,
    pub app_storage: AppStorageResource<'a>,
    pub app_table: AppTableResource<'a>,
    pub app_view: AppViewResource<'a>,
    pub directory_user: DirectoryUserResource<'a>,
    config: &'a Config,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app: AppResource { config },
            db_sync: DbSyncResource { config },
            app_enum: AppEnumResource { config },
            app_storage: AppStorageResource { config },
            app_table: AppTableResource { config },
            app_view: AppViewResource { config },
            directory_user: DirectoryUserResource { config },
            config,
        }
    }

    pub fn go_compatibility(&self) -> GoCompatibility<'a> {
        GoCompatibility::new(self.config)
    }

    /// Deprecated name for [`Self::go_compatibility`].
    #[deprecated(note = "use go_compatibility")]
    pub fn go_v397(&self) -> GoCompatibility<'a> {
        self.go_compatibility()
    }
}
