use std::collections::HashSet;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DmMode {
    #[default]
    Open,
    Disabled,
    Allowlist,
}

#[derive(Debug, Clone)]
pub struct ChannelPolicy {
    pub allow_bot_senders: bool,
    pub allowed_message_types: Option<HashSet<String>>,
    pub max_event_age: Option<Duration>,
    pub reject_duplicates: bool,
    pub dedup_capacity: usize,
    pub group_allowlist: Option<HashSet<String>>,
    pub require_mention: bool,
    pub respond_to_mention_all: bool,
    pub dm_mode: DmMode,
    pub dm_allowlist: Option<HashSet<String>>,
    pub sender_allowlist: Option<HashSet<String>>,
}

impl Default for ChannelPolicy {
    fn default() -> Self {
        Self {
            allow_bot_senders: false,
            allowed_message_types: None,
            max_event_age: Some(Duration::from_secs(30 * 60)),
            reject_duplicates: true,
            dedup_capacity: 4096,
            group_allowlist: None,
            require_mention: true,
            respond_to_mention_all: false,
            dm_mode: DmMode::Open,
            dm_allowlist: None,
            sender_allowlist: None,
        }
    }
}

impl ChannelPolicy {
    pub fn allow_bot_senders(mut self, allow: bool) -> Self {
        self.allow_bot_senders = allow;
        self
    }

    pub fn allow_message_type(mut self, message_type: impl Into<String>) -> Self {
        self.allowed_message_types
            .get_or_insert_with(HashSet::new)
            .insert(message_type.into());
        self
    }

    pub fn allow_all_message_types(mut self) -> Self {
        self.allowed_message_types = None;
        self
    }

    pub fn max_event_age(mut self, age: Option<Duration>) -> Self {
        self.max_event_age = age;
        self
    }

    pub fn reject_duplicates(mut self, reject: bool) -> Self {
        self.reject_duplicates = reject;
        self
    }

    pub fn dedup_capacity(mut self, capacity: usize) -> Self {
        self.dedup_capacity = capacity.max(1);
        self
    }

    pub fn allow_group(mut self, chat_id: impl Into<String>) -> Self {
        self.group_allowlist
            .get_or_insert_with(HashSet::new)
            .insert(chat_id.into());
        self
    }

    pub fn require_mention(mut self, require: bool) -> Self {
        self.require_mention = require;
        self
    }

    pub fn respond_to_mention_all(mut self, respond: bool) -> Self {
        self.respond_to_mention_all = respond;
        self
    }

    pub fn dm_mode(mut self, mode: DmMode) -> Self {
        self.dm_mode = mode;
        self
    }

    pub fn allow_dm_sender(mut self, sender_id: impl Into<String>) -> Self {
        self.dm_allowlist
            .get_or_insert_with(HashSet::new)
            .insert(sender_id.into());
        self
    }

    pub fn allow_sender(mut self, sender_id: impl Into<String>) -> Self {
        self.sender_allowlist
            .get_or_insert_with(HashSet::new)
            .insert(sender_id.into());
        self
    }
}
