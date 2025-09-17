//! Hello World example for the TUI framework.
//!
//! This example demonstrates the basic usage of the framework
//! with a simple "Hello, World!" application.

use tui_framework::prelude::*;

/// A simple Hello World component.
struct HelloWorld {
    base: tui_framework::component::BaseComponent,
}

impl HelloWorld {
    fn new() -> Self {
        Self {
            base: tui_framework::component::BaseComponent::new("HelloWorld"),
        }
    }
}

#[async_trait]
impl Component for HelloWorld {
    fn id(&self) -> tui_framework::component::ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "HelloWorld"
    }

    async fn render(&self, _context: &RenderContext) -> tui_framework::error::Result<VirtualNode> {
        // For now, return an empty node since we haven't implemented the full rendering system
        Ok(VirtualNode::empty())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for debugging
    tracing_subscriber::fmt::init();

    println!("Starting Hello World TUI application...");
    println!("This is a basic example showing the framework structure.");
    println!("Press Ctrl+C to exit.");

    // Create the application
    let _app = App::new()
        .title("Hello World - TUI Framework")
        .component(HelloWorld::new());

    // Note: The full app.run() will be implemented when we have the rendering system
    // For now, we'll just demonstrate the structure
    println!("App created successfully!");
    println!("Framework structure is in place.");
    
    // Simulate a brief run
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    println!("Hello World example completed!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_world_component() {
        let component = HelloWorld::new();
        assert_eq!(component.name(), "HelloWorld");
    }

    #[tokio::test]
    async fn test_app_creation() {
        let app = App::new()
            .title("Test App")
            .component(HelloWorld::new());
        
        // Test that the app was created successfully
        // More comprehensive tests will be added as we implement the full system
    }
}
