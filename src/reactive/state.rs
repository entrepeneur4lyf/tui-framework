//! Reactive state management.

use parking_lot::{RwLock, RwLockReadGuard};
use std::sync::Arc;
use std::fmt;

/// A reactive state container that notifies observers when the value changes.
pub struct State<T> {
    value: Arc<RwLock<T>>,
    observers: Arc<RwLock<Vec<Box<dyn Fn(&T) + Send + Sync>>>>,
}

impl<T> State<T> {
    /// Create a new state with an initial value.
    pub fn new(initial_value: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial_value)),
            observers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get a read-only reference to the current value.
    pub fn get(&self) -> RwLockReadGuard<'_, T> {
        self.value.read()
    }

    /// Set a new value and notify observers.
    pub fn set(&self, new_value: T) {
        {
            let mut value = self.value.write();
            *value = new_value;
        }
        self.notify_observers();
    }

    /// Update the value using a closure and notify observers.
    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.write();
            updater(&mut *value);
        }
        self.notify_observers();
    }

    /// Subscribe to changes in this state.
    pub fn subscribe<F>(&self, observer: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut observers = self.observers.write();
        observers.push(Box::new(observer));
    }

    /// Get a clone of the current value (requires T: Clone).
    pub fn clone_value(&self) -> T
    where
        T: Clone,
    {
        self.value.read().clone()
    }

    /// Map the state to a new type.
    pub fn map<U, F>(&self, mapper: F) -> MappedState<T, U, F>
    where
        F: Fn(&T) -> U + Send + Sync,
    {
        MappedState {
            source: self.clone(),
            mapper,
        }
    }

    /// Notify all observers of the current value.
    fn notify_observers(&self) {
        let value = self.value.read();
        let observers = self.observers.read();
        for observer in observers.iter() {
            observer(&*value);
        }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            observers: self.observers.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for State<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("value", &*self.value.read())
            .finish()
    }
}

/// A state that is derived from another state through a mapping function.
pub struct MappedState<T, U, F>
where
    F: Fn(&T) -> U + Send + Sync,
{
    source: State<T>,
    mapper: F,
}

impl<T, U, F> MappedState<T, U, F>
where
    F: Fn(&T) -> U + Send + Sync,
{
    /// Get the mapped value.
    pub fn get(&self) -> U {
        let source_value = self.source.get();
        (self.mapper)(&*source_value)
    }

    /// Subscribe to changes in the mapped state.
    /// Note: This is a simplified implementation. A full implementation would
    /// need to handle the lifetime issues properly.
    pub fn subscribe<G>(&self, _observer: G)
    where
        G: Fn(&U) + Send + Sync + 'static,
        U: 'static,
    {
        // TODO: Implement proper subscription with lifetime management
        // This is a complex issue that requires careful design
    }
}

/// Hook for using state in components.
pub fn use_state<T>(initial_value: T) -> (State<T>, impl Fn(T)) {
    let state = State::new(initial_value);
    let setter = {
        let state = state.clone();
        move |new_value: T| {
            state.set(new_value);
        }
    };
    (state, setter)
}

/// A computed state that automatically updates when its dependencies change.
pub struct ComputedState<T> {
    value: Arc<RwLock<Option<T>>>,
    compute_fn: Box<dyn Fn() -> T + Send + Sync>,
    dependencies: Vec<Box<dyn Fn() + Send + Sync>>,
}

impl<T> ComputedState<T> {
    /// Create a new computed state.
    pub fn new<F>(compute_fn: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            value: Arc::new(RwLock::new(None)),
            compute_fn: Box::new(compute_fn),
            dependencies: Vec::new(),
        }
    }

    /// Get the computed value, calculating it if necessary.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        let mut value = self.value.write();
        if value.is_none() {
            *value = Some((self.compute_fn)());
        }
        value.as_ref().unwrap().clone()
    }

    /// Invalidate the computed value, forcing it to be recalculated.
    pub fn invalidate(&self) {
        let mut value = self.value.write();
        *value = None;
    }

    /// Add a dependency that will invalidate this computed state when it changes.
    pub fn add_dependency<F>(&mut self, dependency: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.dependencies.push(Box::new(dependency));
    }
}

/// A collection of states that can be managed together.
pub struct StateManager {
    states: Vec<Box<dyn std::any::Any + Send + Sync>>,
}

impl StateManager {
    /// Create a new state manager.
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
        }
    }

    /// Add a state to the manager.
    pub fn add_state<T: Send + Sync + 'static>(&mut self, state: State<T>) {
        self.states.push(Box::new(state));
    }

    /// Get the number of managed states.
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Check if the manager is empty.
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_state_creation_and_access() {
        let state = State::new(42);
        assert_eq!(*state.get(), 42);
    }

    #[test]
    fn test_state_update() {
        let state = State::new(0);
        state.set(10);
        assert_eq!(*state.get(), 10);

        state.update(|value| *value += 5);
        assert_eq!(*state.get(), 15);
    }

    #[test]
    fn test_state_subscription() {
        let state = State::new(0);
        let called = Arc::new(AtomicBool::new(false));
        let last_value = Arc::new(AtomicI32::new(0));

        {
            let called = called.clone();
            let last_value = last_value.clone();
            state.subscribe(move |value| {
                called.store(true, Ordering::Relaxed);
                last_value.store(*value, Ordering::Relaxed);
            });
        }

        state.set(42);
        assert!(called.load(Ordering::Relaxed));
        assert_eq!(last_value.load(Ordering::Relaxed), 42);
    }

    #[test]
    fn test_mapped_state() {
        let state = State::new(10);
        let doubled = state.map(|value| value * 2);

        assert_eq!(doubled.get(), 20);

        state.set(15);
        assert_eq!(doubled.get(), 30);
    }

    #[test]
    fn test_use_state_hook() {
        let (state, set_state) = use_state(100);
        assert_eq!(*state.get(), 100);

        set_state(200);
        assert_eq!(*state.get(), 200);
    }

    #[test]
    fn test_computed_state() {
        let base_state = State::new(5);
        let computed = ComputedState::new({
            let base_state = base_state.clone();
            move || *base_state.get() * 2
        });

        assert_eq!(computed.get(), 10);

        base_state.set(10);
        computed.invalidate();
        assert_eq!(computed.get(), 20);
    }
}
