//! Drive v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Shared sub-types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableFieldAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_value: Option<BitableTableFieldActionValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_value: Option<BitableTableFieldActionValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableFieldActionValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<BitableTableFieldActionValueProperty>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableFieldActionValueProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_fill: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_serial: Option<BitableTableFieldActionValuePropertyAutoSerial>,
    #[serde(default)]
    pub options: Vec<BitableTableFieldActionValuePropertyOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableFieldActionValuePropertyAutoSerial {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default)]
    pub options: Vec<BitableTableFieldActionValuePropertyAutoSerialOption>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableFieldActionValuePropertyAutoSerialOption {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableFieldActionValuePropertyOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableRecordAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    pub before_value: Vec<BitableTableRecordActionField>,
    #[serde(default)]
    pub after_value: Vec<BitableTableRecordActionField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableRecordActionField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_identity_value: Option<BitableTableRecordActionFieldIdentity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableRecordActionFieldIdentity {
    #[serde(default)]
    pub users: Vec<BitableTableRecordActionFieldIdentityUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BitableTableRecordActionFieldIdentityUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Notice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_mentioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_user_type: Option<String>,
}

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileBitableFieldChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub action_list: Vec<BitableTableFieldAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileBitableRecordChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub action_list: Vec<BitableTableRecordAction>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileCreatedInFolderV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub subscriber_ids: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileEditedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default)]
    pub operator_id_list: Vec<UserId>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFilePermissionMemberAddedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub user_list: Vec<UserId>,
    #[serde(default)]
    pub chat_list: Vec<String>,
    #[serde(default)]
    pub open_department_id_list: Vec<String>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFilePermissionMemberAppliedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<UserId>,
    #[serde(default)]
    pub application_user_list: Vec<UserId>,
    #[serde(default)]
    pub application_chat_list: Vec<String>,
    #[serde(default)]
    pub application_department_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(default)]
    pub subscriber_ids: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFilePermissionMemberRemovedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub user_list: Vec<UserId>,
    #[serde(default)]
    pub chat_list: Vec<String>,
    #[serde(default)]
    pub open_department_id_list: Vec<String>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileReadV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default)]
    pub operator_id_list: Vec<UserId>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileTitleUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveFileTrashedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub subscriber_id_list: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2DriveNoticeCommentAddV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice_meta: Option<Notice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_mentioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl P2DriveNoticeCommentAddV1 {
    /// Read the nonempty document token from `notice_meta`, falling back to
    /// the event's top-level field. Neither stored field is changed.
    ///
    /// ```
    /// use larksuite_oapi_sdk_rs::events::drive::P2DriveNoticeCommentAddV1;
    /// let event: P2DriveNoticeCommentAddV1 = serde_json::from_str(
    ///     r#"{"file_token":"doc_1","notice_meta":{"is_mentioned":false}}"#,
    /// ).unwrap();
    /// assert_eq!(event.effective_file_token(), Some("doc_1"));
    /// assert_eq!(event.effective_is_mentioned(), Some(false));
    /// ```
    pub fn effective_file_token(&self) -> Option<&str> {
        self.notice_meta
            .as_ref()
            .and_then(|meta| meta.file_token.as_deref())
            .filter(|value| !value.is_empty())
            .or_else(|| self.file_token.as_deref().filter(|value| !value.is_empty()))
    }

    /// Read the nonempty document type from `notice_meta`, falling back to
    /// the top-level field without trimming or rewriting either value.
    pub fn effective_file_type(&self) -> Option<&str> {
        self.notice_meta
            .as_ref()
            .and_then(|meta| meta.file_type.as_deref())
            .filter(|value| !value.is_empty())
            .or_else(|| self.file_type.as_deref().filter(|value| !value.is_empty()))
    }

    /// Prefer the top-level mention flag, including explicit `false`, then
    /// fall back to `notice_meta`. Missing flags remain `None`.
    pub fn effective_is_mentioned(&self) -> Option<bool> {
        self.is_mentioned
            .or_else(|| self.notice_meta.as_ref().and_then(|meta| meta.is_mentioned))
    }
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_drive_file_bitable_field_changed_v1 => P2DriveFileBitableFieldChangedV1
        : "drive.file.bitable_field_changed_v1",
    on_p2_drive_file_bitable_record_changed_v1 => P2DriveFileBitableRecordChangedV1
        : "drive.file.bitable_record_changed_v1",
    on_p2_drive_file_created_in_folder_v1 => P2DriveFileCreatedInFolderV1
        : "drive.file.created_in_folder_v1",
    on_p2_drive_file_deleted_v1 => P2DriveFileDeletedV1
        : "drive.file.deleted_v1",
    on_p2_drive_file_edited_v1 => P2DriveFileEditedV1
        : "drive.file.edit_v1",
    on_p2_drive_file_permission_member_added_v1 => P2DriveFilePermissionMemberAddedV1
        : "drive.file.permission_member_added_v1",
    on_p2_drive_file_permission_member_applied_v1 => P2DriveFilePermissionMemberAppliedV1
        : "drive.file.permission_member_applied_v1",
    on_p2_drive_file_permission_member_removed_v1 => P2DriveFilePermissionMemberRemovedV1
        : "drive.file.permission_member_removed_v1",
    on_p2_drive_file_read_v1 => P2DriveFileReadV1
        : "drive.file.read_v1",
    on_p2_drive_file_title_updated_v1 => P2DriveFileTitleUpdatedV1
        : "drive.file.title_updated_v1",
    on_p2_drive_file_trashed_v1 => P2DriveFileTrashedV1
        : "drive.file.trashed_v1",
    on_p2_drive_notice_comment_add_v1 => P2DriveNoticeCommentAddV1
        : "drive.notice.comment_add_v1",
}
