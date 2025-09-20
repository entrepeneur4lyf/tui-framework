use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::event::types::{KeyEvent, MouseEvent, NcKey};
use crate::render::{RenderContext, VirtualNode};
use crate::style::Color;
use crate::widget::Widget;
use async_trait::async_trait;
use std::any::Any;

/// Menu item configuration.
pub struct MenuItem {
    /// Item ID for identification.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional icon.
    pub icon: Option<String>,
    /// Whether the item is enabled.
    pub enabled: bool,
    /// Whether the item is visible.
    pub visible: bool,
    /// Optional keyboard shortcut.
    pub shortcut: Option<String>,
    /// Whether this item is a separator.
    pub is_separator: bool,
    /// Submenu items.
    pub submenu: Option<Vec<MenuItem>>,
    /// Action callback.
    pub action: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Clone for MenuItem {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            label: self.label.clone(),
            icon: self.icon.clone(),
            enabled: self.enabled,
            visible: self.visible,
            shortcut: self.shortcut.clone(),
            is_separator: self.is_separator,
            submenu: self.submenu.clone(),
            action: None, // Cannot clone function pointers
        }
    }
}

impl std::fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MenuItem")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("icon", &self.icon)
            .field("enabled", &self.enabled)
            .field("visible", &self.visible)
            .field("shortcut", &self.shortcut)
            .field("is_separator", &self.is_separator)
            .field("submenu", &self.submenu)
            .field("action", &self.action.as_ref().map(|_| "Fn()"))
            .finish()
    }
}

impl MenuItem {
    /// Create a new menu item.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            enabled: true,
            visible: true,
            shortcut: None,
            is_separator: false,
            submenu: None,
            action: None,
        }
    }

    /// Create a separator item.
    pub fn separator() -> Self {
        Self {
            id: "separator".to_string(),
            label: String::new(),
            icon: None,
            enabled: false,
            visible: true,
            shortcut: None,
            is_separator: true,
            submenu: None,
            action: None,
        }
    }

    /// Set the icon.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set enabled state.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set visibility.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set keyboard shortcut.
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set submenu items.
    pub fn with_submenu(mut self, submenu: Vec<MenuItem>) -> Self {
        self.submenu = Some(submenu);
        self
    }

    /// Set action callback.
    pub fn on_action<F>(mut self, action: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.action = Some(Box::new(action));
        self
    }

    /// Check if this item has a submenu.
    pub fn has_submenu(&self) -> bool {
        self.submenu.is_some()
    }
}

/// Menu position relative to trigger element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuPosition {
    /// Below the trigger element.
    Below,
    /// Above the trigger element.
    Above,
    /// To the right of the trigger element.
    Right,
    /// To the left of the trigger element.
    Left,
    /// Auto-position based on available space.
    Auto,
}

/// Menu appearance style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuStyle {
    /// Standard dropdown menu.
    Dropdown,
    /// Context menu (right-click menu).
    Context,
    /// Menu bar style.
    MenuBar,
    /// Popup menu.
    Popup,
}

/// Production-ready Menu widget with comprehensive functionality.
pub struct Menu {
    base: BaseComponent,
    // Core properties
    /// Menu items collection.
    pub items: Vec<MenuItem>,
    position: MenuPosition,
    style: MenuStyle,

    // State
    is_open: bool,
    selected_index: Option<usize>,
    submenu_open: Option<usize>,

    // Configuration
    max_height: Option<usize>,
    min_width: Option<usize>,
    close_on_select: bool,
    close_on_outside_click: bool,

    // Styling
    background_color: Color,
    text_color: Color,
    selected_color: Color,
    disabled_color: Color,
    border_color: Color,

    // Callbacks
    on_open: Option<Box<dyn Fn() + Send + Sync>>,
    on_close: Option<Box<dyn Fn() + Send + Sync>>,
    on_select: Option<Box<dyn Fn(&MenuItem) + Send + Sync>>,
}

impl std::fmt::Debug for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Menu")
            .field("items", &self.items)
            .field("position", &self.position)
            .field("style", &self.style)
            .field("is_open", &self.is_open)
            .field("selected_index", &self.selected_index)
            .field("submenu_open", &self.submenu_open)
            .field("max_height", &self.max_height)
            .field("min_width", &self.min_width)
            .field("close_on_select", &self.close_on_select)
            .field("close_on_outside_click", &self.close_on_outside_click)
            .field("background_color", &self.background_color)
            .field("text_color", &self.text_color)
            .field("selected_color", &self.selected_color)
            .field("disabled_color", &self.disabled_color)
            .field("border_color", &self.border_color)
            .field("on_open", &self.on_open.as_ref().map(|_| "Fn()"))
            .field("on_close", &self.on_close.as_ref().map(|_| "Fn()"))
            .field("on_select", &self.on_select.as_ref().map(|_| "Fn(&MenuItem)"))
            .finish()
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl Menu {
    /// Create a new menu.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("Menu"),
            items: Vec::new(),
            position: MenuPosition::Below,
            style: MenuStyle::Dropdown,
            is_open: false,
            selected_index: None,
            submenu_open: None,
            max_height: None,
            min_width: Some(120),
            close_on_select: true,
            close_on_outside_click: true,
            background_color: Color::rgba(40, 40, 40, 255),
            text_color: Color::rgba(255, 255, 255, 255),
            selected_color: Color::rgba(70, 130, 180, 255),
            disabled_color: Color::rgba(128, 128, 128, 255),
            border_color: Color::rgba(80, 80, 80, 255),
            on_open: None,
            on_close: None,
            on_select: None,
        }
    }

    /// Add a menu item.
    pub fn add_item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add multiple menu items.
    pub fn add_items(mut self, items: Vec<MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Set menu position.
    pub fn with_position(mut self, position: MenuPosition) -> Self {
        self.position = position;
        self
    }

    /// Set menu style.
    pub fn with_style(mut self, style: MenuStyle) -> Self {
        self.style = style;
        self
    }

    /// Set maximum height.
    pub fn with_max_height(mut self, height: usize) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set minimum width.
    pub fn with_min_width(mut self, width: usize) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set close on select behavior.
    pub fn close_on_select(mut self, close: bool) -> Self {
        self.close_on_select = close;
        self
    }

    /// Set close on outside click behavior.
    pub fn close_on_outside_click(mut self, close: bool) -> Self {
        self.close_on_outside_click = close;
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

    /// Set selected item color.
    pub fn with_selected_color(mut self, color: Color) -> Self {
        self.selected_color = color;
        self
    }

    /// Set disabled item color.
    pub fn with_disabled_color(mut self, color: Color) -> Self {
        self.disabled_color = color;
        self
    }

    /// Set border color.
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set open callback.
    pub fn on_open<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_open = Some(Box::new(callback));
        self
    }

    /// Set close callback.
    pub fn on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_close = Some(Box::new(callback));
        self
    }

    /// Set select callback.
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&MenuItem) + Send + Sync + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Open the menu.
    pub fn open(&mut self) {
        if !self.is_open {
            self.is_open = true;
            self.selected_index = self.find_first_enabled_item();
            if let Some(callback) = &self.on_open {
                callback();
            }
        }
    }

    /// Close the menu.
    pub fn close(&mut self) {
        if self.is_open {
            self.is_open = false;
            self.selected_index = None;
            self.submenu_open = None;
            if let Some(callback) = &self.on_close {
                callback();
            }
        }
    }

    /// Toggle menu open/close state.
    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }

    /// Check if menu is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Get currently selected item index.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    /// Find the first enabled item.
    fn find_first_enabled_item(&self) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .find(|(_, item)| item.enabled && item.visible && !item.is_separator)
            .map(|(index, _)| index)
    }

    /// Find the last enabled item.
    fn find_last_enabled_item(&self) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .rev()
            .find(|(_, item)| item.enabled && item.visible && !item.is_separator)
            .map(|(index, _)| index)
    }

    /// Move selection up.
    fn move_selection_up(&mut self) {
        if let Some(current) = self.selected_index {
            for i in (0..current).rev() {
                let item = &self.items[i];
                if item.enabled && item.visible && !item.is_separator {
                    self.selected_index = Some(i);
                    return;
                }
            }
            // Wrap to last item
            self.selected_index = self.find_last_enabled_item();
        } else {
            self.selected_index = self.find_last_enabled_item();
        }
    }

    /// Move selection down.
    fn move_selection_down(&mut self) {
        if let Some(current) = self.selected_index {
            for i in (current + 1)..self.items.len() {
                let item = &self.items[i];
                if item.enabled && item.visible && !item.is_separator {
                    self.selected_index = Some(i);
                    return;
                }
            }
            // Wrap to first item
            self.selected_index = self.find_first_enabled_item();
        } else {
            self.selected_index = self.find_first_enabled_item();
        }
    }

    /// Activate the currently selected item.
    fn activate_selected(&mut self) {
        if let Some(index) = self.selected_index {
            if let Some(item) = self.items.get(index) {
                if item.enabled && !item.is_separator {
                    // Handle submenu
                    if item.has_submenu() {
                        self.submenu_open = Some(index);
                    } else {
                        // Execute action
                        if let Some(action) = &item.action {
                            action();
                        }

                        // Trigger select callback
                        if let Some(callback) = &self.on_select {
                            callback(item);
                        }

                        // Close menu if configured
                        if self.close_on_select {
                            self.close();
                        }
                    }
                }
            }
        }
    }

    /// Handle keyboard events.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if !self.is_open {
            return false;
        }

        match event.key {
            NcKey::Up => {
                self.move_selection_up();
                true
            }
            NcKey::Down => {
                self.move_selection_down();
                true
            }
            NcKey::Enter => {
                self.activate_selected();
                true
            }
            NcKey::Esc => {
                self.close();
                true
            }
            NcKey::Right => {
                // Open submenu if available
                if let Some(index) = self.selected_index {
                    if let Some(item) = self.items.get(index) {
                        if item.has_submenu() {
                            self.submenu_open = Some(index);
                            return true;
                        }
                    }
                }
                false
            }
            NcKey::Left => {
                // Close submenu if open
                if self.submenu_open.is_some() {
                    self.submenu_open = None;
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    /// Handle mouse events.
    pub fn handle_mouse_event(&mut self, _event: &MouseEvent) -> bool {
        if !self.is_open {
            return false;
        }

        // TODO: Implement mouse interaction based on coordinates
        // This would include:
        // - Click to select items
        // - Hover to highlight items
        // - Click outside to close (if enabled)
        // - Submenu interaction

        false
    }

    /// Calculate menu dimensions.
    fn calculate_dimensions(&self) -> (usize, usize) {
        let visible_items: Vec<_> = self.items.iter().filter(|item| item.visible).collect();

        let height = if let Some(max_height) = self.max_height {
            max_height.min(visible_items.len())
        } else {
            visible_items.len()
        };

        let width = visible_items
            .iter()
            .map(|item| {
                let mut width = item.label.len();
                if let Some(icon) = &item.icon {
                    width += icon.len() + 1; // Icon + space
                }
                if let Some(shortcut) = &item.shortcut {
                    width += shortcut.len() + 2; // Space + shortcut
                }
                if item.has_submenu() {
                    width += 2; // Arrow indicator
                }
                width
            })
            .max()
            .unwrap_or(0);

        let min_width = self.min_width.unwrap_or(0);
        (width.max(min_width), height)
    }

    /// Render a menu item.
    fn render_item(&self, item: &MenuItem, _index: usize, is_selected: bool) -> VirtualNode {
        if item.is_separator {
            return VirtualNode::element("div")
                .attr("class", "menu-separator")
                .attr("style", format!(
                    "border-top: 1px solid {}; margin: 2px 0;",
                    self.border_color.to_hex()
                ));
        }

        let mut item_style = format!(
            "padding: 4px 8px; cursor: {}; color: {};",
            if item.enabled { "pointer" } else { "default" },
            if item.enabled {
                if is_selected {
                    self.selected_color.to_hex()
                } else {
                    self.text_color.to_hex()
                }
            } else {
                self.disabled_color.to_hex()
            }
        );

        if is_selected && item.enabled {
            item_style.push_str(&format!(
                " background-color: {};",
                self.selected_color.to_hex()
            ));
        }

        let mut content = VirtualNode::element("div")
            .attr("class", "menu-item")
            .attr("style", item_style)
            .attr("data-item-id", &item.id);

        // Add icon if present
        if let Some(icon) = &item.icon {
            content = content.child(
                VirtualNode::element("span")
                    .attr("class", "menu-item-icon")
                    .attr("style", "margin-right: 8px;")
                    .child(VirtualNode::text(icon))
            );
        }

        // Add label
        content = content.child(
            VirtualNode::element("span")
                .attr("class", "menu-item-label")
                .child(VirtualNode::text(&item.label))
        );

        // Add shortcut if present
        if let Some(shortcut) = &item.shortcut {
            content = content.child(
                VirtualNode::element("span")
                    .attr("class", "menu-item-shortcut")
                    .attr("style", "margin-left: auto; opacity: 0.7;")
                    .child(VirtualNode::text(shortcut))
            );
        }

        // Add submenu indicator
        if item.has_submenu() {
            content = content.child(
                VirtualNode::element("span")
                    .attr("class", "menu-item-arrow")
                    .attr("style", "margin-left: auto;")
                    .child(VirtualNode::text("▶"))
            );
        }

        content
    }
}

#[async_trait]
impl Component for Menu {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        if !self.is_open {
            return Ok(VirtualNode::empty());
        }

        let (width, height) = self.calculate_dimensions();

        let menu_style = format!(
            "position: absolute; background-color: {}; border: 1px solid {}; \
             border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.3); \
             min-width: {}px; max-height: {}px; overflow-y: auto; z-index: 1000;",
            self.background_color.to_hex(),
            self.border_color.to_hex(),
            width,
            height * 24 // Approximate item height
        );

        let mut menu_element = VirtualNode::element("div")
            .attr("class", format!("menu menu-{:?}", self.style).to_lowercase())
            .attr("style", menu_style);

        // Render visible items
        for (index, item) in self.items.iter().enumerate() {
            if item.visible {
                let is_selected = self.selected_index == Some(index);
                let item_node = self.render_item(item, index, is_selected);
                menu_element = menu_element.child(item_node);
            }
        }

        Ok(menu_element)
    }
}

impl Widget for Menu {
    fn widget_type(&self) -> &'static str {
        "Menu"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::RenderContext;
    use crate::style::Theme;
    use crate::event::types::KeyModifiers;

    #[test]
    fn test_menu_creation() {
        let menu = Menu::new();
        assert!(!menu.is_open());
        assert_eq!(menu.selected_index(), None);
        assert_eq!(menu.position, MenuPosition::Below);
        assert_eq!(menu.style, MenuStyle::Dropdown);
        assert!(menu.close_on_select);
        assert!(menu.close_on_outside_click);
        assert_eq!(menu.items.len(), 0);
    }

    #[test]
    fn test_menu_item_creation() {
        let item = MenuItem::new("test", "Test Item")
            .with_icon("🔧")
            .with_shortcut("Ctrl+T")
            .enabled(true)
            .visible(true);

        assert_eq!(item.id, "test");
        assert_eq!(item.label, "Test Item");
        assert_eq!(item.icon, Some("🔧".to_string()));
        assert_eq!(item.shortcut, Some("Ctrl+T".to_string()));
        assert!(item.enabled);
        assert!(item.visible);
        assert!(!item.is_separator);
        assert!(!item.has_submenu());
    }

    #[test]
    fn test_menu_item_separator() {
        let separator = MenuItem::separator();
        assert_eq!(separator.id, "separator");
        assert!(separator.label.is_empty());
        assert!(!separator.enabled);
        assert!(separator.visible);
        assert!(separator.is_separator);
    }

    #[test]
    fn test_menu_item_submenu() {
        let submenu_items = vec![
            MenuItem::new("sub1", "Submenu Item 1"),
            MenuItem::new("sub2", "Submenu Item 2"),
        ];

        let item = MenuItem::new("parent", "Parent Item")
            .with_submenu(submenu_items);

        assert!(item.has_submenu());
        assert_eq!(item.submenu.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_menu_configuration() {
        let menu = Menu::new()
            .with_position(MenuPosition::Above)
            .with_style(MenuStyle::Context)
            .with_max_height(200)
            .with_min_width(150)
            .close_on_select(false)
            .close_on_outside_click(false)
            .with_background_color(Color::rgba(50, 50, 50, 255))
            .with_text_color(Color::rgba(200, 200, 200, 255));

        assert_eq!(menu.position, MenuPosition::Above);
        assert_eq!(menu.style, MenuStyle::Context);
        assert_eq!(menu.max_height, Some(200));
        assert_eq!(menu.min_width, Some(150));
        assert!(!menu.close_on_select);
        assert!(!menu.close_on_outside_click);
        assert_eq!(menu.background_color, Color::rgba(50, 50, 50, 255));
        assert_eq!(menu.text_color, Color::rgba(200, 200, 200, 255));
    }

    #[test]
    fn test_menu_items() {
        let items = vec![
            MenuItem::new("file", "File"),
            MenuItem::new("edit", "Edit"),
            MenuItem::separator(),
            MenuItem::new("help", "Help"),
        ];

        let menu = Menu::new().add_items(items);
        assert_eq!(menu.items.len(), 4);
        assert_eq!(menu.items[0].label, "File");
        assert_eq!(menu.items[1].label, "Edit");
        assert!(menu.items[2].is_separator);
        assert_eq!(menu.items[3].label, "Help");
    }

    #[test]
    fn test_menu_open_close() {
        let mut menu = Menu::new()
            .add_item(MenuItem::new("test", "Test"));

        // Test opening
        assert!(!menu.is_open());
        menu.open();
        assert!(menu.is_open());
        assert_eq!(menu.selected_index(), Some(0)); // First enabled item

        // Test closing
        menu.close();
        assert!(!menu.is_open());
        assert_eq!(menu.selected_index(), None);

        // Test toggle
        menu.toggle();
        assert!(menu.is_open());
        menu.toggle();
        assert!(!menu.is_open());
    }

    #[test]
    fn test_menu_navigation() {
        let items = vec![
            MenuItem::new("item1", "Item 1"),
            MenuItem::separator(),
            MenuItem::new("item2", "Item 2").enabled(false),
            MenuItem::new("item3", "Item 3"),
        ];

        let mut menu = Menu::new().add_items(items);
        menu.open();

        // Should start at first enabled item (index 0)
        assert_eq!(menu.selected_index(), Some(0));

        // Move down should skip separator and disabled item to index 3
        menu.move_selection_down();
        assert_eq!(menu.selected_index(), Some(3));

        // Move down again should wrap to first item
        menu.move_selection_down();
        assert_eq!(menu.selected_index(), Some(0));

        // Move up should go to last enabled item
        menu.move_selection_up();
        assert_eq!(menu.selected_index(), Some(3));
    }

    #[test]
    fn test_menu_key_events() {
        let mut menu = Menu::new()
            .add_item(MenuItem::new("test", "Test"));

        menu.open();

        // Test arrow keys
        let down_event = KeyEvent {
            key: NcKey::Down,
            modifiers: KeyModifiers::empty(),
            bubbles: true,
            prevent_default: false,
        };
        assert!(menu.handle_key_event(&down_event));

        let up_event = KeyEvent {
            key: NcKey::Up,
            modifiers: KeyModifiers::empty(),
            bubbles: true,
            prevent_default: false,
        };
        assert!(menu.handle_key_event(&up_event));

        // Test escape key
        let escape_event = KeyEvent {
            key: NcKey::Esc,
            modifiers: KeyModifiers::empty(),
            bubbles: true,
            prevent_default: false,
        };
        assert!(menu.handle_key_event(&escape_event));
        assert!(!menu.is_open());
    }

    #[test]
    fn test_menu_callbacks() {
        use std::sync::{Arc, Mutex};

        let opened = Arc::new(Mutex::new(false));
        let closed = Arc::new(Mutex::new(false));
        let selected = Arc::new(Mutex::new(false));

        let opened_clone = opened.clone();
        let closed_clone = closed.clone();
        let selected_clone = selected.clone();

        let mut menu = Menu::new()
            .add_item(MenuItem::new("test", "Test"))
            .on_open(move || {
                *opened_clone.lock().unwrap() = true;
            })
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            })
            .on_select(move |_| {
                *selected_clone.lock().unwrap() = true;
            });

        // Test open callback
        menu.open();
        assert!(*opened.lock().unwrap());

        // Test select callback
        menu.activate_selected();
        assert!(*selected.lock().unwrap());
        assert!(*closed.lock().unwrap()); // Should close after selection
    }

    #[tokio::test]
    async fn test_menu_rendering() {
        let menu = Menu::new()
            .add_item(MenuItem::new("file", "File").with_icon("📁"))
            .add_item(MenuItem::separator())
            .add_item(MenuItem::new("exit", "Exit").with_shortcut("Ctrl+Q"));

        let theme = Theme::default();
        let context = RenderContext::new(&theme);

        // Test closed menu rendering
        let result = menu.render(&context).await;
        assert!(result.is_ok());
        let vnode = result.unwrap();
        // Closed menu should render as empty
        assert_eq!(vnode, VirtualNode::empty());

        // Test open menu rendering
        let mut open_menu = menu;
        open_menu.is_open = true;
        open_menu.selected_index = Some(0);

        let result = open_menu.render(&context).await;
        assert!(result.is_ok());
        let vnode = result.unwrap();

        // Should render as a div with menu class
        if let VirtualNode::Element(element) = &vnode {
            assert_eq!(element.tag, "div");
            assert!(element.attributes.get("class").unwrap().contains("menu"));
            assert!(element.attributes.get("style").unwrap().contains("position: absolute"));
        } else {
            panic!("Expected element node");
        }
    }
}
