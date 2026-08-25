use std::sync::Arc;

use crate::card::SendReadyCard;
use crate::event::EventDispatcher;
use crate::events::im::P2MessageReceiveV1;
use crate::service::common::{DownloadResp, EmptyResp};
use crate::service::im::v1::{
    CreateFileResp, CreateImageResp, CreateMessageResp, ReplyMessageResp,
};
use crate::ws::WsClient;
use crate::{LarkClient, LarkError, RequestOption};

use super::builder::ChannelBuilder;
use super::identity::BotIdentity;
use super::policy::ChannelPolicy;
use super::state::ChannelState;
use super::stream::StreamUpdate;
use super::types::{ChannelDecision, NormalizedMessage, SendInput, SendResult, SendTarget};

pub struct Channel<'a> {
    pub(super) client: &'a LarkClient,
    pub(super) ws_client: WsClient,
    pub(super) state: Arc<ChannelState>,
}

impl<'a> Channel<'a> {
    pub fn builder(client: &'a LarkClient, dispatcher: EventDispatcher) -> ChannelBuilder<'a> {
        ChannelBuilder::new(client, dispatcher)
    }

    pub async fn start(self) -> Result<(), LarkError> {
        self.ws_client.start().await
    }

    pub fn remember_bot_identity(&self, identity: BotIdentity) {
        self.state.remember_bot_identity(identity);
    }

    pub fn policy(&self) -> ChannelPolicy {
        self.state.policy()
    }

    pub fn update_policy(&self, policy: ChannelPolicy) {
        self.state.update_policy(policy);
    }

    pub fn accept_message(&self, event: P2MessageReceiveV1) -> ChannelDecision<NormalizedMessage> {
        self.state.accept_message(event)
    }

    pub async fn get_bot_identity(&self, option: &RequestOption) -> Result<BotIdentity, LarkError> {
        self.state.get_bot_identity(self.client, option).await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().send_text")]
    pub async fn send_text(
        &self,
        target: &SendTarget,
        text: &str,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        self.client
            .channel_messaging()
            .send_text(target, text, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().send")]
    pub async fn send(
        &self,
        input: &SendInput,
        option: &RequestOption,
    ) -> Result<SendResult, LarkError> {
        self.client.channel_messaging().send(input, option).await
    }

    /// Reply to a message without ever retrying it as a top-level message.
    ///
    /// The API's default reply behavior applies. In particular, replying to a
    /// message already in a topic remains in that topic.
    #[deprecated(note = "use LarkClient::channel_messaging().reply")]
    pub async fn reply(
        &self,
        message_id: &str,
        input: &SendInput,
        option: &RequestOption,
    ) -> Result<SendResult, LarkError> {
        self.client
            .channel_messaging()
            .reply(message_id, input, option)
            .await
    }

    /// Reply to a message in its topic without ever retrying it as a top-level
    /// message.
    #[deprecated(note = "use LarkClient::channel_messaging().reply_in_thread")]
    pub async fn reply_in_thread(
        &self,
        message_id: &str,
        input: &SendInput,
        option: &RequestOption,
    ) -> Result<SendResult, LarkError> {
        self.client
            .channel_messaging()
            .reply_in_thread(message_id, input, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().send_text_with_fallback")]
    pub async fn send_text_with_fallback(
        &self,
        targets: &[SendTarget],
        text: &str,
        option: &RequestOption,
    ) -> Result<CreateMessageResp, LarkError> {
        self.client
            .channel_messaging()
            .send_text_with_fallback(targets, text, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().reply_text")]
    pub async fn reply_text(
        &self,
        message_id: &str,
        text: &str,
        reply_in_thread: bool,
        option: &RequestOption,
    ) -> Result<ReplyMessageResp, LarkError> {
        self.client
            .channel_messaging()
            .reply_text(message_id, text, reply_in_thread, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().edit_text")]
    pub async fn edit_text(
        &self,
        message_id: &str,
        text: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        self.client
            .channel_messaging()
            .edit_text(message_id, text, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().edit_card")]
    pub async fn edit_card(
        &self,
        message_id: &str,
        card: &impl SendReadyCard,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        self.client
            .channel_messaging()
            .edit_card(message_id, card, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().send_markdown_chunks")]
    pub async fn send_markdown_chunks(
        &self,
        target: &SendTarget,
        markdown: &str,
        max_chars: usize,
        option: &RequestOption,
    ) -> Result<Vec<CreateMessageResp>, LarkError> {
        self.client
            .channel_messaging()
            .send_markdown_chunks(target, markdown, max_chars, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().upload_image")]
    pub async fn upload_image(
        &self,
        image_type: &str,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<CreateImageResp, LarkError> {
        self.client
            .channel_messaging()
            .upload_image(image_type, data, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().upload_file")]
    pub async fn upload_file(
        &self,
        file_type: &str,
        file_name: &str,
        duration: Option<i64>,
        data: Vec<u8>,
        option: &RequestOption,
    ) -> Result<CreateFileResp, LarkError> {
        self.client
            .channel_messaging()
            .upload_file(file_type, file_name, duration, data, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().download_message_resource")]
    pub async fn download_message_resource(
        &self,
        message_id: &str,
        file_key: &str,
        resource_type: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        self.client
            .channel_messaging()
            .download_message_resource(message_id, file_key, resource_type, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().download_file")]
    pub async fn download_file(
        &self,
        message_id: &str,
        file_key: &str,
        media_type: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        self.client
            .channel_messaging()
            .download_file(message_id, file_key, media_type, option)
            .await
    }

    #[deprecated(note = "use LarkClient::channel_messaging().flush_stream_text")]
    pub async fn flush_stream_text(
        &self,
        stream: &mut StreamUpdate,
        option: &RequestOption,
    ) -> Result<Option<EmptyResp>, LarkError> {
        self.client
            .channel_messaging()
            .flush_stream_text(stream, option)
            .await
    }
}
