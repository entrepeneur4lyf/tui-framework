//! Enhanced CSS-like style properties for TUI components.

use crate::style::Color;
use serde::{Deserialize, Serialize};

/// Represents a CSS-like style property value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    /// String value
    String(String),
    /// Numeric value
    Number(f64),
    /// Color value
    Color(Color),
    /// Boolean value
    Boolean(bool),
    /// Percentage value
    Percentage(f64),
    /// Pixel value
    Pixels(u32),
    /// Em value (relative to font size)
    Em(f64),
    /// Rem value (relative to root font size)
    Rem(f64),
    /// Viewport width percentage
    Vw(f64),
    /// Viewport height percentage
    Vh(f64),
    /// Auto value
    Auto,
    /// None/null value
    None,
    /// Inherit from parent
    Inherit,
    /// Initial/default value
    Initial,
}

impl PropertyValue {
    /// Convert to string representation.
    pub fn to_string(&self) -> String {
        match self {
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Number(n) => n.to_string(),
            PropertyValue::Color(c) => format!("{:?}", c),
            PropertyValue::Boolean(b) => b.to_string(),
            PropertyValue::Percentage(p) => format!("{}%", p),
            PropertyValue::Pixels(px) => format!("{}px", px),
            PropertyValue::Em(em) => format!("{}em", em),
            PropertyValue::Rem(rem) => format!("{}rem", rem),
            PropertyValue::Vw(vw) => format!("{}vw", vw),
            PropertyValue::Vh(vh) => format!("{}vh", vh),
            PropertyValue::Auto => "auto".to_string(),
            PropertyValue::None => "none".to_string(),
            PropertyValue::Inherit => "inherit".to_string(),
            PropertyValue::Initial => "initial".to_string(),
        }
    }

    /// Try to convert to a number.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            PropertyValue::Number(n) => Some(*n),
            PropertyValue::Percentage(p) => Some(*p),
            PropertyValue::Pixels(px) => Some(*px as f64),
            PropertyValue::Em(em) => Some(*em),
            PropertyValue::Rem(rem) => Some(*rem),
            PropertyValue::Vw(vw) => Some(*vw),
            PropertyValue::Vh(vh) => Some(*vh),
            _ => None,
        }
    }

    /// Try to convert to a color.
    pub fn as_color(&self) -> Option<Color> {
        match self {
            PropertyValue::Color(c) => Some(*c),
            _ => None,
        }
    }

    /// Try to convert to a boolean.
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            PropertyValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert to pixels given context (viewport size, font size, etc.).
    pub fn to_pixels(&self, context: &PropertyContext) -> Option<u32> {
        match self {
            PropertyValue::Pixels(px) => Some(*px),
            PropertyValue::Percentage(p) => {
                context.parent_size.map(|size| ((size as f64 * p) / 100.0) as u32)
            }
            PropertyValue::Em(em) => {
                Some((context.font_size as f64 * em) as u32)
            }
            PropertyValue::Rem(rem) => {
                Some((context.root_font_size as f64 * rem) as u32)
            }
            PropertyValue::Vw(vw) => {
                Some(((context.viewport_width as f64 * vw) / 100.0) as u32)
            }
            PropertyValue::Vh(vh) => {
                Some(((context.viewport_height as f64 * vh) / 100.0) as u32)
            }
            PropertyValue::Number(n) => Some(*n as u32),
            _ => None,
        }
    }
}

/// Context for resolving relative property values.
#[derive(Debug, Clone)]
pub struct PropertyContext {
    /// Current viewport width
    pub viewport_width: u32,
    /// Current viewport height
    pub viewport_height: u32,
    /// Current font size
    pub font_size: u32,
    /// Root font size
    pub root_font_size: u32,
    /// Parent element size (for percentage calculations)
    pub parent_size: Option<u32>,
}

impl Default for PropertyContext {
    fn default() -> Self {
        Self {
            viewport_width: 80,
            viewport_height: 24,
            font_size: 16,
            root_font_size: 16,
            parent_size: None,
        }
    }
}

/// Enhanced CSS properties with more comprehensive support.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CssProperty {
    // Layout properties
    /// CSS display property - controls the display type of an element
    Display(DisplayValue),
    /// CSS position property - controls the positioning method of an element
    Position(PositionValue),
    /// CSS top property - sets the top edge position for positioned elements
    Top(PropertyValue),
    /// CSS right property - sets the right edge position for positioned elements
    Right(PropertyValue),
    /// CSS bottom property - sets the bottom edge position for positioned elements
    Bottom(PropertyValue),
    /// CSS left property - sets the left edge position for positioned elements
    Left(PropertyValue),
    /// CSS width property - sets the width of an element
    Width(PropertyValue),
    /// CSS height property - sets the height of an element
    Height(PropertyValue),
    /// CSS min-width property - sets the minimum width of an element
    MinWidth(PropertyValue),
    /// CSS max-width property - sets the maximum width of an element
    MaxWidth(PropertyValue),
    /// CSS min-height property - sets the minimum height of an element
    MinHeight(PropertyValue),
    /// CSS max-height property - sets the maximum height of an element
    MaxHeight(PropertyValue),

    // Box model
    /// CSS margin property - sets all four margin values at once
    Margin(BoxValue),
    /// CSS margin-top property - sets the top margin of an element
    MarginTop(PropertyValue),
    /// CSS margin-right property - sets the right margin of an element
    MarginRight(PropertyValue),
    /// CSS margin-bottom property - sets the bottom margin of an element
    MarginBottom(PropertyValue),
    /// CSS margin-left property - sets the left margin of an element
    MarginLeft(PropertyValue),
    /// CSS padding property - sets all four padding values at once
    Padding(BoxValue),
    /// CSS padding-top property - sets the top padding of an element
    PaddingTop(PropertyValue),
    /// CSS padding-right property - sets the right padding of an element
    PaddingRight(PropertyValue),
    /// CSS padding-bottom property - sets the bottom padding of an element
    PaddingBottom(PropertyValue),
    /// CSS padding-left property - sets the left padding of an element
    PaddingLeft(PropertyValue),

    // Border
    /// CSS border property - sets all border properties at once
    Border(BorderValue),
    /// CSS border-width property - sets the width of the border
    BorderWidth(PropertyValue),
    /// CSS border-style property - sets the style of the border
    BorderStyle(BorderStyleValue),
    /// CSS border-color property - sets the color of the border
    BorderColor(PropertyValue),
    /// CSS border-radius property - sets the border radius for rounded corners
    BorderRadius(PropertyValue),

    // Background
    /// CSS background-color property - sets the background color of an element
    BackgroundColor(PropertyValue),
    /// CSS background-image property - sets the background image of an element
    BackgroundImage(PropertyValue),
    /// CSS background-size property - sets the size of the background image
    BackgroundSize(BackgroundSizeValue),
    /// CSS background-position property - sets the position of the background image
    BackgroundPosition(BackgroundPositionValue),
    /// CSS background-repeat property - sets how the background image repeats
    BackgroundRepeat(BackgroundRepeatValue),

    // Text
    /// CSS color property - sets the text color of an element
    Color(PropertyValue),
    /// CSS font-size property - sets the size of the font
    FontSize(PropertyValue),
    /// CSS font-weight property - sets the weight (boldness) of the font
    FontWeight(FontWeightValue),
    /// CSS font-style property - sets the style of the font (normal, italic, oblique)
    FontStyle(FontStyleValue),
    /// CSS font-family property - sets the font family for text
    FontFamily(PropertyValue),
    /// CSS text-align property - sets the horizontal alignment of text
    TextAlign(TextAlignValue),
    /// CSS text-decoration property - sets text decoration (underline, overline, etc.)
    TextDecoration(TextDecorationValue),
    /// CSS line-height property - sets the height of a line box
    LineHeight(PropertyValue),
    /// CSS letter-spacing property - sets the spacing between characters
    LetterSpacing(PropertyValue),
    /// CSS word-spacing property - sets the spacing between words
    WordSpacing(PropertyValue),

    // Flexbox
    /// CSS flex-direction property - sets the direction of flex items
    FlexDirection(FlexDirectionValue),
    /// CSS flex-wrap property - sets whether flex items wrap to new lines
    FlexWrap(FlexWrapValue),
    /// CSS justify-content property - sets alignment along the main axis
    JustifyContent(JustifyContentValue),
    /// CSS align-items property - sets alignment along the cross axis
    AlignItems(AlignItemsValue),
    /// CSS align-content property - sets alignment of wrapped lines
    AlignContent(AlignContentValue),
    /// CSS flex-grow property - sets how much a flex item should grow
    FlexGrow(PropertyValue),
    /// CSS flex-shrink property - sets how much a flex item should shrink
    FlexShrink(PropertyValue),
    /// CSS flex-basis property - sets the initial main size of a flex item
    FlexBasis(PropertyValue),

    // Visual effects
    /// CSS opacity property - sets the transparency of an element
    Opacity(PropertyValue),
    /// CSS visibility property - sets whether an element is visible
    Visibility(VisibilityValue),
    /// CSS overflow property - sets how content that overflows is handled
    Overflow(OverflowValue),
    /// CSS z-index property - sets the stack order of positioned elements
    ZIndex(PropertyValue),

    // Animation
    /// CSS transition property - sets transition effects for property changes
    Transition(TransitionValue),
    /// CSS animation property - sets keyframe animations
    Animation(AnimationValue),
    /// CSS transform property - sets 2D and 3D transformations
    Transform(TransformValue),

    // Interaction
    /// CSS cursor property - sets the cursor type when hovering over an element
    Cursor(CursorValue),
    /// CSS user-select property - sets whether text can be selected
    UserSelect(UserSelectValue),
    /// CSS pointer-events property - sets whether an element can be the target of pointer events
    PointerEvents(PointerEventsValue),
}

/// Display property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayValue {
    /// Block-level element that starts on a new line
    Block,
    /// Inline element that flows with text
    Inline,
    /// Inline element that can have width and height
    InlineBlock,
    /// Flexible box container
    Flex,
    /// Inline flexible box container
    InlineFlex,
    /// Grid container
    Grid,
    /// Inline grid container
    InlineGrid,
    /// Element is not displayed
    None,
}

/// Position property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionValue {
    /// Default positioning - element follows normal document flow
    Static,
    /// Positioned relative to its normal position
    Relative,
    /// Positioned relative to the nearest positioned ancestor
    Absolute,
    /// Positioned relative to the viewport
    Fixed,
    /// Positioned based on scroll position
    Sticky,
}

/// Box model value (for margin, padding).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BoxValue {
    /// All sides
    All(PropertyValue),
    /// Vertical and horizontal
    VerticalHorizontal(PropertyValue, PropertyValue),
    /// Top, horizontal, bottom
    TopHorizontalBottom(PropertyValue, PropertyValue, PropertyValue),
    /// Top, right, bottom, left
    Individual(PropertyValue, PropertyValue, PropertyValue, PropertyValue),
}

/// Border property value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderValue {
    /// Border width
    pub width: PropertyValue,
    /// Border style
    pub style: BorderStyleValue,
    /// Border color
    pub color: PropertyValue,
}

/// Border style values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderStyleValue {
    /// No border
    None,
    /// Solid border line
    Solid,
    /// Dashed border line
    Dashed,
    /// Dotted border line
    Dotted,
    /// Double border line
    Double,
}

/// Background size values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackgroundSizeValue {
    /// Automatic sizing
    Auto,
    /// Scale to cover entire container
    Cover,
    /// Scale to fit within container
    Contain,
    /// Specific width and height values
    Size(PropertyValue, PropertyValue),
}

/// Background position values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackgroundPositionValue {
    /// Center position
    Center,
    /// Top edge
    Top,
    /// Bottom edge
    Bottom,
    /// Left edge
    Left,
    /// Right edge
    Right,
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Specific x and y position values
    Position(PropertyValue, PropertyValue),
}

/// Background repeat values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundRepeatValue {
    /// Repeat in both directions
    Repeat,
    /// Repeat horizontally only
    RepeatX,
    /// Repeat vertically only
    RepeatY,
    /// No repetition
    NoRepeat,
    /// Space between repetitions
    Space,
    /// Round repetitions to fit
    Round,
}

/// Font weight values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FontWeightValue {
    /// Normal font weight
    Normal,
    /// Bold font weight
    Bold,
    /// Bolder than parent
    Bolder,
    /// Lighter than parent
    Lighter,
    /// Numeric weight value (100-900)
    Weight(u32), // 100-900
}

/// Font style values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontStyleValue {
    /// Normal font style
    Normal,
    /// Italic font style
    Italic,
    /// Oblique font style
    Oblique,
}

/// Text align values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextAlignValue {
    /// Left alignment
    Left,
    /// Right alignment
    Right,
    /// Center alignment
    Center,
    /// Justified alignment
    Justify,
    /// Start alignment (language-dependent)
    Start,
    /// End alignment (language-dependent)
    End,
}

/// Text decoration values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDecorationValue {
    /// No text decoration
    None,
    /// Underline text decoration
    Underline,
    /// Overline text decoration
    Overline,
    /// Line-through text decoration
    LineThrough,
}

/// Flex direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexDirectionValue {
    /// Horizontal direction, left to right
    Row,
    /// Horizontal direction, right to left
    RowReverse,
    /// Vertical direction, top to bottom
    Column,
    /// Vertical direction, bottom to top
    ColumnReverse,
}

/// Flex wrap values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexWrapValue {
    /// No wrapping
    NoWrap,
    /// Wrap to new lines
    Wrap,
    /// Wrap to new lines in reverse order
    WrapReverse,
}

/// Justify content values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JustifyContentValue {
    /// Pack items at start
    FlexStart,
    /// Pack items at end
    FlexEnd,
    /// Center items
    Center,
    /// Distribute space between items
    SpaceBetween,
    /// Distribute space around items
    SpaceAround,
    /// Distribute space evenly
    SpaceEvenly,
}

/// Align items values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignItemsValue {
    /// Align at start
    FlexStart,
    /// Align at end
    FlexEnd,
    /// Center alignment
    Center,
    /// Baseline alignment
    Baseline,
    /// Stretch to fill
    Stretch,
}

/// Align content values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignContentValue {
    /// Pack lines at start
    FlexStart,
    /// Pack lines at end
    FlexEnd,
    /// Center lines
    Center,
    /// Distribute space between lines
    SpaceBetween,
    /// Distribute space around lines
    SpaceAround,
    /// Distribute space evenly
    SpaceEvenly,
    /// Stretch lines to fill
    Stretch,
}

/// Visibility values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisibilityValue {
    /// Element is visible
    Visible,
    /// Element is hidden but takes up space
    Hidden,
    /// Element is hidden and removed from layout
    Collapse,
}

/// Overflow values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverflowValue {
    /// Content overflows visibly
    Visible,
    /// Content is clipped
    Hidden,
    /// Scrollbars are shown
    Scroll,
    /// Scrollbars shown only when needed
    Auto,
}

/// Transition property value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionValue {
    /// CSS property to transition
    pub property: String,
    /// Transition duration
    pub duration: PropertyValue,
    /// Timing function for transition
    pub timing_function: PropertyValue,
    /// Transition delay
    pub delay: PropertyValue,
}

/// Animation property value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationValue {
    /// Animation name
    pub name: String,
    /// Animation duration
    pub duration: PropertyValue,
    /// Animation timing function
    pub timing_function: PropertyValue,
    /// Animation delay
    pub delay: PropertyValue,
    /// Animation iteration count
    pub iteration_count: PropertyValue,
    /// Animation direction
    pub direction: PropertyValue,
    /// Animation fill mode
    pub fill_mode: PropertyValue,
}

/// Transform property value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransformValue {
    /// No transformation
    None,
    /// Translation transformation
    Translate(PropertyValue, PropertyValue),
    /// Scale transformation
    Scale(PropertyValue, PropertyValue),
    /// Rotation transformation
    Rotate(PropertyValue),
    /// Skew transformation
    Skew(PropertyValue, PropertyValue),
    /// Matrix transformation
    Matrix(f64, f64, f64, f64, f64, f64),
}

/// Cursor values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CursorValue {
    /// Automatic cursor
    Auto,
    /// Default cursor
    Default,
    /// Pointer cursor
    Pointer,
    /// Text cursor
    Text,
    /// Wait cursor
    Wait,
    /// Help cursor
    Help,
    /// Not allowed cursor
    NotAllowed,
    /// Grab cursor
    Grab,
    /// Grabbing cursor
    Grabbing,
}

/// User select values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserSelectValue {
    /// Automatic selection
    Auto,
    /// No selection allowed
    None,
    /// Text selection allowed
    Text,
    /// All content selectable
    All,
}

/// Pointer events values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerEventsValue {
    /// Automatic pointer events
    Auto,
    /// No pointer events
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_value_conversion() {
        let context = PropertyContext::default();

        // Test pixel conversion
        let px_value = PropertyValue::Pixels(100);
        assert_eq!(px_value.to_pixels(&context), Some(100));

        // Test percentage conversion
        let mut context_with_parent = context.clone();
        context_with_parent.parent_size = Some(200);
        let pct_value = PropertyValue::Percentage(50.0);
        assert_eq!(pct_value.to_pixels(&context_with_parent), Some(100));

        // Test em conversion
        let em_value = PropertyValue::Em(2.0);
        assert_eq!(em_value.to_pixels(&context), Some(32)); // 16 * 2

        // Test viewport units
        let vw_value = PropertyValue::Vw(50.0);
        assert_eq!(vw_value.to_pixels(&context), Some(40)); // 80 * 0.5
    }

    #[test]
    fn test_property_value_string_conversion() {
        assert_eq!(PropertyValue::Pixels(100).to_string(), "100px");
        assert_eq!(PropertyValue::Percentage(50.0).to_string(), "50%");
        assert_eq!(PropertyValue::Em(1.5).to_string(), "1.5em");
        assert_eq!(PropertyValue::Auto.to_string(), "auto");
        assert_eq!(PropertyValue::Inherit.to_string(), "inherit");
    }

    #[test]
    fn test_box_value_variants() {
        let all = BoxValue::All(PropertyValue::Pixels(10));
        let vh = BoxValue::VerticalHorizontal(
            PropertyValue::Pixels(10),
            PropertyValue::Pixels(20)
        );
        let thb = BoxValue::TopHorizontalBottom(
            PropertyValue::Pixels(5),
            PropertyValue::Pixels(10),
            PropertyValue::Pixels(15)
        );
        let individual = BoxValue::Individual(
            PropertyValue::Pixels(1),
            PropertyValue::Pixels(2),
            PropertyValue::Pixels(3),
            PropertyValue::Pixels(4)
        );

        // Just test that they can be created without panicking
        assert!(matches!(all, BoxValue::All(_)));
        assert!(matches!(vh, BoxValue::VerticalHorizontal(_, _)));
        assert!(matches!(thb, BoxValue::TopHorizontalBottom(_, _, _)));
        assert!(matches!(individual, BoxValue::Individual(_, _, _, _)));
    }
}
