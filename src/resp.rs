use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::constants::{
    HTTP_HEADER_KEY_LOG_ID, HTTP_HEADER_KEY_REQUEST_ID, HTTP_HEADER_REQUEST_ID,
};

#[derive(Debug, Clone)]
pub struct ApiResp {
    pub status_code: u16,
    pub header: HeaderMap,
    pub raw_body: Vec<u8>,
}

impl ApiResp {
    pub fn request_id(&self) -> Option<&str> {
        self.header
            .get(HTTP_HEADER_KEY_REQUEST_ID)
            .or_else(|| self.header.get(HTTP_HEADER_REQUEST_ID))
            .and_then(|v| v.to_str().ok())
    }

    pub fn log_id(&self) -> Option<&str> {
        self.header
            .get(HTTP_HEADER_KEY_LOG_ID)
            .and_then(|v| v.to_str().ok())
    }

    pub fn file_name_by_header(&self) -> Option<String> {
        let disposition = self.header.get("Content-Disposition")?;
        let val = disposition.to_str().ok()?;
        for part in val.split(';') {
            let part = part.trim();
            if let Some(rest) = part.strip_prefix("filename=") {
                let name = rest.trim_matches('"');
                return Some(name.to_string());
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeError {
    pub code: i64,
    #[serde(default)]
    pub msg: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CodeErrorInfo>,
}

impl CodeError {
    pub fn success(&self) -> bool {
        self.code == 0
    }
}

impl std::fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)?;
        if let Some(ref err) = self.error
            && let Some(ref log_id) = err.log_id
        {
            write!(f, ", log_id: {log_id}")?;
        }
        Ok(())
    }
}

impl std::error::Error for CodeError {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub troubleshooter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<CodeErrorDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_violations: Option<Vec<CodeErrorPermissionViolation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_violations: Option<Vec<CodeErrorFieldViolation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helps: Option<Vec<CodeErrorHelp>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeErrorPermissionViolation {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub violation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeErrorFieldViolation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeErrorHelp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct RawResponse<T> {
    #[serde(flatten)]
    pub code_error: CodeError,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
