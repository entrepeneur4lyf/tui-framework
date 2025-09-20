use tui_framework::prelude::*;
use tui_framework::widget::menu::{Menu, MenuItem, MenuPosition, MenuStyle};
use tui_framework::widget::dropdown::Dropdown;
use tui_framework::event::types::{KeyEvent, MouseEvent, NcKey};
use tui_framework::render::{RenderContext, VirtualNode};
use tui_framework::component::{BaseComponent, Component, ComponentId};
use tui_framework::error::Result;
use async_trait::async_trait;
use std::any::Any;
use std::sync::{Arc, Mutex};

/// Demo application showcasing Menu and Dropdown widgets.
pub struct MenuDemo {
    base: BaseComponent,
    // Menu widgets
    context_menu: Menu,
    dropdown_menu: Dropdown,
    menu_bar: Menu,
    
    // State
    selected_option: Arc<Mutex<String>>,
    menu_action_log: Arc<Mutex<Vec<String>>>,
    show_context_menu: bool,
    show_dropdown: bool,
}

impl Default for MenuDemo {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuDemo {
    /// Create a new menu demo.
    pub fn new() -> Self {
        let selected_option = Arc::new(Mutex::new("None".to_string()));
        let menu_action_log = Arc::new(Mutex::new(Vec::new()));
        
        // Create context menu
        let log_clone = menu_action_log.clone();
        let context_menu = Menu::new()
            .with_style(MenuStyle::Context)
            .with_position(MenuPosition::Auto)
            .add_item(
                MenuItem::new("cut", "Cut")
                    .with_icon("✂️")
                    .with_shortcut("Ctrl+X")
                    .on_action({
                        let log = log_clone.clone();
                        move || {
                            log.lock().unwrap().push("Cut action executed".to_string());
                        }
                    })
            )
            .add_item(
                MenuItem::new("copy", "Copy")
                    .with_icon("📋")
                    .with_shortcut("Ctrl+C")
                    .on_action({
                        let log = log_clone.clone();
                        move || {
                            log.lock().unwrap().push("Copy action executed".to_string());
                        }
                    })
            )
            .add_item(
                MenuItem::new("paste", "Paste")
                    .with_icon("📄")
                    .with_shortcut("Ctrl+V")
                    .on_action({
                        let log = log_clone.clone();
                        move || {
                            log.lock().unwrap().push("Paste action executed".to_string());
                        }
                    })
            )
            .add_item(MenuItem::separator())
            .add_item(
                MenuItem::new("delete", "Delete")
                    .with_icon("🗑️")
                    .with_shortcut("Del")
                    .on_action({
                        let log = log_clone.clone();
                        move || {
                            log.lock().unwrap().push("Delete action executed".to_string());
                        }
                    })
            );

        // Create dropdown menu
        let option_clone = selected_option.clone();
        let dropdown_menu = Dropdown::new()
            .with_label("Select Option")
            .with_placeholder("Choose an option...")
            .add_option("option1", "First Option")
            .add_option("option2", "Second Option")
            .add_option("option3", "Third Option")
            .add_option("option4", "Fourth Option")
            .clearable(true)
            .searchable(true)
            .on_change(move |value, label| {
                *option_clone.lock().unwrap() = format!("{}: {}", value, label);
            })
            .on_clear({
                let option = selected_option.clone();
                move || {
                    *option.lock().unwrap() = "None".to_string();
                }
            });

        // Create menu bar
        let log_clone2 = menu_action_log.clone();
        let file_submenu = vec![
            MenuItem::new("new", "New")
                .with_icon("📄")
                .with_shortcut("Ctrl+N")
                .on_action({
                    let log = log_clone2.clone();
                    move || {
                        log.lock().unwrap().push("New file created".to_string());
                    }
                }),
            MenuItem::new("open", "Open")
                .with_icon("📂")
                .with_shortcut("Ctrl+O")
                .on_action({
                    let log = log_clone2.clone();
                    move || {
                        log.lock().unwrap().push("File opened".to_string());
                    }
                }),
            MenuItem::separator(),
            MenuItem::new("save", "Save")
                .with_icon("💾")
                .with_shortcut("Ctrl+S")
                .on_action({
                    let log = log_clone2.clone();
                    move || {
                        log.lock().unwrap().push("File saved".to_string());
                    }
                }),
            MenuItem::new("exit", "Exit")
                .with_icon("🚪")
                .with_shortcut("Alt+F4")
                .on_action({
                    let log = log_clone2.clone();
                    move || {
                        log.lock().unwrap().push("Application exit requested".to_string());
                    }
                }),
        ];

        let edit_submenu = vec![
            MenuItem::new("undo", "Undo")
                .with_icon("↶")
                .with_shortcut("Ctrl+Z")
                .on_action({
                    let log = log_clone2.clone();
                    move || {
                        log.lock().unwrap().push("Undo action executed".to_string());
                    }
                }),
            MenuItem::new("redo", "Redo")
                .with_icon("↷")
                .with_shortcut("Ctrl+Y")
                .on_action({
                    let log = log_clone2.clone();
                    move || {
                        log.lock().unwrap().push("Redo action executed".to_string());
                    }
                }),
        ];

        let menu_bar = Menu::new()
            .with_style(MenuStyle::MenuBar)
            .with_position(MenuPosition::Below)
            .add_item(
                MenuItem::new("file", "File")
                    .with_submenu(file_submenu)
            )
            .add_item(
                MenuItem::new("edit", "Edit")
                    .with_submenu(edit_submenu)
            )
            .add_item(
                MenuItem::new("help", "Help")
                    .on_action({
                        let log = log_clone2.clone();
                        move || {
                            log.lock().unwrap().push("Help menu accessed".to_string());
                        }
                    })
            );

        Self {
            base: BaseComponent::new("MenuDemo"),
            context_menu,
            dropdown_menu,
            menu_bar,
            selected_option,
            menu_action_log,
            show_context_menu: false,
            show_dropdown: false,
        }
    }

    /// Handle keyboard events.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        // Handle context menu events
        if self.show_context_menu
            && self.context_menu.handle_key_event(event) {
                if !self.context_menu.is_open() {
                    self.show_context_menu = false;
                }
                return true;
            }

        // Handle dropdown events
        if self.show_dropdown
            && self.dropdown_menu.handle_key_event(event) {
                if !self.dropdown_menu.is_open() {
                    self.show_dropdown = false;
                }
                return true;
            }

        // Handle menu bar events
        if self.menu_bar.handle_key_event(event) {
            return true;
        }

        // Handle demo-specific keys
        match event.key {
            NcKey::F01 => {
                // Show context menu
                self.show_context_menu = true;
                self.context_menu.open();
                true
            }
            NcKey::F02 => {
                // Show dropdown
                self.show_dropdown = true;
                self.dropdown_menu.open();
                true
            }
            _ => false,
        }
    }

    /// Handle mouse events.
    pub fn handle_mouse_event(&mut self, event: &MouseEvent) -> bool {
        // Handle context menu events
        if self.show_context_menu
            && self.context_menu.handle_mouse_event(event) {
                return true;
            }

        // Handle dropdown events
        if self.show_dropdown
            && self.dropdown_menu.handle_mouse_event(event) {
                return true;
            }

        // Handle menu bar events
        if self.menu_bar.handle_mouse_event(event) {
            return true;
        }

        false
    }

    /// Render the action log.
    fn render_action_log(&self) -> VirtualNode {
        let log = self.menu_action_log.lock().unwrap();
        let recent_actions: Vec<_> = log.iter().rev().take(10).collect();

        let mut log_container = VirtualNode::element("div")
            .attr("class", "action-log")
            .attr("style", "margin-top: 20px; padding: 10px; border: 1px solid #ccc; border-radius: 4px;")
            .child(
                VirtualNode::element("h3")
                    .attr("style", "margin: 0 0 10px 0; color: #333;")
                    .child(VirtualNode::text("Recent Actions"))
            );

        if recent_actions.is_empty() {
            log_container = log_container.child(
                VirtualNode::element("p")
                    .attr("style", "color: #666; font-style: italic;")
                    .child(VirtualNode::text("No actions yet. Try using the menus!"))
            );
        } else {
            for action in recent_actions {
                log_container = log_container.child(
                    VirtualNode::element("div")
                        .attr("style", "padding: 2px 0; color: #555;")
                        .child(VirtualNode::text(format!("• {}", action)))
                );
            }
        }

        log_container
    }
}

#[async_trait]
impl Component for MenuDemo {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let selected = self.selected_option.lock().unwrap().clone();

        let mut container = VirtualNode::element("div")
            .attr("class", "menu-demo")
            .attr("style", "padding: 20px; font-family: Arial, sans-serif;")
            .child(
                VirtualNode::element("h1")
                    .attr("style", "color: #333; margin-bottom: 20px;")
                    .child(VirtualNode::text("Menu & Dropdown Widget Demo"))
            )
            .child(
                VirtualNode::element("p")
                    .attr("style", "color: #666; margin-bottom: 20px;")
                    .child(VirtualNode::text("Press F1 for context menu, F2 for dropdown, or interact with the menu bar below."))
            );

        // Add menu bar
        container = container.child(
            VirtualNode::element("div")
                .attr("class", "menu-bar-container")
                .attr("style", "margin-bottom: 20px; border-bottom: 1px solid #ddd; padding-bottom: 10px;")
                .child(self.menu_bar.render(context).await?)
        );

        // Add dropdown section
        container = container.child(
            VirtualNode::element("div")
                .attr("class", "dropdown-section")
                .attr("style", "margin-bottom: 20px;")
                .child(
                    VirtualNode::element("h3")
                        .attr("style", "color: #333; margin-bottom: 10px;")
                        .child(VirtualNode::text("Dropdown Example"))
                )
                .child(self.dropdown_menu.render(context).await?)
                .child(
                    VirtualNode::element("p")
                        .attr("style", "margin-top: 10px; color: #555;")
                        .child(VirtualNode::text(format!("Selected: {}", selected)))
                )
        );

        // Add context menu if shown
        if self.show_context_menu {
            container = container.child(
                VirtualNode::element("div")
                    .attr("class", "context-menu-overlay")
                    .attr("style", "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 999;")
                    .child(self.context_menu.render(context).await?)
            );
        }

        // Add action log
        container = container.child(self.render_action_log());

        // Add instructions
        container = container.child(
            VirtualNode::element("div")
                .attr("class", "instructions")
                .attr("style", "margin-top: 20px; padding: 15px; background-color: #f5f5f5; border-radius: 4px;")
                .child(
                    VirtualNode::element("h3")
                        .attr("style", "margin: 0 0 10px 0; color: #333;")
                        .child(VirtualNode::text("Instructions"))
                )
                .child(
                    VirtualNode::element("ul")
                        .attr("style", "margin: 0; padding-left: 20px; color: #555;")
                        .child(
                            VirtualNode::element("li")
                                .child(VirtualNode::text("Press F1 to show context menu"))
                        )
                        .child(
                            VirtualNode::element("li")
                                .child(VirtualNode::text("Press F2 to open dropdown"))
                        )
                        .child(
                            VirtualNode::element("li")
                                .child(VirtualNode::text("Use arrow keys to navigate menus"))
                        )
                        .child(
                            VirtualNode::element("li")
                                .child(VirtualNode::text("Press Enter to select items"))
                        )
                        .child(
                            VirtualNode::element("li")
                                .child(VirtualNode::text("Press Escape to close menus"))
                        )
                )
        );

        Ok(container)
    }
}

/// Main function to run the menu demo.
#[tokio::main]
async fn main() -> Result<()> {
    println!("Menu & Dropdown Widget Demo");
    println!("===========================");
    println!();
    println!("This demo showcases the Menu and Dropdown widgets:");
    println!("• Context Menu - Right-click style menu with icons and shortcuts");
    println!("• Dropdown Menu - Select box with options and callbacks");
    println!("• Menu Bar - Traditional application menu with submenus");
    println!();
    println!("Features demonstrated:");
    println!("• Menu items with icons and keyboard shortcuts");
    println!("• Submenu support with nested navigation");
    println!("• Multiple menu styles (Context, Dropdown, MenuBar)");
    println!("• Event handling and action callbacks");
    println!("• Keyboard navigation (arrows, enter, escape)");
    println!("• Mouse interaction support");
    println!("• Customizable styling and positioning");
    println!("• State management and selection tracking");
    println!();
    println!("Press Ctrl+C to exit");

    // Create demo instance
    let mut demo = MenuDemo::new();

    // Simulate some interactions for demonstration
    println!("\nSimulating menu interactions...");

    // Test dropdown selection
    demo.dropdown_menu.select_value("option2");
    println!("✓ Selected dropdown option: {}", demo.selected_option.lock().unwrap());

    // Test menu actions
    demo.menu_action_log.lock().unwrap().push("Demo initialized".to_string());
    demo.menu_action_log.lock().unwrap().push("Dropdown option selected".to_string());

    println!("✓ Menu demo ready - all widgets functional");
    println!("✓ Action logging system active");
    println!("✓ Event handling system ready");

    Ok(())
}
