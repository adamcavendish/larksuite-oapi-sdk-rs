use std::collections::HashMap;

use http::HeaderMap;
use serde::Serialize;

use crate::AccessTokenType;

#[derive(Debug, Clone)]
pub struct ApiReq {
    pub http_method: http::Method,
    pub api_path: String,
    pub body: Option<ReqBody>,
    pub query_params: QueryParams,
    pub path_params: PathParams,
    pub supported_access_token_types: Vec<AccessTokenType>,
}

impl ApiReq {
    pub fn new(method: http::Method, path: impl Into<String>) -> Self {
        Self {
            http_method: method,
            api_path: path.into(),
            body: None,
            query_params: QueryParams::new(),
            path_params: PathParams::new(),
            supported_access_token_types: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReqBody {
    Json(serde_json::Value),
    FormData(Vec<FormDataField>),
}

impl ReqBody {
    pub fn json<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        let v = serde_json::to_value(value)?;
        Ok(Self::Json(v))
    }
}

#[derive(Debug, Clone)]
pub struct FormDataField {
    pub name: String,
    pub value: FormDataValue,
}

#[derive(Debug, Clone)]
pub enum FormDataValue {
    Text(String),
    File {
        filename: String,
        data: Vec<u8>,
        content_type: Option<String>,
    },
}

#[derive(Debug, Clone, Default)]
pub struct QueryParams(pub HashMap<String, Vec<String>>);

impl QueryParams {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.first().map(|s| s.as_str()))
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), vec![value.into()]);
    }

    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.entry(key.into()).or_default().push(value.into());
    }

    pub fn encode(&self) -> String {
        let mut pairs: Vec<(&str, &str)> = Vec::new();
        for (key, values) in &self.0 {
            for value in values {
                pairs.push((key.as_str(), value.as_str()));
            }
        }
        pairs.sort_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(b.1)));
        url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(pairs)
            .finish()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PathParams(pub HashMap<String, String>);

impl PathParams {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }
}

/// Builder for constructing multipart form-data request bodies.
///
/// Matches the Go SDK's `larkcore.NewFormdata()` fluent API.
///
/// ```rust
/// use larksuite_oapi_sdk_rs::FormDataBuilder;
///
/// let body = FormDataBuilder::new()
///     .field("name", "document.pdf")
///     .file("file", "doc.pdf", b"contents".to_vec(), Some("application/pdf"))
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct FormDataBuilder {
    fields: Vec<FormDataField>,
}

impl FormDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push(FormDataField {
            name: name.into(),
            value: FormDataValue::Text(value.into()),
        });
        self
    }

    pub fn file(
        mut self,
        name: impl Into<String>,
        filename: impl Into<String>,
        data: Vec<u8>,
        content_type: Option<impl Into<String>>,
    ) -> Self {
        self.fields.push(FormDataField {
            name: name.into(),
            value: FormDataValue::File {
                filename: filename.into(),
                data,
                content_type: content_type.map(|s| s.into()),
            },
        });
        self
    }

    pub fn build(self) -> Vec<FormDataField> {
        self.fields
    }

    pub fn into_body(self) -> ReqBody {
        ReqBody::FormData(self.fields)
    }
}

/// Per-request options that override client-level defaults.
#[derive(Debug, Clone, Default)]
pub struct RequestOption {
    /// Tenant key for marketplace multi-tenant requests.
    pub tenant_key: Option<String>,
    /// Explicit user access token (skips automatic token resolution).
    pub user_access_token: Option<String>,
    /// Explicit app access token (skips automatic token resolution).
    pub app_access_token: Option<String>,
    /// Explicit tenant access token (skips automatic token resolution).
    pub tenant_access_token: Option<String>,
    /// Attach helpdesk authorization header.
    pub need_helpdesk_auth: bool,
    /// Custom request ID for tracing.
    pub request_id: Option<String>,
    /// App ticket for marketplace app token exchange.
    pub app_ticket: Option<String>,
    /// Send body as multipart/form-data.
    pub file_upload: bool,
    /// Expect a binary file response instead of JSON.
    pub file_download: bool,
    /// Extra headers merged into this request.
    pub headers: Option<HeaderMap>,
}
