pub use crate::config::Config;
pub use crate::constants::AccessTokenType;
use crate::error::LarkError;
pub use crate::req::{ApiReq, ReqBody, RequestOption};
pub use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::transport;

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
