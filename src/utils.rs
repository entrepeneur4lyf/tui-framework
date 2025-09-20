//! Utility functions and helpers.

pub mod memory_pool;

use std::collections::HashMap;
use std::hash::Hash;

/// A simple cache implementation.
pub struct Cache<K, V> {
    data: HashMap<K, V>,
    max_size: usize,
}

impl<K: Hash + Eq + Clone, V> Cache<K, V> {
    /// Create a new cache with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }

    /// Get a value from the cache.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Insert a value into the cache.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.data.len() >= self.max_size && !self.data.contains_key(&key) {
            // Simple eviction: remove the first item
            let keys: Vec<K> = self.data.keys().cloned().collect();
            if let Some(key_to_remove) = keys.first() {
                self.data.remove(key_to_remove);
            }
        }
        self.data.insert(key, value)
    }

    /// Remove a value from the cache.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the current size of the cache.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Debounce utility for rate-limiting function calls.
pub struct Debouncer {
    last_call: std::time::Instant,
    delay: std::time::Duration,
}

impl Debouncer {
    /// Create a new debouncer with the specified delay.
    pub fn new(delay: std::time::Duration) -> Self {
        Self {
            last_call: std::time::Instant::now() - delay,
            delay,
        }
    }

    /// Check if enough time has passed since the last call.
    pub fn should_call(&mut self) -> bool {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_call) >= self.delay {
            self.last_call = now;
            true
        } else {
            false
        }
    }
}

/// Utility for generating unique IDs.
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Utility for clamping values to a range.
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cache() {
        let mut cache = Cache::new(2);

        cache.insert("key1", "value1");
        cache.insert("key2", "value2");

        assert_eq!(cache.get(&"key1"), Some(&"value1"));
        assert_eq!(cache.get(&"key2"), Some(&"value2"));
        assert_eq!(cache.len(), 2);

        // This should evict one of the existing keys
        cache.insert("key3", "value3");
        assert_eq!(cache.len(), 2); // Still only 2 items
        assert_eq!(cache.get(&"key3"), Some(&"value3"));

        // At least one of the original keys should be gone
        let key1_exists = cache.get(&"key1").is_some();
        let key2_exists = cache.get(&"key2").is_some();
        assert!(!(key1_exists && key2_exists)); // Not both can exist
    }

    #[test]
    fn test_debouncer() {
        let mut debouncer = Debouncer::new(Duration::from_millis(100));

        assert!(debouncer.should_call());
        assert!(!debouncer.should_call()); // Too soon

        std::thread::sleep(Duration::from_millis(101));
        assert!(debouncer.should_call());
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-1, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }
}
