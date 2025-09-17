//! Component system and traits.

use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use async_trait::async_trait;
use std::any::Any;
use std::fmt;
use uuid::Uuid;

/// Unique identifier for components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(Uuid);

impl ComponentId {
    /// Create a new component ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The main trait for all UI components.
#[async_trait]
pub trait Component: Send + Sync {
    /// Get the component's unique identifier.
    fn id(&self) -> ComponentId;

    /// Get the component's name for debugging.
    fn name(&self) -> &str {
        "Component"
    }

    /// Render the component to a virtual node.
    async fn render(&self, context: &RenderContext) -> Result<VirtualNode>;

    /// Called when the component is mounted.
    async fn on_mount(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the component is unmounted.
    async fn on_unmount(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the component's props change.
    async fn on_props_changed(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the component's state changes.
    async fn on_state_changed(&mut self) -> Result<()> {
        Ok(())
    }

    /// Get the component as Any for downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Get the component as mutable Any for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Base implementation for components.
pub struct BaseComponent {
    id: ComponentId,
    name: String,
}

impl BaseComponent {
    /// Create a new base component.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ComponentId::new(),
            name: name.into(),
        }
    }
}

#[async_trait]
impl Component for BaseComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        // Default implementation returns an empty node
        Ok(VirtualNode::empty())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Trait for components that can have children.
pub trait Container: Component {
    /// Add a child component.
    fn add_child(&mut self, child: Box<dyn Component>);

    /// Remove a child component by ID.
    fn remove_child(&mut self, id: ComponentId) -> Option<Box<dyn Component>>;

    /// Get all children.
    fn children(&self) -> &[Box<dyn Component>];

    /// Get all children mutably.
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Component>>;
}

/// Trait for components that have props.
pub trait HasProps<P> {
    /// Get the component's props.
    fn props(&self) -> &P;

    /// Set the component's props.
    fn set_props(&mut self, props: P);
}

/// Trait for components that have state.
pub trait HasState<S> {
    /// Get the component's state.
    fn state(&self) -> &S;

    /// Set the component's state.
    fn set_state(&mut self, state: S);
}

/// Macro to help implement the Component trait.
#[macro_export]
macro_rules! impl_component {
    ($type:ty, $name:expr) => {
        #[async_trait::async_trait]
        impl Component for $type {
            fn id(&self) -> ComponentId {
                self.base.id()
            }

            fn name(&self) -> &str {
                $name
            }

            async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
                Ok(VirtualNode::empty())
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        base: BaseComponent,
    }

    impl TestComponent {
        fn new() -> Self {
            Self {
                base: BaseComponent::new("TestComponent"),
            }
        }
    }

    impl_component!(TestComponent, "TestComponent");



    #[tokio::test]
    async fn test_component_creation() {
        let component = TestComponent::new();
        assert_eq!(component.name(), "TestComponent");
    }

    #[test]
    fn test_component_id() {
        let id1 = ComponentId::new();
        let id2 = ComponentId::new();
        assert_ne!(id1, id2);
    }
}
