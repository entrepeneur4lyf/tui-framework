//! # TUI Framework
//!
//! A React-like TUI framework for Rust that combines the developer experience of React
//! with the performance of libnotcurses. Build beautiful, interactive terminal applications
//! with familiar patterns and modern tooling.
//!
//! ## Features
//!
//! - **🚀 React-like API**: Familiar hooks, components, and patterns from React
//! - **⚡ High Performance**: Built on libnotcurses for efficient terminal rendering
//! - **🔒 Type Safety**: Leverages Rust's type system for safe, reliable applications
//! - **🎨 Modern Styling**: CSS-like styling system with themes and responsive design
//! - **🧩 Rich Components**: Comprehensive set of built-in widgets and components
//! - **🔄 Reactive State**: Powerful state management with automatic updates
//! - **📱 Responsive Layout**: Flexbox-inspired layout system that adapts to terminal size
//! - **🎯 Event Handling**: Type-safe event system with keyboard and mouse support
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tui-framework = "0.1.0"
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! Create a simple counter application:
//!
//! ```rust,no_run
//! use tui_framework::prelude::*;
//! use tui_framework::render::vdom::nodes::*;
//!
//! #[derive(Clone)]
//! struct CounterComponent {
//!     base: BaseComponent,
//! }
//!
//! impl CounterComponent {
//!     fn new() -> Self {
//!         Self {
//!             base: BaseComponent::new("Counter"),
//!         }
//!     }
//! }
//!
//! #[async_trait]
//! impl Component for CounterComponent {
//!     fn id(&self) -> ComponentId { self.base.id() }
//!     fn name(&self) -> &str { self.base.name() }
//!     fn as_any(&self) -> &dyn std::any::Any { self }
//!     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
//!
//!     async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
//!         let (count, set_count) = use_state(0);
//!
//!         Ok(div()
//!             .style("padding: 2; border: 1px solid blue;")
//!             .child(text(&format!("Count: {}", count.get())))
//!             .child(
//!                 button("Increment")
//!                     .on_click(move |_| {
//!                         set_count.set(*count.get() + 1);
//!                     })
//!             ))
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let app = App::new()
//!         .title("Counter App")
//!         .component(CounterComponent::new());
//!
//!     app.run().await
//! }
//! ```
//!
//! ## Core Concepts
//!
//! ### Components
//!
//! Components are the building blocks of your application. They implement the [`Component`] trait
//! and can have state, props, and lifecycle methods:
//!
//! ```rust,no_run
//! # use tui_framework::prelude::*;
//! # use tui_framework::render::vdom::nodes::*;
//! #[derive(Clone)]
//! struct MyComponent {
//!     base: BaseComponent,
//!     title: String,
//! }
//!
//! impl MyComponent {
//!     fn new(title: String) -> Self {
//!         Self {
//!             base: BaseComponent::new("MyComponent"),
//!             title,
//!         }
//!     }
//! }
//!
//! #[async_trait]
//! impl Component for MyComponent {
//!     fn id(&self) -> ComponentId { self.base.id() }
//!     fn name(&self) -> &str { self.base.name() }
//!     fn as_any(&self) -> &dyn std::any::Any { self }
//!     fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
//!
//!     async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
//!         Ok(div()
//!             .child(text(&self.title))
//!             .child(text("Hello, World!")))
//!     }
//! }
//! ```
//!
//! ### Hooks
//!
//! Hooks let you use state and other features in your components:
//!
//! - [`use_state`]: Manage component state
//! - [`use_effect`]: Handle side effects and lifecycle
//! - [`use_memo`]: Memoize expensive computations
//! - [`use_context`]: Access shared context data
//!
//! ```rust,no_run
//! # use tui_framework::prelude::*;
//! # use tui_framework::render::vdom::nodes::*;
//! async fn my_component_render(_context: &RenderContext) -> Result<VirtualNode> {
//!     // State management
//!     let (count, set_count) = use_state(0);
//!
//!     // Side effects
//!     use_effect(move || {
//!         println!("Count changed to: {}", count.get());
//!     }, vec![count.get()]);
//!
//!     // Memoized computation
//!     let doubled = use_memo(move || *count.get() * 2, vec![count.get()]);
//!
//!     Ok(div()
//!         .child(text(&format!("Count: {}, Doubled: {}", count.get(), doubled.get()))))
//! }
//! ```
//!
//! ### Widgets
//!
//! The framework provides a comprehensive set of built-in widgets:
//!
//! - **Layout**: [`Div`], [`Container`] for organizing content
//! - **Text**: [`Text`] for displaying text with styling and wrapping
//! - **Input**: [`Button`], [`Input`] for user interaction
//! - **Lists**: List and ListItem widgets for displaying collections
//! - **Tables**: Table widget for tabular data with sorting and selection
//! - **Menus**: Menu and Dropdown widgets for navigation and selection
//! - **Dialogs**: Modal widget for overlays and confirmations
//! - **Progress**: ProgressBar widget for showing progress and loading states
//!
//! ### Styling
//!
//! Style your components with CSS-like syntax:
//!
//! ```rust,no_run
//! # use tui_framework::prelude::*;
//! # use tui_framework::render::vdom::nodes::*;
//! # async fn example() -> Result<VirtualNode> {
//! Ok(div()
//!     .style("
//!         background-color: #1e1e1e;
//!         color: #ffffff;
//!         padding: 2;
//!         border: 1px solid #444444;
//!         border-radius: 4px;
//!     ")
//!     .child(text("Styled content")))
//! # }
//! ```
//!
//! ## Examples
//!
//! The framework includes several example applications:
//!
//! - **Hello World**: Basic component and rendering (`examples/hello_world.rs`)
//! - **Counter**: State management and events (`examples/counter.rs`)
//! - **Calculator**: Complex state and component composition (`examples/calculator.rs`)
//! - **Todo App**: Real-world application patterns (`examples/todo_app.rs`)
//! - **Responsive Layout**: Adaptive layouts and performance (`examples/responsive_layout.rs`)
//!
//! Run an example with:
//!
//! ```bash
//! cargo run --example hello_world
//! ```
//!
//! ## Architecture
//!
//! The framework is built around several core systems:
//!
//! - **Component System**: Reusable UI elements with state and lifecycle
//! - **Reactive System**: Hooks and state management with automatic updates
//! - **Virtual DOM**: Efficient diffing and rendering system
//! - **Event System**: Type-safe event handling with propagation
//! - **Layout Engine**: Flexbox-inspired responsive layout system
//! - **Styling System**: CSS-like styling with themes and responsive design
//! - **Widget Library**: Comprehensive set of built-in components
//! - **Rendering Backend**: High-performance libnotcurses integration

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::type_complexity)]

// Re-export commonly used types and traits
#[cfg(feature = "notcurses")]
pub use libnotcurses_sys;

// Core modules
pub mod app;
pub mod component;
pub mod error;
pub mod prelude;

// Reactive system
pub mod reactive;

// Rendering system
pub mod render;

// Widget system
pub mod widget;

// Event system
pub mod event;

// Styling system
pub mod style;

// Layout system
pub mod layout;

// Utilities
pub mod utils;

// Re-export macros
pub use tui_framework_macros::*;

/// The main error type for the framework
pub use error::{Error, Result};

pub use app::App;
/// Core traits and types
pub use component::{Component, ComponentId};

/// Reactive system exports
pub use reactive::{Context, ContextProvider, State, use_context, use_effect, use_memo, use_state};

/// Event system exports
pub use event::{Event, EventHandler, FocusEvent, KeyEvent, MouseEvent};

/// Styling exports
pub use style::{Color, Style, StyleBuilder, Theme};

/// Layout exports
pub use layout::{AlignItems, FlexDirection, JustifyContent, Layout, Position, Rect, Size};

/// Widget exports
pub use widget::{Button, Container, Div, Input, Text, Widget};

/// Render system exports
pub use render::{RenderContext, Renderer, VirtualNode};
