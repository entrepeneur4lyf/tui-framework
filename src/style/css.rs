//! CSS utility functions for the TUI framework.

use crate::style::{Color, StyleBuilder, StyleValue};

/// Apply CSS-like properties to a StyleBuilder.
///
/// This function provides a simple way to apply CSS-like styling
/// using string-based property names and values.
///
/// # Example
/// ```rust,ignore
/// use tui_framework::style::{StyleBuilder, css::apply_css_property};
///
/// let builder = StyleBuilder::new();
/// let builder = apply_css_property(builder, "background_color", "blue");
/// let builder = apply_css_property(builder, "width", "100");
/// ```
pub fn apply_css_property(builder: StyleBuilder, property: &str, value: &str) -> StyleBuilder {
    match property {
        "background_color" | "background-color" => {
            if let Some(color) = parse_color(value) {
                builder.background_color(color)
            } else {
                builder
            }
        }
        "color" | "text-color" => {
            if let Some(color) = parse_color(value) {
                builder.color(color)
            } else {
                builder
            }
        }
        "border_color" | "border-color" => {
            if let Some(color) = parse_color(value) {
                builder.border_color(color)
            } else {
                builder
            }
        }
        "width" => {
            if let Some(style_value) = parse_style_value(value) {
                builder.width(style_value)
            } else {
                builder
            }
        }
        "height" => {
            if let Some(style_value) = parse_style_value(value) {
                builder.height(style_value)
            } else {
                builder
            }
        }
        _ => builder, // Unknown property, ignore
    }
}

/// Parse a color value from a string.
///
/// Supports basic color names and hex values.
fn parse_color(value: &str) -> Option<Color> {
    match value.to_lowercase().as_str() {
        "black" => Some(Color::BLACK),
        "white" => Some(Color::WHITE),
        "red" => Some(Color::RED),
        "green" => Some(Color::GREEN),
        "blue" => Some(Color::BLUE),
        "yellow" => Some(Color::YELLOW),
        "cyan" => Some(Color::CYAN),
        "magenta" => Some(Color::MAGENTA),
        "gray" | "grey" => Some(Color::GRAY),
        "dark_gray" | "dark-gray" | "darkgray" => Some(Color::DARK_GRAY),
        "light_gray" | "light-gray" | "lightgray" => Some(Color::LIGHT_GRAY),
        "transparent" => Some(Color::TRANSPARENT),
        _ => {
            // Try to parse as hex color
            if value.starts_with('#') && value.len() == 7 {
                if let Ok(r) = u8::from_str_radix(&value[1..3], 16) {
                    if let Ok(g) = u8::from_str_radix(&value[3..5], 16) {
                        if let Ok(b) = u8::from_str_radix(&value[5..7], 16) {
                            return Some(Color::rgb(r, g, b));
                        }
                    }
                }
            }
            None
        }
    }
}

/// Parse a style value from a string.
///
/// Supports absolute values, percentages, and keywords.
fn parse_style_value(value: &str) -> Option<StyleValue> {
    match value.to_lowercase().as_str() {
        "auto" => Some(StyleValue::Auto),
        "fill" => Some(StyleValue::Fill),
        _ => {
            // Try to parse as percentage
            if let Some(percent_str) = value.strip_suffix('%') {
                if let Ok(percent) = percent_str.parse::<f32>() {
                    return Some(StyleValue::Percentage(percent / 100.0));
                }
            }

            // Try to parse as absolute value
            if let Ok(abs_value) = value.parse::<u32>() {
                return Some(StyleValue::Absolute(abs_value));
            }

            // Try to parse with "px" suffix
            if let Some(px_str) = value.strip_suffix("px") {
                if let Ok(px_value) = px_str.parse::<u32>() {
                    return Some(StyleValue::Absolute(px_value));
                }
            }

            None
        }
    }
}

/// Create a CSS utility class parser for common patterns.
///
/// This function provides Tailwind-like utility class parsing
/// for common styling patterns.
///
/// # Example
/// ```rust,ignore
/// use tui_framework::style::{StyleBuilder, css::apply_utility_classes};
///
/// let builder = StyleBuilder::new();
/// let builder = apply_utility_classes(builder, "bg-blue text-white w-full");
/// ```
pub fn apply_utility_classes(builder: StyleBuilder, classes: &str) -> StyleBuilder {
    let mut current_builder = builder;

    for class in classes.split_whitespace() {
        current_builder = apply_utility_class(current_builder, class);
    }

    current_builder
}

/// Apply a single utility class to a StyleBuilder.
fn apply_utility_class(builder: StyleBuilder, class: &str) -> StyleBuilder {
    // Background color utilities
    if let Some(color_name) = class.strip_prefix("bg-") {
        if let Some(color) = parse_color(color_name) {
            return builder.background_color(color);
        }
    }

    // Text color utilities
    if let Some(color_name) = class.strip_prefix("text-") {
        if let Some(color) = parse_color(color_name) {
            return builder.color(color);
        }
    }

    // Border color utilities
    if let Some(color_name) = class.strip_prefix("border-") {
        if let Some(color) = parse_color(color_name) {
            return builder.border_color(color);
        }
    }

    // Width utilities
    match class {
        "w-full" => builder.width(StyleValue::Fill),
        "w-auto" => builder.width(StyleValue::Auto),
        _ => {
            if let Some(width_str) = class.strip_prefix("w-") {
                if let Some(width) = parse_style_value(width_str) {
                    return builder.width(width);
                }
            }
            builder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("red"), Some(Color::RED));
        assert_eq!(parse_color("blue"), Some(Color::BLUE));
        assert_eq!(parse_color("#ff0000"), Some(Color::rgb(255, 0, 0)));
        assert_eq!(parse_color("invalid"), None);
    }

    #[test]
    fn test_parse_style_value() {
        assert_eq!(parse_style_value("auto"), Some(StyleValue::Auto));
        assert_eq!(parse_style_value("fill"), Some(StyleValue::Fill));
        assert_eq!(parse_style_value("100"), Some(StyleValue::Absolute(100)));
        assert_eq!(parse_style_value("50%"), Some(StyleValue::Percentage(0.5)));
        assert_eq!(parse_style_value("20px"), Some(StyleValue::Absolute(20)));
    }

    #[test]
    fn test_apply_css_property() {
        let builder = StyleBuilder::new();
        let builder = apply_css_property(builder, "background_color", "red");
        let style = builder.build();
        assert_eq!(style.background_color, Some(Color::RED));
    }

    #[test]
    fn test_apply_utility_classes() {
        let builder = StyleBuilder::new();
        let builder = apply_utility_classes(builder, "bg-blue text-white w-full");
        let style = builder.build();

        assert_eq!(style.background_color, Some(Color::BLUE));
        assert_eq!(style.color, Some(Color::WHITE));
        assert_eq!(style.width, Some(StyleValue::Fill));
    }
}
