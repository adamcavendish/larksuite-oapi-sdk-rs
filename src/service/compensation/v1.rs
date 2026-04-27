use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::parse_v2;
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompensationPlan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanListData {
    #[serde(default)]
    pub items: Vec<CompensationPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListPlanResp, PlanListData);

// ── Helpers for v2-style resources (Option<CodeError> pattern) ──

impl_resp_v2!(CreateArchiveResp, serde_json::Value);
impl_resp_v2!(QueryArchiveResp, serde_json::Value);
impl_resp_v2!(ListChangeReasonResp, serde_json::Value);
impl_resp_v2!(ListIndicatorResp, serde_json::Value);
impl_resp_v2!(ListItemResp, serde_json::Value);
impl_resp_v2!(ListItemCategoryResp, serde_json::Value);
impl_resp_v2!(BatchCreateLumpSumPaymentResp, serde_json::Value);
impl_resp_v2!(BatchRemoveLumpSumPaymentResp, serde_json::Value);
impl_resp_v2!(BatchUpdateLumpSumPaymentResp, serde_json::Value);
impl_resp_v2!(QueryLumpSumPaymentResp, serde_json::Value);
impl_resp_v2!(QueryDetailLumpSumPaymentResp, serde_json::Value);
impl_resp_v2!(BatchCreateRecurringPaymentResp, serde_json::Value);
impl_resp_v2!(BatchRemoveRecurringPaymentResp, serde_json::Value);
impl_resp_v2!(BatchUpdateRecurringPaymentResp, serde_json::Value);
impl_resp_v2!(QueryRecurringPaymentResp, serde_json::Value);
impl_resp_v2!(QuerySocialArchiveResp, serde_json::Value);
impl_resp_v2!(QuerySocialArchiveAdjustRecordResp, serde_json::Value);
impl_resp_v2!(ListSocialInsuranceResp, serde_json::Value);
impl_resp_v2!(ListSocialPlanResp, serde_json::Value);
impl_resp_v2!(QuerySocialPlanResp, serde_json::Value);

// ── Resources ──

pub struct PlanResource<'a> {
    config: &'a Config,
}

impl<'a> PlanResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPlanResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/compensation/v1/plans");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<PlanListData>(self.config, &api_req, option).await?;
        Ok(ListPlanResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Archive resource ──

pub struct ArchiveResource<'a> {
    config: &'a Config,
}

impl ArchiveResource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateArchiveResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/compensation/v1/archives");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateArchiveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryArchiveResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/archives/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryArchiveResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ChangeReason resource ──

pub struct ChangeReasonResource<'a> {
    config: &'a Config,
}

impl ChangeReasonResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListChangeReasonResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/compensation/v1/change_reasons",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListChangeReasonResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Indicator resource ──

pub struct IndicatorResource<'a> {
    config: &'a Config,
}

impl IndicatorResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListIndicatorResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/compensation/v1/indicators");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListIndicatorResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Item resource ──

pub struct ItemResource<'a> {
    config: &'a Config,
}

impl ItemResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListItemResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/compensation/v1/items");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListItemResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ItemCategory resource ──

pub struct ItemCategoryResource<'a> {
    config: &'a Config,
}

impl ItemCategoryResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListItemCategoryResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/compensation/v1/item_categories",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListItemCategoryResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── LumpSumPayment resource ──

pub struct LumpSumPaymentResource<'a> {
    config: &'a Config,
}

impl LumpSumPaymentResource<'_> {
    pub async fn batch_create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchCreateLumpSumPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/batch_create",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateLumpSumPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_remove(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchRemoveLumpSumPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/batch_remove",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchRemoveLumpSumPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateLumpSumPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/batch_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchUpdateLumpSumPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryLumpSumPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryLumpSumPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query_detail(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryDetailLumpSumPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/query_detail",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryDetailLumpSumPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── RecurringPayment resource ──

pub struct RecurringPaymentResource<'a> {
    config: &'a Config,
}

impl RecurringPaymentResource<'_> {
    pub async fn batch_create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchCreateRecurringPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/batch_create",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateRecurringPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_remove(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchRemoveRecurringPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/batch_remove",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchRemoveRecurringPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateRecurringPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/batch_update",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchUpdateRecurringPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryRecurringPaymentResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryRecurringPaymentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── SocialArchive resource ──

pub struct SocialArchiveResource<'a> {
    config: &'a Config,
}

impl SocialArchiveResource<'_> {
    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QuerySocialArchiveResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/social_archive/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QuerySocialArchiveResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── SocialArchiveAdjustRecord resource ──

pub struct SocialArchiveAdjustRecordResource<'a> {
    config: &'a Config,
}

impl SocialArchiveAdjustRecordResource<'_> {
    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QuerySocialArchiveAdjustRecordResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/social_archive_adjust_record/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QuerySocialArchiveAdjustRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── SocialInsurance resource ──

pub struct SocialInsuranceResource<'a> {
    config: &'a Config,
}

impl SocialInsuranceResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListSocialInsuranceResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/compensation/v1/social_insurances",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListSocialInsuranceResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── SocialPlan resource ──

pub struct SocialPlanResource<'a> {
    config: &'a Config,
}

impl SocialPlanResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSocialPlanResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/compensation/v1/social_plans");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListSocialPlanResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QuerySocialPlanResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/compensation/v1/social_plans/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QuerySocialPlanResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub plan: PlanResource<'a>,
    pub archive: ArchiveResource<'a>,
    pub change_reason: ChangeReasonResource<'a>,
    pub indicator: IndicatorResource<'a>,
    pub item: ItemResource<'a>,
    pub item_category: ItemCategoryResource<'a>,
    pub lump_sum_payment: LumpSumPaymentResource<'a>,
    pub recurring_payment: RecurringPaymentResource<'a>,
    pub social_archive: SocialArchiveResource<'a>,
    pub social_archive_adjust_record: SocialArchiveAdjustRecordResource<'a>,
    pub social_insurance: SocialInsuranceResource<'a>,
    pub social_plan: SocialPlanResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            plan: PlanResource { config },
            archive: ArchiveResource { config },
            change_reason: ChangeReasonResource { config },
            indicator: IndicatorResource { config },
            item: ItemResource { config },
            item_category: ItemCategoryResource { config },
            lump_sum_payment: LumpSumPaymentResource { config },
            recurring_payment: RecurringPaymentResource { config },
            social_archive: SocialArchiveResource { config },
            social_archive_adjust_record: SocialArchiveAdjustRecordResource { config },
            social_insurance: SocialInsuranceResource { config },
            social_plan: SocialPlanResource { config },
        }
    }
}
