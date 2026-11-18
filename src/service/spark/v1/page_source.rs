//! Shared creative app source-tree reads.

use serde::Deserialize;

use super::{AppResource, require_app_user};
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::service::common::{DownloadStreamResp, FromRawResponse, PageQuery, RestRequest};

const PAGE_SOURCE_ERROR_LIMIT: usize = 4096;

fn required_meta_token(value: &str) -> Result<&str, LarkError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(LarkError::IllegalParam("meta_token is required".into()));
    }
    if value
        .chars()
        .any(|character| character.is_whitespace() || character.is_control())
        || value
            .chars()
            .any(|character| matches!(character, '/' | '\\' | ':' | '?' | '#'))
    {
        return Err(LarkError::IllegalParam(
            "meta_token must be a bare share token".into(),
        ));
    }
    Ok(value)
}

fn validate_source_path(path: &str, required: bool) -> Result<&str, LarkError> {
    let drive_path =
        path.len() >= 2 && path.as_bytes()[1] == b':' && path.as_bytes()[0].is_ascii_alphabetic();
    if required && (path.trim().is_empty() || matches!(path, "." | "./")) {
        return Err(LarkError::IllegalParam(
            "path must name a source file".into(),
        ));
    }
    if path.starts_with('/')
        || drive_path
        || path.contains('\\')
        || path.split('/').any(|segment| segment == "..")
        || path.chars().any(char::is_control)
    {
        return Err(LarkError::IllegalParam(
            "path must be a relative project path".into(),
        ));
    }
    Ok(path)
}

/// Query parameters for listing one directory of a shared creative app.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PageSourceListQuery<'a> {
    pub meta_token: &'a str,
    pub path: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> PageSourceListQuery<'a> {
    pub fn new(meta_token: &'a str) -> Self {
        Self {
            meta_token,
            path: None,
            page: PageQuery::new(),
        }
    }

    pub fn path(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.path = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    fn validate(&self) -> Result<(&str, Option<&str>), LarkError> {
        let meta_token = required_meta_token(self.meta_token)?;
        if self
            .page
            .page_size
            .is_some_and(|size| !(1..=100).contains(&size))
        {
            return Err(LarkError::IllegalParam(
                "page_size must be between 1 and 100".into(),
            ));
        }
        let path = self
            .path
            .filter(|path| !path.is_empty())
            .map(|path| validate_source_path(path, false))
            .transpose()?;
        Ok((meta_token, path))
    }
}

/// One file or directory in a shared creative app source tree.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct PageSourceItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default, rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub size: i64,
}

/// One page of a shared creative app source tree.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct PageSourceListData {
    #[serde(default)]
    pub items: Vec<PageSourceItem>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PageSourceListResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
    pub data: Option<PageSourceListData>,
}

impl PageSourceListResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

impl FromRawResponse<PageSourceListData> for PageSourceListResp {
    fn from_raw_response(api_resp: ApiResp, raw: RawResponse<PageSourceListData>) -> Self {
        Self {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        }
    }
}

impl<'a> AppResource<'a> {
    /// List one directory of a shared creative app's source tree.
    ///
    /// This endpoint requires a user access token with `spark:app:read`.
    /// `path` is relative to the project root. A missing path lists the root.
    pub async fn list_page_source(
        &self,
        query: &PageSourceListQuery<'_>,
        option: &RequestOption,
    ) -> Result<PageSourceListResp, LarkError> {
        require_app_user(option)?;
        let (meta_token, path) = query.validate()?;
        let request = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/source/files",
            vec![AccessTokenType::User],
            option,
        )
        .query("meta_token", meta_token)
        .query("path", path)
        .query("page_size", query.page.page_size)
        .query("page_token", query.page.page_token);
        request
            .send_response::<PageSourceListData, PageSourceListResp>()
            .await
    }

    /// Stream one source file from a shared creative app.
    ///
    /// The response body is kept as raw bytes, even when it resembles a JSON
    /// API response. The platform limits source files to 10 MiB. This method
    /// requires a user access token with `spark:app:read`.
    pub async fn get_page_source(
        &self,
        meta_token: &str,
        path: &str,
        option: &RequestOption,
    ) -> Result<DownloadStreamResp, LarkError> {
        require_app_user(option)?;
        let meta_token = required_meta_token(meta_token)?;
        let path = validate_source_path(path, true)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/source/file",
            vec![AccessTokenType::User],
            option,
        )
        .query("meta_token", meta_token)
        .query("path", path)
        .download_stream_with_error_limit(Some(PAGE_SOURCE_ERROR_LIMIT))
        .await
    }
}
