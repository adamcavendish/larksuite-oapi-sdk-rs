use larksuite_oapi_sdk_rs::cache::{Cache, LocalCache};
use std::time::Duration;

#[tokio::test]
async fn test_set_and_get() {
    let cache = LocalCache::new();
    cache
        .set("k1", "v1", Duration::from_secs(60))
        .await
        .unwrap();
    let val = cache.get("k1").await.unwrap();
    assert_eq!(val.as_deref(), Some("v1"));
}

#[tokio::test]
async fn test_get_missing_key_returns_none() {
    let cache = LocalCache::new();
    let val = cache.get("missing").await.unwrap();
    assert!(val.is_none());
}

#[tokio::test]
async fn test_expired_entry_returns_none() {
    let cache = LocalCache::new();
    cache
        .set("k2", "v2", Duration::from_millis(1))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(10)).await;
    let val = cache.get("k2").await.unwrap();
    assert!(val.is_none());
}

#[tokio::test]
async fn test_overwrite_key() {
    let cache = LocalCache::new();
    cache
        .set("k3", "old", Duration::from_secs(60))
        .await
        .unwrap();
    cache
        .set("k3", "new", Duration::from_secs(60))
        .await
        .unwrap();
    let val = cache.get("k3").await.unwrap();
    assert_eq!(val.as_deref(), Some("new"));
}

#[tokio::test]
async fn test_spawn_cleanup_task_evicts_expired() {
    let cache = LocalCache::new();
    cache
        .set("temp", "data", Duration::from_millis(20))
        .await
        .unwrap();

    let _handle = cache.spawn_cleanup_task(Duration::from_millis(10));
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Entry should be gone (either evicted by cleanup or by lazy eviction on get)
    let val = cache.get("temp").await.unwrap();
    assert!(val.is_none());
}
