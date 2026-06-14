//! Instant Messaging (IM) v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReceiveV1 {
    #[serde(default)]
    pub sender: MessageSender,
    #[serde(default)]
    pub message: Message,
}

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSender {
    #[serde(default)]
    pub sender_id: Option<serde_json::Value>,
    #[serde(default)]
    pub sender_type: String,
    #[serde(default)]
    pub tenant_key: String,
}

impl MessageSender {
    pub fn sender_user_id(&self) -> Option<UserId> {
        self.sender_id
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn open_id(&self) -> Option<&str> {
        self.sender_id.as_ref()?.get("open_id")?.as_str()
    }

    pub fn user_id(&self) -> Option<&str> {
        self.sender_id.as_ref()?.get("user_id")?.as_str()
    }

    pub fn union_id(&self) -> Option<&str> {
        self.sender_id.as_ref()?.get("union_id")?.as_str()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mention {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub id: Option<UserId>,
    #[serde(default)]
    pub mentioned_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tenant_key: String,
}

impl Mention {
    pub fn open_id(&self) -> Option<&str> {
        self.id.as_ref().and_then(UserId::open_id)
    }

    pub fn user_id(&self) -> Option<&str> {
        self.id.as_ref().and_then(UserId::user_id)
    }

    pub fn union_id(&self) -> Option<&str> {
        self.id.as_ref().and_then(UserId::union_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventMessageReader {
    #[serde(default)]
    pub reader_id: Option<UserId>,
    #[serde(default)]
    pub read_time: String,
    #[serde(default)]
    pub tenant_key: String,
}

impl EventMessageReader {
    pub fn open_id(&self) -> Option<&str> {
        self.reader_id.as_ref().and_then(UserId::open_id)
    }

    pub fn user_id(&self) -> Option<&str> {
        self.reader_id.as_ref().and_then(UserId::user_id)
    }

    pub fn union_id(&self) -> Option<&str> {
        self.reader_id.as_ref().and_then(UserId::union_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Emoji {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<String>,
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
pub struct ChatMemberUser {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub user_id: Option<UserId>,
}

impl ChatMemberUser {
    pub fn open_id(&self) -> Option<&str> {
        self.user_id.as_ref().and_then(UserId::open_id)
    }

    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_ref().and_then(UserId::user_id)
    }

    pub fn union_id(&self) -> Option<&str> {
        self.user_id.as_ref().and_then(UserId::union_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEventModerator {
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub user_id: Option<UserId>,
}

impl ListEventModerator {
    pub fn open_id(&self) -> Option<&str> {
        self.user_id.as_ref().and_then(UserId::open_id)
    }

    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_ref().and_then(UserId::user_id)
    }

    pub fn union_id(&self) -> Option<&str> {
        self.user_id.as_ref().and_then(UserId::union_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModeratorList {
    #[serde(default)]
    pub added_member_list: Vec<ListEventModerator>,
    #[serde(default)]
    pub removed_member_list: Vec<ListEventModerator>,
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
pub struct ChatChange {
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
    #[serde(default)]
    pub add_member_permission: String,
    #[serde(default)]
    pub share_card_permission: String,
    #[serde(default)]
    pub at_all_permission: String,
    #[serde(default)]
    pub edit_permission: String,
    #[serde(default)]
    pub membership_approval: String,
    #[serde(default)]
    pub join_message_visibility: String,
    #[serde(default)]
    pub leave_message_visibility: String,
    #[serde(default)]
    pub moderation_permission: String,
    #[serde(default)]
    pub owner_id: Option<UserId>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub restricted_mode_setting: Option<RestrictedModeSetting>,
    #[serde(default)]
    pub group_message_type: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub root_id: String,
    #[serde(default)]
    pub parent_id: String,
    #[serde(default)]
    pub thread_id: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub chat_type: String,
    #[serde(default)]
    pub message_type: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub mentions: Vec<serde_json::Value>,
}

impl Message {
    pub fn typed_mentions(&self) -> Result<Vec<Mention>, serde_json::Error> {
        self.mentions
            .iter()
            .cloned()
            .map(serde_json::from_value)
            .collect()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReadV1 {
    #[serde(default)]
    pub reader: EventMessageReader,
    #[serde(default)]
    pub message_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageRecalledV1 {
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub recall_time: String,
    #[serde(default)]
    pub recall_type: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberUserAddedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub users: Vec<ChatMemberUser>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberUserDeletedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub users: Vec<ChatMemberUser>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberUserWithdrawnV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub users: Vec<ChatMemberUser>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatDisbandedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatUpdatedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub after_change: Option<ChatChange>,
    #[serde(default)]
    pub before_change: Option<ChatChange>,
    #[serde(default)]
    pub moderator_list: Option<ModeratorList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReactionCreatedV1 {
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub reaction_type: Emoji,
    #[serde(default)]
    pub operator_type: String,
    #[serde(default)]
    pub user_id: Option<UserId>,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub action_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReactionDeletedV1 {
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub reaction_type: Emoji,
    #[serde(default)]
    pub operator_type: String,
    #[serde(default)]
    pub user_id: Option<UserId>,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub action_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatAccessEventBotP2pChatEnteredV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub last_message_id: String,
    #[serde(default)]
    pub last_message_create_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberBotAddedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberBotDeletedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: Option<UserId>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_im_message_receive_v1 => P2MessageReceiveV1
        : "im.message.receive_v1",
    on_p2_im_message_read_v1 => P2MessageReadV1
        : "im.message.message_read_v1",
    on_p2_im_message_recalled_v1 => P2MessageRecalledV1
        : "im.message.recalled_v1",
    on_p2_im_chat_member_user_added_v1 => P2ChatMemberUserAddedV1
        : "im.chat.member.user.added_v1",
    on_p2_im_chat_member_user_deleted_v1 => P2ChatMemberUserDeletedV1
        : "im.chat.member.user.deleted_v1",
    on_p2_im_chat_disbanded_v1 => P2ChatDisbandedV1
        : "im.chat.disbanded_v1",
    on_p2_im_chat_updated_v1 => P2ChatUpdatedV1
        : "im.chat.updated_v1",
    on_p2_im_message_reaction_created_v1 => P2MessageReactionCreatedV1
        : "im.message.reaction.created_v1",
    on_p2_im_message_reaction_deleted_v1 => P2MessageReactionDeletedV1
        : "im.message.reaction.deleted_v1",
    on_p2_im_chat_member_bot_added_v1 => P2ChatMemberBotAddedV1
        : "im.chat.member.bot.added_v1",
    on_p2_im_chat_member_bot_deleted_v1 => P2ChatMemberBotDeletedV1
        : "im.chat.member.bot.deleted_v1",
    on_p2_im_chat_access_event_bot_p2p_chat_entered_v1 => P2ChatAccessEventBotP2pChatEnteredV1
        : "im.chat.access_event.bot_p2p_chat_entered_v1",
    on_p2_im_chat_member_user_withdrawn_v1 => P2ChatMemberUserWithdrawnV1
        : "im.chat.member.user.withdrawn_v1",
}
