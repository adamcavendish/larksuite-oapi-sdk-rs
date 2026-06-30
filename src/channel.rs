mod builder;
mod handler;
mod identity;
mod policy;
mod runtime;
mod state;
mod stream;
mod types;
mod util;

pub use builder::ChannelBuilder;
pub use identity::{BotIdentity, BotIdentityCacheConfig};
pub use policy::{ChannelPolicy, DmMode};
pub use runtime::Channel;
pub use stream::{StreamUpdate, split_markdown, split_text, text_content};
pub use types::{
    BotMembership, BotMembershipAction, ChannelDecision, ChannelEvent, ChannelMention,
    ChannelSender, NormalizedCardAction, NormalizedMessage, NormalizedReaction, ReactionAction,
    RejectEvent, RejectReason, SendTarget,
};

#[cfg(test)]
mod tests;
