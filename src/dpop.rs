//! DPoP proof generation for OAuth token requests.
//!
//! The key never leaves this type. Callers receive only the public JWK and a
//! compact proof, making it safe to attach proofs to HTTP requests.

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use p256::ecdsa::{Signature, SigningKey, VerifyingKey, signature::Signer};
use serde::Serialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

fn encode_part<T: Serialize>(value: &T) -> Result<String, DPoPError> {
    Ok(URL_SAFE_NO_PAD.encode(serde_json::to_vec(value)?))
}

#[derive(Debug, thiserror::Error)]
pub enum DPoPError {
    #[error("invalid DPoP URL: {0}")]
    InvalidUrl(String),
    #[error("failed to serialize DPoP proof: {0}")]
    Serialize(#[from] serde_json::Error),
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
        let parsed = url::Url::parse(uri).map_err(|e| DPoPError::InvalidUrl(e.to_string()))?;
        let htu = format!(
            "{}://{}{}",
            parsed.scheme(),
            parsed.host_str().unwrap_or_default(),
            parsed.path()
        );
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
