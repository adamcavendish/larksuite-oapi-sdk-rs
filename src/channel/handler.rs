use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::event::{CardActionTriggerResponse, EventDispatcher};
use crate::events::im::{P2ChatMemberBotAddedV1, P2ChatMemberBotDeletedV1, P2MessageReceiveV1};
use crate::{LarkClient, LarkError, RequestOption};

use super::state::ChannelState;
use super::types::{
    BotMembership, BotMembershipAction, ChannelDecision, ChannelEvent, NormalizedCardAction,
    NormalizedMessage, NormalizedReaction, ReactionAction, RejectEvent,
};

pub(super) type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
pub(super) type EventObserver =
    Arc<dyn Fn(ChannelEvent) -> BoxFuture<Result<(), LarkError>> + Send + Sync>;
pub(super) type MessageHandler =
    Arc<dyn Fn(NormalizedMessage) -> BoxFuture<Result<(), LarkError>> + Send + Sync>;
pub(super) type ReactionHandler =
    Arc<dyn Fn(NormalizedReaction) -> BoxFuture<Result<(), LarkError>> + Send + Sync>;
pub(super) type BotMembershipHandler =
    Arc<dyn Fn(BotMembership) -> BoxFuture<Result<(), LarkError>> + Send + Sync>;
pub(super) type CardActionHandler =
    Arc<dyn Fn(NormalizedCardAction) -> BoxFuture<Result<(), LarkError>> + Send + Sync>;
pub(super) type RejectedHandler =
    Arc<dyn Fn(RejectEvent) -> BoxFuture<Result<(), LarkError>> + Send + Sync>;
#[derive(Clone, Default)]
pub(super) struct ChannelHandlers {
    pub(super) messages: Vec<MessageHandler>,
    pub(super) reactions: Vec<ReactionHandler>,
    pub(super) bot_memberships: Vec<BotMembershipHandler>,
    pub(super) card_actions: Vec<CardActionHandler>,
    pub(super) rejected: Vec<RejectedHandler>,
    pub(super) observers: Vec<EventObserver>,
}

impl ChannelHandlers {
    pub(super) fn has_any(&self) -> bool {
        !self.messages.is_empty()
            || !self.reactions.is_empty()
            || !self.bot_memberships.is_empty()
            || !self.card_actions.is_empty()
            || !self.rejected.is_empty()
            || !self.observers.is_empty()
    }
}

pub(super) fn register_channel_handlers(
    dispatcher: EventDispatcher,
    client: LarkClient,
    state: Arc<ChannelState>,
    handlers: Arc<ChannelHandlers>,
) -> EventDispatcher {
    let message_client = client.clone();
    let message_state = state.clone();
    let message_handlers = handlers.clone();
    let dispatcher = dispatcher.on_p2_im_message_receive_v1(move |event| {
        emit_message_event(
            message_client.clone(),
            message_state.clone(),
            message_handlers.clone(),
            event,
        )
    });

    let reaction_handlers = handlers.clone();
    let dispatcher = dispatcher.on_p2_im_message_reaction_created_v1(move |event| {
        emit_reaction_event(
            reaction_handlers.clone(),
            NormalizedReaction {
                action: ReactionAction::Created,
                message_id: event.message_id,
                emoji_type: event.reaction_type.emoji_type,
                operator_type: event.operator_type,
                user_id: event.user_id,
                app_id: event.app_id,
                action_time: event.action_time,
            },
        )
    });

    let reaction_handlers = handlers.clone();
    let dispatcher = dispatcher.on_p2_im_message_reaction_deleted_v1(move |event| {
        emit_reaction_event(
            reaction_handlers.clone(),
            NormalizedReaction {
                action: ReactionAction::Deleted,
                message_id: event.message_id,
                emoji_type: event.reaction_type.emoji_type,
                operator_type: event.operator_type,
                user_id: event.user_id,
                app_id: event.app_id,
                action_time: event.action_time,
            },
        )
    });

    let bot_handlers = handlers.clone();
    let dispatcher = dispatcher.on_p2_im_chat_member_bot_added_v1(move |event| {
        emit_bot_membership_event(bot_handlers.clone(), bot_membership_added(event))
    });

    let bot_handlers = handlers.clone();
    let dispatcher = dispatcher.on_p2_im_chat_member_bot_deleted_v1(move |event| {
        emit_bot_membership_event(bot_handlers.clone(), bot_membership_deleted(event))
    });

    if dispatcher.has_callback_handler("card.action.trigger") {
        dispatcher
    } else {
        dispatcher.on_card_action_trigger(move |req| {
            let handlers = handlers.clone();
            async move {
                emit_card_action_event(handlers, NormalizedCardAction::from_request(req)).await?;
                Ok(CardActionTriggerResponse::default())
            }
        })
    }
}

pub(super) async fn emit_message_event(
    client: LarkClient,
    state: Arc<ChannelState>,
    handlers: Arc<ChannelHandlers>,
    event: P2MessageReceiveV1,
) -> Result<(), LarkError> {
    let option = bot_identity_option_for_message(&event);
    let _ = state.get_bot_identity(&client, &option).await;

    match state.accept_message(event) {
        ChannelDecision::Accepted(message) => {
            for handler in &handlers.messages {
                handler(message.clone()).await?;
            }
            emit_observers(&handlers, ChannelEvent::Message(Box::new(message))).await
        }
        ChannelDecision::Rejected(reject) => {
            for handler in &handlers.rejected {
                handler(reject.clone()).await?;
            }
            emit_observers(&handlers, ChannelEvent::Rejected(reject)).await
        }
    }
}

fn bot_identity_option_for_message(event: &P2MessageReceiveV1) -> RequestOption {
    RequestOption {
        tenant_key: (!event.sender.tenant_key.is_empty()).then(|| event.sender.tenant_key.clone()),
        ..Default::default()
    }
}

async fn emit_reaction_event(
    handlers: Arc<ChannelHandlers>,
    event: NormalizedReaction,
) -> Result<(), LarkError> {
    for handler in &handlers.reactions {
        handler(event.clone()).await?;
    }
    emit_observers(&handlers, ChannelEvent::Reaction(Box::new(event))).await
}

async fn emit_bot_membership_event(
    handlers: Arc<ChannelHandlers>,
    event: BotMembership,
) -> Result<(), LarkError> {
    for handler in &handlers.bot_memberships {
        handler(event.clone()).await?;
    }
    emit_observers(&handlers, ChannelEvent::BotMembership(Box::new(event))).await
}

async fn emit_card_action_event(
    handlers: Arc<ChannelHandlers>,
    event: NormalizedCardAction,
) -> Result<(), LarkError> {
    for handler in &handlers.card_actions {
        handler(event.clone()).await?;
    }
    emit_observers(&handlers, ChannelEvent::CardAction(Box::new(event))).await
}

async fn emit_observers(handlers: &ChannelHandlers, event: ChannelEvent) -> Result<(), LarkError> {
    for observer in &handlers.observers {
        observer(event.clone()).await?;
    }
    Ok(())
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
