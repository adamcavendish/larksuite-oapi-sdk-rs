use larksuite_oapi_sdk_rs::{
    CardActionHandler, EventDispatcher, EventReq, WebhookCredentials, WebhookCredentialsError,
};
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicUsize, Ordering},
};

fn request(value: serde_json::Value) -> EventReq {
    EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&value).unwrap(),
        request_uri: "/webhook".into(),
    }
}
fn event(token: &str) -> serde_json::Value {
    serde_json::json!({"schema":"2.0","header":{"event_type":"audit.test","token":token},"event":{}})
}
fn credentials(token: &str, key: &str) -> WebhookCredentials {
    WebhookCredentials {
        verification_token: token.into(),
        encrypt_key: key.into(),
    }
}

#[tokio::test]
async fn snapshot_is_resolved_once_and_rotation_replaces_the_complete_pair() {
    let current = Arc::new(RwLock::new(credentials("first", "")));
    let calls = Arc::new(AtomicUsize::new(0));
    let read = current.clone();
    let count = calls.clone();
    let dispatcher = EventDispatcher::new("static", "static-key")
        .with_webhook_credentials_provider(Arc::new(move || {
            count.fetch_add(1, Ordering::SeqCst);
            Ok(read.read().unwrap().clone())
        }))
        .on_event("audit.test", |_| async { Ok(()) });
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        dispatcher.handle(request(event("first"))).await.status_code,
        200
    );
    *current.write().unwrap() = credentials("second", "second-key");
    let plain = serde_json::to_string(&event("irrelevant-encrypted-token")).unwrap();
    let cipher = larksuite_oapi_sdk_rs::crypto::event_encrypt("second-key", &plain).unwrap();
    let mut req = request(serde_json::json!({"encrypt":cipher}));
    use sha2::{Digest, Sha256};
    let signature = hex::encode(Sha256::digest(
        [b"timestamp".as_slice(), b"nonce", b"second-key", &req.body].concat(),
    ));
    req.headers
        .insert("X-Lark-Request-Timestamp".into(), vec!["timestamp".into()]);
    req.headers
        .insert("X-Lark-Request-Nonce".into(), vec!["nonce".into()]);
    req.headers
        .insert("X-Lark-Signature".into(), vec![signature]);
    assert_eq!(dispatcher.handle(req).await.status_code, 200);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn provider_failures_do_not_fall_back_or_invoke_handlers() {
    for panics in [false, true] {
        let handlers = Arc::new(AtomicUsize::new(0));
        let count = handlers.clone();
        let dispatcher = EventDispatcher::new("static", "")
            .with_webhook_credentials_provider(Arc::new(move || {
                if panics {
                    panic!("secret panic payload");
                }
                Err(WebhookCredentialsError)
            }))
            .on_event("audit.test", move |_| {
                count.fetch_add(1, Ordering::SeqCst);
                async { Ok(()) }
            });
        for req in [
            request(event("static")),
            request(
                serde_json::json!({"type":"url_verification","token":"static","challenge":"x"}),
            ),
            request(serde_json::json!(null)),
        ] {
            let response = dispatcher.handle(req).await;
            assert_eq!(response.status_code, 500);
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&response.body).unwrap()["msg"],
                "event error: webhook credentials provider failed"
            );
        }
        assert_eq!(handlers.load(Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn both_handlers_use_dynamic_challenge_credentials() {
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let provider = Arc::new(move || {
        count.fetch_add(1, Ordering::SeqCst);
        Ok(credentials("dynamic", ""))
    });
    let dispatcher =
        EventDispatcher::new("static", "").with_webhook_credentials_provider(provider.clone());
    let cards = CardActionHandler::new("static", "", |_| async { Ok(serde_json::json!({})) })
        .with_webhook_credentials_provider(provider);
    for token in ["dynamic", "static"] {
        let value = serde_json::json!({"type":"url_verification","token":token,"challenge":"x"});
        let expected = if token == "dynamic" { 200 } else { 500 };
        assert_eq!(
            dispatcher.handle(request(value.clone())).await.status_code,
            expected
        );
        assert_eq!(cards.handle(request(value)).await.status_code, expected);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 4);
}

#[test]
fn credentials_debug_is_redacted() {
    let debug = format!("{:?}", credentials("secret-token", "secret-key"));
    assert!(!debug.contains("secret-token"));
    assert!(!debug.contains("secret-key"));
}

#[test]
fn concurrent_requests_keep_their_owned_credential_snapshot() {
    use std::sync::Barrier;
    let current = Arc::new(RwLock::new(credentials("first", "first-key")));
    let captured = Arc::new(Barrier::new(2));
    let release = Arc::new(Barrier::new(2));
    let calls = Arc::new(AtomicUsize::new(0));
    let (read, first_captured, first_release, count) = (
        current.clone(),
        captured.clone(),
        release.clone(),
        calls.clone(),
    );
    let handlers = Arc::new(AtomicUsize::new(0));
    let handler_calls = handlers.clone();
    let dispatcher = Arc::new(
        EventDispatcher::new("static", "static-key")
            .with_webhook_credentials_provider(Arc::new(move || {
                let snapshot = read.read().unwrap().clone();
                if count.fetch_add(1, Ordering::SeqCst) == 0 {
                    first_captured.wait();
                    first_release.wait();
                }
                Ok(snapshot)
            }))
            .on_event("audit.test", move |_| {
                handler_calls.fetch_add(1, Ordering::SeqCst);
                async { Ok(()) }
            }),
    );
    fn signed(token: &str, key: &str) -> EventReq {
        use sha2::{Digest, Sha256};
        let cipher = larksuite_oapi_sdk_rs::crypto::event_encrypt(
            key,
            &serde_json::to_string(&event(token)).unwrap(),
        )
        .unwrap();
        let mut req = request(serde_json::json!({"encrypt":cipher}));
        let sig = hex::encode(Sha256::digest(
            [b"ts".as_slice(), b"nonce", key.as_bytes(), &req.body].concat(),
        ));
        req.headers
            .insert("X-Lark-Request-Timestamp".into(), vec!["ts".into()]);
        req.headers
            .insert("X-Lark-Request-Nonce".into(), vec!["nonce".into()]);
        req.headers.insert("X-Lark-Signature".into(), vec![sig]);
        req
    }
    let first = dispatcher.clone();
    let thread = std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(first.handle(signed("first", "first-key")))
    });
    captured.wait();
    *current.write().unwrap() = credentials("second", "second-key");
    let second = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(dispatcher.handle(signed("second", "second-key")));
    release.wait();
    assert_eq!(thread.join().unwrap().status_code, 200);
    assert_eq!(second.status_code, 200);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(handlers.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn legacy_card_signatures_use_snapshot_token_without_body_token() {
    use sha1::{Digest, Sha1};
    let handlers = Arc::new(AtomicUsize::new(0));
    let count = handlers.clone();
    let calls = Arc::new(AtomicUsize::new(0));
    let read = calls.clone();
    let cards = CardActionHandler::new("static", "", move |_| {
        count.fetch_add(1, Ordering::SeqCst);
        async { Ok(serde_json::json!({})) }
    })
    .with_webhook_credentials_provider(Arc::new(move || {
        read.fetch_add(1, Ordering::SeqCst);
        Ok(credentials("dynamic", ""))
    }));
    for key in ["dynamic", "static"] {
        let mut req = request(serde_json::json!({"action":{}}));
        let signature = hex::encode(Sha1::digest(
            [b"ts".as_slice(), b"nonce", key.as_bytes(), &req.body].concat(),
        ));
        req.headers
            .insert("X-Lark-Request-Timestamp".into(), vec!["ts".into()]);
        req.headers
            .insert("X-Lark-Request-Nonce".into(), vec!["nonce".into()]);
        req.headers
            .insert("X-Lark-Signature".into(), vec![signature]);
        assert_eq!(
            cards.handle(req).await.status_code,
            if key == "dynamic" { 200 } else { 500 }
        );
    }
    assert_eq!(handlers.load(Ordering::SeqCst), 1);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn empty_snapshots_preserve_compatibility_and_helpers_stay_static() {
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let dispatcher = EventDispatcher::new("static", "static-key")
        .with_webhook_credentials_provider(Arc::new(move || {
            count.fetch_add(1, Ordering::SeqCst);
            Ok(WebhookCredentials::default())
        }));
    assert_eq!(
        dispatcher
            .handle(request(event("anything")))
            .await
            .status_code,
        200
    );
    assert_eq!(
        dispatcher
            .handle(request(
                serde_json::json!({"type":"url_verification","challenge":"x"})
            ))
            .await
            .status_code,
        200
    );
    let cipher =
        larksuite_oapi_sdk_rs::crypto::event_encrypt("static-key", "static plaintext").unwrap();
    let req = request(serde_json::json!({"encrypt":cipher}));
    let extracted = dispatcher.parse_req(&req).unwrap();
    assert_eq!(
        dispatcher.decrypt_event(&extracted).unwrap(),
        "static plaintext"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn dynamic_v2_callbacks_resolve_one_snapshot() {
    use larksuite_oapi_sdk_rs::CardActionTriggerResponse;
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let dispatcher = EventDispatcher::new("static", "")
        .with_webhook_credentials_provider(Arc::new(move || {
            count.fetch_add(1, Ordering::SeqCst);
            Ok(credentials("dynamic", ""))
        }))
        .on_card_action_trigger(|_| async {
            Ok(CardActionTriggerResponse {
                toast: None,
                card: None,
            })
        });
    assert_eq!(dispatcher.handle(request(serde_json::json!({"schema":"2.0","header":{"event_type":"card.action.trigger","token":"dynamic"},"event":{}}))).await.status_code, 200);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn encrypted_challenges_validate_the_snapshot_token_for_both_handlers() {
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let provider = Arc::new(move || {
        count.fetch_add(1, Ordering::SeqCst);
        Ok(credentials("dynamic", "dynamic-key"))
    });
    let dispatcher = EventDispatcher::new("static", "static-key")
        .with_webhook_credentials_provider(provider.clone());
    let cards = CardActionHandler::new("static", "static-key", |_| async {
        Ok(serde_json::json!({}))
    })
    .with_webhook_credentials_provider(provider);
    for token in [
        None,
        Some(serde_json::json!("dynamic")),
        Some(serde_json::json!("static")),
        Some(serde_json::json!(123)),
    ] {
        let allowed = token.as_ref().and_then(|v| v.as_str()) == Some("dynamic");
        let mut plain = serde_json::json!({"type":"url_verification","challenge":"x"});
        if let Some(token) = token {
            plain["token"] = token;
        }
        let cipher = larksuite_oapi_sdk_rs::crypto::event_encrypt(
            "dynamic-key",
            &serde_json::to_string(&plain).unwrap(),
        )
        .unwrap();
        let value = serde_json::json!({"encrypt":cipher});
        assert_eq!(
            dispatcher.handle(request(value.clone())).await.status_code == 200,
            allowed
        );
        assert_eq!(
            cards.handle(request(value)).await.status_code == 200,
            allowed
        );
    }
    assert_eq!(calls.load(Ordering::SeqCst), 8);
}
