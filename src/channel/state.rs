use std::collections::{HashSet, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::events::im::P2MessageReceiveV1;
use crate::{Client, LarkError, RequestOption};

use super::identity::{BotIdentity, BotIdentityCache, BotIdentityCacheConfig, fetch_bot_identity};
use super::policy::{ChannelPolicy, DmMode};
use super::types::{ChannelDecision, ChannelSender, NormalizedMessage, RejectEvent, RejectReason};
use super::util::lock;

#[derive(Debug)]
pub(super) struct ChannelState {
    policy: Mutex<ChannelPolicy>,
    dedup: Mutex<DedupCache>,
    bot_cache: Mutex<BotIdentityCache>,
    bot_refresh: tokio::sync::Mutex<()>,
}

impl ChannelState {
    pub(super) fn new(policy: ChannelPolicy, bot_cache_config: BotIdentityCacheConfig) -> Self {
        Self {
            dedup: Mutex::new(DedupCache::new(policy.dedup_capacity)),
            policy: Mutex::new(policy),
            bot_cache: Mutex::new(BotIdentityCache::new(bot_cache_config)),
            bot_refresh: tokio::sync::Mutex::new(()),
        }
    }

    pub(super) fn policy(&self) -> ChannelPolicy {
        lock(&self.policy).clone()
    }

    pub(super) fn update_policy(&self, policy: ChannelPolicy) {
        let mut current = lock(&self.policy);
        let capacity = policy.dedup_capacity;
        *current = policy;
        lock(&self.dedup).resize(capacity);
    }

    pub(super) fn accept_message(
        &self,
        event: P2MessageReceiveV1,
    ) -> ChannelDecision<NormalizedMessage> {
        let mut message = NormalizedMessage::from_event(event);
        let identity = lock(&self.bot_cache).identity.clone();
        message.apply_bot_identity(identity.as_ref());

        let policy = self.policy();
        let event_key = message.message_id.clone();
        if policy.reject_duplicates
            && !event_key.is_empty()
            && !lock(&self.dedup).insert(event_key.clone())
        {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::Duplicate,
                event_key,
                detail: "message already handled".into(),
            });
        }

        if let Some(max_age) = policy.max_event_age
            && is_stale(&message.create_time, max_age)
        {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::Stale,
                event_key,
                detail: "message create_time is older than policy max_event_age".into(),
            });
        }

        if !policy.allow_bot_senders && self.is_bot_sender(&message.sender) {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::BotSender,
                event_key,
                detail: "bot sender rejected by channel policy".into(),
            });
        }

        if let Some(allowed) = &policy.allowed_message_types
            && !allowed.contains(&message.message_type)
        {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::MessageType,
                event_key,
                detail: format!(
                    "message type {} rejected by channel policy",
                    message.message_type
                ),
            });
        }

        if let Some(allowed) = &policy.sender_allowlist
            && !message.sender.matches_any(allowed)
        {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::SenderNotAllowed,
                event_key,
                detail: "sender rejected by channel policy".into(),
            });
        }

        if message.chat_type == "group" {
            self.evaluate_group_policy(message, event_key, &policy)
        } else {
            self.evaluate_dm_policy(message, event_key, &policy)
        }
    }

    fn evaluate_group_policy(
        &self,
        message: NormalizedMessage,
        event_key: String,
        policy: &ChannelPolicy,
    ) -> ChannelDecision<NormalizedMessage> {
        if let Some(allowed) = &policy.group_allowlist
            && !allowed.contains(&message.chat_id)
        {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::GroupNotAllowed,
                event_key,
                detail: "group rejected by channel policy".into(),
            });
        }

        if policy.require_mention && !message.mentioned_bot && !message.mention_all {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::NoMention,
                event_key,
                detail: "group message did not mention the bot".into(),
            });
        }

        if message.mention_all && !policy.respond_to_mention_all {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::MentionAll,
                event_key,
                detail: "mention-all message rejected by channel policy".into(),
            });
        }

        ChannelDecision::Accepted(message)
    }

    fn evaluate_dm_policy(
        &self,
        message: NormalizedMessage,
        event_key: String,
        policy: &ChannelPolicy,
    ) -> ChannelDecision<NormalizedMessage> {
        match policy.dm_mode {
            DmMode::Open => ChannelDecision::Accepted(message),
            DmMode::Disabled => ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::DmDisabled,
                event_key,
                detail: "DM rejected by channel policy".into(),
            }),
            DmMode::Allowlist => {
                let allowed = policy
                    .dm_allowlist
                    .as_ref()
                    .is_some_and(|ids| message.sender.matches_any(ids));
                if allowed {
                    ChannelDecision::Accepted(message)
                } else {
                    ChannelDecision::Rejected(RejectEvent {
                        reason: RejectReason::SenderNotAllowed,
                        event_key,
                        detail: "DM sender rejected by channel policy".into(),
                    })
                }
            }
        }
    }

    pub(super) fn remember_bot_identity(&self, identity: BotIdentity) {
        lock(&self.bot_cache).remember(identity);
    }

    pub(super) fn fresh_bot_identity(&self) -> Option<BotIdentity> {
        lock(&self.bot_cache).fresh()
    }

    pub(super) fn stale_bot_identity(&self) -> Option<BotIdentity> {
        lock(&self.bot_cache).identity.clone()
    }

    pub(super) fn throttled_stale_bot_identity(&self) -> Option<BotIdentity> {
        lock(&self.bot_cache).throttled_stale()
    }

    pub(super) fn mark_bot_identity_failure(&self) {
        lock(&self.bot_cache).last_failure_at = Some(Instant::now());
    }

    pub(super) async fn get_bot_identity(
        &self,
        client: &Client,
        option: &RequestOption,
    ) -> Result<BotIdentity, LarkError> {
        if let Some(identity) = self.fresh_bot_identity() {
            return Ok(identity);
        }
        if let Some(identity) = self.throttled_stale_bot_identity() {
            return Ok(identity);
        }

        let _guard = self.bot_refresh.lock().await;

        if let Some(identity) = self.fresh_bot_identity() {
            return Ok(identity);
        }
        if let Some(identity) = self.throttled_stale_bot_identity() {
            return Ok(identity);
        }

        match fetch_bot_identity(client, option).await {
            Ok(identity) => {
                self.remember_bot_identity(identity.clone());
                Ok(identity)
            }
            Err(err) => {
                self.mark_bot_identity_failure();
                if let Some(identity) = self.stale_bot_identity() {
                    Ok(identity)
                } else {
                    Err(err)
                }
            }
        }
    }

    fn is_bot_sender(&self, sender: &ChannelSender) -> bool {
        if matches!(sender.sender_type.as_str(), "app" | "bot") {
            return true;
        }
        lock(&self.bot_cache).matches_user(sender.user_id.as_ref())
    }
}

#[derive(Debug)]
struct DedupCache {
    capacity: usize,
    seen: HashSet<String>,
    order: VecDeque<String>,
}

impl DedupCache {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            seen: HashSet::new(),
            order: VecDeque::new(),
        }
    }

    fn resize(&mut self, capacity: usize) {
        self.capacity = capacity.max(1);
        while self.order.len() > self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.seen.remove(&old);
            }
        }
    }

    fn insert(&mut self, key: String) -> bool {
        if self.seen.contains(&key) {
            return false;
        }
        self.seen.insert(key.clone());
        self.order.push_back(key);
        while self.order.len() > self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.seen.remove(&old);
            }
        }
        true
    }
}

fn is_stale(create_time: &str, max_age: Duration) -> bool {
    let Ok(create_ms) = create_time.parse::<u128>() else {
        return false;
    };
    let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return false;
    };
    now.as_millis().saturating_sub(create_ms) > max_age.as_millis()
}
