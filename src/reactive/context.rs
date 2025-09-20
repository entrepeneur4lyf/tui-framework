//! Context system for sharing state across components.

use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

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

/// Context stack for managing nested contexts.
pub struct ContextStack {
    stack: Vec<Context>,
}

impl ContextStack {
    /// Create a new empty context stack.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a new context onto the stack.
    pub fn push(&mut self, context: Context) {
        self.stack.push(context);
    }

    /// Pop the top context from the stack.
    pub fn pop(&mut self) -> Option<Context> {
        self.stack.pop()
    }

    /// Get a reference to the current (top) context.
    pub fn current(&self) -> Option<&Context> {
        self.stack.last()
    }

    /// Get a value of type T from the context stack, searching from top to bottom.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        // Search from the top of the stack (most recent context) down
        for context in self.stack.iter().rev() {
            if let Some(value) = context.get::<T>() {
                return Some(value);
            }
        }
        None
    }
}

impl Default for ContextStack {
    fn default() -> Self {
        Self::new()
    }
}

// Thread-local context stack
thread_local! {
    static CONTEXT_STACK: RefCell<ContextStack> = RefCell::new(ContextStack::new());
}

/// Push a context onto the current thread's context stack.
pub fn push_context(context: Context) {
    CONTEXT_STACK.with(|stack| {
        stack.borrow_mut().push(context);
    });
}

/// Pop a context from the current thread's context stack.
pub fn pop_context() -> Option<Context> {
    CONTEXT_STACK.with(|stack| stack.borrow_mut().pop())
}

/// Get the current context from the stack.
pub fn current_context() -> Option<Context> {
    CONTEXT_STACK.with(|stack| stack.borrow().current().cloned())
}

/// Hook for accessing context values.
/// This searches up the context stack for a value of type T.
pub fn use_context<T: Send + Sync + 'static>() -> Option<Arc<T>> {
    CONTEXT_STACK.with(|stack| stack.borrow().get::<T>())
}

/// A context guard that automatically pops the context when dropped.
pub struct ContextGuard {
    _phantom: std::marker::PhantomData<()>,
}

impl ContextGuard {
    /// Create a new context guard that pushes the context onto the stack.
    pub fn new(context: Context) -> Self {
        push_context(context);
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        pop_context();
    }
}

/// Convenience macro for providing context to a block of code.
#[macro_export]
macro_rules! with_context {
    ($context:expr, $block:block) => {{
        let _guard = $crate::reactive::context::ContextGuard::new($context);
        $block
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestData {
        value: i32,
    }

    #[test]
    fn test_context_basic() {
        let context = Context::new();
        let test_data = TestData { value: 42 };
        context.insert(test_data.clone());

        let retrieved: Option<Arc<TestData>> = context.get();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 42);
    }

    #[test]
    fn test_context_stack() {
        let mut stack = ContextStack::new();

        // Create first context
        let context1 = Context::new();
        context1.insert(TestData { value: 1 });
        stack.push(context1);

        // Create second context
        let context2 = Context::new();
        context2.insert(TestData { value: 2 });
        stack.push(context2);

        // Should get the most recent value
        let retrieved: Option<Arc<TestData>> = stack.get();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 2);

        // Pop and try again
        stack.pop();
        let retrieved: Option<Arc<TestData>> = stack.get();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 1);
    }

    #[test]
    fn test_use_context_hook() {
        // Clear any existing context
        while pop_context().is_some() {}

        // Should return None when no context
        let result: Option<Arc<TestData>> = use_context();
        assert!(result.is_none());

        // Push a context
        let context = Context::new();
        context.insert(TestData { value: 123 });
        push_context(context);

        // Should now find the value
        let result: Option<Arc<TestData>> = use_context();
        assert!(result.is_some());
        assert_eq!(result.unwrap().value, 123);

        // Clean up
        pop_context();
    }

    #[test]
    fn test_context_guard() {
        // Clear any existing context
        while pop_context().is_some() {}

        {
            let context = Context::new();
            context.insert(TestData { value: 456 });
            let _guard = ContextGuard::new(context);

            // Should find the value while guard is alive
            let result: Option<Arc<TestData>> = use_context();
            assert!(result.is_some());
            assert_eq!(result.unwrap().value, 456);
        } // Guard drops here

        // Should not find the value after guard is dropped
        let result: Option<Arc<TestData>> = use_context();
        assert!(result.is_none());
    }
}
