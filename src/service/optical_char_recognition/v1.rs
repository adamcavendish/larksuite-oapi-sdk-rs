use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

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

// ── Resources ──

pub struct ImageResource<'a> {
    config: &'a Config,
}

impl<'a> ImageResource<'a> {
    pub async fn basic_recognize(
        &self,
        body: &RecognizeBasicImageReqBody,
        option: &RequestOption,
    ) -> Result<BasicRecognizeResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/optical_char_recognition/v1/image/basic_recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<OcrData>(self.config, &api_req, option).await?;
        Ok(BasicRecognizeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
