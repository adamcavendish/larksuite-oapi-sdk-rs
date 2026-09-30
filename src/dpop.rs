//! DPoP proof generation for OAuth token requests.
//!
//! The key never leaves this type. Callers receive only the public JWK and a
//! compact proof, making it safe to attach proofs to HTTP requests.

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use p256::ecdsa::{Signature, SigningKey, VerifyingKey, signature::Signer};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DPoPMode {
    #[default]
    Disabled,
    Preferred,
    Required,
}

pub trait DPoPClock: Send + Sync + std::fmt::Debug {
    fn now_unix_seconds(&self) -> Result<i64, DPoPError>;
}

#[derive(Debug, Default)]
pub struct SystemDPoPClock;

impl DPoPClock for SystemDPoPClock {
    fn now_unix_seconds(&self) -> Result<i64, DPoPError> {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| DPoPError::Clock(e.to_string()))?
            .as_secs() as i64)
    }
}

fn encode_part<T: Serialize>(value: &T) -> Result<String, DPoPError> {
    Ok(URL_SAFE_NO_PAD.encode(serde_json::to_vec(value)?))
}

#[derive(Debug, thiserror::Error)]
pub enum DPoPError {
    #[error("DPoP access token is empty")]
    EmptyAccessToken,
    #[error("invalid DPoP URL: {0}")]
    InvalidUrl(String),
    #[error("failed to serialize DPoP proof: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to read DPoP clock: {0}")]
    Clock(String),
    #[error("DPoP key store error: {0}")]
    KeyStore(String),
}

pub trait DPoPKeyStore: Send + Sync + std::fmt::Debug {
    fn load(&self, key_id: &str) -> Result<Option<DPoPKey>, DPoPError>;
    fn save(&self, key_id: &str, key: &DPoPKey) -> Result<(), DPoPError>;
    fn remove(&self, key_id: &str) -> Result<(), DPoPError>;
    fn load_or_generate(&self, key_id: &str) -> Result<DPoPKey, DPoPError> {
        if let Some(key) = self.load(key_id)? {
            return Ok(key);
        }
        let key = DPoPKey::generate();
        self.save(key_id, &key)?;
        Ok(key)
    }
}

#[derive(Clone, Default)]
pub struct MemoryDPoPKeyStore {
    keys: Arc<Mutex<HashMap<String, DPoPKey>>>,
}

impl std::fmt::Debug for MemoryDPoPKeyStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryDPoPKeyStore")
            .field(
                "key_count",
                &self.keys.lock().map(|keys| keys.len()).unwrap_or_default(),
            )
            .finish()
    }
}

impl MemoryDPoPKeyStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_or_generate(&self, key_id: &str) -> Result<DPoPKey, DPoPError> {
        <Self as DPoPKeyStore>::load_or_generate(self, key_id)
    }
}

impl DPoPKeyStore for MemoryDPoPKeyStore {
    fn load(&self, key_id: &str) -> Result<Option<DPoPKey>, DPoPError> {
        self.keys
            .lock()
            .map_err(|_| DPoPError::KeyStore("memory key store lock poisoned".into()))
            .map(|keys| keys.get(key_id).cloned())
    }

    fn save(&self, key_id: &str, key: &DPoPKey) -> Result<(), DPoPError> {
        self.keys
            .lock()
            .map_err(|_| DPoPError::KeyStore("memory key store lock poisoned".into()))?
            .insert(key_id.to_string(), key.clone());
        Ok(())
    }

    fn remove(&self, key_id: &str) -> Result<(), DPoPError> {
        self.keys
            .lock()
            .map_err(|_| DPoPError::KeyStore("memory key store lock poisoned".into()))?
            .remove(key_id);
        Ok(())
    }
}

#[derive(Clone)]
pub struct DPoPKey {
    signing_key: SigningKey,
    kid: Option<String>,
}

impl std::fmt::Debug for DPoPKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DPoPKey")
            .field("kid", &self.kid)
            .finish_non_exhaustive()
    }
}

impl DPoPKey {
    pub fn generate() -> Self {
    pub fn private_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes().into()
    }

    pub fn from_private_key_bytes(bytes: [u8; 32]) -> Result<Self, DPoPError> {
        let signing_key = SigningKey::from_bytes((&bytes).into())
            .map_err(|e| DPoPError::KeyStore(format!("invalid P-256 private key: {e}")))?;
        Ok(Self { signing_key, kid: None })
    }

        Self {
            signing_key: SigningKey::random(&mut rand_core::OsRng),
            kid: None,
        }
    }

    pub fn with_kid(mut self, kid: impl Into<String>) -> Self {
        self.kid = Some(kid.into());
        self
    }

    pub fn kid(&self) -> Option<&str> {
        self.kid.as_deref()
    }

    pub fn proof(&self, method: &str, uri: &str, iat: i64) -> Result<String, DPoPError> {
        self.proof_with_access_token(method, uri, iat, None)
    }

    /// Generate a resource proof bound to an access token. Token endpoint
    /// proofs omit `ath`; resource proofs must include it.
    pub fn resource_proof(
        &self,
        method: &str,
        uri: &str,
        iat: i64,
        access_token: &str,
    ) -> Result<String, DPoPError> {
        self.proof_with_access_token(method, uri, iat, Some(access_token))
    }

    fn proof_with_access_token(
        &self,
        method: &str,
        uri: &str,
        iat: i64,
        access_token: Option<&str>,
    ) -> Result<String, DPoPError> {
        let parsed = url::Url::parse(uri).map_err(|e| DPoPError::InvalidUrl(e.to_string()))?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
            return Err(DPoPError::InvalidUrl(uri.to_string()));
        }
        let mut htu_url = parsed;
        htu_url.set_query(None);
        htu_url.set_fragment(None);
        let htu = htu_url.to_string();
        let header = Header {
            typ: "dpop+jwt",
            alg: "ES256",
            jwk: self.jwk(),
        };
        let claims = Claims {
            jti: Uuid::new_v4().to_string(),
            htm: method,
            htu,
            iat,
            ath: access_token.map(|token| URL_SAFE_NO_PAD.encode(Sha256::digest(token.as_bytes()))),
        };
        let signing = format!("{}.{}", encode_part(&header)?, encode_part(&claims)?);
        let sig: Signature = self.signing_key.sign(signing.as_bytes());
        Ok(format!(
            "{}.{}",
            signing,
            URL_SAFE_NO_PAD.encode(sig.to_bytes())
        ))
    }

    fn jwk(&self) -> Jwk {
        let point = VerifyingKey::from(&self.signing_key).to_encoded_point(false);
        Jwk {
            kty: "EC",
            crv: "P-256",
            x: URL_SAFE_NO_PAD.encode(point.x().unwrap()),
            y: URL_SAFE_NO_PAD.encode(point.y().unwrap()),
        }
    }

    pub fn thumbprint(&self) -> String {
        let jwk = self.jwk();
        let canonical = format!(
            r#"{{"crv":"P-256","kty":"EC","x":"{}","y":"{}"}}"#,
            jwk.x, jwk.y
        );
        URL_SAFE_NO_PAD.encode(Sha256::digest(canonical.as_bytes()))
    }
}

/// An access token issued with `token_type=DPoP` and its corresponding key.
/// Construct this only after validating the token endpoint response.
#[derive(Clone)]
pub struct DPoPBinding {
    access_token: String,
    key: DPoPKey,
}

impl std::fmt::Debug for DPoPBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DPoPBinding")
            .field("access_token", &"[redacted]")
            .field("key_thumbprint", &self.key.thumbprint())
            .finish()
    }
}

impl DPoPBinding {
    pub fn new(access_token: impl Into<String>, key: DPoPKey) -> Result<Self, DPoPError> {
        let access_token = access_token.into();
        if access_token.is_empty() {
            return Err(DPoPError::EmptyAccessToken);
        }
        Ok(Self { access_token, key })
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn key_thumbprint(&self) -> String {
        self.key.thumbprint()
    }

    pub fn proof(&self, method: &str, uri: &str, iat: i64) -> Result<String, DPoPError> {
        self.key
            .resource_proof(method, uri, iat, &self.access_token)
    }
}

#[derive(Serialize)]
struct Header<'a> {
    typ: &'a str,
    alg: &'a str,
    jwk: Jwk,
}
#[derive(Serialize)]
struct Claims<'a> {
    jti: String,
    htm: &'a str,
    htu: String,
    iat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    ath: Option<String>,
}
#[derive(Serialize)]
struct Jwk {
    kty: &'static str,
    crv: &'static str,
    x: String,
    y: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn memory_store_reuses_and_removes_keys() {
        let store = super::MemoryDPoPKeyStore::new();
        let first = store.load_or_generate("tenant-a").unwrap();
        let second = store.load_or_generate("tenant-a").unwrap();
        assert_eq!(first.thumbprint(), second.thumbprint());
        assert!(store.load("tenant-b").unwrap().is_none());
        store.remove("tenant-a").unwrap();
        assert!(store.load("tenant-a").unwrap().is_none());
    }
    #[test]
    fn resource_proof_binds_token_and_preserves_non_default_port() {
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
        use sha2::{Digest, Sha256};

        let key = super::DPoPKey::generate();
        let proof = key
            .resource_proof(
                "GET",
                "https://example.com:8443/path?q=secret#part",
                123,
                "bound-token",
            )
            .unwrap();
        let payload = proof.split('.').nth(1).unwrap();
        let claims: serde_json::Value =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(payload).unwrap()).unwrap();
        assert_eq!(claims["htu"], "https://example.com:8443/path");
        assert_eq!(claims["htm"], "GET");
        assert_eq!(
            claims["ath"],
            URL_SAFE_NO_PAD.encode(Sha256::digest(b"bound-token"))
        );
        assert!(!proof.contains("bound-token"));
    }
    use super::*;

    #[test]
    fn proof_contains_dpop_jwt_parts_and_bound_request() {
        let proof = DPoPKey::generate()
            .proof(
                "POST",
                "https://accounts.feishu.cn/oauth/token?x=1",
                1_700_000_000,
            )
            .unwrap();
        let parts: Vec<_> = proof.split('.').collect();
        assert_eq!(parts.len(), 3);
        let header: serde_json::Value =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[0]).unwrap()).unwrap();
        let claims: serde_json::Value =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[1]).unwrap()).unwrap();
        assert_eq!(header["typ"], "dpop+jwt");
        assert_eq!(header["alg"], "ES256");
        assert_eq!(claims["htm"], "POST");
        assert_eq!(claims["htu"], "https://accounts.feishu.cn/oauth/token");
        assert_eq!(claims["iat"], 1_700_000_000);
        assert!(claims["jti"].as_str().is_some());
    }

    #[test]
    fn debug_does_not_expose_private_key() {
        let key = DPoPKey::generate().with_kid("test-kid");
        let rendered = format!("{key:?}");
        assert!(rendered.contains("test-kid"));
        assert!(!rendered.contains("signing_key"));
    }
}
