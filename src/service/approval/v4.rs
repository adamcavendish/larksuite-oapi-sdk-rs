use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

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
    pub option: Option<serde_json::Value>,
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
    pub privilege_field: Option<serde_json::Value>,
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
    pub files: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
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
    pub comment_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Vec<ApprovalTimeline>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_instance_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverted_instance_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_resubmit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_submit_again: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_bot_notification: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_delete_after_revoked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<serde_json::Value>,
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
    pub i18n_resources: Option<Vec<serde_json::Value>>,
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
    pub form: Option<Vec<serde_json::Value>>,
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
    pub cc_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_schema_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<serde_json::Value>>,
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
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_configs: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_statistics: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
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
    pub settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<serde_json::Value>>,
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
    pub node_approver_user_id_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_approver_open_id_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cc_user_id_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cc_open_id_list: Option<Vec<serde_json::Value>>,
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
    pub form: Option<Vec<serde_json::Value>>,
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
    pub cc_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_schema_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<serde_json::Value>>,
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
    pub i18n_resources: Option<Vec<serde_json::Value>>,
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

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

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
    pub diff_instance_list: Vec<serde_json::Value>,
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
    ) -> Result<CreateApprovalResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/approval/v4/approvals");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateApprovalData>(self.config, &api_req, option).await?;
        Ok(CreateApprovalResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        approval_code: &str,
        locale: Option<&str>,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApprovalResp> {
        let path = format!("/open-apis/approval/v4/approvals/{approval_code}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = locale {
            api_req.query_params.set("locale", v);
        }
        if let Some(v) = user_id {
            api_req.query_params.set("user_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ApprovalDefinition>(self.config, &api_req, option).await?;
        Ok(GetApprovalResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreateInstanceResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/approval/v4/instances");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateInstanceData>(self.config, &api_req, option).await?;
        Ok(CreateInstanceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        instance_id: &str,
        locale: Option<&str>,
        user_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetInstanceResp> {
        let path = format!("/open-apis/approval/v4/instances/{instance_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = locale {
            api_req.query_params.set("locale", v);
        }
        if let Some(v) = user_id {
            api_req.query_params.set("user_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ApprovalInstance>(self.config, &api_req, option).await?;
        Ok(GetInstanceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn cancel(
        &self,
        instance_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/approval/v4/instances/{instance_id}/cancel");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        approval_code: &str,
        start_time: &str,
        end_time: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInstanceResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/approval/v4/instances");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("approval_code", approval_code);
        api_req.query_params.set("start_time", start_time);
        api_req.query_params.set("end_time", end_time);
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<InstanceListData>(self.config, &api_req, option).await?;
        Ok(ListInstanceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/approval/v4/tasks/approve");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn reject(
        &self,
        body: &RejectTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/approval/v4/tasks/reject");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn transfer(
        &self,
        body: &TransferTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/approval/v4/tasks/transfer");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct ApprovalSubscribeResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalSubscribeResource<'a> {
    pub async fn subscribe(
        &self,
        body: &SubscribeApprovalReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/approval/v4/subscribes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscribe(
        &self,
        body: &UnsubscribeApprovalReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::DELETE, "/open-apis/approval/v4/subscribes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
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
    ) -> Result<CreateExternalApprovalResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/approval/v4/external_approvals",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ExternalApproval>(self.config, &api_req, option).await?;
        Ok(CreateExternalApprovalResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        approval_code: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetExternalApprovalResp> {
        let path = format!("/open-apis/approval/v4/external_approvals/{approval_code}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ExternalApproval>(self.config, &api_req, option).await?;
        Ok(GetExternalApprovalResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreateExternalInstanceResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/approval/v4/external_instances",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ExternalInstance>(self.config, &api_req, option).await?;
        Ok(CreateExternalInstanceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn check(
        &self,
        instances: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CheckExternalInstanceResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/approval/v4/external_instances/check",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(instances)?);
        let (api_resp, raw) =
            transport::request_typed::<CheckExternalInstanceData>(self.config, &api_req, option)
                .await?;
        Ok(CheckExternalInstanceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V4<'a> {
    pub approval: ApprovalDefinitionResource<'a>,
    pub instance: ApprovalInstanceResource<'a>,
    pub task: ApprovalTaskResource<'a>,
    pub subscribe: ApprovalSubscribeResource<'a>,
    pub external_approval: ExternalApprovalResource<'a>,
    pub external_instance: ExternalInstanceResource<'a>,
}

impl<'a> V4<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            approval: ApprovalDefinitionResource { config },
            instance: ApprovalInstanceResource { config },
            task: ApprovalTaskResource { config },
            subscribe: ApprovalSubscribeResource { config },
            external_approval: ExternalApprovalResource { config },
            external_instance: ExternalInstanceResource { config },
        }
    }
}
