use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecognizeBasicSpeechReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecognizeSpeechStreamReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecognizeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recognition_text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamRecognizeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recognition_text: Option<String>,
}

impl_resp!(RecognizeBasicSpeechResp, RecognizeData);
impl_resp!(RecognizeSpeechStreamResp, StreamRecognizeData);

// ── Resources ──

pub struct SpeechResource<'a> {
    config: &'a Config,
}

impl<'a> SpeechResource<'a> {
    pub async fn file_recognize(
        &self,
        body: &RecognizeBasicSpeechReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeBasicSpeechResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/speech_to_text/v1/speech/file_recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RecognizeData>(self.config, &api_req, option).await?;
        Ok(RecognizeBasicSpeechResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn stream_recognize(
        &self,
        body: &RecognizeSpeechStreamReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeSpeechStreamResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/speech_to_text/v1/speech/stream_recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<StreamRecognizeData>(self.config, &api_req, option).await?;
        Ok(RecognizeSpeechStreamResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub speech: SpeechResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            speech: SpeechResource { config },
        }
    }
}
