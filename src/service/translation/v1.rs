use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
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
    pub glossary: Option<Vec<crate::JsonValue>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DetectLanguageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TranslateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DetectData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl_resp!(TranslateResp, TranslateData);
impl_resp!(DetectLanguageResp, DetectData);

#[derive(Debug, Clone, Copy)]
pub struct TranslateTextQuery<'a> {
    pub body: &'a TranslateReqBody,
}

impl<'a> TranslateTextQuery<'a> {
    pub fn new(body: &'a TranslateReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DetectLanguageQuery<'a> {
    pub body: &'a DetectLanguageReqBody,
}

impl<'a> DetectLanguageQuery<'a> {
    pub fn new(body: &'a DetectLanguageReqBody) -> Self {
        Self { body }
    }
}

// ── Resources ──

pub struct TextResource<'a> {
    config: &'a Config,
}

impl<'a> TextResource<'a> {
    pub async fn translate(
        &self,
        body: &TranslateReqBody,
        option: &RequestOption,
    ) -> Result<TranslateResp, LarkError> {
        self.translate_by_query(&TranslateTextQuery::new(body), option)
            .await
    }

    pub async fn translate_by_query(
        &self,
        query: &TranslateTextQuery<'_>,
        option: &RequestOption,
    ) -> Result<TranslateResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/translation/v1/text/translate",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<TranslateData, TranslateResp>()
        .await
    }

    pub async fn detect_language(
        &self,
        body: &DetectLanguageReqBody,
        option: &RequestOption,
    ) -> Result<DetectLanguageResp, LarkError> {
        self.detect_language_by_query(&DetectLanguageQuery::new(body), option)
            .await
    }

    pub async fn detect_language_by_query(
        &self,
        query: &DetectLanguageQuery<'_>,
        option: &RequestOption,
    ) -> Result<DetectLanguageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/translation/v1/text/detect",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<DetectData, DetectLanguageResp>()
        .await
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
