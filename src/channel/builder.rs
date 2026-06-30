use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use crate::event::EventDispatcher;
use crate::{Client, LarkError};

use super::handler::{ChannelHandlers, register_channel_handlers};
use super::identity::BotIdentityCacheConfig;
use super::policy::ChannelPolicy;
use super::runtime::Channel;
use super::state::ChannelState;
use super::types::{
    BotMembership, ChannelEvent, NormalizedCardAction, NormalizedMessage, NormalizedReaction,
    RejectEvent,
};

type LifecycleCallback = Arc<dyn Fn() + Send + Sync>;
type ErrorCallback = Arc<dyn Fn(&LarkError) + Send + Sync>;

pub struct ChannelBuilder<'a> {
    client: &'a Client,
    dispatcher: EventDispatcher,
    policy: ChannelPolicy,
    bot_identity_cache: BotIdentityCacheConfig,
    handlers: ChannelHandlers,
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
            bot_identity_cache: BotIdentityCacheConfig::default(),
            handlers: ChannelHandlers::default(),
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

    pub fn bot_identity_cache(mut self, config: BotIdentityCacheConfig) -> Self {
        self.bot_identity_cache = config.normalized();
        self
    }

    pub fn on_message<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(NormalizedMessage) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.handlers
            .messages
            .push(Arc::new(move |event| Box::pin(handler(event))));
        self
    }

    pub fn on_reaction<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(NormalizedReaction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.handlers
            .reactions
            .push(Arc::new(move |event| Box::pin(handler(event))));
        self
    }

    pub fn on_bot_membership<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(BotMembership) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.handlers
            .bot_memberships
            .push(Arc::new(move |event| Box::pin(handler(event))));
        self
    }

    pub fn on_card_action<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(NormalizedCardAction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.handlers
            .card_actions
            .push(Arc::new(move |event| Box::pin(handler(event))));
        self
    }

    pub fn on_rejected<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(RejectEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.handlers
            .rejected
            .push(Arc::new(move |event| Box::pin(handler(event))));
        self
    }

    pub fn on_event<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(ChannelEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.handlers
            .observers
            .push(Arc::new(move |event| Box::pin(handler(event))));
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
        let state = Arc::new(ChannelState::new(self.policy, self.bot_identity_cache));
        let dispatcher = if self.handlers.has_any() {
            register_channel_handlers(
                self.dispatcher,
                self.client.clone(),
                state.clone(),
                Arc::new(self.handlers),
            )
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
