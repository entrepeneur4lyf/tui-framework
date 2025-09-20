//! Text widget implementation.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use crate::style::Color;
use crate::style::properties::Style;
use crate::widget::Widget;
use async_trait::async_trait;

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    /// Left-aligned text
    Left,
    /// Center-aligned text
    Center,
    /// Right-aligned text
    Right,
    /// Justified text
    Justify,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Left
    }
}

/// Text wrapping options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextWrap {
    /// No wrapping - text may overflow
    None,
    /// Wrap at word boundaries
    Word,
    /// Wrap at character boundaries
    Char,
    /// Wrap with hyphenation
    Hyphen,
}

impl Default for TextWrap {
    fn default() -> Self {
        Self::Word
    }
}

/// Text style formatting.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextStyle {
    /// Text color
    pub color: Option<Color>,
    /// Background color
    pub background_color: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
    /// Strikethrough text
    pub strikethrough: bool,
}

impl TextStyle {
    /// Create a new text style.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set background color.
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set bold.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set italic.
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Set underline.
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// Set strikethrough.
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }
}

/// A span of text with optional styling.
#[derive(Debug, Clone)]
pub struct TextSpan {
    /// The text content
    pub text: String,
    /// Optional styling for this span
    pub style: Option<TextStyle>,
}

impl TextSpan {
    /// Create a new text span.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: None,
        }
    }

    /// Create a text span with styling.
    pub fn styled(text: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: text.into(),
            style: Some(style),
        }
    }
}

impl From<&str> for TextSpan {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for TextSpan {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// Rich text content composed of multiple spans.
#[derive(Debug, Clone)]
pub struct RichText {
    /// Text spans
    pub spans: Vec<TextSpan>,
}

impl RichText {
    /// Create new rich text.
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    /// Add a text span.
    pub fn span(mut self, span: TextSpan) -> Self {
        self.spans.push(span);
        self
    }

    /// Add plain text.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.spans.push(TextSpan::new(text));
        self
    }

    /// Add styled text.
    pub fn styled_text(mut self, text: impl Into<String>, style: TextStyle) -> Self {
        self.spans.push(TextSpan::styled(text, style));
        self
    }

    /// Get the plain text content.
    pub fn to_plain_text(&self) -> String {
        self.spans.iter().map(|span| span.text.as_str()).collect()
    }

    /// Check if the rich text is empty.
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.spans.iter().all(|span| span.text.is_empty())
    }

    /// Get the length of the text content.
    pub fn len(&self) -> usize {
        self.spans.iter().map(|span| span.text.len()).sum()
    }
}

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for RichText {
    fn from(text: &str) -> Self {
        Self::new().text(text)
    }
}

impl From<String> for RichText {
    fn from(text: String) -> Self {
        Self::new().text(text)
    }
}

/// A text widget for displaying text content with rich formatting.
pub struct Text {
    base: BaseComponent,
    content: RichText,
    alignment: TextAlign,
    wrap: TextWrap,
    max_width: Option<usize>,
    max_height: Option<usize>,
    style: Style,
}

impl Text {
    /// Create a new text widget.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            base: BaseComponent::new("Text"),
            content: RichText::from(content.into()),
            alignment: TextAlign::default(),
            wrap: TextWrap::default(),
            max_width: None,
            max_height: None,
            style: Style::default(),
        }
    }

    /// Create a new text widget with rich content.
    pub fn rich(content: RichText) -> Self {
        Self {
            base: BaseComponent::new("Text"),
            content,
            alignment: TextAlign::default(),
            wrap: TextWrap::default(),
            max_width: None,
            max_height: None,
            style: Style::default(),
        }
    }

    /// Set the text content.
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = RichText::from(content.into());
    }

    /// Set rich text content.
    pub fn set_rich_content(&mut self, content: RichText) {
        self.content = content;
    }

    /// Get the text content as plain text.
    pub fn content(&self) -> String {
        self.content.to_plain_text()
    }

    /// Get the rich text content.
    pub fn rich_content(&self) -> &RichText {
        &self.content
    }

    /// Set text alignment.
    pub fn with_alignment(mut self, alignment: TextAlign) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set text wrapping.
    pub fn with_wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Set maximum width.
    pub fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Set maximum height.
    pub fn with_max_height(mut self, max_height: usize) -> Self {
        self.max_height = Some(max_height);
        self
    }

    /// Set the text style.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get text alignment.
    pub fn alignment(&self) -> TextAlign {
        self.alignment
    }

    /// Get text wrapping.
    pub fn wrap(&self) -> TextWrap {
        self.wrap
    }

    /// Get maximum width.
    pub fn max_width(&self) -> Option<usize> {
        self.max_width
    }

    /// Get maximum height.
    pub fn max_height(&self) -> Option<usize> {
        self.max_height
    }

    /// Wrap text to fit within the specified width.
    pub fn wrap_text(&self, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![];
        }

        let plain_text = self.content.to_plain_text();

        match self.wrap {
            TextWrap::None => vec![plain_text],
            TextWrap::Word => self.wrap_at_words(&plain_text, width),
            TextWrap::Char => self.wrap_at_chars(&plain_text, width),
            TextWrap::Hyphen => self.wrap_with_hyphen(&plain_text, width),
        }
    }

    /// Wrap text at word boundaries.
    fn wrap_at_words(&self, text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                if word.len() <= width {
                    current_line = word.to_string();
                } else {
                    // Word is too long, break it at character boundaries
                    lines.extend(self.wrap_at_chars(word, width));
                }
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line.clone());
                if word.len() <= width {
                    current_line = word.to_string();
                } else {
                    // Word is too long, break it at character boundaries
                    lines.extend(self.wrap_at_chars(word, width));
                    current_line.clear();
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Wrap text at character boundaries.
    fn wrap_at_chars(&self, text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let chars: Vec<char> = text.chars().collect();

        for chunk in chars.chunks(width) {
            lines.push(chunk.iter().collect());
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Wrap text with hyphenation.
    fn wrap_with_hyphen(&self, text: &str, width: usize) -> Vec<String> {
        if width < 2 {
            return self.wrap_at_chars(text, width);
        }

        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                if word.len() <= width {
                    current_line = word.to_string();
                } else {
                    // Break long word with hyphen
                    let mut remaining = word;
                    while remaining.len() > width {
                        let split_pos = width - 1; // Leave space for hyphen
                        let (part, rest) = remaining.split_at(split_pos);
                        lines.push(format!("{}-", part));
                        remaining = rest;
                    }
                    if !remaining.is_empty() {
                        current_line = remaining.to_string();
                    }
                }
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line.clone());
                if word.len() <= width {
                    current_line = word.to_string();
                } else {
                    // Break long word with hyphen
                    let mut remaining = word;
                    while remaining.len() > width {
                        let split_pos = width - 1; // Leave space for hyphen
                        let (part, rest) = remaining.split_at(split_pos);
                        lines.push(format!("{}-", part));
                        remaining = rest;
                    }
                    current_line = remaining.to_string();
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Apply text alignment to a line.
    pub fn align_text(&self, text: &str, width: usize) -> String {
        if text.len() >= width {
            return text.to_string();
        }

        match self.alignment {
            TextAlign::Left => format!("{:<width$}", text, width = width),
            TextAlign::Right => format!("{:>width$}", text, width = width),
            TextAlign::Center => {
                let padding = width - text.len();
                let left_padding = padding / 2;
                let right_padding = padding - left_padding;
                format!(
                    "{}{}{}",
                    " ".repeat(left_padding),
                    text,
                    " ".repeat(right_padding)
                )
            }
            TextAlign::Justify => {
                if text.trim().is_empty() {
                    return text.to_string();
                }

                let words: Vec<&str> = text.split_whitespace().collect();
                if words.len() <= 1 {
                    return text.to_string();
                }

                let total_chars: usize = words.iter().map(|w| w.len()).sum();
                let total_spaces = width - total_chars;
                let gaps = words.len() - 1;

                if gaps == 0 {
                    return text.to_string();
                }

                let spaces_per_gap = total_spaces / gaps;
                let extra_spaces = total_spaces % gaps;

                let mut result = String::new();
                for (i, word) in words.iter().enumerate() {
                    result.push_str(word);
                    if i < words.len() - 1 {
                        result.push_str(&" ".repeat(spaces_per_gap));
                        if i < extra_spaces {
                            result.push(' ');
                        }
                    }
                }

                result
            }
        }
    }
}

#[async_trait]
impl Component for Text {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "Text"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let plain_text = self.content.to_plain_text();

        // Create a text element with rich text attributes
        let mut text_node = VirtualNode::element("text")
            .attr("content", &plain_text)
            .attr("alignment", format!("{:?}", self.alignment).to_lowercase())
            .attr("wrap", format!("{:?}", self.wrap).to_lowercase());

        if let Some(max_width) = self.max_width {
            text_node = text_node.attr("max_width", max_width.to_string());
        }

        if let Some(max_height) = self.max_height {
            text_node = text_node.attr("max_height", max_height.to_string());
        }

        // Add span information for rich text
        if self.content.spans.len() > 1
            || self
                .content
                .spans
                .first()
                .map_or(false, |s| s.style.is_some())
        {
            text_node = text_node.attr("rich_text", "true");

            // Add each span as a child element
            for (i, span) in self.content.spans.iter().enumerate() {
                let mut span_node = VirtualNode::element("span")
                    .attr("index", i.to_string())
                    .child(VirtualNode::text(&span.text));

                if let Some(ref style) = span.style {
                    if let Some(ref color) = style.color {
                        span_node = span_node.attr("color", format!("{:?}", color));
                    }
                    if let Some(ref bg_color) = style.background_color {
                        span_node = span_node.attr("background_color", format!("{:?}", bg_color));
                    }
                    if style.bold {
                        span_node = span_node.attr("bold", "true");
                    }
                    if style.italic {
                        span_node = span_node.attr("italic", "true");
                    }
                    if style.underline {
                        span_node = span_node.attr("underline", "true");
                    }
                    if style.strikethrough {
                        span_node = span_node.attr("strikethrough", "true");
                    }
                }

                text_node = text_node.child(span_node);
            }
        } else {
            // Simple text content
            text_node = text_node.child(VirtualNode::text(&plain_text));
        }

        Ok(text_node)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[async_trait]
impl Widget for Text {
    fn widget_type(&self) -> &'static str {
        "text"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let text = Text::new("Hello World");
        assert_eq!(text.content(), "Hello World");
        assert_eq!(text.alignment(), TextAlign::Left);
        assert_eq!(text.wrap(), TextWrap::Word);
        assert!(text.max_width().is_none());
        assert!(text.max_height().is_none());
    }

    #[test]
    fn test_rich_text_creation() {
        let rich_text = RichText::new()
            .text("Hello ")
            .styled_text("World", TextStyle::new().bold(true).color(Color::RED));

        let text = Text::rich(rich_text);
        assert_eq!(text.content(), "Hello World");
        assert_eq!(text.rich_content().spans.len(), 2);

        let first_span = &text.rich_content().spans[0];
        assert_eq!(first_span.text, "Hello ");
        assert!(first_span.style.is_none());

        let second_span = &text.rich_content().spans[1];
        assert_eq!(second_span.text, "World");
        assert!(second_span.style.is_some());
        let style = second_span.style.as_ref().unwrap();
        assert!(style.bold);
        assert_eq!(style.color, Some(Color::RED));
    }

    #[test]
    fn test_text_style() {
        let style = TextStyle::new()
            .color(Color::BLUE)
            .background_color(Color::YELLOW)
            .bold(true)
            .italic(true)
            .underline(true)
            .strikethrough(true);

        assert_eq!(style.color, Some(Color::BLUE));
        assert_eq!(style.background_color, Some(Color::YELLOW));
        assert!(style.bold);
        assert!(style.italic);
        assert!(style.underline);
        assert!(style.strikethrough);
    }

    #[test]
    fn test_text_span_creation() {
        let span1 = TextSpan::new("Plain text");
        assert_eq!(span1.text, "Plain text");
        assert!(span1.style.is_none());

        let span2 = TextSpan::styled("Styled text", TextStyle::new().bold(true));
        assert_eq!(span2.text, "Styled text");
        assert!(span2.style.is_some());
        assert!(span2.style.unwrap().bold);

        let span3: TextSpan = "From string".into();
        assert_eq!(span3.text, "From string");
        assert!(span3.style.is_none());
    }

    #[test]
    fn test_rich_text_operations() {
        let mut rich_text = RichText::new();
        assert!(rich_text.is_empty());
        assert_eq!(rich_text.len(), 0);
        assert_eq!(rich_text.to_plain_text(), "");

        rich_text = rich_text
            .text("Hello ")
            .styled_text("beautiful ", TextStyle::new().italic(true))
            .text("world!");

        assert!(!rich_text.is_empty());
        assert_eq!(rich_text.len(), 22);
        assert_eq!(rich_text.to_plain_text(), "Hello beautiful world!");
        assert_eq!(rich_text.spans.len(), 3);

        let from_string: RichText = "Simple text".into();
        assert_eq!(from_string.to_plain_text(), "Simple text");
        assert_eq!(from_string.spans.len(), 1);
    }

    #[test]
    fn test_text_wrapping_word() {
        let text = Text::new("Hello beautiful world this is a test").with_wrap(TextWrap::Word);

        let lines = text.wrap_text(10);
        assert_eq!(
            lines,
            vec![
                "Hello".to_string(),
                "beautiful".to_string(),
                "world this".to_string(),
                "is a test".to_string()
            ]
        );

        let lines = text.wrap_text(15);
        assert_eq!(
            lines,
            vec![
                "Hello beautiful".to_string(),
                "world this is a".to_string(),
                "test".to_string()
            ]
        );

        // Test with very long word
        let text = Text::new("supercalifragilisticexpialidocious word").with_wrap(TextWrap::Word);
        let lines = text.wrap_text(10);
        assert_eq!(lines.len(), 5); // Long word gets broken + "word"
    }

    #[test]
    fn test_text_wrapping_char() {
        let text = Text::new("Hello world").with_wrap(TextWrap::Char);

        let lines = text.wrap_text(5);
        assert_eq!(
            lines,
            vec!["Hello".to_string(), " worl".to_string(), "d".to_string()]
        );

        let lines = text.wrap_text(3);
        assert_eq!(
            lines,
            vec![
                "Hel".to_string(),
                "lo ".to_string(),
                "wor".to_string(),
                "ld".to_string()
            ]
        );
    }

    #[test]
    fn test_text_wrapping_hyphen() {
        let text = Text::new("supercalifragilisticexpialidocious").with_wrap(TextWrap::Hyphen);

        let lines = text.wrap_text(10);
        assert!(lines.len() > 1);
        // Check that hyphens are added
        assert!(lines[0].ends_with('-'));

        let text = Text::new("Hello world test").with_wrap(TextWrap::Hyphen);
        let lines = text.wrap_text(10);
        assert_eq!(lines, vec!["Hello".to_string(), "world test".to_string()]);
    }

    #[test]
    fn test_text_alignment() {
        let text = Text::new("Hello");

        // Left alignment (default)
        assert_eq!(text.align_text("Hello", 10), "Hello     ");

        // Right alignment
        let text = text.with_alignment(TextAlign::Right);
        assert_eq!(text.align_text("Hello", 10), "     Hello");

        // Center alignment
        let text = text.with_alignment(TextAlign::Center);
        assert_eq!(text.align_text("Hello", 10), "  Hello   ");
        assert_eq!(text.align_text("Hi", 10), "    Hi    ");

        // Justify alignment
        let text = text.with_alignment(TextAlign::Justify);
        assert_eq!(text.align_text("Hello world", 15), "Hello     world");
        assert_eq!(text.align_text("A B C", 9), "A   B   C");
    }

    #[test]
    fn test_text_configuration() {
        let text = Text::new("Test")
            .with_alignment(TextAlign::Center)
            .with_wrap(TextWrap::Char)
            .with_max_width(100)
            .with_max_height(50)
            .with_style(Style::default());

        assert_eq!(text.alignment(), TextAlign::Center);
        assert_eq!(text.wrap(), TextWrap::Char);
        assert_eq!(text.max_width(), Some(100));
        assert_eq!(text.max_height(), Some(50));
    }

    #[test]
    fn test_text_content_updates() {
        let mut text = Text::new("Original");
        assert_eq!(text.content(), "Original");

        text.set_content("Updated");
        assert_eq!(text.content(), "Updated");

        let rich_content = RichText::new()
            .text("Rich ")
            .styled_text("content", TextStyle::new().bold(true));

        text.set_rich_content(rich_content);
        assert_eq!(text.content(), "Rich content");
        assert_eq!(text.rich_content().spans.len(), 2);
    }

    #[test]
    fn test_edge_cases() {
        // Empty text
        let text = Text::new("");
        assert_eq!(text.content(), "");
        let lines = text.wrap_text(10);
        assert_eq!(lines, vec!["".to_string()]);

        // Zero width wrapping
        let text = Text::new("Hello");
        let lines = text.wrap_text(0);
        let empty_vec: Vec<String> = vec![];
        assert_eq!(lines, empty_vec);

        // Text longer than width
        let text = Text::new("Hello");
        assert_eq!(text.align_text("Hello", 3), "Hello");

        // Single character wrapping
        let text = Text::new("A").with_wrap(TextWrap::Char);
        let lines = text.wrap_text(1);
        assert_eq!(lines, vec!["A".to_string()]);
    }
}
