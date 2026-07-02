use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecognizeBasicImageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OcrData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recognition_text: Option<String>,
}

impl_resp!(BasicRecognizeResp, OcrData);

#[derive(Debug, Clone, Copy)]
pub struct BasicRecognizeImageQuery<'a> {
    pub body: &'a RecognizeBasicImageReqBody,
}

impl<'a> BasicRecognizeImageQuery<'a> {
    pub fn new(body: &'a RecognizeBasicImageReqBody) -> Self {
        Self { body }
    }
}

// ── Resources ──

pub struct ImageResource<'a> {
    config: &'a Config,
}

impl<'a> ImageResource<'a> {
    pub async fn basic_recognize(
        &self,
        body: &RecognizeBasicImageReqBody,
        option: &RequestOption,
    ) -> Result<BasicRecognizeResp, LarkError> {
        self.basic_recognize_by_query(&BasicRecognizeImageQuery::new(body), option)
            .await
    }

    pub async fn basic_recognize_by_query(
        &self,
        query: &BasicRecognizeImageQuery<'_>,
        option: &RequestOption,
    ) -> Result<BasicRecognizeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/optical_char_recognition/v1/image/basic_recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<OcrData, BasicRecognizeResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub image: ImageResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            image: ImageResource { config },
        }
    }
}
