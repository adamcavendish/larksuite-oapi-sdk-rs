use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use larksuite_oapi_sdk_rs::event::{
    CardAction, CardActionHandler, EventDispatcher, EventReq, EventResp,
};

// ── EventResp ──

#[test]
fn event_resp_success_has_json_content_type() {
    let resp = EventResp::success(serde_json::json!({"ok": true}));
    assert_eq!(resp.status_code, 200);
    assert_eq!(
        resp.headers.get("Content-Type").unwrap(),
        "application/json"
    );
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["ok"], true);
}

#[test]
fn event_resp_error_has_msg() {
    let resp = EventResp::error(500, "internal error");
    assert_eq!(resp.status_code, 500);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["msg"], "internal error");
}

// ── EventDispatcher: url_verification with wrong token ──

#[tokio::test]
async fn url_verification_wrong_token_rejected() {
    let dispatcher = EventDispatcher::new("correct_token", "");
    let body = serde_json::json!({
        "type": "url_verification",
        "token": "wrong_token",
        "challenge": "test"
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

#[tokio::test]
async fn url_verification_empty_token_accepts_any() {
    let dispatcher = EventDispatcher::new("", "");
    let body = serde_json::json!({
        "type": "url_verification",
        "token": "anything",
        "challenge": "abc"
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["challenge"], "abc");
}

// ── EventDispatcher: encrypted body ──

#[tokio::test]
async fn encrypted_url_verification() {
    use base64::Engine as _;
    use sha2::{Digest, Sha256};

    let encrypt_key = "test_encrypt_key";
    let plaintext = serde_json::json!({
        "type": "url_verification",
        "token": "",
        "challenge": "encrypted_challenge"
    });
    let plaintext_str = serde_json::to_string(&plaintext).unwrap();

    let key_bytes: [u8; 32] = {
        let mut h = Sha256::new();
        h.update(encrypt_key.as_bytes());
        h.finalize().into()
    };

    let iv: [u8; 16] = [1; 16];
    let ciphertext = {
        use aes::cipher::{BlockModeEncrypt, KeyIvInit};
        type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

        let pad_len = 16 - (plaintext_str.len() % 16);
        let mut buf = plaintext_str.as_bytes().to_vec();
        buf.extend(std::iter::repeat_n(pad_len as u8, pad_len));
        let buf_len = buf.len();
        let enc = Aes256CbcEnc::new_from_slices(&key_bytes, &iv).unwrap();
        enc.encrypt_padded::<aes::cipher::block_padding::NoPadding>(&mut buf, buf_len)
            .unwrap()
            .to_vec()
    };

    let mut combined = iv.to_vec();
    combined.extend_from_slice(&ciphertext);
    let encoded = base64::engine::general_purpose::STANDARD.encode(&combined);

    let encrypted_body = serde_json::json!({ "encrypt": encoded });

    let dispatcher = EventDispatcher::new("", encrypt_key);
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&encrypted_body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["challenge"], "encrypted_challenge");
}

// ── EventDispatcher: callback handler ──

#[tokio::test]
async fn callback_handler_returns_custom_response() {
    let dispatcher = EventDispatcher::new("", "").on_callback("card.action", |_val| async {
        Ok(serde_json::json!({"toast": {"type": "success"}}))
    });

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev1",
            "event_type": "card.action",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {"action": "click"}
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["toast"]["type"], "success");
}

// ── EventDispatcher: handler error returns 500 ──

#[tokio::test]
async fn event_handler_error_returns_500() {
    let dispatcher = EventDispatcher::new("", "").on_event("test.event_v1", |_val| async {
        Err(larksuite_oapi_sdk_rs::error::Error::Event(
            "handler failed".to_string(),
        ))
    });

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev2",
            "event_type": "test.event_v1",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {}
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── EventDispatcher: invalid utf8 body ──

#[tokio::test]
async fn invalid_utf8_body_returns_500() {
    let dispatcher = EventDispatcher::new("", "");
    let req = EventReq {
        headers: Default::default(),
        body: vec![0xFF, 0xFE],
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── EventDispatcher: invalid json body ──

#[tokio::test]
async fn invalid_json_body_returns_500() {
    let dispatcher = EventDispatcher::new("", "");
    let req = EventReq {
        headers: Default::default(),
        body: b"not json".to_vec(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── EventDispatcher: skip_sign_verify ──

#[tokio::test]
async fn skip_sign_verify_bypasses_signature_check() {
    let received = Arc::new(Mutex::new(false));
    let received_clone = Arc::clone(&received);

    let dispatcher = EventDispatcher::new("", "some_encrypt_key")
        .skip_sign_verify()
        .on_event("test.event_v1", move |_val| {
            let r = Arc::clone(&received_clone);
            async move {
                *r.lock().unwrap() = true;
                Ok(())
            }
        });

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev3",
            "event_type": "test.event_v1",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {}
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    assert!(*received.lock().unwrap());
}

// ── CardActionHandler: url verification ──

#[tokio::test]
async fn card_handler_url_verification() {
    let handler = CardActionHandler::new("token123", "", |_action: CardAction| async {
        Ok(serde_json::json!({}))
    });

    let body = serde_json::json!({
        "type": "url_verification",
        "token": "token123",
        "challenge": "card_challenge"
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["challenge"], "card_challenge");
}

#[tokio::test]
async fn card_handler_wrong_verification_token() {
    let handler = CardActionHandler::new("correct", "", |_action: CardAction| async {
        Ok(serde_json::json!({}))
    });

    let body = serde_json::json!({
        "type": "url_verification",
        "token": "wrong",
        "challenge": "test"
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── CardActionHandler: action dispatch ──

#[tokio::test]
async fn card_handler_dispatches_action() {
    let received_action = Arc::new(Mutex::new(None::<serde_json::Value>));
    let received_clone = Arc::clone(&received_action);

    let handler = CardActionHandler::new("", "", move |action: CardAction| {
        let r = Arc::clone(&received_clone);
        async move {
            *r.lock().unwrap() = Some(action.action.clone());
            Ok(serde_json::json!({"status": "handled"}))
        }
    })
    .skip_sign_verify();

    let body = serde_json::json!({
        "open_id": "ou_123",
        "user_id": "u_456",
        "open_message_id": "om_789",
        "open_chat_id": "oc_abc",
        "tenant_key": "tk_1",
        "token": "",
        "action": {"tag": "button", "value": {"key": "val"}},
        "host": "",
        "delivery_type": ""
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 200);

    let action_val = received_action.lock().unwrap().clone().unwrap();
    assert_eq!(action_val["tag"], "button");
}

// ── EventDispatcher: SHA-256 signature verification ──

fn compute_sha256_sig(timestamp: &str, nonce: &str, key: &str, body: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut content = String::new();
    content.push_str(timestamp);
    content.push_str(nonce);
    content.push_str(key);
    content.push_str(&String::from_utf8_lossy(body));
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

fn compute_sha1_sig(timestamp: &str, nonce: &str, key: &str, body: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut content = String::new();
    content.push_str(timestamp);
    content.push_str(nonce);
    content.push_str(key);
    content.push_str(body);
    let mut hasher = Sha1::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

#[tokio::test]
async fn event_sha256_signature_valid() {
    let encrypt_key = "test_sign_key";
    let timestamp = "1620000000";
    let nonce = "abc123";

    let received = Arc::new(Mutex::new(false));
    let received_clone = Arc::clone(&received);

    let dispatcher =
        EventDispatcher::new("", encrypt_key).on_event("im.message.receive_v1", move |_val| {
            let r = Arc::clone(&received_clone);
            async move {
                *r.lock().unwrap() = true;
                Ok(())
            }
        });

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev_sig",
            "event_type": "im.message.receive_v1",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {}
    });
    let body_bytes = serde_json::to_vec(&body).unwrap();
    let sig = compute_sha256_sig(timestamp, nonce, encrypt_key, &body_bytes);

    let headers = HashMap::from([
        (
            "X-Lark-Request-Timestamp".to_string(),
            vec![timestamp.to_string()],
        ),
        ("X-Lark-Request-Nonce".to_string(), vec![nonce.to_string()]),
        ("X-Lark-Signature".to_string(), vec![sig]),
    ]);

    let req = EventReq {
        headers,
        body: body_bytes,
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    assert!(*received.lock().unwrap());
}

#[tokio::test]
async fn event_sha256_signature_invalid_returns_500() {
    let encrypt_key = "test_sign_key";

    let dispatcher =
        EventDispatcher::new("", encrypt_key).on_event("test.event_v1", |_val| async { Ok(()) });

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev_bad_sig",
            "event_type": "test.event_v1",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {}
    });
    let body_bytes = serde_json::to_vec(&body).unwrap();

    let headers = HashMap::from([
        (
            "X-Lark-Request-Timestamp".to_string(),
            vec!["1620000000".to_string()],
        ),
        (
            "X-Lark-Request-Nonce".to_string(),
            vec!["nonce".to_string()],
        ),
        (
            "X-Lark-Signature".to_string(),
            vec!["0000000000000000000000000000000000000000000000000000000000000000".to_string()],
        ),
    ]);

    let req = EventReq {
        headers,
        body: body_bytes,
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

#[tokio::test]
async fn event_missing_signature_header_returns_500() {
    let encrypt_key = "test_sign_key";

    let dispatcher =
        EventDispatcher::new("", encrypt_key).on_event("test.event_v1", |_val| async { Ok(()) });

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev_no_sig",
            "event_type": "test.event_v1",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {}
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── EventDispatcher: P1 protocol (no header, event type from event.type) ──

#[tokio::test]
async fn event_p1_protocol_dispatches_by_event_type() {
    let received = Arc::new(Mutex::new(false));
    let received_clone = Arc::clone(&received);

    let dispatcher = EventDispatcher::new("", "").on_event("app_open", move |_val| {
        let r = Arc::clone(&received_clone);
        async move {
            *r.lock().unwrap() = true;
            Ok(())
        }
    });

    let body = serde_json::json!({
        "event": {
            "type": "app_open",
            "app_id": "cli_test"
        }
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    assert!(*received.lock().unwrap());
}

// ── CardActionHandler: SHA-1 signature verification ──

#[tokio::test]
async fn card_sha1_signature_valid() {
    let encrypt_key = "card_sign_key";
    let timestamp = "1620000000";
    let nonce = "xyz789";

    let received = Arc::new(Mutex::new(false));
    let received_clone = Arc::clone(&received);

    let handler = CardActionHandler::new("", encrypt_key, move |_action: CardAction| {
        let r = Arc::clone(&received_clone);
        async move {
            *r.lock().unwrap() = true;
            Ok(serde_json::json!({}))
        }
    });

    let body = serde_json::json!({
        "open_id": "ou_1",
        "user_id": "u_1",
        "open_message_id": "om_1",
        "open_chat_id": "oc_1",
        "tenant_key": "tk",
        "token": "",
        "action": {},
        "host": "",
        "delivery_type": ""
    });
    let body_str = serde_json::to_string(&body).unwrap();
    let sig = compute_sha1_sig(timestamp, nonce, encrypt_key, &body_str);

    let headers = HashMap::from([
        (
            "X-Lark-Request-Timestamp".to_string(),
            vec![timestamp.to_string()],
        ),
        ("X-Lark-Request-Nonce".to_string(), vec![nonce.to_string()]),
        ("X-Lark-Signature".to_string(), vec![sig]),
    ]);

    let req = EventReq {
        headers,
        body: body_str.into_bytes(),
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 200);
    assert!(*received.lock().unwrap());
}

#[tokio::test]
async fn card_sha1_signature_invalid_returns_500() {
    let encrypt_key = "card_sign_key";

    let handler = CardActionHandler::new("", encrypt_key, |_action: CardAction| async {
        Ok(serde_json::json!({}))
    });

    let body = serde_json::json!({
        "open_id": "ou_1",
        "user_id": "u_1",
        "open_message_id": "om_1",
        "open_chat_id": "oc_1",
        "tenant_key": "tk",
        "token": "",
        "action": {},
        "host": "",
        "delivery_type": ""
    });
    let body_bytes = serde_json::to_vec(&body).unwrap();

    let headers = HashMap::from([
        (
            "X-Lark-Request-Timestamp".to_string(),
            vec!["1620000000".to_string()],
        ),
        (
            "X-Lark-Request-Nonce".to_string(),
            vec!["nonce".to_string()],
        ),
        (
            "X-Lark-Signature".to_string(),
            vec!["0000000000000000000000000000000000000000".to_string()],
        ),
    ]);

    let req = EventReq {
        headers,
        body: body_bytes,
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

#[tokio::test]
async fn card_missing_signature_header_returns_500() {
    let encrypt_key = "card_sign_key";

    let handler = CardActionHandler::new("", encrypt_key, |_action: CardAction| async {
        Ok(serde_json::json!({}))
    });

    let body = serde_json::json!({
        "open_id": "ou_1",
        "user_id": "u_1",
        "open_message_id": "om_1",
        "open_chat_id": "oc_1",
        "tenant_key": "tk",
        "token": "",
        "action": {},
        "host": "",
        "delivery_type": ""
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── EventReq: serde ──

#[test]
fn event_req_serde_roundtrip() {
    let req = EventReq {
        headers: HashMap::from([("X-Test".to_string(), vec!["val".to_string()])]),
        body: b"hello".to_vec(),
        request_uri: "/test".to_string(),
    };
    let json = serde_json::to_string(&req).unwrap();
    let deserialized: EventReq = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.request_uri, "/test");
    assert_eq!(deserialized.headers.get("X-Test").unwrap(), &vec!["val"]);
}

// ── EventDispatcher: with_auto_app_ticket ──

#[tokio::test]
async fn auto_app_ticket_stores_ticket() {
    let dispatcher = EventDispatcher::new("", "")
        .skip_sign_verify()
        .with_auto_app_ticket();

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev_ticket",
            "event_type": "app_ticket",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {
            "type": "app_ticket",
            "app_id": "test_app_123",
            "app_ticket": "ticket_value_abc"
        }
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
}

#[tokio::test]
async fn auto_app_ticket_empty_fields_noop() {
    let dispatcher = EventDispatcher::new("", "")
        .skip_sign_verify()
        .with_auto_app_ticket();

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev_ticket2",
            "event_type": "app_ticket",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {
            "type": "app_ticket",
            "app_id": "",
            "app_ticket": ""
        }
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
}

// ── EventDispatcher: no handler returns 200 with message ──

#[tokio::test]
async fn event_no_handler_returns_200() {
    let dispatcher = EventDispatcher::new("", "").skip_sign_verify();

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev_noop",
            "event_type": "unknown.event_v1",
            "app_id": "cli_test",
            "tenant_key": "t1",
            "create_time": "0"
        },
        "event": {}
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert!(parsed["msg"].as_str().unwrap().contains("no handler"));
}

// ── CardActionHandler: invalid utf8 body returns 500 ──

#[tokio::test]
async fn card_handler_invalid_utf8_returns_500() {
    let handler = CardActionHandler::new("", "", |_action: CardAction| async {
        Ok(serde_json::json!({}))
    });
    let req = EventReq {
        headers: Default::default(),
        body: vec![0xFF, 0xFE],
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── CardActionHandler: invalid json body returns 500 ──

#[tokio::test]
async fn card_handler_invalid_json_returns_500() {
    let handler = CardActionHandler::new("", "", |_action: CardAction| async {
        Ok(serde_json::json!({}))
    });
    let req = EventReq {
        headers: Default::default(),
        body: b"not json".to_vec(),
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── CardActionHandler: handler error returns 500 ──

#[tokio::test]
async fn card_handler_error_returns_500() {
    let handler = CardActionHandler::new("", "", |_action: CardAction| async {
        Err(larksuite_oapi_sdk_rs::error::Error::Event(
            "handler failed".to_string(),
        ))
    })
    .skip_sign_verify();

    let body = serde_json::json!({
        "open_id": "ou_1",
        "user_id": "u_1",
        "open_message_id": "om_1",
        "open_chat_id": "oc_1",
        "tenant_key": "tk",
        "token": "",
        "action": {},
        "host": "",
        "delivery_type": ""
    });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/card".to_string(),
    };
    let resp = handler.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── EventDispatcher: encrypted body with decrypt error ──

#[tokio::test]
async fn event_encrypted_body_decrypt_error_returns_500() {
    let dispatcher = EventDispatcher::new("", "some_key");
    let body = serde_json::json!({ "encrypt": "!!!invalid-base64!!!" });
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 500);
}

// ── EventDispatcher: on_customized_event ──

#[tokio::test]
async fn customized_event_handler_receives_full_req() {
    let captured_uri = Arc::new(Mutex::new(String::new()));
    let captured_header = Arc::new(Mutex::new(None));

    let uri_clone = captured_uri.clone();
    let header_clone = captured_header.clone();

    let dispatcher = EventDispatcher::new("", "")
        .skip_sign_verify()
        .on_customized_event(
            "im.message.receive_v1",
            move |req: EventReq, body: larksuite_oapi_sdk_rs::event::EventV2Body| {
                let uri = uri_clone.clone();
                let hdr = header_clone.clone();
                async move {
                    *uri.lock().unwrap() = req.request_uri.clone();
                    *hdr.lock().unwrap() = body.header;
                    Ok(())
                }
            },
        );

    let body = serde_json::json!({
        "schema": "2.0",
        "header": {
            "event_id": "ev_123",
            "event_type": "im.message.receive_v1",
            "app_id": "cli_test",
            "tenant_key": "tk",
            "create_time": "1234567890",
            "token": ""
        },
        "event": {
            "message": { "message_id": "om_abc" }
        }
    });

    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&body).unwrap(),
        request_uri: "/webhook/event".to_string(),
    };
    let resp = dispatcher.handle(req).await;
    assert_eq!(resp.status_code, 200);
    assert_eq!(&*captured_uri.lock().unwrap(), "/webhook/event");
    let hdr = captured_header.lock().unwrap();
    assert_eq!(hdr.as_ref().unwrap().event_id, "ev_123");
    assert_eq!(hdr.as_ref().unwrap().app_id, "cli_test");
}
