//! Style properties and definitions.

use crate::style::Color;
use serde::{Deserialize, Serialize};

/// A complete style definition for an element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Style {
    /// Background color
    pub background_color: Option<Color>,
    /// Text color
    pub color: Option<Color>,
    /// Border color
    pub border_color: Option<Color>,
    /// Width
    pub width: Option<StyleValue>,
    /// Height
    pub height: Option<StyleValue>,
}

/// A style property value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StyleProperty {
    /// Background color property
    BackgroundColor(Color),
    /// Text color property
    Color(Color),
    /// Border color property
    BorderColor(Color),
    /// Width property
    Width(StyleValue),
    /// Height property
    Height(StyleValue),
}

/// A style value that can be absolute or relative.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StyleValue {
    /// Absolute value in terminal cells
    Absolute(u32),
    /// Percentage of parent
    Percentage(f32),
    /// Auto-calculated
    Auto,
    /// Fill available space
    Fill,
}
