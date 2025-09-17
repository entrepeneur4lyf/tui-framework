//! Virtual DOM implementation for efficient rendering.

use crate::layout::Rect;
use crate::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a node in the virtual DOM tree.
#[derive(Debug, Clone, PartialEq)]
pub enum VirtualNode {
    /// An element with tag, attributes, and children
    Element(Box<VirtualElement>),
    /// A text node with content
    Text(VirtualText),
    /// An empty/null node
    Empty,
}

impl VirtualNode {
    /// Create an empty virtual node.
    pub fn empty() -> Self {
        VirtualNode::Empty
    }

    /// Create a text node.
    pub fn text(content: impl Into<String>) -> Self {
        VirtualNode::Text(VirtualText {
            content: content.into(),
        })
    }

    /// Create an element node.
    pub fn element(tag: impl Into<String>) -> Self {
        VirtualNode::Element(Box::new(VirtualElement {
            tag: tag.into(),
            attributes: HashMap::new(),
            style: VirtualStyle::default(),
            children: Vec::new(),
            layout: None,
        }))
    }

    /// Add a child to this node (if it's an element).
    pub fn child(mut self, child: VirtualNode) -> Self {
        if let VirtualNode::Element(ref mut element) = self {
            element.children.push(child);
        }
        self
    }

    /// Add multiple children to this node (if it's an element).
    pub fn children(mut self, children: Vec<VirtualNode>) -> Self {
        if let VirtualNode::Element(ref mut element) = self {
            element.children.extend(children);
        }
        self
    }

    /// Set an attribute on this node (if it's an element).
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if let VirtualNode::Element(ref mut element) = self {
            element.attributes.insert(key.into(), value.into());
        }
        self
    }

    /// Set the style on this node (if it's an element).
    pub fn style(mut self, style: VirtualStyle) -> Self {
        if let VirtualNode::Element(ref mut element) = self {
            element.style = style;
        }
        self
    }

    /// Get the tag name if this is an element.
    pub fn tag(&self) -> Option<&str> {
        match self {
            VirtualNode::Element(element) => Some(&element.tag),
            _ => None,
        }
    }

    /// Get the text content if this is a text node.
    pub fn text_content(&self) -> Option<&str> {
        match self {
            VirtualNode::Text(text) => Some(&text.content),
            _ => None,
        }
    }

    /// Check if this node is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, VirtualNode::Empty)
    }

    /// Get the children if this is an element.
    pub fn get_children(&self) -> &[VirtualNode] {
        match self {
            VirtualNode::Element(element) => &element.children,
            _ => &[],
        }
    }

    /// Get the layout information if available.
    pub fn layout(&self) -> Option<&LayoutInfo> {
        match self {
            VirtualNode::Element(element) => element.layout.as_ref(),
            _ => None,
        }
    }

    /// Set the layout information.
    pub fn set_layout(&mut self, layout: LayoutInfo) {
        if let VirtualNode::Element(element) = self {
            element.layout = Some(layout);
        }
    }
}

/// Represents an element in the virtual DOM.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualElement {
    /// The tag name (e.g., "div", "button", "text")
    pub tag: String,
    /// Element attributes
    pub attributes: HashMap<String, String>,
    /// Element style
    pub style: VirtualStyle,
    /// Child nodes
    pub children: Vec<VirtualNode>,
    /// Layout information (computed during layout pass)
    pub layout: Option<LayoutInfo>,
}

/// Represents a text node in the virtual DOM.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualText {
    /// The text content
    pub content: String,
}

/// Style information for virtual elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub struct VirtualStyle {
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
    /// Padding
    pub padding: Option<StyleSpacing>,
    /// Margin
    pub margin: Option<StyleSpacing>,
    /// Display type
    pub display: Option<DisplayType>,
    /// Flex direction
    pub flex_direction: Option<FlexDirection>,
    /// Justify content
    pub justify_content: Option<JustifyContent>,
    /// Align items
    pub align_items: Option<AlignItems>,
    /// Text alignment
    pub text_align: Option<TextAlign>,
    /// Font weight
    pub font_weight: Option<FontWeight>,
    /// Font style
    pub font_style: Option<FontStyle>,
    /// Visibility
    pub visibility: Option<Visibility>,
    /// Z-index
    pub z_index: Option<i32>,
}


/// Style value that can be absolute or relative.
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

/// Spacing values for padding and margin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSpacing {
    pub top: StyleValue,
    pub right: StyleValue,
    pub bottom: StyleValue,
    pub left: StyleValue,
}

/// Display types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayType {
    Block,
    Inline,
    Flex,
    Grid,
    None,
}

/// Flex direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

/// Justify content values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Align items values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

/// Text alignment values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

/// Font weight values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontWeight {
    Normal,
    Bold,
    Light,
}

/// Font style values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Underline,
}

/// Visibility values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

/// Layout information computed during the layout pass.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutInfo {
    /// The computed rectangle for this element
    pub rect: Rect,
    /// Whether this element is visible
    pub visible: bool,
    /// Z-index for layering
    pub z_index: i32,
}

impl LayoutInfo {
    /// Create new layout info.
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            visible: true,
            z_index: 0,
        }
    }
}

/// Helper functions for creating common virtual nodes.
pub mod nodes {
    use super::*;

    /// Create a div element.
    pub fn div() -> VirtualNode {
        VirtualNode::element("div")
    }

    /// Create a text element.
    pub fn text(content: impl Into<String>) -> VirtualNode {
        VirtualNode::text(content)
    }

    /// Create a button element.
    pub fn button(label: impl Into<String>) -> VirtualNode {
        VirtualNode::element("button").child(VirtualNode::text(label))
    }

    /// Create an input element.
    pub fn input() -> VirtualNode {
        VirtualNode::element("input")
    }

    /// Create a container element.
    pub fn container() -> VirtualNode {
        VirtualNode::element("container")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::nodes::*;

    #[test]
    fn test_virtual_node_creation() {
        let node = div()
            .child(text("Hello"))
            .child(button("Click me"));

        assert_eq!(node.tag(), Some("div"));
        assert_eq!(node.get_children().len(), 2);
    }

    #[test]
    fn test_text_node() {
        let node = text("Hello, world!");
        assert_eq!(node.text_content(), Some("Hello, world!"));
    }

    #[test]
    fn test_empty_node() {
        let node = VirtualNode::empty();
        assert!(node.is_empty());
    }

    #[test]
    fn test_node_attributes() {
        let node = div()
            .attr("id", "test")
            .attr("class", "container");

        if let VirtualNode::Element(element) = node {
            assert_eq!(element.attributes.get("id"), Some(&"test".to_string()));
            assert_eq!(element.attributes.get("class"), Some(&"container".to_string()));
        } else {
            panic!("Expected element node");
        }
    }
}
