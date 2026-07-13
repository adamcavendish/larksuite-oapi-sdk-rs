use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_tenant_manager: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Badge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_image: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeGrant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_detail: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_grant_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminDeptStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_active: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_total: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_activation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_video: Option<serde_json::Value>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResetPasswordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBadgeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_image: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBadgeGrantReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grant_all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge: Option<Badge>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeListData {
    #[serde(default)]
    pub items: Vec<Badge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeGrantData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant: Option<BadgeGrant>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeGrantListData {
    #[serde(default)]
    pub items: Vec<BadgeGrant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeptStatListData {
    #[serde(default)]
    pub items: Vec<AdminDeptStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateBadgeResp, BadgeData);
impl_resp!(GetBadgeResp, BadgeData);
impl_resp!(ListBadgeResp, BadgeListData);
impl_resp!(CreateBadgeGrantResp, BadgeGrantData);
impl_resp!(GetBadgeGrantResp, BadgeGrantData);
impl_resp!(ListBadgeGrantResp, BadgeGrantListData);
impl_resp!(GetDeptStatResp, DeptStatListData);

// ── impl_resp_v2! macro ──

impl_resp_v2!(UpdateBadgeResp, BadgeData);
impl_resp_v2!(UpdateBadgeGrantResp, BadgeGrantData);
impl_resp_v2!(ListAdminUserStatResp, ListAdminUserStatRespData);
impl_resp_v2!(ListAuditInfoResp, ListAuditInfoRespData);
impl_resp_v2!(CreateBadgeImageResp, CreateBadgeImageRespData);
impl_resp_v2!(ListAdminDeptStatResp, ListAdminDeptStatRespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAdminUserStatRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<AdminUserStat>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAuditInfoRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<AuditInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBadgeImageRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAdminDeptStatRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<ResponseAdminDeptStat>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseAdminDeptStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_user_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suite_dau: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suite_active_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_active_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resign_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub im_dau: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_messenger_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_messenger_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_send_messenger_num: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_dau: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_docs_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_docs_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_create_docs_num: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cal_dau: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_cal_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_cal_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_create_cal_num: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vc_dau: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vc_duration: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_vc_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_dau: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_task_user_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_task_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_create_task_num: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_send_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_receive_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_send_ext_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_receive_ext_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_send_in_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_receive_in_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_active_dau: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_search_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_search_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_search_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminUserStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_active_flag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suite_active_flag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_active_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub im_active_flag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_messenger_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_active_flag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_docs_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cal_active_flag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_cal_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vc_active_flag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vc_duration: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_os: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_task_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vc_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_package_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_send_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_receive_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_send_ext_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_receive_ext_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_send_in_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_receive_in_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_active_flag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_search_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_search_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_search_count: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiAuditCommonDrawers {
    #[serde(default)]
    pub common_draw_info_list: Vec<ApiAuditDrawerInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiAuditDrawerInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info_val: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub val_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub val_i18n_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditAndroidContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_ver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_i: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_r: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hw_brand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hw_manuf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wifip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_iip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_gip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env_su: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env_tz: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env_ml: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_ip_detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_base_station: Option<String>,
    #[serde(default, rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios_context: Option<AuditIosContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_context: Option<AuditPcContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_context: Option<AuditWebContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android_context: Option<AuditAndroidContext>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditEventExtend {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub two_step_validation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_people_num_in_video: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_people_num_in_video: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_people_num_in_chat: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_group: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quit_group: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_people_num_in_doc_share: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(default)]
    pub department_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_module: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_value: Option<String>,
    #[serde(default)]
    pub objects: Vec<AuditObjectEntity>,
    #[serde(default)]
    pub recipients: Vec<AuditRecipientEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_app: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extend: Option<AuditEventExtend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_drawers: Option<ApiAuditCommonDrawers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit_detail: Option<AuditDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_detail: Option<OperatorDetail>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditIosContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_ver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(default, rename = "STZone", skip_serializing_if = "Option::is_none")]
    pub st_zone: Option<String>,
    #[serde(default, rename = "ML", skip_serializing_if = "Option::is_none")]
    pub ml: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sjd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxyip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wifip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_ip_detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_base_station: Option<String>,
    #[serde(default, rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditObjectDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(
        default,
        rename = "third_party_appID",
        skip_serializing_if = "Option::is_none"
    )]
    pub third_party_app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contain_file_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_setting_type: Option<String>,
    #[serde(
        default,
        rename = "permission_external_access_Type",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_external_access_type: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_share_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_service_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_download_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_page: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditObjectEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_detail: Option<AuditObjectDetail>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditPcContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_ver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wifip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditRecipientDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_action_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_flag: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditRecipientEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_detail: Option<AuditRecipientDetail>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditWebContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(default, rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperatorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<OperatorName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperatorName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<I18n>,
}
// ── Resources ──

pub struct PasswordResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ResetPasswordQuery<'a> {
    pub body: &'a ResetPasswordReqBody,
}

impl<'a> ResetPasswordQuery<'a> {
    pub fn new(body: &'a ResetPasswordReqBody) -> Self {
        Self { body }
    }
}

impl<'a> PasswordResource<'a> {
    pub async fn reset(
        &self,
        body: &ResetPasswordReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = ResetPasswordQuery::new(body);
        self.reset_by_query(&query, option).await
    }

    pub async fn reset_by_query(
        &self,
        query: &ResetPasswordQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/admin/v1/password/reset",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_empty()
        .await
    }
}

pub struct BadgeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateBadgeQuery<'a> {
    pub body: &'a CreateBadgeReqBody,
}

impl<'a> CreateBadgeQuery<'a> {
    pub fn new(body: &'a CreateBadgeReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetBadgeQuery<'a> {
    pub badge_id: &'a str,
}

impl<'a> GetBadgeQuery<'a> {
    pub fn new(badge_id: &'a str) -> Self {
        Self { badge_id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListBadgeQuery<'a> {
    pub page: PageQuery<'a>,
    pub name: Option<&'a str>,
}

impl<'a> ListBadgeQuery<'a> {
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

    pub fn name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.name = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateBadgeQuery<'a> {
    pub badge_id: &'a str,
    pub body: &'a CreateBadgeReqBody,
}

impl<'a> UpdateBadgeQuery<'a> {
    pub fn new(badge_id: &'a str, body: &'a CreateBadgeReqBody) -> Self {
        Self { badge_id, body }
    }
}

impl<'a> BadgeResource<'a> {
    pub async fn create(
        &self,
        body: &CreateBadgeReqBody,
        option: &RequestOption,
    ) -> Result<CreateBadgeResp, LarkError> {
        let query = CreateBadgeQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateBadgeQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateBadgeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/admin/v1/badges",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<BadgeData, CreateBadgeResp>()
        .await
    }

    pub async fn get(
        &self,
        badge_id: &str,
        option: &RequestOption,
    ) -> Result<GetBadgeResp, LarkError> {
        let query = GetBadgeQuery::new(badge_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetBadgeQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBadgeResp, LarkError> {
        let path = format!("/open-apis/admin/v1/badges/{}", query.badge_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<BadgeData, GetBadgeResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        name: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBadgeResp, LarkError> {
        let query = ListBadgeQuery {
            page: PageQuery::from_parts(page_size, page_token),
            name,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListBadgeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListBadgeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/admin/v1/badges",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("name", query.name)
        .send_response::<BadgeListData, ListBadgeResp>()
        .await
    }

    pub async fn update(
        &self,
        badge_id: &str,
        body: &CreateBadgeReqBody,
        option: &RequestOption,
    ) -> Result<UpdateBadgeResp, LarkError> {
        let query = UpdateBadgeQuery::new(badge_id, body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateBadgeQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateBadgeResp, LarkError> {
        let path = format!("/open-apis/admin/v1/badges/{}", query.badge_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<BadgeData, UpdateBadgeResp>()
        .await
    }
}

pub struct BadgeGrantResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateBadgeGrantQuery<'a> {
    pub badge_id: &'a str,
    pub body: &'a CreateBadgeGrantReqBody,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> CreateBadgeGrantQuery<'a> {
    pub fn new(badge_id: &'a str, body: &'a CreateBadgeGrantReqBody) -> Self {
        Self {
            badge_id,
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
pub struct GetBadgeGrantQuery<'a> {
    pub badge_id: &'a str,
    pub grant_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> GetBadgeGrantQuery<'a> {
    pub fn new(badge_id: &'a str, grant_id: &'a str) -> Self {
        Self {
            badge_id,
            grant_id,
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
pub struct DeleteBadgeGrantQuery<'a> {
    pub badge_id: &'a str,
    pub grant_id: &'a str,
}

impl<'a> DeleteBadgeGrantQuery<'a> {
    pub fn new(badge_id: &'a str, grant_id: &'a str) -> Self {
        Self { badge_id, grant_id }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListBadgeGrantQuery<'a> {
    pub badge_id: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub name: Option<&'a str>,
}

impl<'a> ListBadgeGrantQuery<'a> {
    pub fn new(badge_id: &'a str) -> Self {
        Self {
            badge_id,
            page: PageQuery::new(),
            user_id_type: None,
            department_id_type: None,
            name: None,
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.name = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateBadgeGrantQuery<'a> {
    pub badge_id: &'a str,
    pub grant_id: &'a str,
    pub body: &'a CreateBadgeGrantReqBody,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> UpdateBadgeGrantQuery<'a> {
    pub fn new(badge_id: &'a str, grant_id: &'a str, body: &'a CreateBadgeGrantReqBody) -> Self {
        Self {
            badge_id,
            grant_id,
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

impl<'a> BadgeGrantResource<'a> {
    pub async fn create(
        &self,
        badge_id: &str,
        body: &CreateBadgeGrantReqBody,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateBadgeGrantResp, LarkError> {
        let query = CreateBadgeGrantQuery::new(badge_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateBadgeGrantQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateBadgeGrantResp, LarkError> {
        let path = format!("/open-apis/admin/v1/badges/{}/grants", query.badge_id);
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
        .send_response::<BadgeGrantData, CreateBadgeGrantResp>()
        .await
    }

    pub async fn get(
        &self,
        badge_id: &str,
        grant_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBadgeGrantResp, LarkError> {
        let query = GetBadgeGrantQuery::new(badge_id, grant_id)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetBadgeGrantQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBadgeGrantResp, LarkError> {
        let path = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            query.badge_id, query.grant_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .send_response::<BadgeGrantData, GetBadgeGrantResp>()
        .await
    }

    pub async fn delete(
        &self,
        badge_id: &str,
        grant_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteBadgeGrantQuery::new(badge_id, grant_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteBadgeGrantQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            query.badge_id, query.grant_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_empty()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        badge_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        name: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBadgeGrantResp, LarkError> {
        let query = ListBadgeGrantQuery {
            badge_id,
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
            department_id_type,
            name,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListBadgeGrantQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListBadgeGrantResp, LarkError> {
        let path = format!("/open-apis/admin/v1/badges/{}/grants", query.badge_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("name", query.name)
        .send_response::<BadgeGrantListData, ListBadgeGrantResp>()
        .await
    }

    pub async fn update(
        &self,
        badge_id: &str,
        grant_id: &str,
        body: &CreateBadgeGrantReqBody,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateBadgeGrantResp, LarkError> {
        let query = UpdateBadgeGrantQuery::new(badge_id, grant_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateBadgeGrantQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateBadgeGrantResp, LarkError> {
        let path = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            query.badge_id, query.grant_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<BadgeGrantData, UpdateBadgeGrantResp>()
        .await
    }
}

pub struct AdminDeptStatResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetAdminDeptStatQuery<'a> {
    pub department_id_type: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub department_id: &'a str,
    pub contains_child_dept: bool,
    pub page: PageQuery<'a>,
}

impl<'a> GetAdminDeptStatQuery<'a> {
    pub fn new(
        department_id_type: &'a str,
        start_date: &'a str,
        end_date: &'a str,
        department_id: &'a str,
        contains_child_dept: bool,
    ) -> Self {
        Self {
            department_id_type,
            start_date,
            end_date,
            department_id,
            contains_child_dept,
            page: PageQuery::new(),
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
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListAdminDeptStatQuery<'a> {
    pub department_id_type: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub department_id: &'a str,
    pub contains_child_dept: bool,
    pub page: PageQuery<'a>,
    pub target_geo: Option<&'a str>,
    pub with_product_version: Option<bool>,
}

impl<'a> ListAdminDeptStatQuery<'a> {
    pub fn new(
        department_id_type: &'a str,
        start_date: &'a str,
        end_date: &'a str,
        department_id: &'a str,
        contains_child_dept: bool,
    ) -> Self {
        Self {
            department_id_type,
            start_date,
            end_date,
            department_id,
            contains_child_dept,
            page: PageQuery::new(),
            target_geo: None,
            with_product_version: None,
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

    pub fn target_geo(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.target_geo = value.into();
        self
    }

    pub fn with_product_version(mut self, value: impl Into<Option<bool>>) -> Self {
        self.with_product_version = value.into();
        self
    }
}

impl<'a> AdminDeptStatResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        department_id_type: &str,
        start_date: &str,
        end_date: &str,
        department_id: &str,
        contains_child_dept: bool,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetDeptStatResp, LarkError> {
        let query = GetAdminDeptStatQuery {
            department_id_type,
            start_date,
            end_date,
            department_id,
            contains_child_dept,
            page: PageQuery::from_parts(page_size, page_token),
        };
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetAdminDeptStatQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDeptStatResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/admin/v1/dept_stats",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .query("start_date", query.start_date)
        .query("end_date", query.end_date)
        .query("department_id", query.department_id)
        .query("contains_child_dept", query.contains_child_dept)
        .page_query(query.page)
        .send_response::<DeptStatListData, GetDeptStatResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        department_id_type: &str,
        start_date: &str,
        end_date: &str,
        department_id: &str,
        contains_child_dept: bool,
        page_size: Option<i32>,
        page_token: Option<&str>,
        target_geo: Option<&str>,
        with_product_version: Option<bool>,
        option: &RequestOption,
    ) -> Result<ListAdminDeptStatResp, LarkError> {
        let query = ListAdminDeptStatQuery {
            department_id_type,
            start_date,
            end_date,
            department_id,
            contains_child_dept,
            page: PageQuery::from_parts(page_size, page_token),
            target_geo,
            with_product_version,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAdminDeptStatQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAdminDeptStatResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/admin/v1/admin_dept_stats",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .query("start_date", query.start_date)
        .query("end_date", query.end_date)
        .query("department_id", query.department_id)
        .query("contains_child_dept", query.contains_child_dept)
        .page_query(query.page)
        .query("target_geo", query.target_geo)
        .query("with_product_version", query.with_product_version)
        .send_v2_response::<ListAdminDeptStatRespData, ListAdminDeptStatResp>()
        .await
    }
}

pub struct AdminUserStatResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListAdminUserStatQuery<'a> {
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub department_id: Option<&'a str>,
    pub user_id: Option<&'a str>,
}

impl<'a> ListAdminUserStatQuery<'a> {
    pub fn new(start_date: &'a str, end_date: &'a str) -> Self {
        Self {
            start_date,
            end_date,
            page: PageQuery::new(),
            user_id_type: None,
            department_id_type: None,
            department_id: None,
            user_id: None,
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn department_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id = value.into();
        self
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }
}

impl<'a> AdminUserStatResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        start_date: &str,
        end_date: &str,
        department_id: Option<&str>,
        user_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAdminUserStatResp, LarkError> {
        let query = ListAdminUserStatQuery {
            start_date,
            end_date,
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
            department_id_type,
            department_id,
            user_id,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAdminUserStatQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAdminUserStatResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/admin/v1/admin_user_stats",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("start_date", query.start_date)
        .query("end_date", query.end_date)
        .query("department_id", query.department_id)
        .query("user_id", query.user_id)
        .page_query(query.page)
        .send_v2_response::<ListAdminUserStatRespData, ListAdminUserStatResp>()
        .await
    }
}

pub struct AuditInfoResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListAuditInfoQuery<'a> {
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
    pub latest: Option<&'a str>,
    pub oldest: Option<&'a str>,
    pub event_name: Option<&'a str>,
    pub operator_type: Option<&'a str>,
    pub operator_value: Option<&'a str>,
    pub event_module: Option<i32>,
}

impl<'a> ListAuditInfoQuery<'a> {
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

    pub fn latest(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.latest = value.into();
        self
    }

    pub fn oldest(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.oldest = value.into();
        self
    }

    pub fn event_name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.event_name = value.into();
        self
    }

    pub fn operator_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_type = value.into();
        self
    }

    pub fn operator_value(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_value = value.into();
        self
    }

    pub fn event_module(mut self, value: impl Into<Option<i32>>) -> Self {
        self.event_module = value.into();
        self
    }
}

impl<'a> AuditInfoResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        latest: Option<&str>,
        oldest: Option<&str>,
        event_name: Option<&str>,
        operator_type: Option<&str>,
        operator_value: Option<&str>,
        event_module: Option<i32>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAuditInfoResp, LarkError> {
        let query = ListAuditInfoQuery {
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
            latest,
            oldest,
            event_name,
            operator_type,
            operator_value,
            event_module,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAuditInfoQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAuditInfoResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/admin/v1/audit_infos",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("latest", query.latest)
        .query("oldest", query.oldest)
        .query("event_name", query.event_name)
        .query("operator_type", query.operator_type)
        .query("operator_value", query.operator_value)
        .query("event_module", query.event_module)
        .page_query(query.page)
        .send_v2_response::<ListAuditInfoRespData, ListAuditInfoResp>()
        .await
    }
}

pub struct BadgeImageResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateBadgeImageQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreateBadgeImageQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

impl<'a> BadgeImageResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateBadgeImageResp, LarkError> {
        let query = CreateBadgeImageQuery::new(&body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateBadgeImageQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateBadgeImageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/admin/v1/badge_images",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CreateBadgeImageRespData, CreateBadgeImageResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub password: PasswordResource<'a>,
    pub badge: BadgeResource<'a>,
    pub badge_grant: BadgeGrantResource<'a>,
    pub dept_stat: AdminDeptStatResource<'a>,
    pub user_stat: AdminUserStatResource<'a>,
    pub audit_info: AuditInfoResource<'a>,
    pub badge_image: BadgeImageResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            password: PasswordResource { config },
            badge: BadgeResource { config },
            badge_grant: BadgeGrantResource { config },
            dept_stat: AdminDeptStatResource { config },
            user_stat: AdminUserStatResource { config },
            audit_info: AuditInfoResource { config },
            badge_image: BadgeImageResource { config },
        }
    }
}
