use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkplaceAccessData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_workplace_access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_workplace_access: Option<Vec<PerWorkplaceAccess>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerWorkplaceAccess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workplace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workplace_name: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_user_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockAccessData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_user_count: Option<i64>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkplaceAccessListData {
    #[serde(default)]
    pub items: Vec<WorkplaceAccessData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockAccessListData {
    #[serde(default)]
    pub items: Vec<BlockAccessData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(GetWorkplaceAccessResp, WorkplaceAccessListData);
impl_resp!(GetBlockAccessResp, BlockAccessListData);

// ── Resources ──

pub struct WorkplaceAccessDataResource<'a> {
    config: &'a Config,
}

impl<'a> WorkplaceAccessDataResource<'a> {
    pub async fn search(
        &self,
        from_date: &str,
        to_date: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetWorkplaceAccessResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/workplace/v1/workplace_access_data/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("from_date", from_date);
        api_req.query_params.set("to_date", to_date);
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<WorkplaceAccessListData>(self.config, &api_req, option)
                .await?;
        Ok(GetWorkplaceAccessResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        from_date: &str,
        to_date: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetWorkplaceAccessResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/workplace/v1/workplace_access_data/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("from_date", from_date);
        api_req.query_params.set("to_date", to_date);
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<WorkplaceAccessListData>(self.config, &api_req, option)
                .await?;
        Ok(GetWorkplaceAccessResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CustomWorkplaceAccessDataResource<'a> {
    config: &'a Config,
}

impl<'a> CustomWorkplaceAccessDataResource<'a> {
    pub async fn search(
        &self,
        from_date: &str,
        to_date: &str,
        workplace_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/workplace/v1/custom_workplace_access_data/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("from_date", from_date);
        api_req.query_params.set("to_date", to_date);
        if let Some(v) = workplace_id {
            api_req.query_params.set("workplace_id", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<BlockAccessListData>(self.config, &api_req, option).await?;
        Ok(GetBlockAccessResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        from_date: &str,
        to_date: &str,
        workplace_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/workplace/v1/custom_workplace_access_data/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("from_date", from_date);
        api_req.query_params.set("to_date", to_date);
        if let Some(v) = workplace_id {
            api_req.query_params.set("workplace_id", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<BlockAccessListData>(self.config, &api_req, option).await?;
        Ok(GetBlockAccessResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct WorkplaceBlockAccessDataResource<'a> {
    config: &'a Config,
}

impl<'a> WorkplaceBlockAccessDataResource<'a> {
    pub async fn search(
        &self,
        from_date: &str,
        to_date: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        block_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/workplace/v1/workplace_block_access_data/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("from_date", from_date);
        api_req.query_params.set("to_date", to_date);
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = block_id {
            api_req.query_params.set("block_id", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<BlockAccessListData>(self.config, &api_req, option).await?;
        Ok(GetBlockAccessResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct V1<'a> {
    pub workplace_access_data: WorkplaceAccessDataResource<'a>,
    pub custom_workplace_access_data: CustomWorkplaceAccessDataResource<'a>,
    pub workplace_block_access_data: WorkplaceBlockAccessDataResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            workplace_access_data: WorkplaceAccessDataResource { config },
            custom_workplace_access_data: CustomWorkplaceAccessDataResource { config },
            workplace_block_access_data: WorkplaceBlockAccessDataResource { config },
        }
    }
}
