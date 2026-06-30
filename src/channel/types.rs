use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::event::CardActionTriggerRequest;
use crate::events::common::UserId;
use crate::events::im::P2MessageReceiveV1;

use super::identity::BotIdentity;
use super::stream::extract_text_content;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSender {
    pub sender_type: String,
    pub tenant_key: String,
    pub user_id: Option<UserId>,
}

impl ChannelSender {
    pub(super) fn matches_any(&self, ids: &HashSet<String>) -> bool {
        self.user_id.as_ref().is_some_and(|user| {
            user.open_id().is_some_and(|id| ids.contains(id))
                || user.user_id().is_some_and(|id| ids.contains(id))
                || user.union_id().is_some_and(|id| ids.contains(id))
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMention {
    pub key: String,
    pub id: Option<UserId>,
    pub mentioned_type: String,
    pub name: String,
    pub tenant_key: String,
    pub is_bot: bool,
}

impl ChannelMention {
    pub(super) fn is_mention_all(&self) -> bool {
        let mentioned_type = self.mentioned_type.to_ascii_lowercase();
        let key = self.key.to_ascii_lowercase();
        matches!(
            mentioned_type.as_str(),
            "all" | "mention_all" | "at_all" | "user_all"
        ) || matches!(key.as_str(), "all" | "@all")
    }

    pub(super) fn matches_identity(&self, identity: &BotIdentity) -> bool {
        let Some(id) = &self.id else {
            return false;
        };
        identity
            .open_id
            .as_deref()
            .is_some_and(|bot| id.open_id() == Some(bot))
            || identity
                .user_id
                .as_deref()
                .is_some_and(|bot| id.user_id() == Some(bot))
            || identity
                .union_id
                .as_deref()
                .is_some_and(|bot| id.union_id() == Some(bot))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedMessage {
    pub message_id: String,
    pub root_id: String,
    pub parent_id: String,
    pub thread_id: String,
    pub chat_id: String,
    pub chat_type: String,
    pub message_type: String,
    pub content: String,
    pub text: Option<String>,
    pub create_time: String,
    pub update_time: String,
    pub sender: ChannelSender,
    pub mentions: Vec<ChannelMention>,
    pub mention_all: bool,
    pub mentioned_bot: bool,
}

impl NormalizedMessage {
    pub fn from_event(event: P2MessageReceiveV1) -> Self {
        let text = extract_text_content(&event.message.message_type, &event.message.content);
        let mentions: Vec<_> = event
            .message
            .mentions
            .into_iter()
            .map(|mention| ChannelMention {
                key: mention.key,
                id: mention.id,
                mentioned_type: mention.mentioned_type,
                name: mention.name,
                tenant_key: mention.tenant_key,
                is_bot: false,
            })
            .collect();
        let mention_all = mentions.iter().any(ChannelMention::is_mention_all);
        Self {
            message_id: event.message.message_id,
            root_id: event.message.root_id,
            parent_id: event.message.parent_id,
            thread_id: event.message.thread_id,
            chat_id: event.message.chat_id,
            chat_type: event.message.chat_type,
            message_type: event.message.message_type,
            content: event.message.content,
            text,
            create_time: event.message.create_time,
            update_time: event.message.update_time,
            sender: ChannelSender {
                sender_type: event.sender.sender_type,
                tenant_key: event.sender.tenant_key,
                user_id: event.sender.sender_id,
            },
            mentions,
            mention_all,
            mentioned_bot: false,
        }
    }

    pub(super) fn apply_bot_identity(&mut self, identity: Option<&BotIdentity>) {
        let Some(identity) = identity else {
            return;
        };
        for mention in &mut self.mentions {
            if mention.matches_identity(identity) {
                mention.is_bot = true;
                self.mentioned_bot = true;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReactionAction {
    Created,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedReaction {
    pub action: ReactionAction,
    pub message_id: String,
    pub emoji_type: Option<String>,
    pub operator_type: String,
    pub user_id: Option<UserId>,
    pub app_id: String,
    pub action_time: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BotMembershipAction {
    Added,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotMembership {
    pub action: BotMembershipAction,
    pub chat_id: String,
    pub operator_id: Option<UserId>,
    pub external: bool,
    pub operator_tenant_key: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedCardAction {
    pub operator_open_id: Option<String>,
    pub operator_user_id: Option<String>,
    pub tenant_key: Option<String>,
    pub open_message_id: Option<String>,
    pub open_chat_id: Option<String>,
    pub action_tag: Option<String>,
    pub action_name: Option<String>,
    pub action_value: serde_json::Map<String, serde_json::Value>,
}

impl NormalizedCardAction {
    pub fn from_request(req: CardActionTriggerRequest) -> Self {
        let (action_tag, action_name, action_value) = req
            .action
            .map(|action| (Some(action.tag), Some(action.name), action.value))
            .unwrap_or_else(|| (None, None, serde_json::Map::new()));
        let (open_message_id, open_chat_id) = req
            .context
            .map(|context| (Some(context.open_message_id), Some(context.open_chat_id)))
            .unwrap_or((None, None));
        let (operator_open_id, operator_user_id, tenant_key) = req
            .operator
            .map(|operator| {
                (
                    Some(operator.open_id),
                    operator.user_id,
                    operator.tenant_key,
                )
            })
            .unwrap_or((None, None, None));

        Self {
            operator_open_id,
            operator_user_id,
            tenant_key,
            open_message_id,
            open_chat_id,
            action_tag,
            action_name,
            action_value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectReason {
    Duplicate,
    Stale,
    BotSender,
    MessageType,
    GroupNotAllowed,
    NoMention,
    MentionAll,
    DmDisabled,
    SenderNotAllowed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectEvent {
    pub reason: RejectReason,
    pub event_key: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelEvent {
    Message(Box<NormalizedMessage>),
    Reaction(Box<NormalizedReaction>),
    BotMembership(Box<BotMembership>),
    CardAction(Box<NormalizedCardAction>),
    Rejected(RejectEvent),
}

#[derive(Debug, Clone)]
pub enum ChannelDecision<T> {
    Accepted(T),
    Rejected(RejectEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendTarget {
    pub receive_id_type: String,
    pub receive_id: String,
}

impl SendTarget {
    pub fn new(receive_id_type: impl Into<String>, receive_id: impl Into<String>) -> Self {
        Self {
            receive_id_type: receive_id_type.into(),
            receive_id: receive_id.into(),
        }
    }
}
