//! CSS-like pseudo-selectors for interactive states.
//!
//! This module provides pseudo-selector support for different component states
//! like hover, focus, active, disabled, etc.

use crate::render::vdom::VirtualStyle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pseudo-selector states that components can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PseudoState {
    /// Normal state (no pseudo-selector)
    Normal,
    /// Mouse is hovering over the element
    Hover,
    /// Element has keyboard focus
    Focus,
    /// Element is being activated (e.g., mouse pressed)
    Active,
    /// Element is disabled
    Disabled,
    /// Element is checked (for checkboxes, radio buttons)
    Checked,
    /// Element is selected (for list items, options)
    Selected,
    /// First child element
    FirstChild,
    /// Last child element
    LastChild,
    /// Nth child element
    NthChild(u32),
    /// Element is empty (no content)
    Empty,
    /// Element is valid (for form inputs)
    Valid,
    /// Element is invalid (for form inputs)
    Invalid,
}

impl PseudoState {
    /// Get the CSS-like string representation of this pseudo-state.
    pub fn to_css_string(&self) -> String {
        match self {
            PseudoState::Normal => String::new(),
            PseudoState::Hover => ":hover".to_string(),
            PseudoState::Focus => ":focus".to_string(),
            PseudoState::Active => ":active".to_string(),
            PseudoState::Disabled => ":disabled".to_string(),
            PseudoState::Checked => ":checked".to_string(),
            PseudoState::Selected => ":selected".to_string(),
            PseudoState::FirstChild => ":first-child".to_string(),
            PseudoState::LastChild => ":last-child".to_string(),
            PseudoState::NthChild(n) => format!(":nth-child({})", n),
            PseudoState::Empty => ":empty".to_string(),
            PseudoState::Valid => ":valid".to_string(),
            PseudoState::Invalid => ":invalid".to_string(),
        }
    }

    /// Parse a CSS-like pseudo-selector string.
    pub fn from_css_string(css: &str) -> Option<Self> {
        match css {
            "" => Some(PseudoState::Normal),
            ":hover" => Some(PseudoState::Hover),
            ":focus" => Some(PseudoState::Focus),
            ":active" => Some(PseudoState::Active),
            ":disabled" => Some(PseudoState::Disabled),
            ":checked" => Some(PseudoState::Checked),
            ":selected" => Some(PseudoState::Selected),
            ":first-child" => Some(PseudoState::FirstChild),
            ":last-child" => Some(PseudoState::LastChild),
            ":empty" => Some(PseudoState::Empty),
            ":valid" => Some(PseudoState::Valid),
            ":invalid" => Some(PseudoState::Invalid),
            _ => {
                // Try to parse nth-child
                if css.starts_with(":nth-child(") && css.ends_with(')') {
                    let number_str = &css[11..css.len()-1];
                    if let Ok(n) = number_str.parse::<u32>() {
                        return Some(PseudoState::NthChild(n));
                    }
                }
                None
            }
        }
    }
}

/// A style rule that applies to a specific pseudo-state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoStyleRule {
    /// The pseudo-state this rule applies to
    pub state: PseudoState,
    /// The style to apply in this state
    pub style: VirtualStyle,
    /// Priority of this rule (higher = more important)
    pub priority: u32,
}

impl PseudoStyleRule {
    /// Create a new pseudo-style rule.
    pub fn new(state: PseudoState, style: VirtualStyle) -> Self {
        Self {
            state,
            style,
            priority: 0,
        }
    }

    /// Set the priority of this rule.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}

/// A collection of pseudo-style rules for a component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PseudoStyleSheet {
    /// Rules organized by pseudo-state
    rules: HashMap<PseudoState, Vec<PseudoStyleRule>>,
}

impl PseudoStyleSheet {
    /// Create a new empty pseudo-style sheet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a style rule for a pseudo-state.
    pub fn add_rule(&mut self, rule: PseudoStyleRule) {
        let state = rule.state;
        self.rules
            .entry(state)
            .or_insert_with(Vec::new)
            .push(rule);

        // Sort by priority (highest first)
        if let Some(rules) = self.rules.get_mut(&state) {
            rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        }
    }

    /// Add a style for a specific pseudo-state.
    pub fn add_style(&mut self, state: PseudoState, style: VirtualStyle) {
        self.add_rule(PseudoStyleRule::new(state, style));
    }

    /// Get the computed style for a set of active pseudo-states.
    pub fn get_computed_style(&self, active_states: &[PseudoState], base_style: &VirtualStyle) -> VirtualStyle {
        let mut computed = base_style.clone();

        // Collect all applicable rules
        let mut applicable_rules = Vec::new();
        
        for &state in active_states {
            if let Some(rules) = self.rules.get(&state) {
                applicable_rules.extend(rules.iter());
            }
        }

        // Sort by priority (highest first)
        applicable_rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Apply rules in priority order
        for rule in applicable_rules {
            computed = merge_styles(&computed, &rule.style);
        }

        computed
    }

    /// Get all rules for a specific pseudo-state.
    pub fn get_rules(&self, state: PseudoState) -> Option<&Vec<PseudoStyleRule>> {
        self.rules.get(&state)
    }

    /// Remove all rules for a specific pseudo-state.
    pub fn remove_state(&mut self, state: PseudoState) {
        self.rules.remove(&state);
    }

    /// Clear all rules.
    pub fn clear(&mut self) {
        self.rules.clear();
    }

    /// Check if there are any rules defined.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Get the number of defined states.
    pub fn state_count(&self) -> usize {
        self.rules.len()
    }

    /// Get all defined pseudo-states.
    pub fn get_states(&self) -> Vec<PseudoState> {
        self.rules.keys().copied().collect()
    }
}

/// Component state tracker for pseudo-selectors.
#[derive(Debug, Clone, Default)]
pub struct ComponentState {
    /// Currently active pseudo-states
    active_states: Vec<PseudoState>,
    /// Whether the component is currently hovered
    pub is_hovered: bool,
    /// Whether the component has focus
    pub is_focused: bool,
    /// Whether the component is being activated
    pub is_active: bool,
    /// Whether the component is disabled
    pub is_disabled: bool,
    /// Whether the component is checked (for checkboxes)
    pub is_checked: bool,
    /// Whether the component is selected (for list items)
    pub is_selected: bool,
    /// Position in parent (for nth-child)
    pub child_index: Option<u32>,
    /// Whether this is the first child
    pub is_first_child: bool,
    /// Whether this is the last child
    pub is_last_child: bool,
    /// Whether the component is empty
    pub is_empty: bool,
    /// Whether the component is valid (for form inputs)
    pub is_valid: bool,
}

impl ComponentState {
    /// Create a new component state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the active pseudo-states based on current component state.
    pub fn update_active_states(&mut self) {
        self.active_states.clear();
        self.active_states.push(PseudoState::Normal);

        if self.is_hovered {
            self.active_states.push(PseudoState::Hover);
        }
        if self.is_focused {
            self.active_states.push(PseudoState::Focus);
        }
        if self.is_active {
            self.active_states.push(PseudoState::Active);
        }
        if self.is_disabled {
            self.active_states.push(PseudoState::Disabled);
        }
        if self.is_checked {
            self.active_states.push(PseudoState::Checked);
        }
        if self.is_selected {
            self.active_states.push(PseudoState::Selected);
        }
        if self.is_first_child {
            self.active_states.push(PseudoState::FirstChild);
        }
        if self.is_last_child {
            self.active_states.push(PseudoState::LastChild);
        }
        if let Some(index) = self.child_index {
            self.active_states.push(PseudoState::NthChild(index));
        }
        if self.is_empty {
            self.active_states.push(PseudoState::Empty);
        }
        if self.is_valid {
            self.active_states.push(PseudoState::Valid);
        } else {
            self.active_states.push(PseudoState::Invalid);
        }
    }

    /// Get the currently active pseudo-states.
    pub fn get_active_states(&self) -> &[PseudoState] {
        &self.active_states
    }

    /// Set hover state.
    pub fn set_hover(&mut self, hovered: bool) {
        self.is_hovered = hovered;
        self.update_active_states();
    }

    /// Set focus state.
    pub fn set_focus(&mut self, focused: bool) {
        self.is_focused = focused;
        self.update_active_states();
    }

    /// Set active state.
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        self.update_active_states();
    }

    /// Set disabled state.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.is_disabled = disabled;
        self.update_active_states();
    }
}

/// Merge two styles, with the second style taking precedence.
fn merge_styles(base: &VirtualStyle, overlay: &VirtualStyle) -> VirtualStyle {
    let mut result = base.clone();

    // Merge each field, preferring overlay values when present
    if overlay.display.is_some() {
        result.display = overlay.display;
    }
    if overlay.width.is_some() {
        result.width = overlay.width.clone();
    }
    if overlay.height.is_some() {
        result.height = overlay.height.clone();
    }
    if overlay.background_color.is_some() {
        result.background_color = overlay.background_color;
    }
    if overlay.color.is_some() {
        result.color = overlay.color;
    }
    if overlay.border_color.is_some() {
        result.border_color = overlay.border_color;
    }
    if overlay.flex_direction.is_some() {
        result.flex_direction = overlay.flex_direction;
    }
    if overlay.justify_content.is_some() {
        result.justify_content = overlay.justify_content;
    }
    if overlay.align_items.is_some() {
        result.align_items = overlay.align_items;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_pseudo_state_css_strings() {
        assert_eq!(PseudoState::Hover.to_css_string(), ":hover");
        assert_eq!(PseudoState::Focus.to_css_string(), ":focus");
        assert_eq!(PseudoState::NthChild(3).to_css_string(), ":nth-child(3)");
        
        assert_eq!(PseudoState::from_css_string(":hover"), Some(PseudoState::Hover));
        assert_eq!(PseudoState::from_css_string(":nth-child(5)"), Some(PseudoState::NthChild(5)));
    }

    #[test]
    fn test_pseudo_style_sheet() {
        let mut sheet = PseudoStyleSheet::new();
        
        let mut hover_style = VirtualStyle::default();
        hover_style.background_color = Some(Color::rgb(255, 0, 0));

        sheet.add_style(PseudoState::Hover, hover_style);

        let base_style = VirtualStyle::default();
        let computed = sheet.get_computed_style(&[PseudoState::Hover], &base_style);

        assert_eq!(computed.background_color, Some(Color::rgb(255, 0, 0)));
    }

    #[test]
    fn test_component_state() {
        let mut state = ComponentState::new();
        
        state.set_hover(true);
        assert!(state.get_active_states().contains(&PseudoState::Hover));
        
        state.set_focus(true);
        assert!(state.get_active_states().contains(&PseudoState::Focus));
        assert!(state.get_active_states().contains(&PseudoState::Hover));
    }

    #[test]
    fn test_style_merging() {
        let mut base = VirtualStyle::default();
        base.color = Some(Color::rgb(0, 0, 0));

        let mut overlay = VirtualStyle::default();
        overlay.background_color = Some(Color::rgb(255, 255, 255));

        let merged = merge_styles(&base, &overlay);

        assert_eq!(merged.color, Some(Color::rgb(0, 0, 0)));
        assert_eq!(merged.background_color, Some(Color::rgb(255, 255, 255)));
    }
}
