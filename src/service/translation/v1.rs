use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranslationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct TranslateReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glossary: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DetectLanguageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranslateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetectData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl_resp!(TranslateResp, TranslateData);
impl_resp!(DetectLanguageResp, DetectData);

// ── Resources ──

pub struct TextResource<'a> {
    config: &'a Config,
}

impl<'a> TextResource<'a> {
    pub async fn translate(
        &self,
        body: &TranslateReqBody,
        option: &RequestOption,
    ) -> Result<TranslateResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/translation/v1/text/translate",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TranslateData>(self.config, &api_req, option).await?;
        Ok(TranslateResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn detect_language(
        &self,
        body: &DetectLanguageReqBody,
        option: &RequestOption,
    ) -> Result<DetectLanguageResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/translation/v1/text/detect");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<DetectData>(self.config, &api_req, option).await?;
        Ok(DetectLanguageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub text: TextResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            text: TextResource { config },
        }
    }
}
