use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Avatar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "avatar_72")]
    pub avatar_72: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "avatar_240")]
    pub avatar_240: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "avatar_640")]
    pub avatar_640: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductI18nName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tenant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_tag: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Avatar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantAssignInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_plan_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<ProductI18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_seats: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_seats: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

// ── Response types ──

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QueryTenantRespData {
    pub tenant: Option<Tenant>,
}

#[derive(Debug, Clone)]
pub struct QueryTenantResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
    pub data: Option<QueryTenantRespData>,
}

impl QueryTenantResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QueryTenantProductAssignInfoRespData {
    pub assign_info_list: Option<Vec<TenantAssignInfo>>,
}

#[derive(Debug, Clone)]
pub struct QueryTenantProductAssignInfoResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
    pub data: Option<QueryTenantProductAssignInfoRespData>,
}

impl QueryTenantProductAssignInfoResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

// ── Resources ──

pub struct TenantResource<'a> {
    config: &'a Config,
}

impl<'a> TenantResource<'a> {
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTenantResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/tenant/v2/tenant/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let (api_resp, raw) =
            transport::request_typed::<QueryTenantRespData>(self.config, &api_req, option).await?;

        Ok(QueryTenantResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct TenantProductAssignInfoResource<'a> {
    config: &'a Config,
}

impl<'a> TenantProductAssignInfoResource<'a> {
    pub async fn query(
        &self,
        option: &RequestOption,
    ) -> Result<QueryTenantProductAssignInfoResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/tenant/v2/tenant/assign_info_list/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let (api_resp, raw) = transport::request_typed::<QueryTenantProductAssignInfoRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;

        Ok(QueryTenantProductAssignInfoResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V2<'a> {
    pub tenant: TenantResource<'a>,
    pub product_assign_info: TenantProductAssignInfoResource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            tenant: TenantResource { config },
            product_assign_info: TenantProductAssignInfoResource { config },
        }
    }
}
