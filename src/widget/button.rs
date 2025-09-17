//! Button widget implementation.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use crate::widget::Widget;
use async_trait::async_trait;

/// A button widget for user interaction.
pub struct Button {
    base: BaseComponent,
    label: String,
    enabled: bool,
}

impl Button {
    /// Create a new button widget.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            base: BaseComponent::new("Button"),
            label: label.into(),
            enabled: true,
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
        let button_node = VirtualNode::element("button")
            .attr("enabled", self.enabled.to_string())
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
