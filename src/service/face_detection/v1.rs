use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

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
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/face_detection/v1/image/detect_face_attributes",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FaceDetectData>(self.config, &api_req, option).await?;
        Ok(DetectFaceResp {
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
