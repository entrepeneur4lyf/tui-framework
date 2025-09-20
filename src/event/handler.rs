//! Event handler traits and implementations.

use crate::error::Result;
use crate::event::Event;
use async_trait::async_trait;
use std::sync::Arc;

/// Trait for handling events.
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event.
    async fn handle(&self, event: Event) -> Result<()>;
}

/// A boxed event handler for dynamic dispatch.
pub type BoxedEventHandler = Box<dyn EventHandler>;

/// An Arc-wrapped event handler for shared ownership.
pub type SharedEventHandler = Arc<dyn EventHandler>;

/// Event handler that wraps a closure.
pub struct ClosureEventHandler<F>
where
    F: Fn(Event) -> Result<()> + Send + Sync,
{
    handler: F,
}

impl<F> ClosureEventHandler<F>
where
    F: Fn(Event) -> Result<()> + Send + Sync,
{
    /// Create a new closure event handler.
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait]
impl<F> EventHandler for ClosureEventHandler<F>
where
    F: Fn(Event) -> Result<()> + Send + Sync,
{
    async fn handle(&self, event: Event) -> Result<()> {
        (self.handler)(event)
    }
}

/// Event handler that wraps an async closure.
pub struct AsyncClosureEventHandler<F, Fut>
where
    F: Fn(Event) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<()>> + Send,
{
    handler: F,
}

impl<F, Fut> AsyncClosureEventHandler<F, Fut>
where
    F: Fn(Event) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<()>> + Send,
{
    /// Create a new async closure event handler.
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait]
impl<F, Fut> EventHandler for AsyncClosureEventHandler<F, Fut>
where
    F: Fn(Event) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<()>> + Send,
{
    async fn handle(&self, event: Event) -> Result<()> {
        (self.handler)(event).await
    }
}

/// Composite event handler that can handle multiple handlers.
pub struct CompositeEventHandler {
    handlers: Vec<SharedEventHandler>,
}

impl CompositeEventHandler {
    /// Create a new composite event handler.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Add a handler to the composite.
    pub fn add_handler(&mut self, handler: SharedEventHandler) {
        self.handlers.push(handler);
    }

    /// Remove a handler from the composite.
    pub fn remove_handler(&mut self, index: usize) -> Option<SharedEventHandler> {
        if index < self.handlers.len() {
            Some(self.handlers.remove(index))
        } else {
            None
        }
    }

    /// Get the number of handlers.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Check if the composite is empty.
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}

impl Default for CompositeEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventHandler for CompositeEventHandler {
    async fn handle(&self, event: Event) -> Result<()> {
        for handler in &self.handlers {
            handler.handle(event.clone()).await?;
        }
        Ok(())
    }
}

/// Conditional event handler that only handles events matching a condition.
pub struct ConditionalEventHandler<F, H>
where
    F: Fn(&Event) -> bool + Send + Sync,
    H: EventHandler,
{
    condition: F,
    handler: H,
}

impl<F, H> ConditionalEventHandler<F, H>
where
    F: Fn(&Event) -> bool + Send + Sync,
    H: EventHandler,
{
    /// Create a new conditional event handler.
    pub fn new(condition: F, handler: H) -> Self {
        Self { condition, handler }
    }
}

#[async_trait]
impl<F, H> EventHandler for ConditionalEventHandler<F, H>
where
    F: Fn(&Event) -> bool + Send + Sync,
    H: EventHandler,
{
    async fn handle(&self, event: Event) -> Result<()> {
        if (self.condition)(&event) {
            self.handler.handle(event).await
        } else {
            Ok(())
        }
    }
}

/// Helper functions for creating event handlers.
pub mod handlers {
    use super::*;
    use crate::event::{KeyEvent, MouseEvent};

    /// Create a handler for key events.
    pub fn on_key<F>(handler: F) -> impl EventHandler
    where
        F: Fn(KeyEvent) -> Result<()> + Send + Sync + 'static,
    {
        ClosureEventHandler::new(move |event| {
            if let Event::Key(key_event) = event {
                handler(key_event)
            } else {
                Ok(())
            }
        })
    }

    /// Create a handler for mouse events.
    pub fn on_mouse<F>(handler: F) -> impl EventHandler
    where
        F: Fn(MouseEvent) -> Result<()> + Send + Sync + 'static,
    {
        ClosureEventHandler::new(move |event| {
            if let Event::Mouse(mouse_event) = event {
                handler(mouse_event)
            } else {
                Ok(())
            }
        })
    }

    /// Create a handler for specific key presses.
    pub fn on_key_press<F>(key: crate::event::types::NcKey, handler: F) -> impl EventHandler
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        ClosureEventHandler::new(move |event| {
            if let Event::Key(key_event) = event {
                if key_event.key == key {
                    handler()
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::types::NcKey;
    use crate::event::{Event, KeyEvent};

    struct TestHandler {
        called: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }

    impl TestHandler {
        fn new() -> (Self, std::sync::Arc<std::sync::atomic::AtomicBool>) {
            let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            (
                Self {
                    called: called.clone(),
                },
                called,
            )
        }
    }

    #[async_trait]
    impl EventHandler for TestHandler {
        async fn handle(&self, _event: Event) -> Result<()> {
            self.called
                .store(true, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_event_handler() {
        let (handler, called) = TestHandler::new();
        let event = Event::Key(KeyEvent::new(NcKey::Enter));

        handler.handle(event).await.unwrap();
        assert!(called.load(std::sync::atomic::Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_composite_handler() {
        let mut composite = CompositeEventHandler::new();
        let (handler1, called1) = TestHandler::new();
        let (handler2, called2) = TestHandler::new();

        composite.add_handler(Arc::new(handler1));
        composite.add_handler(Arc::new(handler2));

        let event = Event::Key(KeyEvent::new(NcKey::Enter));
        composite.handle(event).await.unwrap();

        assert!(called1.load(std::sync::atomic::Ordering::Relaxed));
        assert!(called2.load(std::sync::atomic::Ordering::Relaxed));
    }
}
