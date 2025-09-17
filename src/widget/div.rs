//! Div widget implementation.

use crate::component::{BaseComponent, Component, ComponentId, Container};
use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use crate::widget::Widget;
use async_trait::async_trait;

/// A div widget for layout and grouping.
pub struct Div {
    base: BaseComponent,
    children: Vec<Box<dyn Component>>,
}

impl Div {
    /// Create a new div widget.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("Div"),
            children: Vec::new(),
        }
    }
}

impl Default for Div {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Component for Div {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "Div"
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let mut div_node = VirtualNode::element("div");
        
        for child in &self.children {
            let child_node = child.render(context).await?;
            div_node = div_node.child(child_node);
        }
        
        Ok(div_node)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Container for Div {
    fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }

    fn remove_child(&mut self, id: ComponentId) -> Option<Box<dyn Component>> {
        if let Some(index) = self.children.iter().position(|child| child.id() == id) {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    fn children(&self) -> &[Box<dyn Component>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.children
    }
}

#[async_trait]
impl Widget for Div {
    fn widget_type(&self) -> &'static str {
        "div"
    }
}
