use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

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

#[derive(Debug, Clone, Copy)]
pub struct FileRecognizeSpeechQuery<'a> {
    pub body: &'a RecognizeBasicSpeechReqBody,
}

impl<'a> FileRecognizeSpeechQuery<'a> {
    pub fn new(body: &'a RecognizeBasicSpeechReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StreamRecognizeSpeechQuery<'a> {
    pub body: &'a RecognizeSpeechStreamReqBody,
}

impl<'a> StreamRecognizeSpeechQuery<'a> {
    pub fn new(body: &'a RecognizeSpeechStreamReqBody) -> Self {
        Self { body }
    }
}

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
        self.file_recognize_by_query(&FileRecognizeSpeechQuery::new(body), option)
            .await
    }

    pub async fn file_recognize_by_query(
        &self,
        query: &FileRecognizeSpeechQuery<'_>,
        option: &RequestOption,
    ) -> Result<RecognizeBasicSpeechResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/speech_to_text/v1/speech/file_recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send::<RecognizeData>()
        .await?;
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
        self.stream_recognize_by_query(&StreamRecognizeSpeechQuery::new(body), option)
            .await
    }

    pub async fn stream_recognize_by_query(
        &self,
        query: &StreamRecognizeSpeechQuery<'_>,
        option: &RequestOption,
    ) -> Result<RecognizeSpeechStreamResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/speech_to_text/v1/speech/stream_recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send::<StreamRecognizeData>()
        .await?;
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
