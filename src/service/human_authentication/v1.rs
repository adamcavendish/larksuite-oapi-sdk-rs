use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::transport;

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateIdentityReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdentityData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify_uid: Option<String>,
}

impl_resp!(CreateIdentityResp, IdentityData);

// ── Resources ──

pub struct IdentityResource<'a> {
    config: &'a Config,
}

impl<'a> IdentityResource<'a> {
    pub async fn create(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        body: &CreateIdentityReqBody,
        option: &RequestOption,
    ) -> Result<CreateIdentityResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/human_authentication/v1/identities",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id", user_id);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<IdentityData>(self.config, &api_req, option).await?;
        Ok(CreateIdentityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub identity: IdentityResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            identity: IdentityResource { config },
        }
    }
}
