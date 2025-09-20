//! Comprehensive List widget demonstration.
//!
//! This example showcases all the features of the List widget including:
//! - Item management (add, remove, insert)
//! - Different selection modes (single, multiple, none)
//! - Keyboard navigation and scrolling
//! - Mouse interaction
//! - Event handling and callbacks
//! - Styling and theming

use std::sync::{Arc, Mutex};
use tui_framework::event::types::NcKey;
use tui_framework::prelude::*;

/// Demo application state
#[derive(Debug, Clone)]
struct AppState {
    items: Vec<String>,
    selected_items: Vec<usize>,
    focused_item: Option<usize>,
    selection_mode: SelectionMode,
    status_message: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            items: vec![
                "Apple".to_string(),
                "Banana".to_string(),
                "Cherry".to_string(),
                "Date".to_string(),
                "Elderberry".to_string(),
                "Fig".to_string(),
                "Grape".to_string(),
                "Honeydew".to_string(),
                "Kiwi".to_string(),
                "Lemon".to_string(),
                "Mango".to_string(),
                "Orange".to_string(),
                "Papaya".to_string(),
                "Quince".to_string(),
                "Raspberry".to_string(),
            ],
            selected_items: Vec::new(),
            focused_item: None,
            selection_mode: SelectionMode::Single,
            status_message: "Welcome to List Demo! Use arrow keys to navigate.".to_string(),
        }
    }
}

/// Create a list widget with the current state
fn create_list_widget(state: &AppState) -> List {
    let mut list = List::new()
        .with_selection_mode(state.selection_mode)
        .with_visible_items(8);

    // Add items to the list
    for (index, item_text) in state.items.iter().enumerate() {
        let item = ListItem::new(index.to_string(), item_text.clone());
        list.add_item(item);
    }

    // Set focus and selection
    if let Some(focused) = state.focused_item {
        list.set_focused_index(Some(focused));
    }

    for &selected in &state.selected_items {
        list.select_item(selected);
    }

    list
}

/// Create the main application component
fn create_app_component(state: Arc<Mutex<AppState>>) -> VirtualNode {
    let state_guard = state.lock().unwrap();
    let current_state = state_guard.clone();
    drop(state_guard);

    // Create the list widget
    let _list = create_list_widget(&current_state);

    // Create the UI layout
    div()
        .attr("class", "app-container")
        .child(
            div()
                .attr("class", "header")
                .child(text("🗂️  List Widget Demo"))
                .child(text(format!(
                    "Selection Mode: {:?}",
                    current_state.selection_mode
                ))),
        )
        .child(
            div()
                .attr("class", "main-content")
                .child(
                    div()
                        .attr("class", "list-container")
                        .child(text("📋 Fruit List:"))
                        // In a real implementation, this would be the actual List widget
                        // For now, we'll create a mock representation
                        .child(create_list_mock(&current_state)),
                )
                .child(
                    div()
                        .attr("class", "controls")
                        .child(text("🎮 Controls:"))
                        .child(text("↑↓ - Navigate"))
                        .child(text("Space - Select"))
                        .child(text("Enter - Activate"))
                        .child(text("PgUp/PgDn - Page"))
                        .child(text("Home/End - First/Last"))
                        .child(text(""))
                        .child(text("🔧 Actions:"))
                        .child(text("A - Add item"))
                        .child(text("D - Delete selected"))
                        .child(text("M - Toggle mode"))
                        .child(text("C - Clear selection")),
                ),
        )
        .child(
            div()
                .attr("class", "status")
                .child(text(format!("Status: {}", current_state.status_message)))
                .child(text(format!(
                    "Selected: {:?}",
                    current_state.selected_items
                )))
                .child(text(format!("Focused: {:?}", current_state.focused_item))),
        )
}

/// Create a mock representation of the list for demonstration
fn create_list_mock(state: &AppState) -> VirtualNode {
    let mut list_node = div().attr("class", "list-mock");

    for (index, item) in state.items.iter().enumerate() {
        let is_selected = state.selected_items.contains(&index);
        let is_focused = state.focused_item == Some(index);

        let mut classes = vec!["list-item"];
        if is_selected {
            classes.push("selected");
        }
        if is_focused {
            classes.push("focused");
        }

        let prefix = if is_selected && is_focused {
            "►● "
        } else if is_selected {
            "  ● "
        } else if is_focused {
            "► "
        } else {
            "  "
        };

        list_node = list_node.child(
            div()
                .attr("class", classes.join(" "))
                .child(text(format!("{}{}", prefix, item))),
        );
    }

    list_node
}

/// Handle keyboard input for the demo
#[allow(dead_code)]
fn handle_key_input(key: NcKey, state: Arc<Mutex<AppState>>) {
    let mut state_guard = state.lock().unwrap();

    match key {
        NcKey::Up => {
            if let Some(focused) = state_guard.focused_item {
                if focused > 0 {
                    state_guard.focused_item = Some(focused - 1);
                    state_guard.status_message = "Moved up".to_string();
                }
            } else if !state_guard.items.is_empty() {
                state_guard.focused_item = Some(state_guard.items.len() - 1);
                state_guard.status_message = "Focus set to last item".to_string();
            }
        }
        NcKey::Down => {
            if let Some(focused) = state_guard.focused_item {
                if focused + 1 < state_guard.items.len() {
                    state_guard.focused_item = Some(focused + 1);
                    state_guard.status_message = "Moved down".to_string();
                }
            } else if !state_guard.items.is_empty() {
                state_guard.focused_item = Some(0);
                state_guard.status_message = "Focus set to first item".to_string();
            }
        }
        NcKey::Home => {
            if !state_guard.items.is_empty() {
                state_guard.focused_item = Some(0);
                state_guard.status_message = "Moved to first item".to_string();
            }
        }
        NcKey::End => {
            if !state_guard.items.is_empty() {
                state_guard.focused_item = Some(state_guard.items.len() - 1);
                state_guard.status_message = "Moved to last item".to_string();
            }
        }
        NcKey::Space => {
            if let Some(focused) = state_guard.focused_item {
                match state_guard.selection_mode {
                    SelectionMode::None => {
                        state_guard.status_message = "Selection disabled".to_string();
                    }
                    SelectionMode::Single => {
                        state_guard.selected_items.clear();
                        state_guard.selected_items.push(focused);
                        state_guard.status_message =
                            format!("Selected: {}", state_guard.items[focused]);
                    }
                    SelectionMode::Multiple => {
                        if let Some(pos) = state_guard
                            .selected_items
                            .iter()
                            .position(|&i| i == focused)
                        {
                            state_guard.selected_items.remove(pos);
                            state_guard.status_message =
                                format!("Deselected: {}", state_guard.items[focused]);
                        } else {
                            state_guard.selected_items.push(focused);
                            state_guard.selected_items.sort_unstable();
                            state_guard.status_message =
                                format!("Selected: {}", state_guard.items[focused]);
                        }
                    }
                }
            }
        }
        NcKey::Enter => {
            if let Some(focused) = state_guard.focused_item {
                state_guard.status_message = format!("Activated: {}", state_guard.items[focused]);
            }
        }
        _ => {
            // Handle character input for commands
            // This would need to be implemented based on the actual character
        }
    }
}

/// Handle character commands
#[allow(dead_code)]
fn handle_command(command: char, state: Arc<Mutex<AppState>>) {
    let mut state_guard = state.lock().unwrap();

    match command.to_ascii_lowercase() {
        'a' => {
            let new_item = format!("New Item {}", state_guard.items.len() + 1);
            state_guard.items.push(new_item.clone());
            state_guard.status_message = format!("Added: {}", new_item);
        }
        'd' => {
            if !state_guard.selected_items.is_empty() {
                let mut removed_items = Vec::new();

                // Remove items in reverse order to maintain indices
                let mut indices_to_remove = state_guard.selected_items.clone();
                indices_to_remove.sort_unstable();
                for &index in indices_to_remove.iter().rev() {
                    if index < state_guard.items.len() {
                        removed_items.push(state_guard.items.remove(index));
                    }
                }

                state_guard.selected_items.clear();
                state_guard.focused_item = None;
                state_guard.status_message = format!("Deleted {} items", removed_items.len());
            } else {
                state_guard.status_message = "No items selected for deletion".to_string();
            }
        }
        'm' => {
            state_guard.selection_mode = match state_guard.selection_mode {
                SelectionMode::None => SelectionMode::Single,
                SelectionMode::Single => SelectionMode::Multiple,
                SelectionMode::Multiple => SelectionMode::None,
            };
            state_guard.selected_items.clear();
            state_guard.status_message =
                format!("Selection mode: {:?}", state_guard.selection_mode);
        }
        'c' => {
            state_guard.selected_items.clear();
            state_guard.status_message = "Selection cleared".to_string();
        }
        _ => {
            state_guard.status_message = format!("Unknown command: {}", command);
        }
    }
}

/// Main demo function
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🗂️  List Widget Demo");
    println!("==================");
    println!();
    println!("This demo showcases the List widget functionality:");
    println!("• Navigation with arrow keys, Home, End, PgUp, PgDn");
    println!("• Selection with Space key");
    println!("• Activation with Enter key");
    println!("• Different selection modes (None, Single, Multiple)");
    println!("• Dynamic item management");
    println!();
    println!("Commands:");
    println!("  A - Add new item");
    println!("  D - Delete selected items");
    println!("  M - Toggle selection mode");
    println!("  C - Clear selection");
    println!("  Q - Quit");
    println!();

    let state = Arc::new(Mutex::new(AppState::default()));

    // In a real TUI application, this would be handled by the event loop
    // For this demo, we'll just show the initial state
    let _app_component = create_app_component(state.clone());

    println!("Initial state:");
    println!("Items: {:?}", state.lock().unwrap().items);
    println!("Selection mode: {:?}", state.lock().unwrap().selection_mode);
    println!();
    println!("Demo completed! In a real application, this would be interactive.");

    Ok(())
}
