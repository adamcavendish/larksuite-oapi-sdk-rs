use std::sync::Arc;

use crate::event::EventDispatcher;
use crate::events::im::P2MessageReceiveV1;
use crate::service::common::{DownloadResp, EmptyResp};
use crate::service::im::v1::{
    CreateFileResp, CreateImageResp, CreateMessageReqBody, CreateMessageResp, FileResource,
    ImageResource, MessageResource, MessageResourceDownload, MessageType, PatchMessageReqBody,
    ReplyMessageReqBody, ReplyMessageResp,
};
use crate::ws::WsClient;
use crate::{Client, LarkError, RequestOption};

use super::builder::ChannelBuilder;
use super::identity::BotIdentity;
use super::policy::ChannelPolicy;
use super::state::ChannelState;
use super::stream::{StreamUpdate, split_markdown, text_content};
use super::types::{ChannelDecision, NormalizedMessage, SendTarget};

pub struct Channel<'a> {
    pub(super) client: &'a Client,
    pub(super) ws_client: WsClient,
    pub(super) state: Arc<ChannelState>,
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
