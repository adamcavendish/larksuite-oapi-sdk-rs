use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyResp, parse_v2};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct App {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_public_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<AppTableFieldDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableFieldDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_sync: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dashboard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAppReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTableReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<AppTable>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateTableReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<AppTable>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchTableReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFieldReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AppTableFieldDescription>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFieldReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AppTableFieldDescription>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<AppTableRecord>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchUpdateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<AppTableRecord>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeleteRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<String>>,
}

// ── Additional domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleTableRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_perm: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_perm: Option<std::collections::HashMap<String, i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_add_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_delete_record: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleBlockRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_perm: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<AppRoleTableRole>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<AppRoleBlockRole>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleMemberId {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppWorkflow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableForm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_limit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit_limit_once: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableFormField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableFormPatchedField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Condition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChildrenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChildrenFilter>>,
}

// ── Additional request body types ──

/// Body for `app.copy`
#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyAppReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// Body for `app.create`
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAppReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// Body for `app_role.create` / `app_role.update`
#[derive(Debug, Clone, Default, Serialize)]
pub struct AppRoleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<AppRoleTableRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<AppRoleBlockRole>>,
}

/// Body for `app_role_member.batch_create` and `app_role_member.batch_delete`
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchAppRoleMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_list: Option<Vec<AppRoleMemberId>>,
}

/// Body for `app_role_member.create`
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAppRoleMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
}

/// Body for `app_workflow.update`
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateWorkflowReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Body for `app_dashboard.copy`
#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyAppDashboardReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Body for `app_table_record.batch_get`
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchGetRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_shared_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fields: Option<bool>,
}

/// Body for `app_table_record.search`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Sort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fields: Option<bool>,
}

/// Body for `app_table_form.patch`
#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchAppTableFormReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_limit_once: Option<bool>,
}

/// Body for `app_table_form_field.patch`
#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchAppTableFormFieldReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

// ── Additional data types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<AppRole>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleListData {
    #[serde(default)]
    pub items: Vec<AppRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleMemberListData {
    #[serde(default)]
    pub items: Vec<AppRoleMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowListData {
    #[serde(default)]
    pub workflows: Vec<AppWorkflow>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateTableData {
    #[serde(default)]
    pub table_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableListData {
    #[serde(default)]
    pub items: Vec<AppTable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<AppTableView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewListData {
    #[serde(default)]
    pub items: Vec<AppTableView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<AppTableField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldListData {
    #[serde(default)]
    pub items: Vec<AppTableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<AppTableRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordListData {
    #[serde(default)]
    pub items: Vec<AppTableRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateRecordData {
    #[serde(default)]
    pub records: Vec<AppTableRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateRecordData {
    #[serde(default)]
    pub records: Vec<AppTableRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDeleteRecordData {
    #[serde(default)]
    pub records: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardListData {
    #[serde(default)]
    pub dashboards: Vec<Dashboard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CopyAppDashboardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchGetRecordData {
    #[serde(default)]
    pub records: Vec<AppTableRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forbidden_record_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absent_record_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchRecordData {
    #[serde(default)]
    pub items: Vec<AppTableRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<AppTableForm>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldListData {
    #[serde(default)]
    pub items: Vec<AppTableFormField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormFieldPatchData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<AppTableFormPatchedField>,
}

impl_resp!(GetAppResp, AppData);
impl_resp!(UpdateAppResp, AppData);
impl_resp!(CreateTableResp, TableData);
impl_resp!(BatchCreateTableResp, BatchCreateTableData);
impl_resp!(ListTableResp, TableListData);
impl_resp!(CreateViewResp, ViewData);
impl_resp!(GetViewResp, ViewData);
impl_resp!(PatchViewResp, ViewData);
impl_resp!(ListViewResp, ViewListData);
impl_resp!(CreateFieldResp, FieldData);
impl_resp!(UpdateFieldResp, FieldData);
impl_resp!(ListFieldResp, FieldListData);
impl_resp!(CreateRecordResp, RecordData);
impl_resp!(UpdateRecordResp, RecordData);
impl_resp!(GetRecordResp, RecordData);
impl_resp!(ListRecordResp, RecordListData);
impl_resp!(BatchCreateRecordResp, BatchCreateRecordData);
impl_resp!(BatchUpdateRecordResp, BatchUpdateRecordData);
impl_resp!(BatchDeleteRecordResp, BatchDeleteRecordData);
impl_resp!(ListDashboardResp, DashboardListData);
impl_resp!(CopyAppDashboardResp, CopyAppDashboardData);
impl_resp!(BatchGetRecordResp, BatchGetRecordData);
impl_resp!(SearchRecordResp, SearchRecordData);
impl_resp!(GetFormResp, FormData);
impl_resp!(PatchFormResp, FormData);
impl_resp!(ListFormFieldResp, FormFieldListData);
impl_resp!(PatchFormFieldResp, FormFieldPatchData);

// ── v2 helpers (Option<CodeError>, used for newer endpoints) ──

// app.copy / app.create
impl_resp_v2!(CopyAppResp, serde_json::Value);
impl_resp_v2!(CreateAppResp, serde_json::Value);

// app_role
impl_resp_v2!(CreateAppRoleResp, AppRoleData);
impl_resp_v2!(DeleteAppRoleResp, ());
impl_resp_v2!(ListAppRoleResp, AppRoleListData);
impl_resp_v2!(UpdateAppRoleResp, AppRoleData);

// app_role_member
impl_resp_v2!(BatchCreateAppRoleMemberResp, ());
impl_resp_v2!(BatchDeleteAppRoleMemberResp, ());
impl_resp_v2!(CreateAppRoleMemberResp, ());
impl_resp_v2!(DeleteAppRoleMemberResp, ());
impl_resp_v2!(ListAppRoleMemberResp, AppRoleMemberListData);

// app_workflow
impl_resp_v2!(ListAppWorkflowResp, WorkflowListData);
impl_resp_v2!(UpdateAppWorkflowResp, ());

// ── Resources ──

pub struct AppResource<'a> {
    config: &'a Config,
}

impl<'a> AppResource<'a> {
    /// Copy a bitable app.
    /// POST /open-apis/bitable/v1/apps/:app_token/copy
    pub async fn copy(
        &self,
        app_token: &str,
        body: &CopyAppReqBody,
        option: &RequestOption,
    ) -> Result<CopyAppResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/copy");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CopyAppResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Create a new bitable app.
    /// POST /open-apis/bitable/v1/apps
    pub async fn create(
        &self,
        body: &CreateAppReqBody,
        option: &RequestOption,
    ) -> Result<CreateAppResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/bitable/v1/apps");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateAppResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, app_token: &str, option: &RequestOption) -> Result<GetAppResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<AppData>(self.config, &api_req, option).await?;
        Ok(GetAppResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        app_token: &str,
        body: &UpdateAppReqBody,
        option: &RequestOption,
    ) -> Result<UpdateAppResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<AppData>(self.config, &api_req, option).await?;
        Ok(UpdateAppResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        body: &CreateTableReqBody,
        option: &RequestOption,
    ) -> Result<CreateTableResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TableData>(self.config, &api_req, option).await?;
        Ok(CreateTableResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_create(
        &self,
        app_token: &str,
        body: &BatchCreateTableReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchCreateTableResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/batch_create");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchCreateTableData>(self.config, &api_req, option).await?;
        Ok(BatchCreateTableResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn batch_delete(
        &self,
        app_token: &str,
        table_ids: &[&str],
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let body = serde_json::json!({ "table_ids": table_ids });
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn patch(
        &self,
        app_token: &str,
        table_id: &str,
        body: &PatchTableReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTableResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TableListData>(self.config, &api_req, option).await?;
        Ok(ListTableResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableViewResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableViewResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &CreateViewReqBody,
        option: &RequestOption,
    ) -> Result<CreateViewResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ViewData>(self.config, &api_req, option).await?;
        Ok(CreateViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: &str,
        option: &RequestOption,
    ) -> Result<GetViewResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<ViewData>(self.config, &api_req, option).await?;
        Ok(GetViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: &str,
        body: &PatchViewReqBody,
        option: &RequestOption,
    ) -> Result<PatchViewResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ViewData>(self.config, &api_req, option).await?;
        Ok(PatchViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        app_token: &str,
        table_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListViewResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ViewListData>(self.config, &api_req, option).await?;
        Ok(ListViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableFieldResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableFieldResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &CreateFieldReqBody,
        option: &RequestOption,
    ) -> Result<CreateFieldResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FieldData>(self.config, &api_req, option).await?;
        Ok(CreateFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        app_token: &str,
        table_id: &str,
        field_id: &str,
        body: &UpdateFieldReqBody,
        option: &RequestOption,
    ) -> Result<UpdateFieldResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FieldData>(self.config, &api_req, option).await?;
        Ok(UpdateFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        field_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: Option<&str>,
        text_field_as_array: Option<bool>,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListFieldResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = view_id {
            api_req.query_params.set("view_id", v);
        }
        if let Some(v) = text_field_as_array {
            api_req
                .query_params
                .set("text_field_as_array", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<FieldListData>(self.config, &api_req, option).await?;
        Ok(ListFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableRecordResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableRecordResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &CreateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateRecordResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RecordData>(self.config, &api_req, option).await?;
        Ok(CreateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        app_token: &str,
        table_id: &str,
        record_id: &str,
        body: &UpdateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateRecordResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RecordData>(self.config, &api_req, option).await?;
        Ok(UpdateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        app_token: &str,
        table_id: &str,
        record_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRecordResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RecordData>(self.config, &api_req, option).await?;
        Ok(GetRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        record_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: Option<&str>,
        filter: Option<&str>,
        sort: Option<&str>,
        field_names: Option<&str>,
        text_field_as_array: Option<bool>,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListRecordResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = view_id {
            api_req.query_params.set("view_id", v);
        }
        if let Some(v) = filter {
            api_req.query_params.set("filter", v);
        }
        if let Some(v) = sort {
            api_req.query_params.set("sort", v);
        }
        if let Some(v) = field_names {
            api_req.query_params.set("field_names", v);
        }
        if let Some(v) = text_field_as_array {
            api_req
                .query_params
                .set("text_field_as_array", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<RecordListData>(self.config, &api_req, option).await?;
        Ok(ListRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &BatchCreateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchCreateRecordResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchCreateRecordData>(self.config, &api_req, option)
                .await?;
        Ok(BatchCreateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_update(
        &self,
        app_token: &str,
        table_id: &str,
        body: &BatchUpdateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchUpdateRecordResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchUpdateRecordData>(self.config, &api_req, option)
                .await?;
        Ok(BatchUpdateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_delete(
        &self,
        app_token: &str,
        table_id: &str,
        body: &BatchDeleteRecordReqBody,
        option: &RequestOption,
    ) -> Result<BatchDeleteRecordResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchDeleteRecordData>(self.config, &api_req, option)
                .await?;
        Ok(BatchDeleteRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    /// Batch-get records.
    /// POST /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_get
    pub async fn batch_get(
        &self,
        app_token: &str,
        table_id: &str,
        body: &BatchGetRecordReqBody,
        option: &RequestOption,
    ) -> Result<BatchGetRecordResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchGetRecordData>(self.config, &api_req, option).await?;
        Ok(BatchGetRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    /// Search records.
    /// POST /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/search
    #[allow(clippy::too_many_arguments)]
    pub async fn search(
        &self,
        app_token: &str,
        table_id: &str,
        body: &SearchRecordReqBody,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<SearchRecordResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SearchRecordData>(self.config, &api_req, option).await?;
        Ok(SearchRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppDashboardResource<'a> {
    config: &'a Config,
}

impl<'a> AppDashboardResource<'a> {
    /// Copy a dashboard.
    /// POST /open-apis/bitable/v1/apps/:app_token/dashboards/:block_id/copy
    pub async fn copy(
        &self,
        app_token: &str,
        block_id: &str,
        body: &CopyAppDashboardReqBody,
        option: &RequestOption,
    ) -> Result<CopyAppDashboardResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CopyAppDashboardData>(self.config, &api_req, option).await?;
        Ok(CopyAppDashboardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDashboardResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/dashboards");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DashboardListData>(self.config, &api_req, option).await?;
        Ok(ListDashboardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── AppRoleResource ──

pub struct AppRoleResource<'a> {
    config: &'a Config,
}

impl<'a> AppRoleResource<'a> {
    /// Create a custom role.
    /// POST /open-apis/bitable/v1/apps/:app_token/roles
    pub async fn create(
        &self,
        app_token: &str,
        body: &AppRoleReqBody,
        option: &RequestOption,
    ) -> Result<CreateAppRoleResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/roles");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<AppRoleData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateAppRoleResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Delete a custom role.
    /// DELETE /open-apis/bitable/v1/apps/:app_token/roles/:role_id
    pub async fn delete(
        &self,
        app_token: &str,
        role_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteAppRoleResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteAppRoleResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// List custom roles.
    /// GET /open-apis/bitable/v1/apps/:app_token/roles
    pub async fn list(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppRoleResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/roles");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AppRoleListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAppRoleResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Update a custom role (full replacement).
    /// PUT /open-apis/bitable/v1/apps/:app_token/roles/:role_id
    pub async fn update(
        &self,
        app_token: &str,
        role_id: &str,
        body: &AppRoleReqBody,
        option: &RequestOption,
    ) -> Result<UpdateAppRoleResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<AppRoleData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateAppRoleResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── AppRoleMemberResource ──

pub struct AppRoleMemberResource<'a> {
    config: &'a Config,
}

impl<'a> AppRoleMemberResource<'a> {
    /// Batch-create members for a custom role.
    /// POST /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_create
    pub async fn batch_create(
        &self,
        app_token: &str,
        role_id: &str,
        body: &BatchAppRoleMemberReqBody,
        option: &RequestOption,
    ) -> Result<BatchCreateAppRoleMemberResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateAppRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Batch-delete members from a custom role.
    /// POST /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_delete
    pub async fn batch_delete(
        &self,
        app_token: &str,
        role_id: &str,
        body: &BatchAppRoleMemberReqBody,
        option: &RequestOption,
    ) -> Result<BatchDeleteAppRoleMemberResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchDeleteAppRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Add a single member to a custom role.
    /// POST /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members
    pub async fn create(
        &self,
        app_token: &str,
        role_id: &str,
        body: &CreateAppRoleMemberReqBody,
        member_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateAppRoleMemberResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = member_id_type {
            api_req.query_params.set("member_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateAppRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Remove a single member from a custom role.
    /// DELETE /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/:member_id
    pub async fn delete(
        &self,
        app_token: &str,
        role_id: &str,
        member_id: &str,
        member_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeleteAppRoleMemberResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = member_id_type {
            api_req.query_params.set("member_id_type", v);
        }
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteAppRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// List members of a custom role.
    /// GET /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members
    pub async fn list(
        &self,
        app_token: &str,
        role_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppRoleMemberResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AppRoleMemberListData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAppRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── AppWorkflowResource ──

pub struct AppWorkflowResource<'a> {
    config: &'a Config,
}

impl<'a> AppWorkflowResource<'a> {
    /// List automation workflows for a bitable app.
    /// GET /open-apis/bitable/v1/apps/:app_token/workflows
    pub async fn list(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppWorkflowResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/workflows");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<WorkflowListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAppWorkflowResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Update the enabled/disabled status of an automation workflow.
    /// PUT /open-apis/bitable/v1/apps/:app_token/workflows/:workflow_id
    pub async fn update(
        &self,
        app_token: &str,
        workflow_id: &str,
        body: &UpdateWorkflowReqBody,
        option: &RequestOption,
    ) -> Result<UpdateAppWorkflowResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateAppWorkflowResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── AppTableFormResource ──

pub struct AppTableFormResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableFormResource<'a> {
    /// Get form metadata.
    /// GET /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id
    pub async fn get(
        &self,
        app_token: &str,
        table_id: &str,
        form_id: &str,
        option: &RequestOption,
    ) -> Result<GetFormResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<FormData>(self.config, &api_req, option).await?;
        Ok(GetFormResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    /// Patch form metadata.
    /// PATCH /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id
    pub async fn patch(
        &self,
        app_token: &str,
        table_id: &str,
        form_id: &str,
        body: &PatchAppTableFormReqBody,
        option: &RequestOption,
    ) -> Result<PatchFormResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FormData>(self.config, &api_req, option).await?;
        Ok(PatchFormResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── AppTableFormFieldResource ──

pub struct AppTableFormFieldResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableFormFieldResource<'a> {
    /// List form fields.
    /// GET /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields
    pub async fn list(
        &self,
        app_token: &str,
        table_id: &str,
        form_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFormFieldResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/fields"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<FormFieldListData>(self.config, &api_req, option).await?;
        Ok(ListFormFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    /// Patch a form field.
    /// PATCH /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields/:field_id
    pub async fn patch(
        &self,
        app_token: &str,
        table_id: &str,
        form_id: &str,
        field_id: &str,
        body: &PatchAppTableFormFieldReqBody,
        option: &RequestOption,
    ) -> Result<PatchFormFieldResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/fields/{field_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FormFieldPatchData>(self.config, &api_req, option).await?;
        Ok(PatchFormFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub app: AppResource<'a>,
    pub table: AppTableResource<'a>,
    pub view: AppTableViewResource<'a>,
    pub field: AppTableFieldResource<'a>,
    pub record: AppTableRecordResource<'a>,
    pub dashboard: AppDashboardResource<'a>,
    pub role: AppRoleResource<'a>,
    pub role_member: AppRoleMemberResource<'a>,
    pub workflow: AppWorkflowResource<'a>,
    pub form: AppTableFormResource<'a>,
    pub form_field: AppTableFormFieldResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app: AppResource { config },
            table: AppTableResource { config },
            view: AppTableViewResource { config },
            field: AppTableFieldResource { config },
            record: AppTableRecordResource { config },
            dashboard: AppDashboardResource { config },
            role: AppRoleResource { config },
            role_member: AppRoleMemberResource { config },
            workflow: AppWorkflowResource { config },
            form: AppTableFormResource { config },
            form_field: AppTableFormFieldResource { config },
        }
    }
}
