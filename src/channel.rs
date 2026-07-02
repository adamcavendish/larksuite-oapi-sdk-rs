mod builder;
mod duration;
mod handler;
mod identity;
mod normalize;
mod policy;
mod runtime;
mod safety;
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
    ChannelResource, ChannelSender, MediaKind, NormalizedCardAction, NormalizedMessage,
    NormalizedReaction, ReactionAction, ReceiveIdType, RejectEvent, RejectReason, SendInput,
    SendResult, SendTarget, UploadInput, UploadResult,
};

#[cfg(test)]
mod tests;
