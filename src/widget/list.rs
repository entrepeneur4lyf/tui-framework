//! List widget implementation for displaying scrollable lists of items.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::event::types::{KeyEvent, MouseButton, MouseEvent, MouseEventType, NcKey};
use crate::render::{RenderContext, VirtualNode};
use crate::style::properties::Style;
use crate::widget::Widget;
use async_trait::async_trait;
use std::sync::Arc;

/// Selection mode for the list widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// No selection allowed
    None,
    /// Single item selection
    Single,
    /// Multiple item selection
    Multiple,
}

/// List item data structure.
#[derive(Debug, Clone)]
pub struct ListItem {
    /// Unique identifier for the item
    pub id: String,
    /// Display text for the item
    pub text: String,
    /// Whether the item is enabled
    pub enabled: bool,
    /// Custom data associated with the item
    pub data: Option<String>,
}

impl ListItem {
    /// Create a new list item.
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            enabled: true,
            data: None,
        }
    }

    /// Create a new list item with custom data.
    pub fn with_data(
        id: impl Into<String>,
        text: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            enabled: true,
            data: Some(data.into()),
        }
    }

    /// Set whether the item is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// A scrollable list widget for displaying and selecting items.
pub struct List {
    base: BaseComponent,
    items: Vec<ListItem>,
    selected_indices: Vec<usize>,
    focused_index: Option<usize>,
    selection_mode: SelectionMode,
    scroll_offset: usize,
    visible_items: usize,
    style: Style,
    on_selection_changed: Option<Arc<dyn Fn(&[usize]) + Send + Sync>>,
    on_item_activated: Option<Arc<dyn Fn(usize, &ListItem) + Send + Sync>>,
}

impl List {
    /// Create a new list widget.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("List"),
            items: Vec::new(),
            selected_indices: Vec::new(),
            focused_index: None,
            selection_mode: SelectionMode::Single,
            scroll_offset: 0,
            visible_items: 10, // Default visible items
            style: Style::default(),
            on_selection_changed: None,
            on_item_activated: None,
        }
    }

    /// Set the selection mode.
    pub fn with_selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Set the number of visible items.
    pub fn with_visible_items(mut self, count: usize) -> Self {
        self.visible_items = count.max(1);
        self
    }

    /// Set the list style.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the selection changed callback.
    pub fn on_selection_changed<F>(mut self, handler: F) -> Self
    where
        F: Fn(&[usize]) + Send + Sync + 'static,
    {
        self.on_selection_changed = Some(Arc::new(handler));
        self
    }

    /// Set the item activated callback (double-click or Enter).
    pub fn on_item_activated<F>(mut self, handler: F) -> Self
    where
        F: Fn(usize, &ListItem) + Send + Sync + 'static,
    {
        self.on_item_activated = Some(Arc::new(handler));
        self
    }

    /// Add an item to the list.
    pub fn add_item(&mut self, item: ListItem) {
        self.items.push(item);
        self.update_focus_bounds();
    }

    /// Insert an item at a specific index.
    pub fn insert_item(&mut self, index: usize, item: ListItem) {
        if index <= self.items.len() {
            self.items.insert(index, item);
            self.update_selection_after_insert(index);
            self.update_focus_bounds();
        }
    }

    /// Remove an item by index.
    pub fn remove_item(&mut self, index: usize) -> Option<ListItem> {
        if index < self.items.len() {
            let item = self.items.remove(index);
            self.update_selection_after_remove(index);
            self.update_focus_bounds();
            Some(item)
        } else {
            None
        }
    }

    /// Clear all items.
    pub fn clear(&mut self) {
        self.items.clear();
        self.selected_indices.clear();
        self.focused_index = None;
        self.scroll_offset = 0;
    }

    /// Get all items.
    pub fn items(&self) -> &[ListItem] {
        &self.items
    }

    /// Get selected indices.
    pub fn selected_indices(&self) -> &[usize] {
        &self.selected_indices
    }

    /// Get the focused item index.
    pub fn focused_index(&self) -> Option<usize> {
        self.focused_index
    }

    /// Set the focused item index.
    pub fn set_focused_index(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            if idx < self.items.len() {
                self.focused_index = Some(idx);
                self.ensure_focused_visible();
            }
        } else {
            self.focused_index = None;
        }
    }

    /// Select an item by index.
    pub fn select_item(&mut self, index: usize) {
        if index >= self.items.len() || self.selection_mode == SelectionMode::None {
            return;
        }

        match self.selection_mode {
            SelectionMode::None => {}
            SelectionMode::Single => {
                self.selected_indices.clear();
                self.selected_indices.push(index);
            }
            SelectionMode::Multiple => {
                if let Some(pos) = self.selected_indices.iter().position(|&i| i == index) {
                    self.selected_indices.remove(pos);
                } else {
                    self.selected_indices.push(index);
                    self.selected_indices.sort_unstable();
                }
            }
        }

        if let Some(ref handler) = self.on_selection_changed {
            handler(&self.selected_indices);
        }
    }

    /// Clear all selections.
    pub fn clear_selection(&mut self) {
        if !self.selected_indices.is_empty() {
            self.selected_indices.clear();
            if let Some(ref handler) = self.on_selection_changed {
                handler(&self.selected_indices);
            }
        }
    }

    /// Check if an item is selected.
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected_indices.contains(&index)
    }

    /// Get the scroll offset.
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Set the scroll offset.
    pub fn set_scroll_offset(&mut self, offset: usize) {
        let max_offset = self.items.len().saturating_sub(self.visible_items);
        self.scroll_offset = offset.min(max_offset);
    }

    /// Scroll up by one item.
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Scroll down by one item.
    pub fn scroll_down(&mut self) {
        let max_offset = self.items.len().saturating_sub(self.visible_items);
        if self.scroll_offset < max_offset {
            self.scroll_offset += 1;
        }
    }

    /// Move focus up.
    pub fn move_focus_up(&mut self) {
        if let Some(current) = self.focused_index {
            if current > 0 {
                self.focused_index = Some(current - 1);
                self.ensure_focused_visible();
            }
        } else if !self.items.is_empty() {
            self.focused_index = Some(self.items.len() - 1);
            self.ensure_focused_visible();
        }
    }

    /// Move focus down.
    pub fn move_focus_down(&mut self) {
        if let Some(current) = self.focused_index {
            if current + 1 < self.items.len() {
                self.focused_index = Some(current + 1);
                self.ensure_focused_visible();
            }
        } else if !self.items.is_empty() {
            self.focused_index = Some(0);
            self.ensure_focused_visible();
        }
    }

    /// Activate the currently focused item.
    pub fn activate_focused_item(&self) {
        if let Some(index) = self.focused_index {
            if index < self.items.len() {
                if let Some(ref handler) = self.on_item_activated {
                    handler(index, &self.items[index]);
                }
            }
        }
    }

    /// Ensure the focused item is visible in the viewport.
    fn ensure_focused_visible(&mut self) {
        if let Some(focused) = self.focused_index {
            if focused < self.scroll_offset {
                self.scroll_offset = focused;
            } else if focused >= self.scroll_offset + self.visible_items {
                self.scroll_offset = focused.saturating_sub(self.visible_items - 1);
            }
        }
    }

    /// Update focus bounds after items change.
    fn update_focus_bounds(&mut self) {
        if self.items.is_empty() {
            self.focused_index = None;
            self.scroll_offset = 0;
        } else if let Some(focused) = self.focused_index {
            if focused >= self.items.len() {
                self.focused_index = Some(self.items.len() - 1);
            }
        }

        let max_offset = self.items.len().saturating_sub(self.visible_items);
        self.scroll_offset = self.scroll_offset.min(max_offset);
    }

    /// Update selection indices after item insertion.
    fn update_selection_after_insert(&mut self, inserted_index: usize) {
        for selected in &mut self.selected_indices {
            if *selected >= inserted_index {
                *selected += 1;
            }
        }

        if let Some(ref mut focused) = self.focused_index {
            if *focused >= inserted_index {
                *focused += 1;
            }
        }
    }

    /// Update selection indices after item removal.
    fn update_selection_after_remove(&mut self, removed_index: usize) {
        self.selected_indices
            .retain(|&index| index != removed_index);

        for selected in &mut self.selected_indices {
            if *selected > removed_index {
                *selected -= 1;
            }
        }

        if let Some(ref mut focused) = self.focused_index {
            if *focused == removed_index {
                if removed_index < self.items.len() {
                    // Keep same index if there are items after
                    // (the next item will move to this position)
                } else if removed_index > 0 {
                    // Move to previous item if we removed the last item
                    *focused = removed_index - 1;
                } else {
                    // No items left
                    self.focused_index = None;
                }
            } else if *focused > removed_index {
                *focused -= 1;
            }
        }
    }

    /// Handle a key event.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        match event.key {
            NcKey::Up => {
                self.move_focus_up();
                true
            }
            NcKey::Down => {
                self.move_focus_down();
                true
            }
            NcKey::Home => {
                if !self.items.is_empty() {
                    self.focused_index = Some(0);
                    self.ensure_focused_visible();
                }
                true
            }
            NcKey::End => {
                if !self.items.is_empty() {
                    self.focused_index = Some(self.items.len() - 1);
                    self.ensure_focused_visible();
                }
                true
            }
            NcKey::PgUp => {
                if let Some(current) = self.focused_index {
                    let new_index = current.saturating_sub(self.visible_items);
                    self.focused_index = Some(new_index);
                    self.ensure_focused_visible();
                } else if !self.items.is_empty() {
                    self.focused_index = Some(0);
                    self.ensure_focused_visible();
                }
                true
            }
            NcKey::PgDown => {
                if let Some(current) = self.focused_index {
                    let new_index = (current + self.visible_items).min(self.items.len() - 1);
                    self.focused_index = Some(new_index);
                    self.ensure_focused_visible();
                } else if !self.items.is_empty() {
                    self.focused_index = Some(self.items.len() - 1);
                    self.ensure_focused_visible();
                }
                true
            }
            NcKey::Space => {
                if let Some(focused) = self.focused_index {
                    self.select_item(focused);
                }
                true
            }
            NcKey::Enter => {
                if let Some(focused) = self.focused_index {
                    self.select_item(focused);
                    self.activate_focused_item();
                }
                true
            }
            _ => false,
        }
    }

    /// Handle a mouse event.
    pub fn handle_mouse_event(&mut self, event: &MouseEvent) -> bool {
        match event.event_type {
            MouseEventType::Press => {
                if event.button == MouseButton::Left {
                    // Calculate which item was clicked based on mouse position
                    // This is a simplified calculation - in a real implementation,
                    // you'd need to consider the actual rendered positions
                    let clicked_index = self.scroll_offset + event.y as usize;

                    if clicked_index < self.items.len() {
                        self.focused_index = Some(clicked_index);
                        self.select_item(clicked_index);
                        return true;
                    }
                }
                false
            }
            MouseEventType::Release => {
                // Handle double-click logic here if needed
                // For now, we'll just handle single clicks on press
                false
            }
            MouseEventType::Scroll => {
                // Simple scroll handling - scroll up/down one item at a time
                // In a real implementation, you might want to handle scroll delta
                if event.y > 0 {
                    self.scroll_down();
                } else {
                    self.scroll_up();
                }
                true
            }
            _ => false,
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Component for List {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "List"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let mut list_node = VirtualNode::element("list")
            .attr(
                "selection_mode",
                format!("{:?}", self.selection_mode).to_lowercase(),
            )
            .attr("scroll_offset", self.scroll_offset.to_string())
            .attr("visible_items", self.visible_items.to_string())
            .attr("total_items", self.items.len().to_string());

        // Add focused index if present
        if let Some(focused) = self.focused_index {
            list_node = list_node.attr("focused_index", focused.to_string());
        }

        // Add selected indices
        if !self.selected_indices.is_empty() {
            let selected_str = self
                .selected_indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",");
            list_node = list_node.attr("selected_indices", selected_str);
        }

        // Render visible items
        let end_index = (self.scroll_offset + self.visible_items).min(self.items.len());
        for (visible_index, item_index) in (self.scroll_offset..end_index).enumerate() {
            let item = &self.items[item_index];
            let is_selected = self.is_selected(item_index);
            let is_focused = self.focused_index == Some(item_index);

            let mut item_node = VirtualNode::element("list_item")
                .attr("id", &item.id)
                .attr("index", item_index.to_string())
                .attr("visible_index", visible_index.to_string())
                .attr("enabled", item.enabled.to_string())
                .attr("selected", is_selected.to_string())
                .attr("focused", is_focused.to_string())
                .child(VirtualNode::text(&item.text));

            // Add custom data if present
            if let Some(ref data) = item.data {
                item_node = item_node.attr("data", data);
            }

            // Add state classes for styling
            let mut classes = Vec::new();
            if is_selected {
                classes.push("selected");
            }
            if is_focused {
                classes.push("focused");
            }
            if !item.enabled {
                classes.push("disabled");
            }

            if !classes.is_empty() {
                item_node = item_node.attr("class", classes.join(" "));
            }

            list_node = list_node.child(item_node);
        }

        Ok(list_node)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[async_trait]
impl Widget for List {
    fn widget_type(&self) -> &'static str {
        "list"
    }

    async fn handle_widget_event(&mut self, event: &str) -> Result<()> {
        // Handle custom widget events
        match event {
            "refresh" => {
                // Refresh the list display
                self.update_focus_bounds();
            }
            "clear_selection" => {
                self.clear_selection();
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::types::{KeyEvent, MouseButton, MouseEvent, MouseEventType};

    #[test]
    fn test_list_creation() {
        let list = List::new();
        assert_eq!(list.items().len(), 0);
        assert_eq!(list.selected_indices().len(), 0);
        assert_eq!(list.focused_index(), None);
        assert_eq!(list.selection_mode, SelectionMode::Single);
        assert_eq!(list.scroll_offset(), 0);
        assert_eq!(list.visible_items, 10);
    }

    #[test]
    fn test_list_item_creation() {
        let item = ListItem::new("1", "Test Item");
        assert_eq!(item.id, "1");
        assert_eq!(item.text, "Test Item");
        assert!(item.enabled);
        assert!(item.data.is_none());

        let item_with_data = ListItem::with_data("2", "Test Item 2", "custom_data");
        assert_eq!(item_with_data.id, "2");
        assert_eq!(item_with_data.text, "Test Item 2");
        assert_eq!(item_with_data.data, Some("custom_data".to_string()));

        let disabled_item = ListItem::new("3", "Disabled").enabled(false);
        assert!(!disabled_item.enabled);
    }

    #[test]
    fn test_list_configuration() {
        let list = List::new()
            .with_selection_mode(SelectionMode::Multiple)
            .with_visible_items(5);

        assert_eq!(list.selection_mode, SelectionMode::Multiple);
        assert_eq!(list.visible_items, 5);
    }

    #[test]
    fn test_add_and_remove_items() {
        let mut list = List::new();

        // Add items
        list.add_item(ListItem::new("1", "Item 1"));
        list.add_item(ListItem::new("2", "Item 2"));
        list.add_item(ListItem::new("3", "Item 3"));

        assert_eq!(list.items().len(), 3);
        assert_eq!(list.items()[0].text, "Item 1");
        assert_eq!(list.items()[1].text, "Item 2");
        assert_eq!(list.items()[2].text, "Item 3");

        // Insert item
        list.insert_item(1, ListItem::new("1.5", "Item 1.5"));
        assert_eq!(list.items().len(), 4);
        assert_eq!(list.items()[1].text, "Item 1.5");

        // Remove item
        let removed = list.remove_item(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().text, "Item 1.5");
        assert_eq!(list.items().len(), 3);

        // Clear all items
        list.clear();
        assert_eq!(list.items().len(), 0);
        assert_eq!(list.selected_indices().len(), 0);
        assert_eq!(list.focused_index(), None);
    }

    #[test]
    fn test_single_selection() {
        let mut list = List::new();
        list.add_item(ListItem::new("1", "Item 1"));
        list.add_item(ListItem::new("2", "Item 2"));
        list.add_item(ListItem::new("3", "Item 3"));

        // Select first item
        list.select_item(0);
        assert_eq!(list.selected_indices(), &[0]);
        assert!(list.is_selected(0));
        assert!(!list.is_selected(1));

        // Select second item (should replace first)
        list.select_item(1);
        assert_eq!(list.selected_indices(), &[1]);
        assert!(!list.is_selected(0));
        assert!(list.is_selected(1));

        // Clear selection
        list.clear_selection();
        assert_eq!(list.selected_indices().len(), 0);
        assert!(!list.is_selected(0));
        assert!(!list.is_selected(1));
    }

    #[test]
    fn test_multiple_selection() {
        let mut list = List::new().with_selection_mode(SelectionMode::Multiple);
        list.add_item(ListItem::new("1", "Item 1"));
        list.add_item(ListItem::new("2", "Item 2"));
        list.add_item(ListItem::new("3", "Item 3"));

        // Select multiple items
        list.select_item(0);
        list.select_item(2);
        assert_eq!(list.selected_indices(), &[0, 2]);
        assert!(list.is_selected(0));
        assert!(!list.is_selected(1));
        assert!(list.is_selected(2));

        // Toggle selection (deselect item 0)
        list.select_item(0);
        assert_eq!(list.selected_indices(), &[2]);
        assert!(!list.is_selected(0));
        assert!(list.is_selected(2));
    }

    #[test]
    fn test_no_selection_mode() {
        let mut list = List::new().with_selection_mode(SelectionMode::None);
        list.add_item(ListItem::new("1", "Item 1"));
        list.add_item(ListItem::new("2", "Item 2"));

        // Attempt to select items (should be ignored)
        list.select_item(0);
        list.select_item(1);
        assert_eq!(list.selected_indices().len(), 0);
        assert!(!list.is_selected(0));
        assert!(!list.is_selected(1));
    }

    #[test]
    fn test_focus_management() {
        let mut list = List::new();
        list.add_item(ListItem::new("1", "Item 1"));
        list.add_item(ListItem::new("2", "Item 2"));
        list.add_item(ListItem::new("3", "Item 3"));

        // Set focus
        list.set_focused_index(Some(1));
        assert_eq!(list.focused_index(), Some(1));

        // Move focus up
        list.move_focus_up();
        assert_eq!(list.focused_index(), Some(0));

        // Move focus up again (should stay at 0)
        list.move_focus_up();
        assert_eq!(list.focused_index(), Some(0));

        // Move focus down
        list.move_focus_down();
        assert_eq!(list.focused_index(), Some(1));

        // Move to end
        list.move_focus_down();
        list.move_focus_down();
        assert_eq!(list.focused_index(), Some(2));

        // Move focus down again (should stay at 2)
        list.move_focus_down();
        assert_eq!(list.focused_index(), Some(2));
    }

    #[test]
    fn test_scrolling() {
        let mut list = List::new().with_visible_items(3);

        // Add more items than visible
        for i in 0..10 {
            list.add_item(ListItem::new(i.to_string(), format!("Item {}", i)));
        }

        // Test scrolling
        assert_eq!(list.scroll_offset(), 0);

        list.scroll_down();
        assert_eq!(list.scroll_offset(), 1);

        list.scroll_up();
        assert_eq!(list.scroll_offset(), 0);

        // Scroll to maximum
        list.set_scroll_offset(100); // Should be clamped
        assert_eq!(list.scroll_offset(), 7); // 10 items - 3 visible = 7 max offset
    }

    #[test]
    fn test_focus_with_scrolling() {
        let mut list = List::new().with_visible_items(3);

        // Add items
        for i in 0..10 {
            list.add_item(ListItem::new(i.to_string(), format!("Item {}", i)));
        }

        // Focus on item beyond visible range
        list.set_focused_index(Some(8));
        assert_eq!(list.focused_index(), Some(8));
        assert_eq!(list.scroll_offset(), 6); // Should auto-scroll to make item visible

        // Focus on item before visible range
        list.set_focused_index(Some(2));
        assert_eq!(list.focused_index(), Some(2));
        assert_eq!(list.scroll_offset(), 2); // Should auto-scroll
    }

    #[test]
    fn test_keyboard_navigation() {
        let mut list = List::new();
        for i in 0..5 {
            list.add_item(ListItem::new(i.to_string(), format!("Item {}", i)));
        }

        // Test arrow key navigation
        let up_event = KeyEvent::new(NcKey::Up);
        let down_event = KeyEvent::new(NcKey::Down);
        let home_event = KeyEvent::new(NcKey::Home);
        let end_event = KeyEvent::new(NcKey::End);

        // Start with no focus, down should focus first item
        assert!(list.handle_key_event(&down_event));
        assert_eq!(list.focused_index(), Some(0));

        // Move down
        assert!(list.handle_key_event(&down_event));
        assert_eq!(list.focused_index(), Some(1));

        // Move up
        assert!(list.handle_key_event(&up_event));
        assert_eq!(list.focused_index(), Some(0));

        // Home key
        list.set_focused_index(Some(3));
        assert!(list.handle_key_event(&home_event));
        assert_eq!(list.focused_index(), Some(0));

        // End key
        assert!(list.handle_key_event(&end_event));
        assert_eq!(list.focused_index(), Some(4));
    }

    #[test]
    fn test_page_navigation() {
        let mut list = List::new().with_visible_items(3);
        for i in 0..10 {
            list.add_item(ListItem::new(i.to_string(), format!("Item {}", i)));
        }

        let pgup_event = KeyEvent::new(NcKey::PgUp);
        let pgdown_event = KeyEvent::new(NcKey::PgDown);

        // Start at middle
        list.set_focused_index(Some(5));

        // Page up
        assert!(list.handle_key_event(&pgup_event));
        assert_eq!(list.focused_index(), Some(2)); // 5 - 3 = 2

        // Page down
        assert!(list.handle_key_event(&pgdown_event));
        assert_eq!(list.focused_index(), Some(5)); // 2 + 3 = 5

        // Page down near end
        list.set_focused_index(Some(8));
        assert!(list.handle_key_event(&pgdown_event));
        assert_eq!(list.focused_index(), Some(9)); // Clamped to last item
    }

    #[test]
    fn test_selection_keys() {
        let mut list = List::new();
        for i in 0..3 {
            list.add_item(ListItem::new(i.to_string(), format!("Item {}", i)));
        }

        let space_event = KeyEvent::new(NcKey::Space);
        let enter_event = KeyEvent::new(NcKey::Enter);

        // Focus and select with space
        list.set_focused_index(Some(1));
        assert!(list.handle_key_event(&space_event));
        assert!(list.is_selected(1));

        // Select and activate with enter
        list.set_focused_index(Some(2));
        assert!(list.handle_key_event(&enter_event));
        assert!(list.is_selected(2));
    }

    #[test]
    fn test_mouse_events() {
        let mut list = List::new().with_visible_items(3); // Make scrolling possible
        for i in 0..5 {
            list.add_item(ListItem::new(i.to_string(), format!("Item {}", i)));
        }

        // Test mouse click
        let click_event = MouseEvent::new(MouseButton::Left, MouseEventType::Press, 10, 2);
        assert!(list.handle_mouse_event(&click_event));
        assert_eq!(list.focused_index(), Some(2));
        assert!(list.is_selected(2));

        // Test scroll down (positive y) - now max_offset = 5-3 = 2, so scrolling is possible
        let scroll_event = MouseEvent::new(MouseButton::Middle, MouseEventType::Scroll, 0, 1);
        assert!(list.handle_mouse_event(&scroll_event));
        assert_eq!(list.scroll_offset(), 1);

        // Test scroll up (y = 0 means scroll up)
        let scroll_up_event = MouseEvent::new(MouseButton::Middle, MouseEventType::Scroll, 0, 0);
        assert!(list.handle_mouse_event(&scroll_up_event));
        assert_eq!(list.scroll_offset(), 0);
    }

    #[test]
    fn test_selection_after_item_changes() {
        let mut list = List::new().with_selection_mode(SelectionMode::Multiple);
        for i in 0..5 {
            list.add_item(ListItem::new(i.to_string(), format!("Item {}", i)));
        }

        // Select items 1 and 3
        list.select_item(1);
        list.select_item(3);
        list.set_focused_index(Some(2));

        // Insert item at index 1 (should shift selections)
        list.insert_item(1, ListItem::new("new", "New Item"));
        assert_eq!(list.selected_indices(), &[2, 4]); // Shifted by 1
        assert_eq!(list.focused_index(), Some(3)); // Focused index also shifted

        // Remove item at index 2 (was selected)
        list.remove_item(2);
        assert_eq!(list.selected_indices(), &[3]); // Item 2 removed, item 4 becomes 3
        assert_eq!(list.focused_index(), Some(2)); // Focus adjusted
    }

    #[test]
    fn test_edge_cases() {
        let mut list = List::new();

        // Test operations on empty list - Up key should still return true but do nothing
        assert!(list.handle_key_event(&KeyEvent::new(NcKey::Up)));
        list.select_item(0); // Should not crash
        assert_eq!(list.selected_indices().len(), 0);

        // Test invalid indices
        list.add_item(ListItem::new("1", "Item 1"));
        list.select_item(10); // Out of bounds
        assert_eq!(list.selected_indices().len(), 0);

        list.set_focused_index(Some(10)); // Out of bounds - should be ignored
        assert_eq!(list.focused_index(), None);

        // Test remove from empty list
        list.clear();
        assert!(list.remove_item(0).is_none());
    }

    #[tokio::test]
    async fn test_rendering() {
        let mut list = List::new().with_visible_items(2);
        list.add_item(ListItem::new("1", "Item 1"));
        list.add_item(ListItem::new("2", "Item 2"));
        list.add_item(ListItem::new("3", "Item 3"));

        list.select_item(0);
        list.set_focused_index(Some(1));

        let theme = crate::style::theme::Theme::default();
        let context = crate::render::RenderContext::new(&theme);
        let node = list.render(&context).await.unwrap();

        // Verify the rendered structure
        assert_eq!(node.tag(), Some("list"));

        // Check attributes
        if let crate::render::VirtualNode::Element(element) = node {
            assert_eq!(
                element.attributes.get("selection_mode"),
                Some(&"single".to_string())
            );
            assert_eq!(
                element.attributes.get("total_items"),
                Some(&"3".to_string())
            );
            assert_eq!(
                element.attributes.get("visible_items"),
                Some(&"2".to_string())
            );
            assert_eq!(
                element.attributes.get("focused_index"),
                Some(&"1".to_string())
            );
            assert_eq!(
                element.attributes.get("selected_indices"),
                Some(&"0".to_string())
            );

            // Should have 2 visible items (items 0 and 1)
            assert_eq!(element.children.len(), 2);
        }
    }

    #[tokio::test]
    async fn test_widget_events() {
        let mut list = List::new();
        list.add_item(ListItem::new("1", "Item 1"));
        list.select_item(0);

        // Test refresh event
        list.handle_widget_event("refresh").await.unwrap();

        // Test clear selection event
        list.handle_widget_event("clear_selection").await.unwrap();
        assert_eq!(list.selected_indices().len(), 0);

        // Test unknown event (should not crash)
        list.handle_widget_event("unknown").await.unwrap();
    }
}
