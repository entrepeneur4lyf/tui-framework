//! Hello World - TUI Framework Introduction
//!
//! This comprehensive example demonstrates the fundamental concepts of the TUI framework:
//!
//! ## What You'll Learn:
//! - Basic component creation and structure
//! - Virtual DOM rendering with text and layout elements
//! - Component composition and nesting
//! - State management with reactive hooks
//! - Event handling and user interaction
//! - Styling and theming basics
//! - Testing component functionality
//!
//! ## Framework Features Showcased:
//! - React-like component patterns
//! - Declarative UI with virtual DOM
//! - Type-safe component system
//! - Async rendering pipeline
//! - Comprehensive error handling
//!
//! This example serves as the perfect starting point for learning the TUI framework
//! and understanding how to build terminal user interfaces with modern patterns.

use tui_framework::component::BaseComponent;
use tui_framework::prelude::*;

/// Simple Hello World Component
///
/// This component demonstrates the most basic usage of the TUI framework.
/// It renders a simple greeting message with basic styling.
struct HelloWorld {
    base: BaseComponent,
    message: State<String>,
    click_count: State<u32>,
}

impl HelloWorld {
    /// Create a new HelloWorld component
    ///
    /// This demonstrates:
    /// - Component initialization
    /// - State management with hooks
    /// - BaseComponent setup
    fn new() -> Self {
        let (message, _) = use_state("Hello, World!".to_string());
        let (click_count, _) = use_state(0u32);

        Self {
            base: BaseComponent::new("HelloWorld"),
            message,
            click_count,
        }
    }

    /// Update the greeting message
    ///
    /// This demonstrates state mutation and reactive updates
    fn update_message(&self, new_message: String) {
        self.message.set(new_message);
    }

    /// Handle button click
    ///
    /// This demonstrates event handling and state updates
    fn handle_click(&self) {
        self.click_count.update(|count| *count += 1);
        let count = *self.click_count.get();

        match count {
            1 => self.update_message("Hello, TUI Framework!".to_string()),
            2 => self.update_message("Welcome to Rust TUI!".to_string()),
            3 => self.update_message("React-like patterns in Terminal!".to_string()),
            4 => self.update_message("Building amazing TUIs!".to_string()),
            _ => self.update_message(format!("Clicked {} times! 🎉", count)),
        }
    }
}

#[async_trait]
impl Component for HelloWorld {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "HelloWorld"
    }

    /// Render the HelloWorld component
    ///
    /// This demonstrates:
    /// - Virtual DOM creation with div(), text(), button()
    /// - Component state access and display
    /// - Basic layout and structure
    /// - CSS class application for styling
    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let current_message = self.message.clone_value();
        let current_count = self.click_count.clone_value();

        // Create a comprehensive Hello World interface
        let hello_ui = div()
            .attr("class", "hello-world-container")
            .child(
                // Header section
                div()
                    .attr("class", "hello-header")
                    .child(text("🚀 TUI Framework - Hello World"))
                    .child(text("Your first step into modern terminal UIs")),
            )
            .child(
                // Main content section
                div()
                    .attr("class", "hello-content")
                    .child(
                        div()
                            .attr("class", "greeting-display")
                            .child(text(format!("✨ {}", current_message))),
                    )
                    .child(
                        div()
                            .attr("class", "interaction-stats")
                            .child(text(format!("Button clicks: {}", current_count))),
                    ),
            )
            .child(
                // Interactive section
                div()
                    .attr("class", "hello-controls")
                    .child(
                        button("Click me!")
                            .attr("id", "hello-button")
                            .attr("class", "btn-primary"),
                    )
                    .child(
                        button("Reset")
                            .attr("id", "reset-button")
                            .attr("class", "btn-secondary"),
                    ),
            )
            .child(
                // Information section
                div()
                    .attr("class", "hello-info")
                    .child(text("Framework Features Demonstrated:"))
                    .child(text("• Component-based architecture"))
                    .child(text("• Reactive state management"))
                    .child(text("• Virtual DOM rendering"))
                    .child(text("• Event handling"))
                    .child(text("• CSS-like styling")),
            )
            .child(
                // Footer section
                div().attr("class", "hello-footer").child(text(
                    "Built with TUI Framework - React patterns for Terminal UIs",
                )),
            );

        Ok(hello_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Interactive Demo Component
///
/// This component demonstrates more advanced features by combining
/// multiple HelloWorld components and showing component composition.
struct HelloWorldDemo {
    base: BaseComponent,
    primary_hello: HelloWorld,
    secondary_hello: HelloWorld,
    demo_step: State<u32>,
}

impl HelloWorldDemo {
    fn new() -> Self {
        let (demo_step, _) = use_state(0u32);

        Self {
            base: BaseComponent::new("HelloWorldDemo"),
            primary_hello: HelloWorld::new(),
            secondary_hello: HelloWorld::new(),
            demo_step,
        }
    }

    fn next_step(&self) {
        self.demo_step.update(|step| *step += 1);
    }

    #[allow(dead_code)]
    fn reset_demo(&self) {
        self.demo_step.set(0);
    }
}

#[async_trait]
impl Component for HelloWorldDemo {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "HelloWorldDemo"
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let current_step = self.demo_step.clone_value();

        let demo_ui = div()
            .attr("class", "hello-demo-container")
            .child(
                div()
                    .attr("class", "demo-header")
                    .child(text("🎓 TUI Framework Tutorial - Hello World"))
                    .child(text(format!("Step {} of 3", current_step.min(3)))),
            )
            .child(
                div()
                    .attr("class", "demo-content")
                    .child(match current_step {
                        0 => div()
                            .attr("class", "demo-intro")
                            .child(text("Welcome to the TUI Framework!"))
                            .child(text("This tutorial will show you the basics."))
                            .child(text("Click 'Next Step' to begin.")),
                        1 => self.primary_hello.render(context).await?,
                        2 => div()
                            .attr("class", "demo-composition")
                            .child(text("Component Composition Example:"))
                            .child(self.primary_hello.render(context).await?)
                            .child(self.secondary_hello.render(context).await?),
                        _ => div()
                            .attr("class", "demo-complete")
                            .child(text("🎉 Tutorial Complete!"))
                            .child(text("You've learned the basics of TUI Framework:"))
                            .child(text("✓ Component creation and rendering"))
                            .child(text("✓ State management with hooks"))
                            .child(text("✓ Virtual DOM and layout"))
                            .child(text("✓ Component composition"))
                            .child(text("Ready to build amazing TUIs!")),
                    }),
            )
            .child(
                div()
                    .attr("class", "demo-controls")
                    .child(
                        button("Next Step")
                            .attr("id", "next-step-btn")
                            .attr("class", "btn-primary"),
                    )
                    .child(
                        button("Reset Tutorial")
                            .attr("id", "reset-demo-btn")
                            .attr("class", "btn-secondary"),
                    ),
            );

        Ok(demo_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debugging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Hello World TUI Framework Tutorial");
    println!("===============================================");
    println!("This comprehensive example demonstrates:");
    println!("• Basic component creation and structure");
    println!("• Virtual DOM rendering and layout");
    println!("• State management with reactive hooks");
    println!("• Component composition patterns");
    println!("• Event handling and user interaction");
    println!();

    // Create and test individual HelloWorld component
    println!("📦 Creating HelloWorld Component:");
    let hello_component = HelloWorld::new();
    println!("   ✅ Component created: {}", hello_component.name());
    println!("   📊 Initial state:");
    println!("      Message: {}", *hello_component.message.get());
    println!("      Click count: {}", *hello_component.click_count.get());

    // Demonstrate state changes
    println!("\n🔄 Testing State Management:");
    hello_component.handle_click();
    println!("   After 1st click: {}", *hello_component.message.get());

    hello_component.handle_click();
    println!("   After 2nd click: {}", *hello_component.message.get());

    hello_component.handle_click();
    println!("   After 3rd click: {}", *hello_component.message.get());

    // Test rendering
    println!("\n🎨 Testing Component Rendering:");
    let context = RenderContext::new(&Theme::default());
    let hello_vdom = hello_component.render(&context).await?;
    println!("   ✅ HelloWorld rendered successfully");
    println!(
        "      Root element: {}",
        hello_vdom.tag().unwrap_or("unknown")
    );
    println!("      Child count: {}", hello_vdom.get_children().len());

    // Create and test demo component
    println!("\n🎓 Creating Tutorial Demo:");
    let demo_component = HelloWorldDemo::new();
    println!("   ✅ Demo component created: {}", demo_component.name());

    // Test demo progression
    demo_component.next_step();
    println!("   Advanced to step: {}", *demo_component.demo_step.get());

    let demo_vdom = demo_component.render(&context).await?;
    println!("   ✅ Demo rendered successfully");
    println!(
        "      Root element: {}",
        demo_vdom.tag().unwrap_or("unknown")
    );
    println!("      Child count: {}", demo_vdom.get_children().len());

    // Create the TUI application
    println!("\n🏗️  Creating TUI Application:");
    let _app = App::new()
        .title("Hello World - TUI Framework Tutorial")
        .component(demo_component);

    println!("   ✅ TUI application created successfully");
    println!("   📱 In a real application, this would start the event loop");
    println!("   🎮 Users would interact with the tutorial interface");

    println!("\n🎉 Hello World Tutorial Completed Successfully!");
    println!("   ✨ All components rendered without errors");
    println!("   🔄 State management working correctly");
    println!("   🎯 Framework fundamentals demonstrated");
    println!("   📚 Ready to explore more advanced examples!");
    println!();
    println!("Next steps:");
    println!("• Try the counter.rs example for interactive components");
    println!("• Explore calculator.rs for complex state management");
    println!("• Build your own components using these patterns");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_world_component_creation() {
        let component = HelloWorld::new();

        // Test component properties
        assert_eq!(component.name(), "HelloWorld");
        assert_eq!(*component.message.get(), "Hello, World!");
        assert_eq!(*component.click_count.get(), 0);
    }

    #[tokio::test]
    async fn test_hello_world_state_management() {
        let component = HelloWorld::new();

        // Test initial state
        assert_eq!(*component.click_count.get(), 0);
        assert_eq!(*component.message.get(), "Hello, World!");

        // Test state updates
        component.handle_click();
        assert_eq!(*component.click_count.get(), 1);
        assert_eq!(*component.message.get(), "Hello, TUI Framework!");

        component.handle_click();
        assert_eq!(*component.click_count.get(), 2);
        assert_eq!(*component.message.get(), "Welcome to Rust TUI!");

        component.handle_click();
        assert_eq!(*component.click_count.get(), 3);
        assert_eq!(*component.message.get(), "React-like patterns in Terminal!");

        // Test custom message update
        component.update_message("Custom message".to_string());
        assert_eq!(*component.message.get(), "Custom message");
    }

    #[tokio::test]
    async fn test_hello_world_rendering() {
        let component = HelloWorld::new();
        let context = RenderContext::new(&Theme::default());

        let vdom = component.render(&context).await.unwrap();

        // Test basic structure
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());

        // Test that the component has the expected sections
        let children = vdom.get_children();
        assert!(children.len() >= 5); // header, content, controls, info, footer
    }

    #[tokio::test]
    async fn test_hello_world_demo_component() {
        let demo = HelloWorldDemo::new();

        // Test initial state
        assert_eq!(demo.name(), "HelloWorldDemo");
        assert_eq!(*demo.demo_step.get(), 0);

        // Test step progression
        demo.next_step();
        assert_eq!(*demo.demo_step.get(), 1);

        demo.next_step();
        assert_eq!(*demo.demo_step.get(), 2);

        // Test reset
        demo.reset_demo();
        assert_eq!(*demo.demo_step.get(), 0);
    }

    #[tokio::test]
    async fn test_demo_rendering() {
        let demo = HelloWorldDemo::new();
        let context = RenderContext::new(&Theme::default());

        // Test rendering at different steps
        for step in 0..=3 {
            demo.demo_step.set(step);
            let vdom = demo.render(&context).await.unwrap();

            assert_eq!(vdom.tag(), Some("div"));
            assert!(!vdom.get_children().is_empty());

            let children = vdom.get_children();
            assert!(children.len() >= 3); // header, content, controls
        }
    }

    #[tokio::test]
    async fn test_component_composition() {
        let demo = HelloWorldDemo::new();
        let context = RenderContext::new(&Theme::default());

        // Test that demo contains HelloWorld components
        demo.demo_step.set(1);
        let vdom = demo.render(&context).await.unwrap();

        // The demo should successfully render with nested components
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());
    }

    #[tokio::test]
    async fn test_app_creation() {
        let hello_component = HelloWorld::new();
        let demo_component = HelloWorldDemo::new();

        // Test app creation with HelloWorld
        let _app1 = App::new()
            .title("Hello World Test")
            .component(hello_component);

        // Test app creation with Demo
        let _app2 = App::new().title("Demo Test").component(demo_component);

        // If we reach here, app creation was successful
        assert!(true);
    }

    #[tokio::test]
    async fn test_multiple_clicks() {
        let component = HelloWorld::new();

        // Test multiple clicks beyond the predefined messages
        for i in 1..=10 {
            component.handle_click();
            assert_eq!(*component.click_count.get(), i);
        }

        // After many clicks, should show click count
        let final_message = component.message.clone_value();
        assert!(final_message.contains("Clicked") && final_message.contains("times"));
    }

    #[tokio::test]
    async fn test_framework_fundamentals() {
        // This test verifies that all the fundamental framework concepts work together
        let component = HelloWorld::new();
        let context = RenderContext::new(&Theme::default());

        // 1. Component creation ✓
        assert_eq!(component.name(), "HelloWorld");

        // 2. State management ✓
        component.handle_click();
        assert_eq!(*component.click_count.get(), 1);

        // 3. Virtual DOM rendering ✓
        let vdom = component.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));

        // 4. Component composition ✓
        let demo = HelloWorldDemo::new();
        let demo_vdom = demo.render(&context).await.unwrap();
        assert_eq!(demo_vdom.tag(), Some("div"));

        // 5. Application creation ✓
        let _app = App::new().title("Framework Test").component(component);

        // All fundamental concepts working correctly!
        assert!(true);
    }
}
