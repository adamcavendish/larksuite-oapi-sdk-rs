//! Mail v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subscriber {
    #[serde(default)]
    pub user_ids: Vec<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2UserMailboxEventMessageReceivedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailbox_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber: Option<Subscriber>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_mail_user_mailbox_event_message_received_v1 => P2UserMailboxEventMessageReceivedV1
        : "mail.user_mailbox.event.message_received_v1",
}
