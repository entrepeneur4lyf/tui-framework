//! Dirty tracking system for efficient rendering.
//!
//! This module provides dirty tracking capabilities to minimize unnecessary
//! re-renders and layout computations. Only components that have actually
//! changed will be re-rendered.

use crate::component::ComponentId;
use crate::layout::Size;
use crate::render::vdom::VirtualNode;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// Tracks which components need to be re-rendered.
#[derive(Debug, Clone)]
pub struct DirtyTracker {
    /// Components that need re-rendering
    dirty_components: Arc<RwLock<HashSet<ComponentId>>>,
    /// Previous virtual DOM tree for diffing
    previous_vdom: Arc<RwLock<Option<VirtualNode>>>,
    /// Previous layout results
    previous_layouts: Arc<RwLock<HashMap<ComponentId, Size>>>,
    /// Previous viewport size
    previous_viewport: Arc<RwLock<Option<Size>>>,
}

impl Default for DirtyTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl DirtyTracker {
    /// Create a new dirty tracker.
    pub fn new() -> Self {
        Self {
            dirty_components: Arc::new(RwLock::new(HashSet::new())),
            previous_vdom: Arc::new(RwLock::new(None)),
            previous_layouts: Arc::new(RwLock::new(HashMap::new())),
            previous_viewport: Arc::new(RwLock::new(None)),
        }
    }

    /// Mark a component as dirty (needs re-rendering).
    pub fn mark_dirty(&self, component_id: ComponentId) {
        let mut dirty = self.dirty_components.write().unwrap();
        dirty.insert(component_id);
    }

    /// Mark multiple components as dirty.
    pub fn mark_dirty_batch(&self, component_ids: &[ComponentId]) {
        let mut dirty = self.dirty_components.write().unwrap();
        for &id in component_ids {
            dirty.insert(id);
        }
    }

    /// Check if a component is dirty.
    pub fn is_dirty(&self, component_id: ComponentId) -> bool {
        let dirty = self.dirty_components.read().unwrap();
        dirty.contains(&component_id)
    }

    /// Get all dirty components.
    pub fn get_dirty_components(&self) -> HashSet<ComponentId> {
        let dirty = self.dirty_components.read().unwrap();
        dirty.clone()
    }

    /// Clear dirty status for a component.
    pub fn clear_dirty(&self, component_id: ComponentId) {
        let mut dirty = self.dirty_components.write().unwrap();
        dirty.remove(&component_id);
    }

    /// Clear all dirty components.
    pub fn clear_all_dirty(&self) {
        let mut dirty = self.dirty_components.write().unwrap();
        dirty.clear();
    }

    /// Check if viewport size has changed.
    pub fn viewport_changed(&self, current_viewport: Size) -> bool {
        let previous = self.previous_viewport.read().unwrap();
        match *previous {
            Some(prev) => prev != current_viewport,
            None => true, // First render
        }
    }

    /// Update the viewport size.
    pub fn update_viewport(&self, viewport: Size) {
        let mut previous = self.previous_viewport.write().unwrap();
        *previous = Some(viewport);
    }

    /// Check if the virtual DOM has changed significantly.
    pub fn vdom_changed(&self, current_vdom: &VirtualNode) -> bool {
        let previous = self.previous_vdom.read().unwrap();
        match previous.as_ref() {
            Some(prev) => !self.vdom_equals(prev, current_vdom),
            None => true, // First render
        }
    }

    /// Update the stored virtual DOM.
    pub fn update_vdom(&self, vdom: VirtualNode) {
        let mut previous = self.previous_vdom.write().unwrap();
        *previous = Some(vdom);
    }

    /// Compare two virtual DOM trees for equality (shallow comparison for performance).
    fn vdom_equals(&self, a: &VirtualNode, b: &VirtualNode) -> bool {
        match (a, b) {
            (VirtualNode::Empty, VirtualNode::Empty) => true,
            (VirtualNode::Text(a_text), VirtualNode::Text(b_text)) => {
                a_text.content == b_text.content
            }
            (VirtualNode::Element(a_elem), VirtualNode::Element(b_elem)) => {
                // Shallow comparison for performance
                a_elem.tag == b_elem.tag
                    && a_elem.attributes == b_elem.attributes
                    && a_elem.children.len() == b_elem.children.len()
                    && self.style_equals(&a_elem.style, &b_elem.style)
            }
            _ => false,
        }
    }

    /// Compare two styles for equality.
    fn style_equals(&self, a: &crate::render::vdom::VirtualStyle, b: &crate::render::vdom::VirtualStyle) -> bool {
        // Compare key style properties that affect layout
        a.display == b.display
            && a.width == b.width
            && a.height == b.height
            && a.flex_direction == b.flex_direction
            && a.justify_content == b.justify_content
            && a.align_items == b.align_items
    }

    /// Get layout changes since last render.
    pub fn get_layout_changes(&self, current_layouts: &HashMap<ComponentId, Size>) -> Vec<LayoutChange> {
        let previous = self.previous_layouts.read().unwrap();
        let mut changes = Vec::new();

        for (&component_id, &current_size) in current_layouts {
            if let Some(&previous_size) = previous.get(&component_id) {
                if previous_size != current_size {
                    changes.push(LayoutChange::Resize(component_id, current_size));
                }
            } else {
                changes.push(LayoutChange::Add(component_id, current_size));
            }
        }

        // Check for removed components
        for &component_id in previous.keys() {
            if !current_layouts.contains_key(&component_id) {
                changes.push(LayoutChange::Remove(component_id));
            }
        }

        changes
    }

    /// Update stored layout information.
    pub fn update_layouts(&self, layouts: HashMap<ComponentId, Size>) {
        let mut previous = self.previous_layouts.write().unwrap();
        *previous = layouts;
    }

    /// Check if any components need re-rendering.
    pub fn has_dirty_components(&self) -> bool {
        let dirty = self.dirty_components.read().unwrap();
        !dirty.is_empty()
    }

    /// Get statistics about dirty tracking.
    pub fn get_stats(&self) -> DirtyStats {
        let dirty = self.dirty_components.read().unwrap();
        let previous_layouts = self.previous_layouts.read().unwrap();
        let has_vdom = self.previous_vdom.read().unwrap().is_some();
        let has_viewport = self.previous_viewport.read().unwrap().is_some();

        DirtyStats {
            dirty_component_count: dirty.len(),
            tracked_layout_count: previous_layouts.len(),
            has_previous_vdom: has_vdom,
            has_previous_viewport: has_viewport,
        }
    }
}

/// Represents a change in layout.
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutChange {
    /// Component was added with the given size
    Add(ComponentId, Size),
    /// Component was resized
    Resize(ComponentId, Size),
    /// Component was removed
    Remove(ComponentId),
}

/// Statistics about dirty tracking.
#[derive(Debug, Clone)]
pub struct DirtyStats {
    /// Number of components marked as dirty
    pub dirty_component_count: usize,
    /// Number of components with tracked layouts
    pub tracked_layout_count: usize,
    /// Whether we have a previous VDOM for comparison
    pub has_previous_vdom: bool,
    /// Whether we have a previous viewport size
    pub has_previous_viewport: bool,
}

/// Batch update helper for efficient state changes.
pub struct BatchUpdate {
    tracker: Arc<DirtyTracker>,
    pending_components: Vec<ComponentId>,
}

impl BatchUpdate {
    /// Create a new batch update.
    pub fn new(tracker: Arc<DirtyTracker>) -> Self {
        Self {
            tracker,
            pending_components: Vec::new(),
        }
    }

    /// Add a component to the batch.
    pub fn add_component(&mut self, component_id: ComponentId) {
        self.pending_components.push(component_id);
    }

    /// Commit all pending changes.
    pub fn commit(self) {
        if !self.pending_components.is_empty() {
            self.tracker.mark_dirty_batch(&self.pending_components);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::ComponentId;

    #[test]
    fn test_dirty_tracking() {
        let tracker = DirtyTracker::new();
        let component_id = ComponentId::new();

        // Initially not dirty
        assert!(!tracker.is_dirty(component_id));
        assert!(!tracker.has_dirty_components());

        // Mark as dirty
        tracker.mark_dirty(component_id);
        assert!(tracker.is_dirty(component_id));
        assert!(tracker.has_dirty_components());

        // Clear dirty
        tracker.clear_dirty(component_id);
        assert!(!tracker.is_dirty(component_id));
        assert!(!tracker.has_dirty_components());
    }

    #[test]
    fn test_batch_update() {
        let tracker = Arc::new(DirtyTracker::new());
        let mut batch = BatchUpdate::new(tracker.clone());

        let id1 = ComponentId::new();
        let id2 = ComponentId::new();

        batch.add_component(id1);
        batch.add_component(id2);

        // Not dirty until committed
        assert!(!tracker.is_dirty(id1));
        assert!(!tracker.is_dirty(id2));

        batch.commit();

        // Now both are dirty
        assert!(tracker.is_dirty(id1));
        assert!(tracker.is_dirty(id2));
    }

    #[test]
    fn test_viewport_change_detection() {
        let tracker = DirtyTracker::new();
        let size1 = Size::new(80, 24);
        let size2 = Size::new(100, 30);

        // First viewport change
        assert!(tracker.viewport_changed(size1));
        tracker.update_viewport(size1);

        // Same viewport, no change
        assert!(!tracker.viewport_changed(size1));

        // Different viewport, change detected
        assert!(tracker.viewport_changed(size2));
        tracker.update_viewport(size2);
        assert!(!tracker.viewport_changed(size2));
    }
}
