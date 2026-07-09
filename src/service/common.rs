use std::collections::VecDeque;

pub use crate::config::Config;
pub use crate::constants::AccessTokenType;
use crate::error::LarkError;
pub use crate::req::{ApiReq, ReqBody, RequestOption};
pub use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::transport;

pub(crate) trait QueryValue {
    fn set_query(self, req: &mut ApiReq, key: &str);
}

pub(crate) trait FromRawResponse<T> {
    fn from_raw_response(api_resp: ApiResp, raw: RawResponse<T>) -> Self;
}

pub(crate) trait FromV2Response<T> {
    fn from_v2_response(api_resp: ApiResp, code_error: Option<CodeError>, data: Option<T>) -> Self;
}

impl<T: QueryValue> QueryValue for Option<T> {
    fn set_query(self, req: &mut ApiReq, key: &str) {
        if let Some(value) = self {
            value.set_query(req, key);
        }
    }
}

impl QueryValue for &str {
    fn set_query(self, req: &mut ApiReq, key: &str) {
        req.query_params.set(key, self);
    }
}

impl QueryValue for String {
    fn set_query(self, req: &mut ApiReq, key: &str) {
        req.query_params.set(key, self);
    }
}

impl QueryValue for &String {
    fn set_query(self, req: &mut ApiReq, key: &str) {
        req.query_params.set(key, self);
    }
}

macro_rules! impl_query_value_to_string {
    ($($ty:ty),* $(,)?) => {
        $(
            impl QueryValue for $ty {
                fn set_query(self, req: &mut ApiReq, key: &str) {
                    req.query_params.set(key, self.to_string());
                }
            }
        )*
    };
}

impl_query_value_to_string!(bool, i32, i64, u32, u64, usize);

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct PageQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> PageQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parts(page_size: Option<i32>, page_token: Option<&'a str>) -> Self {
        Self {
            page_size,
            page_token,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }
}

pub(crate) struct RestRequest<'a> {
    config: &'a Config,
    api_req: ApiReq,
    option: &'a RequestOption,
    file_upload: bool,
}

impl<'a> RestRequest<'a> {
    pub(crate) fn new(
        config: &'a Config,
        method: http::Method,
        path: impl Into<String>,
        supported_access_token_types: Vec<AccessTokenType>,
        option: &'a RequestOption,
    ) -> Self {
        let mut api_req = ApiReq::new(method, path);
        api_req.supported_access_token_types = supported_access_token_types;
        Self {
            config,
            api_req,
            option,
            file_upload: false,
        }
    }

    pub(crate) fn from_api_req(
        config: &'a Config,
        api_req: ApiReq,
        option: &'a RequestOption,
    ) -> Self {
        Self {
            config,
            api_req,
            option,
            file_upload: false,
        }
    }

    pub(crate) fn file_upload(mut self) -> Self {
        self.file_upload = true;
        self
    }

    pub(crate) fn query<T: QueryValue>(mut self, key: &str, value: T) -> Self {
        value.set_query(&mut self.api_req, key);
        self
    }

    pub(crate) fn query_values<I, T>(mut self, key: &str, values: Option<I>) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        if let Some(values) = values {
            for value in values {
                self.api_req.query_params.add(key, value.to_string());
            }
        }
        self
    }

    pub(crate) fn page_query(self, page: PageQuery<'_>) -> Self {
        self.query("page_size", page.page_size)
            .query("page_token", page.page_token)
    }

    pub(crate) fn json_body<T: serde::Serialize>(mut self, body: &T) -> Result<Self, LarkError> {
        self.api_req.body = Some(ReqBody::json(body)?);
        Ok(self)
    }

    pub(crate) fn form_body(mut self, fields: Vec<crate::req::FormDataField>) -> Self {
        self.api_req.body = Some(ReqBody::FormData(fields));
        self
    }

    pub(crate) async fn send<T: for<'de> serde::Deserialize<'de>>(
        self,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        let mut option;
        let request_option = if self.file_upload {
            option = self.option.clone();
            option.file_upload = true;
            &option
        } else {
            self.option
        };
        transport::request_typed::<T>(self.config, &self.api_req, request_option).await
    }

    pub(crate) async fn send_response<T, R>(self) -> Result<R, LarkError>
    where
        T: for<'de> serde::Deserialize<'de>,
        R: FromRawResponse<T>,
    {
        let (api_resp, raw) = self.send::<T>().await?;
        Ok(R::from_raw_response(api_resp, raw))
    }

    pub(crate) async fn send_v2<T: for<'de> serde::Deserialize<'de>>(
        self,
    ) -> Result<(ApiResp, Option<CodeError>, Option<T>), LarkError> {
        let (api_resp, raw) = self.send::<T>().await?;
        Ok(parse_v2(api_resp, raw))
    }

    pub(crate) async fn send_v2_response<T, R>(self) -> Result<R, LarkError>
    where
        T: for<'de> serde::Deserialize<'de>,
        R: FromV2Response<T>,
    {
        let (api_resp, code_error, data) = self.send_v2::<T>().await?;
        Ok(R::from_v2_response(api_resp, code_error, data))
    }

    pub(crate) async fn send_empty(self) -> Result<EmptyResp, LarkError> {
        self.send_response::<serde_json::Value, EmptyResp>().await
    }

    pub(crate) async fn send_json(self) -> Result<JsonResp, LarkError> {
        let (api_resp, code_error, data) = self.send_v2::<serde_json::Value>().await?;
        Ok(JsonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub(crate) async fn download(self) -> Result<DownloadResp, LarkError> {
        let mut option = self.option.clone();
        option.file_download = true;
        let api_resp = transport::request(self.config, &self.api_req, &option).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }

    pub(crate) async fn download_stream(self) -> Result<DownloadStreamResp, LarkError> {
        let mut option = self.option.clone();
        option.file_download = true;
        let stream_resp = transport::request_stream(self.config, &self.api_req, &option).await?;
        let file_name = stream_resp.api_resp.file_name_by_header();
        let content_length = stream_resp.api_resp.content_length();

        Ok(DownloadStreamResp {
            api_resp: stream_resp.api_resp,
            file_name,
            content_length,
            body: DownloadBody {
                inner: stream_resp.body,
            },
        })
    }
}

pub fn parse_v2<T>(
    api_resp: ApiResp,
    raw: RawResponse<T>,
) -> (ApiResp, Option<CodeError>, Option<T>) {
    if raw.code_error.code != 0 {
        (api_resp, Some(raw.code_error), None)
    } else {
        (api_resp, None, raw.data)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PageIteratorState<T> {
    next_page_token: Option<String>,
    request_page_token: Option<String>,
    items: VecDeque<T>,
    started: bool,
    exhausted: bool,
    limit: Option<usize>,
    emitted: usize,
}

impl<T> Default for PageIteratorState<T> {
    fn default() -> Self {
        Self {
            next_page_token: None,
            request_page_token: None,
            items: VecDeque::new(),
            started: false,
            exhausted: false,
            limit: None,
            emitted: 0,
        }
    }
}

impl<T> PageIteratorState<T> {
    pub(crate) fn limit(mut self, limit: usize) -> Self {
        self.limit = (limit > 0).then_some(limit);
        self
    }

    pub(crate) fn with_page_token(mut self, page_token: Option<String>) -> Self {
        self.request_page_token = page_token;
        self.started = self.request_page_token.is_some();
        self.exhausted = false;
        self
    }

    pub(crate) fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    pub(crate) fn page_token_for_request(&self) -> Option<&str> {
        if self.started {
            self.request_page_token
                .as_deref()
                .or(self.next_page_token.as_deref())
        } else {
            None
        }
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        if self.limit.is_some_and(|limit| self.emitted >= limit) {
            return None;
        }
        let item = self.items.pop_front()?;
        self.emitted += 1;
        Some(item)
    }

    pub(crate) fn should_fetch(&self) -> bool {
        self.limit.is_none_or(|limit| self.emitted < limit)
            && self.items.is_empty()
            && !self.exhausted
            && (!self.started
                || self.request_page_token.is_some()
                || self.next_page_token.is_some())
    }

    pub(crate) fn accept_page(
        &mut self,
        items: Option<Vec<T>>,
        page_token: Option<String>,
        has_more: Option<bool>,
    ) {
        let prev_page_token = self.next_page_token.clone();
        self.started = true;
        self.request_page_token = None;
        self.items = items.unwrap_or_default().into();
        self.next_page_token = if self.items.is_empty() {
            prev_page_token
        } else {
            page_token
        };
        self.exhausted =
            self.items.is_empty() || !has_more.unwrap_or(false) || self.next_page_token.is_none();
    }
}

macro_rules! impl_page_iterator_controls {
    ($iter:ident) => {
        impl<'a> $iter<'a> {
            pub fn limit(mut self, limit: usize) -> Self {
                self.state = self.state.limit(limit);
                self
            }

            pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
                self.state = self.state.with_page_token(Some(page_token.into()));
                self
            }

            pub fn next_page_token(&self) -> Option<&str> {
                self.state.next_page_token()
            }
        }
    };
}

pub(crate) use impl_page_iterator_controls;

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

impl<T> FromRawResponse<T> for EmptyResp {
    fn from_raw_response(api_resp: ApiResp, raw: RawResponse<T>) -> Self {
        Self {
            api_resp,
            code_error: raw.code_error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DownloadResp {
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct DownloadStreamResp {
    /// HTTP metadata available before the body is consumed.
    ///
    /// The streaming path does not pre-read 2xx JSON bodies to discover Lark
    /// code errors, because doing so would either buffer the download or expose
    /// metadata for a response that may later be retried. Non-success JSON
    /// responses may still be buffered to preserve token-refresh retries.
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub content_length: Option<u64>,
    pub body: DownloadBody,
}

#[derive(Debug)]
pub struct DownloadBody {
    inner: transport::StreamBody,
}

impl DownloadBody {
    pub async fn next_chunk(&mut self) -> Result<Option<bytes::Bytes>, LarkError> {
        self.inner.next_chunk().await
    }
}

#[derive(Debug, Clone)]
pub struct EmptyRespV2 {
    pub api_resp: ApiResp,
    pub code_error: Option<CodeError>,
}

impl EmptyRespV2 {
    pub fn success(&self) -> bool {
        self.code_error.as_ref().is_none_or(|e| e.code == 0)
    }
}

impl<T> FromV2Response<T> for EmptyRespV2 {
    fn from_v2_response(
        api_resp: ApiResp,
        code_error: Option<CodeError>,
        _data: Option<T>,
    ) -> Self {
        Self {
            api_resp,
            code_error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonResp {
    pub api_resp: ApiResp,
    pub code_error: Option<CodeError>,
    pub data: Option<serde_json::Value>,
}

impl JsonResp {
    pub fn success(&self) -> bool {
        self.code_error.as_ref().is_none_or(|e| e.code == 0)
    }
}

impl FromV2Response<serde_json::Value> for JsonResp {
    fn from_v2_response(
        api_resp: ApiResp,
        code_error: Option<CodeError>,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            api_resp,
            code_error,
            data,
        }
    }
}

pub async fn request_json(
    config: &Config,
    method: http::Method,
    path: impl Into<String>,
    supported_access_token_types: Vec<AccessTokenType>,
    body: Option<&serde_json::Value>,
    option: &RequestOption,
) -> Result<JsonResp, LarkError> {
    let request = RestRequest::new(config, method, path, supported_access_token_types, option);
    let request = if let Some(body) = body {
        request.json_body(body)?
    } else {
        request
    };
    request.send_json().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_resp_data_stays_vec_u8() {
        let resp = DownloadResp {
            api_resp: ApiResp {
                status_code: 200,
                header: http::HeaderMap::new(),
                raw_body: b"download-bytes".to_vec(),
            },
            file_name: Some("resource.bin".to_string()),
            data: b"download-bytes".to_vec(),
        };

        let data: Vec<u8> = resp.data;
        assert_eq!(data, b"download-bytes");
    }
}
