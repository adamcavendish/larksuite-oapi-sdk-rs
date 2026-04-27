use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, FormDataField, FormDataValue, ReqBody, RequestOption};
use crate::service::common::{DownloadResp, EmptyResp};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nNames {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Emoji {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Operator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sender {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mention {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<MessageBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mention>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReaction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaction_type: Option<Emoji>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchMessageSendProgress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_user_ids_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_user_ids_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_user_ids_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchMessageRecallProgress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recall: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recall_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchMessageReadUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListModerator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatManagers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RestrictedModeSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screenshot_has_permission_setting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_has_permission_setting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_has_permission_setting: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTabConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_built_in: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTabContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_minute: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTab {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_content: Option<ChatTabContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_config: Option<ChatTabConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTopNotice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMenuItemRedirectLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMenuItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_link: Option<ChatMenuItemRedirectLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<I18nNames>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMenuSecondLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_menu_second_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_menu_item: Option<ChatMenuItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMenuTopLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_menu_top_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_menu_item: Option<ChatMenuItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChatMenuSecondLevel>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMenuTree {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_menu_top_levels: Option<Vec<ChatMenuTopLevel>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FollowUp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_contents: Option<Vec<I18nContent>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ReplyMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_in_thread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ForwardMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MergeForwardMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UrgentMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PushFollowUpReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_ups: Option<Vec<FollowUp>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMessageReactionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_type: Option<Emoji>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePinReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChatReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<I18nNames>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_message_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_message_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_approval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_mode_setting: Option<RestrictedModeSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent_setting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_conference_setting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_manage_setting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_member_count_setting: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateChatReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<I18nNames>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_member_permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_card_permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_all_permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_message_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_message_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_approval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_mode_setting: Option<RestrictedModeSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent_setting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_conference_setting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_manage_setting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_member_count_setting: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LinkChatReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChatMembersReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteChatMembersReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateChatManagersReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchChatAnnouncementReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateChatModerationReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_setting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator_added_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator_deleted_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ChatTabsReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_tabs: Option<Vec<ChatTab>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteChatTabsReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SortChatTabsReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutChatTopNoticeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_top_notice: Option<Vec<ChatTopNotice>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchChatMenuItemReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_menu_item: Option<ChatMenuItem>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChatMenuTreeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_tree: Option<ChatMenuTree>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteChatMenuTreeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_menu_top_level_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SortChatMenuTreeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_menu_top_level_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ForwardThreadReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_id: Option<String>,
}

// ── Response data types ──

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MessageRespData {
    pub message_id: Option<String>,
    pub root_id: Option<String>,
    pub parent_id: Option<String>,
    pub thread_id: Option<String>,
    pub msg_type: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub deleted: Option<bool>,
    pub updated: Option<bool>,
    pub chat_id: Option<String>,
    pub sender: Option<Sender>,
    pub body: Option<MessageBody>,
    pub mentions: Option<Vec<Mention>>,
    pub upper_message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetMessageRespData {
    pub items: Option<Vec<Message>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListMessageRespData {
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
    pub items: Option<Vec<Message>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MergeForwardMessageRespData {
    pub message: Option<Message>,
    pub invalid_message_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReadUsersMessageRespData {
    pub items: Option<Vec<ReadUser>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UrgentMessageRespData {
    pub invalid_user_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateMessageReactionRespData {
    pub reaction_id: Option<String>,
    pub operator: Option<Operator>,
    pub action_time: Option<String>,
    pub reaction_type: Option<Emoji>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DeleteMessageReactionRespData {
    pub reaction_id: Option<String>,
    pub operator: Option<Operator>,
    pub action_time: Option<String>,
    pub reaction_type: Option<Emoji>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListMessageReactionRespData {
    pub items: Option<Vec<MessageReaction>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetProgressBatchMessageRespData {
    pub batch_message_send_progress: Option<BatchMessageSendProgress>,
    pub batch_message_recall_progress: Option<BatchMessageRecallProgress>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReadUserBatchMessageRespData {
    pub read_user: Option<BatchMessageReadUser>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreatePinRespData {
    pub pin: Option<Pin>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListPinRespData {
    pub items: Option<Vec<Pin>>,
    pub has_more: Option<bool>,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateChatRespData {
    pub chat_id: Option<String>,
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub i18n_names: Option<I18nNames>,
    pub owner_id: Option<String>,
    pub owner_id_type: Option<String>,
    pub urgent_setting: Option<String>,
    pub video_conference_setting: Option<String>,
    pub pin_manage_setting: Option<String>,
    pub add_member_permission: Option<String>,
    pub share_card_permission: Option<String>,
    pub at_all_permission: Option<String>,
    pub edit_permission: Option<String>,
    pub group_message_type: Option<String>,
    pub chat_mode: Option<String>,
    pub chat_type: Option<String>,
    pub chat_tag: Option<String>,
    pub external: Option<bool>,
    pub tenant_key: Option<String>,
    pub join_message_visibility: Option<String>,
    pub leave_message_visibility: Option<String>,
    pub membership_approval: Option<String>,
    pub moderation_permission: Option<String>,
    pub labels: Option<Vec<String>>,
    pub toolkit_ids: Option<Vec<String>>,
    pub restricted_mode_setting: Option<RestrictedModeSetting>,
    pub hide_member_count_setting: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetChatRespData {
    pub chat_id: Option<String>,
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub i18n_names: Option<I18nNames>,
    pub owner_id: Option<String>,
    pub owner_id_type: Option<String>,
    pub urgent_setting: Option<String>,
    pub video_conference_setting: Option<String>,
    pub pin_manage_setting: Option<String>,
    pub add_member_permission: Option<String>,
    pub share_card_permission: Option<String>,
    pub at_all_permission: Option<String>,
    pub edit_permission: Option<String>,
    pub group_message_type: Option<String>,
    pub chat_mode: Option<String>,
    pub chat_type: Option<String>,
    pub chat_tag: Option<String>,
    pub external: Option<bool>,
    pub tenant_key: Option<String>,
    pub join_message_visibility: Option<String>,
    pub leave_message_visibility: Option<String>,
    pub membership_approval: Option<String>,
    pub moderation_permission: Option<String>,
    pub labels: Option<Vec<String>>,
    pub toolkit_ids: Option<Vec<String>>,
    pub restricted_mode_setting: Option<RestrictedModeSetting>,
    pub hide_member_count_setting: Option<String>,
    pub user_manager_id_list: Option<Vec<String>>,
    pub bot_manager_id_list: Option<Vec<String>>,
    pub user_count: Option<String>,
    pub bot_count: Option<String>,
    pub chat_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LinkChatRespData {
    pub share_link: Option<String>,
    pub expire_time: Option<String>,
    pub is_permanent: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListChatRespData {
    pub items: Option<Vec<ListChat>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateChatMembersRespData {
    pub invalid_id_list: Option<Vec<String>>,
    pub not_existed_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetChatMembersRespData {
    pub items: Option<Vec<ListMember>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
    pub member_total: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct IsInChatChatMembersRespData {
    pub is_in_chat: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateChatManagersRespData {
    pub chat_managers: Option<Vec<ChatManagers>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetChatAnnouncementRespData {
    pub content: Option<String>,
    pub revision: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub owner_id_type: Option<String>,
    pub owner_id: Option<String>,
    pub modifier_id_type: Option<String>,
    pub modifier_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GetChatModerationRespData {
    pub moderation_setting: Option<String>,
    pub items: Option<Vec<ListModerator>>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChatTabsRespData {
    pub chat_tabs: Option<Vec<ChatTab>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PatchChatMenuItemRespData {
    pub chat_menu_item: Option<ChatMenuItem>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChatMenuTreeRespData {
    pub menu_tree: Option<ChatMenuTree>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateFileRespData {
    pub file_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateImageRespData {
    pub image_key: Option<String>,
}

// ── Response wrappers ──

impl_resp!(CreateMessageResp, MessageRespData);
impl_resp!(ReplyMessageResp, MessageRespData);
impl_resp!(UpdateMessageResp, MessageRespData);
impl_resp!(ForwardMessageResp, MessageRespData);
impl_resp!(MergeForwardMessageResp, MergeForwardMessageRespData);
impl_resp!(GetMessageResp, GetMessageRespData);
impl_resp!(ListMessageResp, ListMessageRespData);
impl_resp!(ReadUsersMessageResp, ReadUsersMessageRespData);
impl_resp!(UrgentAppMessageResp, UrgentMessageRespData);
impl_resp!(UrgentPhoneMessageResp, UrgentMessageRespData);
impl_resp!(UrgentSmsMessageResp, UrgentMessageRespData);
impl_resp!(CreateMessageReactionResp, CreateMessageReactionRespData);
impl_resp!(DeleteMessageReactionResp, DeleteMessageReactionRespData);
impl_resp!(ListMessageReactionResp, ListMessageReactionRespData);
impl_resp!(GetProgressBatchMessageResp, GetProgressBatchMessageRespData);
impl_resp!(ReadUserBatchMessageResp, ReadUserBatchMessageRespData);
impl_resp!(CreatePinResp, CreatePinRespData);
impl_resp!(ListPinResp, ListPinRespData);
impl_resp!(CreateChatResp, CreateChatRespData);
impl_resp!(GetChatResp, GetChatRespData);
impl_resp!(LinkChatResp, LinkChatRespData);
impl_resp!(ListChatResp, ListChatRespData);
impl_resp!(CreateChatMembersResp, CreateChatMembersRespData);
impl_resp!(GetChatMembersResp, GetChatMembersRespData);
impl_resp!(IsInChatChatMembersResp, IsInChatChatMembersRespData);
impl_resp!(AddManagersChatManagersResp, UpdateChatManagersRespData);
impl_resp!(DeleteManagersChatManagersResp, UpdateChatManagersRespData);
impl_resp!(GetChatAnnouncementResp, GetChatAnnouncementRespData);
impl_resp!(GetChatModerationResp, GetChatModerationRespData);
impl_resp!(CreateChatTabResp, ChatTabsRespData);
impl_resp!(DeleteTabsChatTabResp, ChatTabsRespData);
impl_resp!(ListTabsChatTabResp, ChatTabsRespData);
impl_resp!(SortTabsChatTabResp, ChatTabsRespData);
impl_resp!(UpdateTabsChatTabResp, ChatTabsRespData);
impl_resp!(PatchChatMenuItemResp, PatchChatMenuItemRespData);
impl_resp!(CreateChatMenuTreeResp, ChatMenuTreeRespData);
impl_resp!(DeleteChatMenuTreeResp, ChatMenuTreeRespData);
impl_resp!(GetChatMenuTreeResp, ChatMenuTreeRespData);
impl_resp!(SortChatMenuTreeResp, ChatMenuTreeRespData);
impl_resp!(CreateFileResp, CreateFileRespData);
impl_resp!(CreateImageResp, CreateImageRespData);
impl_resp!(ForwardThreadResp, MessageRespData);

#[derive(Debug, Clone)]
struct PageIteratorState<T> {
    next_page_token: Option<String>,
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

    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn page_token_for_request(&self) -> Option<&str> {
        if self.started {
            self.next_page_token.as_deref()
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
            && (!self.started || self.next_page_token.is_some())
    }

    fn accept_page(
        &mut self,
        items: Option<Vec<T>>,
        page_token: Option<String>,
        has_more: Option<bool>,
    ) {
        self.started = true;
        self.items = items.unwrap_or_default().into();
        self.next_page_token = page_token;
        self.exhausted =
            self.items.is_empty() || !has_more.unwrap_or(false) || self.next_page_token.is_none();
    }
}

// ── Iterators ──

#[derive(Debug, Clone)]
pub struct ListMessageIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<Message>,
    container_id_type: String,
    container_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
    sort_type: Option<String>,
    page_size: Option<i64>,
}

impl<'a> ListMessageIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<Message>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = MessageResource {
            config: self.config,
        };
        let resp = resource
            .list(
                &self.container_id_type,
                &self.container_id,
                self.start_time.as_deref(),
                self.end_time.as_deref(),
                self.sort_type.as_deref(),
                self.page_size,
                self.state.page_token_for_request(),
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
pub struct ListMessageReactionIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<MessageReaction>,
    message_id: String,
    reaction_type: Option<String>,
    page_size: Option<i64>,
    user_id_type: Option<String>,
}

impl<'a> ListMessageReactionIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<MessageReaction>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = MessageReactionResource {
            config: self.config,
        };
        let resp = resource
            .list(
                &self.message_id,
                self.reaction_type.as_deref(),
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
pub struct ListPinIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<Pin>,
    chat_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
    page_size: Option<i64>,
}

impl<'a> ListPinIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<Pin>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = PinResource {
            config: self.config,
        };
        let resp = resource
            .list(
                &self.chat_id,
                self.start_time.as_deref(),
                self.end_time.as_deref(),
                self.state.page_token_for_request(),
                self.page_size,
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
pub struct ListChatIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<ListChat>,
    user_id_type: Option<String>,
    sort_type: Option<String>,
    page_size: Option<i64>,
}

impl<'a> ListChatIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<ListChat>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = ChatResource {
            config: self.config,
        };
        let resp = resource
            .list(
                self.user_id_type.as_deref(),
                self.sort_type.as_deref(),
                self.state.page_token_for_request(),
                self.page_size,
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
pub struct SearchChatIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<ListChat>,
    user_id_type: Option<String>,
    query: Option<String>,
    page_size: Option<i64>,
}

impl<'a> SearchChatIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<ListChat>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = ChatResource {
            config: self.config,
        };
        let resp = resource
            .search(
                self.user_id_type.as_deref(),
                self.query.as_deref(),
                self.state.page_token_for_request(),
                self.page_size,
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
pub struct GetChatMembersIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<ListMember>,
    chat_id: String,
    member_id_type: Option<String>,
    page_size: Option<i64>,
}

impl<'a> GetChatMembersIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(&mut self, option: &RequestOption) -> Result<Option<ListMember>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = ChatMembersResource {
            config: self.config,
        };
        let resp = resource
            .get(
                &self.chat_id,
                self.member_id_type.as_deref(),
                self.state.page_token_for_request(),
                self.page_size,
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
pub struct GetChatModerationIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<ListModerator>,
    chat_id: String,
    user_id_type: Option<String>,
    page_size: Option<i64>,
}

impl<'a> GetChatModerationIterator<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.state = self.state.limit(limit);
        self
    }

    pub fn next_page_token(&self) -> Option<&str> {
        self.state.next_page_token()
    }

    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<ListModerator>, LarkError> {
        if let Some(item) = self.state.pop() {
            return Ok(Some(item));
        }
        if !self.state.should_fetch() {
            return Ok(None);
        }

        let resource = ChatModerationResource {
            config: self.config,
        };
        let resp = resource
            .get(
                &self.chat_id,
                self.user_id_type.as_deref(),
                self.state.page_token_for_request(),
                self.page_size,
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

#[derive(Debug, Clone)]
pub struct MessageResource<'a> {
    config: &'a Config,
}

impl<'a> MessageResource<'a> {
    pub async fn create(
        &self,
        receive_id_type: &str,
        body: &CreateMessageReqBody,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v1/messages");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("receive_id_type", receive_id_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MessageRespData>(self.config, &api_req, option).await?;
        Ok(CreateMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn reply(
        &self,
        message_id: &str,
        body: &ReplyMessageReqBody,
        option: &RequestOption,
    ) -> Result<ReplyMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/reply");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MessageRespData>(self.config, &api_req, option).await?;
        Ok(ReplyMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        message_id: &str,
        body: &PatchMessageReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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
        message_id: &str,
        body: &UpdateMessageReqBody,
        option: &RequestOption,
    ) -> Result<UpdateMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MessageRespData>(self.config, &api_req, option).await?;
        Ok(UpdateMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        message_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        message_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GetMessageRespData>(self.config, &api_req, option).await?;
        Ok(GetMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        container_id_type: &str,
        container_id: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        sort_type: Option<&str>,
        page_size: Option<i64>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMessageResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/im/v1/messages");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("container_id_type", container_id_type);
        api_req.query_params.set("container_id", container_id);
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = sort_type {
            api_req.query_params.set("sort_type", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListMessageRespData>(self.config, &api_req, option).await?;
        Ok(ListMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        container_id_type: &str,
        container_id: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        sort_type: Option<&str>,
        page_size: Option<i64>,
    ) -> ListMessageIterator<'a> {
        ListMessageIterator {
            config: self.config,
            state: PageIteratorState::default(),
            container_id_type: container_id_type.to_owned(),
            container_id: container_id.to_owned(),
            start_time: start_time.map(|v| v.to_owned()),
            end_time: end_time.map(|v| v.to_owned()),
            sort_type: sort_type.map(|v| v.to_owned()),
            page_size,
        }
    }

    pub async fn forward(
        &self,
        message_id: &str,
        receive_id_type: &str,
        uuid: Option<&str>,
        body: &ForwardMessageReqBody,
        option: &RequestOption,
    ) -> Result<ForwardMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/forward");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("receive_id_type", receive_id_type);
        if let Some(v) = uuid {
            api_req.query_params.set("uuid", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MessageRespData>(self.config, &api_req, option).await?;
        Ok(ForwardMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn merge_forward(
        &self,
        receive_id_type: &str,
        uuid: Option<&str>,
        body: &MergeForwardMessageReqBody,
        option: &RequestOption,
    ) -> Result<MergeForwardMessageResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/im/v1/messages/merge_forward",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("receive_id_type", receive_id_type);
        if let Some(v) = uuid {
            api_req.query_params.set("uuid", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MergeForwardMessageRespData>(self.config, &api_req, option)
                .await?;
        Ok(MergeForwardMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn read_users(
        &self,
        message_id: &str,
        user_id_type: &str,
        page_size: Option<i64>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ReadUsersMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/read_users");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id_type", user_id_type);
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ReadUsersMessageRespData>(self.config, &api_req, option)
                .await?;
        Ok(ReadUsersMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn push_follow_up(
        &self,
        message_id: &str,
        body: &PushFollowUpReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/push_follow_up");
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

    pub async fn urgent_app(
        &self,
        message_id: &str,
        user_id_type: &str,
        body: &UrgentMessageReqBody,
        option: &RequestOption,
    ) -> Result<UrgentAppMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/urgent_app");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id_type", user_id_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UrgentMessageRespData>(self.config, &api_req, option)
                .await?;
        Ok(UrgentAppMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn urgent_phone(
        &self,
        message_id: &str,
        user_id_type: &str,
        body: &UrgentMessageReqBody,
        option: &RequestOption,
    ) -> Result<UrgentPhoneMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/urgent_phone");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id_type", user_id_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UrgentMessageRespData>(self.config, &api_req, option)
                .await?;
        Ok(UrgentPhoneMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn urgent_sms(
        &self,
        message_id: &str,
        user_id_type: &str,
        body: &UrgentMessageReqBody,
        option: &RequestOption,
    ) -> Result<UrgentSmsMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/urgent_sms");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id_type", user_id_type);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UrgentMessageRespData>(self.config, &api_req, option)
                .await?;
        Ok(UrgentSmsMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct MessageReactionResource<'a> {
    config: &'a Config,
}

impl<'a> MessageReactionResource<'a> {
    pub async fn create(
        &self,
        message_id: &str,
        body: &CreateMessageReactionReqBody,
        option: &RequestOption,
    ) -> Result<CreateMessageReactionResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/reactions");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<CreateMessageReactionRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(CreateMessageReactionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        message_id: &str,
        reaction_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteMessageReactionResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<DeleteMessageReactionRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(DeleteMessageReactionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        message_id: &str,
        reaction_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMessageReactionResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/reactions");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = reaction_type {
            api_req.query_params.set("reaction_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ListMessageReactionRespData>(self.config, &api_req, option)
                .await?;
        Ok(ListMessageReactionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        message_id: &str,
        reaction_type: Option<&str>,
        page_size: Option<i64>,
        user_id_type: Option<&str>,
    ) -> ListMessageReactionIterator<'a> {
        ListMessageReactionIterator {
            config: self.config,
            state: PageIteratorState::default(),
            message_id: message_id.to_owned(),
            reaction_type: reaction_type.map(|v| v.to_owned()),
            page_size,
            user_id_type: user_id_type.map(|v| v.to_owned()),
        }
    }
}

pub struct MessageResourceDownload<'a> {
    config: &'a Config,
}

impl<'a> MessageResourceDownload<'a> {
    pub async fn get(
        &self,
        message_id: &str,
        file_key: &str,
        resource_type: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/im/v1/messages/{message_id}/resources/{file_key}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("type", resource_type);
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }
}

pub struct BatchMessageResource<'a> {
    config: &'a Config,
}

impl<'a> BatchMessageResource<'a> {
    pub async fn delete(
        &self,
        batch_message_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/batch_messages/{batch_message_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get_progress(
        &self,
        batch_message_id: &str,
        option: &RequestOption,
    ) -> Result<GetProgressBatchMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/batch_messages/{batch_message_id}/get_progress");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<GetProgressBatchMessageRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(GetProgressBatchMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn read_user(
        &self,
        batch_message_id: &str,
        option: &RequestOption,
    ) -> Result<ReadUserBatchMessageResp, LarkError> {
        let path = format!("/open-apis/im/v1/batch_messages/{batch_message_id}/read_user");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<ReadUserBatchMessageRespData>(self.config, &api_req, option)
                .await?;
        Ok(ReadUserBatchMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct PinResource<'a> {
    config: &'a Config,
}

impl<'a> PinResource<'a> {
    pub async fn create(
        &self,
        body: &CreatePinReqBody,
        option: &RequestOption,
    ) -> Result<CreatePinResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v1/pins");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreatePinRespData>(self.config, &api_req, option).await?;
        Ok(CreatePinResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        message_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/pins/{message_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
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
        chat_id: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i64>,
        option: &RequestOption,
    ) -> Result<ListPinResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/im/v1/pins");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("chat_id", chat_id);
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<ListPinRespData>(self.config, &api_req, option).await?;
        Ok(ListPinResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        chat_id: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        page_size: Option<i64>,
    ) -> ListPinIterator<'a> {
        ListPinIterator {
            config: self.config,
            state: PageIteratorState::default(),
            chat_id: chat_id.to_owned(),
            start_time: start_time.map(|v| v.to_owned()),
            end_time: end_time.map(|v| v.to_owned()),
            page_size,
        }
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

impl<'a> FileResource<'a> {
    pub async fn create(
        &self,
        file_type: &str,
        file_name: &str,
        duration: Option<i64>,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<CreateFileResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v1/files");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let mut fields = vec![
            FormDataField {
                name: "file_type".into(),
                value: FormDataValue::Text(file_type.to_string()),
            },
            FormDataField {
                name: "file_name".into(),
                value: FormDataValue::Text(file_name.to_string()),
            },
            FormDataField {
                name: "file".into(),
                value: FormDataValue::File {
                    filename: file_name.to_string(),
                    data,
                    content_type: None,
                },
            },
        ];
        if let Some(d) = duration {
            fields.push(FormDataField {
                name: "duration".into(),
                value: FormDataValue::Text(d.to_string()),
            });
        }
        api_req.body = Some(ReqBody::FormData(fields));
        let (api_resp, raw) =
            transport::request_typed::<CreateFileRespData>(self.config, &api_req, option).await?;
        Ok(CreateFileResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        file_key: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/im/v1/files/{file_key}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }
}

pub struct ImageResource<'a> {
    config: &'a Config,
}

impl<'a> ImageResource<'a> {
    pub async fn create(
        &self,
        image_type: &str,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<CreateImageResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v1/images");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let fields = vec![
            FormDataField {
                name: "image_type".into(),
                value: FormDataValue::Text(image_type.to_string()),
            },
            FormDataField {
                name: "image".into(),
                value: FormDataValue::File {
                    filename: "image".to_string(),
                    data,
                    content_type: None,
                },
            },
        ];
        api_req.body = Some(ReqBody::FormData(fields));
        let (api_resp, raw) =
            transport::request_typed::<CreateImageRespData>(self.config, &api_req, option).await?;
        Ok(CreateImageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        image_key: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/im/v1/images/{image_key}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }
}

pub struct ChatResource<'a> {
    config: &'a Config,
}

impl<'a> ChatResource<'a> {
    pub async fn create(
        &self,
        user_id_type: Option<&str>,
        body: &CreateChatReqBody,
        option: &RequestOption,
    ) -> Result<CreateChatResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v1/chats");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateChatRespData>(self.config, &api_req, option).await?;
        Ok(CreateChatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        chat_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        chat_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetChatResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GetChatRespData>(self.config, &api_req, option).await?;
        Ok(GetChatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        chat_id: &str,
        user_id_type: Option<&str>,
        body: &UpdateChatReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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

    pub async fn link(
        &self,
        chat_id: &str,
        body: &LinkChatReqBody,
        option: &RequestOption,
    ) -> Result<LinkChatResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/link");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<LinkChatRespData>(self.config, &api_req, option).await?;
        Ok(LinkChatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        sort_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i64>,
        option: &RequestOption,
    ) -> Result<ListChatResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/im/v1/chats");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = sort_type {
            api_req.query_params.set("sort_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<ListChatRespData>(self.config, &api_req, option).await?;
        Ok(ListChatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn search(
        &self,
        user_id_type: Option<&str>,
        query: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i64>,
        option: &RequestOption,
    ) -> Result<ListChatResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/im/v1/chats/search");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = query {
            api_req.query_params.set("query", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<ListChatRespData>(self.config, &api_req, option).await?;
        Ok(ListChatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn list_by_iterator(
        &self,
        user_id_type: Option<&str>,
        sort_type: Option<&str>,
        page_size: Option<i64>,
    ) -> ListChatIterator<'a> {
        ListChatIterator {
            config: self.config,
            state: PageIteratorState::default(),
            user_id_type: user_id_type.map(|v| v.to_owned()),
            sort_type: sort_type.map(|v| v.to_owned()),
            page_size,
        }
    }

    pub fn search_by_iterator(
        &self,
        user_id_type: Option<&str>,
        query: Option<&str>,
        page_size: Option<i64>,
    ) -> SearchChatIterator<'a> {
        SearchChatIterator {
            config: self.config,
            state: PageIteratorState::default(),
            user_id_type: user_id_type.map(|v| v.to_owned()),
            query: query.map(|v| v.to_owned()),
            page_size,
        }
    }
}

pub struct ChatMembersResource<'a> {
    config: &'a Config,
}

impl<'a> ChatMembersResource<'a> {
    pub async fn create(
        &self,
        chat_id: &str,
        member_id_type: Option<&str>,
        succeed_type: Option<&str>,
        body: &CreateChatMembersReqBody,
        option: &RequestOption,
    ) -> Result<CreateChatMembersResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/members");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = member_id_type {
            api_req.query_params.set("member_id_type", v);
        }
        if let Some(v) = succeed_type {
            api_req.query_params.set("succeed_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateChatMembersRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateChatMembersResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        chat_id: &str,
        member_id_type: Option<&str>,
        body: &DeleteChatMembersReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/members");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = member_id_type {
            api_req.query_params.set("member_id_type", v);
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
        chat_id: &str,
        member_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i64>,
        option: &RequestOption,
    ) -> Result<GetChatMembersResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/members");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = member_id_type {
            api_req.query_params.set("member_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<GetChatMembersRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetChatMembersResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn get_by_iterator(
        &self,
        chat_id: &str,
        member_id_type: Option<&str>,
        page_size: Option<i64>,
    ) -> GetChatMembersIterator<'a> {
        GetChatMembersIterator {
            config: self.config,
            state: PageIteratorState::default(),
            chat_id: chat_id.to_owned(),
            member_id_type: member_id_type.map(|v| v.to_owned()),
            page_size,
        }
    }

    pub async fn is_in_chat(
        &self,
        chat_id: &str,
        option: &RequestOption,
    ) -> Result<IsInChatChatMembersResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/members/is_in_chat");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<IsInChatChatMembersRespData>(self.config, &api_req, option)
                .await?;
        Ok(IsInChatChatMembersResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn me_join(
        &self,
        chat_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/members/me_join");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct ChatManagersResource<'a> {
    config: &'a Config,
}

impl<'a> ChatManagersResource<'a> {
    pub async fn add_managers(
        &self,
        chat_id: &str,
        member_id_type: Option<&str>,
        body: &UpdateChatManagersReqBody,
        option: &RequestOption,
    ) -> Result<AddManagersChatManagersResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/managers/add_managers");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = member_id_type {
            api_req.query_params.set("member_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UpdateChatManagersRespData>(self.config, &api_req, option)
                .await?;
        Ok(AddManagersChatManagersResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete_managers(
        &self,
        chat_id: &str,
        member_id_type: Option<&str>,
        body: &UpdateChatManagersReqBody,
        option: &RequestOption,
    ) -> Result<DeleteManagersChatManagersResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/managers/delete_managers");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = member_id_type {
            api_req.query_params.set("member_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<UpdateChatManagersRespData>(self.config, &api_req, option)
                .await?;
        Ok(DeleteManagersChatManagersResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ChatAnnouncementResource<'a> {
    config: &'a Config,
}

impl<'a> ChatAnnouncementResource<'a> {
    pub async fn get(
        &self,
        chat_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetChatAnnouncementResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/announcement");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GetChatAnnouncementRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetChatAnnouncementResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        chat_id: &str,
        body: &PatchChatAnnouncementReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/announcement");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct ChatModerationResource<'a> {
    config: &'a Config,
}

impl<'a> ChatModerationResource<'a> {
    pub async fn get(
        &self,
        chat_id: &str,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i64>,
        option: &RequestOption,
    ) -> Result<GetChatModerationResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/moderation");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
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
            transport::request_typed::<GetChatModerationRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetChatModerationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub fn get_by_iterator(
        &self,
        chat_id: &str,
        user_id_type: Option<&str>,
        page_size: Option<i64>,
    ) -> GetChatModerationIterator<'a> {
        GetChatModerationIterator {
            config: self.config,
            state: PageIteratorState::default(),
            chat_id: chat_id.to_owned(),
            user_id_type: user_id_type.map(|v| v.to_owned()),
            page_size,
        }
    }

    pub async fn update(
        &self,
        chat_id: &str,
        user_id_type: Option<&str>,
        body: &UpdateChatModerationReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/moderation");
        let mut api_req = ApiReq::new(http::Method::PUT, path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
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

pub struct ChatTabResource<'a> {
    config: &'a Config,
}

impl<'a> ChatTabResource<'a> {
    pub async fn create(
        &self,
        chat_id: &str,
        body: &ChatTabsReqBody,
        option: &RequestOption,
    ) -> Result<CreateChatTabResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/chat_tabs");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChatTabsRespData>(self.config, &api_req, option).await?;
        Ok(CreateChatTabResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete_tabs(
        &self,
        chat_id: &str,
        body: &DeleteChatTabsReqBody,
        option: &RequestOption,
    ) -> Result<DeleteTabsChatTabResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/chat_tabs/delete_tabs");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChatTabsRespData>(self.config, &api_req, option).await?;
        Ok(DeleteTabsChatTabResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list_tabs(
        &self,
        chat_id: &str,
        option: &RequestOption,
    ) -> Result<ListTabsChatTabResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/chat_tabs/list_tabs");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<ChatTabsRespData>(self.config, &api_req, option).await?;
        Ok(ListTabsChatTabResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn sort_tabs(
        &self,
        chat_id: &str,
        body: &SortChatTabsReqBody,
        option: &RequestOption,
    ) -> Result<SortTabsChatTabResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/chat_tabs/sort_tabs");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChatTabsRespData>(self.config, &api_req, option).await?;
        Ok(SortTabsChatTabResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update_tabs(
        &self,
        chat_id: &str,
        body: &ChatTabsReqBody,
        option: &RequestOption,
    ) -> Result<UpdateTabsChatTabResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/chat_tabs/update_tabs");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChatTabsRespData>(self.config, &api_req, option).await?;
        Ok(UpdateTabsChatTabResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ChatTopNoticeResource<'a> {
    config: &'a Config,
}

impl<'a> ChatTopNoticeResource<'a> {
    pub async fn put_top_notice(
        &self,
        chat_id: &str,
        body: &PutChatTopNoticeReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/top_notice/put_top_notice");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete_top_notice(
        &self,
        chat_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/top_notice/delete_top_notice");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct ChatMenuItemResource<'a> {
    config: &'a Config,
}

impl<'a> ChatMenuItemResource<'a> {
    pub async fn patch(
        &self,
        chat_id: &str,
        menu_item_id: &str,
        body: &PatchChatMenuItemReqBody,
        option: &RequestOption,
    ) -> Result<PatchChatMenuItemResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/menu_items/{menu_item_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<PatchChatMenuItemRespData>(self.config, &api_req, option)
                .await?;
        Ok(PatchChatMenuItemResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ChatMenuTreeResource<'a> {
    config: &'a Config,
}

impl<'a> ChatMenuTreeResource<'a> {
    pub async fn create(
        &self,
        chat_id: &str,
        body: &CreateChatMenuTreeReqBody,
        option: &RequestOption,
    ) -> Result<CreateChatMenuTreeResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/menu_tree");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChatMenuTreeRespData>(self.config, &api_req, option).await?;
        Ok(CreateChatMenuTreeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        chat_id: &str,
        body: &DeleteChatMenuTreeReqBody,
        option: &RequestOption,
    ) -> Result<DeleteChatMenuTreeResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/menu_tree");
        let mut api_req = ApiReq::new(http::Method::DELETE, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChatMenuTreeRespData>(self.config, &api_req, option).await?;
        Ok(DeleteChatMenuTreeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        chat_id: &str,
        option: &RequestOption,
    ) -> Result<GetChatMenuTreeResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/menu_tree");
        let mut api_req = ApiReq::new(http::Method::GET, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<ChatMenuTreeRespData>(self.config, &api_req, option).await?;
        Ok(GetChatMenuTreeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn sort(
        &self,
        chat_id: &str,
        body: &SortChatMenuTreeReqBody,
        option: &RequestOption,
    ) -> Result<SortChatMenuTreeResp, LarkError> {
        let path = format!("/open-apis/im/v1/chats/{chat_id}/menu_tree/sort");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChatMenuTreeRespData>(self.config, &api_req, option).await?;
        Ok(SortChatMenuTreeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ThreadResource<'a> {
    config: &'a Config,
}

impl<'a> ThreadResource<'a> {
    pub async fn forward(
        &self,
        thread_id: &str,
        receive_id_type: &str,
        uuid: Option<&str>,
        body: &ForwardThreadReqBody,
        option: &RequestOption,
    ) -> Result<ForwardThreadResp, LarkError> {
        let path = format!("/open-apis/im/v1/threads/{thread_id}/forward");
        let mut api_req = ApiReq::new(http::Method::POST, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("receive_id_type", receive_id_type);
        if let Some(v) = uuid {
            api_req.query_params.set("uuid", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<MessageRespData>(self.config, &api_req, option).await?;
        Ok(ForwardThreadResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub message: MessageResource<'a>,
    pub message_reaction: MessageReactionResource<'a>,
    pub message_resource: MessageResourceDownload<'a>,
    pub batch_message: BatchMessageResource<'a>,
    pub pin: PinResource<'a>,
    pub file: FileResource<'a>,
    pub image: ImageResource<'a>,
    pub chat: ChatResource<'a>,
    pub chat_members: ChatMembersResource<'a>,
    pub chat_managers: ChatManagersResource<'a>,
    pub chat_announcement: ChatAnnouncementResource<'a>,
    pub chat_moderation: ChatModerationResource<'a>,
    pub chat_tab: ChatTabResource<'a>,
    pub chat_top_notice: ChatTopNoticeResource<'a>,
    pub chat_menu_item: ChatMenuItemResource<'a>,
    pub chat_menu_tree: ChatMenuTreeResource<'a>,
    pub thread: ThreadResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            message: MessageResource { config },
            message_reaction: MessageReactionResource { config },
            message_resource: MessageResourceDownload { config },
            batch_message: BatchMessageResource { config },
            pin: PinResource { config },
            file: FileResource { config },
            image: ImageResource { config },
            chat: ChatResource { config },
            chat_members: ChatMembersResource { config },
            chat_managers: ChatManagersResource { config },
            chat_announcement: ChatAnnouncementResource { config },
            chat_moderation: ChatModerationResource { config },
            chat_tab: ChatTabResource { config },
            chat_top_notice: ChatTopNoticeResource { config },
            chat_menu_item: ChatMenuItemResource { config },
            chat_menu_tree: ChatMenuTreeResource { config },
            thread: ThreadResource { config },
        }
    }
}
