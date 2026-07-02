use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecognizeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FaceDetectData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_list: Option<Vec<serde_json::Value>>,
}

impl_resp!(DetectFaceResp, FaceDetectData);

#[derive(Debug, Clone, Copy)]
pub struct DetectFaceAttributesQuery<'a> {
    pub body: &'a RecognizeReqBody,
}

impl<'a> DetectFaceAttributesQuery<'a> {
    pub fn new(body: &'a RecognizeReqBody) -> Self {
        Self { body }
    }
}

// ── Resources ──

pub struct ImageResource<'a> {
    config: &'a Config,
}

impl<'a> ImageResource<'a> {
    pub async fn detect_face_attributes(
        &self,
        body: &RecognizeReqBody,
        option: &RequestOption,
    ) -> Result<DetectFaceResp, LarkError> {
        self.detect_face_attributes_by_query(&DetectFaceAttributesQuery::new(body), option)
            .await
    }

    pub async fn detect_face_attributes_by_query(
        &self,
        query: &DetectFaceAttributesQuery<'_>,
        option: &RequestOption,
    ) -> Result<DetectFaceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/face_detection/v1/image/detect_face_attributes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<FaceDetectData, DetectFaceResp>()
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
