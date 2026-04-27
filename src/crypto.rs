use aes::cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyIvInit};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

pub fn event_decrypt(encrypt_key: &str, encrypted: &str) -> crate::Result<String> {
    let key = {
        let mut hasher = Sha256::new();
        hasher.update(encrypt_key.as_bytes());
        hasher.finalize()
    };

    let cipher_bytes =
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted)
            .map_err(|e| crate::Error::Crypto(format!("base64 decode failed: {e}")))?;

    if cipher_bytes.len() < 16 {
        return Err(crate::Error::Crypto("encrypted data too short".to_string()));
    }

    let iv = &cipher_bytes[..16];
    let mut data = cipher_bytes[16..].to_vec();

    let decryptor = Aes256CbcDec::new_from_slices(&key, iv)
        .map_err(|e| crate::Error::Crypto(format!("cipher init failed: {e}")))?;

    let decrypted = decryptor
        .decrypt_padded::<aes::cipher::block_padding::Pkcs7>(&mut data)
        .map_err(|e| crate::Error::Crypto(format!("decryption failed: {e}")))?;

    String::from_utf8(decrypted.to_vec())
        .map_err(|e| crate::Error::Crypto(format!("utf8 decode failed: {e}")))
}

/// Encrypt a plaintext string using the same AES-256-CBC scheme Lark uses for
/// encrypted event callbacks. Useful for constructing mock event payloads in
/// tests. The inverse of [`event_decrypt`].
pub fn event_encrypt(encrypt_key: &str, plaintext: &str) -> crate::Result<String> {
    let key = {
        let mut hasher = Sha256::new();
        hasher.update(encrypt_key.as_bytes());
        hasher.finalize()
    };

    let iv = uuid::Uuid::new_v4();
    let iv = &iv.as_bytes()[..16];

    let encryptor = Aes256CbcEnc::new_from_slices(&key, iv)
        .map_err(|e| crate::Error::Crypto(format!("cipher init failed: {e}")))?;

    let encrypted =
        encryptor.encrypt_padded_vec::<aes::cipher::block_padding::Pkcs7>(plaintext.as_bytes());

    let mut result = Vec::with_capacity(16 + encrypted.len());
    result.extend_from_slice(iv);
    result.extend_from_slice(&encrypted);

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &result,
    ))
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.ct_eq(b).into()
}

pub fn verify_signature_sha256(
    timestamp: &str,
    nonce: &str,
    encrypt_key: &str,
    body: &[u8],
    expected_sig: &str,
) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(timestamp.as_bytes());
    hasher.update(nonce.as_bytes());
    hasher.update(encrypt_key.as_bytes());
    hasher.update(body);
    let result = hex::encode(hasher.finalize());

    constant_time_eq(result.as_bytes(), expected_sig.as_bytes())
}

pub fn verify_signature_sha1(
    timestamp: &str,
    nonce: &str,
    encrypt_key: &str,
    body: &str,
    expected_sig: &str,
) -> bool {
    use sha1::Sha1;
    let mut content = String::new();
    content.push_str(timestamp);
    content.push_str(nonce);
    content.push_str(encrypt_key);
    content.push_str(body);

    let mut hasher = Sha1::new();
    hasher.update(content.as_bytes());
    let result = hex::encode(hasher.finalize());

    constant_time_eq(result.as_bytes(), expected_sig.as_bytes())
}

/// Encrypt data and wrap it in a `{"encrypt": "..."}` JSON string, matching
/// Go SDK's `EncryptedEventMsg`. Useful for constructing mock encrypted event
/// payloads in tests.
pub fn encrypted_event_msg(
    data: &impl serde::Serialize,
    encrypt_key: &str,
) -> crate::Result<String> {
    let json_bytes = serde_json::to_string(data)
        .map_err(|e| crate::Error::Crypto(format!("json serialize failed: {e}")))?;
    let encrypted = event_encrypt(encrypt_key, &json_bytes)?;
    Ok(format!(r#"{{"encrypt":"{encrypted}"}}"#))
}
