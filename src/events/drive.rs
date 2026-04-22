//! Drive v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileReadV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileEditedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id_list: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileTrashedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileDeletedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileTitleUpdatedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFilePermissionMemberAddedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub members: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileCreatedInFolderV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFilePermissionMemberAppliedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFilePermissionMemberRemovedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub members: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileBitableFieldChangedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DriveFileBitableRecordChangedV1 {
    #[serde(default)]
    pub file_token: String,
    #[serde(default)]
    pub file_type: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

// ── Handler registration helpers ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync + 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => {
                Box::pin(handler(typed)) as Pin<Box<dyn Future<Output = Result<()>> + Send>>
            }
            Err(e) => Box::pin(async move {
                Err(Error::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods (all 11 drive/v1 handlers) ──

macro_rules! drive_v1_handler {
    ($method:ident, $event_key:literal, $payload_type:ty) => {
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload_type) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send + 'static,
        {
            self.on_event($event_key, wrap_handler(handler))
        }
    };
}

impl EventDispatcher {
    drive_v1_handler!(
        on_p2_drive_file_read_v1,
        "drive.file.read_v1",
        P2DriveFileReadV1
    );
    drive_v1_handler!(
        on_p2_drive_file_edited_v1,
        "drive.file.edit_v1",
        P2DriveFileEditedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_trashed_v1,
        "drive.file.trashed_v1",
        P2DriveFileTrashedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_deleted_v1,
        "drive.file.deleted_v1",
        P2DriveFileDeletedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_title_updated_v1,
        "drive.file.title_updated_v1",
        P2DriveFileTitleUpdatedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_permission_member_added_v1,
        "drive.file.permission_member_added_v1",
        P2DriveFilePermissionMemberAddedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_created_in_folder_v1,
        "drive.file.created_in_folder_v1",
        P2DriveFileCreatedInFolderV1
    );
    drive_v1_handler!(
        on_p2_drive_file_permission_member_applied_v1,
        "drive.file.permission_member_applied_v1",
        P2DriveFilePermissionMemberAppliedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_permission_member_removed_v1,
        "drive.file.permission_member_removed_v1",
        P2DriveFilePermissionMemberRemovedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_bitable_field_changed_v1,
        "drive.file.bitable_field_changed_v1",
        P2DriveFileBitableFieldChangedV1
    );
    drive_v1_handler!(
        on_p2_drive_file_bitable_record_changed_v1,
        "drive.file.bitable_record_changed_v1",
        P2DriveFileBitableRecordChangedV1
    );
}
