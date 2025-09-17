//! Style builder for fluent style creation.

use crate::style::{Color, Style};
use crate::style::properties::StyleValue;

/// Builder for creating styles fluently.
#[derive(Debug, Clone)]
pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    /// Create a new style builder.
    pub fn new() -> Self {
        Self {
            style: Style::default(),
        }
    }

    /// Set the background color.
    pub fn background_color(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
        self
    }

    /// Set the text color.
    pub fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }

    /// Set the border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.style.border_color = Some(color);
        self
    }

    /// Set the width.
    pub fn width(mut self, width: StyleValue) -> Self {
        self.style.width = Some(width);
        self
    }

    /// Set the height.
    pub fn height(mut self, height: StyleValue) -> Self {
        self.style.height = Some(height);
        self
    }

    /// Build the final style.
    pub fn build(self) -> Style {
        self.style
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
