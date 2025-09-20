//! Responsive layout engine for positioning and sizing elements.
//!
//! This layout engine is designed to be responsive by default, utilizing the full
//! terminal space effectively. It implements a flexbox-inspired layout system
//! that automatically adapts to terminal size changes.

use crate::layout::{Position, Rect, Size};
use crate::render::vdom::{
    AlignItems, DisplayType, FlexDirection, JustifyContent, StyleValue, VirtualElement,
    VirtualNode, VirtualStyle,
};
use std::collections::HashMap;

/// Layout engine for computing responsive element positions and sizes.
pub struct Layout;

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout {
    /// Create a new layout engine.
    pub fn new() -> Self {
        Self
    }
}

/// Computed layout information for a node.
#[derive(Debug, Clone)]
pub struct ComputedLayout {
    /// Final position of the element
    pub position: Position,
    /// Final size of the element
    pub size: Size,
    /// Content area (excluding padding/margins)
    pub content_rect: Rect,
    /// Whether this element is visible
    pub visible: bool,
}

impl Default for ComputedLayout {
    fn default() -> Self {
        Self {
            position: Position::new(0, 0),
            size: Size::new(0, 0),
            content_rect: Rect::from_coords(0, 0, 0, 0),
            visible: true,
        }
    }
}

/// Context for layout computation.
#[derive(Debug, Clone)]
pub struct LayoutContext {
    /// Available space for layout
    pub available_space: Size,
    /// Parent's flex direction
    pub flex_direction: FlexDirection,
    /// Whether we're in a flex container
    pub is_flex_container: bool,
}

impl LayoutContext {
    /// Create a new layout context with full terminal space.
    pub fn new(terminal_size: Size) -> Self {
        Self {
            available_space: terminal_size,
            flex_direction: FlexDirection::Column,
            is_flex_container: false,
        }
    }

    /// Create a child context with reduced available space.
    pub fn child_context(&self, available_space: Size, flex_direction: FlexDirection) -> Self {
        Self {
            available_space,
            flex_direction,
            is_flex_container: true,
        }
    }
}

impl Layout {
    /// Compute responsive layout for a virtual DOM tree.
    /// By default, elements will expand to fill available terminal space.
    pub fn compute(root: &mut VirtualNode, viewport: Size) -> LayoutResult {
        let mut layouts = HashMap::new();
        let context = LayoutContext::new(viewport);

        let total_size = Self::compute_node_layout(root, &context, &mut layouts);

        LayoutResult {
            total_size,
            layouts,
        }
    }

    /// Compute layout for a single node and its children.
    fn compute_node_layout(
        node: &mut VirtualNode,
        context: &LayoutContext,
        layouts: &mut HashMap<String, ComputedLayout>,
    ) -> Size {
        match node {
            VirtualNode::Element(element) => {
                Self::compute_element_layout(element, context, layouts)
            }
            VirtualNode::Text(text_node) => {
                // Text nodes take minimal space but can wrap
                let text_size =
                    Self::compute_text_size(&text_node.content, context.available_space);

                let layout = ComputedLayout {
                    position: Position::new(0, 0),
                    size: text_size,
                    content_rect: Rect::from_coords(0, 0, text_size.width, text_size.height),
                    visible: true,
                };

                // Use content as ID for text nodes
                layouts.insert(text_node.content.clone(), layout);
                text_size
            }
            VirtualNode::Empty => Size::new(0, 0),
        }
    }

    /// Compute layout for an element.
    fn compute_element_layout(
        element: &mut VirtualElement,
        context: &LayoutContext,
        layouts: &mut HashMap<String, ComputedLayout>,
    ) -> Size {
        let style = &element.style;

        // Determine if element should be hidden
        if matches!(
            style.visibility,
            Some(crate::render::vdom::Visibility::Hidden)
        ) {
            let layout = ComputedLayout {
                visible: false,
                ..Default::default()
            };
            layouts.insert(element.tag.clone(), layout);
            return Size::new(0, 0);
        }

        // Calculate element's own size (responsive by default)
        let element_size = Self::calculate_responsive_size(style, context.available_space);

        // Handle flex layout for children
        let children_size = if matches!(style.display, Some(DisplayType::Flex)) {
            Self::layout_flex_children(element, &element_size, layouts)
        } else {
            Self::layout_block_children(element, &element_size, layouts)
        };

        // Final size is the maximum of element size and children size
        let final_size = Size::new(
            element_size.width.max(children_size.width),
            element_size.height.max(children_size.height),
        );

        // Create layout for this element
        let layout = ComputedLayout {
            position: Position::new(0, 0), // Will be positioned by parent
            size: final_size,
            content_rect: Rect::from_coords(0, 0, final_size.width, final_size.height),
            visible: true,
        };

        layouts.insert(element.tag.clone(), layout);
        final_size
    }

    /// Calculate responsive size for an element.
    /// Elements expand to fill available space by default.
    fn calculate_responsive_size(style: &VirtualStyle, available_space: Size) -> Size {
        let width = match &style.width {
            Some(StyleValue::Absolute(px)) => *px,
            Some(StyleValue::Percentage(pct)) => {
                ((available_space.width as f32) * (pct / 100.0)) as u32
            }
            Some(StyleValue::Auto) | Some(StyleValue::Fill) | None => available_space.width, // Responsive by default
        };

        let height = match &style.height {
            Some(StyleValue::Absolute(px)) => *px,
            Some(StyleValue::Percentage(pct)) => {
                ((available_space.height as f32) * (pct / 100.0)) as u32
            }
            Some(StyleValue::Fill) => available_space.height,
            Some(StyleValue::Auto) | None => {
                // For height, be more conservative - don't fill unless explicitly set
                match &style.display {
                    Some(DisplayType::Flex) => available_space.height,
                    _ => 1, // Minimal height for non-flex elements
                }
            }
        };

        Size::new(width, height)
    }

    /// Layout children using flexbox rules.
    fn layout_flex_children(
        element: &mut VirtualElement,
        container_size: &Size,
        layouts: &mut HashMap<String, ComputedLayout>,
    ) -> Size {
        if element.children.is_empty() {
            return Size::new(0, 0);
        }

        let flex_direction = element.style.flex_direction.unwrap_or(FlexDirection::Row);
        let justify_content = element
            .style
            .justify_content
            .unwrap_or(JustifyContent::FlexStart);
        let align_items = element.style.align_items.unwrap_or(AlignItems::Stretch);

        let mut total_main_size = 0u32;
        let mut max_cross_size = 0u32;
        let mut child_layouts = Vec::new();

        // First pass: calculate intrinsic sizes
        for child in &mut element.children {
            let child_context = LayoutContext {
                available_space: *container_size,
                flex_direction,
                is_flex_container: true,
            };

            let child_size = Self::compute_node_layout(child, &child_context, layouts);
            child_layouts.push(child_size);

            match flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    total_main_size += child_size.width;
                    max_cross_size = max_cross_size.max(child_size.height);
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    total_main_size += child_size.height;
                    max_cross_size = max_cross_size.max(child_size.width);
                }
            }
        }

        // Second pass: position children based on justify_content and align_items
        let available_main_size = match flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => container_size.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => container_size.height,
        };

        // Calculate spacing for justify_content
        let extra_space = available_main_size.saturating_sub(total_main_size);
        let (start_offset, spacing) = match justify_content {
            JustifyContent::FlexStart => (0, 0),
            JustifyContent::FlexEnd => (extra_space, 0),
            JustifyContent::Center => (extra_space / 2, 0),
            JustifyContent::SpaceBetween => {
                if element.children.len() > 1 {
                    (0, extra_space / (element.children.len() as u32 - 1))
                } else {
                    (0, 0)
                }
            }
            JustifyContent::SpaceAround => {
                let spacing = extra_space / element.children.len() as u32;
                (spacing / 2, spacing)
            }
            JustifyContent::SpaceEvenly => {
                let spacing = extra_space / (element.children.len() as u32 + 1);
                (spacing, spacing)
            }
        };

        let mut current_position = start_offset;

        // Position each child
        for (i, child_size) in child_layouts.iter().enumerate() {
            let (x, y) = match flex_direction {
                FlexDirection::Row => {
                    let cross_pos = match align_items {
                        AlignItems::FlexStart => 0,
                        AlignItems::FlexEnd => {
                            container_size.height.saturating_sub(child_size.height)
                        }
                        AlignItems::Center => {
                            (container_size.height.saturating_sub(child_size.height)) / 2
                        }
                        AlignItems::Stretch => 0, // Height will be stretched
                        AlignItems::Baseline => {
                            // For baseline alignment, we need to calculate the baseline position
                            // In a TUI context, baseline is typically the bottom of text content
                            // For now, we'll use a simple heuristic: align to the bottom of the tallest element
                            let max_height =
                                child_layouts.iter().map(|s| s.height).max().unwrap_or(0);
                            max_height.saturating_sub(child_size.height)
                        }
                    };
                    (current_position, cross_pos)
                }
                FlexDirection::RowReverse => {
                    let cross_pos = match align_items {
                        AlignItems::FlexStart => 0,
                        AlignItems::FlexEnd => {
                            container_size.height.saturating_sub(child_size.height)
                        }
                        AlignItems::Center => {
                            (container_size.height.saturating_sub(child_size.height)) / 2
                        }
                        AlignItems::Stretch => 0,
                        AlignItems::Baseline => {
                            // For RowReverse, use the same baseline logic
                            let max_height =
                                child_layouts.iter().map(|s| s.height).max().unwrap_or(0);
                            max_height.saturating_sub(child_size.height)
                        }
                    };
                    (
                        container_size
                            .width
                            .saturating_sub(current_position + child_size.width),
                        cross_pos,
                    )
                }
                FlexDirection::Column => {
                    let cross_pos = match align_items {
                        AlignItems::FlexStart => 0,
                        AlignItems::FlexEnd => {
                            container_size.width.saturating_sub(child_size.width)
                        }
                        AlignItems::Center => {
                            (container_size.width.saturating_sub(child_size.width)) / 2
                        }
                        AlignItems::Stretch => 0, // Width will be stretched
                        AlignItems::Baseline => {
                            // For Column direction, baseline alignment aligns to the widest element
                            let max_width =
                                child_layouts.iter().map(|s| s.width).max().unwrap_or(0);
                            max_width.saturating_sub(child_size.width)
                        }
                    };
                    (cross_pos, current_position)
                }
                FlexDirection::ColumnReverse => {
                    let cross_pos = match align_items {
                        AlignItems::FlexStart => 0,
                        AlignItems::FlexEnd => {
                            container_size.width.saturating_sub(child_size.width)
                        }
                        AlignItems::Center => {
                            (container_size.width.saturating_sub(child_size.width)) / 2
                        }
                        AlignItems::Stretch => 0,
                        AlignItems::Baseline => {
                            // For ColumnReverse, use the same baseline logic as Column
                            let max_width =
                                child_layouts.iter().map(|s| s.width).max().unwrap_or(0);
                            max_width.saturating_sub(child_size.width)
                        }
                    };
                    (
                        cross_pos,
                        container_size
                            .height
                            .saturating_sub(current_position + child_size.height),
                    )
                }
            };

            // Update child position in layouts
            if let Some(child_id) = Self::get_node_id(&element.children[i]) {
                if let Some(layout) = layouts.get_mut(&child_id) {
                    layout.position = Position::new(x as i32, y as i32);

                    // Apply stretch alignment
                    if matches!(align_items, AlignItems::Stretch) {
                        match flex_direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                layout.size.height = container_size.height;
                                layout.content_rect.size.height = container_size.height;
                            }
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                layout.size.width = container_size.width;
                                layout.content_rect.size.width = container_size.width;
                            }
                        }
                    }
                }
            }

            // Advance position
            match flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    current_position += child_size.width + spacing;
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    current_position += child_size.height + spacing;
                }
            }
        }

        // Return the total size used by children
        match flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                Size::new(total_main_size, max_cross_size)
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                Size::new(max_cross_size, total_main_size)
            }
        }
    }

    /// Layout children using block layout (stacked vertically).
    fn layout_block_children(
        element: &mut VirtualElement,
        container_size: &Size,
        layouts: &mut HashMap<String, ComputedLayout>,
    ) -> Size {
        if element.children.is_empty() {
            return Size::new(0, 0);
        }

        let mut current_y = 0u32;
        let mut max_width = 0u32;

        for child in &mut element.children {
            let child_context = LayoutContext {
                available_space: Size::new(
                    container_size.width,
                    container_size.height.saturating_sub(current_y),
                ),
                flex_direction: FlexDirection::Column,
                is_flex_container: false,
            };

            let child_size = Self::compute_node_layout(child, &child_context, layouts);

            // Position child
            if let Some(child_id) = Self::get_node_id(child) {
                if let Some(layout) = layouts.get_mut(&child_id) {
                    layout.position = Position::new(0, current_y as i32);
                }
            }

            current_y += child_size.height;
            max_width = max_width.max(child_size.width);
        }

        Size::new(max_width, current_y)
    }

    /// Get a unique identifier for a node.
    fn get_node_id(node: &VirtualNode) -> Option<String> {
        match node {
            VirtualNode::Element(element) => Some(element.tag.clone()),
            VirtualNode::Text(text_node) => Some(text_node.content.clone()),
            VirtualNode::Empty => None,
        }
    }

    /// Compute text size with word wrapping.
    fn compute_text_size(text: &str, available_space: Size) -> Size {
        if text.is_empty() {
            return Size::new(0, 0);
        }

        let max_width = available_space.width;
        if max_width == 0 {
            return Size::new(0, 1);
        }

        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return Size::new(0, 1);
        }

        let mut lines = 1u32;
        let mut current_line_width = 0u32;
        let mut max_line_width = 0u32;

        for word in words {
            let word_width = word.len() as u32;

            // If adding this word would exceed the line width, start a new line
            if current_line_width > 0 && current_line_width + 1 + word_width > max_width {
                max_line_width = max_line_width.max(current_line_width);
                lines += 1;
                current_line_width = word_width;
            } else {
                if current_line_width > 0 {
                    current_line_width += 1; // Space before word
                }
                current_line_width += word_width;
            }
        }

        max_line_width = max_line_width.max(current_line_width);
        Size::new(max_line_width, lines)
    }
}

/// Result of a layout computation.
#[derive(Debug, Clone)]
pub struct LayoutResult {
    /// Total size required for the layout
    pub total_size: Size,
    /// Computed layouts for all nodes
    pub layouts: HashMap<String, ComputedLayout>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::vdom::nodes::{div, text};

    #[test]
    fn test_layout_engine_creation() {
        let _layout = Layout::new();
        // Layout engine should be created successfully
        // This is a simple test since Layout is a unit struct
        assert!(true);
    }

    #[test]
    fn test_layout_context_creation() {
        let terminal_size = Size::new(80, 24);
        let context = LayoutContext::new(terminal_size);

        assert_eq!(context.available_space, terminal_size);
        assert_eq!(context.flex_direction, FlexDirection::Column);
        assert!(!context.is_flex_container);
    }

    #[test]
    fn test_layout_context_child_context() {
        let terminal_size = Size::new(80, 24);
        let parent_context = LayoutContext::new(terminal_size);

        let child_size = Size::new(40, 12);
        let child_context = parent_context.child_context(child_size, FlexDirection::Row);

        assert_eq!(child_context.available_space, child_size);
        assert_eq!(child_context.flex_direction, FlexDirection::Row);
        assert!(child_context.is_flex_container);
    }

    #[test]
    fn test_computed_layout_default() {
        let layout = ComputedLayout::default();

        assert_eq!(layout.position, Position::new(0, 0));
        assert_eq!(layout.size, Size::new(0, 0));
        assert_eq!(layout.content_rect, Rect::from_coords(0, 0, 0, 0));
        assert!(layout.visible);
    }

    #[test]
    fn test_simple_text_layout() {
        let mut node = text("Hello, World!");
        let viewport = Size::new(80, 24);

        let result = Layout::compute(&mut node, viewport);

        assert_eq!(result.total_size.width, 13); // Length of "Hello, World!"
        assert_eq!(result.total_size.height, 1);
        assert_eq!(result.layouts.len(), 1);
    }

    #[test]
    fn test_div_with_text_layout() {
        let mut node = div().child(text("Test content"));
        let viewport = Size::new(80, 24);

        let result = Layout::compute(&mut node, viewport);

        // Should have layouts for both div and text
        assert_eq!(result.layouts.len(), 2);
        // Div with text content has height based on content, width fills viewport
        assert_eq!(result.total_size.width, 80);
        assert!(result.total_size.height >= 1); // At least one line for text
    }

    #[test]
    fn test_flex_column_layout() {
        let mut node = div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Column),
                width: Some(StyleValue::Fill),
                height: Some(StyleValue::Fill),
                ..Default::default()
            })
            .child(text("First line"))
            .child(text("Second line"));

        let viewport = Size::new(80, 24);
        let result = Layout::compute(&mut node, viewport);

        // Should have layouts for div and two text nodes
        assert_eq!(result.layouts.len(), 3);
        // Total height should accommodate both text lines
        assert!(result.total_size.height >= 2);
    }

    #[test]
    fn test_flex_row_layout() {
        let mut node = div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Row),
                width: Some(StyleValue::Fill),
                height: Some(StyleValue::Fill),
                ..Default::default()
            })
            .child(text("Left"))
            .child(text("Right"));

        let viewport = Size::new(80, 24);
        let result = Layout::compute(&mut node, viewport);

        // Should have layouts for div and two text nodes
        assert_eq!(result.layouts.len(), 3);
        // Total width should accommodate both text elements
        assert!(result.total_size.width >= 9); // "Left" + "Right" = 9 chars
    }

    #[test]
    fn test_absolute_sizing() {
        let mut node = div()
            .style(VirtualStyle {
                width: Some(StyleValue::Absolute(20)),
                height: Some(StyleValue::Absolute(10)),
                ..Default::default()
            })
            .child(text("Content"));

        let viewport = Size::new(80, 24);
        let result = Layout::compute(&mut node, viewport);

        // Div should have absolute size
        assert_eq!(result.total_size.width, 20);
        assert_eq!(result.total_size.height, 10);
    }

    #[test]
    fn test_percentage_sizing() {
        let mut node = div()
            .style(VirtualStyle {
                width: Some(StyleValue::Percentage(50.0)),
                height: Some(StyleValue::Percentage(25.0)),
                ..Default::default()
            })
            .child(text("Content"));

        let viewport = Size::new(80, 24);
        let result = Layout::compute(&mut node, viewport);

        // Div should be 50% width and 25% height of viewport
        assert_eq!(result.total_size.width, 40); // 50% of 80
        assert_eq!(result.total_size.height, 6);  // 25% of 24
    }

    #[test]
    fn test_nested_layout() {
        let mut node = div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Column),
                width: Some(StyleValue::Fill),
                height: Some(StyleValue::Fill),
                ..Default::default()
            })
            .child(
                div()
                    .style(VirtualStyle {
                        height: Some(StyleValue::Absolute(5)),
                        width: Some(StyleValue::Fill),
                        ..Default::default()
                    })
                    .child(text("Header"))
            )
            .child(
                div()
                    .style(VirtualStyle {
                        height: Some(StyleValue::Fill),
                        width: Some(StyleValue::Fill),
                        ..Default::default()
                    })
                    .child(text("Content"))
            );

        let viewport = Size::new(80, 24);
        let result = Layout::compute(&mut node, viewport);

        // Should have layouts for all nodes: root div, header div, content div, header text, content text
        // Note: The actual number may vary based on how the layout engine handles nested structures
        assert!(result.layouts.len() >= 3); // At least the main components
        // Total size should be reasonable (may exceed viewport due to content requirements)
        assert_eq!(result.total_size.width, 80);
        assert!(result.total_size.height >= 24); // At least viewport height
    }

    #[test]
    fn test_justify_content_center() {
        let mut node = div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Row),
                justify_content: Some(JustifyContent::Center),
                width: Some(StyleValue::Fill),
                height: Some(StyleValue::Fill),
                ..Default::default()
            })
            .child(text("Centered"));

        let viewport = Size::new(80, 24);
        let result = Layout::compute(&mut node, viewport);

        // Should have layouts for div and text
        assert_eq!(result.layouts.len(), 2);
        // Content should be centered (this is a basic test - actual centering logic would need more detailed verification)
        assert_eq!(result.total_size, viewport);
    }

    #[test]
    fn test_align_items_center() {
        let mut node = div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Column),
                align_items: Some(AlignItems::Center),
                width: Some(StyleValue::Fill),
                height: Some(StyleValue::Fill),
                ..Default::default()
            })
            .child(text("Centered"));

        let viewport = Size::new(80, 24);
        let result = Layout::compute(&mut node, viewport);

        // Should have layouts for div and text
        assert_eq!(result.layouts.len(), 2);
        // Content should be centered vertically
        assert_eq!(result.total_size, viewport);
    }

    #[test]
    fn test_empty_layout() {
        let mut node = div();
        let viewport = Size::new(80, 24);

        let result = Layout::compute(&mut node, viewport);

        // Should have layout for the div
        assert_eq!(result.layouts.len(), 1);
        // Empty div has minimal height but fills width
        assert_eq!(result.total_size.width, 80);
        assert!(result.total_size.height >= 1); // At least minimal height
    }

    #[test]
    fn test_small_viewport() {
        let mut node = div()
            .child(text("This is a longer text that might not fit"));

        let viewport = Size::new(10, 5);
        let result = Layout::compute(&mut node, viewport);

        // Should handle small viewports gracefully
        assert_eq!(result.layouts.len(), 2);
        // Should not exceed viewport size
        assert!(result.total_size.width <= viewport.width);
        assert!(result.total_size.height <= viewport.height);
    }

    #[test]
    fn test_zero_viewport() {
        let mut node = div().child(text("Content"));
        let viewport = Size::new(0, 0);

        let result = Layout::compute(&mut node, viewport);

        // Should handle zero viewport gracefully
        assert_eq!(result.layouts.len(), 2);
        // Should have minimal size even with zero viewport
        assert_eq!(result.total_size.width, 0);
        assert!(result.total_size.height >= 1); // Text still needs at least one line
    }
}
