//! Theme system for styling.

use crate::style::Color;
use serde::{Deserialize, Serialize};

/// A theme defines the color palette and styling for the application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    /// Primary color
    pub primary: Color,
    /// Secondary color
    pub secondary: Color,
    /// Background color
    pub background: Color,
    /// Surface color
    pub surface: Color,
    /// Text color
    pub text: Color,
    /// Text color on primary background
    pub text_on_primary: Color,
    /// Error color
    pub error: Color,
    /// Warning color
    pub warning: Color,
    /// Success color
    pub success: Color,
    /// Info color
    pub info: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create a dark theme.
    pub fn dark() -> Self {
        Self {
            primary: Color::rgb(100, 150, 255),
            secondary: Color::rgb(150, 100, 255),
            background: Color::rgb(20, 20, 25),
            surface: Color::rgb(30, 30, 35),
            text: Color::rgb(240, 240, 245),
            text_on_primary: Color::WHITE,
            error: Color::rgb(255, 100, 100),
            warning: Color::rgb(255, 200, 100),
            success: Color::rgb(100, 255, 100),
            info: Color::rgb(100, 200, 255),
        }
    }

    /// Create a light theme.
    pub fn light() -> Self {
        Self {
            primary: Color::rgb(50, 100, 200),
            secondary: Color::rgb(100, 50, 200),
            background: Color::rgb(250, 250, 255),
            surface: Color::rgb(240, 240, 245),
            text: Color::rgb(20, 20, 25),
            text_on_primary: Color::WHITE,
            error: Color::rgb(200, 50, 50),
            warning: Color::rgb(200, 150, 50),
            success: Color::rgb(50, 200, 50),
            info: Color::rgb(50, 150, 200),
        }
    }
}
