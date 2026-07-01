pub use crate::config::Config;
pub use crate::constants::AccessTokenType;
use crate::error::LarkError;
pub use crate::req::{ApiReq, ReqBody, RequestOption};
pub use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::transport;

pub(crate) trait QueryValue {
    fn set_query(self, req: &mut ApiReq, key: &str);
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

pub(crate) struct RestRequest<'a> {
    config: &'a Config,
    api_req: ApiReq,
    option: &'a RequestOption,
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
        }
    }

    pub(crate) fn query<T: QueryValue>(mut self, key: &str, value: T) -> Self {
        value.set_query(&mut self.api_req, key);
        self
    }

    pub(crate) fn json_body<T: serde::Serialize>(mut self, body: &T) -> Result<Self, LarkError> {
        self.api_req.body = Some(ReqBody::json(body)?);
        Ok(self)
    }

    pub(crate) async fn send<T: for<'de> serde::Deserialize<'de>>(
        self,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        transport::request_typed::<T>(self.config, &self.api_req, self.option).await
    }

    pub(crate) async fn send_v2<T: for<'de> serde::Deserialize<'de>>(
        self,
    ) -> Result<(ApiResp, Option<CodeError>, Option<T>), LarkError> {
        let (api_resp, raw) = self.send::<T>().await?;
        Ok(parse_v2(api_resp, raw))
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
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

#[derive(Debug, Clone)]
pub struct DownloadResp {
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub data: Vec<u8>,
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

pub async fn request_json(
    config: &Config,
    method: http::Method,
    path: impl Into<String>,
    supported_access_token_types: Vec<AccessTokenType>,
    body: Option<&serde_json::Value>,
    option: &RequestOption,
) -> Result<JsonResp, LarkError> {
    let mut api_req = ApiReq::new(method, path);
    api_req.supported_access_token_types = supported_access_token_types;
    if let Some(body) = body {
        api_req.body = Some(ReqBody::Json(body.clone()));
    }
    let (api_resp, raw) =
        transport::request_typed::<serde_json::Value>(config, &api_req, option).await?;
    let (api_resp, code_error, data) = parse_v2(api_resp, raw);
    Ok(JsonResp {
        api_resp,
        code_error,
        data,
    })
}
