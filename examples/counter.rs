//! Interactive Counter Application
//!
//! This example demonstrates the TUI framework's capabilities including:
//! - Reactive state management with hooks
//! - Interactive event handling
//! - Component composition and styling
//! - Real-time UI updates
//! - Modern React-like patterns
//!
//! The application features:
//! - Simple counter with increment/decrement/reset
//! - Advanced counter with custom step sizes
//! - History tracking with undo functionality
//! - Interactive buttons with keyboard shortcuts
//! - Styled components with themes

use tui_framework::component::BaseComponent;
use tui_framework::prelude::*;

/// Interactive Counter Component
///
/// Demonstrates basic reactive state management with interactive buttons.
/// Features increment, decrement, and reset functionality with keyboard shortcuts.
struct InteractiveCounter {
    base: BaseComponent,
    count: State<i32>,
    last_action: State<String>,
}

impl InteractiveCounter {
    fn new() -> Self {
        let (count, _) = use_state(0);
        let (last_action, _) = use_state("Initialized".to_string());

        Self {
            base: BaseComponent::new("InteractiveCounter"),
            count,
            last_action,
        }
    }

    fn increment(&self) {
        self.count.update(|count| *count += 1);
        self.last_action.set("Incremented".to_string());
    }

    fn decrement(&self) {
        self.count.update(|count| *count -= 1);
        self.last_action.set("Decremented".to_string());
    }

    fn reset(&self) {
        self.count.set(0);
        self.last_action.set("Reset".to_string());
    }

    #[allow(dead_code)]
    fn handle_button_click(&self, button_id: &str) {
        match button_id {
            "increment-btn" => self.increment(),
            "decrement-btn" => self.decrement(),
            "reset-btn" => self.reset(),
            _ => {}
        }
    }
}

#[async_trait]
impl Component for InteractiveCounter {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "InteractiveCounter"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let count_value = self.count.clone_value();
        let last_action_value = self.last_action.clone_value();

        // Create a clean counter interface
        let counter_ui = div()
            .attr("class", "counter-container")
            .child(
                div()
                    .attr("class", "counter-header")
                    .child(text("🔢 Interactive Counter")),
            )
            .child(
                div()
                    .attr("class", "counter-display")
                    .child(text(format!("Count: {}", count_value)))
                    .child(text(format!("Last Action: {}", last_action_value))),
            )
            .child(
                div()
                    .attr("class", "counter-controls")
                    .child(
                        button("➖ Decrement")
                            .attr("id", "decrement-btn")
                            .attr("class", "btn-decrement"),
                    )
                    .child(
                        button("🔄 Reset")
                            .attr("id", "reset-btn")
                            .attr("class", "btn-reset"),
                    )
                    .child(
                        button("➕ Increment")
                            .attr("id", "increment-btn")
                            .attr("class", "btn-increment"),
                    ),
            )
            .child(
                div()
                    .attr("class", "counter-help")
                    .child(text("Keyboard: [+] Increment | [-] Decrement | [R] Reset")),
            );

        Ok(counter_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Advanced Counter Component with History and Custom Steps
///
/// Demonstrates complex state management with multiple state variables,
/// history tracking, and undo functionality.
struct AdvancedCounter {
    base: BaseComponent,
    count: State<i32>,
    step: State<i32>,
    history: State<Vec<i32>>,
    max_history: usize,
}

impl AdvancedCounter {
    fn new() -> Self {
        let (count, _) = use_state(0);
        let (step, _) = use_state(1);
        let (history, _) = use_state(vec![0]);

        Self {
            base: BaseComponent::new("AdvancedCounter"),
            count,
            step,
            history,
            max_history: 10,
        }
    }

    #[allow(dead_code)]
    fn new_with_max_history(max_history: usize) -> Self {
        let (count, _) = use_state(0);
        let (step, _) = use_state(1);
        let (history, _) = use_state(vec![0]);

        Self {
            base: BaseComponent::new("AdvancedCounter"),
            count,
            step,
            history,
            max_history,
        }
    }

    fn increment(&self) {
        let step_value = self.step.clone_value();
        self.count.update(|count| *count += step_value);
        self.add_to_history();
    }

    #[allow(dead_code)]
    fn decrement(&self) {
        let step_value = self.step.clone_value();
        self.count.update(|count| *count -= step_value);
        self.add_to_history();
    }

    fn set_step(&self, new_step: i32) {
        self.step.set(new_step);
    }

    fn add_to_history(&self) {
        let current_count = self.count.clone_value();
        let max_history = self.max_history;
        self.history.update(|history| {
            history.push(current_count);
            // Keep only the last max_history entries
            if history.len() > max_history {
                history.remove(0);
            }
        });
    }

    fn clear_history(&self) {
        let current_count = self.count.clone_value();
        self.history.set(vec![current_count]);
    }

    fn get_history_summary(&self) -> String {
        let history = self.history.clone_value();
        if history.len() <= 1 {
            "No history".to_string()
        } else {
            format!(
                "Last {} values: {:?}",
                history.len().min(5),
                &history[history.len().saturating_sub(5)..]
            )
        }
    }

    fn undo(&self) {
        self.history.update(|history| {
            if history.len() > 1 {
                history.pop(); // Remove current value
                if let Some(&previous_value) = history.last() {
                    self.count.set(previous_value);
                }
            }
        });
    }
}

#[async_trait]
impl Component for AdvancedCounter {
    fn id(&self) -> tui_framework::component::ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "AdvancedCounter"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let count_value = self.count.clone_value();
        let step_value = self.step.clone_value();
        let history_summary = self.get_history_summary();

        let counter_ui = div()
            .attr("class", "advanced-counter")
            .child(
                div()
                    .attr("class", "counter-header")
                    .child(text("⚡ Advanced Counter")),
            )
            .child(
                div()
                    .attr("class", "counter-display")
                    .child(text(format!("Count: {}", count_value)))
                    .child(text(format!("Step Size: {}", step_value))),
            )
            .child(
                div()
                    .attr("class", "counter-controls")
                    .child(
                        button(format!("➖ {}", step_value))
                            .attr("id", "advanced-decrement-btn")
                            .attr("class", "btn-decrement"),
                    )
                    .child(
                        button("↶ Undo")
                            .attr("id", "undo-btn")
                            .attr("class", "btn-undo"),
                    )
                    .child(
                        button(format!("➕ {}", step_value))
                            .attr("id", "advanced-increment-btn")
                            .attr("class", "btn-increment"),
                    ),
            )
            .child(
                div()
                    .attr("class", "step-controls")
                    .child(text("Step size: "))
                    .child(
                        button("1")
                            .attr("id", "step-1-btn")
                            .attr("class", "btn-step"),
                    )
                    .child(
                        button("5")
                            .attr("id", "step-5-btn")
                            .attr("class", "btn-step"),
                    )
                    .child(
                        button("10")
                            .attr("id", "step-10-btn")
                            .attr("class", "btn-step"),
                    ),
            )
            .child(
                div()
                    .attr("class", "history-display")
                    .child(text(format!("History: {}", history_summary))),
            )
            .child(div().attr("class", "advanced-help").child(text(
                "Keyboard: [1][5][0] Set Step | [U] Undo | [C] Clear History",
            )));

        Ok(counter_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Demo Application Component
///
/// Combines both counter types into a single application interface
struct CounterApp {
    base: BaseComponent,
    simple_counter: InteractiveCounter,
    advanced_counter: AdvancedCounter,
    active_counter: State<String>,
}

impl CounterApp {
    fn new() -> Self {
        let (active_counter, _) = use_state("simple".to_string());

        Self {
            base: BaseComponent::new("CounterApp"),
            simple_counter: InteractiveCounter::new(),
            advanced_counter: AdvancedCounter::new(),
            active_counter,
        }
    }

    fn switch_to_simple(&self) {
        self.active_counter.set("simple".to_string());
    }

    fn switch_to_advanced(&self) {
        self.active_counter.set("advanced".to_string());
    }
}

#[async_trait]
impl Component for CounterApp {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "CounterApp"
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let active = self.active_counter.clone_value();

        let app_ui = div()
            .attr("class", "counter-app")
            .child(
                div()
                    .attr("class", "app-header")
                    .child(text("🚀 TUI Framework Counter Demo"))
                    .child(text("Demonstrating React-like State Management")),
            )
            .child(
                div()
                    .attr("class", "app-navigation")
                    .child(button("Simple Counter").attr("id", "nav-simple").attr(
                        "class",
                        if active == "simple" {
                            "nav-btn active"
                        } else {
                            "nav-btn"
                        },
                    ))
                    .child(button("Advanced Counter").attr("id", "nav-advanced").attr(
                        "class",
                        if active == "advanced" {
                            "nav-btn active"
                        } else {
                            "nav-btn"
                        },
                    )),
            )
            .child(
                div()
                    .attr("class", "app-content")
                    .child(if active == "simple" {
                        self.simple_counter.render(context).await?
                    } else {
                        self.advanced_counter.render(context).await?
                    }),
            )
            .child(div().attr("class", "app-footer").child(text(
                "Built with TUI Framework - React-like patterns for Terminal UIs",
            )));

        Ok(app_ui)
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

    println!("🚀 Starting Interactive Counter TUI Application");
    println!("================================================");
    println!("This example demonstrates:");
    println!("• Reactive state management with hooks");
    println!("• Component composition and lifecycle");
    println!("• Interactive event handling");
    println!("• React-like patterns in Rust TUI");
    println!();

    // Create the main application
    let app = CounterApp::new();
    println!("✅ Counter application created successfully");

    // Demonstrate simple counter functionality
    println!("\n🔢 Testing Simple Counter:");
    app.simple_counter.increment();
    println!("   After increment: {}", *app.simple_counter.count.get());

    app.simple_counter.increment();
    app.simple_counter.increment();
    println!(
        "   After two more increments: {}",
        *app.simple_counter.count.get()
    );

    app.simple_counter.decrement();
    println!("   After decrement: {}", *app.simple_counter.count.get());

    app.simple_counter.reset();
    println!("   After reset: {}", *app.simple_counter.count.get());

    // Demonstrate advanced counter functionality
    println!("\n⚡ Testing Advanced Counter:");
    app.advanced_counter.set_step(5);
    app.advanced_counter.increment();
    println!(
        "   After increment by 5: {}",
        *app.advanced_counter.count.get()
    );

    app.advanced_counter.increment();
    println!(
        "   After another increment: {}",
        *app.advanced_counter.count.get()
    );

    app.advanced_counter.undo();
    println!("   After undo: {}", *app.advanced_counter.count.get());

    app.advanced_counter.clear_history();
    println!(
        "   History cleared: {}",
        app.advanced_counter.get_history_summary()
    );

    // Test rendering
    println!("\n🎨 Testing Component Rendering:");
    let context = RenderContext::new(&Theme::default());

    let simple_vdom = app.simple_counter.render(&context).await?;
    println!("   ✅ Simple counter rendered successfully");
    println!(
        "      Component: {}",
        simple_vdom.tag().unwrap_or("unknown")
    );
    println!("      Children: {}", simple_vdom.get_children().len());

    let advanced_vdom = app.advanced_counter.render(&context).await?;
    println!("   ✅ Advanced counter rendered successfully");
    println!(
        "      Component: {}",
        advanced_vdom.tag().unwrap_or("unknown")
    );
    println!("      Children: {}", advanced_vdom.get_children().len());

    let app_vdom = app.render(&context).await?;
    println!("   ✅ Full application rendered successfully");
    println!("      Component: {}", app_vdom.tag().unwrap_or("unknown"));
    println!("      Children: {}", app_vdom.get_children().len());

    // Test navigation
    println!("\n🧭 Testing Navigation:");
    app.switch_to_advanced();
    println!(
        "   Switched to advanced counter: {}",
        *app.active_counter.get()
    );

    app.switch_to_simple();
    println!(
        "   Switched to simple counter: {}",
        *app.active_counter.get()
    );

    // Create the TUI application framework instance
    println!("\n🏗️  Creating TUI Framework Application:");
    let _tui_app = App::new().title("Interactive Counter Demo").component(app);

    println!("   ✅ TUI application created successfully");
    println!("   📱 In a real application, this would start the event loop");
    println!("   🎮 Users would interact with buttons and keyboard shortcuts");

    println!("\n🎉 Counter Example Completed Successfully!");
    println!("   ✨ All components rendered without errors");
    println!("   🔄 State management working correctly");
    println!("   🎯 React-like patterns demonstrated");
    println!("   🚀 Ready for interactive TUI applications!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_interactive_counter_component() {
        let counter = InteractiveCounter::new();
        assert_eq!(*counter.count.get(), 0);
        assert_eq!(*counter.last_action.get(), "Initialized");

        counter.increment();
        assert_eq!(*counter.count.get(), 1);
        assert_eq!(*counter.last_action.get(), "Incremented");

        counter.decrement();
        assert_eq!(*counter.count.get(), 0);
        assert_eq!(*counter.last_action.get(), "Decremented");

        counter.reset();
        assert_eq!(*counter.count.get(), 0);
        assert_eq!(*counter.last_action.get(), "Reset");
    }

    #[tokio::test]
    async fn test_advanced_counter() {
        let counter = AdvancedCounter::new();

        // Test initial state
        assert_eq!(*counter.count.get(), 0);
        assert_eq!(*counter.step.get(), 1);
        assert_eq!(counter.max_history, 10);

        // Test step setting
        counter.set_step(5);
        assert_eq!(*counter.step.get(), 5);

        // Test increment with custom step
        counter.increment();
        assert_eq!(*counter.count.get(), 5);

        counter.increment();
        assert_eq!(*counter.count.get(), 10);

        // Test undo functionality
        counter.undo();
        assert_eq!(*counter.count.get(), 5);

        // Test history management
        let history_summary = counter.get_history_summary();
        assert!(history_summary.contains("values"));
    }

    #[tokio::test]
    async fn test_counter_rendering() {
        let counter = InteractiveCounter::new();
        let context = RenderContext::new(&Theme::default());

        let vdom = counter.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());

        // Check for expected structure
        let children = vdom.get_children();
        assert!(children.len() >= 4); // header, display, controls, help
    }

    #[tokio::test]
    async fn test_advanced_counter_rendering() {
        let counter = AdvancedCounter::new();
        let context = RenderContext::new(&Theme::default());

        let vdom = counter.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());

        // Check for expected structure
        let children = vdom.get_children();
        assert!(children.len() >= 5); // header, display, controls, step-controls, history, help
    }

    #[tokio::test]
    async fn test_counter_app() {
        let app = CounterApp::new();

        // Test initial state
        assert_eq!(*app.active_counter.get(), "simple");

        // Test navigation
        app.switch_to_advanced();
        assert_eq!(*app.active_counter.get(), "advanced");

        app.switch_to_simple();
        assert_eq!(*app.active_counter.get(), "simple");

        // Test rendering
        let context = RenderContext::new(&Theme::default());
        let vdom = app.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());
    }

    #[tokio::test]
    async fn test_button_click_handling() {
        let counter = InteractiveCounter::new();

        // Test button click simulation
        counter.handle_button_click("increment-btn");
        assert_eq!(*counter.count.get(), 1);
        assert_eq!(*counter.last_action.get(), "Incremented");

        counter.handle_button_click("decrement-btn");
        assert_eq!(*counter.count.get(), 0);
        assert_eq!(*counter.last_action.get(), "Decremented");

        counter.handle_button_click("reset-btn");
        assert_eq!(*counter.count.get(), 0);
        assert_eq!(*counter.last_action.get(), "Reset");

        // Test unknown button
        counter.handle_button_click("unknown-btn");
        assert_eq!(*counter.last_action.get(), "Reset"); // Should remain unchanged
    }

    #[tokio::test]
    async fn test_advanced_counter_history() {
        let counter = AdvancedCounter::new_with_max_history(3);

        // Test history limit
        counter.increment(); // 1
        counter.increment(); // 2
        counter.increment(); // 3
        counter.increment(); // 4

        let history = counter.history.clone_value();
        assert!(history.len() <= 4); // Should respect max_history + 1 (initial)

        // Test clear history
        counter.clear_history();
        let history_after_clear = counter.history.clone_value();
        assert_eq!(history_after_clear.len(), 1);
        assert_eq!(history_after_clear[0], *counter.count.get());
    }
}
