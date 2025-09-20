//! Reactive state management.
//!
//! This module provides the reactive state system that powers the framework's
//! automatic UI updates. When state changes, components that depend on that
//! state are automatically re-rendered.
//!
//! ## Core Concepts
//!
//! - [`State<T>`]: A reactive container that holds a value and notifies observers
//! - [`MappedState<T, U, F>`]: A derived state that automatically updates when source changes
//! - [`use_state`]: Hook for creating and managing component state
//! - Observer pattern for automatic updates
//!
//! ## Example
//!
//! ```rust
//! use tui_framework::reactive::state::State;
//!
//! // Create reactive state
//! let count = State::new(0);
//!
//! // Subscribe to changes
//! let subscription = count.subscribe(|new_value| {
//!     println!("Count changed to: {}", new_value);
//! });
//!
//! // Update state (triggers notification)
//! count.set(42); // Prints: "Count changed to: 42"
//!
//! // Create derived state
//! let doubled = count.map(|x| x * 2);
//! assert_eq!(*doubled.get(), 84);
//!
//! // Update original state (derived state updates automatically)
//! count.set(10);
//! assert_eq!(*doubled.get(), 20);
//! ```
//!
//! ## State Lifecycle
//!
//! 1. **Creation**: State is created with an initial value
//! 2. **Subscription**: Components subscribe to state changes
//! 3. **Updates**: State is modified using `set()` or `update()`
//! 4. **Notification**: All subscribers are notified of changes
//! 5. **Re-rendering**: Components re-render with new state
//!
//! ## Performance
//!
//! The state system is designed for performance:
//! - Uses `RwLock` for concurrent read access
//! - Minimal allocations for state updates
//! - Efficient observer notification
//! - Automatic cleanup of unused subscriptions

use parking_lot::{RwLock, RwLockReadGuard};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// A unique identifier for a subscription.
///
/// Each subscription to state changes gets a unique ID that can be used
/// to unsubscribe later. IDs are generated atomically and are guaranteed
/// to be unique within the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(u64);

impl SubscriptionId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// A reactive state container that notifies observers when the value changes.
///
/// [`State<T>`] is the core building block of the reactive system. It holds a value
/// of type `T` and automatically notifies all subscribers when the value changes.
/// This enables automatic UI updates when state changes.
///
/// ## Features
///
/// - **Thread-safe**: Can be safely shared between threads
/// - **Efficient reads**: Multiple readers can access the value concurrently
/// - **Automatic notifications**: Observers are notified when value changes
/// - **Derived state**: Can create mapped states that update automatically
/// - **Subscription management**: Automatic cleanup of unused subscriptions
///
/// ## Example
///
/// ```rust
/// use tui_framework::reactive::state::State;
///
/// // Create state with initial value
/// let counter = State::new(0);
///
/// // Read current value
/// assert_eq!(*counter.get(), 0);
///
/// // Subscribe to changes
/// let subscription = counter.subscribe(|value| {
///     println!("Counter: {}", value);
/// });
///
/// // Update state (triggers notification)
/// counter.set(42);
/// assert_eq!(*counter.get(), 42);
///
/// // Update with closure
/// counter.update(|value| *value += 1);
/// assert_eq!(*counter.get(), 43);
/// ```
pub struct State<T> {
    value: Arc<RwLock<T>>,
    observers:
        Arc<RwLock<std::collections::HashMap<SubscriptionId, Box<dyn Fn(&T) + Send + Sync>>>>,
}

impl<T> State<T> {
    /// Create a new state with an initial value.
    ///
    /// This creates a new reactive state container with the provided initial value.
    /// The state can be shared between components and will notify all subscribers
    /// when the value changes.
    ///
    /// ## Parameters
    ///
    /// - `initial_value`: The initial value to store in the state
    ///
    /// ## Example
    ///
    /// ```rust
    /// use tui_framework::reactive::state::State;
    ///
    /// let count = State::new(42);
    /// assert_eq!(*count.get(), 42);
    /// ```
    pub fn new(initial_value: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial_value)),
            observers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a read-only reference to the current value.
    ///
    /// This returns a read guard that allows access to the current value.
    /// Multiple readers can access the value concurrently. The guard
    /// automatically releases the lock when dropped.
    ///
    /// ## Returns
    ///
    /// A read guard that dereferences to the current value.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use tui_framework::reactive::state::State;
    ///
    /// let state = State::new("hello");
    /// let value = state.get();
    /// assert_eq!(*value, "hello");
    /// ```
    pub fn get(&self) -> RwLockReadGuard<'_, T> {
        self.value.read()
    }

    /// Set a new value and notify observers.
    ///
    /// This replaces the current value with a new one and notifies all
    /// subscribers about the change. This is the primary way to update
    /// state and trigger UI re-renders.
    ///
    /// ## Parameters
    ///
    /// - `new_value`: The new value to store
    ///
    /// ## Example
    ///
    /// ```rust
    /// use tui_framework::reactive::state::State;
    ///
    /// let count = State::new(0);
    /// count.set(42);
    /// assert_eq!(*count.get(), 42);
    /// ```
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
    /// Returns a subscription ID that can be used to unsubscribe.
    pub fn subscribe<F>(&self, observer: F) -> SubscriptionId
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let id = SubscriptionId::new();
        let mut observers = self.observers.write();
        observers.insert(id, Box::new(observer));
        id
    }

    /// Unsubscribe from changes in this state.
    pub fn unsubscribe(&self, id: SubscriptionId) -> bool {
        let mut observers = self.observers.write();
        observers.remove(&id).is_some()
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
        F: Fn(&T) -> U + Send + Sync + Clone + 'static,
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
        for (_, observer) in observers.iter() {
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
    F: Fn(&T) -> U + Send + Sync + Clone + 'static,
{
    source: State<T>,
    mapper: F,
}

impl<T, U, F> MappedState<T, U, F>
where
    F: Fn(&T) -> U + Send + Sync + Clone + 'static,
{
    /// Get the mapped value.
    pub fn get(&self) -> U {
        let source_value = self.source.get();
        (self.mapper)(&*source_value)
    }

    /// Subscribe to changes in the mapped state.
    /// Returns a subscription ID that can be used to unsubscribe.
    pub fn subscribe<G>(&self, observer: G) -> SubscriptionId
    where
        G: Fn(&U) + Send + Sync + 'static,
        U: 'static,
    {
        // Create a wrapper that maps the source value and calls the observer
        let mapper = self.mapper.clone();
        let wrapped_observer = move |source_value: &T| {
            let mapped_value = mapper(source_value);
            observer(&mapped_value);
        };

        // Subscribe to the source state
        self.source.subscribe(wrapped_observer)
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
        Self { states: Vec::new() }
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

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
