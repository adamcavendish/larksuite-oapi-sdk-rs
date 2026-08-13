#[cfg(feature = "channel")]
mod builder;
mod composition;
mod duration;
#[cfg(feature = "channel")]
mod handler;
#[cfg(feature = "channel")]
mod identity;
mod messaging;
#[cfg(feature = "channel")]
mod normalize;
#[cfg(feature = "channel")]
mod policy;
#[cfg(feature = "channel")]
mod runtime;
mod safety;
#[cfg(feature = "channel")]
mod state;
mod stream;
mod types;
#[cfg(feature = "channel")]
mod util;

pub use messaging::ChannelMessaging;
pub use stream::{StreamUpdate, split_markdown, split_text, text_content};
pub use types::{
    ChannelMention, MediaKind, ReceiveIdType, SendInput, SendResult, SendTarget, UploadInput,
    UploadResult,
};

#[cfg(feature = "channel")]
pub use builder::ChannelBuilder;
#[cfg(feature = "channel")]
pub use identity::{BotIdentity, BotIdentityCacheConfig};
#[cfg(feature = "channel")]
pub use policy::{ChannelPolicy, DmMode};
#[cfg(feature = "channel")]
pub use runtime::Channel;
#[cfg(feature = "channel")]
pub use types::{
    BotMembership, BotMembershipAction, ChannelDecision, ChannelEvent, ChannelResource,
    ChannelSender, NormalizedCardAction, NormalizedMessage, NormalizedReaction, ReactionAction,
    RejectEvent, RejectReason,
};

#[cfg(test)]
mod messaging_tests;
#[cfg(test)]
mod tests;
