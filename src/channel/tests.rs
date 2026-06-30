use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::{io, thread};

use crate::event::{CardActionTriggerResponse, EventDispatcher, EventReq};
use crate::events::common::UserId;
use crate::events::im::{Mention, Message, MessageSender, P2MessageReceiveV1};
use crate::service::im::v1::MessageType;
use crate::{Client, RequestOption};

use super::handler::{ChannelHandlers, emit_message_event, register_channel_handlers};
use super::identity::BotIdentityCache;
use super::state::ChannelState;
use super::*;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("app_id", "app_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn counting_json_server(
    body: &'static str,
) -> (
    std::net::SocketAddr,
    Arc<AtomicUsize>,
    thread::JoinHandle<()>,
) {
    use std::io::{Read, Write};

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let addr = listener.local_addr().unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let server_calls = calls.clone();

    let handle = thread::spawn(move || {
        let started_at = Instant::now();
        let mut last_accept = None;
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    server_calls.fetch_add(1, Ordering::SeqCst);
                    last_accept = Some(Instant::now());
                    let _ = stream.set_read_timeout(Some(Duration::from_millis(50)));
                    let mut buf = [0; 1024];
                    let _ = stream.read(&mut buf);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(response.as_bytes());
                }
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                    if last_accept.is_some_and(|last| last.elapsed() > Duration::from_millis(200))
                        || started_at.elapsed() > Duration::from_secs(2)
                    {
                        break;
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });

    (addr, calls, handle)
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
async fn typed_handlers_and_observer_receive_events() {
    let (addr, _calls, handle) = counting_json_server(
        r#"{"code":0,"msg":"success","bot":{"open_id":"ou_other","app_name":"Bot"}}"#,
    );
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
    handle.join().unwrap();
    assert!(*message_called.lock().unwrap());
    assert!(*observed_called.lock().unwrap());
}

#[tokio::test]
async fn message_dispatch_fetches_bot_identity_before_policy() {
    let (addr, calls, handle) = counting_json_server(
        r#"{"code":0,"msg":"success","bot":{"open_id":"ou_bot","app_name":"Bot"}}"#,
    );
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
    handle.join().unwrap();

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(*message_called.lock().unwrap());
}

#[tokio::test]
async fn bot_identity_refresh_is_single_flight() {
    let (addr, calls, handle) = counting_json_server(
        r#"{"code":0,"msg":"success","bot":{"open_id":"ou_bot","app_name":"Bot"}}"#,
    );
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
    handle.join().unwrap();

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
        Client::builder("app_id", "app_secret")
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
