//! Text widget implementation.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use crate::widget::Widget;
use async_trait::async_trait;

/// A text widget for displaying text content.
pub struct Text {
    base: BaseComponent,
    content: String,
}

impl Text {
    /// Create a new text widget.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            base: BaseComponent::new("Text"),
            content: content.into(),
        }
    }

    /// Set the text content.
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    /// Get the text content.
    pub fn content(&self) -> &str {
        &self.content
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
        Ok(VirtualNode::text(&self.content))
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
