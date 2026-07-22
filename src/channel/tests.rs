use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::event::{CardActionTriggerResponse, EventDispatcher, EventReq};
use crate::events::common::UserId;
use crate::events::im::{Mention, Message, MessageSender, P2MessageReceiveV1};
use crate::service::im::v1::MessageType;
use crate::{LarkClient, RequestOption};

use super::handler::{ChannelHandlers, emit_message_event, register_channel_handlers};
use super::identity::BotIdentityCache;
use super::state::ChannelState;
use super::*;

#[test]
fn text_content_uses_sdk_error_surface() {
    let content: Result<_, crate::LarkError> = text_content("hello");

    assert_eq!(content.unwrap(), r#"{"text":"hello"}"#);
}

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app_id", "app_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

async fn counting_json_server(
    body: &'static str,
) -> (
    std::net::SocketAddr,
    Arc<AtomicUsize>,
    tokio::task::JoinHandle<()>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let handle = tokio::spawn({
        let calls = Arc::clone(&calls);
        async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let calls = Arc::clone(&calls);
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};

                    let mut buf = [0; 1024];
                    if stream.read(&mut buf).await.is_err() {
                        return;
                    }
                    calls.fetch_add(1, Ordering::SeqCst);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.shutdown().await;
                });
            }
        }
    });

    (addr, calls, handle)
}

async fn mock_json_server_with_requests(
    responses: Vec<&'static str>,
) -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<()>,
    Arc<Mutex<Vec<String>>>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let responses = Arc::new(responses);
    let counter = Arc::new(AtomicUsize::new(0));
    let requests = Arc::new(Mutex::new(Vec::new()));

    let handle = tokio::spawn({
        let requests = requests.clone();
        async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let responses = responses.clone();
                let counter = counter.clone();
                let requests = requests.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};

                    let mut buf = vec![0u8; 131_072];
                    let Ok(n) = stream.read(&mut buf).await else {
                        return;
                    };
                    requests
                        .lock()
                        .unwrap()
                        .push(String::from_utf8_lossy(&buf[..n]).to_string());
                    let idx = counter.fetch_add(1, Ordering::SeqCst);
                    let body = responses[idx.min(responses.len() - 1)];
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.shutdown().await;
                });
            }
        }
    });

    (addr, handle, requests)
}

fn message_event(message_id: &str, message_type: &str, content: &str) -> P2MessageReceiveV1 {
    P2MessageReceiveV1 {
        sender: MessageSender {
            sender_id: Some(UserId {
                open_id: Some("ou_user".into()),
                user_id: Some("user_1".into()),
                ..Default::default()
            }),
            sender_type: "user".into(),
            tenant_key: "tenant".into(),
        },
        message: Message {
            message_id: message_id.into(),
            chat_id: "oc_group".into(),
            chat_type: "group".into(),
            message_type: message_type.into(),
            content: content.into(),
            create_time: now_ms().to_string(),
            ..Default::default()
        },
    }
}

fn dm_message_event(message_id: &str) -> P2MessageReceiveV1 {
    let mut event = message_event(message_id, MessageType::TEXT, r#"{"text":"hello"}"#);
    event.message.chat_type = "p2p".into();
    event
}

fn mentioned_group_event(message_id: &str) -> P2MessageReceiveV1 {
    let mut event = message_event(message_id, MessageType::TEXT, r#"{"text":"hello"}"#);
    event.message.mentions.push(Mention {
        key: "@_user_1".into(),
        id: Some(UserId {
            open_id: Some("ou_bot".into()),
            user_id: Some("bot_user".into()),
            ..Default::default()
        }),
        mentioned_type: "user".into(),
        name: "bot".into(),
        tenant_key: "tenant".into(),
    });
    event
}

#[test]
fn normalizes_text_message_content() {
    let event = message_event("om_1", MessageType::TEXT, r#"{"text":"hello"}"#);
    let message = NormalizedMessage::from_event(event);
    assert_eq!(message.text.as_deref(), Some("hello"));
}

#[test]
fn normalizes_go_channel_content_cases() {
    let cases = [
        (
            MessageType::TEXT,
            r#"{"text":"hello"}"#,
            "hello",
            Vec::<ChannelResource>::new(),
        ),
        (
            MessageType::IMAGE,
            r#"{"image_key":"img_123"}"#,
            "![image](img_123)",
            vec![ChannelResource::new("image", "img_123")],
        ),
        (
            MessageType::FILE,
            r#"{"file_key":"file_123", "file_name":"test.txt"}"#,
            r#"<file key="file_123" name="test.txt"/>"#,
            vec![ChannelResource {
                resource_type: "file".into(),
                file_key: "file_123".into(),
                file_name: "test.txt".into(),
                duration_ms: None,
                cover_image_key: String::new(),
            }],
        ),
        (
            "folder",
            r#"{"file_key":"folder_123", "file_name":"my_folder"}"#,
            r#"<folder key="folder_123" name="my_folder"/>"#,
            Vec::new(),
        ),
        (
            MessageType::AUDIO,
            r#"{"file_key":"audio_123", "duration": 120000}"#,
            r#"<audio key="audio_123" duration="2:00"/>"#,
            vec![ChannelResource {
                resource_type: "audio".into(),
                file_key: "audio_123".into(),
                duration_ms: Some(120000),
                ..Default::default()
            }],
        ),
        (
            MessageType::MEDIA,
            r#"{"file_key":"video_123", "file_name":"vid.mp4", "duration": 65000, "image_key":"cover_1"}"#,
            r#"<video key="video_123" name="vid.mp4" duration="1:05"/>"#,
            vec![ChannelResource {
                resource_type: "video".into(),
                file_key: "video_123".into(),
                file_name: "vid.mp4".into(),
                duration_ms: Some(65000),
                cover_image_key: "cover_1".into(),
            }],
        ),
        (
            MessageType::STICKER,
            r#"{"file_key":"sticker_123"}"#,
            r#"<sticker key="sticker_123"/>"#,
            vec![ChannelResource::new("sticker", "sticker_123")],
        ),
        (
            "hongbao",
            r#"{"text":"Happy New Year"}"#,
            r#"<hongbao text="Happy New Year"/>"#,
            Vec::new(),
        ),
        (
            "location",
            r#"{"name":"Beijing", "latitude":"39.9042", "longitude":"116.4074"}"#,
            r#"<location name="Beijing" coords="lat:39.9042,lng:116.4074"/>"#,
            Vec::new(),
        ),
        (
            MessageType::SHARE_CHAT,
            r#"{"chat_id":"chat_123"}"#,
            r#"<group_card id="chat_123"/>"#,
            Vec::new(),
        ),
        (
            MessageType::SHARE_USER,
            r#"{"user_id":"user_123"}"#,
            r#"<contact_card id="user_123"/>"#,
            Vec::new(),
        ),
        (
            "system",
            r#"{"template":"Hello {name}, welcome to {place}", "name":"Alice", "place":"Wonderland"}"#,
            "Hello Alice, welcome to Wonderland",
            Vec::new(),
        ),
        (
            "vote",
            r#"{"topic":"What's for lunch?", "options":["Pizza", "Burger"]}"#,
            "<vote>\nWhat's for lunch?\n• Pizza\n• Burger\n</vote>",
            Vec::new(),
        ),
        (
            "video_chat",
            r#"{"topic":"Daily Sync", "start_time":"1609459200000"}"#,
            "<meeting>\n📹 Daily Sync\n🕙 2021-01-01 08:00:00\n</meeting>",
            Vec::new(),
        ),
        (
            "calendar",
            r#"{"summary":"Meeting", "start_time":"1609459200000", "end_time":"1609462800000"}"#,
            "<calendar_invite>\n📅 Meeting\n🕙 2021-01-01 08:00:00 ~ 2021-01-01 09:00:00\n</calendar_invite>",
            Vec::new(),
        ),
        (
            "todo",
            r#"{"summary":{"title":"Buy milk", "content":[[{"tag":"text","text":"From the store"}]]}, "due_time":"1609459200000"}"#,
            "<todo>\nBuy milk\nFrom the store\nDue: 2021-01-01 08:00:00\n</todo>",
            Vec::new(),
        ),
        (
            MessageType::INTERACTIVE,
            r#"{"config":{"wide_screen_mode":true}}"#,
            "[interactive card]",
            Vec::new(),
        ),
    ];

    for (message_type, content, expected_text, expected_resources) in cases {
        let message =
            NormalizedMessage::from_event(message_event("om_norm", message_type, content));
        assert_eq!(
            message.text.as_deref(),
            Some(expected_text),
            "{message_type}"
        );
        assert_eq!(message.resources, expected_resources, "{message_type}");
        assert_eq!(message.content, content);
    }
}

#[test]
fn normalizes_post_content_v2_and_rich_text_fallback() {
    let post = NormalizedMessage::from_event(message_event(
        "om_post",
        MessageType::POST,
        r#"{"zh_cn": {"title":"Post Title", "content":[[{"tag":"text","text":"Hello "}, {"tag":"a", "text":"Link", "href":"https://example.com"}], [{"tag":"at", "user_id":"all"}], [{"tag":"img", "image_key":"img_abc"}]]}}"#,
    ));
    assert_eq!(
        post.text.as_deref(),
        Some("**Post Title**\n\nHello [Link](https://example.com)\n@all\n![image](img_abc)")
    );
    assert_eq!(
        post.resources,
        vec![ChannelResource::new("image", "img_abc")]
    );

    let content_v2 = NormalizedMessage::from_event(message_event(
        "om_v2",
        MessageType::POST,
        r#"{"zh_cn":{"content_v2":[[{"tag":"md","text":"<at user_id=\"ou_123\">小明</at> hello ![image](img_key_123)"}]]}}"#,
    ));
    assert_eq!(
        content_v2.text.as_deref(),
        Some("@小明 hello ![image](img_key_123)")
    );
    assert_eq!(
        content_v2.resources,
        vec![ChannelResource::new("image", "img_key_123")]
    );

    let protected = NormalizedMessage::from_event(message_event(
        "om_code",
        MessageType::POST,
        "{\"zh_cn\":{\"content_v2\":[[{\"tag\":\"md\",\"text\":\"Before\\n```go\\n<at user_id=\\\"ou_1\\\">name</at>\\n![image](k)\\n```\\nAfter\"}]]}}",
    ));
    assert_eq!(
        protected.text.as_deref(),
        Some("Before\n```go\n<at user_id=\"ou_1\">name</at>\n![image](k)\n```\nAfter")
    );
    assert!(protected.resources.is_empty());
}

#[test]
fn rejects_duplicate_messages() {
    let state = ChannelState::new(
        ChannelPolicy::default().require_mention(false),
        BotIdentityCacheConfig::default(),
    );
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
    let state = ChannelState::new(
        ChannelPolicy::default().require_mention(false),
        BotIdentityCacheConfig::default(),
    );
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
fn group_policy_requires_bot_mention_by_default() {
    let state = ChannelState::new(ChannelPolicy::default(), BotIdentityCacheConfig::default());
    state.remember_bot_identity(BotIdentity {
        open_id: Some("ou_bot".into()),
        ..Default::default()
    });
    let result = state.accept_message(message_event("om_no_mention", MessageType::TEXT, "{}"));
    assert!(matches!(
        result,
        ChannelDecision::Rejected(RejectEvent {
            reason: RejectReason::NoMention,
            ..
        })
    ));
    let result = state.accept_message(mentioned_group_event("om_mention"));
    assert!(matches!(result, ChannelDecision::Accepted(_)));
}

#[test]
fn mention_all_is_rejected_unless_enabled() {
    let mut event = message_event("om_all", MessageType::TEXT, "{}");
    event.message.mentions.push(Mention {
        key: "@all".into(),
        mentioned_type: "all".into(),
        ..Default::default()
    });
    let state = ChannelState::new(ChannelPolicy::default(), BotIdentityCacheConfig::default());
    assert!(matches!(
        state.accept_message(event.clone()),
        ChannelDecision::Rejected(RejectEvent {
            reason: RejectReason::MentionAll,
            ..
        })
    ));

    let state = ChannelState::new(
        ChannelPolicy::default().respond_to_mention_all(true),
        BotIdentityCacheConfig::default(),
    );
    assert!(matches!(
        state.accept_message(event),
        ChannelDecision::Accepted(_)
    ));
}

#[test]
fn group_and_dm_policy_rejections_are_typed() {
    let state = ChannelState::new(
        ChannelPolicy::default()
            .require_mention(false)
            .allow_group("oc_allowed"),
        BotIdentityCacheConfig::default(),
    );
    assert!(matches!(
        state.accept_message(message_event("om_group", MessageType::TEXT, "{}")),
        ChannelDecision::Rejected(RejectEvent {
            reason: RejectReason::GroupNotAllowed,
            ..
        })
    ));

    let state = ChannelState::new(
        ChannelPolicy::default().dm_mode(DmMode::Disabled),
        BotIdentityCacheConfig::default(),
    );
    assert!(matches!(
        state.accept_message(dm_message_event("om_dm")),
        ChannelDecision::Rejected(RejectEvent {
            reason: RejectReason::DmDisabled,
            ..
        })
    ));

    let state = ChannelState::new(
        ChannelPolicy::default().dm_mode(DmMode::Allowlist),
        BotIdentityCacheConfig::default(),
    );
    assert!(matches!(
        state.accept_message(dm_message_event("om_dm_allow")),
        ChannelDecision::Rejected(RejectEvent {
            reason: RejectReason::SenderNotAllowed,
            ..
        })
    ));
}

#[test]
fn runtime_policy_update_changes_acceptance() {
    let state = ChannelState::new(ChannelPolicy::default(), BotIdentityCacheConfig::default());
    assert!(matches!(
        state.accept_message(message_event("om_before", MessageType::TEXT, "{}")),
        ChannelDecision::Rejected(RejectEvent {
            reason: RejectReason::NoMention,
            ..
        })
    ));
    state.update_policy(ChannelPolicy::default().require_mention(false));
    assert!(matches!(
        state.accept_message(message_event("om_after", MessageType::TEXT, "{}")),
        ChannelDecision::Accepted(_)
    ));
    assert!(!state.policy().require_mention);
}

#[test]
fn bot_identity_cache_normalizes_and_tracks_freshness() {
    let config = BotIdentityCacheConfig {
        ttl: Duration::from_millis(1),
        min_refresh_interval: Duration::from_millis(1),
    }
    .normalized();
    assert_eq!(config.min_refresh_interval, Duration::from_secs(30));

    let mut cache = BotIdentityCache::new(BotIdentityCacheConfig {
        ttl: Duration::from_secs(60),
        min_refresh_interval: Duration::from_secs(30),
    });
    let identity = BotIdentity {
        open_id: Some("ou_bot".into()),
        ..Default::default()
    };
    cache.remember(identity.clone());
    assert_eq!(cache.fresh(), Some(identity.clone()));
    cache.last_failure_at = Some(Instant::now());
    assert_eq!(cache.throttled_stale(), Some(identity));
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
async fn channel_send_text_detects_receive_id_and_mentions() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_sent","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let result = channel
        .send(
            &SendInput {
                receive_id: Some("ou_user".into()),
                text: Some("hello".into()),
                mentions: vec![ChannelMention {
                    id: Some(UserId {
                        user_id: Some("user_1".into()),
                        ..Default::default()
                    }),
                    name: "Alice".into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.message_id, "om_sent");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains("POST /open-apis/im/v1/messages?receive_id_type=open_id"));
    assert!(request_dump.contains(r#""receive_id":"ou_user""#));
    assert!(request_dump.contains(r#""msg_type":"text""#));
    assert!(request_dump.contains("user_1"));
    assert!(request_dump.contains("Alice"));
    assert!(request_dump.contains("hello"));
}

#[tokio::test]
async fn channel_send_user_id_uses_user_id_receive_type() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_user","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let result = channel
        .send(
            &SendInput {
                user_id: Some("user_123".into()),
                text: Some("hello".into()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.message_id, "om_user");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains("POST /open-apis/im/v1/messages?receive_id_type=user_id"));
    assert!(request_dump.contains(r#""receive_id":"user_123""#));
}

#[tokio::test]
async fn channel_send_markdown_uses_post_content() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_post","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let result = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                markdown: Some("**hello**".into()),
                title: Some("Title".into()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.message_id, "om_post");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains("receive_id_type=chat_id"));
    assert!(request_dump.contains(r#""msg_type":"post""#));
    assert!(request_dump.contains(r#"\"tag\":\"md\""#));
    assert!(request_dump.contains("**hello**"));
}

#[tokio::test]
async fn channel_send_direct_image_and_file_keys() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_image","chat_id":"oc_group"}}"#,
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_file","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let image = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                image_key: Some("img_direct".into()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let file = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                file_key: Some("file_direct".into()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(image.message_id, "om_image");
    assert_eq!(file.message_id, "om_file");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains(r#""msg_type":"image""#));
    assert!(request_dump.contains(r#"\"image_key\":\"img_direct\""#));
    assert!(request_dump.contains(r#""msg_type":"file""#));
    assert!(request_dump.contains(r#"\"file_key\":\"file_direct\""#));
}

#[tokio::test]
async fn channel_send_image_path_uploads_then_sends_image_key() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"image_key":"img_uploaded"}}"#,
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_image","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();
    let path = std::env::temp_dir().join(format!("lark-channel-{}.png", now_ms()));
    std::fs::write(&path, b"image-bytes").unwrap();

    let result = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                image_path: Some(path.to_string_lossy().to_string()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();
    std::fs::remove_file(&path).unwrap();

    assert_eq!(result.message_id, "om_image");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains("POST /open-apis/im/v1/images"));
    assert!(request_dump.contains("POST /open-apis/im/v1/messages?receive_id_type=chat_id"));
    assert!(request_dump.contains(r#""msg_type":"image""#));
    assert!(request_dump.contains(r#"\"image_key\":\"img_uploaded\""#));
}

#[tokio::test]
async fn channel_send_file_path_uploads_then_sends_file_key() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"file_key":"file_uploaded"}}"#,
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_file","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();
    let path = std::env::temp_dir().join(format!("lark-channel-{}.txt", now_ms()));
    std::fs::write(&path, b"file-bytes").unwrap();

    let result = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                file_path: Some(path.to_string_lossy().to_string()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();
    std::fs::remove_file(&path).unwrap();

    assert_eq!(result.message_id, "om_file");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains("POST /open-apis/im/v1/files"));
    assert!(request_dump.contains("POST /open-apis/im/v1/messages?receive_id_type=chat_id"));
    assert!(request_dump.contains(r#""msg_type":"file""#));
    assert!(request_dump.contains(r#"\"file_key\":\"file_uploaded\""#));
}

#[tokio::test]
async fn channel_send_audio_source_bytes_infers_duration() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"file_key":"audio_uploaded"}}"#,
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_audio","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let result = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                media: Some(UploadInput {
                    kind: Some(MediaKind::Audio),
                    source_bytes: Some(ogg_duration_bytes(48_000)),
                    file_name: Some("voice.opus".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.message_id, "om_audio");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains("POST /open-apis/im/v1/files"));
    assert!(request_dump.contains(r#"name="duration""#));
    assert!(request_dump.contains("\r\n\r\n1000\r\n"));
    assert!(request_dump.contains(r#""msg_type":"audio""#));
    assert!(request_dump.contains(r#"\"file_key\":\"audio_uploaded\""#));
}

#[tokio::test]
async fn channel_send_audio_zero_duration_infers_duration() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"file_key":"audio_uploaded"}}"#,
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_audio","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let result = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                media: Some(UploadInput {
                    kind: Some(MediaKind::Audio),
                    source_bytes: Some(ogg_duration_bytes(48_000)),
                    file_name: Some("voice.opus".into()),
                    duration: Some(0),
                    ..Default::default()
                }),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.message_id, "om_audio");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains(r#"name="duration""#));
    assert!(request_dump.contains("\r\n\r\n1000\r\n"));
}

#[tokio::test]
async fn channel_send_source_url_to_private_host_is_rejected() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"file_key":"unused"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let err = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                media: Some(UploadInput {
                    kind: Some(MediaKind::File),
                    source_url: Some(format!("http://{addr}/file.bin")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap_err();

    assert!(err.to_string().contains("blocked address"));
    assert!(requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn channel_send_empty_input_is_rejected_without_request() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"message_id":"unused"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let err = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap_err();

    assert!(err.to_string().contains("no valid channel message content"));
    assert!(requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn channel_send_text_chunks_long_messages() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_chunk_1","chat_id":"oc_group"}}"#,
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_chunk_2","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();
    let text = format!("{}b", "a".repeat(20_000));

    let result = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                text: Some(text),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.message_id, "om_chunk_1");
    assert_eq!(result.chunk_ids, vec!["om_chunk_1", "om_chunk_2"]);
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(requests[0].contains(r#""msg_type":"text""#));
    assert!(requests[1].contains(r#""text\":\"b\""#));
}

#[tokio::test]
async fn channel_send_retries_format_error_as_text() {
    let (addr, _handle, requests) = mock_json_server_with_requests(vec![
        r#"{"code":230001,"msg":"format error"}"#,
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_text","chat_id":"oc_group"}}"#,
    ])
    .await;
    let client = client_for(addr);
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let result = channel
        .send(
            &SendInput {
                chat_id: Some("oc_group".into()),
                markdown: Some("fallback **text**".into()),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(result.message_id, "om_text");
    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains(r#""msg_type":"post""#));
    assert!(request_dump.contains(r#""msg_type":"text""#));
    assert!(request_dump.contains("fallback **text**"));
}

#[tokio::test]
async fn typed_handlers_and_observer_receive_events() {
    let (addr, _calls, handle) = counting_json_server(
        r#"{"code":0,"msg":"success","bot":{"open_id":"ou_other","app_name":"Bot"}}"#,
    )
    .await;
    let client = client_for(addr);
    let message_called = Arc::new(Mutex::new(false));
    let observed_called = Arc::new(Mutex::new(false));

    let message_flag = message_called.clone();
    let observed_flag = observed_called.clone();
    let handlers = Arc::new(ChannelHandlers {
        messages: vec![Arc::new(move |_| {
            let message_flag = message_flag.clone();
            Box::pin(async move {
                *message_flag.lock().unwrap() = true;
                Ok(())
            })
        })],
        observers: vec![Arc::new(move |_| {
            let observed_flag = observed_flag.clone();
            Box::pin(async move {
                *observed_flag.lock().unwrap() = true;
                Ok(())
            })
        })],
        ..Default::default()
    });
    let state = Arc::new(ChannelState::new(
        ChannelPolicy::default().require_mention(false),
        BotIdentityCacheConfig::default(),
    ));
    emit_message_event(
        client,
        state,
        handlers,
        message_event("om_handlers", MessageType::TEXT, "{}"),
    )
    .await
    .unwrap();
    handle.abort();
    assert!(*message_called.lock().unwrap());
    assert!(*observed_called.lock().unwrap());
}

#[tokio::test]
async fn message_dispatch_fetches_bot_identity_before_policy() {
    let (addr, calls, handle) = counting_json_server(
        r#"{"code":0,"msg":"success","bot":{"open_id":"ou_bot","app_name":"Bot"}}"#,
    )
    .await;
    let client = client_for(addr);
    let message_called = Arc::new(Mutex::new(false));
    let message_flag = message_called.clone();
    let handlers = Arc::new(ChannelHandlers {
        messages: vec![Arc::new(move |message| {
            let message_flag = message_flag.clone();
            Box::pin(async move {
                assert!(message.mentioned_bot);
                *message_flag.lock().unwrap() = true;
                Ok(())
            })
        })],
        ..Default::default()
    });
    let state = Arc::new(ChannelState::new(
        ChannelPolicy::default(),
        BotIdentityCacheConfig::default(),
    ));

    emit_message_event(
        client,
        state,
        handlers,
        mentioned_group_event("om_prefetch"),
    )
    .await
    .unwrap();
    handle.abort();

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(*message_called.lock().unwrap());
}

#[tokio::test]
async fn bot_identity_refresh_is_single_flight() {
    let (addr, calls, handle) = counting_json_server(
        r#"{"code":0,"msg":"success","bot":{"open_id":"ou_bot","app_name":"Bot"}}"#,
    )
    .await;
    let client = client_for(addr);
    let state = Arc::new(ChannelState::new(
        ChannelPolicy::default(),
        BotIdentityCacheConfig::default(),
    ));

    let mut handles = Vec::new();
    for _ in 0..8 {
        let client = client.clone();
        let state = state.clone();
        handles.push(tokio::spawn(async move {
            state
                .get_bot_identity(&client, &RequestOption::default())
                .await
                .unwrap()
        }));
    }

    for handle in handles {
        let identity = handle.await.unwrap();
        assert_eq!(identity.open_id.as_deref(), Some("ou_bot"));
    }
    handle.abort();

    assert_eq!(calls.load(Ordering::SeqCst), 1);
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
    let handlers = Arc::new(ChannelHandlers {
        card_actions: vec![Arc::new(move |_| {
            let channel_flag = channel_flag.clone();
            Box::pin(async move {
                *channel_flag.lock().unwrap() = true;
                Ok(())
            })
        })],
        ..Default::default()
    });

    let dispatcher = register_channel_handlers(
        dispatcher,
        LarkClient::builder("app_id", "app_secret")
            .disable_token_cache()
            .build()
            .unwrap(),
        Arc::new(ChannelState::new(
            ChannelPolicy::default(),
            BotIdentityCacheConfig::default(),
        )),
        handlers,
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

fn ogg_duration_bytes(granule: i64) -> Vec<u8> {
    let mut data = vec![0u8; 64];
    data[10..14].copy_from_slice(b"OggS");
    data[16..24].copy_from_slice(&granule.to_le_bytes());
    data
}
