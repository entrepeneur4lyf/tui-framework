//! Hook implementations for reactive programming.

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// A unique identifier for an effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectId(u64);

impl EffectId {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// An effect that can be executed and cleaned up.
pub struct Effect {
    /// Unique identifier for this effect.
    pub id: EffectId,
    /// The effect function that returns an optional cleanup function.
    pub effect_fn: Box<dyn Fn() -> Option<Box<dyn Fn() + Send + Sync>> + Send + Sync>,
    /// Optional cleanup function to run when the effect is cleaned up.
    pub cleanup_fn: Option<Box<dyn Fn() + Send + Sync>>,
    /// Dependencies that determine when the effect should re-run.
    pub dependencies: Vec<String>,
}

impl Effect {
    /// Create a new effect with the given function and dependencies.
    pub fn new<F>(effect_fn: F, dependencies: Vec<String>) -> Self
    where
        F: Fn() -> Option<Box<dyn Fn() + Send + Sync>> + Send + Sync + 'static,
    {
        Self {
            id: EffectId::new(),
            effect_fn: Box::new(effect_fn),
            cleanup_fn: None,
            dependencies,
        }
    }

    /// Execute the effect, cleaning up any previous effect first.
    pub fn execute(&mut self) {
        // Clean up previous effect if it exists
        if let Some(cleanup) = self.cleanup_fn.take() {
            cleanup();
        }

        // Execute the effect and store any cleanup function
        self.cleanup_fn = (self.effect_fn)();
    }

    /// Clean up the effect by running its cleanup function.
    pub fn cleanup(&mut self) {
        if let Some(cleanup) = self.cleanup_fn.take() {
            cleanup();
        }
    }
}

/// Global effect manager for tracking and executing effects.
pub struct EffectManager {
    effects: Arc<RwLock<HashMap<EffectId, Effect>>>,
    dependency_cache: Arc<RwLock<HashMap<EffectId, u64>>>,
}

impl EffectManager {
    /// Create a new effect manager.
    pub fn new() -> Self {
        Self {
            effects: Arc::new(RwLock::new(HashMap::new())),
            dependency_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new effect and execute it if its dependencies have changed.
    pub fn register_effect(&self, mut effect: Effect) -> EffectId {
        let id = effect.id;

        // Calculate dependency hash
        let dep_hash = self.calculate_dependency_hash(&effect.dependencies);

        // Check if dependencies have changed
        let should_execute = {
            let cache = self.dependency_cache.read();
            cache
                .get(&id)
                .is_none_or(|&cached_hash| cached_hash != dep_hash)
        };

        if should_execute {
            effect.execute();

            // Update dependency cache
            {
                let mut cache = self.dependency_cache.write();
                cache.insert(id, dep_hash);
            }
        }

        // Store the effect
        {
            let mut effects = self.effects.write();
            effects.insert(id, effect);
        }

        id
    }

    /// Clean up a specific effect by its ID.
    pub fn cleanup_effect(&self, id: EffectId) {
        let mut effects = self.effects.write();
        if let Some(mut effect) = effects.remove(&id) {
            effect.cleanup();
        }

        let mut cache = self.dependency_cache.write();
        cache.remove(&id);
    }

    /// Clean up all registered effects.
    pub fn cleanup_all(&self) {
        let mut effects = self.effects.write();
        for (_, mut effect) in effects.drain() {
            effect.cleanup();
        }

        let mut cache = self.dependency_cache.write();
        cache.clear();
    }

    fn calculate_dependency_hash(&self, dependencies: &[String]) -> u64 {
        let mut hasher = DefaultHasher::new();
        for dep in dependencies {
            dep.hash(&mut hasher);
        }
        hasher.finish()
    }
}

impl Default for EffectManager {
    fn default() -> Self {
        Self::new()
    }
}

// Global effect manager instance
static EFFECT_MANAGER: Lazy<EffectManager> = Lazy::new(EffectManager::new);

/// Hook for side effects with dependency tracking.
pub fn use_effect<F, D>(effect: F, dependencies: D) -> EffectId
where
    F: Fn() -> Option<Box<dyn Fn() + Send + Sync>> + Send + Sync + 'static,
    D: AsRef<[String]>,
{
    let deps = dependencies.as_ref().to_vec();
    let effect_obj = Effect::new(effect, deps);
    EFFECT_MANAGER.register_effect(effect_obj)
}

/// Hook for side effects without cleanup.
pub fn use_effect_simple<F, D>(effect: F, dependencies: D) -> EffectId
where
    F: Fn() + Send + Sync + 'static,
    D: AsRef<[String]>,
{
    use_effect(
        move || {
            effect();
            None // No cleanup function
        },
        dependencies,
    )
}

/// Cleanup a specific effect.
pub fn cleanup_effect(id: EffectId) {
    EFFECT_MANAGER.cleanup_effect(id);
}

/// Cleanup all effects (useful for component unmounting).
pub fn cleanup_all_effects() {
    EFFECT_MANAGER.cleanup_all();
}

/// A memoized value that recalculates only when dependencies change.
pub struct MemoizedValue<T> {
    value: Arc<RwLock<Option<T>>>,
    compute_fn: Box<dyn Fn() -> T + Send + Sync>,
    dependencies: Vec<String>,
    dependency_hash: Arc<RwLock<Option<u64>>>,
}

impl<T> MemoizedValue<T> {
    /// Create a new memoized value with the given compute function and dependencies.
    pub fn new<F>(compute_fn: F, dependencies: Vec<String>) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            value: Arc::new(RwLock::new(None)),
            compute_fn: Box::new(compute_fn),
            dependencies,
            dependency_hash: Arc::new(RwLock::new(None)),
        }
    }

    /// Get the memoized value, recomputing it if dependencies have changed.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        // Calculate current dependency hash
        let mut hasher = DefaultHasher::new();
        for dep in &self.dependencies {
            dep.hash(&mut hasher);
        }
        let current_hash = hasher.finish();

        // Check if we need to recompute
        let needs_recompute = {
            let cached_hash = self.dependency_hash.read();
            *cached_hash != Some(current_hash)
        };

        if needs_recompute {
            // Recompute the value
            let new_value = (self.compute_fn)();

            // Update cached value and hash
            {
                let mut value = self.value.write();
                *value = Some(new_value);
            }
            {
                let mut hash = self.dependency_hash.write();
                *hash = Some(current_hash);
            }
        }

        // Return the cached value
        self.value.read().as_ref().unwrap().clone()
    }
}

/// Hook for memoized values with dependency tracking.
pub fn use_memo<T, F, D>(compute: F, dependencies: D) -> T
where
    T: Clone + Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
    D: AsRef<[String]>,
{
    let deps = dependencies.as_ref().to_vec();
    let memo = MemoizedValue::new(compute, deps);
    memo.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn test_use_effect_simple() {
        let counter = Arc::new(AtomicI32::new(0));
        let counter_clone = counter.clone();

        let effect_id = use_effect_simple(
            move || {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            },
            vec!["test".to_string()],
        );

        // Effect should have executed once
        assert_eq!(counter.load(Ordering::Relaxed), 1);

        cleanup_effect(effect_id);
    }

    #[test]
    fn test_use_memo() {
        let compute_count = Arc::new(AtomicI32::new(0));
        let compute_count_clone = compute_count.clone();

        let result = use_memo(
            move || {
                compute_count_clone.fetch_add(1, Ordering::Relaxed);
                42
            },
            vec!["test".to_string()],
        );

        assert_eq!(result, 42);
        assert_eq!(compute_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_effect_cleanup() {
        let setup_count = Arc::new(AtomicI32::new(0));
        let cleanup_count = Arc::new(AtomicI32::new(0));

        let setup_clone = setup_count.clone();
        let cleanup_clone = cleanup_count.clone();

        let effect_id = use_effect(
            move || {
                setup_clone.fetch_add(1, Ordering::Relaxed);
                let cleanup_clone = cleanup_clone.clone();
                Some(Box::new(move || {
                    cleanup_clone.fetch_add(1, Ordering::Relaxed);
                }) as Box<dyn Fn() + Send + Sync>)
            },
            vec!["test".to_string()],
        );

        // Setup should have run
        assert_eq!(setup_count.load(Ordering::Relaxed), 1);
        assert_eq!(cleanup_count.load(Ordering::Relaxed), 0);

        cleanup_effect(effect_id);

        // Cleanup should have run
        assert_eq!(cleanup_count.load(Ordering::Relaxed), 1);
    }
}
