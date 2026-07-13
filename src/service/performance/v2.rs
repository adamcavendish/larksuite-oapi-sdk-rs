use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricTagListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(QueryActivityV2Resp, QueryActivityV2RespData);
impl_resp_v2!(
    ImportAdditionalInformationV2Resp,
    ImportAdditionalInformationV2RespData
);
impl_resp_v2!(
    QueryAdditionalInformationV2Resp,
    QueryAdditionalInformationV2RespData
);
impl_resp_v2!(DeleteAdditionalInformationsBatchV2Resp, ());
impl_resp_v2!(QueryIndicatorV2Resp, QueryIndicatorV2RespData);
impl_resp_v2!(ImportMetricDetailV2Resp, ImportMetricDetailV2RespData);
impl_resp_v2!(QueryMetricDetailV2Resp, QueryMetricDetailV2RespData);
impl_resp_v2!(QueryMetricFieldV2Resp, QueryMetricFieldV2RespData);
impl_resp_v2!(QueryMetricLibV2Resp, QueryMetricLibV2RespData);
impl_resp_v2!(ListMetricTagV2Resp, MetricTagListData);
impl_resp_v2!(QueryMetricTemplateV2Resp, QueryMetricTemplateV2RespData);
impl_resp_v2!(QueryQuestionV2Resp, QueryQuestionV2RespData);
impl_resp_v2!(QueryReviewDataV2Resp, QueryReviewDataV2RespData);
impl_resp_v2!(QueryReviewTemplateV2Resp, QueryReviewTemplateV2RespData);
impl_resp_v2!(QueryRevieweeV2Resp, QueryRevieweeV2RespData);
impl_resp_v2!(WriteUserGroupUserRelV2Resp, WriteUserGroupUserRelV2RespData);
impl_resp_v2!(QueryUserInfoV2Resp, QueryUserInfoV2RespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryActivityV2RespData {
    #[serde(default)]
    pub activities: Vec<Activity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportAdditionalInformationV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_record_id: Option<String>,
    #[serde(default)]
    pub additional_informations: Vec<AdditionalInformation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryAdditionalInformationV2RespData {
    #[serde(default)]
    pub additional_informations: Vec<AdditionalInformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryIndicatorV2RespData {
    #[serde(default)]
    pub indicators: Vec<Indicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportMetricDetailV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricDetailV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default)]
    pub reviewee_metrics: Vec<RevieweeMetric>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricFieldV2RespData {
    #[serde(default)]
    pub items: Vec<MetricField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricLibV2RespData {
    #[serde(default)]
    pub items: Vec<MetricInLibrary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricTemplateV2RespData {
    #[serde(default)]
    pub items: Vec<MetricTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryQuestionV2RespData {
    #[serde(default)]
    pub tag_based_questions: Vec<Question>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryReviewDataV2RespData {
    #[serde(default)]
    pub datas: Vec<ReviewProfile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryReviewTemplateV2RespData {
    #[serde(default)]
    pub review_templates: Vec<ReviewTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryRevieweeV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default)]
    pub reviewees: Vec<Reviewee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WriteUserGroupUserRelV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<WriteUserGroupScopeData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserInfoV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default)]
    pub user_infos: Vec<UserInfo>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Activity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdditionalInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detailed_description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CooperationProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default)]
    pub roles: Vec<CooperationRole>,
    #[serde(default)]
    pub user_roles: Vec<CooperationUserRole>,
    #[serde(default)]
    pub underling_roles: Vec<CooperationUserRole>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CooperationRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_role: Option<CooperationUserRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_role: Option<CooperationUserRole>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CooperationUserRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomMetricConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_formula_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub least_metrics_size: Option<i32>,
    #[serde(default)]
    pub add_metric_options: Vec<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectProjectLeaderRecordInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<User>,
    #[serde(default)]
    pub cooperation_projects: Vec<CooperationProject>,
    #[serde(default)]
    pub review_depend_projects: Vec<CooperationProject>,
    #[serde(default)]
    pub participated_projects: Vec<CooperationProject>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Field {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_based_question_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_text_qustion_title: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyresult_text_qustion_title: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kpi_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Formula {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_details: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Indicator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub options: Vec<IndicatorOption>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndicatorOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lable: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvitedReviewRecordInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_rejected: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribute_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_diff: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_with_reviewee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitedby: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFamily {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyresultData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyresult_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub fields: Vec<MetricFieldInDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_weight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_from_library: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricDimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_dimension_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluation_rule_id_for_each_metric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_weight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_rule_option: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metric_config: Option<CustomMetricConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricFieldInDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value_person: Option<User>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricFieldInLibrary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_setting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value_person: Option<User>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricFieldInTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_setting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filed_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value_person: Option<User>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricInLibrary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(default)]
    pub tags: Vec<MetricTag>,
    #[serde(default)]
    pub fields: Vec<MetricFieldInLibrary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_setting_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_formula: Option<Formula>,
    #[serde(default)]
    pub data_source_inputters: Vec<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_of_availability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricInTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(default)]
    pub fields: Vec<MetricFieldInTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_from_library: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_setting_type: Option<String>,
    #[serde(default)]
    pub data_source_inputters: Vec<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_dimension_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_rule_config: Option<MetricReviewRuleConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricReviewRuleConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_set_by_group: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_metric_score_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_weight_method: Option<String>,
    #[serde(default)]
    pub metric_dimensions: Vec<MetricDimension>,
    #[serde(default)]
    pub metrics: Vec<MetricInTemplate>,
    #[serde(default)]
    pub groups: Vec<MetricGroup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectiveData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    pub keyresult_data: Vec<KeyresultData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Question {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default)]
    pub tag_items: Vec<TagItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_user_id: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_based_question_id: Option<String>,
    #[serde(default)]
    pub tag_text_item_data: Vec<TagText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perf_coefficient_value: Option<String>,
    #[serde(default)]
    pub sub_indicator_data: Vec<SubIndicator>,
    #[serde(default)]
    pub objective_data: Vec<ObjectiveData>,
    #[serde(default)]
    pub metric_data: Vec<MetricData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_review_data_source: Option<String>,
    #[serde(default)]
    pub multi_texts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
    #[serde(default)]
    pub multi_richtexts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_principal_review_item: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_template_id: Option<String>,
    #[serde(default)]
    pub stages: Vec<ReviewStage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(default)]
    pub units: Vec<ReviewUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_review_record_info: Option<InvitedReviewRecordInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_project_leader_record_info: Option<DirectProjectLeaderRecordInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewStage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    #[serde(default)]
    pub review_stage_roles: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default)]
    pub records: Vec<ReviewRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_stage_role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewTemplate {
    #[serde(default)]
    pub templates: Vec<Template>,
    #[serde(default)]
    pub units: Vec<Unit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewUnit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unknown: Option<bool>,
    #[serde(default)]
    pub data: Vec<ReviewDetail>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reviewee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_user_id: Option<User>,
    #[serde(default)]
    pub activity_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewprofile_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevieweeMetric {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_user_id: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_details: Option<MetricDetail>,
    #[serde(default)]
    pub reviewee_stage_statuses: Vec<RevieweeStageStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevieweeStageStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubIndicator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_text_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_richtext: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Template {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_stage_role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_stage_data_write_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Unit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default)]
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_leader_user_id: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<Department>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family: Option<JobFamily>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<JobLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WriteUserGroupScopeData {
    #[serde(default)]
    pub success_user_ids: Vec<String>,
    #[serde(default)]
    pub fail_user_datas: Vec<WriteUserGroupScopeFailUserData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WriteUserGroupScopeFailUserData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_code: Option<i32>,
}
// -- Query parameter types --

#[derive(Debug, Clone, Copy, Default)]
pub struct ListMetricTagV2Query<'a> {
    pub tag_ids: Option<&'a [&'a str]>,
    pub page: PageQuery<'a>,
}

impl<'a> ListMetricTagV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.tag_ids = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

pub struct V2<'a> {
    pub activity: ActivityV2Resource<'a>,
    pub additional_information: AdditionalInformationV2Resource<'a>,
    pub additional_informations_batch: AdditionalInformationsBatchV2Resource<'a>,
    pub indicator: IndicatorV2Resource<'a>,
    pub metric_detail: MetricDetailV2Resource<'a>,
    pub metric_field: MetricFieldV2Resource<'a>,
    pub metric_lib: MetricLibV2Resource<'a>,
    pub metric_tag: MetricTagV2Resource<'a>,
    pub metric_template: MetricTemplateV2Resource<'a>,
    pub question: QuestionV2Resource<'a>,
    pub review_data: ReviewDataV2Resource<'a>,
    pub review_template: ReviewTemplateV2Resource<'a>,
    pub reviewee: RevieweeV2Resource<'a>,
    pub user_group_user_rel: UserGroupUserRelV2Resource<'a>,
    pub user_info: UserInfoV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            activity: ActivityV2Resource { config },
            additional_information: AdditionalInformationV2Resource { config },
            additional_informations_batch: AdditionalInformationsBatchV2Resource { config },
            indicator: IndicatorV2Resource { config },
            metric_detail: MetricDetailV2Resource { config },
            metric_field: MetricFieldV2Resource { config },
            metric_lib: MetricLibV2Resource { config },
            metric_tag: MetricTagV2Resource { config },
            metric_template: MetricTemplateV2Resource { config },
            question: QuestionV2Resource { config },
            review_data: ReviewDataV2Resource { config },
            review_template: ReviewTemplateV2Resource { config },
            reviewee: RevieweeV2Resource { config },
            user_group_user_rel: UserGroupUserRelV2Resource { config },
            user_info: UserInfoV2Resource { config },
        }
    }
}

macro_rules! post_query {
    ($struct_name:ident, $query_name:ident, $method:ident, $by_query_method:ident, $data:ty, $resp:ident, $path:literal) => {
        #[derive(Debug, Clone, Copy)]
        #[non_exhaustive]
        pub struct $query_name<'a> {
            pub body: &'a serde_json::Value,
        }

        impl<'a> $query_name<'a> {
            pub fn new(body: &'a serde_json::Value) -> Self {
                Self { body }
            }
        }

        pub struct $struct_name<'a> {
            config: &'a Config,
        }

        impl $struct_name<'_> {
            pub async fn $method(
                &self,
                body: serde_json::Value,
                option: &RequestOption,
            ) -> Result<$resp, LarkError> {
                let query = $query_name::new(&body);
                self.$by_query_method(&query, option).await
            }

            pub async fn $by_query_method(
                &self,
                query: &$query_name<'_>,
                option: &RequestOption,
            ) -> Result<$resp, LarkError> {
                RestRequest::new(
                    self.config,
                    http::Method::POST,
                    $path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .json_body(query.body)?
                .send_v2_response::<$data, $resp>()
                .await
            }
        }
    };
}

post_query!(
    ActivityV2Resource,
    QueryActivityV2Query,
    query,
    query_by_query,
    QueryActivityV2RespData,
    QueryActivityV2Resp,
    "/open-apis/performance/v2/activity/query"
);

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ImportAdditionalInformationV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> ImportAdditionalInformationV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryAdditionalInformationV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> QueryAdditionalInformationV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

pub struct AdditionalInformationV2Resource<'a> {
    config: &'a Config,
}

impl AdditionalInformationV2Resource<'_> {
    pub async fn import(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ImportAdditionalInformationV2Resp, LarkError> {
        let query = ImportAdditionalInformationV2Query::new(&body);
        self.import_by_query(&query, option).await
    }

    pub async fn import_by_query(
        &self,
        query: &ImportAdditionalInformationV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ImportAdditionalInformationV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/additional_informations/import",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<ImportAdditionalInformationV2RespData, ImportAdditionalInformationV2Resp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryAdditionalInformationV2Resp, LarkError> {
        let query = QueryAdditionalInformationV2Query::new(&body);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryAdditionalInformationV2Query<'_>,
        option: &RequestOption,
    ) -> Result<QueryAdditionalInformationV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/additional_informations/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<QueryAdditionalInformationV2RespData, QueryAdditionalInformationV2Resp>(
        )
        .await
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteAdditionalInformationsBatchV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> DeleteAdditionalInformationsBatchV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

pub struct AdditionalInformationsBatchV2Resource<'a> {
    config: &'a Config,
}

impl AdditionalInformationsBatchV2Resource<'_> {
    pub async fn delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<DeleteAdditionalInformationsBatchV2Resp, LarkError> {
        let query = DeleteAdditionalInformationsBatchV2Query::new(&body);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteAdditionalInformationsBatchV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteAdditionalInformationsBatchV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/performance/v2/additional_informations/batch",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), DeleteAdditionalInformationsBatchV2Resp>()
        .await
    }
}

post_query!(
    IndicatorV2Resource,
    QueryIndicatorV2Query,
    query,
    query_by_query,
    QueryIndicatorV2RespData,
    QueryIndicatorV2Resp,
    "/open-apis/performance/v2/indicators/query"
);

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ImportMetricDetailV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> ImportMetricDetailV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryMetricDetailV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> QueryMetricDetailV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

pub struct MetricDetailV2Resource<'a> {
    config: &'a Config,
}

impl MetricDetailV2Resource<'_> {
    pub async fn import(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ImportMetricDetailV2Resp, LarkError> {
        let query = ImportMetricDetailV2Query::new(&body);
        self.import_by_query(&query, option).await
    }

    pub async fn import_by_query(
        &self,
        query: &ImportMetricDetailV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ImportMetricDetailV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/metric_details/import",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<ImportMetricDetailV2RespData, ImportMetricDetailV2Resp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryMetricDetailV2Resp, LarkError> {
        let query = QueryMetricDetailV2Query::new(&body);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryMetricDetailV2Query<'_>,
        option: &RequestOption,
    ) -> Result<QueryMetricDetailV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/metric_details/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<QueryMetricDetailV2RespData, QueryMetricDetailV2Resp>()
        .await
    }
}

post_query!(
    MetricFieldV2Resource,
    QueryMetricFieldV2Query,
    query,
    query_by_query,
    QueryMetricFieldV2RespData,
    QueryMetricFieldV2Resp,
    "/open-apis/performance/v2/metric_fields/query"
);
post_query!(
    MetricLibV2Resource,
    QueryMetricLibV2Query,
    query,
    query_by_query,
    QueryMetricLibV2RespData,
    QueryMetricLibV2Resp,
    "/open-apis/performance/v2/metric_libs/query"
);

pub struct MetricTagV2Resource<'a> {
    config: &'a Config,
}

impl MetricTagV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMetricTagV2Resp, LarkError> {
        let query = ListMetricTagV2Query::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListMetricTagV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListMetricTagV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/performance/v2/metric_tags",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query_values("tag_ids", query.tag_ids)
        .send_v2_response::<MetricTagListData, ListMetricTagV2Resp>()
        .await
    }
}

post_query!(
    MetricTemplateV2Resource,
    QueryMetricTemplateV2Query,
    query,
    query_by_query,
    QueryMetricTemplateV2RespData,
    QueryMetricTemplateV2Resp,
    "/open-apis/performance/v2/metric_templates/query"
);
post_query!(
    QuestionV2Resource,
    QueryQuestionV2Query,
    query,
    query_by_query,
    QueryQuestionV2RespData,
    QueryQuestionV2Resp,
    "/open-apis/performance/v2/questions/query"
);
post_query!(
    ReviewDataV2Resource,
    QueryReviewDataV2Query,
    query,
    query_by_query,
    QueryReviewDataV2RespData,
    QueryReviewDataV2Resp,
    "/open-apis/performance/v2/review_datas/query"
);
post_query!(
    ReviewTemplateV2Resource,
    QueryReviewTemplateV2Query,
    query,
    query_by_query,
    QueryReviewTemplateV2RespData,
    QueryReviewTemplateV2Resp,
    "/open-apis/performance/v2/review_templates/query"
);
post_query!(
    RevieweeV2Resource,
    QueryRevieweeV2Query,
    query,
    query_by_query,
    QueryRevieweeV2RespData,
    QueryRevieweeV2Resp,
    "/open-apis/performance/v2/reviewees/query"
);
post_query!(
    UserGroupUserRelV2Resource,
    WriteUserGroupUserRelV2Query,
    write,
    write_by_query,
    WriteUserGroupUserRelV2RespData,
    WriteUserGroupUserRelV2Resp,
    "/open-apis/performance/v2/user_group_user_rels/write"
);
post_query!(
    UserInfoV2Resource,
    QueryUserInfoV2Query,
    query,
    query_by_query,
    QueryUserInfoV2RespData,
    QueryUserInfoV2Resp,
    "/open-apis/performance/v2/user_info/query"
);
