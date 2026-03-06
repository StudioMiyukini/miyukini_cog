use moka::sync::Cache;
use std::sync::Arc;

/// In-memory thumbnail cache backed by moka.
/// Keys are "{file_id}:{width}x{height}", values are PNG bytes.
#[derive(Clone)]
pub struct ThumbnailCache {
    inner: Arc<Cache<String, Vec<u8>>>,
}

impl ThumbnailCache {
    /// Create a new cache with the given max entry count.
    pub fn new(max_entries: u64) -> Self {
        Self {
            inner: Arc::new(
                Cache::builder()
                    .max_capacity(max_entries)
                    .time_to_live(std::time::Duration::from_secs(3600)) // 1 hour TTL
                    .build(),
            ),
        }
    }

    pub fn get(&self, file_id: &str, width: u32, height: u32) -> Option<Vec<u8>> {
        let key = cache_key(file_id, width, height);
        self.inner.get(&key)
    }

    pub fn insert(&self, file_id: &str, width: u32, height: u32, data: Vec<u8>) {
        let key = cache_key(file_id, width, height);
        self.inner.insert(key, data);
    }

    pub fn invalidate(&self, file_id: &str) {
        // Invalidate all sizes - moka doesn't support prefix invalidation,
        // so we invalidate common sizes
        for (w, h) in &[(128, 128), (256, 256), (512, 512)] {
            let key = cache_key(file_id, *w, *h);
            self.inner.invalidate(&key);
        }
    }
}

fn cache_key(file_id: &str, width: u32, height: u32) -> String {
    format!("{file_id}:{width}x{height}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_insert_and_get() {
        let cache = ThumbnailCache::new(100);
        cache.insert("file1", 128, 128, vec![1, 2, 3]);
        let result = cache.get("file1", 128, 128);
        assert_eq!(result, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_cache_miss() {
        let cache = ThumbnailCache::new(100);
        assert!(cache.get("nonexistent", 128, 128).is_none());
    }

    #[test]
    fn test_cache_invalidate() {
        let cache = ThumbnailCache::new(100);
        cache.insert("file1", 128, 128, vec![1, 2, 3]);
        cache.invalidate("file1");
        assert!(cache.get("file1", 128, 128).is_none());
    }
}
