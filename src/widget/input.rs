//! Input widget implementation.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use crate::widget::Widget;
use async_trait::async_trait;

/// An input widget for text entry.
pub struct Input {
    base: BaseComponent,
    value: String,
    placeholder: String,
    enabled: bool,
}

impl Input {
    /// Create a new input widget.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("Input"),
            value: String::new(),
            placeholder: String::new(),
            enabled: true,
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
        let input_node = VirtualNode::element("input")
            .attr("value", &self.value)
            .attr("placeholder", &self.placeholder)
            .attr("enabled", self.enabled.to_string());
        
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
