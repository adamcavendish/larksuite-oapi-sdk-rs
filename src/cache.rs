use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub trait Cache: Send + Sync {
    fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Duration,
    ) -> Pin<Box<dyn Future<Output = crate::Result<()>> + Send + '_>>;

    fn get(
        &self,
        key: &str,
    ) -> Pin<Box<dyn Future<Output = crate::Result<Option<String>>> + Send + '_>>;
}

struct CacheEntry {
    value: String,
    expires_at: Instant,
}

pub struct LocalCache {
    store: Mutex<HashMap<String, CacheEntry>>,
}

impl LocalCache {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for LocalCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache for LocalCache {
    fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Duration,
    ) -> Pin<Box<dyn Future<Output = crate::Result<()>> + Send + '_>> {
        let key = key.to_string();
        let value = value.to_string();
        Box::pin(async move {
            let mut store = self.store.lock().unwrap();
            store.insert(
                key,
                CacheEntry {
                    value,
                    expires_at: Instant::now() + ttl,
                },
            );
            Ok(())
        })
    }

    fn get(
        &self,
        key: &str,
    ) -> Pin<Box<dyn Future<Output = crate::Result<Option<String>>> + Send + '_>> {
        let key = key.to_string();
        Box::pin(async move {
            let mut store = self.store.lock().unwrap();
            match store.get(&key) {
                Some(entry) if entry.expires_at > Instant::now() => Ok(Some(entry.value.clone())),
                Some(_) => {
                    store.remove(&key);
                    Ok(None)
                }
                None => Ok(None),
            }
        })
    }
}
