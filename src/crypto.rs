use aes::cipher::{BlockModeDecrypt, KeyIvInit};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

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
    let mut content = String::new();
    content.push_str(timestamp);
    content.push_str(nonce);
    content.push_str(encrypt_key);
    content.push_str(&String::from_utf8_lossy(body));

    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
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
