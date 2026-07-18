use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldGroup {
    #[serde(default)]
    pub writable: Vec<String>,
    #[serde(default)]
    pub readable: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default)]
    pub texts: Vec<I18nResourceText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nResourceText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalInstanceForm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalInstanceLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ExternalInstanceLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceComment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default)]
    pub files: Vec<ApprovalFile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActionConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_need_reason: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_reason_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_need_attachment: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalInstanceCheckResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default)]
    pub tasks: Vec<ExternalInstanceTask>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert_option: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_option: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_approval_option: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_update_viewer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_update_form: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_update_process: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_update_revert: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalForm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_list: Option<Vec<Widget>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Widget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub print_width: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Widget>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<Vec<ApprovalApprover>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccer: Option<Vec<ApprovalCcer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privilege_field: Option<FieldGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalApprover {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalCcer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<ApprovalForm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_list: Option<Vec<ApprovalNode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewers: Option<Vec<ApprovalViewer>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalViewer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalTimeline {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<ApprovalFile>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApprovalInstance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_list: Option<Vec<InstanceTask>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_list: Option<Vec<InstanceComment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Vec<ApprovalTimeline>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_instance_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverted_instance_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_resubmit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_submit_again: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_bot_notification: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_delete_after_revoked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<ExternalInstanceLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalApproval {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewers: Option<Vec<ApprovalViewer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResource>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_link_mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_link_pc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_pc: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_mobile: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_batch_read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_mark_readed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_callback_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_callback_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_batch_operate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_efficiency_statistics: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalInstance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<Vec<ExternalInstanceForm>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_list: Option<Vec<ExternalInstanceTask>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc_list: Option<Vec<CcNode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_schema_list: Option<Vec<crate::JsonValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalInstanceTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ExternalInstanceLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_configs: Option<Vec<ActionConfig>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_statistics: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
}

// ── Additional domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_start_time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_start_time_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_revoked_instance: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_create_time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_create_time_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_revoked_instance: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpecifiedRollback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub task_def_key_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_info_list: Option<Vec<CommentAtInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskResubmit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_time_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateApprovalReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewers: Option<Vec<ApprovalViewer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<ApprovalForm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_list: Option<Vec<ApprovalNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ApprovalSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ApprovalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResource>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_approver_user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_approver_open_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cc_user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cc_open_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_resubmit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_submit_again: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ApproveTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RejectTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TransferTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SubscribeApprovalReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UnsubscribeApprovalReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateExternalInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<Vec<ExternalInstanceForm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<Vec<ExternalInstanceTask>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_list: Option<Vec<CcNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_schema_list: Option<Vec<crate::JsonValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateExternalApprovalReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewers: Option<Vec<ApprovalViewer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResource>>,
}

// ── Additional request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct AddSignInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_sign_user_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_sign_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_method: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CcInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_open_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PreviewInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListExternalTaskReqBody {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub approval_codes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateApprovalData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceListData {
    #[serde(default)]
    pub instance_code_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckExternalInstanceData {
    #[serde(default)]
    pub diff_instance_list: Vec<ExternalInstanceCheckResponse>,
}

impl_resp!(CreateApprovalResp, CreateApprovalData);
impl_resp!(GetApprovalResp, ApprovalDefinition);
impl_resp!(CreateInstanceResp, CreateInstanceData);
impl_resp!(GetInstanceResp, ApprovalInstance);
impl_resp!(ListInstanceResp, InstanceListData);
impl_resp!(CreateExternalApprovalResp, ExternalApproval);
impl_resp!(GetExternalApprovalResp, ExternalApproval);
impl_resp!(CreateExternalInstanceResp, ExternalInstance);
impl_resp!(CheckExternalInstanceResp, CheckExternalInstanceData);

// ── v2 response helpers ──

// ── v2 response types for new methods ──

impl_resp_v2!(AddSignInstanceResp, ());
impl_resp_v2!(CcInstanceResp, ());
impl_resp_v2!(PreviewInstanceResp, PreviewInstanceRespData);
impl_resp_v2!(QueryInstanceResp, QueryInstanceRespData);
impl_resp_v2!(SearchCcInstanceResp, SearchCcInstanceRespData);
impl_resp_v2!(SpecifiedRollbackInstanceResp, ());
impl_resp_v2!(CreateInstanceCommentResp, CreateInstanceCommentRespData);
impl_resp_v2!(DeleteInstanceCommentResp, DeleteInstanceCommentRespData);
impl_resp_v2!(ListInstanceCommentResp, ListInstanceCommentRespData);
impl_resp_v2!(RemoveInstanceCommentResp, RemoveInstanceCommentRespData);
impl_resp_v2!(QueryTaskResp, QueryTaskRespData);
impl_resp_v2!(ResubmitTaskResp, ());
impl_resp_v2!(SearchTaskResp, SearchTaskRespData);
impl_resp_v2!(ListExternalTaskResp, ListExternalTaskRespData);

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetApprovalQuery<'a> {
    pub approval_code: &'a str,
    pub locale: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApprovalQuery<'a> {
    pub fn new(approval_code: &'a str) -> Self {
        Self {
            approval_code,
            locale: None,
            user_id: None,
            user_id_type: None,
        }
    }

    pub fn locale(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.locale = value.into();
        self
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetInstanceQuery<'a> {
    pub instance_id: &'a str,
    pub locale: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetInstanceQuery<'a> {
    pub fn new(instance_id: &'a str) -> Self {
        Self {
            instance_id,
            locale: None,
            user_id: None,
            user_id_type: None,
        }
    }

    pub fn locale(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.locale = value.into();
        self
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListInstanceQuery<'a> {
    pub approval_code: &'a str,
    pub start_time: &'a str,
    pub end_time: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListInstanceQuery<'a> {
    pub fn new(approval_code: &'a str, start_time: &'a str, end_time: &'a str) -> Self {
        Self {
            approval_code,
            start_time,
            end_time,
            page: PageQuery::new(),
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
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
pub struct QueryInstanceQuery<'a> {
    pub body: &'a InstanceSearch,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> QueryInstanceQuery<'a> {
    pub fn new(body: &'a InstanceSearch) -> Self {
        Self {
            body,
            page: PageQuery::new(),
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchCcInstanceQuery<'a> {
    pub body: &'a CcSearch,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SearchCcInstanceQuery<'a> {
    pub fn new(body: &'a CcSearch) -> Self {
        Self {
            body,
            page: PageQuery::new(),
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct QueryTaskQuery<'a> {
    pub user_id: Option<&'a str>,
    pub topic: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> QueryTaskQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn topic(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.topic = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
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
pub struct SearchTaskQuery<'a> {
    pub body: &'a TaskSearch,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> SearchTaskQuery<'a> {
    pub fn new(body: &'a TaskSearch) -> Self {
        Self {
            body,
            user_id_type: None,
            page: PageQuery::new(),
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
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
pub struct GetExternalApprovalQuery<'a> {
    pub approval_code: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetExternalApprovalQuery<'a> {
    pub fn new(approval_code: &'a str) -> Self {
        Self {
            approval_code,
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
pub struct ListInstanceCommentQuery<'a> {
    pub instance_id: &'a str,
    pub user_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListInstanceCommentQuery<'a> {
    pub fn new(instance_id: &'a str) -> Self {
        Self {
            instance_id,
            user_id: None,
            user_id_type: None,
            page: PageQuery::new(),
        }
    }

    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
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
pub struct ListExternalTaskQuery<'a> {
    pub body: &'a ListExternalTaskReqBody,
    pub page: PageQuery<'a>,
}

impl<'a> ListExternalTaskQuery<'a> {
    pub fn new(body: &'a ListExternalTaskReqBody) -> Self {
        Self {
            body,
            page: PageQuery::new(),
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
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

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewInstanceRespData {
    #[serde(default)]
    pub preview_nodes: Vec<PreviewNode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryInstanceRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    pub instance_list: Vec<InstanceSearchItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchCcInstanceRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    pub cc_list: Vec<CcSearchItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceCommentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteInstanceCommentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceCommentRespData {
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveInstanceCommentRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryTaskRespData {
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<Count>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchTaskRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    pub task_list: Vec<TaskSearchItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListExternalTaskRespData {
    #[serde(default)]
    pub data: Vec<ExternalTaskList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcSearchItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<InstanceSearchApproval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<InstanceSearchGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<InstanceSearchNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<CcSearchNode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcSearchNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<InstanceSearchLink>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Comment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_delete: Option<i32>,
    #[serde(default)]
    pub replies: Vec<CommentReply>,
    #[serde(default)]
    pub at_info_list: Vec<CommentAtInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commentator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentAtInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_delete: Option<i32>,
    #[serde(default)]
    pub at_info_list: Vec<CommentAtInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commentator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Count {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalTaskItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalTaskList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default)]
    pub tasks: Vec<ExternalTaskItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceSearchApproval {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<InstanceSearchApprovalExternal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceSearchApprovalExternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_cc_read: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceSearchGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceSearchItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<InstanceSearchApproval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<InstanceSearchGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<InstanceSearchNode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceSearchLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceSearchNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<InstanceSearchLink>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(default)]
    pub comments: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_node_id: Option<String>,
    #[serde(default)]
    pub user_id_list: Vec<String>,
    #[serde(default)]
    pub end_cc_id_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_empty_logic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_approver_type_free: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_cc_type_free: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Task {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<TaskUrls>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_code: Option<String>,
    #[serde(default)]
    pub initiators: Vec<String>,
    #[serde(default)]
    pub initiator_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskSearchItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<InstanceSearchApproval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<InstanceSearchGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<InstanceSearchNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskSearchNode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskSearchNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<InstanceSearchLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_external_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskUrls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc: Option<String>,
}
// ── Resources ──

pub struct ApprovalDefinitionResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalDefinitionResource<'a> {
    pub async fn create(
        &self,
        body: &CreateApprovalReqBody,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateApprovalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/approvals",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", department_id_type)
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<CreateApprovalData, CreateApprovalResp>()
        .await
    }

    pub async fn get(
        &self,
        approval_code: &str,
        locale: Option<&str>,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApprovalResp, LarkError> {
        let query = GetApprovalQuery::new(approval_code)
            .locale(locale)
            .user_id(user_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApprovalQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApprovalResp, LarkError> {
        let path = format!("/open-apis/approval/v4/approvals/{}", query.approval_code);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("locale", query.locale)
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<ApprovalDefinition, GetApprovalResp>()
        .await
    }
}

pub struct ApprovalInstanceResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalInstanceResource<'a> {
    pub async fn create(
        &self,
        body: &CreateInstanceReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<CreateInstanceData, CreateInstanceResp>()
        .await
    }

    pub async fn get(
        &self,
        instance_id: &str,
        locale: Option<&str>,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetInstanceResp, LarkError> {
        let query = GetInstanceQuery::new(instance_id)
            .locale(locale)
            .user_id(user_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetInstanceQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetInstanceResp, LarkError> {
        let path = format!("/open-apis/approval/v4/instances/{}", query.instance_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("locale", query.locale)
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<ApprovalInstance, GetInstanceResp>()
        .await
    }

    pub async fn cancel(
        &self,
        body: &CancelInstanceReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances/cancel",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        approval_code: &str,
        start_time: &str,
        end_time: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInstanceResp, LarkError> {
        let query = ListInstanceQuery::new(approval_code, start_time, end_time)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInstanceQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/approval/v4/instances",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("approval_code", query.approval_code)
        .query("start_time", query.start_time)
        .query("end_time", query.end_time)
        .page_query(query.page)
        .send_response::<InstanceListData, ListInstanceResp>()
        .await
    }

    pub async fn add_sign(
        &self,
        body: &AddSignInstanceReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<AddSignInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances/add_sign",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2_response::<(), AddSignInstanceResp>()
        .await
    }

    pub async fn cc(
        &self,
        body: &CcInstanceReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CcInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances/cc",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2_response::<(), CcInstanceResp>()
        .await
    }

    pub async fn preview(
        &self,
        body: &PreviewInstanceReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PreviewInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances/preview",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2_response::<PreviewInstanceRespData, PreviewInstanceResp>()
        .await
    }

    pub async fn query(
        &self,
        body: &InstanceSearch,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryInstanceResp, LarkError> {
        let query = QueryInstanceQuery::new(body)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryInstanceQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<QueryInstanceRespData, QueryInstanceResp>()
        .await
    }

    pub async fn search_cc(
        &self,
        body: &CcSearch,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchCcInstanceResp, LarkError> {
        let query = SearchCcInstanceQuery::new(body)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.search_cc_by_query(&query, option).await
    }

    pub async fn search_cc_by_query(
        &self,
        query: &SearchCcInstanceQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchCcInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances/search_cc",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<SearchCcInstanceRespData, SearchCcInstanceResp>()
        .await
    }

    pub async fn specified_rollback(
        &self,
        body: &SpecifiedRollback,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SpecifiedRollbackInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/instances/specified_rollback",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2_response::<(), SpecifiedRollbackInstanceResp>()
        .await
    }
}

pub struct ApprovalTaskResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalTaskResource<'a> {
    pub async fn approve(
        &self,
        body: &ApproveTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/tasks/approve",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn reject(
        &self,
        body: &RejectTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/tasks/reject",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn transfer(
        &self,
        body: &TransferTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/tasks/transfer",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn query(
        &self,
        user_id: Option<&str>,
        topic: Option<&str>,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryTaskResp, LarkError> {
        let query = QueryTaskQuery::new()
            .user_id(user_id)
            .topic(topic)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/approval/v4/tasks/query",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id", query.user_id)
        .query("topic", query.topic)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2_response::<QueryTaskRespData, QueryTaskResp>()
        .await
    }

    pub async fn resubmit(
        &self,
        body: &TaskResubmit,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ResubmitTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/tasks/resubmit",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2_response::<(), ResubmitTaskResp>()
        .await
    }

    pub async fn search(
        &self,
        body: &TaskSearch,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchTaskResp, LarkError> {
        let query = SearchTaskQuery::new(body)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/tasks/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .json_body(query.body)?
        .send_v2_response::<SearchTaskRespData, SearchTaskResp>()
        .await
    }
}

pub struct ApprovalSubscribeResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalSubscribeResource<'a> {
    pub async fn subscribe(
        &self,
        approval_code: &str,
        body: &SubscribeApprovalReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/approval/v4/approvals/{approval_code}/subscribe");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn unsubscribe(
        &self,
        approval_code: &str,
        body: &UnsubscribeApprovalReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/approval/v4/approvals/{approval_code}/unsubscribe");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }
}

pub struct ExternalApprovalResource<'a> {
    config: &'a Config,
}

impl<'a> ExternalApprovalResource<'a> {
    pub async fn create(
        &self,
        body: &CreateExternalApprovalReqBody,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateExternalApprovalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/external_approvals",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", department_id_type)
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<ExternalApproval, CreateExternalApprovalResp>()
        .await
    }

    pub async fn get(
        &self,
        approval_code: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetExternalApprovalResp, LarkError> {
        let query = GetExternalApprovalQuery::new(approval_code).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetExternalApprovalQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetExternalApprovalResp, LarkError> {
        let path = format!(
            "/open-apis/approval/v4/external_approvals/{}",
            query.approval_code
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<ExternalApproval, GetExternalApprovalResp>()
        .await
    }
}

pub struct ExternalInstanceResource<'a> {
    config: &'a Config,
}

impl<'a> ExternalInstanceResource<'a> {
    pub async fn create(
        &self,
        body: &CreateExternalInstanceReqBody,
        option: &RequestOption,
    ) -> Result<CreateExternalInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/external_instances",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<ExternalInstance, CreateExternalInstanceResp>()
        .await
    }

    pub async fn check(
        &self,
        instances: &crate::JsonValue,
        option: &RequestOption,
    ) -> Result<CheckExternalInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/approval/v4/external_instances/check",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(instances)?
        .send_response::<CheckExternalInstanceData, CheckExternalInstanceResp>()
        .await
    }
}

// ── InstanceComment resource ──

pub struct InstanceCommentResource<'a> {
    config: &'a Config,
}

impl<'a> InstanceCommentResource<'a> {
    pub async fn create(
        &self,
        instance_id: &str,
        body: &CommentRequest,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateInstanceCommentResp, LarkError> {
        let path = format!("/open-apis/approval/v4/instances/{instance_id}/comments");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", user_id)
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_v2_response::<CreateInstanceCommentRespData, CreateInstanceCommentResp>()
        .await
    }

    pub async fn delete(
        &self,
        instance_id: &str,
        comment_id: &str,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeleteInstanceCommentResp, LarkError> {
        let path = format!("/open-apis/approval/v4/instances/{instance_id}/comments/{comment_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", user_id)
        .query("user_id_type", user_id_type)
        .send_v2_response::<DeleteInstanceCommentRespData, DeleteInstanceCommentResp>()
        .await
    }

    pub async fn list(
        &self,
        instance_id: &str,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInstanceCommentResp, LarkError> {
        let query = ListInstanceCommentQuery::new(instance_id)
            .user_id(user_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInstanceCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListInstanceCommentResp, LarkError> {
        let path = format!(
            "/open-apis/approval/v4/instances/{}/comments",
            query.instance_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2_response::<ListInstanceCommentRespData, ListInstanceCommentResp>()
        .await
    }

    pub async fn remove(
        &self,
        instance_id: &str,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<RemoveInstanceCommentResp, LarkError> {
        let path = format!("/open-apis/approval/v4/instances/{instance_id}/comments/remove");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", user_id)
        .query("user_id_type", user_id_type)
        .send_v2_response::<RemoveInstanceCommentRespData, RemoveInstanceCommentResp>()
        .await
    }
}

// ── ExternalTask resource ──

pub struct ExternalTaskResource<'a> {
    config: &'a Config,
}

impl<'a> ExternalTaskResource<'a> {
    pub async fn list(
        &self,
        body: &ListExternalTaskReqBody,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListExternalTaskResp, LarkError> {
        let query =
            ListExternalTaskQuery::new(body).page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListExternalTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListExternalTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/approval/v4/external_tasks",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .json_body(query.body)?
        .send_v2_response::<ListExternalTaskRespData, ListExternalTaskResp>()
        .await
    }
}

// ── Version struct ──

pub struct V4<'a> {
    pub approval: ApprovalDefinitionResource<'a>,
    pub instance: ApprovalInstanceResource<'a>,
    pub instance_comment: InstanceCommentResource<'a>,
    pub task: ApprovalTaskResource<'a>,
    pub subscribe: ApprovalSubscribeResource<'a>,
    pub external_approval: ExternalApprovalResource<'a>,
    pub external_instance: ExternalInstanceResource<'a>,
    pub external_task: ExternalTaskResource<'a>,
}

impl<'a> V4<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            approval: ApprovalDefinitionResource { config },
            instance: ApprovalInstanceResource { config },
            instance_comment: InstanceCommentResource { config },
            task: ApprovalTaskResource { config },
            subscribe: ApprovalSubscribeResource { config },
            external_approval: ExternalApprovalResource { config },
            external_instance: ExternalInstanceResource { config },
            external_task: ExternalTaskResource { config },
        }
    }
}
