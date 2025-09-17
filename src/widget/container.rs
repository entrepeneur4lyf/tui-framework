//! Container widget implementation.

use crate::component::{BaseComponent, Component, ComponentId, Container as ContainerTrait};
use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use crate::widget::Widget;
use async_trait::async_trait;

/// A container widget for grouping other widgets.
pub struct Container {
    base: BaseComponent,
    children: Vec<Box<dyn Component>>,
}

impl Container {
    /// Create a new container widget.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("Container"),
            children: Vec::new(),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Component for Container {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "Container"
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let mut container_node = VirtualNode::element("container");
        
        for child in &self.children {
            let child_node = child.render(context).await?;
            container_node = container_node.child(child_node);
        }
        
        Ok(container_node)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl ContainerTrait for Container {
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
impl Widget for Container {
    fn widget_type(&self) -> &'static str {
        "container"
    }
}
