use std::path::Path;
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
use super::normalize::{compose_mentions_text_prefix, markdown_to_post};
use super::policy::ChannelPolicy;
use super::state::ChannelState;
use super::stream::{StreamUpdate, split_markdown, text_content};
use super::types::{
    ChannelDecision, MediaKind, NormalizedMessage, ReceiveIdType, SendInput, SendResult,
    SendTarget, UploadInput, UploadResult,
};

const DEFAULT_TEXT_CHUNK_LIMIT: usize = 20_000;

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

    pub async fn send(
        &self,
        input: &SendInput,
        option: &RequestOption,
    ) -> Result<SendResult, LarkError> {
        let (receive_id_type, receive_id) = resolve_target(input)?;
        let mut input = input.clone();
        self.prepare_uploads(&mut input, option).await?;

        match build_payloads(&input)? {
            SendPayloads::Single { msg_type, content } => {
                self.send_one_with_fallback(
                    &receive_id_type,
                    &receive_id,
                    &msg_type,
                    &content,
                    &input,
                    option,
                )
                .await
            }
            SendPayloads::Chunks(chunks) => {
                let mut chunk_ids = Vec::new();
                let mut chat_id = None;
                for (msg_type, content) in chunks {
                    let result = self
                        .send_one_with_fallback(
                            &receive_id_type,
                            &receive_id,
                            &msg_type,
                            &content,
                            &input,
                            option,
                        )
                        .await?;
                    if chat_id.is_none() {
                        chat_id = result.chat_id.clone();
                    }
                    chunk_ids.push(result.message_id);
                }
                let message_id = chunk_ids.first().cloned().ok_or_else(|| {
                    LarkError::IllegalParam("channel send produced no chunks".into())
                })?;
                Ok(SendResult {
                    message_id,
                    chunk_ids: if chunk_ids.len() > 1 {
                        chunk_ids
                    } else {
                        Vec::new()
                    },
                    chat_id,
                })
            }
        }
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

    pub async fn download_file(
        &self,
        message_id: &str,
        file_key: &str,
        media_type: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        self.download_message_resource(message_id, file_key, media_type, option)
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

    async fn prepare_uploads(
        &self,
        input: &mut SendInput,
        option: &RequestOption,
    ) -> Result<(), LarkError> {
        if input
            .image_path
            .as_deref()
            .is_some_and(|path| !path.is_empty())
            && input.image_key.as_deref().is_none_or(str::is_empty)
        {
            let path = input.image_path.as_deref().unwrap();
            let data = std::fs::read(path).map_err(|e| {
                LarkError::IllegalParam(format!("failed to read image_path {path}: {e}"))
            })?;
            let resp = self.upload_image("message", data, option).await?;
            if !resp.success() {
                return Err(LarkError::Api(Box::new(resp.code_error)));
            }
            input.image_key =
                Some(resp.data.and_then(|data| data.image_key).ok_or_else(|| {
                    LarkError::IllegalParam("image upload returned no key".into())
                })?);
        }

        if input
            .file_path
            .as_deref()
            .is_some_and(|path| !path.is_empty())
            && input.file_key.as_deref().is_none_or(str::is_empty)
        {
            let path = input.file_path.as_deref().unwrap();
            let data = std::fs::read(path).map_err(|e| {
                LarkError::IllegalParam(format!("failed to read file_path {path}: {e}"))
            })?;
            let file_name = file_name_from_path(path);
            let resp = self
                .upload_file("stream", &file_name, None, data, option)
                .await?;
            if !resp.success() {
                return Err(LarkError::Api(Box::new(resp.code_error)));
            }
            input.file_key =
                Some(resp.data.and_then(|data| data.file_key).ok_or_else(|| {
                    LarkError::IllegalParam("file upload returned no key".into())
                })?);
        }

        if let Some(media) = input.media.as_ref().filter(|_| {
            input.audio_key.as_deref().is_none_or(str::is_empty)
                && input.video_key.as_deref().is_none_or(str::is_empty)
                && input.file_key.as_deref().is_none_or(str::is_empty)
                && input.image_key.as_deref().is_none_or(str::is_empty)
        }) {
            let uploaded = self.upload_media(media, option).await?;
            match uploaded.kind {
                Some(MediaKind::Image) => input.image_key = Some(uploaded.file_key),
                Some(MediaKind::Audio) => input.audio_key = Some(uploaded.file_key),
                Some(MediaKind::Video) => input.video_key = Some(uploaded.file_key),
                Some(MediaKind::File) | None => input.file_key = Some(uploaded.file_key),
            }
        }

        Ok(())
    }

    async fn upload_media(
        &self,
        input: &UploadInput,
        option: &RequestOption,
    ) -> Result<UploadResult, LarkError> {
        if input
            .source_url
            .as_deref()
            .is_some_and(|url| !url.is_empty())
        {
            return Err(LarkError::IllegalParam(
                "channel media source_url upload is not supported yet; pass source_bytes, source_path, or a pre-uploaded key".into(),
            ));
        }
        let kind = input
            .kind
            .ok_or_else(|| LarkError::IllegalParam("media kind is required".into()))?;
        let data = if let Some(bytes) = &input.source_bytes {
            bytes.clone()
        } else if let Some(path) = input.source_path.as_deref().filter(|path| !path.is_empty()) {
            std::fs::read(path).map_err(|e| {
                LarkError::IllegalParam(format!("failed to read media source_path {path}: {e}"))
            })?
        } else {
            return Err(LarkError::IllegalParam(
                "media source_bytes or source_path is required".into(),
            ));
        };

        if kind == MediaKind::Image {
            let resp = self.upload_image("message", data, option).await?;
            if !resp.success() {
                return Err(LarkError::Api(Box::new(resp.code_error)));
            }
            return Ok(UploadResult {
                kind: Some(kind),
                file_key: resp.data.and_then(|data| data.image_key).ok_or_else(|| {
                    LarkError::IllegalParam("image upload returned no key".into())
                })?,
                duration_ms: None,
            });
        }

        if matches!(kind, MediaKind::Audio | MediaKind::Video) && input.duration.is_none() {
            return Err(LarkError::IllegalParam(
                "audio/video media upload requires explicit duration; automatic duration detection is not supported yet".into(),
            ));
        }
        let file_type = match kind {
            MediaKind::Audio => "opus",
            MediaKind::Video => "mp4",
            MediaKind::File | MediaKind::Image => "stream",
        };
        let file_name = input
            .file_name
            .clone()
            .or_else(|| input.source_path.as_deref().map(file_name_from_path))
            .unwrap_or_else(|| match kind {
                MediaKind::Audio => "voice.opus".into(),
                MediaKind::Video => "video.mp4".into(),
                _ => "upload.bin".into(),
            });
        let resp = self
            .upload_file(file_type, &file_name, input.duration, data, option)
            .await?;
        if !resp.success() {
            return Err(LarkError::Api(Box::new(resp.code_error)));
        }
        Ok(UploadResult {
            kind: Some(kind),
            file_key: resp
                .data
                .and_then(|data| data.file_key)
                .ok_or_else(|| LarkError::IllegalParam("media upload returned no key".into()))?,
            duration_ms: input.duration,
        })
    }

    async fn send_one_with_fallback(
        &self,
        receive_id_type: &str,
        receive_id: &str,
        msg_type: &str,
        content: &str,
        input: &SendInput,
        option: &RequestOption,
    ) -> Result<SendResult, LarkError> {
        match self
            .send_one(
                receive_id_type,
                receive_id,
                msg_type,
                content,
                input.reply_message_id.as_deref(),
                option,
            )
            .await
        {
            Ok(result) => Ok(result),
            Err(err) if is_reply_target_gone(&err) && input.reply_message_id.is_some() => {
                self.send_one(receive_id_type, receive_id, msg_type, content, None, option)
                    .await
            }
            Err(err) if is_format_error(&err) && msg_type != MessageType::TEXT => {
                let fallback = input.markdown.as_deref().or(input.text.as_deref());
                if let Some(fallback) = fallback {
                    let fallback_text = format!(
                        "{}{}",
                        compose_mentions_text_prefix(&input.mentions),
                        fallback
                    );
                    let content = text_content(&fallback_text)?;
                    self.send_one(
                        receive_id_type,
                        receive_id,
                        MessageType::TEXT,
                        &content,
                        input.reply_message_id.as_deref(),
                        option,
                    )
                    .await
                } else {
                    Err(err)
                }
            }
            Err(err) => Err(err),
        }
    }

    async fn send_one(
        &self,
        receive_id_type: &str,
        receive_id: &str,
        msg_type: &str,
        content: &str,
        reply_message_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<SendResult, LarkError> {
        if let Some(message_id) = reply_message_id.filter(|id| !id.is_empty()) {
            let resp = self
                .message_resource()
                .reply(
                    message_id,
                    &ReplyMessageReqBody {
                        content: Some(content.to_string()),
                        msg_type: Some(msg_type.to_string()),
                        reply_in_thread: None,
                        uuid: None,
                    },
                    option,
                )
                .await?;
            if !resp.success() {
                return Err(LarkError::Api(Box::new(resp.code_error)));
            }
            let data = resp.data.unwrap_or_default();
            return Ok(SendResult {
                message_id: data.message_id.unwrap_or_default(),
                chat_id: data.chat_id,
                chunk_ids: Vec::new(),
            });
        }

        let resp = self
            .message_resource()
            .create(
                receive_id_type,
                &CreateMessageReqBody {
                    receive_id: Some(receive_id.to_string()),
                    msg_type: Some(msg_type.to_string()),
                    content: Some(content.to_string()),
                    uuid: None,
                },
                option,
            )
            .await?;
        if !resp.success() {
            return Err(LarkError::Api(Box::new(resp.code_error)));
        }
        let data = resp.data.unwrap_or_default();
        Ok(SendResult {
            message_id: data.message_id.unwrap_or_default(),
            chat_id: data.chat_id,
            chunk_ids: Vec::new(),
        })
    }
}

fn resolve_target(input: &SendInput) -> Result<(String, String), LarkError> {
    if let Some(receive_id) = input.receive_id.as_deref().filter(|id| !id.is_empty()) {
        return Ok((
            ReceiveIdType::detect(receive_id)?.as_str().to_string(),
            receive_id.to_string(),
        ));
    }
    if let Some(chat_id) = input.chat_id.as_deref().filter(|id| !id.is_empty()) {
        return Ok((
            ReceiveIdType::ChatId.as_str().to_string(),
            chat_id.to_string(),
        ));
    }
    if let Some(user_id) = input.user_id.as_deref().filter(|id| !id.is_empty()) {
        return Ok((
            ReceiveIdType::UserId.as_str().to_string(),
            user_id.to_string(),
        ));
    }
    Err(LarkError::IllegalParam(
        "ReceiveID, ChatID, or UserID must be provided".into(),
    ))
}

enum SendPayloads {
    Single { msg_type: String, content: String },
    Chunks(Vec<(String, String)>),
}

fn build_payloads(input: &SendInput) -> Result<SendPayloads, LarkError> {
    if let Some(image_key) = non_empty(input.image_key.as_deref()) {
        return single_json(
            MessageType::IMAGE,
            serde_json::json!({ "image_key": image_key }),
        );
    }
    if let Some(audio_key) = non_empty(input.audio_key.as_deref()) {
        return single_json(
            MessageType::AUDIO,
            serde_json::json!({ "file_key": audio_key }),
        );
    }
    if let Some(video_key) = non_empty(input.video_key.as_deref()) {
        return single_json(
            MessageType::MEDIA,
            serde_json::json!({ "file_key": video_key }),
        );
    }
    if let Some(file_key) = non_empty(input.file_key.as_deref()) {
        return single_json(
            MessageType::FILE,
            serde_json::json!({ "file_key": file_key }),
        );
    }
    if let Some(card) = non_empty(input.card.as_deref()) {
        return Ok(single(MessageType::INTERACTIVE, card));
    }
    if let Some(post) = non_empty(input.post.as_deref()) {
        return Ok(single(MessageType::POST, post));
    }
    if let Some(chat_id) = non_empty(input.share_chat_id.as_deref()) {
        return single_json(
            MessageType::SHARE_CHAT,
            serde_json::json!({ "chat_id": chat_id }),
        );
    }
    if let Some(user_id) = non_empty(input.share_user_id.as_deref()) {
        return single_json(
            MessageType::SHARE_USER,
            serde_json::json!({ "user_id": user_id }),
        );
    }
    if let Some(file_key) = non_empty(input.sticker_file_key.as_deref()) {
        return single_json(
            MessageType::STICKER,
            serde_json::json!({ "file_key": file_key }),
        );
    }
    if let Some(markdown) = non_empty(input.markdown.as_deref()) {
        let mut chunks = Vec::new();
        for (idx, chunk) in split_markdown(markdown, DEFAULT_TEXT_CHUNK_LIMIT)
            .iter()
            .enumerate()
        {
            let mentions = if idx == 0 {
                input.mentions.as_slice()
            } else {
                &[]
            };
            chunks.push((
                MessageType::POST.to_string(),
                markdown_to_post(input.title.as_deref().unwrap_or_default(), chunk, mentions)?,
            ));
        }
        return Ok(SendPayloads::Chunks(chunks));
    }
    if let Some(text) = non_empty(input.text.as_deref()) {
        let full_text = format!("{}{}", compose_mentions_text_prefix(&input.mentions), text);
        let mut chunks = Vec::new();
        for chunk in super::stream::split_text(&full_text, DEFAULT_TEXT_CHUNK_LIMIT) {
            chunks.push((MessageType::TEXT.to_string(), text_content(&chunk)?));
        }
        return Ok(SendPayloads::Chunks(chunks));
    }
    Err(LarkError::IllegalParam(
        "no valid channel message content provided".into(),
    ))
}

fn single(msg_type: &str, content: &str) -> SendPayloads {
    SendPayloads::Single {
        msg_type: msg_type.to_string(),
        content: content.to_string(),
    }
}

fn single_json(msg_type: &str, content: serde_json::Value) -> Result<SendPayloads, LarkError> {
    Ok(SendPayloads::Single {
        msg_type: msg_type.to_string(),
        content: serde_json::to_string(&content)?,
    })
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.filter(|value| !value.is_empty())
}

fn file_name_from_path(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("upload.bin")
        .to_string()
}

fn is_reply_target_gone(err: &LarkError) -> bool {
    matches!(
        err,
        LarkError::Api(code)
            if matches!(code.code, 230020 | 230017 | 230011 | 230040)
    )
}

fn is_format_error(err: &LarkError) -> bool {
    matches!(err, LarkError::Api(code) if code.code == 230001)
        || err.to_string().to_ascii_lowercase().contains("format")
}
