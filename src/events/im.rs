//! Instant Messaging (IM) v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReceiveV1 {
    #[serde(default)]
    pub sender: MessageSender,
    #[serde(default)]
    pub message: Message,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSender {
    #[serde(default)]
    pub sender_id: Option<serde_json::Value>,
    #[serde(default)]
    pub sender_type: String,
    #[serde(default)]
    pub tenant_key: String,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReadV1 {
    #[serde(default)]
    pub reader: serde_json::Value,
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
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub operator_type: String,
    #[serde(default)]
    pub recall_message_type: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberUserAddedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub operator_type: String,
    #[serde(default)]
    pub users: Vec<serde_json::Value>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberUserDeletedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub operator_type: String,
    #[serde(default)]
    pub users: Vec<serde_json::Value>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatDisbandedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatUpdatedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
    #[serde(default)]
    pub after_change: serde_json::Value,
    #[serde(default)]
    pub before_change: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReactionCreatedV1 {
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub action_time: String,
    #[serde(default)]
    pub reaction_type: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MessageReactionDeletedV1 {
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub action_time: String,
    #[serde(default)]
    pub reaction_type: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberBotAddedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ChatMemberBotDeletedV1 {
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub operator_tenant_key: String,
}

// ── Handler registration helpers ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
+ Send
+ Sync
+ 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => Box::pin(handler(typed))
                as Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>,
            Err(e) => Box::pin(async move {
                Err(LarkError::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods ──

impl EventDispatcher {
    pub fn on_p2_im_message_receive_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MessageReceiveV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.message.receive_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_message_read_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MessageReadV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.message.message_read_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_message_recalled_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MessageRecalledV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.message.recalled_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_chat_member_user_added_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ChatMemberUserAddedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.member.user.added_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_chat_member_user_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ChatMemberUserDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.member.user.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_chat_disbanded_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ChatDisbandedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.disbanded_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_chat_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ChatUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_message_reaction_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MessageReactionCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.message.reaction.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_message_reaction_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MessageReactionDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.message.reaction.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_chat_member_bot_added_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ChatMemberBotAddedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.member.bot.added_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_chat_member_bot_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ChatMemberBotDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.member.bot.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_im_chat_access_event_bot_p2p_chat_entered_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "im.chat.access_event.bot_p2p_chat_entered_v1",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_im_chat_member_user_withdrawn_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.member.user.withdrawn_v1", wrap_handler(handler))
    }
}
