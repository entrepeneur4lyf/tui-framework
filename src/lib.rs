//! # TUI Framework
//!
//! A React-like TUI framework for Rust that combines the developer experience of React
//! with the performance of libnotcurses.
//!
//! ## Features
//!
//! - **React-like API**: Familiar hooks, components, and patterns from React
//! - **High Performance**: Built on libnotcurses for efficient terminal rendering
//! - **Type Safety**: Leverages Rust's type system for safe, reliable applications
//! - **Modern Styling**: CSS-like styling system with themes and responsive design
//! - **Rich Components**: Comprehensive set of built-in widgets and components
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tui_framework::prelude::*;
//!
//! #[component]
//! fn Counter() -> impl Component {
//!     let (count, set_count) = use_state(0);
//!     
//!     div()
//!         .child(text(&format!("Count: {}", count)))
//!         .child(button("Increment")
//!             .on_click(move |_| set_count(count + 1)))
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app = App::new()
//!         .title("Counter App")
//!         .component(Counter);
//!     
//!     app.run().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! The framework is built around several core concepts:
//!
//! - **Components**: Reusable UI elements that can have state and props
//! - **Hooks**: Functions that let you use state and lifecycle features
//! - **Virtual DOM**: Efficient diffing and rendering system
//! - **Event System**: Type-safe event handling with propagation
//! - **Styling**: CSS-like styling with themes and responsive design

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

// Re-export macros (when available)
// TODO: Implement macro system
// pub use tui_framework_macros::*;

/// The main error type for the framework
pub use error::{Error, Result};

/// Core traits and types
pub use component::{Component, ComponentId};
pub use app::App;

/// Reactive system exports
pub use reactive::{
    State, 
    use_state, 
    use_effect, 
    use_memo, 
    use_context,
    Context,
    ContextProvider,
};

/// Event system exports
pub use event::{
    Event,
    EventHandler,
    KeyEvent,
    MouseEvent,
    FocusEvent,
};

/// Styling exports
pub use style::{
    Style,
    Color,
    Theme,
    StyleBuilder,
};

/// Layout exports
pub use layout::{
    Layout,
    FlexDirection,
    JustifyContent,
    AlignItems,
    Size,
    Position,
    Rect,
};

/// Widget exports
pub use widget::{
    Widget,
    Button,
    Text,
    Div,
    Input,
    Container,
};

/// Render system exports
pub use render::{
    Renderer,
    VirtualNode,
    RenderContext,
};
