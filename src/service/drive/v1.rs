use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{DownloadResp, EmptyResp, PageQuery, RestRequest};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShortcutInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortcut_info: Option<ShortcutInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Property {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refer_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refer_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextRun {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocsLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Person {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyElement {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_run: Option<TextRun>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_link: Option<DocsLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<ReplyElement>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyExtra {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileCommentReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ReplyContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<ReplyExtra>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<FileCommentReply>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileComment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_solved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solved_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solver_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_whole: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_list: Option<ReplyList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pv: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_today: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pv_today: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count_today: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileViewRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_view_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileSubscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_subcribe: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Version {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportTaskMountPoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<ImportTaskMountPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TmpDownloadUrl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tmp_download_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perm_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Member {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perm_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_label: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Owner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestDoc {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_modify_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sec_label_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetaFailed {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionPublicV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_access: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_share_entity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_switch: Option<bool>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyFileReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<Property>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFolderFileReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateShortcutFileReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer_entity: Option<ReferEntity>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MoveFileReqBody {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FileUploadInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadFinishFileReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_num: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchQueryFileCommentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchFileCommentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_solved: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFileCommentReplyReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ReplyContent>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFileSubscriptionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchFileSubscriptionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MediaUploadInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadFinishMediaReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_num: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MetaRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_docs: Option<Vec<RequestDoc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_url: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreatePermissionMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<BaseMember>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePermissionMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm_type: Option<String>,
}

// ── Response data types ──

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateExportTaskRespData {
    pub ticket: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetExportTaskRespData {
    pub result: Option<ExportTask>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CopyFileRespData {
    pub file: Option<File>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateFolderFileRespData {
    pub token: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateShortcutFileRespData {
    pub succ_shortcut_node: Option<File>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DeleteFileRespData {
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetSubscribeFileRespData {
    pub is_subscribe: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListFileRespData {
    pub files: Option<Vec<File>>,
    pub next_page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MoveFileRespData {
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaskCheckFileRespData {
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UploadAllFileRespData {
    pub file_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UploadPrepareFileRespData {
    pub upload_id: Option<String>,
    pub block_size: Option<i64>,
    pub block_num: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UploadFinishFileRespData {
    pub file_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchQueryFileCommentRespData {
    pub items: Option<Vec<FileComment>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListFileCommentRespData {
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
    pub items: Option<Vec<FileComment>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListFileCommentReplyRespData {
    pub items: Option<Vec<FileCommentReply>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetFileStatisticsRespData {
    pub file_token: Option<String>,
    pub file_type: Option<String>,
    pub statistics: Option<FileStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FileSubscriptionRespData {
    pub subscription_id: Option<String>,
    pub subscription_type: Option<String>,
    pub is_subcribe: Option<bool>,
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListFileViewRecordRespData {
    pub items: Option<Vec<FileViewRecord>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateFileVersionRespData {
    pub name: Option<String>,
    pub version: Option<String>,
    pub parent_token: Option<String>,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub status: Option<String>,
    pub obj_type: Option<String>,
    pub parent_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListFileVersionRespData {
    pub items: Option<Vec<Version>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateImportTaskRespData {
    pub ticket: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetImportTaskRespData {
    pub result: Option<ImportTask>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchGetTmpDownloadUrlMediaRespData {
    pub tmp_download_urls: Option<Vec<TmpDownloadUrl>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UploadAllMediaRespData {
    pub file_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UploadPrepareMediaRespData {
    pub upload_id: Option<String>,
    pub block_size: Option<i64>,
    pub block_num: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UploadFinishMediaRespData {
    pub file_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchQueryMetaRespData {
    pub metas: Option<Vec<Meta>>,
    pub failed_list: Option<Vec<MetaFailed>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuthPermissionMemberRespData {
    pub auth_result: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BatchCreatePermissionMemberRespData {
    pub members: Option<Vec<BaseMember>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreatePermissionMemberRespData {
    pub member: Option<BaseMember>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListPermissionMemberRespData {
    pub items: Option<Vec<Member>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdatePermissionMemberRespData {
    pub member: Option<BaseMember>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetPermissionPublicV1RespData {
    pub permission_public: Option<PermissionPublicV1>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreatePermissionPublicPasswordRespData {
    pub password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdatePermissionPublicPasswordRespData {
    pub password: Option<String>,
}

// ── Response wrappers ──

impl_resp!(CreateExportTaskResp, CreateExportTaskRespData);
impl_resp!(GetExportTaskResp, GetExportTaskRespData);
impl_resp!(CopyFileResp, CopyFileRespData);
impl_resp!(CreateFolderFileResp, CreateFolderFileRespData);
impl_resp!(CreateShortcutFileResp, CreateShortcutFileRespData);
impl_resp!(DeleteFileResp, DeleteFileRespData);
impl_resp!(GetSubscribeFileResp, GetSubscribeFileRespData);
impl_resp!(ListFileResp, ListFileRespData);
impl_resp!(MoveFileResp, MoveFileRespData);
impl_resp!(TaskCheckFileResp, TaskCheckFileRespData);
impl_resp!(UploadAllFileResp, UploadAllFileRespData);
impl_resp!(UploadPrepareFileResp, UploadPrepareFileRespData);
impl_resp!(UploadFinishFileResp, UploadFinishFileRespData);
impl_resp!(BatchQueryFileCommentResp, BatchQueryFileCommentRespData);
impl_resp!(CreateFileCommentResp, FileComment);
impl_resp!(GetFileCommentResp, FileComment);
impl_resp!(ListFileCommentResp, ListFileCommentRespData);
impl_resp!(ListFileCommentReplyResp, ListFileCommentReplyRespData);
impl_resp!(GetFileStatisticsResp, GetFileStatisticsRespData);
impl_resp!(CreateFileSubscriptionResp, FileSubscriptionRespData);
impl_resp!(GetFileSubscriptionResp, FileSubscriptionRespData);
impl_resp!(PatchFileSubscriptionResp, FileSubscriptionRespData);
impl_resp!(ListFileViewRecordResp, ListFileViewRecordRespData);
impl_resp!(CreateFileVersionResp, CreateFileVersionRespData);
impl_resp!(GetFileVersionResp, Version);
impl_resp!(ListFileVersionResp, ListFileVersionRespData);
impl_resp!(CreateImportTaskResp, CreateImportTaskRespData);
impl_resp!(GetImportTaskResp, GetImportTaskRespData);
impl_resp!(
    BatchGetTmpDownloadUrlMediaResp,
    BatchGetTmpDownloadUrlMediaRespData
);
impl_resp!(UploadAllMediaResp, UploadAllMediaRespData);
impl_resp!(UploadPrepareMediaResp, UploadPrepareMediaRespData);
impl_resp!(UploadFinishMediaResp, UploadFinishMediaRespData);
impl_resp!(BatchQueryMetaResp, BatchQueryMetaRespData);
impl_resp!(AuthPermissionMemberResp, AuthPermissionMemberRespData);
impl_resp!(
    BatchCreatePermissionMemberResp,
    BatchCreatePermissionMemberRespData
);
impl_resp!(CreatePermissionMemberResp, CreatePermissionMemberRespData);
impl_resp!(ListPermissionMemberResp, ListPermissionMemberRespData);
impl_resp!(UpdatePermissionMemberResp, UpdatePermissionMemberRespData);
impl_resp!(GetPermissionPublicV1Resp, GetPermissionPublicV1RespData);
impl_resp!(PatchPermissionPublicV1Resp, GetPermissionPublicV1RespData);
impl_resp!(
    CreatePermissionPublicPasswordResp,
    CreatePermissionPublicPasswordRespData
);
impl_resp!(
    UpdatePermissionPublicPasswordResp,
    UpdatePermissionPublicPasswordRespData
);

#[derive(Debug, Clone)]
struct PageIteratorState<T> {
    next_page_token: Option<String>,
    request_page_token: Option<String>,
    items: VecDeque<T>,
    started: bool,
    exhausted: bool,
    limit: Option<usize>,
    emitted: usize,
}

impl<T> Default for PageIteratorState<T> {
    fn default() -> Self {
        Self {
            next_page_token: None,
            request_page_token: None,
            items: VecDeque::new(),
            started: false,
            exhausted: false,
            limit: None,
            emitted: 0,
        }
    }
}

impl<T> PageIteratorState<T> {
    fn limit(mut self, limit: usize) -> Self {
        self.limit = (limit > 0).then_some(limit);
        self
    }

    fn with_page_token(mut self, page_token: Option<String>) -> Self {
        self.request_page_token = page_token;
        self.started = self.request_page_token.is_some();
        self.exhausted = false;
        self
    }

    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn page_token_for_request(&self) -> Option<&str> {
        if self.started {
            self.request_page_token
                .as_deref()
                .or(self.next_page_token.as_deref())
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.limit.is_some_and(|limit| self.emitted >= limit) {
            return None;
        }
        let item = self.items.pop_front()?;
        self.emitted += 1;
        Some(item)
    }

    fn should_fetch(&self) -> bool {
        self.limit.is_none_or(|limit| self.emitted < limit)
            && self.items.is_empty()
            && !self.exhausted
            && (!self.started
                || self.request_page_token.is_some()
                || self.next_page_token.is_some())
    }

    fn accept_page(
        &mut self,
        items: Option<Vec<T>>,
        page_token: Option<String>,
        has_more: Option<bool>,
    ) {
        let prev_page_token = self.next_page_token.clone();
        self.started = true;
        self.request_page_token = None;
        self.items = items.unwrap_or_default().into();
        self.next_page_token = if self.items.is_empty() {
            prev_page_token
        } else {
            page_token
        };
        self.exhausted =
            self.items.is_empty() || !has_more.unwrap_or(false) || self.next_page_token.is_none();
    }
}

mod multipart_form {
    use crate::req::{FormDataField, FormDataValue};

    pub(super) fn upload_all_fields(
        file_name: &str,
        parent_type: &str,
        parent_node: &str,
        size: i64,
        checksum: Option<&str>,
        extra: Option<&str>,
        data: Vec<u8>,
    ) -> Vec<FormDataField> {
        let mut fields = vec![
            text("file_name", file_name),
            text("parent_type", parent_type),
            text("parent_node", parent_node),
            text("size", size),
            file("file", file_name, data),
        ];
        if let Some(value) = checksum {
            fields.push(text("checksum", value));
        }
        if let Some(value) = extra {
            fields.push(text("extra", value));
        }
        fields
    }

    pub(super) fn upload_part_fields(
        upload_id: &str,
        seq: i32,
        size: i64,
        checksum: Option<&str>,
        data: Vec<u8>,
    ) -> Vec<FormDataField> {
        let mut fields = vec![
            text("upload_id", upload_id),
            text("seq", seq),
            text("size", size),
            file("file", "part", data),
        ];
        if let Some(value) = checksum {
            fields.push(text("checksum", value));
        }
        fields
    }

    fn text(name: &str, value: impl ToString) -> FormDataField {
        FormDataField {
            name: name.to_string(),
            value: FormDataValue::Text(value.to_string()),
        }
    }

    fn file(name: &str, filename: &str, data: Vec<u8>) -> FormDataField {
        FormDataField {
            name: name.to_string(),
            value: FormDataValue::File {
                filename: filename.to_string(),
                data,
                content_type: None,
            },
        }
    }
}

// ── Iterators ──

#[derive(Debug, Clone)]
pub struct FileCommentIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<FileComment>,
    file_token: String,
    file_type: String,
    is_whole: Option<bool>,
    is_solved: Option<bool>,
    user_id_type: Option<String>,
    page_size: Option<i32>,
}

impl<'a> FileCommentIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.state = self.state.with_page_token(Some(page_token.into()));
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<FileComment>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = FileCommentResource {
            config: self.config,
        };
        let resp = resource
            .list(
                &self.file_token,
                &self.file_type,
                self.is_whole,
                self.is_solved,
                self.state.page_token_for_request(),
                self.page_size,
                self.user_id_type.as_deref(),
                option,
            )
            .await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(data.items, data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct FileCommentReplyIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<FileCommentReply>,
    file_token: String,
    comment_id: String,
    file_type: String,
    user_id_type: Option<String>,
    page_size: Option<i32>,
}

impl<'a> FileCommentReplyIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.state = self.state.with_page_token(Some(page_token.into()));
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<FileCommentReply>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = FileCommentReplyResource {
            config: self.config,
        };
        let resp = resource
            .list(
                &self.file_token,
                &self.comment_id,
                &self.file_type,
                self.page_size,
                self.state.page_token_for_request(),
                self.user_id_type.as_deref(),
                option,
            )
            .await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(data.items, data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct FileVersionIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<Version>,
    file_token: String,
    obj_type: String,
    user_id_type: Option<String>,
    page_size: Option<i32>,
}

impl<'a> FileVersionIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.state = self.state.with_page_token(Some(page_token.into()));
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<Version>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = FileVersionResource {
            config: self.config,
        };
        let resp = resource
            .list(
                &self.file_token,
                &self.obj_type,
                self.page_size,
                self.state.page_token_for_request(),
                self.user_id_type.as_deref(),
                option,
            )
            .await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(data.items, data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

#[derive(Debug, Clone)]
pub struct FileViewRecordIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<FileViewRecord>,
    file_token: String,
    file_type: String,
    viewer_id_type: Option<String>,
    page_size: Option<i32>,
}

impl<'a> FileViewRecordIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.state = self.state.with_page_token(Some(page_token.into()));
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<FileViewRecord>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = FileViewRecordResource {
            config: self.config,
        };
        let resp = resource
            .list(
                &self.file_token,
                &self.file_type,
                self.page_size,
                self.state.page_token_for_request(),
                self.viewer_id_type.as_deref(),
                option,
            )
            .await?;
        let data = resp.data.unwrap_or_default();
        self.state
            .accept_page(data.items, data.page_token, data.has_more);
        Ok(self.state.pop())
    }
}

// ── Resources ──

pub struct ExportTaskResource<'a> {
    config: &'a Config,
}

impl<'a> ExportTaskResource<'a> {
    pub async fn create(
        &self,
        body: &ExportTask,
        option: &RequestOption,
    ) -> Result<CreateExportTaskResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/export_tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateExportTaskRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateExportTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        ticket: &str,
        token: &str,
        option: &RequestOption,
    ) -> Result<GetExportTaskResp, LarkError> {
        let path = format!("/open-apis/drive/v1/export_tasks/{ticket}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("token", token);
        let (api_resp, raw) =
            transport::request_typed::<GetExportTaskRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetExportTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn download(
        &self,
        file_token: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/drive/v1/export_tasks/file/{file_token}/download");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .download()
        .await
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListFileQuery<'a> {
    pub folder_token: Option<&'a str>,
    pub order_by: Option<&'a str>,
    pub direction: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
}

impl<'a> ListFileQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn folder_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.folder_token = value.into();
        self
    }

    pub fn order_by(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.order_by = value.into();
        self
    }

    pub fn direction(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.direction = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
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

impl<'a> FileResource<'a> {
    pub async fn copy(
        &self,
        file_token: &str,
        user_id_type: Option<&str>,
        body: &CopyFileReqBody,
        option: &RequestOption,
    ) -> Result<CopyFileResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/copy");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CopyFileRespData>(self.config, &api_req, option).await?;
        Ok(CopyFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create_folder(
        &self,
        body: &CreateFolderFileReqBody,
        option: &RequestOption,
    ) -> Result<CreateFolderFileResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/drive/v1/files/create_folder",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateFolderFileRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateFolderFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create_shortcut(
        &self,
        user_id_type: Option<&str>,
        body: &CreateShortcutFileReqBody,
        option: &RequestOption,
    ) -> Result<CreateShortcutFileResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/drive/v1/files/create_shortcut",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateShortcutFileRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateShortcutFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        file_token: &str,
        file_type: &str,
        option: &RequestOption,
    ) -> Result<DeleteFileResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", file_type);
        let (api_resp, raw) =
            transport::request_typed::<DeleteFileRespData>(self.config, &api_req, option).await?;
        Ok(DeleteFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn download(
        &self,
        file_token: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/download");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .download()
        .await
    }

    pub async fn subscribe(
        &self,
        file_token: &str,
        file_type: &str,
        event_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/subscribe");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("file_type", file_type);
        if let Some(v) = event_type {
            api_req.query_params.set("event_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete_subscribe(
        &self,
        file_token: &str,
        file_type: &str,
        event_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/delete_subscribe");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("file_type", file_type);
        if let Some(v) = event_type {
            api_req.query_params.set("event_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get_subscribe(
        &self,
        file_token: &str,
        file_type: &str,
        event_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetSubscribeFileResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/get_subscribe");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("file_type", file_type);
        if let Some(v) = event_type {
            api_req.query_params.set("event_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GetSubscribeFileRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetSubscribeFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        folder_token: Option<&str>,
        order_by: Option<&str>,
        direction: Option<&str>,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFileResp, LarkError> {
        let query = ListFileQuery::new()
            .folder_token(folder_token)
            .order_by(order_by)
            .direction(direction)
            .user_id_type(user_id_type)
            .page_size(page_size)
            .page_token(page_token);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFileQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFileResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/drive/v1/files",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("folder_token", query.folder_token)
        .query("order_by", query.order_by)
        .query("direction", query.direction)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page_query())
        .send::<ListFileRespData>()
        .await?;
        Ok(ListFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn move_file(
        &self,
        file_token: &str,
        body: &MoveFileReqBody,
        option: &RequestOption,
    ) -> Result<MoveFileResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/move");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MoveFileRespData>(self.config, &api_req, option).await?;
        Ok(MoveFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn task_check(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<TaskCheckFileResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/drive/v1/files/task_check");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("task_id", task_id);
        let (api_resp, raw) =
            transport::request_typed::<TaskCheckFileRespData>(self.config, &api_req, option)
                .await?;
        Ok(TaskCheckFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn upload_prepare(
        &self,
        body: &FileUploadInfo,
        option: &RequestOption,
    ) -> Result<UploadPrepareFileResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/drive/v1/files/upload_prepare",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UploadPrepareFileRespData>(self.config, &api_req, option)
                .await?;
        Ok(UploadPrepareFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn upload_finish(
        &self,
        body: &UploadFinishFileReqBody,
        option: &RequestOption,
    ) -> Result<UploadFinishFileResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/drive/v1/files/upload_finish",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UploadFinishFileRespData>(self.config, &api_req, option)
                .await?;
        Ok(UploadFinishFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn upload_all(
        &self,
        file_name: &str,
        parent_type: &str,
        parent_node: &str,
        size: i64,
        checksum: Option<&str>,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<UploadAllFileResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/files/upload_all");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let fields = multipart_form::upload_all_fields(
            file_name,
            parent_type,
            parent_node,
            size,
            checksum,
            None,
            data,
        );
        api_req.body = Some(ReqBody::FormData(fields));
        let (api_resp, raw) =
            transport::request_typed::<UploadAllFileRespData>(self.config, &api_req, option)
                .await?;
        Ok(UploadAllFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn upload_part(
        &self,
        upload_id: &str,
        seq: i32,
        size: i64,
        checksum: Option<&str>,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/files/upload_part");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let fields = multipart_form::upload_part_fields(upload_id, seq, size, checksum, data);
        api_req.body = Some(ReqBody::FormData(fields));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct FileCommentResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchQueryFileCommentQuery<'a> {
    pub file_token: &'a str,
    pub file_type: &'a str,
    pub user_id_type: Option<&'a str>,
    pub body: &'a BatchQueryFileCommentReqBody,
}

impl<'a> BatchQueryFileCommentQuery<'a> {
    pub fn new(
        file_token: &'a str,
        file_type: &'a str,
        body: &'a BatchQueryFileCommentReqBody,
    ) -> Self {
        Self {
            file_token,
            file_type,
            user_id_type: None,
            body,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetFileCommentQuery<'a> {
    pub file_token: &'a str,
    pub comment_id: &'a str,
    pub file_type: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetFileCommentQuery<'a> {
    pub fn new(file_token: &'a str, comment_id: &'a str, file_type: &'a str) -> Self {
        Self {
            file_token,
            comment_id,
            file_type,
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
pub struct ListFileCommentQuery<'a> {
    pub file_token: &'a str,
    pub file_type: &'a str,
    pub is_whole: Option<bool>,
    pub is_solved: Option<bool>,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListFileCommentQuery<'a> {
    pub fn new(file_token: &'a str, file_type: &'a str) -> Self {
        Self {
            file_token,
            file_type,
            is_whole: None,
            is_solved: None,
            page: PageQuery::new(),
            user_id_type: None,
        }
    }

    pub fn is_whole(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_whole = value.into();
        self
    }

    pub fn is_solved(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_solved = value.into();
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> FileCommentResource<'a> {
    pub async fn batch_query(
        &self,
        file_token: &str,
        file_type: &str,
        user_id_type: Option<&str>,
        body: &BatchQueryFileCommentReqBody,
        option: &RequestOption,
    ) -> Result<BatchQueryFileCommentResp, LarkError> {
        let query =
            BatchQueryFileCommentQuery::new(file_token, file_type, body).user_id_type(user_id_type);
        self.batch_query_by_query(&query, option).await
    }

    pub async fn batch_query_by_query(
        &self,
        query: &BatchQueryFileCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchQueryFileCommentResp, LarkError> {
        let path = format!(
            "/open-apis/drive/v1/files/{}/comments/batch_query",
            query.file_token
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("file_type", query.file_type)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<BatchQueryFileCommentRespData>()
        .await?;
        Ok(BatchQueryFileCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create(
        &self,
        file_token: &str,
        file_type: &str,
        user_id_type: Option<&str>,
        body: &FileComment,
        option: &RequestOption,
    ) -> Result<CreateFileCommentResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/comments");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("file_type", file_type);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FileComment>(self.config, &api_req, option).await?;
        Ok(CreateFileCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        file_token: &str,
        comment_id: &str,
        file_type: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetFileCommentResp, LarkError> {
        let query =
            GetFileCommentQuery::new(file_token, comment_id, file_type).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetFileCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetFileCommentResp, LarkError> {
        let path = format!(
            "/open-apis/drive/v1/files/{}/comments/{}",
            query.file_token, query.comment_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("file_type", query.file_type)
        .query("user_id_type", query.user_id_type)
        .send::<FileComment>()
        .await?;
        Ok(GetFileCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        file_token: &str,
        file_type: &str,
        is_whole: Option<bool>,
        is_solved: Option<bool>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFileCommentResp, LarkError> {
        let query = ListFileCommentQuery::new(file_token, file_type)
            .is_whole(is_whole)
            .is_solved(is_solved)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFileCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFileCommentResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{}/comments", query.file_token);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("file_type", query.file_type)
        .query("is_whole", query.is_whole)
        .query("is_solved", query.is_solved)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send::<ListFileCommentRespData>()
        .await?;
        Ok(ListFileCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        file_token: &str,
        file_type: &str,
        is_whole: Option<bool>,
        is_solved: Option<bool>,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
    ) -> FileCommentIterator<'a> {
        FileCommentIterator {
            config: self.config,
            state: PageIteratorState::default(),
            file_token: file_token.to_owned(),
            file_type: file_type.to_owned(),
            is_whole,
            is_solved,
            user_id_type: user_id_type.map(|v| v.to_owned()),
            page_size,
        }
    }

    pub async fn patch(
        &self,
        file_token: &str,
        comment_id: &str,
        file_type: &str,
        body: &PatchFileCommentReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/comments/{comment_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("file_type", file_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct FileCommentReplyResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListFileCommentReplyQuery<'a> {
    pub file_token: &'a str,
    pub comment_id: &'a str,
    pub file_type: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListFileCommentReplyQuery<'a> {
    pub fn new(file_token: &'a str, comment_id: &'a str, file_type: &'a str) -> Self {
        Self {
            file_token,
            comment_id,
            file_type,
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

impl<'a> FileCommentReplyResource<'a> {
    pub async fn delete(
        &self,
        file_token: &str,
        comment_id: &str,
        reply_id: &str,
        file_type: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/drive/v1/files/{file_token}/comments/{comment_id}/replies/{reply_id}"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("file_type", file_type);
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
        file_token: &str,
        comment_id: &str,
        file_type: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFileCommentReplyResp, LarkError> {
        let query = ListFileCommentReplyQuery::new(file_token, comment_id, file_type)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFileCommentReplyQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFileCommentReplyResp, LarkError> {
        let path = format!(
            "/open-apis/drive/v1/files/{}/comments/{}/replies",
            query.file_token, query.comment_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("file_type", query.file_type)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send::<ListFileCommentReplyRespData>()
        .await?;
        Ok(ListFileCommentReplyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        file_token: &str,
        comment_id: &str,
        file_type: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
    ) -> FileCommentReplyIterator<'a> {
        FileCommentReplyIterator {
            config: self.config,
            state: PageIteratorState::default(),
            file_token: file_token.to_owned(),
            comment_id: comment_id.to_owned(),
            file_type: file_type.to_owned(),
            user_id_type: user_id_type.map(|v| v.to_owned()),
            page_size,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        file_token: &str,
        comment_id: &str,
        reply_id: &str,
        file_type: &str,
        user_id_type: Option<&str>,
        body: &UpdateFileCommentReplyReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/drive/v1/files/{file_token}/comments/{comment_id}/replies/{reply_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("file_type", file_type);
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

pub struct FileStatisticsResource<'a> {
    config: &'a Config,
}

impl<'a> FileStatisticsResource<'a> {
    pub async fn get(
        &self,
        file_token: &str,
        file_type: &str,
        option: &RequestOption,
    ) -> Result<GetFileStatisticsResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/statistics");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("file_type", file_type);
        let (api_resp, raw) =
            transport::request_typed::<GetFileStatisticsRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetFileStatisticsResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct FileSubscriptionResource<'a> {
    config: &'a Config,
}

impl<'a> FileSubscriptionResource<'a> {
    pub async fn create(
        &self,
        file_token: &str,
        body: &FileSubscription,
        option: &RequestOption,
    ) -> Result<CreateFileSubscriptionResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/subscriptions");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FileSubscriptionRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateFileSubscriptionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        file_token: &str,
        subscription_id: &str,
        body: &GetFileSubscriptionReqBody,
        option: &RequestOption,
    ) -> Result<GetFileSubscriptionResp, LarkError> {
        let path =
            format!("/open-apis/drive/v1/files/{file_token}/subscriptions/{subscription_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FileSubscriptionRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetFileSubscriptionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        file_token: &str,
        subscription_id: &str,
        body: &PatchFileSubscriptionReqBody,
        option: &RequestOption,
    ) -> Result<PatchFileSubscriptionResp, LarkError> {
        let path =
            format!("/open-apis/drive/v1/files/{file_token}/subscriptions/{subscription_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FileSubscriptionRespData>(self.config, &api_req, option)
                .await?;
        Ok(PatchFileSubscriptionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct FileVersionResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListFileVersionQuery<'a> {
    pub file_token: &'a str,
    pub obj_type: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListFileVersionQuery<'a> {
    pub fn new(file_token: &'a str, obj_type: &'a str) -> Self {
        Self {
            file_token,
            obj_type,
            page_size: None,
            page_token: None,
            user_id_type: None,
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

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> FileVersionResource<'a> {
    pub async fn create(
        &self,
        file_token: &str,
        user_id_type: Option<&str>,
        body: &Version,
        option: &RequestOption,
    ) -> Result<CreateFileVersionResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/versions");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateFileVersionRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateFileVersionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        file_token: &str,
        version_id: &str,
        obj_type: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/versions/{version_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("obj_type", obj_type);
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

    pub async fn get(
        &self,
        file_token: &str,
        version_id: &str,
        obj_type: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetFileVersionResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{file_token}/versions/{version_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("obj_type", obj_type);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<Version>(self.config, &api_req, option).await?;
        Ok(GetFileVersionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        file_token: &str,
        obj_type: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFileVersionResp, LarkError> {
        let query = ListFileVersionQuery::new(file_token, obj_type)
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFileVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFileVersionResp, LarkError> {
        let path = format!("/open-apis/drive/v1/files/{}/versions", query.file_token);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("obj_type", query.obj_type)
        .page_query(query.page_query())
        .query("user_id_type", query.user_id_type)
        .send::<ListFileVersionRespData>()
        .await?;
        Ok(ListFileVersionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        file_token: &str,
        obj_type: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
    ) -> FileVersionIterator<'a> {
        FileVersionIterator {
            config: self.config,
            state: PageIteratorState::default(),
            file_token: file_token.to_owned(),
            obj_type: obj_type.to_owned(),
            user_id_type: user_id_type.map(|v| v.to_owned()),
            page_size,
        }
    }
}

pub struct FileViewRecordResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListFileViewRecordQuery<'a> {
    pub file_token: &'a str,
    pub file_type: &'a str,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub viewer_id_type: Option<&'a str>,
}

impl<'a> ListFileViewRecordQuery<'a> {
    pub fn new(file_token: &'a str, file_type: &'a str) -> Self {
        Self {
            file_token,
            file_type,
            page_size: None,
            page_token: None,
            viewer_id_type: None,
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

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn viewer_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.viewer_id_type = value.into();
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }
}

impl<'a> FileViewRecordResource<'a> {
    pub async fn list(
        &self,
        file_token: &str,
        file_type: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        viewer_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFileViewRecordResp, LarkError> {
        let query = ListFileViewRecordQuery::new(file_token, file_type)
            .page_size(page_size)
            .page_token(page_token)
            .viewer_id_type(viewer_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFileViewRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFileViewRecordResp, LarkError> {
        let path = format!(
            "/open-apis/drive/v1/files/{}/view_records",
            query.file_token
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("file_type", query.file_type)
        .page_query(query.page_query())
        .query("viewer_id_type", query.viewer_id_type)
        .send::<ListFileViewRecordRespData>()
        .await?;
        Ok(ListFileViewRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        file_token: &str,
        file_type: &str,
        viewer_id_type: Option<&str>,
        page_size: Option<i32>,
    ) -> FileViewRecordIterator<'a> {
        FileViewRecordIterator {
            config: self.config,
            state: PageIteratorState::default(),
            file_token: file_token.to_owned(),
            file_type: file_type.to_owned(),
            viewer_id_type: viewer_id_type.map(|v| v.to_owned()),
            page_size,
        }
    }
}

pub struct ImportTaskResource<'a> {
    config: &'a Config,
}

impl<'a> ImportTaskResource<'a> {
    pub async fn create(
        &self,
        body: &ImportTask,
        option: &RequestOption,
    ) -> Result<CreateImportTaskResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/import_tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateImportTaskRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateImportTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        ticket: &str,
        option: &RequestOption,
    ) -> Result<GetImportTaskResp, LarkError> {
        let path = format!("/open-apis/drive/v1/import_tasks/{ticket}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<GetImportTaskRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetImportTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct MediaResource<'a> {
    config: &'a Config,
}

impl<'a> MediaResource<'a> {
    pub async fn batch_get_tmp_download_url(
        &self,
        file_tokens: &[&str],
        extra: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchGetTmpDownloadUrlMediaResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/drive/v1/medias/batch_get_tmp_download_url",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        for t in file_tokens {
            api_req.query_params.add("file_tokens", *t);
        }
        if let Some(v) = extra {
            api_req.query_params.set("extra", v);
        }
        let (api_resp, raw) = transport::request_typed::<BatchGetTmpDownloadUrlMediaRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(BatchGetTmpDownloadUrlMediaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn download(
        &self,
        file_token: &str,
        extra: Option<&str>,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/drive/v1/medias/{file_token}/download");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("extra", extra)
        .download()
        .await
    }

    pub async fn upload_prepare(
        &self,
        body: &MediaUploadInfo,
        option: &RequestOption,
    ) -> Result<UploadPrepareMediaResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/drive/v1/medias/upload_prepare",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UploadPrepareMediaRespData>(self.config, &api_req, option)
                .await?;
        Ok(UploadPrepareMediaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn upload_finish(
        &self,
        body: &UploadFinishMediaReqBody,
        option: &RequestOption,
    ) -> Result<UploadFinishMediaResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/drive/v1/medias/upload_finish",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UploadFinishMediaRespData>(self.config, &api_req, option)
                .await?;
        Ok(UploadFinishMediaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn upload_all(
        &self,
        file_name: &str,
        parent_type: &str,
        parent_node: &str,
        size: i64,
        checksum: Option<&str>,
        extra: Option<&str>,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<UploadAllMediaResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/medias/upload_all");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let fields = multipart_form::upload_all_fields(
            file_name,
            parent_type,
            parent_node,
            size,
            checksum,
            extra,
            data,
        );
        api_req.body = Some(ReqBody::FormData(fields));
        let (api_resp, raw) =
            transport::request_typed::<UploadAllMediaRespData>(self.config, &api_req, option)
                .await?;
        Ok(UploadAllMediaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn upload_part(
        &self,
        upload_id: &str,
        seq: i32,
        size: i64,
        checksum: Option<&str>,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/medias/upload_part");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let fields = multipart_form::upload_part_fields(upload_id, seq, size, checksum, data);
        api_req.body = Some(ReqBody::FormData(fields));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct MetaResource<'a> {
    config: &'a Config,
}

impl<'a> MetaResource<'a> {
    pub async fn batch_query(
        &self,
        user_id_type: Option<&str>,
        body: &MetaRequest,
        option: &RequestOption,
    ) -> Result<BatchQueryMetaResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/metas/batch_query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchQueryMetaRespData>(self.config, &api_req, option)
                .await?;
        Ok(BatchQueryMetaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct PermissionMemberResource<'a> {
    config: &'a Config,
}

impl<'a> PermissionMemberResource<'a> {
    pub async fn auth(
        &self,
        token: &str,
        token_type: &str,
        action: &str,
        option: &RequestOption,
    ) -> Result<AuthPermissionMemberResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/members/auth");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        api_req.query_params.set("action", action);
        let (api_resp, raw) =
            transport::request_typed::<AuthPermissionMemberRespData>(self.config, &api_req, option)
                .await?;
        Ok(AuthPermissionMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_create(
        &self,
        token: &str,
        token_type: &str,
        need_notification: Option<bool>,
        body: &BatchCreatePermissionMemberReqBody,
        option: &RequestOption,
    ) -> Result<BatchCreatePermissionMemberResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/members/batch_create");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        if let Some(v) = need_notification {
            api_req.query_params.set("need_notification", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<BatchCreatePermissionMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(BatchCreatePermissionMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create(
        &self,
        token: &str,
        token_type: &str,
        need_notification: Option<bool>,
        body: &BaseMember,
        option: &RequestOption,
    ) -> Result<CreatePermissionMemberResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/members");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        if let Some(v) = need_notification {
            api_req.query_params.set("need_notification", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<CreatePermissionMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(CreatePermissionMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        token: &str,
        member_id: &str,
        token_type: &str,
        member_type: &str,
        body: &DeletePermissionMemberReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/members/{member_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        api_req.query_params.set("member_type", member_type);
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
        token: &str,
        token_type: &str,
        fields: Option<&str>,
        perm_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPermissionMemberResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/members");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        if let Some(v) = fields {
            api_req.query_params.set("fields", v);
        }
        if let Some(v) = perm_type {
            api_req.query_params.set("perm_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListPermissionMemberRespData>(self.config, &api_req, option)
                .await?;
        Ok(ListPermissionMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn transfer_owner(
        &self,
        token: &str,
        token_type: &str,
        need_notification: Option<bool>,
        remove_old_owner: Option<bool>,
        stay_put: Option<bool>,
        old_owner_perm: Option<&str>,
        body: &Owner,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/members/transfer_owner");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        if let Some(v) = need_notification {
            api_req.query_params.set("need_notification", v.to_string());
        }
        if let Some(v) = remove_old_owner {
            api_req.query_params.set("remove_old_owner", v.to_string());
        }
        if let Some(v) = stay_put {
            api_req.query_params.set("stay_put", v.to_string());
        }
        if let Some(v) = old_owner_perm {
            api_req.query_params.set("old_owner_perm", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn update(
        &self,
        token: &str,
        member_id: &str,
        token_type: &str,
        need_notification: Option<bool>,
        body: &BaseMember,
        option: &RequestOption,
    ) -> Result<UpdatePermissionMemberResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/members/{member_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        if let Some(v) = need_notification {
            api_req.query_params.set("need_notification", v.to_string());
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<UpdatePermissionMemberRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(UpdatePermissionMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct PermissionPublicResource<'a> {
    config: &'a Config,
}

impl<'a> PermissionPublicResource<'a> {
    pub async fn get(
        &self,
        token: &str,
        token_type: &str,
        option: &RequestOption,
    ) -> Result<GetPermissionPublicV1Resp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/public");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        let (api_resp, raw) = transport::request_typed::<GetPermissionPublicV1RespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(GetPermissionPublicV1Resp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        token: &str,
        token_type: &str,
        body: &PermissionPublicV1,
        option: &RequestOption,
    ) -> Result<PatchPermissionPublicV1Resp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/public");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<GetPermissionPublicV1RespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(PatchPermissionPublicV1Resp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct PermissionPublicPasswordResource<'a> {
    config: &'a Config,
}

impl<'a> PermissionPublicPasswordResource<'a> {
    pub async fn create(
        &self,
        token: &str,
        token_type: &str,
        option: &RequestOption,
    ) -> Result<CreatePermissionPublicPasswordResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/public/password");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        let (api_resp, raw) = transport::request_typed::<CreatePermissionPublicPasswordRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(CreatePermissionPublicPasswordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        token: &str,
        token_type: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/public/password");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn update(
        &self,
        token: &str,
        token_type: &str,
        option: &RequestOption,
    ) -> Result<UpdatePermissionPublicPasswordResp, LarkError> {
        let path = format!("/open-apis/drive/v1/permissions/{token}/public/password");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("type", token_type);
        let (api_resp, raw) = transport::request_typed::<UpdatePermissionPublicPasswordRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(UpdatePermissionPublicPasswordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub export_task: ExportTaskResource<'a>,
    pub file: FileResource<'a>,
    pub file_comment: FileCommentResource<'a>,
    pub file_comment_reply: FileCommentReplyResource<'a>,
    pub file_statistics: FileStatisticsResource<'a>,
    pub file_subscription: FileSubscriptionResource<'a>,
    pub file_version: FileVersionResource<'a>,
    pub file_view_record: FileViewRecordResource<'a>,
    pub import_task: ImportTaskResource<'a>,
    pub media: MediaResource<'a>,
    pub meta: MetaResource<'a>,
    pub permission_member: PermissionMemberResource<'a>,
    pub permission_public: PermissionPublicResource<'a>,
    pub permission_public_password: PermissionPublicPasswordResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            export_task: ExportTaskResource { config },
            file: FileResource { config },
            file_comment: FileCommentResource { config },
            file_comment_reply: FileCommentReplyResource { config },
            file_statistics: FileStatisticsResource { config },
            file_subscription: FileSubscriptionResource { config },
            file_version: FileVersionResource { config },
            file_view_record: FileViewRecordResource { config },
            import_task: ImportTaskResource { config },
            media: MediaResource { config },
            meta: MetaResource { config },
            permission_member: PermissionMemberResource { config },
            permission_public: PermissionPublicResource { config },
            permission_public_password: PermissionPublicPasswordResource { config },
        }
    }
}
