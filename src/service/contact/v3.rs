use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyResp, PageQuery, RestRequest};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentI18nName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentLeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<DepartmentI18nName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DepartmentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_group_chat: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaders: Option<Vec<DepartmentLeader>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_chat_employee_types: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_hrbps: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_member_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvatarInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_activated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_exited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unjoin: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomAttrGenericUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCustomAttrValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic_user: Option<CustomAttrGenericUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCustomAttr {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<UserCustomAttrValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPosition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_position_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_major: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOrder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary_dept: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserAssignInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_plan_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<AvatarInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_station: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_tenant_manager: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<UserPosition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attrs: Option<Vec<UserCustomAttr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idp_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_send_notification: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_option: Option<NotificationOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_info: Option<Vec<UserAssignInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_path: Option<Vec<DepartmentDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserContactInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDepartmentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceAcceptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeTypeEnum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_content: Option<Vec<I18nContent>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionalRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionalRoleMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionalRoleMemberResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFamily {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobTitle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Unit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Memberlist {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemberResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_user_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_department_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkCity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_city_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchDepartmentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UnbindDepartmentChatReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDepartmentIdReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchGetIdUserReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobiles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resigned: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUserReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_chat_acceptor_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_chat_acceptor_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_acceptor_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_acceptor_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_acceptor_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_acceptor_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survey_acceptor_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_acceptor: Option<ResourceAcceptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycross_acceptor_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResurrectUserReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<UserDepartmentInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserIdReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFunctionalRoleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFunctionalRoleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateFunctionalRoleMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeleteFunctionalRoleMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ScopesFunctionalRoleMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AddGroupMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchAddGroupMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Memberlist>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchRemoveGroupMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Memberlist>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveGroupMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUnitReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchUnitReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BindDepartmentUnitReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UnbindDepartmentUnitReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

// ── Response wrappers (macro) ──

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateDepartmentRespData {
    pub department: Option<Department>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetDepartmentRespData {
    pub department: Option<Department>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PatchDepartmentRespData {
    pub department: Option<Department>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateDepartmentRespData {
    pub department: Option<Department>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListDepartmentRespData {
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
    pub items: Option<Vec<Department>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchDepartmentRespData {
    pub items: Option<Vec<Department>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SearchDepartmentRespData {
    pub items: Option<Vec<Department>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateUserRespData {
    pub user: Option<User>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetUserRespData {
    pub user: Option<User>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PatchUserRespData {
    pub user: Option<User>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateUserRespData {
    pub user: Option<User>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListUserRespData {
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
    pub items: Option<Vec<User>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchUserRespData {
    pub items: Option<Vec<User>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchGetIdUserRespData {
    pub user_list: Option<Vec<UserContactInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateEmployeeTypeEnumRespData {
    pub employee_type_enum: Option<EmployeeTypeEnum>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateEmployeeTypeEnumRespData {
    pub employee_type_enum: Option<EmployeeTypeEnum>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListEmployeeTypeEnumRespData {
    pub items: Option<Vec<EmployeeTypeEnum>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateFunctionalRoleRespData {
    pub role_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchCreateFunctionalRoleMemberRespData {
    pub results: Option<Vec<FunctionalRoleMemberResult>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchDeleteFunctionalRoleMemberRespData {
    pub result: Option<Vec<FunctionalRoleMemberResult>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetFunctionalRoleMemberRespData {
    pub member: Option<FunctionalRoleMember>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListFunctionalRoleMemberRespData {
    pub members: Option<Vec<FunctionalRoleMember>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ScopesFunctionalRoleMemberRespData {
    pub results: Option<Vec<FunctionalRoleMemberResult>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateGroupRespData {
    pub group_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetGroupRespData {
    pub group: Option<Group>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MemberBelongGroupRespData {
    pub group_list: Option<Vec<String>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SimplelistGroupRespData {
    pub grouplist: Option<Vec<Group>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchAddGroupMemberRespData {
    pub results: Option<Vec<MemberResult>>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SimplelistGroupMemberRespData {
    pub memberlist: Option<Vec<Memberlist>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateJobFamilyRespData {
    pub job_family: Option<JobFamily>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetJobFamilyRespData {
    pub job_family: Option<JobFamily>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateJobFamilyRespData {
    pub job_family: Option<JobFamily>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListJobFamilyRespData {
    pub items: Option<Vec<JobFamily>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateJobLevelRespData {
    pub job_level: Option<JobLevel>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetJobLevelRespData {
    pub job_level: Option<JobLevel>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateJobLevelRespData {
    pub job_level: Option<JobLevel>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListJobLevelRespData {
    pub items: Option<Vec<JobLevel>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetJobTitleRespData {
    pub job_title: Option<JobTitle>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListJobTitleRespData {
    pub items: Option<Vec<JobTitle>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListScopeRespData {
    pub department_ids: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
    pub group_ids: Option<Vec<String>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateUnitRespData {
    pub unit_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetUnitRespData {
    pub unit: Option<Unit>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListUnitRespData {
    pub unitlist: Option<Vec<Unit>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListDepartmentUnitRespData {
    pub departmentlist: Option<Vec<UnitDepartment>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetWorkCityRespData {
    pub work_city: Option<WorkCity>,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListWorkCityRespData {
    pub items: Option<Vec<WorkCity>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

impl_resp!(CreateDepartmentResp, CreateDepartmentRespData);
impl_resp!(GetDepartmentResp, GetDepartmentRespData);
impl_resp!(PatchDepartmentResp, PatchDepartmentRespData);
impl_resp!(UpdateDepartmentResp, UpdateDepartmentRespData);
impl_resp!(ListDepartmentResp, ListDepartmentRespData);
impl_resp!(BatchDepartmentResp, BatchDepartmentRespData);
impl_resp!(ChildrenDepartmentResp, ListDepartmentRespData);
impl_resp!(ParentDepartmentResp, ListDepartmentRespData);
impl_resp!(SearchDepartmentResp, SearchDepartmentRespData);
impl_resp!(CreateUserResp, CreateUserRespData);
impl_resp!(GetUserResp, GetUserRespData);
impl_resp!(PatchUserResp, PatchUserRespData);
impl_resp!(UpdateUserResp, UpdateUserRespData);
impl_resp!(ListUserResp, ListUserRespData);
impl_resp!(BatchUserResp, BatchUserRespData);
impl_resp!(BatchGetIdUserResp, BatchGetIdUserRespData);
impl_resp!(FindByDepartmentUserResp, ListUserRespData);
impl_resp!(CreateEmployeeTypeEnumResp, CreateEmployeeTypeEnumRespData);
impl_resp!(UpdateEmployeeTypeEnumResp, UpdateEmployeeTypeEnumRespData);
impl_resp!(ListEmployeeTypeEnumResp, ListEmployeeTypeEnumRespData);
impl_resp!(CreateFunctionalRoleResp, CreateFunctionalRoleRespData);
impl_resp!(
    BatchCreateFunctionalRoleMemberResp,
    BatchCreateFunctionalRoleMemberRespData
);
impl_resp!(
    BatchDeleteFunctionalRoleMemberResp,
    BatchDeleteFunctionalRoleMemberRespData
);
impl_resp!(GetFunctionalRoleMemberResp, GetFunctionalRoleMemberRespData);
impl_resp!(
    ListFunctionalRoleMemberResp,
    ListFunctionalRoleMemberRespData
);
impl_resp!(
    ScopesFunctionalRoleMemberResp,
    ScopesFunctionalRoleMemberRespData
);
impl_resp!(CreateGroupResp, CreateGroupRespData);
impl_resp!(GetGroupResp, GetGroupRespData);
impl_resp!(MemberBelongGroupResp, MemberBelongGroupRespData);
impl_resp!(SimplelistGroupResp, SimplelistGroupRespData);
impl_resp!(BatchAddGroupMemberResp, BatchAddGroupMemberRespData);
impl_resp!(SimplelistGroupMemberResp, SimplelistGroupMemberRespData);
impl_resp!(CreateJobFamilyResp, CreateJobFamilyRespData);
impl_resp!(GetJobFamilyResp, GetJobFamilyRespData);
impl_resp!(UpdateJobFamilyResp, UpdateJobFamilyRespData);
impl_resp!(ListJobFamilyResp, ListJobFamilyRespData);
impl_resp!(CreateJobLevelResp, CreateJobLevelRespData);
impl_resp!(GetJobLevelResp, GetJobLevelRespData);
impl_resp!(UpdateJobLevelResp, UpdateJobLevelRespData);
impl_resp!(ListJobLevelResp, ListJobLevelRespData);
impl_resp!(GetJobTitleResp, GetJobTitleRespData);
impl_resp!(ListJobTitleResp, ListJobTitleRespData);
impl_resp!(ListScopeResp, ListScopeRespData);
impl_resp!(CreateUnitResp, CreateUnitRespData);
impl_resp!(GetUnitResp, GetUnitRespData);
impl_resp!(ListUnitResp, ListUnitRespData);
impl_resp!(ListDepartmentUnitResp, ListDepartmentUnitRespData);
impl_resp!(GetWorkCityResp, GetWorkCityRespData);
impl_resp!(ListWorkCityResp, ListWorkCityRespData);

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CustomAttrListData {
    pub items: Option<Vec<serde_json::Value>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

impl_resp!(ListCustomAttrResp, CustomAttrListData);

// ── Resources ──

pub struct DepartmentResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListDepartmentQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub parent_department_id: Option<&'a str>,
    pub fetch_child: Option<bool>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListDepartmentQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn parent_department_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.parent_department_id = value.into();
        self
    }

    pub fn fetch_child(mut self, value: impl Into<Option<bool>>) -> Self {
        self.fetch_child = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ChildrenDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub fetch_child: Option<bool>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ChildrenDepartmentQuery<'a> {
    pub fn new(department_id: &'a str) -> Self {
        Self {
            department_id,
            user_id_type: None,
            department_id_type: None,
            fetch_child: None,
            page_size: None,
            page_token: None,
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

    pub fn fetch_child(mut self, value: impl Into<Option<bool>>) -> Self {
        self.fetch_child = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ParentDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ParentDepartmentQuery<'a> {
    pub fn new(department_id: &'a str) -> Self {
        Self {
            department_id,
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
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

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct SearchDepartmentQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> SearchDepartmentQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> GetDepartmentQuery<'a> {
    pub fn new(department_id: &'a str) -> Self {
        Self {
            department_id,
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct BatchDepartmentQuery<'a> {
    pub department_ids: &'a [&'a str],
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchDepartmentQuery<'a> {
    pub fn new(department_ids: &'a [&'a str]) -> Self {
        Self {
            department_ids,
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UnbindDepartmentChatQuery<'a> {
    pub body: &'a UnbindDepartmentChatReqBody,
    pub department_id_type: Option<&'a str>,
}

impl<'a> UnbindDepartmentChatQuery<'a> {
    pub fn new(body: &'a UnbindDepartmentChatReqBody) -> Self {
        Self {
            body,
            department_id_type: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }
}

impl<'a> DepartmentResource<'a> {
    pub async fn create(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        client_token: Option<&str>,
        body: &Department,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/contact/v3/departments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = client_token {
            api_req.query_params.set("client_token", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateDepartmentRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        department_id: &str,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetDepartmentResp, LarkError> {
        let query = GetDepartmentQuery::new(department_id)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDepartmentResp, LarkError> {
        let path = format!("/open-apis/contact/v3/departments/{}", query.department_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .send::<GetDepartmentRespData>()
        .await?;
        Ok(GetDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &Department,
        option: &RequestOption,
    ) -> Result<PatchDepartmentResp, LarkError> {
        let path = format!("/open-apis/contact/v3/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<PatchDepartmentRespData>(self.config, &api_req, option)
                .await?;
        Ok(PatchDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &Department,
        option: &RequestOption,
    ) -> Result<UpdateDepartmentResp, LarkError> {
        let path = format!("/open-apis/contact/v3/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UpdateDepartmentRespData>(self.config, &api_req, option)
                .await?;
        Ok(UpdateDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update_department_id(
        &self,
        department_id: &str,
        department_id_type: Option<&str>,
        body: &UpdateDepartmentIdReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/contact/v3/departments/{department_id}/update_department_id");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
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
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        parent_department_id: Option<&str>,
        fetch_child: Option<bool>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDepartmentResp, LarkError> {
        let query = ListDepartmentQuery::new()
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .parent_department_id(parent_department_id)
            .fetch_child(fetch_child)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDepartmentResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/departments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("parent_department_id", query.parent_department_id)
        .query("fetch_child", query.fetch_child)
        .page_query(query.page_query())
        .send::<ListDepartmentRespData>()
        .await?;
        Ok(ListDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch(
        &self,
        department_ids: &[&str],
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchDepartmentResp, LarkError> {
        let query = BatchDepartmentQuery::new(department_ids)
            .department_id_type(department_id_type)
            .user_id_type(user_id_type);
        self.batch_by_query(&query, option).await
    }

    pub async fn batch_by_query(
        &self,
        query: &BatchDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchDepartmentResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/departments/batch",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query_values("department_ids", Some(query.department_ids.iter().copied()))
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .send::<BatchDepartmentRespData>()
        .await?;
        Ok(BatchDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn children(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        fetch_child: Option<bool>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ChildrenDepartmentResp, LarkError> {
        let query = ChildrenDepartmentQuery::new(department_id)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .fetch_child(fetch_child)
            .page_size(page_size)
            .page_token(page_token);
        self.children_by_query(&query, option).await
    }

    pub async fn children_by_query(
        &self,
        query: &ChildrenDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ChildrenDepartmentResp, LarkError> {
        let path = format!(
            "/open-apis/contact/v3/departments/{}/children",
            query.department_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("fetch_child", query.fetch_child)
        .page_query(query.page_query())
        .send::<ListDepartmentRespData>()
        .await?;
        Ok(ChildrenDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn parent(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ParentDepartmentResp, LarkError> {
        let query = ParentDepartmentQuery::new(department_id)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .page_token(page_token)
            .page_size(page_size);
        self.parent_by_query(&query, option).await
    }

    pub async fn parent_by_query(
        &self,
        query: &ParentDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ParentDepartmentResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/departments/parent",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("department_id", query.department_id)
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .page_query(query.page_query())
        .send::<ListDepartmentRespData>()
        .await?;
        Ok(ParentDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn search(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        body: &SearchDepartmentReqBody,
        option: &RequestOption,
    ) -> Result<SearchDepartmentResp, LarkError> {
        let query = SearchDepartmentQuery::new()
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .page_token(page_token)
            .page_size(page_size);
        self.search_by_query(&query, body, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchDepartmentQuery<'_>,
        body: &SearchDepartmentReqBody,
        option: &RequestOption,
    ) -> Result<SearchDepartmentResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/contact/v3/departments/search",
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .page_query(query.page_query())
        .json_body(body)?
        .send::<SearchDepartmentRespData>()
        .await?;
        Ok(SearchDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn unbind_department_chat(
        &self,
        department_id_type: Option<&str>,
        body: &UnbindDepartmentChatReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = UnbindDepartmentChatQuery::new(body).department_id_type(department_id_type);
        self.unbind_department_chat_by_query(&query, option).await
    }

    pub async fn unbind_department_chat_by_query(
        &self,
        query: &UnbindDepartmentChatQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/contact/v3/departments/unbind_department_chat",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct UserResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListUserQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub department_id: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListUserQuery<'a> {
    pub fn new() -> Self {
        Self::default()
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

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct FindUserByDepartmentQuery<'a> {
    pub department_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> FindUserByDepartmentQuery<'a> {
    pub fn new(department_id: &'a str) -> Self {
        Self {
            department_id,
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
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

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetUserQuery<'a> {
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> GetUserQuery<'a> {
    pub fn new(user_id: &'a str) -> Self {
        Self {
            user_id,
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct BatchUserQuery<'a> {
    pub user_ids: &'a [&'a str],
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> BatchUserQuery<'a> {
    pub fn new(user_ids: &'a [&'a str]) -> Self {
        Self {
            user_ids,
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct BatchGetIdUserQuery<'a> {
    pub body: &'a BatchGetIdUserReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchGetIdUserQuery<'a> {
    pub fn new(body: &'a BatchGetIdUserReqBody) -> Self {
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ResurrectUserQuery<'a> {
    pub user_id: &'a str,
    pub body: &'a ResurrectUserReqBody,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> ResurrectUserQuery<'a> {
    pub fn new(user_id: &'a str, body: &'a ResurrectUserReqBody) -> Self {
        Self {
            user_id,
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

impl<'a> UserResource<'a> {
    pub async fn create(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        client_token: Option<&str>,
        body: &User,
        option: &RequestOption,
    ) -> Result<CreateUserResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/contact/v3/users");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = client_token {
            api_req.query_params.set("client_token", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateUserRespData>(self.config, &api_req, option).await?;
        Ok(CreateUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        body: &DeleteUserReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/users/{user_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
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

    pub async fn get(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetUserResp, LarkError> {
        let query = GetUserQuery::new(user_id)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetUserResp, LarkError> {
        let path = format!("/open-apis/contact/v3/users/{}", query.user_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .send::<GetUserRespData>()
        .await?;
        Ok(GetUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &User,
        option: &RequestOption,
    ) -> Result<PatchUserResp, LarkError> {
        let path = format!("/open-apis/contact/v3/users/{user_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<PatchUserRespData>(self.config, &api_req, option).await?;
        Ok(PatchUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &User,
        option: &RequestOption,
    ) -> Result<UpdateUserResp, LarkError> {
        let path = format!("/open-apis/contact/v3/users/{user_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UpdateUserRespData>(self.config, &api_req, option).await?;
        Ok(UpdateUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        department_id: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListUserResp, LarkError> {
        let query = ListUserQuery::new()
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .department_id(department_id)
            .page_token(page_token)
            .page_size(page_size);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListUserResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/users",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .query("department_id", query.department_id)
        .page_query(query.page_query())
        .send::<ListUserRespData>()
        .await?;
        Ok(ListUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch(
        &self,
        user_ids: &[&str],
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchUserResp, LarkError> {
        let query = BatchUserQuery::new(user_ids)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.batch_by_query(&query, option).await
    }

    pub async fn batch_by_query(
        &self,
        query: &BatchUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchUserResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/users/batch",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query_values("user_ids", Some(query.user_ids.iter().copied()))
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .send::<BatchUserRespData>()
        .await?;
        Ok(BatchUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_get_id(
        &self,
        user_id_type: Option<&str>,
        body: &BatchGetIdUserReqBody,
        option: &RequestOption,
    ) -> Result<BatchGetIdUserResp, LarkError> {
        let query = BatchGetIdUserQuery::new(body).user_id_type(user_id_type);
        self.batch_get_id_by_query(&query, option).await
    }

    pub async fn batch_get_id_by_query(
        &self,
        query: &BatchGetIdUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchGetIdUserResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/contact/v3/users/batch_get_id",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<BatchGetIdUserRespData>()
        .await?;
        Ok(BatchGetIdUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn find_by_department(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<FindByDepartmentUserResp, LarkError> {
        let query = FindUserByDepartmentQuery::new(department_id)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .page_size(page_size)
            .page_token(page_token);
        self.find_by_department_by_query(&query, option).await
    }

    pub async fn find_by_department_by_query(
        &self,
        query: &FindUserByDepartmentQuery<'_>,
        option: &RequestOption,
    ) -> Result<FindByDepartmentUserResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/users/find_by_department",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("department_id", query.department_id)
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .page_query(query.page_query())
        .send::<ListUserRespData>()
        .await?;
        Ok(FindByDepartmentUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn resurrect(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &ResurrectUserReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = ResurrectUserQuery::new(user_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.resurrect_by_query(&query, option).await
    }

    pub async fn resurrect_by_query(
        &self,
        query: &ResurrectUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/users/{}/resurrect", query.user_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn update_user_id(
        &self,
        user_id: &str,
        body: &UpdateUserIdReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/users/{user_id}/update_user_id");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
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

pub struct ScopeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListScopeQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListScopeQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> ScopeResource<'a> {
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListScopeResp, LarkError> {
        let query = ListScopeQuery::new()
            .user_id_type(user_id_type)
            .department_id_type(department_id_type)
            .page_token(page_token)
            .page_size(page_size);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListScopeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListScopeResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/scopes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .page_query(query.page_query())
        .send::<ListScopeRespData>()
        .await?;
        Ok(ListScopeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct EmployeeTypeEnumResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListEmployeeTypeEnumQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListEmployeeTypeEnumQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> EmployeeTypeEnumResource<'a> {
    pub async fn create(
        &self,
        body: &EmployeeTypeEnum,
        option: &RequestOption,
    ) -> Result<CreateEmployeeTypeEnumResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/contact/v3/employee_type_enums",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<CreateEmployeeTypeEnumRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(CreateEmployeeTypeEnumResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        enum_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/employee_type_enums/{enum_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn update(
        &self,
        enum_id: &str,
        body: &EmployeeTypeEnum,
        option: &RequestOption,
    ) -> Result<UpdateEmployeeTypeEnumResp, LarkError> {
        let path = format!("/open-apis/contact/v3/employee_type_enums/{enum_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<UpdateEmployeeTypeEnumRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(UpdateEmployeeTypeEnumResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListEmployeeTypeEnumResp, LarkError> {
        let query = ListEmployeeTypeEnumQuery::new()
            .page_token(page_token)
            .page_size(page_size);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEmployeeTypeEnumQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEmployeeTypeEnumResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/employee_type_enums",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send::<ListEmployeeTypeEnumRespData>()
        .await?;
        Ok(ListEmployeeTypeEnumResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct UnitResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListUnitQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListUnitQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListDepartmentUnitQuery<'a> {
    pub unit_id: &'a str,
    pub department_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListDepartmentUnitQuery<'a> {
    pub fn new(unit_id: &'a str) -> Self {
        Self {
            unit_id,
            department_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> UnitResource<'a> {
    pub async fn create(
        &self,
        body: &CreateUnitReqBody,
        option: &RequestOption,
    ) -> Result<CreateUnitResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/contact/v3/unit");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateUnitRespData>(self.config, &api_req, option).await?;
        Ok(CreateUnitResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        unit_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/unit/{unit_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        unit_id: &str,
        option: &RequestOption,
    ) -> Result<GetUnitResp, LarkError> {
        let path = format!("/open-apis/contact/v3/unit/{unit_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<GetUnitRespData>(self.config, &api_req, option).await?;
        Ok(GetUnitResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        unit_id: &str,
        body: &PatchUnitReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/unit/{unit_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
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
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListUnitResp, LarkError> {
        let query = ListUnitQuery::new()
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListUnitQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListUnitResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/unit",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .send::<ListUnitRespData>()
        .await?;
        Ok(ListUnitResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn bind_department(
        &self,
        body: &BindDepartmentUnitReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/contact/v3/unit/bind_department",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unbind_department(
        &self,
        body: &UnbindDepartmentUnitReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/contact/v3/unit/unbind_department",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list_department(
        &self,
        unit_id: &str,
        department_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListDepartmentUnitResp, LarkError> {
        let query = ListDepartmentUnitQuery::new(unit_id)
            .department_id_type(department_id_type)
            .page_token(page_token)
            .page_size(page_size);
        self.list_department_by_query(&query, option).await
    }

    pub async fn list_department_by_query(
        &self,
        query: &ListDepartmentUnitQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDepartmentUnitResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/unit/list_department",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("unit_id", query.unit_id)
        .query("department_id_type", query.department_id_type)
        .page_query(query.page_query())
        .send::<ListDepartmentUnitRespData>()
        .await?;
        Ok(ListDepartmentUnitResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct GroupResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetGroupQuery<'a> {
    pub group_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> GetGroupQuery<'a> {
    pub fn new(group_id: &'a str) -> Self {
        Self {
            group_id,
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

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct SimplelistGroupQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub group_type: Option<i32>,
}

impl<'a> SimplelistGroupQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn group_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.group_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MemberBelongGroupQuery<'a> {
    pub member_id: &'a str,
    pub member_id_type: Option<&'a str>,
    pub group_type: Option<i32>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> MemberBelongGroupQuery<'a> {
    pub fn new(member_id: &'a str) -> Self {
        Self {
            member_id,
            member_id_type: None,
            group_type: None,
            page_size: None,
            page_token: None,
        }
    }

    pub fn member_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.member_id_type = value.into();
        self
    }

    pub fn group_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.group_type = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> GroupResource<'a> {
    pub async fn create(
        &self,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &Group,
        option: &RequestOption,
    ) -> Result<CreateGroupResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/contact/v3/group");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateGroupRespData>(self.config, &api_req, option).await?;
        Ok(CreateGroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        group_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/group/{group_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        group_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetGroupResp, LarkError> {
        let query = GetGroupQuery::new(group_id)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetGroupResp, LarkError> {
        let path = format!("/open-apis/contact/v3/group/{}", query.group_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .send::<GetGroupRespData>()
        .await?;
        Ok(GetGroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        group_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &Group,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/group/{group_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn simplelist(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        group_type: Option<i32>,
        option: &RequestOption,
    ) -> Result<SimplelistGroupResp, LarkError> {
        let query = SimplelistGroupQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .group_type(group_type);
        self.simplelist_by_query(&query, option).await
    }

    pub async fn simplelist_by_query(
        &self,
        query: &SimplelistGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<SimplelistGroupResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/group/simplelist",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("type", query.group_type)
        .send::<SimplelistGroupRespData>()
        .await?;
        Ok(SimplelistGroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn member_belong(
        &self,
        member_id: &str,
        member_id_type: Option<&str>,
        group_type: Option<i32>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<MemberBelongGroupResp, LarkError> {
        let query = MemberBelongGroupQuery::new(member_id)
            .member_id_type(member_id_type)
            .group_type(group_type)
            .page_size(page_size)
            .page_token(page_token);
        self.member_belong_by_query(&query, option).await
    }

    pub async fn member_belong_by_query(
        &self,
        query: &MemberBelongGroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<MemberBelongGroupResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/contact/v3/group/member_belong",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("member_id", query.member_id)
        .query("member_id_type", query.member_id_type)
        .query("group_type", query.group_type)
        .page_query(query.page_query())
        .send::<MemberBelongGroupRespData>()
        .await?;
        Ok(MemberBelongGroupResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct GroupMemberResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SimplelistGroupMemberQuery<'a> {
    pub group_id: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub member_id_type: Option<&'a str>,
    pub member_type: Option<&'a str>,
}

impl<'a> SimplelistGroupMemberQuery<'a> {
    pub fn new(group_id: &'a str) -> Self {
        Self {
            group_id,
            page_size: None,
            page_token: None,
            member_id_type: None,
            member_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn member_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.member_id_type = value.into();
        self
    }

    pub fn member_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.member_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> GroupMemberResource<'a> {
    pub async fn add(
        &self,
        group_id: &str,
        body: &AddGroupMemberReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/group/{group_id}/member/add");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn batch_add(
        &self,
        group_id: &str,
        body: &BatchAddGroupMemberReqBody,
        option: &RequestOption,
    ) -> Result<BatchAddGroupMemberResp, LarkError> {
        let path = format!("/open-apis/contact/v3/group/{group_id}/member/batch_add");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchAddGroupMemberRespData>(self.config, &api_req, option)
                .await?;
        Ok(BatchAddGroupMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_remove(
        &self,
        group_id: &str,
        body: &BatchRemoveGroupMemberReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/group/{group_id}/member/batch_remove");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn remove(
        &self,
        group_id: &str,
        body: &RemoveGroupMemberReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/group/{group_id}/member/remove");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn simplelist(
        &self,
        group_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        member_id_type: Option<&str>,
        member_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SimplelistGroupMemberResp, LarkError> {
        let query = SimplelistGroupMemberQuery::new(group_id)
            .page_size(page_size)
            .page_token(page_token)
            .member_id_type(member_id_type)
            .member_type(member_type);
        self.simplelist_by_query(&query, option).await
    }

    pub async fn simplelist_by_query(
        &self,
        query: &SimplelistGroupMemberQuery<'_>,
        option: &RequestOption,
    ) -> Result<SimplelistGroupMemberResp, LarkError> {
        let path = format!(
            "/open-apis/contact/v3/group/{}/member/simplelist",
            query.group_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("member_id_type", query.member_id_type)
        .query("member_type", query.member_type)
        .send::<SimplelistGroupMemberRespData>()
        .await?;
        Ok(SimplelistGroupMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct FunctionalRoleResource<'a> {
    config: &'a Config,
}

impl<'a> FunctionalRoleResource<'a> {
    pub async fn create(
        &self,
        body: &CreateFunctionalRoleReqBody,
        option: &RequestOption,
    ) -> Result<CreateFunctionalRoleResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/contact/v3/functional_roles");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateFunctionalRoleRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateFunctionalRoleResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        role_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/functional_roles/{role_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn update(
        &self,
        role_id: &str,
        body: &UpdateFunctionalRoleReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/functional_roles/{role_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
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

pub struct FunctionalRoleMemberResource<'a> {
    config: &'a Config,
}

impl<'a> FunctionalRoleMemberResource<'a> {
    pub async fn batch_create(
        &self,
        role_id: &str,
        user_id_type: Option<&str>,
        body: &BatchCreateFunctionalRoleMemberReqBody,
        option: &RequestOption,
    ) -> Result<BatchCreateFunctionalRoleMemberResp, LarkError> {
        let path = format!("/open-apis/contact/v3/functional_roles/{role_id}/members/batch_create");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<BatchCreateFunctionalRoleMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(BatchCreateFunctionalRoleMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_delete(
        &self,
        role_id: &str,
        user_id_type: Option<&str>,
        body: &BatchDeleteFunctionalRoleMemberReqBody,
        option: &RequestOption,
    ) -> Result<BatchDeleteFunctionalRoleMemberResp, LarkError> {
        let path = format!("/open-apis/contact/v3/functional_roles/{role_id}/members/batch_delete");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<BatchDeleteFunctionalRoleMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(BatchDeleteFunctionalRoleMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        role_id: &str,
        member_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetFunctionalRoleMemberResp, LarkError> {
        let path = format!("/open-apis/contact/v3/functional_roles/{role_id}/members/{member_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        let (api_resp, raw) = transport::request_typed::<GetFunctionalRoleMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(GetFunctionalRoleMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        role_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFunctionalRoleMemberResp, LarkError> {
        let path = format!("/open-apis/contact/v3/functional_roles/{role_id}/members");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        let (api_resp, raw) = transport::request_typed::<ListFunctionalRoleMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(ListFunctionalRoleMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn scopes(
        &self,
        role_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &ScopesFunctionalRoleMemberReqBody,
        option: &RequestOption,
    ) -> Result<ScopesFunctionalRoleMemberResp, LarkError> {
        let path = format!("/open-apis/contact/v3/functional_roles/{role_id}/members/scopes");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<ScopesFunctionalRoleMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(ScopesFunctionalRoleMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct JobLevelResource<'a> {
    config: &'a Config,
}

impl<'a> JobLevelResource<'a> {
    pub async fn create(
        &self,
        body: &JobLevel,
        option: &RequestOption,
    ) -> Result<CreateJobLevelResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/contact/v3/job_levels");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateJobLevelRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        job_level_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/job_levels/{job_level_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        job_level_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobLevelResp, LarkError> {
        let path = format!("/open-apis/contact/v3/job_levels/{job_level_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<GetJobLevelRespData>(self.config, &api_req, option).await?;
        Ok(GetJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        job_level_id: &str,
        body: &JobLevel,
        option: &RequestOption,
    ) -> Result<UpdateJobLevelResp, LarkError> {
        let path = format!("/open-apis/contact/v3/job_levels/{job_level_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UpdateJobLevelRespData>(self.config, &api_req, option)
                .await?;
        Ok(UpdateJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        name: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobLevelResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/contact/v3/job_levels");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = name {
            api_req.query_params.set("name", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListJobLevelRespData>(self.config, &api_req, option).await?;
        Ok(ListJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct JobFamilyResource<'a> {
    config: &'a Config,
}

impl<'a> JobFamilyResource<'a> {
    pub async fn create(
        &self,
        body: &JobFamily,
        option: &RequestOption,
    ) -> Result<CreateJobFamilyResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/contact/v3/job_families");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateJobFamilyRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateJobFamilyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/job_families/{job_family_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobFamilyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/job_families/{job_family_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<GetJobFamilyRespData>(self.config, &api_req, option).await?;
        Ok(GetJobFamilyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        job_family_id: &str,
        body: &JobFamily,
        option: &RequestOption,
    ) -> Result<UpdateJobFamilyResp, LarkError> {
        let path = format!("/open-apis/contact/v3/job_families/{job_family_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UpdateJobFamilyRespData>(self.config, &api_req, option)
                .await?;
        Ok(UpdateJobFamilyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        name: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobFamilyResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/contact/v3/job_families");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = name {
            api_req.query_params.set("name", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListJobFamilyRespData>(self.config, &api_req, option)
                .await?;
        Ok(ListJobFamilyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct JobTitleResource<'a> {
    config: &'a Config,
}

impl<'a> JobTitleResource<'a> {
    pub async fn get(
        &self,
        job_title_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobTitleResp, LarkError> {
        let path = format!("/open-apis/contact/v3/job_titles/{job_title_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<GetJobTitleRespData>(self.config, &api_req, option).await?;
        Ok(GetJobTitleResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobTitleResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/contact/v3/job_titles");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListJobTitleRespData>(self.config, &api_req, option).await?;
        Ok(ListJobTitleResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct WorkCityResource<'a> {
    config: &'a Config,
}

impl<'a> WorkCityResource<'a> {
    pub async fn get(
        &self,
        work_city_id: &str,
        option: &RequestOption,
    ) -> Result<GetWorkCityResp, LarkError> {
        let path = format!("/open-apis/contact/v3/work_cities/{work_city_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<GetWorkCityRespData>(self.config, &api_req, option).await?;
        Ok(GetWorkCityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkCityResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/contact/v3/work_cities");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListWorkCityRespData>(self.config, &api_req, option).await?;
        Ok(ListWorkCityResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CustomAttrResource<'a> {
    config: &'a Config,
}

impl<'a> CustomAttrResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCustomAttrResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/contact/v3/custom_attrs");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CustomAttrListData>(self.config, &api_req, option).await?;
        Ok(ListCustomAttrResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V3<'a> {
    pub department: DepartmentResource<'a>,
    pub user: UserResource<'a>,
    pub scope: ScopeResource<'a>,
    pub employee_type_enum: EmployeeTypeEnumResource<'a>,
    pub unit: UnitResource<'a>,
    pub group: GroupResource<'a>,
    pub group_member: GroupMemberResource<'a>,
    pub functional_role: FunctionalRoleResource<'a>,
    pub functional_role_member: FunctionalRoleMemberResource<'a>,
    pub job_level: JobLevelResource<'a>,
    pub job_family: JobFamilyResource<'a>,
    pub job_title: JobTitleResource<'a>,
    pub work_city: WorkCityResource<'a>,
    pub custom_attr: CustomAttrResource<'a>,
}

impl<'a> V3<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            department: DepartmentResource { config },
            user: UserResource { config },
            scope: ScopeResource { config },
            employee_type_enum: EmployeeTypeEnumResource { config },
            unit: UnitResource { config },
            group: GroupResource { config },
            group_member: GroupMemberResource { config },
            functional_role: FunctionalRoleResource { config },
            functional_role_member: FunctionalRoleMemberResource { config },
            job_level: JobLevelResource { config },
            job_family: JobFamilyResource { config },
            job_title: JobTitleResource { config },
            work_city: WorkCityResource { config },
            custom_attr: CustomAttrResource { config },
        }
    }
}
