use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Response helper ──

// ── Response types ──

impl_resp_v2!(SetAppBadgeResp, ());

impl_resp_v2!(ListAppRecommendRuleResp, ListAppRecommendRuleRespData);

impl_resp_v2!(
    ContactsRangeConfigurationApplicationResp,
    ContactsRangeConfigurationApplicationRespData
);

impl_resp_v2!(GetApplicationResp, GetApplicationRespData);

impl_resp_v2!(ListApplicationResp, ListApplicationRespData);

impl_resp_v2!(PatchApplicationResp, ());

impl_resp_v2!(
    UnderauditlistApplicationResp,
    UnderauditlistApplicationRespData
);

impl_resp_v2!(
    DepartmentOverviewApplicationAppUsageResp,
    DepartmentOverviewApplicationAppUsageRespData
);

impl_resp_v2!(
    MessagePushOverviewApplicationAppUsageResp,
    MessagePushOverviewApplicationAppUsageRespData
);

impl_resp_v2!(
    OverviewApplicationAppUsageResp,
    OverviewApplicationAppUsageRespData
);

impl_resp_v2!(
    ContactsRangeSuggestApplicationAppVersionResp,
    ContactsRangeSuggestApplicationAppVersionRespData
);

impl_resp_v2!(
    GetApplicationAppVersionResp,
    GetApplicationAppVersionRespData
);

impl_resp_v2!(
    ListApplicationAppVersionResp,
    ListApplicationAppVersionRespData
);

impl_resp_v2!(PatchApplicationAppVersionResp, ());

impl_resp_v2!(
    GetApplicationCollaboratorsResp,
    GetApplicationCollaboratorsRespData
);

impl_resp_v2!(UpdateApplicationCollaboratorsResp, ());

impl_resp_v2!(PatchApplicationContactsRangeResp, ());

impl_resp_v2!(ListApplicationFeedbackResp, ListApplicationFeedbackRespData);

impl_resp_v2!(PatchApplicationFeedbackResp, ());

impl_resp_v2!(UpdateApplicationManagementResp, ());

impl_resp_v2!(UpdateApplicationOwnerResp, ());

impl_resp_v2!(
    CheckWhiteBlackListApplicationVisibilityResp,
    CheckWhiteBlackListApplicationVisibilityRespData
);

impl_resp_v2!(PatchApplicationVisibilityResp, ());

impl_resp_v2!(ApplyScopeResp, ());

impl_resp_v2!(ListScopeResp, ListScopeRespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAppRecommendRuleRespData {
    #[serde(default)]
    pub rules: Vec<AppRecommendRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactsRangeConfigurationApplicationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts_range: Option<ApplicationAppContactsRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<Application>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationRespData {
    #[serde(default)]
    pub app_list: Vec<Application>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnderauditlistApplicationRespData {
    #[serde(default)]
    pub items: Vec<Application>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentOverviewApplicationAppUsageRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<ApplicationDepartmentAppUsage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePushOverviewApplicationAppUsageRespData {
    #[serde(default)]
    pub items: Vec<ApplicationAppUsage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OverviewApplicationAppUsageRespData {
    #[serde(default)]
    pub items: Vec<ApplicationAppUsage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactsRangeSuggestApplicationAppVersionRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts_range: Option<ApplicationAppContactsRange>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationAppVersionRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_version: Option<ApplicationAppVersion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationAppVersionRespData {
    #[serde(default)]
    pub items: Vec<ApplicationAppVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationCollaboratorsRespData {
    #[serde(default)]
    pub collaborators: Vec<AppCollaborator>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationFeedbackRespData {
    #[serde(default)]
    pub feedback_list: Vec<ApplicationFeedback>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckWhiteBlackListApplicationVisibilityRespData {
    #[serde(default)]
    pub user_visibility_list: Vec<ApplicationVisibilityUserWhiteBlackInfo>,
    #[serde(default)]
    pub department_visibility_list: Vec<ApplicationVisibilityDepartmentWhiteBlackInfo>,
    #[serde(default)]
    pub group_visibility_list: Vec<ApplicationVisibilityGroupWhiteBlackInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListScopeRespData {
    #[serde(default)]
    pub scopes: Vec<Scope>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppAbility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gadget: Option<Gadget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
    #[serde(default)]
    pub workplace_widgets: Vec<WorkplaceWidget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigate: Option<Navigate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_doc: Option<CloudDoc>,
    #[serde(default)]
    pub docs_blocks: Vec<DocsBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_action: Option<MessageAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plus_menu: Option<PlusMenu>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppCollaborator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigSecurityItem {
    #[serde(default)]
    pub redirect_urls: Vec<String>,
    #[serde(default)]
    pub allowed_ips: Vec<String>,
    #[serde(default)]
    pub h5_trusted_domains: Vec<String>,
    #[serde(default)]
    pub web_view_trusted_domains: Vec<String>,
    #[serde(default)]
    pub allowed_schemas: Vec<String>,
    #[serde(default)]
    pub allowed_server_domains: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_use: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRecommendRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_info: Option<AppRecommendRuleVisibilityInfo>,
    #[serde(default)]
    pub recommend_item_infos: Vec<AppRecommendRuleItemInfo>,
    #[serde(default)]
    pub distributed_recommend_item_infos: Vec<AppRecommendRuleItemInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRecommendRuleItemInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<AppRecommendRuleItemInfoI18nName>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRecommendRuleItemInfoI18nName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_hk: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_tw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRecommendRuleVisibilityInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all: Option<bool>,
    #[serde(default)]
    pub department_ids: Vec<String>,
    #[serde(default)]
    pub user_ids: Vec<String>,
    #[serde(default)]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(default)]
    pub token_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVersionRemark {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AppVisibility>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVisibility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_list: Option<AppVisibleList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invisible_list: Option<AppVisibleList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVisibleList {
    #[serde(default)]
    pub open_ids: Vec<String>,
    #[serde(default)]
    pub department_ids: Vec<String>,
    #[serde(default)]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_source: Option<String>,
    #[serde(default)]
    pub redirect_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unaudit_version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub scopes: Vec<AppScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_home_url: Option<String>,
    #[serde(default)]
    pub i18n: Vec<AppI18nInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<String>,
    #[serde(default)]
    pub common_categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<ApplicationOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_default_ability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_default_ability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<SubscribedEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback: Option<Callback>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EventAndCallbackEncryptStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<AppConfigSecurityItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_refresh_token: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_info: Option<CallbackInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationAppContactsRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts_scope_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_list: Option<AppVisibleList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationAppUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_value: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationAppVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub scopes: Vec<AppScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_home_url: Option<String>,
    #[serde(default)]
    pub i18n: Vec<AppI18nInfo>,
    #[serde(default)]
    pub common_categories: Vec<String>,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability: Option<AppAbility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<AppVersionRemark>,
    #[serde(default)]
    pub event_infos: Vec<Event>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationDepartmentAppUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default)]
    pub app: Vec<ApplicationAppUsage>,
    #[serde(default)]
    pub gadget: Vec<ApplicationAppUsage>,
    #[serde(default)]
    pub webapp: Vec<ApplicationAppUsage>,
    #[serde(default)]
    pub bot: Vec<ApplicationAppUsage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationFeedback {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default)]
    pub fault_type: Vec<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationOwner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_desk: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_service_account: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationVisibilityDepartmentWhiteBlackInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_white_list: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_black_list: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationVisibilityGroupWhiteBlackInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_white_list: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_black_list: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationVisibilityUserWhiteBlackInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_white_list: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_black_list: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_paid_list: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_request_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Callback {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    #[serde(default)]
    pub subscribed_callbacks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    #[serde(default)]
    pub subscribed_callbacks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudDoc {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_url: Option<String>,
    #[serde(default)]
    pub i18n: Vec<CloudDocI18nInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudDocI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocsBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_type_id: Option<String>,
    #[serde(default)]
    pub i18n: Vec<BlockI18nInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_icon_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Event {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventAndCallbackEncryptStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Gadget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_pc_mode: Option<i32>,
    #[serde(default)]
    pub schema_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_use_mobile_pkg: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_min_lark_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_min_lark_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_app_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_app_link: Option<String>,
    #[serde(default)]
    pub i18n: Vec<MessageActionI18nInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageActionI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Navigate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc: Option<NavigateMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<NavigateMeta>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NavigateMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover_image_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlusMenu {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_app_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_app_link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscribedEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    #[serde(default)]
    pub subscribed_events: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebApp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkplaceWidget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_lark_version: Option<String>,
}
// ── Resources ──

pub struct AppBadgeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SetAppBadgeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SetAppBadgeQuery<'a> {
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

impl<'a> AppBadgeResource<'a> {
    /// Set app badge — POST /open-apis/application/v6/app_badge/set
    pub async fn set(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SetAppBadgeResp, LarkError> {
        let query = SetAppBadgeQuery::new(body).user_id_type(user_id_type);
        self.set_by_query(&query, option).await
    }

    pub async fn set_by_query(
        &self,
        query: &SetAppBadgeQuery<'_>,
        option: &RequestOption,
    ) -> Result<SetAppBadgeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/application/v6/app_badge/set",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), SetAppBadgeResp>()
        .await
    }
}

pub struct AppRecommendRuleResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListAppRecommendRuleQuery<'a> {
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListAppRecommendRuleQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> AppRecommendRuleResource<'a> {
    /// List recommend rules — GET /open-apis/application/v6/app_recommend_rules
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppRecommendRuleResp, LarkError> {
        let query = ListAppRecommendRuleQuery {
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAppRecommendRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAppRecommendRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/app_recommend_rules",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListAppRecommendRuleRespData, ListAppRecommendRuleResp>()
        .await
    }
}

pub struct ApplicationResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ApplicationContactsRangeConfigurationQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ApplicationContactsRangeConfigurationQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            page: PageQuery::new(),
            department_id_type: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetApplicationQuery<'a> {
    pub app_id: &'a str,
    pub lang: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApplicationQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            lang: None,
            user_id_type: None,
        }
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub lang: Option<&'a str>,
}

impl<'a> PatchApplicationQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            lang: None,
        }
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListApplicationQuery<'a> {
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
    pub lang: Option<&'a str>,
    pub status: Option<i32>,
    pub payment_type: Option<i32>,
    pub owner_type: Option<i32>,
}

impl<'a> ListApplicationQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn payment_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.payment_type = value.into();
        self
    }

    pub fn owner_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.owner_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct UnderauditlistApplicationQuery<'a> {
    pub page: PageQuery<'a>,
    pub lang: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UnderauditlistApplicationQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationResource<'a> {
    /// ContactsRangeConfiguration — GET /open-apis/application/v6/applications/:app_id/contacts_range_configuration
    #[allow(clippy::too_many_arguments)]
    pub async fn contacts_range_configuration(
        &self,
        app_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ContactsRangeConfigurationApplicationResp, LarkError> {
        let query = ApplicationContactsRangeConfigurationQuery {
            app_id,
            page: PageQuery::from_parts(page_size, page_token),
            department_id_type,
            user_id_type,
        };
        self.contacts_range_configuration_by_query(&query, option)
            .await
    }

    pub async fn contacts_range_configuration_by_query(
        &self,
        query: &ApplicationContactsRangeConfigurationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ContactsRangeConfigurationApplicationResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/contacts_range_configuration",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ContactsRangeConfigurationApplicationRespData, ContactsRangeConfigurationApplicationResp>()
        .await
    }

    /// Get application info — GET /open-apis/application/v6/applications/:app_id
    pub async fn get(
        &self,
        app_id: &str,
        lang: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationResp, LarkError> {
        let query = GetApplicationQuery::new(app_id)
            .lang(lang)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApplicationResp, LarkError> {
        let path = format!("/open-apis/application/v6/applications/{}", query.app_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetApplicationRespData, GetApplicationResp>()
        .await
    }

    /// List installed applications — GET /open-apis/application/v6/applications
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        lang: Option<&str>,
        status: Option<i32>,
        payment_type: Option<i32>,
        owner_type: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListApplicationResp, LarkError> {
        let query = ListApplicationQuery {
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
            lang,
            status,
            payment_type,
            owner_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .query("lang", query.lang)
        .query("status", query.status)
        .query("payment_type", query.payment_type)
        .query("owner_type", query.owner_type)
        .send_v2_response::<ListApplicationRespData, ListApplicationResp>()
        .await
    }

    /// Patch application group info — PATCH /open-apis/application/v6/applications/:app_id
    pub async fn patch(
        &self,
        app_id: &str,
        lang: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationResp, LarkError> {
        let query = PatchApplicationQuery::new(app_id, body).lang(lang);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationResp, LarkError> {
        let path = format!("/open-apis/application/v6/applications/{}", query.app_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationResp>()
        .await
    }

    /// Underauditlist — GET /open-apis/application/v6/applications/underauditlist
    #[allow(clippy::too_many_arguments)]
    pub async fn underauditlist(
        &self,
        lang: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UnderauditlistApplicationResp, LarkError> {
        let query = UnderauditlistApplicationQuery {
            page: PageQuery::from_parts(page_size, page_token),
            lang,
            user_id_type,
        };
        self.underauditlist_by_query(&query, option).await
    }

    pub async fn underauditlist_by_query(
        &self,
        query: &UnderauditlistApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<UnderauditlistApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/applications/underauditlist",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<UnderauditlistApplicationRespData, UnderauditlistApplicationResp>()
        .await
    }
}

pub struct ApplicationAppUsageResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ApplicationAppUsageQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub department_id_type: Option<&'a str>,
}

impl<'a> ApplicationAppUsageQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            department_id_type: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }
}

pub type DepartmentOverviewApplicationAppUsageQuery<'a> = ApplicationAppUsageQuery<'a>;
pub type MessagePushOverviewApplicationAppUsageQuery<'a> = ApplicationAppUsageQuery<'a>;
pub type OverviewApplicationAppUsageQuery<'a> = ApplicationAppUsageQuery<'a>;

impl<'a> ApplicationAppUsageResource<'a> {
    /// DepartmentOverview — POST /open-apis/application/v6/applications/:app_id/app_usage/department_overview
    #[allow(clippy::too_many_arguments)]
    pub async fn department_overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<DepartmentOverviewApplicationAppUsageResp, LarkError> {
        let query = DepartmentOverviewApplicationAppUsageQuery::new(app_id, body)
            .department_id_type(department_id_type);
        self.department_overview_by_query(&query, option).await
    }

    pub async fn department_overview_by_query(
        &self,
        query: &DepartmentOverviewApplicationAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<DepartmentOverviewApplicationAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/department_overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<DepartmentOverviewApplicationAppUsageRespData, DepartmentOverviewApplicationAppUsageResp>()
        .await
    }

    /// MessagePushOverview — POST /open-apis/application/v6/applications/:app_id/app_usage/message_push_overview
    pub async fn message_push_overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<MessagePushOverviewApplicationAppUsageResp, LarkError> {
        let query = MessagePushOverviewApplicationAppUsageQuery::new(app_id, body)
            .department_id_type(department_id_type);
        self.message_push_overview_by_query(&query, option).await
    }

    pub async fn message_push_overview_by_query(
        &self,
        query: &MessagePushOverviewApplicationAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<MessagePushOverviewApplicationAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/message_push_overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<MessagePushOverviewApplicationAppUsageRespData, MessagePushOverviewApplicationAppUsageResp>()
        .await
    }

    /// Overview — POST /open-apis/application/v6/applications/:app_id/app_usage/overview
    pub async fn overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<OverviewApplicationAppUsageResp, LarkError> {
        let query = OverviewApplicationAppUsageQuery::new(app_id, body)
            .department_id_type(department_id_type);
        self.overview_by_query(&query, option).await
    }

    pub async fn overview_by_query(
        &self,
        query: &OverviewApplicationAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<OverviewApplicationAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<OverviewApplicationAppUsageRespData, OverviewApplicationAppUsageResp>()
        .await
    }
}

pub struct ApplicationAppVersionResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ContactsRangeSuggestApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub version_id: &'a str,
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ContactsRangeSuggestApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, version_id: &'a str) -> Self {
        Self {
            app_id,
            version_id,
            department_id_type: None,
            user_id_type: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub version_id: &'a str,
    pub lang: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, version_id: &'a str) -> Self {
        Self {
            app_id,
            version_id,
            lang: None,
            user_id_type: None,
        }
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub lang: Option<&'a str>,
    pub order: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            page: PageQuery::new(),
            lang: None,
            order: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn order(mut self, value: impl Into<Option<i32>>) -> Self {
        self.order = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub version_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
    pub operator_id: Option<&'a str>,
    pub reject_reason: Option<&'a str>,
}

impl<'a> PatchApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, version_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            version_id,
            body,
            user_id_type: None,
            operator_id: None,
            reject_reason: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn operator_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_id = value.into();
        self
    }

    pub fn reject_reason(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.reject_reason = value.into();
        self
    }
}

impl<'a> ApplicationAppVersionResource<'a> {
    /// ContactsRangeSuggest — GET /open-apis/application/v6/applications/:app_id/app_versions/:version_id/contacts_range_suggest
    pub async fn contacts_range_suggest(
        &self,
        app_id: &str,
        version_id: &str,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ContactsRangeSuggestApplicationAppVersionResp, LarkError> {
        let query = ContactsRangeSuggestApplicationAppVersionQuery::new(app_id, version_id)
            .department_id_type(department_id_type)
            .user_id_type(user_id_type);
        self.contacts_range_suggest_by_query(&query, option).await
    }

    pub async fn contacts_range_suggest_by_query(
        &self,
        query: &ContactsRangeSuggestApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ContactsRangeSuggestApplicationAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}/contacts_range_suggest",
            query.app_id, query.version_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ContactsRangeSuggestApplicationAppVersionRespData, ContactsRangeSuggestApplicationAppVersionResp>()
        .await
    }

    /// Get app version — GET /open-apis/application/v6/applications/:app_id/app_versions/:version_id
    pub async fn get(
        &self,
        app_id: &str,
        version_id: &str,
        lang: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationAppVersionResp, LarkError> {
        let query = GetApplicationAppVersionQuery::new(app_id, version_id)
            .lang(lang)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApplicationAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}",
            query.app_id, query.version_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetApplicationAppVersionRespData, GetApplicationAppVersionResp>()
        .await
    }

    /// List app versions — GET /open-apis/application/v6/applications/:app_id/app_versions
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_id: &str,
        lang: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        order: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListApplicationAppVersionResp, LarkError> {
        let query = ListApplicationAppVersionQuery {
            app_id,
            page: PageQuery::from_parts(page_size, page_token),
            lang,
            order,
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .page_query(query.page)
        .query("order", query.order)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListApplicationAppVersionRespData, ListApplicationAppVersionResp>()
        .await
    }

    /// Patch app version audit status — PATCH /open-apis/application/v6/applications/:app_id/app_versions/:version_id
    #[allow(clippy::too_many_arguments)]
    pub async fn patch(
        &self,
        app_id: &str,
        version_id: &str,
        user_id_type: Option<&str>,
        operator_id: Option<&str>,
        reject_reason: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationAppVersionResp, LarkError> {
        let query = PatchApplicationAppVersionQuery::new(app_id, version_id, body)
            .user_id_type(user_id_type)
            .operator_id(operator_id)
            .reject_reason(reject_reason);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}",
            query.app_id, query.version_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("operator_id", query.operator_id)
        .query("reject_reason", query.reject_reason)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationAppVersionResp>()
        .await
    }
}

pub struct ApplicationCollaboratorsResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetApplicationCollaboratorsQuery<'a> {
    pub app_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApplicationCollaboratorsQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
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
pub struct UpdateApplicationCollaboratorsQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateApplicationCollaboratorsQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationCollaboratorsResource<'a> {
    /// Get collaborators — GET /open-apis/application/v6/applications/:app_id/collaborators
    pub async fn get(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationCollaboratorsResp, LarkError> {
        let query = GetApplicationCollaboratorsQuery::new(app_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApplicationCollaboratorsQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApplicationCollaboratorsResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetApplicationCollaboratorsRespData, GetApplicationCollaboratorsResp>()
        .await
    }

    /// Update collaborators — PUT /open-apis/application/v6/applications/:app_id/collaborators
    pub async fn update(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationCollaboratorsResp, LarkError> {
        let query =
            UpdateApplicationCollaboratorsQuery::new(app_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateApplicationCollaboratorsQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateApplicationCollaboratorsResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), UpdateApplicationCollaboratorsResp>()
        .await
    }
}

pub struct ApplicationContactsRangeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationContactsRangeQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> PatchApplicationContactsRangeQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            user_id_type: None,
            department_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }
}

impl<'a> ApplicationContactsRangeResource<'a> {
    /// Patch contacts range — PATCH /open-apis/application/v6/applications/:app_id/contacts_range
    pub async fn patch(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationContactsRangeResp, LarkError> {
        let query = PatchApplicationContactsRangeQuery::new(app_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationContactsRangeQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationContactsRangeResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/contacts_range",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationContactsRangeResp>()
        .await
    }
}

pub struct ApplicationFeedbackResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListApplicationFeedbackQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub from_date: Option<&'a str>,
    pub to_date: Option<&'a str>,
    pub feedback_type: Option<i32>,
    pub status: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListApplicationFeedbackQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            page: PageQuery::new(),
            from_date: None,
            to_date: None,
            feedback_type: None,
            status: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn from_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.from_date = value.into();
        self
    }

    pub fn to_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.to_date = value.into();
        self
    }

    pub fn feedback_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.feedback_type = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationFeedbackQuery<'a> {
    pub app_id: &'a str,
    pub feedback_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub status: Option<i32>,
    pub operator_id: Option<&'a str>,
}

impl<'a> PatchApplicationFeedbackQuery<'a> {
    pub fn new(app_id: &'a str, feedback_id: &'a str) -> Self {
        Self {
            app_id,
            feedback_id,
            user_id_type: None,
            status: None,
            operator_id: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn operator_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_id = value.into();
        self
    }
}

impl<'a> ApplicationFeedbackResource<'a> {
    /// List feedbacks — GET /open-apis/application/v6/applications/:app_id/feedbacks
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_id: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
        feedback_type: Option<i32>,
        status: Option<i32>,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListApplicationFeedbackResp, LarkError> {
        let query = ListApplicationFeedbackQuery {
            app_id,
            page: PageQuery::from_parts(page_size, page_token),
            from_date,
            to_date,
            feedback_type,
            status,
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationFeedbackQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationFeedbackResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/feedbacks",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("from_date", query.from_date)
        .query("to_date", query.to_date)
        .query("feedback_type", query.feedback_type)
        .query("status", query.status)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2_response::<ListApplicationFeedbackRespData, ListApplicationFeedbackResp>()
        .await
    }

    /// Patch feedback — PATCH /open-apis/application/v6/applications/:app_id/feedbacks/:feedback_id
    #[allow(clippy::too_many_arguments)]
    pub async fn patch(
        &self,
        app_id: &str,
        feedback_id: &str,
        user_id_type: Option<&str>,
        status: Option<i32>,
        operator_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchApplicationFeedbackResp, LarkError> {
        let query = PatchApplicationFeedbackQuery::new(app_id, feedback_id)
            .user_id_type(user_id_type)
            .status(status)
            .operator_id(operator_id);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationFeedbackQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationFeedbackResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/feedbacks/{}",
            query.app_id, query.feedback_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("status", query.status)
        .query("operator_id", query.operator_id)
        .send_v2_response::<(), PatchApplicationFeedbackResp>()
        .await
    }
}

pub struct ApplicationManagementResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateApplicationManagementQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateApplicationManagementQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { app_id, body }
    }
}

impl<'a> ApplicationManagementResource<'a> {
    /// Update management — PUT /open-apis/application/v6/applications/:app_id/management
    pub async fn update(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationManagementResp, LarkError> {
        self.update_by_query(&UpdateApplicationManagementQuery::new(app_id, body), option)
            .await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateApplicationManagementQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateApplicationManagementResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/management",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateApplicationManagementResp>()
        .await
    }
}

pub struct ApplicationOwnerResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateApplicationOwnerQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateApplicationOwnerQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationOwnerResource<'a> {
    /// Update owner — PUT /open-apis/application/v6/applications/:app_id/owner
    pub async fn update(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationOwnerResp, LarkError> {
        let query = UpdateApplicationOwnerQuery::new(app_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateApplicationOwnerQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateApplicationOwnerResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/owner",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), UpdateApplicationOwnerResp>()
        .await
    }
}

pub struct ApplicationVisibilityResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CheckWhiteBlackListApplicationVisibilityQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> CheckWhiteBlackListApplicationVisibilityQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            user_id_type: None,
            department_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationVisibilityQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchApplicationVisibilityQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            department_id_type: None,
            user_id_type: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationVisibilityResource<'a> {
    /// CheckWhiteBlackList — POST /open-apis/application/v6/applications/:app_id/visibility/check_white_black_list
    pub async fn check_white_black_list(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CheckWhiteBlackListApplicationVisibilityResp, LarkError> {
        let query = CheckWhiteBlackListApplicationVisibilityQuery::new(app_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.check_white_black_list_by_query(&query, option).await
    }

    pub async fn check_white_black_list_by_query(
        &self,
        query: &CheckWhiteBlackListApplicationVisibilityQuery<'_>,
        option: &RequestOption,
    ) -> Result<CheckWhiteBlackListApplicationVisibilityResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/visibility/check_white_black_list",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<CheckWhiteBlackListApplicationVisibilityRespData, CheckWhiteBlackListApplicationVisibilityResp>()
        .await
    }

    /// Patch visibility — PATCH /open-apis/application/v6/applications/:app_id/visibility
    pub async fn patch(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationVisibilityResp, LarkError> {
        let query = PatchApplicationVisibilityQuery::new(app_id, body)
            .department_id_type(department_id_type)
            .user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationVisibilityQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationVisibilityResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/visibility",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationVisibilityResp>()
        .await
    }
}

pub struct ScopeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ApplyScopeQuery;

impl ApplyScopeQuery {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListScopeQuery;

impl ListScopeQuery {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> ScopeResource<'a> {
    /// Apply scope — POST /open-apis/application/v6/scopes/apply
    pub async fn apply(&self, option: &RequestOption) -> Result<ApplyScopeResp, LarkError> {
        self.apply_by_query(&ApplyScopeQuery::new(), option).await
    }

    pub async fn apply_by_query(
        &self,
        _query: &ApplyScopeQuery,
        option: &RequestOption,
    ) -> Result<ApplyScopeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/application/v6/scopes/apply",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), ApplyScopeResp>()
        .await
    }

    /// List scopes — GET /open-apis/application/v6/scopes
    pub async fn list(&self, option: &RequestOption) -> Result<ListScopeResp, LarkError> {
        self.list_by_query(&ListScopeQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        _query: &ListScopeQuery,
        option: &RequestOption,
    ) -> Result<ListScopeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/scopes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<ListScopeRespData, ListScopeResp>()
        .await
    }
}

// ── Version struct ──

pub struct V6<'a> {
    pub app_badge: AppBadgeResource<'a>,
    pub app_recommend_rule: AppRecommendRuleResource<'a>,
    pub application: ApplicationResource<'a>,
    pub application_app_usage: ApplicationAppUsageResource<'a>,
    pub application_app_version: ApplicationAppVersionResource<'a>,
    pub application_collaborators: ApplicationCollaboratorsResource<'a>,
    pub application_contacts_range: ApplicationContactsRangeResource<'a>,
    pub application_feedback: ApplicationFeedbackResource<'a>,
    pub application_management: ApplicationManagementResource<'a>,
    pub application_owner: ApplicationOwnerResource<'a>,
    pub application_visibility: ApplicationVisibilityResource<'a>,
    pub scope: ScopeResource<'a>,
}

impl<'a> V6<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app_badge: AppBadgeResource { config },
            app_recommend_rule: AppRecommendRuleResource { config },
            application: ApplicationResource { config },
            application_app_usage: ApplicationAppUsageResource { config },
            application_app_version: ApplicationAppVersionResource { config },
            application_collaborators: ApplicationCollaboratorsResource { config },
            application_contacts_range: ApplicationContactsRangeResource { config },
            application_feedback: ApplicationFeedbackResource { config },
            application_management: ApplicationManagementResource { config },
            application_owner: ApplicationOwnerResource { config },
            application_visibility: ApplicationVisibilityResource { config },
            scope: ScopeResource { config },
        }
    }
}
