use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::event::types::{KeyEvent, MouseEvent};
use crate::render::{RenderContext, VirtualNode};
use crate::style::Color;
use crate::widget::menu::{Menu, MenuItem, MenuPosition, MenuStyle};
use crate::widget::Widget;
use async_trait::async_trait;
use std::any::Any;

/// Dropdown widget that combines a trigger element with a menu.
pub struct Dropdown {
    base: BaseComponent,
    // Core properties
    label: String,
    placeholder: String,
    selected_value: Option<String>,
    selected_label: Option<String>,
    
    // Menu
    menu: Menu,
    
    // Configuration
    disabled: bool,
    searchable: bool,
    clearable: bool,
    
    // Styling
    background_color: Color,
    text_color: Color,
    border_color: Color,
    focus_color: Color,
    disabled_color: Color,
    
    // Callbacks
    on_change: Option<Box<dyn Fn(&str, &str) + Send + Sync>>, // (value, label)
    on_clear: Option<Box<dyn Fn() + Send + Sync>>,
}

impl std::fmt::Debug for Dropdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dropdown")
            .field("label", &self.label)
            .field("placeholder", &self.placeholder)
            .field("selected_value", &self.selected_value)
            .field("selected_label", &self.selected_label)
            .field("disabled", &self.disabled)
            .field("searchable", &self.searchable)
            .field("clearable", &self.clearable)
            .field("background_color", &self.background_color)
            .field("text_color", &self.text_color)
            .field("border_color", &self.border_color)
            .field("focus_color", &self.focus_color)
            .field("disabled_color", &self.disabled_color)
            .field("on_change", &self.on_change.as_ref().map(|_| "Fn(&str, &str)"))
            .field("on_clear", &self.on_clear.as_ref().map(|_| "Fn()"))
            .finish()
    }
}

impl Default for Dropdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Dropdown {
    /// Create a new dropdown.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("Dropdown"),
            label: String::new(),
            placeholder: "Select an option...".to_string(),
            selected_value: None,
            selected_label: None,
            menu: Menu::new()
                .with_style(MenuStyle::Dropdown)
                .with_position(MenuPosition::Below),
            disabled: false,
            searchable: false,
            clearable: false,
            background_color: Color::rgba(255, 255, 255, 255),
            text_color: Color::rgba(0, 0, 0, 255),
            border_color: Color::rgba(200, 200, 200, 255),
            focus_color: Color::rgba(70, 130, 180, 255),
            disabled_color: Color::rgba(128, 128, 128, 255),
            on_change: None,
            on_clear: None,
        }
    }

    /// Set the dropdown label.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set the placeholder text.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Add an option to the dropdown.
    pub fn add_option(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        let value_str = value.into();
        let label_str = label.into();

        let item = MenuItem::new(&value_str, &label_str)
            .on_action(move || {
                // This will be handled by the dropdown's select callback
            });

        self.menu = self.menu.add_item(item);
        self
    }

    /// Add multiple options to the dropdown.
    pub fn add_options(mut self, options: Vec<(String, String)>) -> Self {
        for (value, label) in options {
            self = self.add_option(value, label);
        }
        self
    }

    /// Set the selected value.
    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        let value_str = value.into();
        
        // Find the corresponding label
        for item in &self.menu.items {
            if item.id == value_str {
                self.selected_value = Some(value_str.clone());
                self.selected_label = Some(item.label.clone());
                break;
            }
        }
        
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enable search functionality.
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// Enable clear functionality.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Set background color.
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set text color.
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Set border color.
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set focus color.
    pub fn with_focus_color(mut self, color: Color) -> Self {
        self.focus_color = color;
        self
    }

    /// Set disabled color.
    pub fn with_disabled_color(mut self, color: Color) -> Self {
        self.disabled_color = color;
        self
    }

    /// Set change callback.
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, &str) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set clear callback.
    pub fn on_clear<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_clear = Some(Box::new(callback));
        self
    }

    /// Get the selected value.
    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    /// Get the selected label.
    pub fn selected_label(&self) -> Option<&str> {
        self.selected_label.as_deref()
    }

    /// Check if dropdown is open.
    pub fn is_open(&self) -> bool {
        self.menu.is_open()
    }

    /// Open the dropdown.
    pub fn open(&mut self) {
        if !self.disabled {
            self.menu.open();
        }
    }

    /// Close the dropdown.
    pub fn close(&mut self) {
        self.menu.close();
    }

    /// Toggle dropdown open/close state.
    pub fn toggle(&mut self) {
        if self.disabled {
            return;
        }
        
        self.menu.toggle();
    }

    /// Clear the selection.
    pub fn clear(&mut self) {
        if self.clearable && !self.disabled {
            self.selected_value = None;
            self.selected_label = None;
            
            if let Some(callback) = &self.on_clear {
                callback();
            }
        }
    }

    /// Select an option by value.
    pub fn select_value(&mut self, value: &str) {
        // Find the item with matching value
        for item in &self.menu.items {
            if item.id == value {
                self.selected_value = Some(value.to_string());
                self.selected_label = Some(item.label.clone());
                
                if let Some(callback) = &self.on_change {
                    callback(value, &item.label);
                }
                
                self.close();
                break;
            }
        }
    }

    /// Handle keyboard events.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if self.disabled {
            return false;
        }

        // Delegate to menu if open
        if self.is_open() {
            return self.menu.handle_key_event(event);
        }

        // Handle trigger events when closed
        match event.key {
            crate::event::types::NcKey::Enter | crate::event::types::NcKey::Space => {
                self.open();
                true
            }
            _ => false,
        }
    }

    /// Handle mouse events.
    pub fn handle_mouse_event(&mut self, event: &MouseEvent) -> bool {
        if self.disabled {
            return false;
        }

        // Delegate to menu if open
        if self.is_open() {
            return self.menu.handle_mouse_event(event);
        }

        // TODO: Handle click on trigger to open dropdown
        false
    }

    /// Render the dropdown trigger.
    fn render_trigger(&self) -> VirtualNode {
        let display_text = if let Some(label) = &self.selected_label {
            label.clone()
        } else {
            self.placeholder.clone()
        };

        let trigger_style = format!(
            "display: flex; align-items: center; justify-content: space-between; \
             padding: 8px 12px; border: 1px solid {}; border-radius: 4px; \
             background-color: {}; color: {}; cursor: {}; min-height: 20px;",
            if self.is_open() {
                self.focus_color.to_hex()
            } else {
                self.border_color.to_hex()
            },
            if self.disabled {
                self.disabled_color.to_hex()
            } else {
                self.background_color.to_hex()
            },
            if self.disabled {
                self.disabled_color.to_hex()
            } else {
                self.text_color.to_hex()
            },
            if self.disabled { "not-allowed" } else { "pointer" }
        );

        let mut trigger = VirtualNode::element("div")
            .attr("class", "dropdown-trigger")
            .attr("style", trigger_style)
            .child(
                VirtualNode::element("span")
                    .attr("class", "dropdown-text")
                    .child(VirtualNode::text(display_text))
            );

        // Add clear button if clearable and has selection
        if self.clearable && self.selected_value.is_some() && !self.disabled {
            trigger = trigger.child(
                VirtualNode::element("button")
                    .attr("class", "dropdown-clear")
                    .attr("style", "background: none; border: none; cursor: pointer; margin-left: 8px;")
                    .child(VirtualNode::text("✕"))
            );
        }

        // Add dropdown arrow
        trigger = trigger.child(
            VirtualNode::element("span")
                .attr("class", "dropdown-arrow")
                .attr("style", "margin-left: 8px;")
                .child(VirtualNode::text(if self.is_open() { "▲" } else { "▼" }))
        );

        trigger
    }
}

#[async_trait]
impl Component for Dropdown {
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
        let mut container = VirtualNode::element("div")
            .attr("class", "dropdown")
            .attr("style", "position: relative; display: inline-block;");

        // Add label if present
        if !self.label.is_empty() {
            container = container.child(
                VirtualNode::element("label")
                    .attr("class", "dropdown-label")
                    .attr("style", "display: block; margin-bottom: 4px; font-weight: 500;")
                    .child(VirtualNode::text(&self.label))
            );
        }

        // Add trigger
        container = container.child(self.render_trigger());

        // Add menu if open
        if self.is_open() {
            container = container.child(self.menu.render(context).await?);
        }

        Ok(container)
    }
}

impl Widget for Dropdown {
    fn widget_type(&self) -> &'static str {
        "Dropdown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::RenderContext;
    use crate::style::Theme;
    use crate::event::types::{KeyModifiers, NcKey};

    #[test]
    fn test_dropdown_creation() {
        let dropdown = Dropdown::new();
        assert!(dropdown.label.is_empty());
        assert_eq!(dropdown.placeholder, "Select an option...");
        assert_eq!(dropdown.selected_value(), None);
        assert_eq!(dropdown.selected_label(), None);
        assert!(!dropdown.disabled);
        assert!(!dropdown.searchable);
        assert!(!dropdown.clearable);
        assert!(!dropdown.is_open());
    }

    #[test]
    fn test_dropdown_configuration() {
        let dropdown = Dropdown::new()
            .with_label("Test Dropdown")
            .with_placeholder("Choose...")
            .disabled(true)
            .searchable(true)
            .clearable(true)
            .with_background_color(Color::rgba(240, 240, 240, 255))
            .with_text_color(Color::rgba(50, 50, 50, 255));

        assert_eq!(dropdown.label, "Test Dropdown");
        assert_eq!(dropdown.placeholder, "Choose...");
        assert!(dropdown.disabled);
        assert!(dropdown.searchable);
        assert!(dropdown.clearable);
        assert_eq!(dropdown.background_color, Color::rgba(240, 240, 240, 255));
        assert_eq!(dropdown.text_color, Color::rgba(50, 50, 50, 255));
    }

    #[test]
    fn test_dropdown_options() {
        let dropdown = Dropdown::new()
            .add_option("opt1", "Option 1")
            .add_option("opt2", "Option 2")
            .add_options(vec![
                ("opt3".to_string(), "Option 3".to_string()),
                ("opt4".to_string(), "Option 4".to_string()),
            ]);

        assert_eq!(dropdown.menu.items.len(), 4);
        assert_eq!(dropdown.menu.items[0].id, "opt1");
        assert_eq!(dropdown.menu.items[0].label, "Option 1");
        assert_eq!(dropdown.menu.items[3].id, "opt4");
        assert_eq!(dropdown.menu.items[3].label, "Option 4");
    }

    #[test]
    fn test_dropdown_selection() {
        let mut dropdown = Dropdown::new()
            .add_option("opt1", "Option 1")
            .add_option("opt2", "Option 2")
            .with_selected_value("opt1");

        assert_eq!(dropdown.selected_value(), Some("opt1"));
        assert_eq!(dropdown.selected_label(), Some("Option 1"));

        // Test programmatic selection
        dropdown.select_value("opt2");
        assert_eq!(dropdown.selected_value(), Some("opt2"));
        assert_eq!(dropdown.selected_label(), Some("Option 2"));
    }

    #[test]
    fn test_dropdown_clear() {
        let mut dropdown = Dropdown::new()
            .add_option("opt1", "Option 1")
            .clearable(true)
            .with_selected_value("opt1");

        assert_eq!(dropdown.selected_value(), Some("opt1"));

        dropdown.clear();
        assert_eq!(dropdown.selected_value(), None);
        assert_eq!(dropdown.selected_label(), None);

        // Test clear when not clearable
        let mut dropdown_no_clear = Dropdown::new()
            .add_option("opt1", "Option 1")
            .clearable(false)
            .with_selected_value("opt1");

        dropdown_no_clear.clear();
        assert_eq!(dropdown_no_clear.selected_value(), Some("opt1")); // Should not clear
    }

    #[test]
    fn test_dropdown_open_close() {
        let mut dropdown = Dropdown::new()
            .add_option("opt1", "Option 1");

        // Test opening
        assert!(!dropdown.is_open());
        dropdown.open();
        assert!(dropdown.is_open());

        // Test closing
        dropdown.close();
        assert!(!dropdown.is_open());

        // Test toggle
        dropdown.toggle();
        assert!(dropdown.is_open());
        dropdown.toggle();
        assert!(!dropdown.is_open());

        // Test disabled dropdown
        let mut disabled_dropdown = Dropdown::new()
            .add_option("opt1", "Option 1")
            .disabled(true);

        disabled_dropdown.open();
        assert!(!disabled_dropdown.is_open()); // Should not open when disabled

        disabled_dropdown.toggle();
        assert!(!disabled_dropdown.is_open()); // Should not toggle when disabled
    }

    #[test]
    fn test_dropdown_key_events() {
        let mut dropdown = Dropdown::new()
            .add_option("opt1", "Option 1");

        // Test opening with Enter key
        let enter_event = KeyEvent {
            key: NcKey::Enter,
            modifiers: KeyModifiers::empty(),
            bubbles: true,
            prevent_default: false,
        };
        assert!(dropdown.handle_key_event(&enter_event));
        assert!(dropdown.is_open());

        // Test disabled dropdown
        let mut disabled_dropdown = Dropdown::new()
            .add_option("opt1", "Option 1")
            .disabled(true);

        assert!(!disabled_dropdown.handle_key_event(&enter_event));
        assert!(!disabled_dropdown.is_open());
    }

    #[test]
    fn test_dropdown_callbacks() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(false));
        let cleared = Arc::new(Mutex::new(false));

        let changed_clone = changed.clone();
        let cleared_clone = cleared.clone();

        let mut dropdown = Dropdown::new()
            .add_option("opt1", "Option 1")
            .clearable(true)
            .on_change(move |_value, _label| {
                *changed_clone.lock().unwrap() = true;
            })
            .on_clear(move || {
                *cleared_clone.lock().unwrap() = true;
            });

        // Test change callback
        dropdown.select_value("opt1");
        assert!(*changed.lock().unwrap());

        // Test clear callback
        dropdown.clear();
        assert!(*cleared.lock().unwrap());
    }

    #[tokio::test]
    async fn test_dropdown_rendering() {
        let dropdown = Dropdown::new()
            .with_label("Test Dropdown")
            .add_option("opt1", "Option 1")
            .add_option("opt2", "Option 2")
            .with_selected_value("opt1");

        let theme = Theme::default();
        let context = RenderContext::new(&theme);

        let result = dropdown.render(&context).await;
        assert!(result.is_ok());
        let vnode = result.unwrap();

        // Should render as a div with dropdown class
        if let VirtualNode::Element(element) = &vnode {
            assert_eq!(element.tag, "div");
            assert!(element.attributes.get("class").unwrap().contains("dropdown"));
            assert!(element.attributes.get("style").unwrap().contains("position: relative"));
        } else {
            panic!("Expected element node");
        }
    }
}
