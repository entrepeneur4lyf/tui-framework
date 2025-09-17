//! Test the libnotcurses backend implementation.

use tui_framework::prelude::*;
use tui_framework::component::{BaseComponent, Component, ComponentId};
use tui_framework::render::backend::{Backend, PlaceholderBackend};
use tui_framework::render::vdom::nodes::{div, text};
use tui_framework::render::context::RenderContext;
use tui_framework::layout::{Position, Rect};
use tui_framework::style::Theme;


#[cfg(feature = "notcurses")]
use tui_framework::render::backend::NotcursesBackend;

/// Simple component for testing the backend.
struct BackendTest {
    base: BaseComponent,
}

impl BackendTest {
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("BackendTest"),
        }
    }
}

#[async_trait::async_trait]
impl Component for BackendTest {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        Ok(div()
            .child(text("Hello from libnotcurses backend!"))
            .child(text("Press Ctrl+C to exit"))
            .child(text("This is a test of the rendering system")))
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

    println!("Testing TUI Framework Backend");
    println!("==============================");

    // Test placeholder backend first
    println!("\n1. Testing Placeholder Backend:");
    test_placeholder_backend().await?;

    // Test libnotcurses backend if available
    #[cfg(feature = "notcurses")]
    {
        println!("\n2. Testing Libnotcurses Backend:");
        test_notcurses_backend().await?;
    }

    #[cfg(not(feature = "notcurses"))]
    {
        println!("\n2. Libnotcurses backend not available (feature not enabled)");
    }

    println!("\nAll backend tests completed successfully!");
    Ok(())
}

async fn test_placeholder_backend() -> Result<()> {
    let mut backend = PlaceholderBackend::new();
    
    // Initialize
    backend.init()?;
    println!("  ✓ Placeholder backend initialized");
    
    // Get size
    let size = backend.size()?;
    println!("  ✓ Terminal size: {}x{}", size.width, size.height);
    
    // Create test component
    let component = BackendTest::new();
    let context = RenderContext::new(&Theme::default()).with_viewport_size(size);
    let vdom = component.render(&context).await?;
    
    // Test rendering
    let rect = Rect::new(Position::new(0, 0), size);
    backend.clear()?;
    backend.render_node(&vdom, rect)?;
    backend.present()?;
    println!("  ✓ Virtual DOM rendered successfully");
    
    // Test event polling
    let event = backend.poll_event()?;
    println!("  ✓ Event polling works: {:?}", event.is_some());
    
    // Cleanup
    backend.cleanup()?;
    println!("  ✓ Placeholder backend cleaned up");
    
    Ok(())
}

#[cfg(feature = "notcurses")]
async fn test_notcurses_backend() -> Result<()> {
    let mut backend = NotcursesBackend::new();
    
    // Initialize
    backend.init()?;
    println!("  ✓ Libnotcurses backend initialized");
    
    // Get size
    let size = backend.size()?;
    println!("  ✓ Terminal size: {}x{}", size.width, size.height);
    
    // Create test component
    let component = BackendTest::new();
    let context = RenderContext::new(&Theme::default()).with_viewport_size(size);
    let vdom = component.render(&context).await?;
    
    // Test rendering
    let rect = Rect::new(Position::new(0, 0), size);
    backend.clear()?;
    backend.render_node(&vdom, rect)?;
    backend.present()?;
    println!("  ✓ Virtual DOM rendered to terminal");
    
    // Wait for a key press to demonstrate input
    println!("  → Press any key to test input (or Ctrl+C to exit)...");
    let event = backend.wait_event()?;
    println!("  ✓ Received event: {:?}", event);
    
    // Cleanup
    backend.cleanup()?;
    println!("  ✓ Libnotcurses backend cleaned up");
    
    Ok(())
}
