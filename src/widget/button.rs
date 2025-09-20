//! Button widget implementation.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::event::types::{MouseButton, MouseEvent, MouseEventType};
use crate::render::{RenderContext, VirtualNode};
use crate::style::properties::Style;
use crate::widget::Widget;
use async_trait::async_trait;
use std::sync::Arc;

/// Button state for visual feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// Normal state
    Normal,
    /// Hovered state
    Hovered,
    /// Pressed state
    Pressed,
    /// Disabled state
    Disabled,
}

/// A button widget for user interaction.
pub struct Button {
    base: BaseComponent,
    label: String,
    enabled: bool,
    state: ButtonState,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    style: Style,
}

impl Button {
    /// Create a new button widget.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            base: BaseComponent::new("Button"),
            label: label.into(),
            enabled: true,
            state: ButtonState::Normal,
            on_click: None,
            style: Style::default(),
        }
    }

    /// Set the button label.
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    /// Get the button label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Set whether the button is enabled.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if the button is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set the click handler for the button.
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Arc::new(handler));
        self
    }

    /// Get the current button state.
    pub fn state(&self) -> ButtonState {
        self.state
    }

    /// Set the button state.
    pub fn set_state(&mut self, state: ButtonState) {
        self.state = if self.enabled {
            state
        } else {
            ButtonState::Disabled
        };
    }

    /// Set the button style.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get the button style.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Handle a mouse event on the button.
    pub fn handle_mouse_event(&mut self, event: &MouseEvent) -> bool {
        if !self.enabled {
            return false;
        }

        match event.button {
            MouseButton::Left => {
                match event.event_type {
                    MouseEventType::Press => {
                        self.set_state(ButtonState::Pressed);
                        true
                    }
                    MouseEventType::Release => {
                        if self.state == ButtonState::Pressed {
                            self.set_state(ButtonState::Hovered);
                            // Trigger click event
                            if let Some(ref handler) = self.on_click {
                                handler();
                            }
                            true
                        } else {
                            false
                        }
                    }
                    MouseEventType::Move => {
                        if self.state != ButtonState::Pressed {
                            self.set_state(ButtonState::Hovered);
                        }
                        true
                    }
                    MouseEventType::Enter => {
                        self.on_mouse_enter();
                        true
                    }
                    MouseEventType::Leave => {
                        self.on_mouse_leave();
                        true
                    }
                    MouseEventType::Scroll => {
                        // Buttons don't handle scroll events
                        false
                    }
                }
            }
            _ => false,
        }
    }

    /// Handle mouse enter event.
    pub fn on_mouse_enter(&mut self) {
        if self.enabled && self.state != ButtonState::Pressed {
            self.set_state(ButtonState::Hovered);
        }
    }

    /// Handle mouse leave event.
    pub fn on_mouse_leave(&mut self) {
        if self.enabled {
            self.set_state(ButtonState::Normal);
        }
    }
}

#[async_trait]
impl Component for Button {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "Button"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        // Determine button appearance based on state
        let state_class = match self.state {
            ButtonState::Normal => "button-normal",
            ButtonState::Hovered => "button-hovered",
            ButtonState::Pressed => "button-pressed",
            ButtonState::Disabled => "button-disabled",
        };

        let button_node = VirtualNode::element("button")
            .attr("enabled", self.enabled.to_string())
            .attr("state", format!("{:?}", self.state).to_lowercase())
            .attr("class", state_class)
            .child(VirtualNode::text(&self.label));

        Ok(button_node)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[async_trait]
impl Widget for Button {
    fn widget_type(&self) -> &'static str {
        "button"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::types::{MouseButton, MouseEventType};

    #[test]
    fn test_button_creation() {
        let button = Button::new("Click me");
        assert_eq!(button.label(), "Click me");
        assert!(button.is_enabled());
        assert_eq!(button.state(), ButtonState::Normal);
    }

    #[test]
    fn test_button_click_handler() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let clicked = Arc::new(AtomicBool::new(false));
        let clicked_clone = clicked.clone();

        let mut button = Button::new("Test").on_click(move || {
            clicked_clone.store(true, Ordering::SeqCst);
        });

        // Simulate mouse press and release
        let press_event = MouseEvent {
            button: MouseButton::Left,
            event_type: MouseEventType::Press,
            x: 0,
            y: 0,
            modifiers: crate::event::types::KeyModifiers::empty(),
            bubbles: true,
        };

        let release_event = MouseEvent {
            button: MouseButton::Left,
            event_type: MouseEventType::Release,
            x: 0,
            y: 0,
            modifiers: crate::event::types::KeyModifiers::empty(),
            bubbles: true,
        };

        button.handle_mouse_event(&press_event);
        assert_eq!(button.state(), ButtonState::Pressed);

        button.handle_mouse_event(&release_event);
        assert_eq!(button.state(), ButtonState::Hovered);
        assert!(clicked.load(Ordering::SeqCst));
    }

    #[test]
    fn test_button_hover_states() {
        let mut button = Button::new("Hover test");

        button.on_mouse_enter();
        assert_eq!(button.state(), ButtonState::Hovered);

        button.on_mouse_leave();
        assert_eq!(button.state(), ButtonState::Normal);
    }

    #[test]
    fn test_disabled_button() {
        let mut button = Button::new("Disabled");
        button.set_enabled(false);

        assert!(!button.is_enabled());

        // Disabled button should not respond to mouse events
        let press_event = MouseEvent {
            button: MouseButton::Left,
            event_type: MouseEventType::Press,
            x: 0,
            y: 0,
            modifiers: crate::event::types::KeyModifiers::empty(),
            bubbles: true,
        };

        let handled = button.handle_mouse_event(&press_event);
        assert!(!handled);
        assert_eq!(button.state(), ButtonState::Normal);
    }
}
