//! Counter example demonstrating reactive state management.
//!
//! This example shows how to use the reactive state system
//! to create a simple counter application.

use tui_framework::prelude::*;
use tui_framework::render::vdom::nodes::*;

/// A counter component that demonstrates state management.
struct Counter {
    base: tui_framework::component::BaseComponent,
    count: State<i32>,
}

impl Counter {
    fn new() -> Self {
        let (count, _) = use_state(0);
        
        Self {
            base: tui_framework::component::BaseComponent::new("Counter"),
            count,
        }
    }

    fn increment(&self) {
        self.count.update(|count| *count += 1);
    }

    fn decrement(&self) {
        self.count.update(|count| *count -= 1);
    }

    fn reset(&self) {
        self.count.set(0);
    }
}

#[async_trait]
impl Component for Counter {
    fn id(&self) -> tui_framework::component::ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "Counter"
    }

    async fn render(&self, _context: &RenderContext) -> tui_framework::error::Result<VirtualNode> {
        let count_value = self.count.clone_value();
        
        // Create a virtual DOM structure for the counter
        let counter_ui = div()
            .attr("class", "counter-container")
            .child(
                div()
                    .attr("class", "counter-display")
                    .child(text(&format!("Count: {}", count_value)))
            )
            .child(
                div()
                    .attr("class", "counter-controls")
                    .child(
                        button("Decrement")
                            .attr("id", "decrement-btn")
                    )
                    .child(
                        button("Reset")
                            .attr("id", "reset-btn")
                    )
                    .child(
                        button("Increment")
                            .attr("id", "increment-btn")
                    )
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

/// A more complex counter with additional features.
struct AdvancedCounter {
    base: tui_framework::component::BaseComponent,
    count: State<i32>,
    step: State<i32>,
    history: State<Vec<i32>>,
}

impl AdvancedCounter {
    fn new() -> Self {
        let (count, _) = use_state(0);
        let (step, _) = use_state(1);
        let (history, _) = use_state(vec![0]);
        
        Self {
            base: tui_framework::component::BaseComponent::new("AdvancedCounter"),
            count,
            step,
            history,
        }
    }

    fn increment(&self) {
        let step_value = self.step.clone_value();
        self.count.update(|count| *count += step_value);
        self.add_to_history();
    }

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
        self.history.update(|history| {
            history.push(current_count);
            // Keep only the last 10 entries
            if history.len() > 10 {
                history.remove(0);
            }
        });
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

    async fn render(&self, _context: &RenderContext) -> tui_framework::error::Result<VirtualNode> {
        let count_value = self.count.clone_value();
        let step_value = self.step.clone_value();
        let history_value = self.history.clone_value();
        
        let counter_ui = div()
            .attr("class", "advanced-counter")
            .child(
                div()
                    .attr("class", "counter-header")
                    .child(text("Advanced Counter"))
            )
            .child(
                div()
                    .attr("class", "counter-display")
                    .child(text(&format!("Count: {}", count_value)))
                    .child(text(&format!("Step: {}", step_value)))
            )
            .child(
                div()
                    .attr("class", "counter-controls")
                    .child(button(&format!("- {}", step_value)))
                    .child(button("Undo"))
                    .child(button(&format!("+ {}", step_value)))
            )
            .child(
                div()
                    .attr("class", "step-controls")
                    .child(text("Step size:"))
                    .child(button("1"))
                    .child(button("5"))
                    .child(button("10"))
            )
            .child(
                div()
                    .attr("class", "history")
                    .child(text("History:"))
                    .child(text(&format!("{:?}", history_value)))
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

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debugging
    tracing_subscriber::fmt::init();

    println!("Starting Counter TUI application...");
    println!("This example demonstrates reactive state management.");

    // Create a simple counter
    let simple_counter = Counter::new();
    println!("Simple counter created with initial count: {}", *simple_counter.count.get());

    // Demonstrate state changes
    simple_counter.increment();
    println!("After increment: {}", *simple_counter.count.get());

    simple_counter.increment();
    simple_counter.increment();
    println!("After two more increments: {}", *simple_counter.count.get());

    simple_counter.decrement();
    println!("After decrement: {}", *simple_counter.count.get());

    simple_counter.reset();
    println!("After reset: {}", *simple_counter.count.get());

    // Create an advanced counter
    let advanced_counter = AdvancedCounter::new();
    println!("\nAdvanced counter created");

    // Demonstrate advanced features
    advanced_counter.set_step(5);
    advanced_counter.increment();
    println!("Advanced counter after increment by 5: {}", *advanced_counter.count.get());

    advanced_counter.increment();
    println!("After another increment: {}", *advanced_counter.count.get());

    advanced_counter.undo();
    println!("After undo: {}", *advanced_counter.count.get());

    // Test rendering
    let context = RenderContext::new(&Theme::default());
    let _simple_vdom = simple_counter.render(&context).await?;
    let _advanced_vdom = advanced_counter.render(&context).await?;

    println!("\nSimple counter rendered successfully");
    println!("Advanced counter rendered successfully");

    // Create the application (this would normally run the event loop)
    let _app = App::new()
        .title("Counter App")
        .component(simple_counter);

    println!("\nCounter example completed successfully!");
    println!("Framework reactive system is working correctly.");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_counter_component() {
        let counter = Counter::new();
        assert_eq!(*counter.count.get(), 0);

        counter.increment();
        assert_eq!(*counter.count.get(), 1);

        counter.decrement();
        assert_eq!(*counter.count.get(), 0);

        counter.reset();
        assert_eq!(*counter.count.get(), 0);
    }

    #[tokio::test]
    async fn test_advanced_counter() {
        let counter = AdvancedCounter::new();
        
        counter.set_step(5);
        assert_eq!(*counter.step.get(), 5);

        counter.increment();
        assert_eq!(*counter.count.get(), 5);

        counter.increment();
        assert_eq!(*counter.count.get(), 10);

        counter.undo();
        assert_eq!(*counter.count.get(), 5);
    }

    #[tokio::test]
    async fn test_counter_rendering() {
        let counter = Counter::new();
        let context = RenderContext::new(&Theme::default());
        
        let vdom = counter.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());
    }
}
