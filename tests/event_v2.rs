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

// ── EventDispatcher: with_config skip_sign_verify ──

#[tokio::test]
async fn skip_sign_verify_bypasses_signature_check() {
    use larksuite_oapi_sdk_rs::Config;

    let mut config = Config::new("app", "secret");
    config.skip_sign_verify = true;

    let received = Arc::new(Mutex::new(false));
    let received_clone = Arc::clone(&received);

    let dispatcher = EventDispatcher::new("", "some_encrypt_key")
        .with_config(config)
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
    use larksuite_oapi_sdk_rs::Config;

    let mut config = Config::new("app", "secret");
    config.skip_sign_verify = true;

    let received_action = Arc::new(Mutex::new(None::<serde_json::Value>));
    let received_clone = Arc::clone(&received_action);

    let handler = CardActionHandler::new("", "", move |action: CardAction| {
        let r = Arc::clone(&received_clone);
        async move {
            *r.lock().unwrap() = Some(action.action.clone());
            Ok(serde_json::json!({"status": "handled"}))
        }
    })
    .with_config(config);

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
