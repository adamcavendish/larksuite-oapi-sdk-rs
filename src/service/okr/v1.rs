use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_month: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_month: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_list: Option<Vec<OkrObjective>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrObjective {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kr_list: Option<Vec<OkrKeyResult>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aligned_objective_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aligning_objective_list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrKeyResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kr_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<serde_json::Value>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeriodListData {
    #[serde(default)]
    pub items: Vec<OkrPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrListData {
    #[serde(default)]
    pub okr_list: Vec<OkrItem>,
}

impl_resp!(ListPeriodResp, PeriodListData);
impl_resp!(GetUserOkrResp, OkrListData);

// ── Resources ──

pub struct PeriodResource<'a> {
    config: &'a Config,
}

impl<'a> PeriodResource<'a> {
    pub async fn list(
        &self,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListPeriodResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/okr/v1/periods");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<PeriodListData>(self.config, &api_req, option).await?;
        Ok(ListPeriodResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct UserOkrResource<'a> {
    config: &'a Config,
}

impl<'a> UserOkrResource<'a> {
    pub async fn get(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        period_ids: Option<Vec<&str>>,
        option: &RequestOption,
    ) -> Result<GetUserOkrResp> {
        let path = format!("/open-apis/okr/v1/users/{user_id}/okrs");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(ids) = period_ids {
            for id in ids {
                api_req.query_params.add("period_ids", id);
            }
        }
        let (api_resp, raw) =
            transport::request_typed::<OkrListData>(self.config, &api_req, option).await?;
        Ok(GetUserOkrResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub period: PeriodResource<'a>,
    pub user_okr: UserOkrResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            period: PeriodResource { config },
            user_okr: UserOkrResource { config },
        }
    }
}
