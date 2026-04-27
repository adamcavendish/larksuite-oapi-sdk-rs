//! In-memory token cache with TTL expiry.
//!
//! [`LocalCache`] is a thread-safe HashMap-backed cache used by [`crate::token::TokenManager`]
//! to avoid redundant token fetches. Call [`LocalCache::spawn_cleanup_task`] to periodically
//! evict expired entries.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Token cache interface. Implement this trait to use a custom cache backend
/// (e.g. Redis). The default [`LocalCache`] is an in-memory HashMap with TTL.
pub trait Cache: Send + Sync {
    fn set(
        &self,
        key: &str,
        value: &str,
        ttl: Duration,
    ) -> Pin<Box<dyn Future<Output = Result<(), crate::LarkError>> + Send + '_>>;

    fn get(
        &self,
        key: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, crate::LarkError>> + Send + '_>>;
}

struct CacheEntry {
    value: String,
    expires_at: Instant,
}

pub struct LocalCache {
    store: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl LocalCache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Spawn a background task that purges expired entries every `interval`.
    /// Returns a `tokio::task::JoinHandle` — drop it to stop the cleanup loop.
    pub fn spawn_cleanup_task(&self, interval: Duration) -> tokio::task::JoinHandle<()> {
        let store = Arc::clone(&self.store);
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                let now = Instant::now();
                let mut guard = store.lock().unwrap_or_else(|p| p.into_inner());
                guard.retain(|_, entry| entry.expires_at > now);
            }
        })
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
    ) -> Pin<Box<dyn Future<Output = Result<(), crate::LarkError>> + Send + '_>> {
        let key = key.to_string();
        let value = value.to_string();
        Box::pin(async move {
            let mut store = self.store.lock().unwrap_or_else(|p| p.into_inner());
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
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, crate::LarkError>> + Send + '_>> {
        let key = key.to_string();
        Box::pin(async move {
            let mut store = self.store.lock().unwrap_or_else(|p| p.into_inner());
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
