//! Application-owned persistent DPoP key store.
//!
//! This example deliberately does not choose a default path. Applications
//! should decide where secrets live, set permissions, and perform deployment-
//! specific backup and rotation policies.

use larksuite_oapi_sdk_rs::{DPoPError, DPoPKey, DPoPKeyStore};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct StoredKey {
    private_key: String,
    kid: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DPoPFileKeyStore {
    directory: PathBuf,
}

impl DPoPFileKeyStore {
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
        }
    }

    fn path(&self, key_id: &str) -> PathBuf {
        self.directory.join(format!("{}.json", hex::encode(key_id)))
    }
}

impl DPoPKeyStore for DPoPFileKeyStore {
    fn load(&self, key_id: &str) -> Result<Option<DPoPKey>, DPoPError> {
        let path = self.path(key_id);
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(DPoPError::KeyStore(error.to_string())),
        };
        let stored: StoredKey = serde_json::from_slice(&bytes)
            .map_err(|error| DPoPError::KeyStore(error.to_string()))?;
        let raw = hex::decode(stored.private_key)
            .map_err(|error| DPoPError::KeyStore(error.to_string()))?;
        let raw: [u8; 32] = raw
            .try_into()
            .map_err(|_| DPoPError::KeyStore("private key must be 32 bytes".into()))?;
        let key = DPoPKey::from_private_key_bytes(raw)?;
        Ok(Some(
            stored.kid.map_or(key.clone(), |kid| key.with_kid(kid)),
        ))
    }

    fn save(&self, key_id: &str, key: &DPoPKey) -> Result<(), DPoPError> {
        let path = self.path(key_id);
        let temporary = path.with_extension("json.tmp");
        let stored = StoredKey {
            private_key: hex::encode(key.private_key_bytes()),
            kid: key.kid().map(str::to_owned),
        };
        let bytes = serde_json::to_vec_pretty(&stored)
            .map_err(|error| DPoPError::KeyStore(error.to_string()))?;
        fs::write(&temporary, bytes).map_err(|error| DPoPError::KeyStore(error.to_string()))?;
        fs::rename(&temporary, &path).map_err(|error| DPoPError::KeyStore(error.to_string()))
    }

    fn remove(&self, key_id: &str) -> Result<(), DPoPError> {
        match fs::remove_file(self.path(key_id)) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(DPoPError::KeyStore(error.to_string())),
        }
    }
}

fn main() {
    let _ = DPoPFileKeyStore::new(Path::new("./private/lark-dpop"));
    println!("Provide an application-owned directory before using DPoPFileKeyStore");
}
