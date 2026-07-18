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

impl_resp_v2!(CreateArchiveResp, CreateArchiveRespData);
impl_resp_v2!(QueryArchiveResp, QueryArchiveRespData);
impl_resp_v2!(ListChangeReasonResp, ListChangeReasonRespData);
impl_resp_v2!(ListIndicatorResp, ListIndicatorRespData);
impl_resp_v2!(ListItemResp, ListItemRespData);
impl_resp_v2!(ListItemCategoryResp, ListItemCategoryRespData);
impl_resp_v2!(
    BatchCreateLumpSumPaymentResp,
    BatchCreateLumpSumPaymentRespData
);
impl_resp_v2!(
    BatchRemoveLumpSumPaymentResp,
    BatchRemoveLumpSumPaymentRespData
);
impl_resp_v2!(
    BatchUpdateLumpSumPaymentResp,
    BatchUpdateLumpSumPaymentRespData
);
impl_resp_v2!(QueryLumpSumPaymentResp, QueryLumpSumPaymentRespData);
impl_resp_v2!(
    QueryDetailLumpSumPaymentResp,
    QueryDetailLumpSumPaymentRespData
);
impl_resp_v2!(
    BatchCreateRecurringPaymentResp,
    BatchCreateRecurringPaymentRespData
);
impl_resp_v2!(
    BatchRemoveRecurringPaymentResp,
    BatchRemoveRecurringPaymentRespData
);
impl_resp_v2!(
    BatchUpdateRecurringPaymentResp,
    BatchUpdateRecurringPaymentRespData
);
impl_resp_v2!(QueryRecurringPaymentResp, QueryRecurringPaymentRespData);
impl_resp_v2!(QuerySocialArchiveResp, QuerySocialArchiveRespData);
impl_resp_v2!(
    QuerySocialArchiveAdjustRecordResp,
    QuerySocialArchiveAdjustRecordRespData
);
impl_resp_v2!(ListSocialInsuranceResp, ListSocialInsuranceRespData);
impl_resp_v2!(ListSocialPlanResp, ListSocialPlanRespData);
impl_resp_v2!(QuerySocialPlanResp, QuerySocialPlanRespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateArchiveRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_tid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryArchiveRespData {
    #[serde(default)]
    pub items: Vec<ArchiveDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChangeReasonRespData {
    #[serde(default)]
    pub items: Vec<ChangeReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIndicatorRespData {
    #[serde(default)]
    pub items: Vec<Indicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListItemRespData {
    #[serde(default)]
    pub items: Vec<Item>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListItemCategoryRespData {
    #[serde(default)]
    pub items: Vec<ItemCategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateLumpSumPaymentRespData {
    #[serde(default)]
    pub operate_results: Vec<LumpSumPaymentOperateResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchRemoveLumpSumPaymentRespData {
    #[serde(default)]
    pub operate_results: Vec<LumpSumPaymentOperateResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateLumpSumPaymentRespData {
    #[serde(default)]
    pub operate_results: Vec<LumpSumPaymentOperateResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryLumpSumPaymentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub records: Vec<LumpSumPayment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryDetailLumpSumPaymentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub records: Vec<LumpSumPaymentDetail>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateRecurringPaymentRespData {
    #[serde(default)]
    pub operate_results: Vec<RecurringPaymentOperateResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchRemoveRecurringPaymentRespData {
    #[serde(default)]
    pub operate_results: Vec<RecurringPaymentOperateResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateRecurringPaymentRespData {
    #[serde(default)]
    pub operate_results: Vec<RecurringPaymentOperateResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryRecurringPaymentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub records: Vec<RecurringPayment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuerySocialArchiveRespData {
    #[serde(default)]
    pub archives: Vec<SocialArchive>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuerySocialArchiveAdjustRecordRespData {
    #[serde(default)]
    pub records: Vec<SocialArchiveAdjustRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSocialInsuranceRespData {
    #[serde(default)]
    pub items: Vec<SocialInsurance>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSocialPlanRespData {
    #[serde(default)]
    pub plans: Vec<SocialPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuerySocialPlanRespData {
    #[serde(default)]
    pub plans: Vec<SocialPlan>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_reason_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salary_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(default)]
    pub archive_items: Vec<ArchiveItem>,
    #[serde(default)]
    pub archive_indicators: Vec<ArchiveIndicator>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveIndicator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_result_regular: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_result_regular: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default)]
    pub i18n_names: Vec<I18nContent>,
    #[serde(default)]
    pub i18n_notes: Vec<I18nContent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Indicator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default)]
    pub i18n_names: Vec<I18nContent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Item {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_off_frequency_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default)]
    pub i18n_names: Vec<I18nContent>,
    #[serde(default)]
    pub i18n_descriptions: Vec<I18nContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemCategory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub i18n_names: Vec<I18nContent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LumpSumPayment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_period: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_frequency: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_detail_text: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_amount_before_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_amount_after_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_period_offboarding_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_period_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_period_end_date: Option<String>,
    #[serde(default)]
    pub details: Vec<LumpSumPaymentDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_period_decimal: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LumpSumPaymentDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_way: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub belong_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_pay_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_reference_period_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_reference_period_end_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LumpSumPaymentOperateResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecurringPayment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub each_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_country_region_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecurringPaymentOperateResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialArchive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub details: Vec<SocialArchiveDetail>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialArchiveAdjustRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    #[serde(default)]
    pub details: Vec<SocialArchiveDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialArchiveDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insurance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insurance_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insurance_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_salary: Option<String>,
    #[serde(default)]
    pub insurance_details: Vec<SocialArchiveItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialArchiveItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insurance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insurance_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_deduction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_setting: Option<SocialPlanItemSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_deduction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_setting: Option<SocialPlanItemSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_frequency: Option<String>,
    #[serde(default)]
    pub payment_months: Vec<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialInsurance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insurance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialPlan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insurance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<SocialPlanScope>,
    #[serde(default)]
    pub item_detail: Vec<SocialPlanItemDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialPlanItemDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_setting_of_person: Option<SocialPlanItemSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_setting_of_company: Option<SocialPlanItemSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_frequency: Option<String>,
    #[serde(default)]
    pub payment_months: Vec<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialPlanItemSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_ratio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_rounding_rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_decimals: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_payment: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialPlanScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all: Option<bool>,
    #[serde(default)]
    pub rules: Vec<crate::JsonValue>,
}
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
        body: impl Serialize,
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
        .send_v2_response::<CreateArchiveRespData, CreateArchiveResp>()
        .await
    }

    pub async fn query(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<QueryArchiveRespData, QueryArchiveResp>()
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
        .send_v2_response::<ListChangeReasonRespData, ListChangeReasonResp>()
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
        .send_v2_response::<ListIndicatorRespData, ListIndicatorResp>()
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
        .send_v2_response::<ListItemRespData, ListItemResp>()
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
        .send_v2_response::<ListItemCategoryRespData, ListItemCategoryResp>()
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
        body: impl Serialize,
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
        .send_v2_response::<BatchCreateLumpSumPaymentRespData, BatchCreateLumpSumPaymentResp>()
        .await
    }

    pub async fn batch_remove(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<BatchRemoveLumpSumPaymentRespData, BatchRemoveLumpSumPaymentResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<BatchUpdateLumpSumPaymentRespData, BatchUpdateLumpSumPaymentResp>()
        .await
    }

    pub async fn query(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<QueryLumpSumPaymentRespData, QueryLumpSumPaymentResp>()
        .await
    }

    pub async fn query_detail(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<QueryDetailLumpSumPaymentRespData, QueryDetailLumpSumPaymentResp>()
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
        body: impl Serialize,
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
        .send_v2_response::<BatchCreateRecurringPaymentRespData, BatchCreateRecurringPaymentResp>()
        .await
    }

    pub async fn batch_remove(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<BatchRemoveRecurringPaymentRespData, BatchRemoveRecurringPaymentResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<BatchUpdateRecurringPaymentRespData, BatchUpdateRecurringPaymentResp>()
        .await
    }

    pub async fn query(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<QueryRecurringPaymentRespData, QueryRecurringPaymentResp>()
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
        body: impl Serialize,
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
        .send_v2_response::<QuerySocialArchiveRespData, QuerySocialArchiveResp>()
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
        body: impl Serialize,
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
        .send_v2_response::<QuerySocialArchiveAdjustRecordRespData, QuerySocialArchiveAdjustRecordResp>()
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
        .send_v2_response::<ListSocialInsuranceRespData, ListSocialInsuranceResp>()
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
        .send_v2_response::<ListSocialPlanRespData, ListSocialPlanResp>()
        .await
    }

    pub async fn query(
        &self,
        body: impl Serialize,
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
        .send_v2_response::<QuerySocialPlanRespData, QuerySocialPlanResp>()
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
