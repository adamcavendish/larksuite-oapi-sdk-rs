pub use crate::config::Config;
pub use crate::constants::AccessTokenType;
pub use crate::error::Result;
pub use crate::req::{ApiReq, ReqBody, RequestOption};
pub use crate::resp::{ApiResp, CodeError, RawResponse};

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
