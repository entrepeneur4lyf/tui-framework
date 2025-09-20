//! Component system and traits.
//!
//! This module provides the core component system for building TUI applications.
//! Components are the fundamental building blocks that encapsulate UI logic,
//! state, and rendering behavior.
//!
//! ## Overview
//!
//! The component system is built around the [`Component`] trait, which defines
//! the interface for all UI elements. Components can:
//!
//! - Maintain internal state
//! - Respond to events
//! - Render themselves to virtual DOM nodes
//! - Have lifecycle methods for setup and cleanup
//! - Be composed together to build complex UIs
//!
//! ## Component Lifecycle
//!
//! Components have several lifecycle methods:
//!
//! 1. **Creation**: Components are created with `new()` methods
//! 2. **Mounting**: [`Component::on_mount`] is called when added to the UI
//! 3. **Rendering**: [`Component::render`] is called to generate virtual DOM
//! 4. **Updates**: Components re-render when state changes
//! 5. **Unmounting**: [`Component::on_unmount`] is called when removed
//!
//! ## Example
//!
//! ```rust,no_run
//! use tui_framework::prelude::*;
//! use tui_framework::render::vdom::nodes::*;
//!
//! #[derive(Clone)]
//! struct CounterComponent {
//!     base: BaseComponent,
//!     initial_value: i32,
//! }
//!
//! impl CounterComponent {
//!     fn new(initial_value: i32) -> Self {
//!         Self {
//!             base: BaseComponent::new("Counter"),
//!             initial_value,
//!         }
//!     }
//! }
//!
//! #[async_trait]
//! impl Component for CounterComponent {
//!     fn id(&self) -> ComponentId {
//!         self.base.id()
//!     }
//!
//!     fn name(&self) -> &str {
//!         self.base.name()
//!     }
//!
//!     fn as_any(&self) -> &dyn std::any::Any {
//!         self
//!     }
//!
//!     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//!         self
//!     }
//!
//!     async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
//!         let (count, set_count) = use_state(self.initial_value);
//!
//!         Ok(div()
//!             .style("padding: 1; border: 1px solid blue;")
//!             .child(text(&format!("Count: {}", count.get())))
//!             .child(
//!                 button("Increment")
//!                     .on_click(move |_| {
//!                         set_count.set(*count.get() + 1);
//!                     })
//!             )
//!             .child(
//!                 button("Reset")
//!                     .on_click(move |_| {
//!                         set_count.set(0);
//!                     })
//!             ))
//!     }
//!
//!     async fn on_mount(&mut self) -> Result<()> {
//!         println!("Counter component mounted with initial value: {}", self.initial_value);
//!         Ok(())
//!     }
//!
//!     async fn on_unmount(&mut self) -> Result<()> {
//!         println!("Counter component unmounted");
//!         Ok(())
//!     }
//! }
//! ```

use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use async_trait::async_trait;
use std::any::Any;
use std::fmt;
use uuid::Uuid;

/// Unique identifier for components.
///
/// Each component instance has a unique ID that persists for the lifetime
/// of the component. This is used internally for tracking components in
/// the virtual DOM and for debugging purposes.
///
/// ## Example
///
/// ```rust
/// use tui_framework::component::ComponentId;
///
/// let id1 = ComponentId::new();
/// let id2 = ComponentId::new();
/// assert_ne!(id1, id2); // Each ID is unique
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(Uuid);

impl ComponentId {
    /// Create a new unique component ID.
    ///
    /// This generates a new UUID v4 for the component. Each call
    /// to this method will return a different ID.
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
///
/// This trait defines the interface that all components must implement.
/// Components are the building blocks of TUI applications and encapsulate
/// UI logic, state management, and rendering behavior.
///
/// ## Required Methods
///
/// - [`id`](Component::id): Returns the component's unique identifier
/// - [`render`](Component::render): Renders the component to a virtual DOM node
/// - [`as_any`](Component::as_any): Enables downcasting to concrete types
/// - [`as_any_mut`](Component::as_any_mut): Enables mutable downcasting
///
/// ## Optional Methods
///
/// - [`name`](Component::name): Returns a human-readable name for debugging
/// - [`on_mount`](Component::on_mount): Called when the component is added to the UI
/// - [`on_unmount`](Component::on_unmount): Called when the component is removed
/// - [`on_props_changed`](Component::on_props_changed): Called when the component's props change
/// - [`on_state_changed`](Component::on_state_changed): Called when the component's state changes
///
/// ## Implementation Notes
///
/// Components must be `Send + Sync` to work with the async rendering system.
/// Most components should store a [`BaseComponent`] to handle common functionality.
///
/// ## Example
///
/// ```rust,no_run
/// use tui_framework::prelude::*;
/// use tui_framework::render::vdom::nodes::*;
///
/// #[derive(Clone)]
/// struct MyComponent {
///     base: BaseComponent,
///     message: String,
/// }
///
/// impl MyComponent {
///     fn new(message: String) -> Self {
///         Self {
///             base: BaseComponent::new("MyComponent"),
///             message,
///         }
///     }
/// }
///
/// #[async_trait]
/// impl Component for MyComponent {
///     fn id(&self) -> ComponentId {
///         self.base.id()
///     }
///
///     fn name(&self) -> &str {
///         self.base.name()
///     }
///
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
///
///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
///         self
///     }
///
///     async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
///         Ok(div()
///             .child(text(&self.message)))
///     }
/// }
/// ```
#[async_trait]
pub trait Component: Send + Sync {
    /// Get the component's unique identifier.
    ///
    /// This ID is used internally for tracking components and should
    /// remain constant for the lifetime of the component instance.
    fn id(&self) -> ComponentId;

    /// Get the component's name for debugging and development.
    ///
    /// This should return a human-readable name that helps identify
    /// the component type in logs and debugging tools.
    fn name(&self) -> &str {
        "Component"
    }

    /// Render the component to a virtual DOM node.
    ///
    /// This is the core method where components define their UI structure.
    /// It receives a [`RenderContext`] with theme and viewport information,
    /// and returns a [`VirtualNode`] representing the component's UI.
    ///
    /// ## Parameters
    ///
    /// - `context`: Rendering context with theme, viewport size, and debug info
    ///
    /// ## Returns
    ///
    /// A [`Result`] containing the virtual DOM node, or an error if rendering fails.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use tui_framework::prelude::*;
    /// # use tui_framework::render::vdom::nodes::*;
    /// # struct MyComponent;
    /// # #[async_trait]
    /// # impl Component for MyComponent {
    /// #     fn id(&self) -> ComponentId { ComponentId::new() }
    /// #     fn as_any(&self) -> &dyn std::any::Any { self }
    /// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    /// async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
    ///     let (count, set_count) = use_state(0);
    ///
    ///     Ok(div()
    ///         .style("padding: 1;")
    ///         .child(text(&format!("Count: {}", count.get())))
    ///         .child(button("Increment")
    ///             .on_click(move |_| set_count.set(*count.get() + 1))))
    /// }
    /// # }
    /// ```
    async fn render(&self, context: &RenderContext) -> Result<VirtualNode>;

    /// Called when the component is mounted to the UI.
    ///
    /// This lifecycle method is called once when the component is first
    /// added to the component tree. Use this for initialization that
    /// requires the component to be part of the UI.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use tui_framework::prelude::*;
    /// # struct MyComponent;
    /// # #[async_trait]
    /// # impl Component for MyComponent {
    /// #     fn id(&self) -> ComponentId { ComponentId::new() }
    /// #     fn as_any(&self) -> &dyn std::any::Any { self }
    /// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    /// #     async fn render(&self, _: &RenderContext) -> Result<VirtualNode> { todo!() }
    /// async fn on_mount(&mut self) -> Result<()> {
    ///     println!("Component {} mounted", self.name());
    ///     // Initialize resources, start timers, etc.
    ///     Ok(())
    /// }
    /// # }
    /// ```
    async fn on_mount(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the component is unmounted from the UI.
    ///
    /// This lifecycle method is called when the component is removed
    /// from the component tree. Use this for cleanup operations like
    /// stopping timers, closing connections, or freeing resources.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use tui_framework::prelude::*;
    /// # struct MyComponent;
    /// # #[async_trait]
    /// # impl Component for MyComponent {
    /// #     fn id(&self) -> ComponentId { ComponentId::new() }
    /// #     fn as_any(&self) -> &dyn std::any::Any { self }
    /// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    /// #     async fn render(&self, _: &RenderContext) -> Result<VirtualNode> { todo!() }
    /// async fn on_unmount(&mut self) -> Result<()> {
    ///     println!("Component {} unmounted", self.name());
    ///     // Clean up resources, stop timers, etc.
    ///     Ok(())
    /// }
    /// # }
    /// ```
    async fn on_unmount(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the component's props change.
    ///
    /// This method is called when external properties passed to the
    /// component are updated. Use this to respond to prop changes
    /// and update internal state accordingly.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use tui_framework::prelude::*;
    /// # struct MyComponent;
    /// # #[async_trait]
    /// # impl Component for MyComponent {
    /// #     fn id(&self) -> ComponentId { ComponentId::new() }
    /// #     fn as_any(&self) -> &dyn std::any::Any { self }
    /// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    /// #     async fn render(&self, _: &RenderContext) -> Result<VirtualNode> { todo!() }
    /// async fn on_props_changed(&mut self) -> Result<()> {
    ///     println!("Props changed for component {}", self.name());
    ///     // Update internal state based on new props
    ///     Ok(())
    /// }
    /// # }
    /// ```
    async fn on_props_changed(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the component's state changes.
    ///
    /// This method is called after the component's internal state
    /// has been updated. Use this for side effects that should
    /// happen after state changes.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use tui_framework::prelude::*;
    /// # struct MyComponent;
    /// # #[async_trait]
    /// # impl Component for MyComponent {
    /// #     fn id(&self) -> ComponentId { ComponentId::new() }
    /// #     fn as_any(&self) -> &dyn std::any::Any { self }
    /// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    /// #     async fn render(&self, _: &RenderContext) -> Result<VirtualNode> { todo!() }
    /// async fn on_state_changed(&mut self) -> Result<()> {
    ///     println!("State changed for component {}", self.name());
    ///     // Perform side effects after state update
    ///     Ok(())
    /// }
    /// # }
    /// ```
    async fn on_state_changed(&mut self) -> Result<()> {
        Ok(())
    }

    /// Get the component as [`Any`] for downcasting.
    ///
    /// This method enables runtime type checking and downcasting
    /// to concrete component types. This is used internally by
    /// the framework for type-safe component operations.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use tui_framework::prelude::*;
    /// # #[derive(Clone)]
    /// # struct MyComponent { base: BaseComponent }
    /// # impl MyComponent { fn new() -> Self { Self { base: BaseComponent::new("test") } } }
    /// # #[async_trait]
    /// # impl Component for MyComponent {
    /// #     fn id(&self) -> ComponentId { self.base.id() }
    /// #     fn as_any(&self) -> &dyn std::any::Any { self }
    /// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    /// #     async fn render(&self, _: &RenderContext) -> Result<VirtualNode> { todo!() }
    /// # }
    /// fn downcast_component(component: &dyn Component) -> Option<&MyComponent> {
    ///     component.as_any().downcast_ref::<MyComponent>()
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any;

    /// Get the component as mutable [`Any`] for downcasting.
    ///
    /// This method enables runtime type checking and mutable downcasting
    /// to concrete component types. This is used internally by
    /// the framework for type-safe mutable component operations.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use tui_framework::prelude::*;
    /// # #[derive(Clone)]
    /// # struct MyComponent { base: BaseComponent }
    /// # impl MyComponent { fn new() -> Self { Self { base: BaseComponent::new("test") } } }
    /// # #[async_trait]
    /// # impl Component for MyComponent {
    /// #     fn id(&self) -> ComponentId { self.base.id() }
    /// #     fn as_any(&self) -> &dyn std::any::Any { self }
    /// #     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    /// #     async fn render(&self, _: &RenderContext) -> Result<VirtualNode> { todo!() }
    /// # }
    /// fn downcast_component_mut(component: &mut dyn Component) -> Option<&mut MyComponent> {
    ///     component.as_any_mut().downcast_mut::<MyComponent>()
    /// }
    /// ```
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Base implementation for components.
///
/// [`BaseComponent`] provides a standard implementation of common component
/// functionality like ID generation and name storage. Most custom components
/// should include a `BaseComponent` field and delegate to it for basic operations.
///
/// ## Usage
///
/// ```rust
/// use tui_framework::prelude::*;
///
/// #[derive(Clone)]
/// struct MyComponent {
///     base: BaseComponent,
///     // ... other fields
/// }
///
/// impl MyComponent {
///     fn new() -> Self {
///         Self {
///             base: BaseComponent::new("MyComponent"),
///             // ... initialize other fields
///         }
///     }
/// }
///
/// #[async_trait]
/// impl Component for MyComponent {
///     fn id(&self) -> ComponentId {
///         self.base.id()  // Delegate to base
///     }
///
///     fn name(&self) -> &str {
///         self.base.name()  // Delegate to base
///     }
///
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
///
///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
///         self
///     }
///
///     async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
///         // Custom rendering logic
///         Ok(VirtualNode::empty())
///     }
/// }
/// ```
pub struct BaseComponent {
    id: ComponentId,
    name: String,
}

impl BaseComponent {
    /// Create a new base component with the given name.
    ///
    /// This generates a unique ID for the component and stores the name
    /// for debugging purposes.
    ///
    /// ## Parameters
    ///
    /// - `name`: A human-readable name for the component
    ///
    /// ## Example
    ///
    /// ```rust
    /// use tui_framework::component::BaseComponent;
    ///
    /// let base = BaseComponent::new("MyComponent");
    /// assert_eq!(base.name(), "MyComponent");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ComponentId::new(),
            name: name.into(),
        }
    }

    /// Get the component's unique identifier.
    ///
    /// Returns the unique ID that was generated when this component was created.
    pub fn id(&self) -> ComponentId {
        self.id
    }

    /// Get the component's name.
    ///
    /// Returns the human-readable name that was provided when creating the component.
    pub fn name(&self) -> &str {
        &self.name
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
