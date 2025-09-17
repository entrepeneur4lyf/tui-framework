//! Convenient re-exports for common use cases.
//!
//! This module provides a convenient way to import the most commonly used
//! types and traits from the framework.
//!
//! ```rust
//! use tui_framework::prelude::*;
//! ```

// Core traits and types
pub use crate::app::App;
pub use crate::component::{Component, ComponentId};
pub use crate::error::{Error, Result};

// Reactive system
pub use crate::reactive::{
    use_context, use_effect, use_memo, use_state, Context, ContextProvider, State,
};

// Event system
pub use crate::event::{Event, EventHandler, FocusEvent, KeyEvent, MouseEvent};

// Styling system
pub use crate::style::{Color, Style, StyleBuilder, Theme};

// Layout system
pub use crate::layout::{
    AlignItems, FlexDirection, JustifyContent, Layout, Position, Rect, Size,
};

// Widget system
pub use crate::widget::{Button, Container, Div, Input, Text, Widget};

// Render system
pub use crate::render::{RenderContext, Renderer, VirtualNode};

// Macros (when available)
// TODO: Implement macro system
// pub use tui_framework_macros::{component, css, theme};

// Common standard library types
pub use std::collections::HashMap;
pub use std::sync::Arc;

// Async traits
pub use async_trait::async_trait;

// Commonly used external types
pub use tokio;
pub use futures;

// Re-export libnotcurses types that users might need
#[cfg(feature = "notcurses")]
pub use libnotcurses_sys::{NcKey, NcResult};
