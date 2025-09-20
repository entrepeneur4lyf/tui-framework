//! Input widget implementation.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::event::types::{KeyEvent, NcKey};
use crate::render::{RenderContext, VirtualNode};
use crate::style::properties::Style;
use crate::widget::Widget;
use async_trait::async_trait;
use std::sync::Arc;

/// Input validation function type.
pub type ValidationFn = Arc<dyn Fn(&str) -> std::result::Result<(), String> + Send + Sync>;

/// Text selection range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Start position of selection (inclusive).
    pub start: usize,
    /// End position of selection (exclusive).
    pub end: usize,
}

impl Selection {
    /// Create a new selection.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: start.min(end),
            end: start.max(end),
        }
    }

    /// Check if the selection is empty (cursor position).
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the length of the selection.
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

/// An input widget for text entry.
pub struct Input {
    base: BaseComponent,
    value: String,
    placeholder: String,
    enabled: bool,
    cursor_position: usize,
    selection: Option<Selection>,
    max_length: Option<usize>,
    validator: Option<ValidationFn>,
    style: Style,
    focused: bool,
}

impl Input {
    /// Create a new input widget.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("Input"),
            value: String::new(),
            placeholder: String::new(),
            enabled: true,
            cursor_position: 0,
            selection: None,
            max_length: None,
            validator: None,
            style: Style::default(),
            focused: false,
        }
    }

    /// Set the input value.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    /// Get the input value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the placeholder text.
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
        self.placeholder = placeholder.into();
    }

    /// Get the placeholder text.
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Set whether the input is enabled.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if the input is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set the maximum length of the input.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Set a validation function for the input.
    pub fn with_validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> std::result::Result<(), String> + Send + Sync + 'static,
    {
        self.validator = Some(Arc::new(validator));
        self
    }

    /// Set the input style.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get the cursor position.
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Set the cursor position.
    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor_position = position.min(self.value.len());
        self.selection = None; // Clear selection when moving cursor
    }

    /// Get the current selection.
    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }

    /// Set the selection.
    pub fn set_selection(&mut self, selection: Option<Selection>) {
        if let Some(sel) = selection {
            let max_pos = self.value.len();
            self.selection = Some(Selection::new(sel.start.min(max_pos), sel.end.min(max_pos)));
            self.cursor_position = self.selection.unwrap().end;
        } else {
            self.selection = None;
        }
    }

    /// Check if the input is focused.
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Set the focus state.
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        if !focused {
            self.selection = None; // Clear selection when losing focus
        }
    }

    /// Insert text at the current cursor position.
    pub fn insert_text(&mut self, text: &str) -> std::result::Result<(), String> {
        // Store original state for rollback
        let original_value = self.value.clone();
        let original_cursor = self.cursor_position;
        let original_selection = self.selection;

        // Remove selected text if any
        if let Some(selection) = self.selection {
            self.value.drain(selection.start..selection.end);
            self.cursor_position = selection.start;
            self.selection = None;
        }

        // Check max length after removing selection
        if let Some(max_len) = self.max_length {
            if self.value.len() + text.len() > max_len {
                // Rollback
                self.value = original_value;
                self.cursor_position = original_cursor;
                self.selection = original_selection;
                return Err("Text exceeds maximum length".to_string());
            }
        }

        // Insert new text
        self.value.insert_str(self.cursor_position, text);
        self.cursor_position += text.len();

        // Validate if validator is set
        if let Some(ref validator) = self.validator {
            if let Err(e) = validator(&self.value) {
                // Rollback on validation failure
                self.value = original_value;
                self.cursor_position = original_cursor;
                self.selection = original_selection;
                return Err(e);
            }
        }

        Ok(())
    }

    /// Delete character at cursor position (backspace).
    pub fn delete_char_backward(&mut self) {
        if let Some(selection) = self.selection {
            // Delete selected text
            self.value.drain(selection.start..selection.end);
            self.cursor_position = selection.start;
            self.selection = None;
        } else if self.cursor_position > 0 {
            // Delete character before cursor
            self.cursor_position -= 1;
            self.value.remove(self.cursor_position);
        }
    }

    /// Delete character after cursor position (delete key).
    pub fn delete_char_forward(&mut self) {
        if let Some(selection) = self.selection {
            // Delete selected text
            self.value.drain(selection.start..selection.end);
            self.cursor_position = selection.start;
            self.selection = None;
        } else if self.cursor_position < self.value.len() {
            // Delete character after cursor
            self.value.remove(self.cursor_position);
        }
    }

    /// Move cursor left.
    pub fn move_cursor_left(&mut self, select: bool) {
        if select && self.selection.is_none() {
            // Start selection
            self.selection = Some(Selection::new(self.cursor_position, self.cursor_position));
        }

        if self.cursor_position > 0 {
            self.cursor_position -= 1;

            if select {
                if let Some(ref mut selection) = self.selection {
                    selection.start = self.cursor_position;
                }
            } else {
                self.selection = None;
            }
        }
    }

    /// Move cursor right.
    pub fn move_cursor_right(&mut self, select: bool) {
        if select && self.selection.is_none() {
            // Start selection
            self.selection = Some(Selection::new(self.cursor_position, self.cursor_position));
        }

        if self.cursor_position < self.value.len() {
            self.cursor_position += 1;

            if select {
                if let Some(ref mut selection) = self.selection {
                    selection.end = self.cursor_position;
                }
            } else {
                self.selection = None;
            }
        }
    }

    /// Move cursor to beginning of input.
    pub fn move_cursor_home(&mut self, select: bool) {
        if select && self.selection.is_none() {
            self.selection = Some(Selection::new(self.cursor_position, self.cursor_position));
        }

        self.cursor_position = 0;

        if select {
            if let Some(ref mut selection) = self.selection {
                selection.start = 0;
            }
        } else {
            self.selection = None;
        }
    }

    /// Move cursor to end of input.
    pub fn move_cursor_end(&mut self, select: bool) {
        if select && self.selection.is_none() {
            self.selection = Some(Selection::new(self.cursor_position, self.cursor_position));
        }

        self.cursor_position = self.value.len();

        if select {
            if let Some(ref mut selection) = self.selection {
                selection.end = self.value.len();
            }
        } else {
            self.selection = None;
        }
    }

    /// Handle a key event.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if !self.enabled || !self.focused {
            return false;
        }

        match event.key {
            NcKey::Backspace => {
                self.delete_char_backward();
                true
            }
            NcKey::Del => {
                self.delete_char_forward();
                true
            }
            NcKey::Left => {
                self.move_cursor_left(event.shift());
                true
            }
            NcKey::Right => {
                self.move_cursor_right(event.shift());
                true
            }
            NcKey::Home => {
                self.move_cursor_home(event.shift());
                true
            }
            NcKey::End => {
                self.move_cursor_end(event.shift());
                true
            }
            _ => false, // Let other keys be handled as character input
        }
    }

    /// Validate the current input value.
    pub fn validate(&self) -> std::result::Result<(), String> {
        if let Some(ref validator) = self.validator {
            validator(&self.value)
        } else {
            Ok(())
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Component for Input {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "Input"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let display_value = if self.value.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.value
        };

        let mut input_node = VirtualNode::element("input")
            .attr("value", display_value)
            .attr("placeholder", &self.placeholder)
            .attr("enabled", self.enabled.to_string())
            .attr("focused", self.focused.to_string())
            .attr("cursor_position", self.cursor_position.to_string());

        // Add selection information if present
        if let Some(selection) = self.selection {
            input_node = input_node
                .attr("selection_start", selection.start.to_string())
                .attr("selection_end", selection.end.to_string());
        }

        // Add validation state
        let is_valid = self.validate().is_ok();
        input_node = input_node.attr("valid", is_valid.to_string());

        // Add max length if set
        if let Some(max_len) = self.max_length {
            input_node = input_node.attr("max_length", max_len.to_string());
        }

        Ok(input_node)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[async_trait]
impl Widget for Input {
    fn widget_type(&self) -> &'static str {
        "input"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::types::{KeyEvent, NcKey};

    #[test]
    fn test_input_creation() {
        let input = Input::new();
        assert_eq!(input.value(), "");
        assert_eq!(input.placeholder(), "");
        assert!(input.is_enabled());
        assert!(!input.is_focused());
        assert_eq!(input.cursor_position(), 0);
        assert!(input.selection().is_none());
    }

    #[test]
    fn test_input_text_insertion() {
        let mut input = Input::new();
        input.set_focused(true);

        assert!(input.insert_text("Hello").is_ok());
        assert_eq!(input.value(), "Hello");
        assert_eq!(input.cursor_position(), 5);

        input.set_cursor_position(2);
        assert!(input.insert_text(" World").is_ok());
        assert_eq!(input.value(), "He Worldllo");
        assert_eq!(input.cursor_position(), 8);
    }

    #[test]
    fn test_input_max_length() {
        let mut input = Input::new().with_max_length(5);
        input.set_focused(true);

        assert!(input.insert_text("Hello").is_ok());
        assert_eq!(input.value(), "Hello");

        let result = input.insert_text(" World");
        assert!(result.is_err());
        assert_eq!(input.value(), "Hello"); // Should remain unchanged
    }

    #[test]
    fn test_input_validation() {
        let mut input = Input::new().with_validator(|text| {
            if text.contains("bad") {
                Err("Contains forbidden word".to_string())
            } else {
                Ok(())
            }
        });
        input.set_focused(true);

        assert!(input.insert_text("good text").is_ok());
        assert_eq!(input.value(), "good text");

        let result = input.insert_text(" bad");
        assert!(result.is_err());
        assert_eq!(input.value(), "good text"); // Should remain unchanged
    }

    #[test]
    fn test_cursor_movement() {
        let mut input = Input::new();
        input.set_value("Hello World");
        input.set_focused(true);
        input.set_cursor_position(5);

        input.move_cursor_left(false);
        assert_eq!(input.cursor_position(), 4);
        assert!(input.selection().is_none());

        input.move_cursor_right(false);
        assert_eq!(input.cursor_position(), 5);

        input.move_cursor_home(false);
        assert_eq!(input.cursor_position(), 0);

        input.move_cursor_end(false);
        assert_eq!(input.cursor_position(), 11);
    }

    #[test]
    fn test_text_selection() {
        let mut input = Input::new();
        input.set_value("Hello World");
        input.set_focused(true);
        input.set_cursor_position(5);

        // Select text by moving with shift
        input.move_cursor_right(true);
        input.move_cursor_right(true);

        let selection = input.selection().unwrap();
        assert_eq!(selection.start, 5);
        assert_eq!(selection.end, 7);
        assert_eq!(selection.len(), 2);

        // Insert text should replace selection
        assert!(input.insert_text("_").is_ok());
        assert_eq!(input.value(), "Hello_orld");
        assert_eq!(input.cursor_position(), 6);
        assert!(input.selection().is_none());
    }

    #[test]
    fn test_key_event_handling() {
        let mut input = Input::new();
        input.set_value("Hello");
        input.set_focused(true);
        input.set_cursor_position(5);

        // Test backspace
        let backspace_event = KeyEvent::new(NcKey::Backspace);
        assert!(input.handle_key_event(&backspace_event));
        assert_eq!(input.value(), "Hell");
        assert_eq!(input.cursor_position(), 4);

        // Test delete
        input.set_cursor_position(2);
        let delete_event = KeyEvent::new(NcKey::Del);
        assert!(input.handle_key_event(&delete_event));
        assert_eq!(input.value(), "Hel");
        assert_eq!(input.cursor_position(), 2);

        // Test arrow keys
        let left_event = KeyEvent::new(NcKey::Left);
        assert!(input.handle_key_event(&left_event));
        assert_eq!(input.cursor_position(), 1);

        let right_event = KeyEvent::new(NcKey::Right);
        assert!(input.handle_key_event(&right_event));
        assert_eq!(input.cursor_position(), 2);
    }

    #[test]
    fn test_disabled_input() {
        let mut input = Input::new();
        input.set_enabled(false);
        input.set_focused(true);

        let key_event = KeyEvent::new(NcKey::Backspace);
        assert!(!input.handle_key_event(&key_event));

        assert!(input.insert_text("test").is_ok()); // Should still work programmatically
        assert_eq!(input.value(), "test");
    }
}
