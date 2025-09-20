//! Event types and definitions.

/// Mock key type when notcurses is not available
#[cfg(not(feature = "notcurses"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NcKey {
    /// Enter key
    Enter,
    /// Escape key
    Esc,
    /// Space bar
    Space,
    /// Backspace key
    Backspace,
    /// Delete key
    Del,
    /// Tab key
    Tab,
    /// Up arrow key
    Up,
    /// Down arrow key
    Down,
    /// Left arrow key
    Left,
    /// Right arrow key
    Right,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PgUp,
    /// Page Down key
    PgDown,
    /// Function key F1
    F01,
    /// Function key F2
    F02,
    /// Function key F3
    F03,
    /// Function key F4
    F04,
    /// Function key F5
    F05,
    /// Function key F6
    F06,
    /// Function key F7
    F07,
    /// Function key F8
    F08,
    /// Function key F9
    F09,
    /// Function key F10
    F10,
    /// Function key F11
    F11,
    /// Function key F12
    F12,
    // Character input is handled separately in the event system
}

#[cfg(feature = "notcurses")]
pub use libnotcurses_sys::{NcKey, NcMiceEvents};

/// Represents different types of events in the TUI framework.
#[derive(Debug, Clone)]
pub enum Event {
    /// Keyboard event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Focus event
    Focus(FocusEvent),
    /// Resize event
    Resize(ResizeEvent),
    /// Custom user-defined event
    Custom(CustomEvent),
}

impl Event {
    /// Get the event type as a string.
    pub fn event_type(&self) -> String {
        match self {
            Event::Key(_) => "key".to_string(),
            Event::Mouse(_) => "mouse".to_string(),
            Event::Focus(_) => "focus".to_string(),
            Event::Resize(_) => "resize".to_string(),
            Event::Custom(custom) => custom.event_type.clone(),
        }
    }

    /// Check if this event should bubble up to parent components.
    pub fn should_bubble(&self) -> bool {
        match self {
            Event::Key(key_event) => key_event.bubbles,
            Event::Mouse(mouse_event) => mouse_event.bubbles,
            Event::Focus(_) => false,  // Focus events don't bubble
            Event::Resize(_) => false, // Resize events don't bubble
            Event::Custom(custom) => custom.bubbles,
        }
    }

    /// Stop event propagation.
    pub fn stop_propagation(&mut self) {
        match self {
            Event::Key(key_event) => key_event.bubbles = false,
            Event::Mouse(mouse_event) => mouse_event.bubbles = false,
            Event::Focus(_) => {}  // Focus events don't bubble anyway
            Event::Resize(_) => {} // Resize events don't bubble anyway
            Event::Custom(custom) => custom.bubbles = false,
        }
    }
}

/// Keyboard event information.
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// The key that was pressed
    pub key: NcKey,
    /// Modifier keys (Ctrl, Alt, Shift)
    pub modifiers: KeyModifiers,
    /// Whether the event should bubble
    pub bubbles: bool,
    /// Whether the default action should be prevented
    pub prevent_default: bool,
}

impl KeyEvent {
    /// Create a new key event.
    pub fn new(key: NcKey) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::empty(),
            bubbles: true,
            prevent_default: false,
        }
    }

    /// Create a key event with modifiers.
    pub fn with_modifiers(key: NcKey, modifiers: KeyModifiers) -> Self {
        Self {
            key,
            modifiers,
            bubbles: true,
            prevent_default: false,
        }
    }

    /// Check if Ctrl is pressed.
    pub fn ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CTRL)
    }

    /// Check if Alt is pressed.
    pub fn alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }

    /// Check if Shift is pressed.
    pub fn shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }
}

bitflags::bitflags! {
    /// Key modifier flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct KeyModifiers: u8 {
        /// Control key modifier.
        const CTRL = 1 << 0;
        /// Alt key modifier.
        const ALT = 1 << 1;
        /// Shift key modifier.
        const SHIFT = 1 << 2;
        /// Meta/Windows/Cmd key modifier.
        const META = 1 << 3;
    }
}

/// Mouse event information.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// Mouse button that was pressed/released
    pub button: MouseButton,
    /// Mouse event type
    pub event_type: MouseEventType,
    /// X coordinate
    pub x: u32,
    /// Y coordinate
    pub y: u32,
    /// Modifier keys
    pub modifiers: KeyModifiers,
    /// Whether the event should bubble
    pub bubbles: bool,
}

impl MouseEvent {
    /// Create a new mouse event.
    pub fn new(button: MouseButton, event_type: MouseEventType, x: u32, y: u32) -> Self {
        Self {
            button,
            event_type,
            x,
            y,
            modifiers: KeyModifiers::empty(),
            bubbles: true,
        }
    }
}

/// Mouse button types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button (scroll wheel)
    Middle,
    /// Other mouse button with custom ID
    Other(u8),
}

/// Mouse event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    /// Mouse button pressed
    Press,
    /// Mouse button released
    Release,
    /// Mouse moved
    Move,
    /// Mouse wheel scrolled
    Scroll,
    /// Mouse entered element
    Enter,
    /// Mouse left element
    Leave,
}

/// Focus event information.
#[derive(Debug, Clone)]
pub struct FocusEvent {
    /// Whether focus was gained or lost
    pub focused: bool,
    /// The component that previously had focus (if any)
    pub related_target: Option<String>,
}

impl FocusEvent {
    /// Create a focus gained event.
    pub fn gained() -> Self {
        Self {
            focused: true,
            related_target: None,
        }
    }

    /// Create a focus lost event.
    pub fn lost() -> Self {
        Self {
            focused: false,
            related_target: None,
        }
    }
}

/// Resize event information.
#[derive(Debug, Clone)]
pub struct ResizeEvent {
    /// New width
    pub width: u32,
    /// New height
    pub height: u32,
}

impl ResizeEvent {
    /// Create a new resize event.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Custom event for user-defined events.
#[derive(Debug, Clone)]
pub struct CustomEvent {
    /// Event type identifier
    pub event_type: String,
    /// Event data
    pub data: serde_json::Value,
    /// Whether the event should bubble
    pub bubbles: bool,
}

impl CustomEvent {
    /// Create a new custom event.
    pub fn new(event_type: impl Into<String>, data: serde_json::Value) -> Self {
        Self {
            event_type: event_type.into(),
            data,
            bubbles: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_event() {
        let event = KeyEvent::new(NcKey::Enter);
        assert_eq!(event.key, NcKey::Enter);
        assert!(!event.ctrl());
        assert!(!event.alt());
        assert!(!event.shift());
    }

    #[test]
    fn test_key_event_with_modifiers() {
        let event = KeyEvent::with_modifiers(NcKey::Enter, KeyModifiers::CTRL);
        assert_eq!(event.key, NcKey::Enter);
        assert!(event.ctrl());
        assert!(!event.alt());
    }

    #[test]
    fn test_mouse_event() {
        let event = MouseEvent::new(MouseButton::Left, MouseEventType::Press, 10, 20);
        assert_eq!(event.button, MouseButton::Left);
        assert_eq!(event.event_type, MouseEventType::Press);
        assert_eq!(event.x, 10);
        assert_eq!(event.y, 20);
    }

    #[test]
    fn test_focus_event() {
        let gained = FocusEvent::gained();
        assert!(gained.focused);

        let lost = FocusEvent::lost();
        assert!(!lost.focused);
    }

    #[test]
    fn test_event_bubbling() {
        let mut event = Event::Key(KeyEvent::new(NcKey::Enter));
        assert!(event.should_bubble());

        event.stop_propagation();
        assert!(!event.should_bubble());
    }
}
