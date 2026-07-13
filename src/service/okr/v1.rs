use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyRespV2 as EmptyResp, PageQuery, RestRequest};

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

impl_resp_v2!(UploadImageResp, UploadImageRespData);
impl_resp_v2!(BatchGetOkrResp, BatchGetOkrRespData);
impl_resp_v2!(CreatePeriodResp, CreatePeriodRespData);
impl_resp_v2!(PatchPeriodResp, PatchPeriodRespData);
impl_resp_v2!(ListPeriodRuleResp, ListPeriodRuleRespData);
impl_resp_v2!(CreateProgressRecordResp, CreateProgressRecordRespData);
impl_resp_v2!(GetProgressRecordResp, GetProgressRecordRespData);
impl_resp_v2!(UpdateProgressRecordResp, UpdateProgressRecordRespData);
impl_resp_v2!(QueryReviewResp, QueryReviewRespData);

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListPeriodQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListPeriodQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreatePeriodQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreatePeriodQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchPeriodQuery<'a> {
    pub period_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> PatchPeriodQuery<'a> {
    pub fn new(period_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { period_id, body }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListPeriodRuleQuery;

impl ListPeriodRuleQuery {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UploadImageQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> UploadImageQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchGetOkrQuery<'a> {
    pub okr_ids: &'a [&'a str],
    pub user_id_type: Option<&'a str>,
    pub lang: Option<&'a str>,
}

impl<'a> BatchGetOkrQuery<'a> {
    pub fn new(okr_ids: &'a [&'a str]) -> Self {
        Self {
            okr_ids,
            user_id_type: None,
            lang: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateProgressRecordQuery<'a> {
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateProgressRecordQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteProgressRecordQuery<'a> {
    pub progress_id: &'a str,
}

impl<'a> DeleteProgressRecordQuery<'a> {
    pub fn new(progress_id: &'a str) -> Self {
        Self { progress_id }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetProgressRecordQuery<'a> {
    pub progress_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetProgressRecordQuery<'a> {
    pub fn new(progress_id: &'a str) -> Self {
        Self {
            progress_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateProgressRecordQuery<'a> {
    pub progress_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateProgressRecordQuery<'a> {
    pub fn new(progress_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            progress_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryReviewQuery<'a> {
    pub user_ids: &'a [&'a str],
    pub period_ids: Option<&'a [&'a str]>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> QueryReviewQuery<'a> {
    pub fn new(user_ids: &'a [&'a str]) -> Self {
        Self {
            user_ids,
            period_ids: None,
            user_id_type: None,
        }
    }

    pub fn period_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.period_ids = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetUserOkrQuery<'a> {
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub period_ids: Option<Vec<&'a str>>,
    pub offset: Option<&'a str>,
    pub limit: Option<&'a str>,
    pub lang: Option<&'a str>,
}

impl<'a> GetUserOkrQuery<'a> {
    pub fn new(user_id: &'a str) -> Self {
        Self {
            user_id,
            user_id_type: None,
            period_ids: None,
            offset: None,
            limit: None,
            lang: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn period_ids(mut self, value: impl Into<Option<Vec<&'a str>>>) -> Self {
        self.period_ids = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.offset = value.into();
        self
    }

    pub fn limit(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadImageRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchGetOkrRespData {
    #[serde(default)]
    pub okr_list: Vec<OkrBatch>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePeriodRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_month: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_month: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchPeriodRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPeriodRuleRespData {
    #[serde(default)]
    pub period_rules: Vec<PeriodRule>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateProgressRecordRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetProgressRecordRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProgressRecordRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryReviewRespData {
    #[serde(default)]
    pub review_list: Vec<OkrReview>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentBlock {
    #[serde(default)]
    pub blocks: Vec<ContentBlockElement>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentBlockElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<ContentParagraph>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gallery: Option<ContentGallery>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentColor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub red: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub green: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blue: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentDocsLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentGallery {
    #[serde(default, rename = "imageList")]
    pub image_list: Vec<ContentImageItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentImageItem {
    #[serde(default, rename = "fileToken", skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        rename = "indentLevel",
        skip_serializing_if = "Option::is_none"
    )]
    pub indent_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentParagraph {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentParagraphStyle>,
    #[serde(default)]
    pub elements: Vec<ContentParagraphElement>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentParagraphElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, rename = "textRun", skip_serializing_if = "Option::is_none")]
    pub text_run: Option<ContentTextRun>,
    #[serde(default, rename = "docsLink", skip_serializing_if = "Option::is_none")]
    pub docs_link: Option<ContentDocsLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<ContentPerson>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentParagraphStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<ContentList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentPerson {
    #[serde(default, rename = "openId", skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentTextRun {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentTextStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentTextStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(
        default,
        rename = "strikeThrough",
        skip_serializing_if = "Option::is_none"
    )]
    pub strike_through: Option<bool>,
    #[serde(default, rename = "backColor", skip_serializing_if = "Option::is_none")]
    pub back_color: Option<ContentColor>,
    #[serde(default, rename = "textColor", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ContentColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<ContentLink>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrBatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub objective_list: Vec<ResponseOkrObjective>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseOkrObjective {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_report: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<OkrObjectiveProgressRate>,
    #[serde(default)]
    pub kr_list: Vec<OkrObjectiveKr>,
    #[serde(default)]
    pub aligned_objective_list: Vec<OkrObjectiveAlignedObjective>,
    #[serde(default)]
    pub aligning_objective_list: Vec<OkrObjectiveAlignedObjective>,
    #[serde(default)]
    pub progress_record_list: Vec<ProgressRecordSimplify>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_percent_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_status_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_record_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_report_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(default)]
    pub mentioned_user_list: Vec<OkrObjectiveAlignedObjectiveOwner>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrObjectiveAlignedObjective {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<OkrObjectiveAlignedObjectiveOwner>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrObjectiveAlignedObjectiveOwner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrObjectiveKr {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kr_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<OkrObjectiveProgressRate>,
    #[serde(default)]
    pub progress_record_list: Vec<ProgressRecordSimplify>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_percent_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_status_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_record_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_report_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(default)]
    pub mentioned_user_list: Vec<OkrObjectiveAlignedObjectiveOwner>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrObjectiveProgressRate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrReview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<OkrObjectiveAlignedObjectiveOwner>,
    #[serde(default)]
    pub review_period_list: Vec<OkrReviewPeriod>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrReviewPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    #[serde(default)]
    pub cycle_review_list: Vec<OkrReviewPeriodUrl>,
    #[serde(default)]
    pub progress_report_list: Vec<OkrReviewPeriodUrl>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrReviewPeriodUrl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeriodRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_month: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgressRateNew {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgressRecordSimplify {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
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
    ) -> Result<ListPeriodResp, LarkError> {
        let query = ListPeriodQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPeriodQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPeriodResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/periods",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_response::<PeriodListData, ListPeriodResp>()
        .await
    }

    pub async fn create(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreatePeriodResp, LarkError> {
        let query = CreatePeriodQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreatePeriodQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreatePeriodResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/okr/v1/periods",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CreatePeriodRespData, CreatePeriodResp>()
        .await
    }

    pub async fn patch(
        &self,
        period_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPeriodResp, LarkError> {
        let query = PatchPeriodQuery::new(period_id, body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchPeriodQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchPeriodResp, LarkError> {
        let path = format!("/open-apis/okr/v1/periods/{}", query.period_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<PatchPeriodRespData, PatchPeriodResp>()
        .await
    }
}

// ── PeriodRule resource ──

pub struct PeriodRuleResource<'a> {
    config: &'a Config,
}

impl PeriodRuleResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListPeriodRuleResp, LarkError> {
        let query = ListPeriodRuleQuery::new();
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        _query: &ListPeriodRuleQuery,
        option: &RequestOption,
    ) -> Result<ListPeriodRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/period_rules",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<ListPeriodRuleRespData, ListPeriodRuleResp>()
        .await
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
    ) -> Result<UploadImageResp, LarkError> {
        let query = UploadImageQuery::new(body);
        self.upload_by_query(&query, option).await
    }

    pub async fn upload_by_query(
        &self,
        query: &UploadImageQuery<'_>,
        option: &RequestOption,
    ) -> Result<UploadImageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/okr/v1/images/upload",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<UploadImageRespData, UploadImageResp>()
        .await
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
    ) -> Result<BatchGetOkrResp, LarkError> {
        let query = BatchGetOkrQuery::new(okr_ids)
            .user_id_type(user_id_type)
            .lang(lang);
        self.batch_get_by_query(&query, option).await
    }

    pub async fn batch_get_by_query(
        &self,
        query: &BatchGetOkrQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchGetOkrResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/okrs/batch_get",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query_values("okr_ids", Some(query.okr_ids.iter().copied()))
        .query("user_id_type", query.user_id_type)
        .query("lang", query.lang)
        .send_v2_response::<BatchGetOkrRespData, BatchGetOkrResp>()
        .await
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
    ) -> Result<CreateProgressRecordResp, LarkError> {
        let query = CreateProgressRecordQuery::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateProgressRecordResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/okr/v1/progress_records",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CreateProgressRecordRespData, CreateProgressRecordResp>()
        .await
    }

    pub async fn delete(
        &self,
        progress_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteProgressRecordQuery::new(progress_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/okr/v1/progress_records/{}", query.progress_id);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<serde_json::Value, EmptyResp>()
        .await
    }

    pub async fn get(
        &self,
        progress_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetProgressRecordResp, LarkError> {
        let query = GetProgressRecordQuery::new(progress_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetProgressRecordResp, LarkError> {
        let path = format!("/open-apis/okr/v1/progress_records/{}", query.progress_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetProgressRecordRespData, GetProgressRecordResp>()
        .await
    }

    pub async fn update(
        &self,
        progress_id: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateProgressRecordResp, LarkError> {
        let query = UpdateProgressRecordQuery::new(progress_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateProgressRecordResp, LarkError> {
        let path = format!("/open-apis/okr/v1/progress_records/{}", query.progress_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<UpdateProgressRecordRespData, UpdateProgressRecordResp>()
        .await
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
    ) -> Result<QueryReviewResp, LarkError> {
        let query = QueryReviewQuery::new(user_ids)
            .period_ids(period_ids)
            .user_id_type(user_id_type);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryReviewQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryReviewResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/reviews/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query_values("user_ids", Some(query.user_ids.iter().copied()))
        .query_values(
            "period_ids",
            query.period_ids.map(|ids| ids.iter().copied()),
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<QueryReviewRespData, QueryReviewResp>()
        .await
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
    ) -> Result<GetUserOkrResp, LarkError> {
        let query = GetUserOkrQuery::new(user_id)
            .user_id_type(user_id_type)
            .period_ids(period_ids)
            .offset(offset)
            .limit(limit)
            .lang(lang);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetUserOkrQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetUserOkrResp, LarkError> {
        let path = format!("/open-apis/okr/v1/users/{}/okrs", query.user_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query_values(
            "period_ids",
            query.period_ids.as_ref().map(|ids| ids.iter().copied()),
        )
        .query("offset", query.offset)
        .query("limit", query.limit)
        .query("lang", query.lang)
        .send_response::<OkrListData, GetUserOkrResp>()
        .await
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
