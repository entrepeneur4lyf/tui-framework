//! Layout caching system for performance optimization.
//!
//! This module provides caching capabilities for layout computations to avoid
//! recalculating layouts when the input hasn't changed.

use crate::component::ComponentId;
use crate::layout::Size;
use crate::layout::layout_engine::LayoutResult;
use crate::render::vdom::VirtualNode;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// A cache key for layout computations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LayoutCacheKey {
    /// Viewport size
    pub viewport: Size,
    /// Hash of the virtual DOM structure
    pub vdom_hash: u64,
    /// Component ID if applicable
    pub component_id: Option<ComponentId>,
}

impl LayoutCacheKey {
    /// Create a new cache key.
    pub fn new(viewport: Size, vdom: &VirtualNode, component_id: Option<ComponentId>) -> Self {
        Self {
            viewport,
            vdom_hash: Self::hash_vdom(vdom),
            component_id,
        }
    }

    /// Compute a hash for a virtual DOM node.
    fn hash_vdom(node: &VirtualNode) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        Self::hash_vdom_recursive(node, &mut hasher);
        hasher.finish()
    }

    /// Recursively hash a virtual DOM node.
    fn hash_vdom_recursive(node: &VirtualNode, hasher: &mut impl Hasher) {
        match node {
            VirtualNode::Empty => {
                "empty".hash(hasher);
            }
            VirtualNode::Text(text) => {
                "text".hash(hasher);
                text.content.hash(hasher);
            }
            VirtualNode::Element(element) => {
                "element".hash(hasher);
                element.tag.hash(hasher);
                
                // Hash style properties that affect layout
                if let Some(ref display) = element.style.display {
                    display.hash(hasher);
                }
                if let Some(ref width) = element.style.width {
                    width.hash(hasher);
                }
                if let Some(ref height) = element.style.height {
                    height.hash(hasher);
                }
                if let Some(ref flex_direction) = element.style.flex_direction {
                    flex_direction.hash(hasher);
                }
                
                // Hash children count (not full content for performance)
                element.children.len().hash(hasher);
                
                // Hash first few children for better cache discrimination
                for child in element.children.iter().take(3) {
                    Self::hash_vdom_recursive(child, hasher);
                }
            }
        }
    }
}

/// Cached layout entry with metadata.
#[derive(Debug, Clone)]
pub struct CachedLayout {
    /// The cached layout result
    pub result: LayoutResult,
    /// When this entry was created
    pub created_at: Instant,
    /// How many times this entry has been accessed
    pub access_count: u32,
    /// Last access time
    pub last_accessed: Instant,
}

impl CachedLayout {
    /// Create a new cached layout entry.
    pub fn new(result: LayoutResult) -> Self {
        let now = Instant::now();
        Self {
            result,
            created_at: now,
            access_count: 0,
            last_accessed: now,
        }
    }

    /// Mark this entry as accessed.
    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
        self.last_accessed = Instant::now();
    }

    /// Check if this entry is stale.
    pub fn is_stale(&self, max_age: Duration) -> bool {
        self.created_at.elapsed() > max_age
    }
}

/// Layout cache for storing computed layouts.
pub struct LayoutCache {
    /// The actual cache storage
    cache: Arc<RwLock<HashMap<LayoutCacheKey, CachedLayout>>>,
    /// Maximum number of entries to keep
    max_entries: usize,
    /// Maximum age for cache entries
    max_age: Duration,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
}

impl Default for LayoutCache {
    fn default() -> Self {
        Self::new(1000, Duration::from_secs(60))
    }
}

impl LayoutCache {
    /// Create a new layout cache.
    pub fn new(max_entries: usize, max_age: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_entries,
            max_age,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Get a cached layout result.
    pub fn get(&self, key: &LayoutCacheKey) -> Option<LayoutResult> {
        let mut cache = self.cache.write().unwrap();
        let mut stats = self.stats.write().unwrap();
        
        stats.total_requests += 1;

        if let Some(cached) = cache.get_mut(key) {
            if !cached.is_stale(self.max_age) {
                cached.mark_accessed();
                stats.hits += 1;
                return Some(cached.result.clone());
            } else {
                // Remove stale entry
                cache.remove(key);
                stats.stale_evictions += 1;
            }
        }

        stats.misses += 1;
        None
    }

    /// Store a layout result in the cache.
    pub fn put(&self, key: LayoutCacheKey, result: LayoutResult) {
        let mut cache = self.cache.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        // Check if we need to evict entries
        if cache.len() >= self.max_entries {
            self.evict_lru(&mut cache, &mut stats);
        }

        cache.insert(key, CachedLayout::new(result));
        stats.insertions += 1;
    }

    /// Evict least recently used entries.
    fn evict_lru(&self, cache: &mut HashMap<LayoutCacheKey, CachedLayout>, stats: &mut CacheStats) {
        // Find the least recently used entry
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for (key, cached) in cache.iter() {
            if cached.last_accessed < oldest_time {
                oldest_time = cached.last_accessed;
                oldest_key = Some(key.clone());
            }
        }

        if let Some(key) = oldest_key {
            cache.remove(&key);
            stats.lru_evictions += 1;
        }
    }

    /// Clear all cached entries.
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        let mut stats = self.stats.write().unwrap();
        
        cache.clear();
        stats.manual_clears += 1;
    }

    /// Remove stale entries from the cache.
    pub fn cleanup_stale(&self) {
        let mut cache = self.cache.write().unwrap();
        let mut stats = self.stats.write().unwrap();
        
        let keys_to_remove: Vec<_> = cache
            .iter()
            .filter(|(_, cached)| cached.is_stale(self.max_age))
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            cache.remove(&key);
            stats.stale_evictions += 1;
        }
    }

    /// Get cache statistics.
    pub fn get_stats(&self) -> CacheStats {
        let stats = self.stats.read().unwrap();
        stats.clone()
    }

    /// Get current cache size.
    pub fn size(&self) -> usize {
        let cache = self.cache.read().unwrap();
        cache.len()
    }

    /// Get cache hit ratio.
    pub fn hit_ratio(&self) -> f64 {
        let stats = self.stats.read().unwrap();
        if stats.total_requests == 0 {
            0.0
        } else {
            stats.hits as f64 / stats.total_requests as f64
        }
    }
}

/// Cache statistics for monitoring performance.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of cache requests
    pub total_requests: u64,
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of entries inserted
    pub insertions: u64,
    /// Number of LRU evictions
    pub lru_evictions: u64,
    /// Number of stale evictions
    pub stale_evictions: u64,
    /// Number of manual clears
    pub manual_clears: u64,
}

impl CacheStats {
    /// Get the hit ratio as a percentage.
    pub fn hit_ratio_percent(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.hits as f64 / self.total_requests as f64) * 100.0
        }
    }

    /// Get the miss ratio as a percentage.
    pub fn miss_ratio_percent(&self) -> f64 {
        100.0 - self.hit_ratio_percent()
    }
}

/// Incremental layout updater for efficient partial updates.
pub struct IncrementalLayoutUpdater {
    /// Previous layout result
    previous_result: Option<LayoutResult>,
    /// Cache for incremental updates
    cache: LayoutCache,
}

impl Default for IncrementalLayoutUpdater {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalLayoutUpdater {
    /// Create a new incremental layout updater.
    pub fn new() -> Self {
        Self {
            previous_result: None,
            cache: LayoutCache::default(),
        }
    }

    /// Update layout incrementally based on changes.
    pub fn update_layout(
        &mut self,
        viewport: Size,
        vdom: &VirtualNode,
        changes: &[crate::render::dirty_tracking::LayoutChange],
    ) -> LayoutResult {
        // If we have no previous result, compute full layout
        if self.previous_result.is_none() || changes.is_empty() {
            let result = crate::layout::layout_engine::Layout::compute(
                &mut vdom.clone(), 
                viewport
            );
            self.previous_result = Some(result.clone());
            return result;
        }

        // Try incremental update
        if let Some(ref previous) = self.previous_result {
            let mut updated_result = previous.clone();
            let mut needs_full_recompute = false;

            for change in changes {
                match change {
                    crate::render::dirty_tracking::LayoutChange::Resize(component_id, new_size) => {
                        // Update specific component size
                        if let Some(layout) = updated_result.layouts.get_mut(&component_id.to_string()) {
                            layout.size = *new_size;
                        } else {
                            needs_full_recompute = true;
                            break;
                        }
                    }
                    crate::render::dirty_tracking::LayoutChange::Add(_, _) |
                    crate::render::dirty_tracking::LayoutChange::Remove(_) => {
                        // Structural changes require full recompute
                        needs_full_recompute = true;
                        break;
                    }
                }
            }

            if !needs_full_recompute {
                self.previous_result = Some(updated_result.clone());
                return updated_result;
            }
        }

        // Fall back to full recompute
        let result = crate::layout::layout_engine::Layout::compute(
            &mut vdom.clone(), 
            viewport
        );
        self.previous_result = Some(result.clone());
        result
    }

    /// Clear the previous layout state.
    pub fn clear(&mut self) {
        self.previous_result = None;
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::vdom::VirtualNode;

    #[test]
    fn test_cache_key_creation() {
        let viewport = Size::new(80, 24);
        let vdom = VirtualNode::text("test");
        let component_id = Some(ComponentId::new());

        let key = LayoutCacheKey::new(viewport, &vdom, component_id);
        assert_eq!(key.viewport, viewport);
        assert_eq!(key.component_id, component_id);
        assert_ne!(key.vdom_hash, 0);
    }

    #[test]
    fn test_cache_operations() {
        let cache = LayoutCache::new(10, Duration::from_secs(60));
        let key = LayoutCacheKey::new(
            Size::new(80, 24),
            &VirtualNode::text("test"),
            None,
        );
        let result = LayoutResult {
            total_size: Size::new(80, 24),
            layouts: HashMap::new(),
        };

        // Initially empty
        assert!(cache.get(&key).is_none());
        assert_eq!(cache.size(), 0);

        // Store and retrieve
        cache.put(key.clone(), result.clone());
        assert_eq!(cache.size(), 1);
        
        let retrieved = cache.get(&key);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().total_size, result.total_size);

        // Check stats
        let stats = cache.get_stats();
        assert_eq!(stats.total_requests, 2); // One miss, one hit
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.insertions, 1);
    }

    #[test]
    fn test_cache_eviction() {
        let cache = LayoutCache::new(2, Duration::from_secs(60));
        let result = LayoutResult {
            total_size: Size::new(80, 24),
            layouts: HashMap::new(),
        };

        // Fill cache to capacity
        for i in 0..3 {
            let key = LayoutCacheKey::new(
                Size::new(80 + i, 24),
                &VirtualNode::text(&format!("test{}", i)),
                None,
            );
            cache.put(key, result.clone());
        }

        // Should have evicted one entry
        assert_eq!(cache.size(), 2);
        
        let stats = cache.get_stats();
        assert_eq!(stats.lru_evictions, 1);
    }
}
