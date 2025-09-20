//! Simple List widget example.
//!
//! This example demonstrates basic List widget usage with:
//! - Creating a list with items
//! - Setting up selection callbacks
//! - Basic keyboard navigation
//! - Rendering the list component

use tui_framework::event::types::{MouseButton, MouseEventType, NcKey};
use tui_framework::prelude::*;

/// Simple list component that demonstrates basic functionality
struct SimpleListComponent {
    base: tui_framework::component::BaseComponent,
    list: List,
}

impl SimpleListComponent {
    /// Create a new simple list component
    pub fn new() -> Self {
        let mut list = List::new()
            .with_selection_mode(SelectionMode::Single)
            .with_visible_items(5);

        // Add some sample items
        let fruits = [
            "🍎 Apple",
            "🍌 Banana",
            "🍒 Cherry",
            "📅 Date",
            "🫐 Elderberry",
            "🥝 Kiwi",
            "🍋 Lemon",
            "🥭 Mango",
            "🍊 Orange",
            "🍑 Peach",
        ];

        for (i, fruit) in fruits.iter().enumerate() {
            list.add_item(ListItem::new(i.to_string(), fruit.to_string()));
        }

        // Set up callbacks
        list = list
            .on_selection_changed(|selected| {
                println!("Selection changed: {:?}", selected);
            })
            .on_item_activated(|index, item| {
                println!("Item activated: {} - {}", index, item.text);
            });

        Self {
            base: tui_framework::component::BaseComponent::new("SimpleListComponent"),
            list,
        }
    }

    /// Handle keyboard events
    pub fn handle_key(&mut self, event: &KeyEvent) -> bool {
        self.list.handle_key_event(event)
    }

    /// Handle mouse events
    pub fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        self.list.handle_mouse_event(event)
    }

    /// Get the current selection
    pub fn get_selected_items(&self) -> &[usize] {
        self.list.selected_indices()
    }

    /// Get the focused item
    pub fn get_focused_item(&self) -> Option<usize> {
        self.list.focused_index()
    }

    /// Add a new item to the list
    pub fn add_item(&mut self, text: &str) {
        let id = self.list.items().len().to_string();
        self.list.add_item(ListItem::new(id, text.to_string()));
    }

    /// Remove the currently selected item
    #[allow(dead_code)]
    pub fn remove_selected(&mut self) -> bool {
        if let Some(&selected) = self.list.selected_indices().first() {
            self.list.remove_item(selected);
            true
        } else {
            false
        }
    }
}

#[async_trait]
impl Component for SimpleListComponent {
    fn id(&self) -> tui_framework::component::ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "SimpleListComponent"
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        // Create a container with the list and some instructions
        let list_node = self.list.render(context).await?;

        Ok(div()
            .attr("class", "simple-list-container")
            .child(
                div()
                    .attr("class", "header")
                    .child(text("🗂️ Simple List Example")),
            )
            .child(div().attr("class", "list-wrapper").child(list_node))
            .child(
                div()
                    .attr("class", "instructions")
                    .child(text("Instructions:"))
                    .child(text("• Use ↑↓ arrow keys to navigate"))
                    .child(text("• Press Space to select"))
                    .child(text("• Press Enter to activate"))
                    .child(text("• Use PgUp/PgDn for page navigation")),
            )
            .child(
                div()
                    .attr("class", "status")
                    .child(text(format!(
                        "Selected: {:?}",
                        self.list.selected_indices()
                    )))
                    .child(text(format!("Focused: {:?}", self.list.focused_index()))),
            ))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Demo function showing how to use the List widget
async fn demo_list_usage() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🗂️ Simple List Widget Demo");
    println!("==========================");

    // Create the list component
    let mut list_component = SimpleListComponent::new();

    // Create a render context
    let theme = tui_framework::style::theme::Theme::default();
    let context = RenderContext::new(&theme);

    // Render the component
    let _rendered = list_component.render(&context).await?;

    println!("✅ List component created and rendered successfully!");
    println!("📊 Component stats:");
    println!("   - Items: {}", list_component.list.items().len());
    println!("   - Selection mode: Single (default)");
    println!("   - Visible items: 5");

    // Simulate some interactions
    println!("\n🎮 Simulating interactions:");

    // Navigate down
    let down_event = KeyEvent::new(NcKey::Down);
    list_component.handle_key(&down_event);
    println!(
        "   - Pressed Down: focused = {:?}",
        list_component.get_focused_item()
    );

    // Navigate down again
    list_component.handle_key(&down_event);
    println!(
        "   - Pressed Down: focused = {:?}",
        list_component.get_focused_item()
    );

    // Select current item
    let space_event = KeyEvent::new(NcKey::Space);
    list_component.handle_key(&space_event);
    println!(
        "   - Pressed Space: selected = {:?}",
        list_component.get_selected_items()
    );

    // Navigate to end
    let end_event = KeyEvent::new(NcKey::End);
    list_component.handle_key(&end_event);
    println!(
        "   - Pressed End: focused = {:?}",
        list_component.get_focused_item()
    );

    // Add a new item
    list_component.add_item("🆕 New Fruit");
    println!(
        "   - Added item: total items = {}",
        list_component.list.items().len()
    );

    // Simulate mouse click
    let mouse_event = MouseEvent::new(MouseButton::Left, MouseEventType::Press, 0, 3);
    list_component.handle_mouse(&mouse_event);
    println!(
        "   - Mouse click at row 3: focused = {:?}, selected = {:?}",
        list_component.get_focused_item(),
        list_component.get_selected_items()
    );

    println!("\n✨ Demo completed successfully!");
    println!("   The List widget supports:");
    println!("   ✓ Keyboard navigation (arrows, home, end, page up/down)");
    println!("   ✓ Mouse interaction (click to select)");
    println!("   ✓ Multiple selection modes (none, single, multiple)");
    println!("   ✓ Scrolling for large lists");
    println!("   ✓ Event callbacks for selection and activation");
    println!("   ✓ Dynamic item management (add, remove, insert)");

    Ok(())
}

/// Test the List widget functionality
#[tokio::test]
async fn test_simple_list() {
    let mut list_component = SimpleListComponent::new();

    // Test initial state
    assert_eq!(list_component.list.items().len(), 10);
    assert_eq!(list_component.get_selected_items().len(), 0);
    assert_eq!(list_component.get_focused_item(), None);

    // Test navigation
    let down_event = KeyEvent::new(NcKey::Down);
    assert!(list_component.handle_key(&down_event));
    assert_eq!(list_component.get_focused_item(), Some(0));

    // Test selection
    let space_event = KeyEvent::new(NcKey::Space);
    assert!(list_component.handle_key(&space_event));
    assert_eq!(list_component.get_selected_items(), &[0]);

    // Test adding items
    list_component.add_item("Test Item");
    assert_eq!(list_component.list.items().len(), 11);

    // Test rendering
    let theme = tui_framework::style::theme::Theme::default();
    let context = RenderContext::new(&theme);
    let rendered = list_component.render(&context).await.unwrap();
    assert_eq!(rendered.tag(), Some("div"));
}

/// Main function for running the demo
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    demo_list_usage().await
}
