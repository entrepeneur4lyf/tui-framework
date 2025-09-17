//! Rendering context and state.

use crate::style::Theme;

/// Context passed to components during rendering.
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// The current theme
    pub theme: Theme,
    /// Whether we're in debug mode
    pub debug: bool,
    /// Current viewport size
    pub viewport_size: Option<crate::layout::Size>,
}

impl RenderContext {
    /// Create a new render context with the given theme.
    pub fn new(theme: &Theme) -> Self {
        Self {
            theme: theme.clone(),
            debug: false,
            viewport_size: None,
        }
    }

    /// Create a debug render context.
    pub fn debug(theme: &Theme) -> Self {
        Self {
            theme: theme.clone(),
            debug: true,
            viewport_size: None,
        }
    }

    /// Set the viewport size.
    pub fn with_viewport_size(mut self, size: crate::layout::Size) -> Self {
        self.viewport_size = Some(size);
        self
    }
}
