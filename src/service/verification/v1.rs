use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerificationTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateVerificationTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerificationTaskData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<VerificationTask>,
}

impl_resp!(CreateVerificationTaskResp, VerificationTaskData);
impl_resp!(GetVerificationTaskResp, VerificationTaskData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerificationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<serde_json::Value>,
}

impl_resp!(GetVerificationResp, VerificationData);

// ── Resources ──

pub struct VerificationTaskResource<'a> {
    config: &'a Config,
}

impl<'a> VerificationTaskResource<'a> {
    pub async fn create(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        body: &CreateVerificationTaskReqBody,
        option: &RequestOption,
    ) -> Result<CreateVerificationTaskResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/verification/v1/verification_tasks",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id", user_id);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<VerificationTaskData>(self.config, &api_req, option).await?;
        Ok(CreateVerificationTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<GetVerificationTaskResp, LarkError> {
        let path = format!("/open-apis/verification/v1/verification_tasks/{task_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<VerificationTaskData>(self.config, &api_req, option).await?;
        Ok(GetVerificationTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct VerificationResource<'a> {
    config: &'a Config,
}

impl<'a> VerificationResource<'a> {
    pub async fn get(&self, option: &RequestOption) -> Result<GetVerificationResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/verification/v1/verification");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<VerificationData>(self.config, &api_req, option).await?;
        Ok(GetVerificationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct V1<'a> {
    pub verification: VerificationResource<'a>,
    pub verification_task: VerificationTaskResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            verification: VerificationResource { config },
            verification_task: VerificationTaskResource { config },
        }
    }
}
