//! Unified KMS autonomous-key operations.
//!
//! All operations accept tenant access tokens only. The deletion-plan and
//! recovery request formats are intentionally `Serialize`-generic because the
//! platform evolves their policy fields independently of the route contract.

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{JsonResp, PageQuery, RestRequest};

/// A key returned by the autonomous-key APIs.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct AutonomousKey {
    pub key_version_id: Option<String>,
    pub status: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub algorithm_type: Option<String>,
    pub key_usage: Option<String>,
    pub feature_code: Option<String>,
    pub key_alias: Option<String>,
}

/// Body for importing an autonomous key.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct CreateAutonomousKeyReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_encrypted_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_alias: Option<String>,
}

impl CreateAutonomousKeyReqBody {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn encrypted_token(mut self, value: impl Into<String>) -> Self {
        self.encrypted_token = Some(value.into());
        self
    }
    pub fn public_encrypted_key(mut self, value: impl Into<String>) -> Self {
        self.public_encrypted_key = Some(value.into());
        self
    }
    pub fn algorithm_type(mut self, value: impl Into<String>) -> Self {
        self.algorithm_type = Some(value.into());
        self
    }
    pub fn feature_code(mut self, value: impl Into<String>) -> Self {
        self.feature_code = Some(value.into());
        self
    }
    pub fn key_alias(mut self, value: impl Into<String>) -> Self {
        self.key_alias = Some(value.into());
        self
    }
}

/// Query parameters for listing autonomous keys.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListAutonomousKeyQuery<'a> {
    pub page: PageQuery<'a>,
    pub feature_code: Option<&'a str>,
    pub start_time: Option<&'a str>,
    pub end_time: Option<&'a str>,
    pub algorithm_type: Option<&'a str>,
    pub key_alias: Option<&'a str>,
    pub top_class: Option<&'a str>,
    pub operator_id: Option<&'a str>,
}

impl<'a> ListAutonomousKeyQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
    pub fn feature_code(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.feature_code = value.into();
        self
    }
    pub fn start_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_time = value.into();
        self
    }
    pub fn end_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_time = value.into();
        self
    }
    pub fn algorithm_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.algorithm_type = value.into();
        self
    }
    pub fn key_alias(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.key_alias = value.into();
        self
    }
    pub fn top_class(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.top_class = value.into();
        self
    }
    pub fn operator_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_id = value.into();
        self
    }
}

pub struct AutonomousKeyResource<'a> {
    config: &'a Config,
}
pub struct AutonomousKeyDeletionPlanResource<'a> {
    config: &'a Config,
}
pub struct AutonomousKeyRecoverResource<'a> {
    config: &'a Config,
}
pub struct KeyImportMaterialResource<'a> {
    config: &'a Config,
}

impl AutonomousKeyResource<'_> {
    pub async fn create(
        &self,
        body: &CreateAutonomousKeyReqBody,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/unified_kms/v1/autonomous_keys",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_json()
        .await
    }
    pub async fn get(
        &self,
        key_version_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/unified_kms/v1/autonomous_keys/:key_version_id",
            vec![AccessTokenType::Tenant],
            option,
        )
        .path_param("key_version_id", key_version_id)
        .send_json()
        .await
    }
    pub async fn list(
        &self,
        query: &ListAutonomousKeyQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/unified_kms/v1/autonomous_keys",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("feature_code", query.feature_code)
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .query("algorithm_type", query.algorithm_type)
        .query("key_alias", query.key_alias)
        .query("top_class", query.top_class)
        .query("operator_id", query.operator_id)
        .send_json()
        .await
    }
    pub async fn delete(
        &self,
        key_version_id: &str,
        feature_code: Option<&str>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/unified_kms/v1/autonomous_keys/:key_version_id",
            vec![AccessTokenType::Tenant],
            option,
        )
        .path_param("key_version_id", key_version_id)
        .query("feature_code", feature_code)
        .send_json()
        .await
    }
}

impl AutonomousKeyDeletionPlanResource<'_> {
    pub async fn create(
        &self,
        key_version_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/unified_kms/v1/autonomous_keys/:key_version_id/deletion_plan",
            vec![AccessTokenType::Tenant],
            option,
        )
        .path_param("key_version_id", key_version_id)
        .json_body(&body)?
        .send_json()
        .await
    }
    pub async fn delete(
        &self,
        key_version_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/unified_kms/v1/autonomous_keys/:key_version_id/deletion_plan",
            vec![AccessTokenType::Tenant],
            option,
        )
        .path_param("key_version_id", key_version_id)
        .send_json()
        .await
    }
}

impl AutonomousKeyRecoverResource<'_> {
    pub async fn create(
        &self,
        key_version_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/unified_kms/v1/autonomous_keys/:key_version_id/recover",
            vec![AccessTokenType::Tenant],
            option,
        )
        .path_param("key_version_id", key_version_id)
        .json_body(&body)?
        .send_json()
        .await
    }
}

impl KeyImportMaterialResource<'_> {
    pub async fn get(&self, option: &RequestOption) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/unified_kms/v1/key_import_material",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_json()
        .await
    }
}

pub struct V1<'a> {
    pub autonomous_key: AutonomousKeyResource<'a>,
    pub autonomous_key_deletion_plan: AutonomousKeyDeletionPlanResource<'a>,
    pub autonomous_key_recover: AutonomousKeyRecoverResource<'a>,
    pub key_import_material: KeyImportMaterialResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            autonomous_key: AutonomousKeyResource { config },
            autonomous_key_deletion_plan: AutonomousKeyDeletionPlanResource { config },
            autonomous_key_recover: AutonomousKeyRecoverResource { config },
            key_import_material: KeyImportMaterialResource { config },
        }
    }
}
