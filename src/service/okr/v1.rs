use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::service::common::parse_v2;
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

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: Option<CodeError>,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.as_ref().is_none_or(|e| e.code == 0)
    }
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

impl_resp_v2!(UploadImageResp, serde_json::Value);
impl_resp_v2!(BatchGetOkrResp, serde_json::Value);
impl_resp_v2!(CreatePeriodResp, serde_json::Value);
impl_resp_v2!(PatchPeriodResp, serde_json::Value);
impl_resp_v2!(ListPeriodRuleResp, serde_json::Value);
impl_resp_v2!(CreateProgressRecordResp, serde_json::Value);
impl_resp_v2!(GetProgressRecordResp, serde_json::Value);
impl_resp_v2!(UpdateProgressRecordResp, serde_json::Value);
impl_resp_v2!(QueryReviewResp, serde_json::Value);

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
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
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

    pub async fn create(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreatePeriodResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/okr/v1/periods");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreatePeriodResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        period_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPeriodResp> {
        let path = format!("/open-apis/okr/v1/periods/{period_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchPeriodResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── PeriodRule resource ──

pub struct PeriodRuleResource<'a> {
    config: &'a Config,
}

impl PeriodRuleResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListPeriodRuleResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/okr/v1/period_rules");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListPeriodRuleResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Image resource ──

pub struct ImageResource<'a> {
    config: &'a Config,
}

impl ImageResource<'_> {
    pub async fn upload(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadImageResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/okr/v1/images/upload");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UploadImageResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Okr resource ──

pub struct OkrResource<'a> {
    config: &'a Config,
}

impl OkrResource<'_> {
    pub async fn batch_get(
        &self,
        okr_ids: &[&str],
        user_id_type: Option<&str>,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchGetOkrResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/okr/v1/okrs/batch_get");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        for id in okr_ids {
            api_req.query_params.add("okr_ids", *id);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchGetOkrResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ProgressRecord resource ──

pub struct ProgressRecordResource<'a> {
    config: &'a Config,
}

impl ProgressRecordResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateProgressRecordResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/okr/v1/progress_records");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateProgressRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(&self, progress_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/okr/v1/progress_records/{progress_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, _) = parse_v2(api_resp, raw);
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn get(
        &self,
        progress_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetProgressRecordResp> {
        let path = format!("/open-apis/okr/v1/progress_records/{progress_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetProgressRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        progress_id: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateProgressRecordResp> {
        let path = format!("/open-apis/okr/v1/progress_records/{progress_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateProgressRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Review resource ──

pub struct ReviewResource<'a> {
    config: &'a Config,
}

impl ReviewResource<'_> {
    pub async fn query(
        &self,
        user_ids: &[&str],
        period_ids: Option<&[&str]>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryReviewResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/okr/v1/reviews/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        for id in user_ids {
            api_req.query_params.add("user_ids", *id);
        }
        if let Some(ids) = period_ids {
            for id in ids {
                api_req.query_params.add("period_ids", *id);
            }
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryReviewResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct UserOkrResource<'a> {
    config: &'a Config,
}

impl<'a> UserOkrResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        period_ids: Option<Vec<&str>>,
        offset: Option<&str>,
        limit: Option<&str>,
        lang: Option<&str>,
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
        if let Some(v) = offset {
            api_req.query_params.set("offset", v);
        }
        if let Some(v) = limit {
            api_req.query_params.set("limit", v);
        }
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
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
    pub period_rule: PeriodRuleResource<'a>,
    pub image: ImageResource<'a>,
    pub okr: OkrResource<'a>,
    pub progress_record: ProgressRecordResource<'a>,
    pub review: ReviewResource<'a>,
    pub user_okr: UserOkrResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            period: PeriodResource { config },
            period_rule: PeriodRuleResource { config },
            image: ImageResource { config },
            okr: OkrResource { config },
            progress_record: ProgressRecordResource { config },
            review: ReviewResource { config },
            user_okr: UserOkrResource { config },
        }
    }
}
