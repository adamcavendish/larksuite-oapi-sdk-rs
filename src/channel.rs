use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::event::{CardActionTriggerRequest, CardActionTriggerResponse, EventDispatcher};
use crate::events::common::UserId;
use crate::events::im::{P2ChatMemberBotAddedV1, P2ChatMemberBotDeletedV1, P2MessageReceiveV1};
use crate::service::common::{DownloadResp, EmptyResp};
use crate::service::im::v1::{
    CreateFileResp, CreateImageResp, CreateMessageReqBody, CreateMessageResp, FileResource,
    ImageResource, MessageResource, MessageResourceDownload, MessageType, PatchMessageReqBody,
    ReplyMessageReqBody, ReplyMessageResp,
};
use crate::ws::WsClient;
use crate::{Client, LarkError, RequestOption};

type ChannelHandler = Arc<
    dyn Fn(ChannelEvent) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
        + Send
        + Sync,
>;
type LifecycleCallback = Arc<dyn Fn() + Send + Sync>;
type ErrorCallback = Arc<dyn Fn(&LarkError) + Send + Sync>;

#[derive(Debug, Clone)]
pub struct ChannelPolicy {
    pub allow_bot_senders: bool,
    pub allowed_message_types: Option<HashSet<String>>,
    pub max_event_age: Option<Duration>,
    pub reject_duplicates: bool,
    pub dedup_capacity: usize,
}

impl Default for ChannelPolicy {
    fn default() -> Self {
        Self {
            allow_bot_senders: false,
            allowed_message_types: None,
            max_event_age: Some(Duration::from_secs(10 * 60)),
            reject_duplicates: true,
            dedup_capacity: 4096,
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
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotIdentity {
    pub app_id: Option<String>,
    pub open_id: Option<String>,
    pub user_id: Option<String>,
    pub union_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSender {
    pub sender_type: String,
    pub tenant_key: String,
    pub user_id: Option<UserId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMention {
    pub key: String,
    pub id: Option<UserId>,
    pub mentioned_type: String,
    pub name: String,
    pub tenant_key: String,
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
}

impl NormalizedMessage {
    pub fn from_event(event: P2MessageReceiveV1) -> Self {
        let text = extract_text_content(&event.message.message_type, &event.message.content);
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
            mentions: event
                .message
                .mentions
                .into_iter()
                .map(|mention| ChannelMention {
                    key: mention.key,
                    id: mention.id,
                    mentioned_type: mention.mentioned_type,
                    name: mention.name,
                    tenant_key: mention.tenant_key,
                })
                .collect(),
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

pub struct ChannelBuilder<'a> {
    client: &'a Client,
    dispatcher: EventDispatcher,
    policy: ChannelPolicy,
    handler: Option<ChannelHandler>,
    domain: Option<String>,
    headers: HashMap<String, String>,
    auto_reconnect: bool,
    log_level: Option<tracing::Level>,
    on_ready: Option<LifecycleCallback>,
    on_error: Option<ErrorCallback>,
    on_reconnecting: Option<LifecycleCallback>,
    on_reconnected: Option<LifecycleCallback>,
    on_disconnected: Option<LifecycleCallback>,
}

impl<'a> ChannelBuilder<'a> {
    pub fn new(client: &'a Client, dispatcher: EventDispatcher) -> Self {
        Self {
            client,
            dispatcher,
            policy: ChannelPolicy::default(),
            handler: None,
            domain: None,
            headers: HashMap::new(),
            auto_reconnect: true,
            log_level: None,
            on_ready: None,
            on_error: None,
            on_reconnecting: None,
            on_reconnected: None,
            on_disconnected: None,
        }
    }

    pub fn policy(mut self, policy: ChannelPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn on_event<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(ChannelEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.handler = Some(Arc::new(move |event| Box::pin(handler(event))));
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn auto_reconnect(mut self, enable: bool) -> Self {
        self.auto_reconnect = enable;
        self
    }

    pub fn log_level(mut self, level: tracing::Level) -> Self {
        self.log_level = Some(level);
        self
    }

    pub fn on_ready<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_ready = Some(Arc::new(handler));
        self
    }

    pub fn on_error<F>(mut self, handler: F) -> Self
    where
        F: Fn(&LarkError) + Send + Sync + 'static,
    {
        self.on_error = Some(Arc::new(handler));
        self
    }

    pub fn on_reconnecting<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_reconnecting = Some(Arc::new(handler));
        self
    }

    pub fn on_reconnected<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_reconnected = Some(Arc::new(handler));
        self
    }

    pub fn on_disconnected<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_disconnected = Some(Arc::new(handler));
        self
    }

    pub fn build(self) -> Channel<'a> {
        let state = Arc::new(ChannelState::new(self.policy));
        let dispatcher = if let Some(handler) = self.handler {
            register_channel_handlers(self.dispatcher, state.clone(), handler)
        } else {
            self.dispatcher
        };

        let mut ws_client = self
            .client
            .ws_client(dispatcher)
            .auto_reconnect(self.auto_reconnect);
        if let Some(domain) = self.domain {
            ws_client = ws_client.domain(domain);
        }
        if !self.headers.is_empty() {
            ws_client = ws_client.headers(self.headers);
        }
        if let Some(level) = self.log_level {
            ws_client = ws_client.log_level(level);
        }
        if let Some(handler) = self.on_ready {
            ws_client = ws_client.on_ready(move || handler());
        }
        if let Some(handler) = self.on_error {
            ws_client = ws_client.on_error(move |err| handler(err));
        }
        if let Some(handler) = self.on_reconnecting {
            ws_client = ws_client.on_reconnecting(move || handler());
        }
        if let Some(handler) = self.on_reconnected {
            ws_client = ws_client.on_reconnected(move || handler());
        }
        if let Some(handler) = self.on_disconnected {
            ws_client = ws_client.on_disconnected(move || handler());
        }

        Channel {
            client: self.client,
            ws_client,
            state,
        }
    }
}

pub struct Channel<'a> {
    client: &'a Client,
    ws_client: WsClient,
    state: Arc<ChannelState>,
}

impl<'a> Channel<'a> {
    pub fn builder(client: &'a Client, dispatcher: EventDispatcher) -> ChannelBuilder<'a> {
        ChannelBuilder::new(client, dispatcher)
    }

    pub async fn start(self) -> Result<(), LarkError> {
        self.ws_client.start().await
    }

    pub fn remember_bot_identity(&self, identity: BotIdentity) {
        self.state.remember_bot_identity(identity);
    }

    pub fn accept_message(&self, event: P2MessageReceiveV1) -> ChannelDecision<NormalizedMessage> {
        self.state.accept_message(event)
    }

    pub async fn send_text(
        &self,
        target: &SendTarget,
        text: &str,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        self.message_resource()
            .create(
                &target.receive_id_type,
                &CreateMessageReqBody {
                    receive_id: Some(target.receive_id.clone()),
                    msg_type: Some(MessageType::TEXT.to_string()),
                    content: Some(text_content(text)?),
                    uuid: None,
                },
                option,
            )
            .await
    }

    pub async fn send_text_with_fallback(
        &self,
        targets: &[SendTarget],
        text: &str,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        let mut last_error = None;
        let mut last_api_error = None;
        for target in targets {
            match self.send_text(target, text, option).await {
                Ok(resp) if resp.success() => return Ok(resp),
                Ok(resp) => last_api_error = Some(resp.code_error.clone()),
                Err(err) => last_error = Some(err),
            }
        }
        if let Some(code_error) = last_api_error {
            return Err(LarkError::Api(Box::new(code_error)));
        }
        Err(last_error.unwrap_or_else(|| {
            LarkError::IllegalParam("send_text_with_fallback requires at least one target".into())
        }))
    }

    pub async fn reply_text(
        &self,
        message_id: &str,
        text: &str,
        reply_in_thread: bool,
        option: &RequestOption,
    ) -> Result<ReplyMessageResp, LarkError> {
        self.message_resource()
            .reply(
                message_id,
                &ReplyMessageReqBody {
                    content: Some(text_content(text)?),
                    msg_type: Some(MessageType::TEXT.to_string()),
                    reply_in_thread: Some(reply_in_thread),
                    uuid: None,
                },
                option,
            )
            .await
    }

    pub async fn edit_text(
        &self,
        message_id: &str,
        text: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        self.message_resource()
            .patch(
                message_id,
                &PatchMessageReqBody {
                    content: Some(text_content(text)?),
                },
                option,
            )
            .await
    }

    pub async fn send_markdown_chunks(
        &self,
        target: &SendTarget,
        markdown: &str,
        max_chars: usize,
        option: &RequestOption,
    ) -> Result<Vec<CreateMessageResp>, LarkError> {
        let mut responses = Vec::new();
        for chunk in split_markdown(markdown, max_chars) {
            responses.push(self.send_text(target, &chunk, option).await?);
        }
        Ok(responses)
    }

    pub async fn upload_image(
        &self,
        image_type: &str,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<CreateImageResp, LarkError> {
        self.image_resource().create(image_type, data, option).await
    }

    pub async fn upload_file(
        &self,
        file_type: &str,
        file_name: &str,
        duration: Option<i64>,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<CreateFileResp, LarkError> {
        self.file_resource()
            .create(file_type, file_name, duration, data, option)
            .await
    }

    pub async fn download_message_resource(
        &self,
        message_id: &str,
        file_key: &str,
        resource_type: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        self.message_resource_download()
            .get(message_id, file_key, resource_type, option)
            .await
    }

    pub async fn flush_stream_text(
        &self,
        stream: &mut StreamUpdate,
        option: &RequestOption,
    ) -> Result<Option<EmptyResp>, LarkError> {
        if !stream.should_flush() {
            return Ok(None);
        }
        let content = stream.pending_content();
        let resp = self.edit_text(&stream.message_id, &content, option).await?;
        stream.mark_flushed();
        Ok(Some(resp))
    }

    fn message_resource(&self) -> MessageResource<'_> {
        self.client.im().message
    }

    fn message_resource_download(&self) -> MessageResourceDownload<'_> {
        self.client.im().message_resource
    }

    fn file_resource(&self) -> FileResource<'_> {
        self.client.im().file
    }

    fn image_resource(&self) -> ImageResource<'_> {
        self.client.im().image
    }
}

#[derive(Debug, Clone)]
pub struct StreamUpdate {
    pub message_id: String,
    pub content: String,
    throttle: Duration,
    last_flush: Option<Instant>,
}

impl StreamUpdate {
    pub fn new(message_id: impl Into<String>, throttle: Duration) -> Self {
        Self {
            message_id: message_id.into(),
            content: String::new(),
            throttle,
            last_flush: None,
        }
    }

    pub fn push(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn force_due(&mut self) {
        self.last_flush = None;
    }

    fn should_flush(&self) -> bool {
        !self.content.is_empty()
            && self
                .last_flush
                .is_none_or(|last| last.elapsed() >= self.throttle)
    }

    fn pending_content(&self) -> String {
        self.content.clone()
    }

    fn mark_flushed(&mut self) {
        self.last_flush = Some(Instant::now());
    }
}

#[derive(Debug)]
struct ChannelState {
    policy: ChannelPolicy,
    dedup: Mutex<DedupCache>,
    bot_cache: Mutex<BotIdentityCache>,
}

impl ChannelState {
    fn new(policy: ChannelPolicy) -> Self {
        Self {
            dedup: Mutex::new(DedupCache::new(policy.dedup_capacity)),
            bot_cache: Mutex::new(BotIdentityCache::default()),
            policy,
        }
    }

    fn accept_message(&self, event: P2MessageReceiveV1) -> ChannelDecision<NormalizedMessage> {
        let message = NormalizedMessage::from_event(event);
        let event_key = message.message_id.clone();
        if self.policy.reject_duplicates
            && !event_key.is_empty()
            && !lock(&self.dedup).insert(event_key.clone())
        {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::Duplicate,
                event_key,
                detail: "message already handled".into(),
            });
        }

        if let Some(max_age) = self.policy.max_event_age
            && is_stale(&message.create_time, max_age)
        {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::Stale,
                event_key,
                detail: "message create_time is older than policy max_event_age".into(),
            });
        }

        if !self.policy.allow_bot_senders && self.is_bot_sender(&message.sender) {
            return ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::BotSender,
                event_key,
                detail: "bot sender rejected by channel policy".into(),
            });
        }

        if let Some(allowed) = &self.policy.allowed_message_types
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

        ChannelDecision::Accepted(message)
    }

    fn remember_bot_identity(&self, identity: BotIdentity) {
        lock(&self.bot_cache).remember(identity);
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
    fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            seen: HashSet::new(),
            order: VecDeque::new(),
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

#[derive(Debug, Default)]
struct BotIdentityCache {
    identities: Vec<BotIdentity>,
}

impl BotIdentityCache {
    fn remember(&mut self, identity: BotIdentity) {
        self.identities.push(identity);
    }

    fn matches_user(&self, user: Option<&UserId>) -> bool {
        let Some(user) = user else {
            return false;
        };
        self.identities.iter().any(|identity| {
            identity
                .open_id
                .as_deref()
                .is_some_and(|id| user.open_id() == Some(id))
                || identity
                    .user_id
                    .as_deref()
                    .is_some_and(|id| user.user_id() == Some(id))
                || identity
                    .union_id
                    .as_deref()
                    .is_some_and(|id| user.union_id() == Some(id))
        })
    }
}

fn register_channel_handlers(
    dispatcher: EventDispatcher,
    state: Arc<ChannelState>,
    handler: ChannelHandler,
) -> EventDispatcher {
    let message_state = state.clone();
    let message_handler = handler.clone();
    let dispatcher = dispatcher.on_p2_im_message_receive_v1(move |event| {
        emit_message_event(message_state.clone(), message_handler.clone(), event)
    });

    let reaction_handler = handler.clone();
    let dispatcher = dispatcher.on_p2_im_message_reaction_created_v1(move |event| {
        emit_channel_event(
            reaction_handler.clone(),
            ChannelEvent::Reaction(Box::new(NormalizedReaction {
                action: ReactionAction::Created,
                message_id: event.message_id,
                emoji_type: event.reaction_type.emoji_type,
                operator_type: event.operator_type,
                user_id: event.user_id,
                app_id: event.app_id,
                action_time: event.action_time,
            })),
        )
    });

    let reaction_handler = handler.clone();
    let dispatcher = dispatcher.on_p2_im_message_reaction_deleted_v1(move |event| {
        emit_channel_event(
            reaction_handler.clone(),
            ChannelEvent::Reaction(Box::new(NormalizedReaction {
                action: ReactionAction::Deleted,
                message_id: event.message_id,
                emoji_type: event.reaction_type.emoji_type,
                operator_type: event.operator_type,
                user_id: event.user_id,
                app_id: event.app_id,
                action_time: event.action_time,
            })),
        )
    });

    let bot_handler = handler.clone();
    let dispatcher = dispatcher.on_p2_im_chat_member_bot_added_v1(move |event| {
        emit_channel_event(
            bot_handler.clone(),
            ChannelEvent::BotMembership(Box::new(bot_membership_added(event))),
        )
    });

    let bot_handler = handler.clone();
    let dispatcher = dispatcher.on_p2_im_chat_member_bot_deleted_v1(move |event| {
        emit_channel_event(
            bot_handler.clone(),
            ChannelEvent::BotMembership(Box::new(bot_membership_deleted(event))),
        )
    });

    if dispatcher.has_callback_handler("card.action.trigger") {
        dispatcher
    } else {
        dispatcher.on_card_action_trigger(move |req| {
            let handler = handler.clone();
            async move {
                handler(ChannelEvent::CardAction(Box::new(
                    NormalizedCardAction::from_request(req),
                )))
                .await?;
                Ok(CardActionTriggerResponse::default())
            }
        })
    }
}

async fn emit_message_event(
    state: Arc<ChannelState>,
    handler: ChannelHandler,
    event: P2MessageReceiveV1,
) -> Result<(), LarkError> {
    match state.accept_message(event) {
        ChannelDecision::Accepted(message) => {
            handler(ChannelEvent::Message(Box::new(message))).await
        }
        ChannelDecision::Rejected(reject) => handler(ChannelEvent::Rejected(reject)).await,
    }
}

async fn emit_channel_event(handler: ChannelHandler, event: ChannelEvent) -> Result<(), LarkError> {
    handler(event).await
}

fn bot_membership_added(event: P2ChatMemberBotAddedV1) -> BotMembership {
    BotMembership {
        action: BotMembershipAction::Added,
        chat_id: event.chat_id,
        operator_id: event.operator_id,
        external: event.external,
        operator_tenant_key: event.operator_tenant_key,
        name: event.name,
    }
}

fn bot_membership_deleted(event: P2ChatMemberBotDeletedV1) -> BotMembership {
    BotMembership {
        action: BotMembershipAction::Deleted,
        chat_id: event.chat_id,
        operator_id: event.operator_id,
        external: event.external,
        operator_tenant_key: event.operator_tenant_key,
        name: event.name,
    }
}

pub fn text_content(text: &str) -> Result<String, serde_json::Error> {
    serde_json::to_string(&serde_json::json!({ "text": text }))
}

pub fn split_text(text: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(1);
    let mut chunks = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if current.chars().count() >= max_chars {
            chunks.push(std::mem::take(&mut current));
        }
        current.push(ch);
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

pub fn split_markdown(markdown: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(1);
    let mut chunks = Vec::new();
    let mut current = String::new();
    for line in markdown.split_inclusive('\n') {
        if !current.is_empty() && current.chars().count() + line.chars().count() > max_chars {
            chunks.push(std::mem::take(&mut current));
        }
        if line.chars().count() > max_chars {
            chunks.extend(split_text(line, max_chars));
        } else {
            current.push_str(line);
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

fn extract_text_content(message_type: &str, content: &str) -> Option<String> {
    if message_type != MessageType::TEXT {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(content)
        .ok()
        .and_then(|value| {
            value
                .get("text")
                .and_then(|text| text.as_str())
                .map(str::to_owned)
        })
        .or_else(|| Some(content.to_string()).filter(|s| !s.is_empty()))
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

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|err| err.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventReq;
    use crate::events::im::{Message, MessageSender};

    fn message_event(message_id: &str, message_type: &str, content: &str) -> P2MessageReceiveV1 {
        P2MessageReceiveV1 {
            sender: MessageSender {
                sender_id: Some(UserId {
                    open_id: Some("ou_user".into()),
                    ..Default::default()
                }),
                sender_type: "user".into(),
                tenant_key: "tenant".into(),
            },
            message: Message {
                message_id: message_id.into(),
                message_type: message_type.into(),
                content: content.into(),
                create_time: now_ms().to_string(),
                ..Default::default()
            },
        }
    }

    #[test]
    fn normalizes_text_message_content() {
        let event = message_event("om_1", MessageType::TEXT, r#"{"text":"hello"}"#);
        let message = NormalizedMessage::from_event(event);
        assert_eq!(message.text.as_deref(), Some("hello"));
    }

    #[test]
    fn rejects_duplicate_messages() {
        let state = ChannelState::new(ChannelPolicy::default());
        let first = state.accept_message(message_event("om_dup", MessageType::TEXT, "{}"));
        let second = state.accept_message(message_event("om_dup", MessageType::TEXT, "{}"));
        assert!(matches!(first, ChannelDecision::Accepted(_)));
        assert!(matches!(
            second,
            ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::Duplicate,
                ..
            })
        ));
    }

    #[test]
    fn rejects_cached_bot_sender() {
        let state = ChannelState::new(ChannelPolicy::default());
        state.remember_bot_identity(BotIdentity {
            open_id: Some("ou_user".into()),
            ..Default::default()
        });
        let result = state.accept_message(message_event("om_bot", MessageType::TEXT, "{}"));
        assert!(matches!(
            result,
            ChannelDecision::Rejected(RejectEvent {
                reason: RejectReason::BotSender,
                ..
            })
        ));
    }

    #[test]
    fn split_markdown_keeps_char_boundaries() {
        let chunks = split_markdown("一二三四五", 2);
        assert_eq!(chunks, vec!["一二", "三四", "五"]);
    }

    #[test]
    fn stream_throttle_blocks_immediate_flush() {
        let mut stream = StreamUpdate::new("om_1", Duration::from_secs(60));
        stream.push("hello");
        assert!(stream.should_flush());
        let _ = stream.pending_content();
        assert!(stream.should_flush());
        stream.mark_flushed();
        assert!(!stream.should_flush());
        stream.force_due();
        assert!(stream.should_flush());
    }

    #[tokio::test]
    async fn channel_registration_preserves_existing_card_action_handler() {
        let existing_called = Arc::new(Mutex::new(false));
        let channel_called = Arc::new(Mutex::new(false));

        let existing_flag = existing_called.clone();
        let dispatcher = EventDispatcher::new("", "")
            .skip_sign_verify()
            .on_card_action_trigger(move |_| {
                let existing_flag = existing_flag.clone();
                async move {
                    *existing_flag.lock().unwrap() = true;
                    Ok(CardActionTriggerResponse::default())
                }
            });

        let channel_flag = channel_called.clone();
        let handler: ChannelHandler = Arc::new(move |_| {
            let channel_flag = channel_flag.clone();
            Box::pin(async move {
                *channel_flag.lock().unwrap() = true;
                Ok(())
            })
        });

        let dispatcher = register_channel_handlers(
            dispatcher,
            Arc::new(ChannelState::new(ChannelPolicy::default())),
            handler,
        );
        let body = serde_json::json!({
            "schema": "2.0",
            "header": {
                "event_id": "ev_trigger",
                "event_type": "card.action.trigger",
                "app_id": "cli_test",
                "tenant_key": "t1",
                "create_time": "0"
            },
            "event": {
                "action": { "tag": "button", "value": {} },
                "token": "tok",
                "host": "lark",
                "delivery_type": "push"
            }
        });
        let resp = dispatcher
            .handle(EventReq {
                headers: Default::default(),
                body: serde_json::to_vec(&body).unwrap(),
                request_uri: String::new(),
            })
            .await;

        assert_eq!(resp.status_code, 200);
        assert!(*existing_called.lock().unwrap());
        assert!(!*channel_called.lock().unwrap());
    }

    fn now_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_millis()
    }
}
