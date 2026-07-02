use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/compensation/v1/plans",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_response::<PlanListData, ListPlanResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/archives",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, CreateArchiveResp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryArchiveResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/archives/query",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QueryArchiveResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/compensation/v1/change_reasons",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<serde_json::Value, ListChangeReasonResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/compensation/v1/indicators",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<serde_json::Value, ListIndicatorResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/compensation/v1/items",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<serde_json::Value, ListItemResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/compensation/v1/item_categories",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<serde_json::Value, ListItemCategoryResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/batch_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchCreateLumpSumPaymentResp>()
        .await
    }

    pub async fn batch_remove(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchRemoveLumpSumPaymentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/batch_remove",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchRemoveLumpSumPaymentResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateLumpSumPaymentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchUpdateLumpSumPaymentResp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryLumpSumPaymentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QueryLumpSumPaymentResp>()
        .await
    }

    pub async fn query_detail(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryDetailLumpSumPaymentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/lump_sum_payment/query_detail",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QueryDetailLumpSumPaymentResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/batch_create",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchCreateRecurringPaymentResp>()
        .await
    }

    pub async fn batch_remove(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchRemoveRecurringPaymentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/batch_remove",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchRemoveRecurringPaymentResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateRecurringPaymentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/batch_update",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, BatchUpdateRecurringPaymentResp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryRecurringPaymentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/recurring_payment/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QueryRecurringPaymentResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/social_archive/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QuerySocialArchiveResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/social_archive_adjust_record/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QuerySocialArchiveAdjustRecordResp>()
        .await
    }
}

// ── SocialInsurance resource ──

pub struct SocialInsuranceResource<'a> {
    config: &'a Config,
}

impl SocialInsuranceResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListSocialInsuranceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/compensation/v1/social_insurances",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<serde_json::Value, ListSocialInsuranceResp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/compensation/v1/social_plans",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<serde_json::Value, ListSocialPlanResp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QuerySocialPlanResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/compensation/v1/social_plans/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_v2_response::<serde_json::Value, QuerySocialPlanResp>()
        .await
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
