//! Color definitions and utilities.

use palette::{FromColor, Hsl, Hsv, Srgb};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Represents a color in the TUI framework.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255)
    pub a: u8,
}

impl Color {
    /// Create a new color from RGB values.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a new color from RGBA values.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from HSL values.
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        let hsl = Hsl::new(h, s, l);
        let rgb: Srgb = Srgb::from_color(hsl);
        Self::rgb(
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
        )
    }

    /// Create a color from HSV values.
    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        let hsv = Hsv::new(h, s, v);
        let rgb: Srgb = Srgb::from_color(hsv);
        Self::rgb(
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
        )
    }

    /// Create a color from a hex string.
    pub fn hex(hex: &str) -> Result<Self, ColorParseError> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            3 => {
                // RGB format: #RGB
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)?;
                Ok(Self::rgb(r, g, b))
            }
            6 => {
                // RGB format: #RRGGBB
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                Ok(Self::rgb(r, g, b))
            }
            8 => {
                // RGBA format: #RRGGBBAA
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                let a = u8::from_str_radix(&hex[6..8], 16)?;
                Ok(Self::rgba(r, g, b, a))
            }
            _ => Err(ColorParseError::InvalidFormat),
        }
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }

    /// Get the luminance of the color.
    pub fn luminance(&self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        0.299 * r + 0.587 * g + 0.114 * b
    }

    /// Check if the color is dark.
    pub fn is_dark(&self) -> bool {
        self.luminance() < 0.5
    }

    /// Check if the color is light.
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }

    /// Lighten the color by a percentage.
    pub fn lighten(&self, amount: f32) -> Self {
        let rgb = Srgb::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        );
        let mut lightened = Hsl::from_color(rgb);
        lightened.lightness = (lightened.lightness + amount).min(1.0);
        let rgb: Srgb = Srgb::from_color(lightened);

        Self::rgba(
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
            self.a,
        )
    }

    /// Darken the color by a percentage.
    pub fn darken(&self, amount: f32) -> Self {
        let rgb = Srgb::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        );
        let mut darkened = Hsl::from_color(rgb);
        darkened.lightness = (darkened.lightness - amount).max(0.0);
        let rgb: Srgb = Srgb::from_color(darkened);

        Self::rgba(
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
            self.a,
        )
    }

    /// Mix this color with another color.
    pub fn mix(&self, other: &Color, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let inv_ratio = 1.0 - ratio;

        Self::rgba(
            (self.r as f32 * inv_ratio + other.r as f32 * ratio) as u8,
            (self.g as f32 * inv_ratio + other.g as f32 * ratio) as u8,
            (self.b as f32 * inv_ratio + other.b as f32 * ratio) as u8,
            (self.a as f32 * inv_ratio + other.a as f32 * ratio) as u8,
        )
    }

    /// Set the alpha channel.
    pub fn with_alpha(&self, alpha: u8) -> Self {
        Self::rgba(self.r, self.g, self.b, alpha)
    }
}

/// Common color constants.
impl Color {
    /// Pure black color.
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    /// Pure white color.
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    /// Pure red color.
    pub const RED: Color = Color::rgb(255, 0, 0);
    /// Pure green color.
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    /// Pure blue color.
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    /// Pure yellow color.
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    /// Pure cyan color.
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    /// Pure magenta color.
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    /// Medium gray color.
    pub const GRAY: Color = Color::rgb(128, 128, 128);
    /// Dark gray color.
    pub const DARK_GRAY: Color = Color::rgb(64, 64, 64);
    /// Light gray color.
    pub const LIGHT_GRAY: Color = Color::rgb(192, 192, 192);
    /// Fully transparent color.
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
}

/// Error type for color parsing.
#[derive(Debug, Clone)]
pub enum ColorParseError {
    /// The color format is invalid.
    InvalidFormat,
    /// Failed to parse hexadecimal value.
    InvalidHex(std::num::ParseIntError),
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorParseError::InvalidFormat => write!(f, "Invalid color format"),
            ColorParseError::InvalidHex(e) => write!(f, "Invalid hex value: {}", e),
        }
    }
}

impl std::error::Error for ColorParseError {}

impl From<std::num::ParseIntError> for ColorParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ColorParseError::InvalidHex(err)
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to parse as hex first
        if s.starts_with('#') {
            return Color::hex(s);
        }

        // Try to parse named colors
        match s.to_lowercase().as_str() {
            "black" => Ok(Color::BLACK),
            "white" => Ok(Color::WHITE),
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            "yellow" => Ok(Color::YELLOW),
            "cyan" => Ok(Color::CYAN),
            "magenta" => Ok(Color::MAGENTA),
            "gray" | "grey" => Ok(Color::GRAY),
            "darkgray" | "darkgrey" => Ok(Color::DARK_GRAY),
            "lightgray" | "lightgrey" => Ok(Color::LIGHT_GRAY),
            "transparent" => Ok(Color::TRANSPARENT),
            _ => Err(ColorParseError::InvalidFormat),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_hex() {
        let color = Color::hex("#ff8040").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);

        assert_eq!(color.to_hex(), "#ff8040");
    }

    #[test]
    fn test_color_parsing() {
        assert_eq!("red".parse::<Color>().unwrap(), Color::RED);
        assert_eq!("#ff0000".parse::<Color>().unwrap(), Color::RED);
    }

    #[test]
    fn test_color_operations() {
        let color = Color::rgb(100, 100, 100);
        let lighter = color.lighten(0.2);
        let darker = color.darken(0.2);

        assert!(lighter.luminance() > color.luminance());
        assert!(darker.luminance() < color.luminance());
    }

    #[test]
    fn test_color_mix() {
        let red = Color::RED;
        let blue = Color::BLUE;
        let purple = red.mix(&blue, 0.5);

        assert_eq!(purple.r, 127);
        assert_eq!(purple.g, 0);
        assert_eq!(purple.b, 127);
    }
}
