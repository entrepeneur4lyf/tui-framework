//! Memory pooling system for efficient object reuse.
//!
//! This module provides memory pools to reduce allocations and improve
//! performance by reusing objects instead of constantly allocating and
//! deallocating them.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A generic memory pool for reusing objects.
pub struct MemoryPool<T> {
    /// Available objects ready for reuse
    available: Arc<Mutex<VecDeque<PooledObject<T>>>>,
    /// Factory function for creating new objects
    factory: Box<dyn Fn() -> T + Send + Sync>,
    /// Reset function for cleaning objects before reuse
    reset: Box<dyn Fn(&mut T) + Send + Sync>,
    /// Maximum number of objects to keep in the pool
    max_size: usize,
    /// Maximum age for pooled objects
    max_age: Duration,
    /// Pool statistics
    stats: Arc<Mutex<PoolStats>>,
}

impl<T> MemoryPool<T>
where
    T: Send + 'static,
{
    /// Create a new memory pool.
    pub fn new<F, R>(factory: F, reset: R, max_size: usize, max_age: Duration) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
        R: Fn(&mut T) + Send + Sync + 'static,
    {
        Self {
            available: Arc::new(Mutex::new(VecDeque::new())),
            factory: Box::new(factory),
            reset: Box::new(reset),
            max_size,
            max_age,
            stats: Arc::new(Mutex::new(PoolStats::default())),
        }
    }

    /// Acquire an object from the pool.
    pub fn acquire(&self) -> PooledItem<T> {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        stats.total_acquisitions += 1;

        // Try to reuse an existing object
        while let Some(pooled) = available.pop_front() {
            if !pooled.is_stale(self.max_age) {
                stats.reuses += 1;
                let mut object = pooled.object;
                (self.reset)(&mut object);
                return PooledItem::new(object, self.available.clone(), self.max_size, &mut stats);
            } else {
                stats.stale_evictions += 1;
            }
        }

        // Create a new object
        stats.new_creations += 1;
        let object = (self.factory)();
        PooledItem::new(object, self.available.clone(), self.max_size, &mut stats)
    }

    /// Get the current size of the pool.
    pub fn size(&self) -> usize {
        let available = self.available.lock().unwrap();
        available.len()
    }

    /// Clear all objects from the pool.
    pub fn clear(&self) {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let cleared_count = available.len();
        available.clear();
        stats.manual_clears += cleared_count as u64;
    }

    /// Remove stale objects from the pool.
    pub fn cleanup_stale(&self) {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let original_len = available.len();
        available.retain(|pooled| !pooled.is_stale(self.max_age));
        let removed = original_len - available.len();
        stats.stale_evictions += removed as u64;
    }

    /// Get pool statistics.
    pub fn get_stats(&self) -> PoolStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Get the reuse ratio as a percentage.
    pub fn reuse_ratio(&self) -> f64 {
        let stats = self.stats.lock().unwrap();
        if stats.total_acquisitions == 0 {
            0.0
        } else {
            (stats.reuses as f64 / stats.total_acquisitions as f64) * 100.0
        }
    }
}

/// A pooled object with metadata.
struct PooledObject<T> {
    /// The actual object
    object: T,
    /// When this object was created
    created_at: Instant,
}

impl<T> PooledObject<T> {
    /// Create a new pooled object.
    fn new(object: T) -> Self {
        Self {
            object,
            created_at: Instant::now(),
        }
    }

    /// Check if this object is stale.
    fn is_stale(&self, max_age: Duration) -> bool {
        self.created_at.elapsed() > max_age
    }
}

/// An item borrowed from the memory pool.
pub struct PooledItem<T> {
    /// The borrowed object (None when returned to pool)
    object: Option<T>,
    /// Reference to the pool for returning the object
    pool: Arc<Mutex<VecDeque<PooledObject<T>>>>,
    /// Maximum pool size
    max_size: usize,
    /// Whether this item has been returned to the pool
    returned: bool,
}

impl<T> PooledItem<T> {
    /// Create a new pooled item.
    fn new(
        object: T,
        pool: Arc<Mutex<VecDeque<PooledObject<T>>>>,
        max_size: usize,
        stats: &mut PoolStats,
    ) -> Self {
        stats.active_items += 1;
        Self {
            object: Some(object),
            pool,
            max_size,
            returned: false,
        }
    }

    /// Get a reference to the object.
    pub fn get(&self) -> Option<&T> {
        self.object.as_ref()
    }

    /// Get a mutable reference to the object.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.object.as_mut()
    }

    /// Manually return the object to the pool.
    pub fn return_to_pool(&mut self) {
        if !self.returned && self.object.is_some() {
            let object = self.object.take().unwrap();
            let mut pool = self.pool.lock().unwrap();

            // Only return to pool if there's space
            if pool.len() < self.max_size {
                pool.push_back(PooledObject::new(object));
            }

            self.returned = true;
        }
    }
}

impl<T> Drop for PooledItem<T> {
    fn drop(&mut self) {
        self.return_to_pool();
    }
}

impl<T> std::ops::Deref for PooledItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.object.as_ref().expect("Object has been returned to pool")
    }
}

impl<T> std::ops::DerefMut for PooledItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.object.as_mut().expect("Object has been returned to pool")
    }
}

/// Statistics for memory pool usage.
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total number of object acquisitions
    pub total_acquisitions: u64,
    /// Number of object reuses from pool
    pub reuses: u64,
    /// Number of new object creations
    pub new_creations: u64,
    /// Number of objects currently active (borrowed)
    pub active_items: u64,
    /// Number of stale objects evicted
    pub stale_evictions: u64,
    /// Number of objects cleared manually
    pub manual_clears: u64,
}

impl PoolStats {
    /// Get the reuse ratio as a percentage.
    pub fn reuse_ratio_percent(&self) -> f64 {
        if self.total_acquisitions == 0 {
            0.0
        } else {
            (self.reuses as f64 / self.total_acquisitions as f64) * 100.0
        }
    }

    /// Get the creation ratio as a percentage.
    pub fn creation_ratio_percent(&self) -> f64 {
        100.0 - self.reuse_ratio_percent()
    }
}

/// Specialized pools for common framework objects.
pub mod pools {
    use super::*;
    use crate::render::vdom::VirtualElement;
    use std::collections::HashMap;

    /// Pool for virtual DOM elements.
    pub type VirtualElementPool = MemoryPool<VirtualElement>;

    /// Pool for hash maps.
    pub type HashMapPool<K, V> = MemoryPool<HashMap<K, V>>;

    /// Pool for vectors.
    pub type VectorPool<T> = MemoryPool<Vec<T>>;

    /// Pool for strings.
    pub type StringPool = MemoryPool<String>;

    /// Create a pool for virtual DOM elements.
    pub fn create_virtual_element_pool() -> VirtualElementPool {
        MemoryPool::new(
            || VirtualElement {
                tag: String::new(),
                attributes: HashMap::new(),
                style: Default::default(),
                children: Vec::new(),
                layout: None,
            },
            |element| {
                element.tag.clear();
                element.attributes.clear();
                element.style = Default::default();
                element.children.clear();
                element.layout = None;
            },
            100, // max_size
            Duration::from_secs(300), // max_age (5 minutes)
        )
    }

    /// Create a pool for hash maps.
    pub fn create_hashmap_pool<K, V>() -> HashMapPool<K, V>
    where
        K: std::hash::Hash + Eq + Send + 'static,
        V: Send + 'static,
    {
        MemoryPool::new(
            HashMap::new,
            |map| map.clear(),
            50, // max_size
            Duration::from_secs(300), // max_age
        )
    }

    /// Create a pool for vectors.
    pub fn create_vector_pool<T>() -> VectorPool<T>
    where
        T: Send + 'static,
    {
        MemoryPool::new(
            Vec::new,
            |vec| vec.clear(),
            100, // max_size
            Duration::from_secs(300), // max_age
        )
    }

    /// Create a pool for strings.
    pub fn create_string_pool() -> StringPool {
        MemoryPool::new(
            String::new,
            |string| string.clear(),
            200, // max_size
            Duration::from_secs(300), // max_age
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_memory_pool_basic_operations() {
        let pool = MemoryPool::new(
            || String::new(),
            |s| s.clear(),
            10,
            Duration::from_secs(60),
        );

        // Acquire an object
        let mut item = pool.acquire();
        item.push_str("test");
        assert_eq!(*item, "test");

        // Pool should be empty initially
        assert_eq!(pool.size(), 0);

        // Return to pool
        drop(item);
        assert_eq!(pool.size(), 1);

        // Acquire again (should reuse)
        let item2 = pool.acquire();
        assert_eq!(*item2, ""); // Should be reset
        
        let stats = pool.get_stats();
        assert_eq!(stats.total_acquisitions, 2);
        assert_eq!(stats.reuses, 1);
        assert_eq!(stats.new_creations, 1);
    }

    #[test]
    fn test_pool_size_limit() {
        let pool = MemoryPool::new(
            || String::new(),
            |s| s.clear(),
            2, // Small limit
            Duration::from_secs(60),
        );

        // Create more items than the pool can hold
        let _item1 = pool.acquire();
        let _item2 = pool.acquire();
        let _item3 = pool.acquire();

        // Drop all items
        drop(_item1);
        drop(_item2);
        drop(_item3);

        // Pool should not exceed its limit
        assert!(pool.size() <= 2);
    }

    #[test]
    fn test_stale_object_cleanup() {
        let pool = MemoryPool::new(
            || String::new(),
            |s| s.clear(),
            10,
            Duration::from_millis(1), // Very short max age
        );

        let item = pool.acquire();
        drop(item);
        assert_eq!(pool.size(), 1);

        // Wait for objects to become stale
        std::thread::sleep(Duration::from_millis(2));

        // Cleanup should remove stale objects
        pool.cleanup_stale();
        assert_eq!(pool.size(), 0);
    }

    #[test]
    fn test_specialized_pools() {
        let element_pool = pools::create_virtual_element_pool();
        let hashmap_pool = pools::create_hashmap_pool::<String, i32>();
        let vector_pool = pools::create_vector_pool::<i32>();
        let string_pool = pools::create_string_pool();

        // Test each pool type
        let _element = element_pool.acquire();
        let _map = hashmap_pool.acquire();
        let _vec = vector_pool.acquire();
        let _string = string_pool.acquire();

        // All should work without panicking
        assert!(true);
    }
}
