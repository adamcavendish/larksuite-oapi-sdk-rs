use larksuite_oapi_sdk_rs::crypto::{
    event_decrypt, event_encrypt, verify_signature_sha1, verify_signature_sha256,
};

#[test]
fn crate_root_crypto_reexports() {
    let _ = larksuite_oapi_sdk_rs::event_encrypt
        as fn(&str, &str) -> Result<String, larksuite_oapi_sdk_rs::LarkError>;
    let _ = larksuite_oapi_sdk_rs::event_decrypt
        as fn(&str, &str) -> Result<String, larksuite_oapi_sdk_rs::LarkError>;
    let _ =
        larksuite_oapi_sdk_rs::verify_signature_sha256 as fn(&str, &str, &str, &[u8], &str) -> bool;
    let _ =
        larksuite_oapi_sdk_rs::verify_signature_sha1 as fn(&str, &str, &str, &str, &str) -> bool;
}

#[test]
fn verify_signature_sha256_correct() {
    use sha2::{Digest, Sha256};
    let timestamp = "1620000000";
    let nonce = "abc";
    let key = "secret";
    let body = b"{\"type\":\"url_verification\"}";

    let mut content = String::new();
    content.push_str(timestamp);
    content.push_str(nonce);
    content.push_str(key);
    content.push_str(std::str::from_utf8(body).unwrap());

    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let expected = hex::encode(hasher.finalize());

    assert!(verify_signature_sha256(
        timestamp, nonce, key, body, &expected
    ));
}

#[test]
fn verify_signature_sha256_wrong_key() {
    use sha2::{Digest, Sha256};
    let timestamp = "1620000000";
    let nonce = "abc";
    let body = b"{}";

    let mut content = String::new();
    content.push_str(timestamp);
    content.push_str(nonce);
    content.push_str("correct_key");
    content.push_str("{}");

    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let expected = hex::encode(hasher.finalize());

    assert!(!verify_signature_sha256(
        timestamp,
        nonce,
        "wrong_key",
        body,
        &expected
    ));
}

#[test]
fn verify_signature_sha1_correct() {
    use sha1::{Digest, Sha1};
    let timestamp = "1620000000";
    let nonce = "xyz";
    let key = "mykey";
    let body = "hello";

    let mut content = String::new();
    content.push_str(timestamp);
    content.push_str(nonce);
    content.push_str(key);
    content.push_str(body);

    let mut hasher = Sha1::new();
    hasher.update(content.as_bytes());
    let expected = hex::encode(hasher.finalize());

    assert!(verify_signature_sha1(
        timestamp, nonce, key, body, &expected
    ));
}

#[test]
fn verify_signature_sha1_wrong_sig() {
    assert!(!verify_signature_sha1(
        "ts",
        "nonce",
        "key",
        "body",
        "0000000000000000000000000000000000000000"
    ));
}

fn aes256_cbc_encrypt(key_bytes: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    use aes::cipher::{BlockModeEncrypt, KeyIvInit};
    type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

    let pad_len = 16 - (plaintext.len() % 16);
    let mut buf = plaintext.to_vec();
    buf.extend(std::iter::repeat_n(pad_len as u8, pad_len));

    let buf_len = buf.len();
    let encryptor = Aes256CbcEnc::new_from_slices(key_bytes, iv).unwrap();
    encryptor
        .encrypt_padded::<aes::cipher::block_padding::NoPadding>(&mut buf, buf_len)
        .unwrap()
        .to_vec()
}

#[test]
fn event_decrypt_roundtrip() {
    use base64::Engine as _;
    use sha2::{Digest, Sha256};

    let encrypt_key = "testkey";
    let plaintext = "{\"challenge\":\"hello\"}";

    let key_bytes: [u8; 32] = {
        let mut h = Sha256::new();
        h.update(encrypt_key.as_bytes());
        h.finalize().into()
    };

    let iv: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let ciphertext = aes256_cbc_encrypt(&key_bytes, &iv, plaintext.as_bytes());

    let mut combined = iv.to_vec();
    combined.extend_from_slice(&ciphertext);
    let encoded = base64::engine::general_purpose::STANDARD.encode(&combined);

    let result = event_decrypt(encrypt_key, &encoded).unwrap();
    assert_eq!(result, plaintext);
}

#[test]
fn event_decrypt_too_short() {
    // "short" base64-encodes to 5 bytes, which is < the required 16-byte IV minimum
    let result = event_decrypt("key", "c2hvcnQ=");
    assert!(result.is_err());
}

#[test]
fn event_decrypt_wrong_key() {
    use base64::Engine as _;
    use sha2::{Digest, Sha256};

    let correct_key = "right_key";
    let wrong_key = "wrong_key";
    let plaintext = "test data 12345!";

    let key_bytes: [u8; 32] = {
        let mut h = Sha256::new();
        h.update(correct_key.as_bytes());
        h.finalize().into()
    };
    let iv: [u8; 16] = [0u8; 16];
    let ciphertext = aes256_cbc_encrypt(&key_bytes, &iv, plaintext.as_bytes());

    let mut combined = iv.to_vec();
    combined.extend_from_slice(&ciphertext);
    let encoded = base64::engine::general_purpose::STANDARD.encode(&combined);

    let result = event_decrypt(wrong_key, &encoded);
    // Decryption with wrong key either errors or produces different bytes
    assert!(result.is_err() || result.unwrap() != plaintext);
}

#[test]
fn event_decrypt_invalid_base64() {
    let result = event_decrypt("key", "!!!not-base64!!!");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("base64"));
}

#[test]
fn event_decrypt_empty_input() {
    let result = event_decrypt("key", "");
    assert!(result.is_err());
}

// ── event_encrypt ──

#[test]
fn event_encrypt_roundtrip() {
    let key = "my_secret_encrypt_key";
    let plaintext = r#"{"challenge":"hello_world","type":"url_verification"}"#;

    let encrypted = event_encrypt(key, plaintext).unwrap();
    let decrypted = event_decrypt(key, &encrypted).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn event_encrypt_produces_different_ciphertext_each_call() {
    let key = "test_key";
    let plaintext = "same input";
    let a = event_encrypt(key, plaintext).unwrap();
    let b = event_encrypt(key, plaintext).unwrap();
    assert_ne!(a, b);
    assert_eq!(event_decrypt(key, &a).unwrap(), plaintext);
    assert_eq!(event_decrypt(key, &b).unwrap(), plaintext);
}

#[test]
fn event_encrypt_with_event_dispatcher() {
    use larksuite_oapi_sdk_rs::event::{EventDispatcher, EventReq};

    let encrypt_key = "dispatcher_test_key";
    let inner_body = serde_json::json!({
        "type": "url_verification",
        "token": "",
        "challenge": "from_encrypt"
    });
    let encrypted =
        event_encrypt(encrypt_key, &serde_json::to_string(&inner_body).unwrap()).unwrap();
    let outer_body = serde_json::json!({ "encrypt": encrypted });

    let dispatcher = EventDispatcher::new("", encrypt_key);
    let req = EventReq {
        headers: Default::default(),
        body: serde_json::to_vec(&outer_body).unwrap(),
        request_uri: String::new(),
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let resp = rt.block_on(dispatcher.handle(req));
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(parsed["challenge"], "from_encrypt");
}

#[test]
fn encrypted_event_msg_roundtrip() {
    use larksuite_oapi_sdk_rs::crypto::{encrypted_event_msg, event_decrypt};

    let key = "test_encrypt_key";
    let payload = serde_json::json!({"type": "url_verification", "challenge": "abc"});

    let msg = encrypted_event_msg(&payload, key).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
    let encrypted = parsed["encrypt"].as_str().unwrap();
    let decrypted = event_decrypt(key, encrypted).unwrap();
    let inner: serde_json::Value = serde_json::from_str(&decrypted).unwrap();
    assert_eq!(inner["challenge"], "abc");
}
