//! Context system for sharing state across components.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// A context for sharing state across components.
pub struct Context {
    data: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl Context {
    /// Create a new context.
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Insert a value into the context.
    pub fn insert<T: Send + Sync + 'static>(&self, value: T) {
        let mut data = self.data.write();
        data.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Get a value from the context.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let data = self.data.read();
        data.get(&TypeId::of::<T>())
            .and_then(|value| value.downcast_ref::<T>())
            .map(|value| Arc::new(unsafe { std::ptr::read(value) }))
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

/// A context provider that makes values available to child components.
pub struct ContextProvider<T> {
    value: T,
    context: Context,
}

impl<T: Send + Sync + Clone + 'static> ContextProvider<T> {
    /// Create a new context provider.
    pub fn new(value: T) -> Self {
        let context = Context::new();
        context.insert(value.clone());

        Self { value, context }
    }

    /// Get the provided value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the context.
    pub fn context(&self) -> &Context {
        &self.context
    }
}

/// Hook for accessing context values.
pub fn use_context<T: Send + Sync + 'static>() -> Option<Arc<T>> {
    // TODO: Get current context from component system
    None
}
