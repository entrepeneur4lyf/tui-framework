//! Convenient re-exports for common use cases.
//!
//! This module provides a convenient way to import the most commonly used
//! types and traits from the framework. Import this to get started quickly
//! with all the essential components.
//!
//! ## Usage
//!
//! ```rust
//! use tui_framework::prelude::*;
//! ```
//!
//! This gives you access to:
//!
//! ### Core Types
//! - [`App`] - Main application struct for running TUI apps
//! - [`Component`] - Trait for creating custom components
//! - [`ComponentId`] - Unique identifier for components
//! - [`Error`], [`Result`] - Error handling types
//!
//! ### Reactive System
//! - [`State`] - Reactive state container
//! - [`use_state`] - Hook for managing component state
//! - [`use_effect`] - Hook for side effects and lifecycle
//! - [`use_memo`] - Hook for memoizing expensive computations
//! - [`use_context`] - Hook for accessing shared context
//! - [`Context`], [`ContextProvider`] - Context system for data sharing
//!
//! ### Event System
//! - [`Event`] - Base event type
//! - [`KeyEvent`] - Keyboard input events
//! - [`MouseEvent`] - Mouse input events
//! - [`FocusEvent`] - Focus change events
//! - [`EventHandler`] - Trait for handling events
//!
//! ### Styling System
//! - [`Color`] - Color representation and utilities
//! - [`Style`] - Style properties for components
//! - [`StyleBuilder`] - Builder for creating styles
//! - [`Theme`] - Theme system for consistent styling
//!
//! ### Layout System
//! - [`Layout`] - Layout computation engine
//! - [`FlexDirection`] - Flexbox direction (row, column)
//! - [`JustifyContent`] - Main axis alignment
//! - [`AlignItems`] - Cross axis alignment
//! - [`Position`], [`Rect`], [`Size`] - Geometric types
//!
//! ### Widget System
//! - [`Button`] - Interactive button widget
//! - [`Input`] - Text input widget
//! - [`Text`] - Text display widget
//! - [`List`], [`ListItem`] - List widgets for collections
//! - [`Container`], [`Div`] - Layout containers
//! - [`Widget`] - Base widget trait
//! - [`SelectionMode`] - Selection behavior for lists
//!
//! ### Virtual DOM
//! - [`VirtualNode`] - Virtual DOM node representation
//! - [`RenderContext`] - Context for rendering operations
//! - [`Renderer`] - Rendering engine
//! - Node creation functions: [`button`], [`div`], [`text`], [`input`], [`list`], [`container`]
//!
//! ### Macros
//! - [`component`] - Macro for creating components
//! - [`Theme`] - Derive macro for theme structs
//! - [`css`] - Macro for CSS-like styling
//! - [`jsx`] - JSX-like syntax (placeholder)
//! - [`use_hooks`] - Hook organization macro
//!
//! ### External Dependencies
//! - Common async types from [`tokio`] and [`futures`]
//! - [`async_trait`] for async trait implementations
//! - [`HashMap`], [`Arc`] from standard library
//! - Optional [`libnotcurses_sys`] types when using notcurses backend
//!
//! ## Example
//!
//! ```rust,no_run
//! use tui_framework::prelude::*;
//!
//! #[derive(Clone)]
//! struct MyApp {
//!     base: BaseComponent,
//! }
//!
//! impl MyApp {
//!     fn new() -> Self {
//!         Self {
//!             base: BaseComponent::new("MyApp"),
//!         }
//!     }
//! }
//!
//! #[async_trait]
//! impl Component for MyApp {
//!     fn id(&self) -> ComponentId { self.base.id() }
//!     fn name(&self) -> &str { self.base.name() }
//!     fn as_any(&self) -> &dyn std::any::Any { self }
//!     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
//!
//!     async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
//!         let (count, set_count) = use_state(0);
//!
//!         Ok(div()
//!             .child(text(&format!("Count: {}", count.get())))
//!             .child(button("Click me!")
//!                 .on_click(move |_| set_count.set(*count.get() + 1))))
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     App::new()
//!         .title("My App")
//!         .component(MyApp::new())
//!         .run()
//!         .await
//! }
//! ```

// Core traits and types
pub use crate::app::App;
pub use crate::component::{Component, ComponentId};
pub use crate::error::{Error, Result};

// Reactive system
pub use crate::reactive::{
    Context, ContextProvider, State, use_context, use_effect, use_memo, use_state,
};

// Event system
pub use crate::event::{Event, EventHandler, FocusEvent, KeyEvent, MouseEvent};

// Styling system
pub use crate::style::{Color, Style, StyleBuilder, Theme};

// Layout system
pub use crate::layout::{AlignItems, FlexDirection, JustifyContent, Layout, Position, Rect, Size};

// Widget system
pub use crate::widget::{
    Button, Container, Div, Input, List, ListItem, SelectionMode, Text, Widget,
};

// Render system
pub use crate::render::vdom::nodes::{button, container, div, input, list, text};
pub use crate::render::{RenderContext, Renderer, VirtualNode};

// Macros
pub use tui_framework_macros::{Theme, component, css, jsx, use_hooks};

// Common standard library types
pub use std::collections::HashMap;
pub use std::sync::Arc;

// Async traits
pub use async_trait::async_trait;

// Commonly used external types
pub use futures;
pub use tokio;

// Re-export libnotcurses types that users might need
#[cfg(feature = "notcurses")]
pub use libnotcurses_sys::{NcKey, NcResult};
