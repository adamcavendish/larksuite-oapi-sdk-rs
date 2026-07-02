use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

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

#[derive(Debug, Clone, Copy)]
pub struct CreateIdentityQuery<'a> {
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub body: &'a CreateIdentityReqBody,
}

impl<'a> CreateIdentityQuery<'a> {
    pub fn new(user_id: &'a str, body: &'a CreateIdentityReqBody) -> Self {
        Self {
            user_id,
            user_id_type: None,
            body,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

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
        self.create_by_query(
            &CreateIdentityQuery::new(user_id, body).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateIdentityQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateIdentityResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/human_authentication/v1/identities",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<IdentityData>()
        .await?;
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
