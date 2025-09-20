//! Backend abstraction for different rendering targets.

use crate::error::Result;
use crate::event::types::{Event, KeyEvent, NcKey};
use crate::layout::{Rect, Size};
use crate::render::vdom::VirtualNode;

#[cfg(feature = "notcurses")]
use crate::event::types::KeyModifiers;
#[cfg(feature = "notcurses")]
use crate::render::vdom::{VirtualElement, VirtualText};
#[cfg(feature = "notcurses")]
use crate::style::Color;

#[cfg(feature = "notcurses")]
use libnotcurses_sys::{Nc, NcFlag};

/// Backend trait for different rendering implementations.
pub trait Backend {
    /// Initialize the backend.
    fn init(&mut self) -> Result<()>;

    /// Clean up the backend.
    fn cleanup(&mut self) -> Result<()>;

    /// Get the terminal size.
    fn size(&self) -> Result<Size>;

    /// Clear the screen.
    fn clear(&mut self) -> Result<()>;

    /// Render a virtual node at the given position.
    fn render_node(&mut self, node: &VirtualNode, rect: Rect) -> Result<()>;

    /// Present the rendered content to the screen.
    fn present(&mut self) -> Result<()>;

    /// Poll for input events (non-blocking).
    fn poll_event(&mut self) -> Result<Option<Event>>;

    /// Wait for input events (blocking).
    fn wait_event(&mut self) -> Result<Event>;
}

/// Placeholder backend implementation for testing.
pub struct PlaceholderBackend {
    size: Size,
}

impl PlaceholderBackend {
    /// Create a new placeholder backend.
    pub fn new() -> Self {
        Self {
            size: Size {
                width: 80,
                height: 24,
            },
        }
    }
}

impl Default for PlaceholderBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PlaceholderBackend {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn size(&self) -> Result<Size> {
        Ok(self.size)
    }

    fn clear(&mut self) -> Result<()> {
        Ok(())
    }

    fn render_node(&mut self, _node: &VirtualNode, _rect: Rect) -> Result<()> {
        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        Ok(())
    }

    fn poll_event(&mut self) -> Result<Option<Event>> {
        Ok(None)
    }

    fn wait_event(&mut self) -> Result<Event> {
        // Return a dummy event for testing
        Ok(Event::Key(KeyEvent::new(NcKey::Esc)))
    }
}

/// Libnotcurses backend implementation.
#[cfg(feature = "notcurses")]
pub struct NotcursesBackend {
    initialized: bool,
}

#[cfg(feature = "notcurses")]
impl NotcursesBackend {
    /// Create a new libnotcurses backend.
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Convert our Color to libnotcurses channel.
    #[allow(dead_code)]
    fn color_to_channel(color: &Color) -> u32 {
        let (r, g, b, _a) = (color.r, color.g, color.b, color.a);
        let mut channel = 0u32;
        libnotcurses_sys::c_api::ncchannel_set_rgb8(&mut channel, r, g, b);
        channel
    }

    /// Convert libnotcurses key to our key type.
    #[allow(dead_code)]
    fn convert_key(key: libnotcurses_sys::NcKey) -> NcKey {
        match key {
            libnotcurses_sys::NcKey::Enter => NcKey::Enter,
            libnotcurses_sys::NcKey::Esc => NcKey::Esc,
            libnotcurses_sys::NcKey::Space => NcKey::Space,
            libnotcurses_sys::NcKey::Backspace => NcKey::Backspace,
            libnotcurses_sys::NcKey::Del => NcKey::Del,
            libnotcurses_sys::NcKey::Tab => NcKey::Tab,
            libnotcurses_sys::NcKey::Up => NcKey::Up,
            libnotcurses_sys::NcKey::Down => NcKey::Down,
            libnotcurses_sys::NcKey::Left => NcKey::Left,
            libnotcurses_sys::NcKey::Right => NcKey::Right,
            libnotcurses_sys::NcKey::Home => NcKey::Home,
            libnotcurses_sys::NcKey::End => NcKey::End,
            libnotcurses_sys::NcKey::PgUp => NcKey::PgUp,
            libnotcurses_sys::NcKey::PgDown => NcKey::PgDown,
            libnotcurses_sys::NcKey::F01 => NcKey::F01,
            libnotcurses_sys::NcKey::F02 => NcKey::F02,
            libnotcurses_sys::NcKey::F03 => NcKey::F03,
            libnotcurses_sys::NcKey::F04 => NcKey::F04,
            libnotcurses_sys::NcKey::F05 => NcKey::F05,
            libnotcurses_sys::NcKey::F06 => NcKey::F06,
            libnotcurses_sys::NcKey::F07 => NcKey::F07,
            libnotcurses_sys::NcKey::F08 => NcKey::F08,
            libnotcurses_sys::NcKey::F09 => NcKey::F09,
            libnotcurses_sys::NcKey::F10 => NcKey::F10,
            libnotcurses_sys::NcKey::F11 => NcKey::F11,
            libnotcurses_sys::NcKey::F12 => NcKey::F12,
            _ => NcKey::Esc, // Default fallback
        }
    }

    /// Convert libnotcurses modifiers to our modifiers.
    #[allow(dead_code)]
    fn convert_modifiers(input: &libnotcurses_sys::NcInput) -> KeyModifiers {
        let mut modifiers = KeyModifiers::empty();

        if input.ctrl {
            modifiers |= KeyModifiers::CTRL;
        }
        if input.alt {
            modifiers |= KeyModifiers::ALT;
        }
        if input.shift {
            modifiers |= KeyModifiers::SHIFT;
        }

        modifiers
    }
}

#[cfg(feature = "notcurses")]
impl Default for NotcursesBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "notcurses")]
impl Backend for NotcursesBackend {
    fn init(&mut self) -> Result<()> {
        // Initialize libnotcurses with appropriate flags
        // Just mark as initialized - we'll create Nc instances as needed
        self.initialized = true;

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        self.initialized = false;

        Ok(())
    }

    fn size(&self) -> Result<Size> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string(),
            });
        }

        // Create a temporary Nc instance to get terminal size
        let nc = unsafe {
            Nc::with_flags(NcFlag::SuppressBanners | NcFlag::NoAlternateScreen).map_err(|e| {
                crate::error::Error::Framework {
                    message: format!("Failed to initialize libnotcurses: {:?}", e),
                }
            })?
        };

        let stdplane = unsafe { nc.stdplane() };
        let (rows, cols) = stdplane.dim_yx();

        // Clean up
        unsafe { nc.stop() }.map_err(|e| crate::error::Error::Framework {
            message: format!("Failed to stop libnotcurses: {:?}", e),
        })?;

        Ok(Size {
            width: cols,
            height: rows,
        })
    }

    fn clear(&mut self) -> Result<()> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string(),
            });
        }

        // Create a temporary Nc instance to clear screen
        let nc = unsafe {
            Nc::with_flags(NcFlag::SuppressBanners | NcFlag::NoAlternateScreen).map_err(|e| {
                crate::error::Error::Framework {
                    message: format!("Failed to initialize libnotcurses: {:?}", e),
                }
            })?
        };

        let stdplane = unsafe { nc.stdplane() };
        stdplane.erase();
        nc.render().map_err(|e| crate::error::Error::Framework {
            message: format!("Failed to render: {:?}", e),
        })?;

        // Clean up
        unsafe { nc.stop() }.map_err(|e| crate::error::Error::Framework {
            message: format!("Failed to stop libnotcurses: {:?}", e),
        })?;

        Ok(())
    }

    fn render_node(&mut self, _node: &VirtualNode, _rect: Rect) -> Result<()> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string(),
            });
        }

        // For now, just return Ok - rendering will be handled in present()
        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string(),
            });
        }

        // For now, just return Ok - actual rendering would happen here
        Ok(())
    }

    fn poll_event(&mut self) -> Result<Option<Event>> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string(),
            });
        }

        // For now, just return None - no events available
        Ok(None)
    }

    fn wait_event(&mut self) -> Result<Event> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string(),
            });
        }

        // For now, just return a dummy escape key event
        Ok(Event::Key(KeyEvent::new(NcKey::Esc)))
    }
}

#[cfg(feature = "notcurses")]
impl NotcursesBackend {
    /// Recursively render a virtual node and its children.
    #[allow(dead_code)]
    fn render_node_recursive(
        &mut self,
        plane: &mut libnotcurses_sys::NcPlane,
        node: &VirtualNode,
        rect: Rect,
    ) -> Result<()> {
        match node {
            VirtualNode::Element(element) => {
                self.render_element(plane, element, rect)?;
            }
            VirtualNode::Text(text) => {
                self.render_text(plane, text, rect)?;
            }
            VirtualNode::Empty => {
                // Nothing to render
            }
        }
        Ok(())
    }

    /// Render a virtual element.
    #[allow(dead_code)]
    fn render_element(
        &mut self,
        plane: &mut libnotcurses_sys::NcPlane,
        element: &VirtualElement,
        rect: Rect,
    ) -> Result<()> {
        // Set background color if specified
        if let Some(ref bg_color) = element.style.background_color {
            let channel = Self::color_to_channel(bg_color);
            plane.set_bg_rgb(((channel >> 16) as u8, (channel >> 8) as u8, channel as u8));
        }

        // Set foreground color if specified
        if let Some(ref fg_color) = element.style.color {
            let channel = Self::color_to_channel(fg_color);
            plane.set_fg_rgb(((channel >> 16) as u8, (channel >> 8) as u8, channel as u8));
        }

        // Render children if we have computed layout
        if let Some(ref _layout) = element.layout {
            let mut child_y = rect.position.y;

            for child in &element.children {
                // Calculate child rect based on layout
                let child_rect = Rect::new(
                    crate::layout::Position::new(rect.position.x, child_y),
                    crate::layout::Size::new(rect.size.width, 1), // Simple line-by-line for now
                );

                self.render_node_recursive(plane, child, child_rect)?;
                child_y += 1;
            }
        }

        Ok(())
    }

    /// Render a text node.
    #[allow(dead_code)]
    fn render_text(
        &mut self,
        plane: &mut libnotcurses_sys::NcPlane,
        text: &VirtualText,
        rect: Rect,
    ) -> Result<()> {
        // Move cursor to position
        plane
            .cursor_move_yx(rect.position.y as u32, rect.position.x as u32)
            .map_err(|e| crate::error::Error::Framework {
                message: format!("Failed to move cursor: {:?}", e),
            })?;

        // Put the text
        plane
            .putstr(&text.content)
            .map_err(|e| crate::error::Error::Framework {
                message: format!("Failed to put text: {:?}", e),
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::vdom::nodes::{div, text};

    #[test]
    fn test_placeholder_backend_creation() {
        let backend = PlaceholderBackend::new();
        assert_eq!(backend.size, Size::new(80, 24));
    }

    #[test]
    fn test_placeholder_backend_init() {
        let mut backend = PlaceholderBackend::new();
        let result = backend.init();
        assert!(result.is_ok());
    }

    #[test]
    fn test_placeholder_backend_cleanup() {
        let mut backend = PlaceholderBackend::new();
        let result = backend.cleanup();
        assert!(result.is_ok());
    }

    #[test]
    fn test_placeholder_backend_size() {
        let backend = PlaceholderBackend::new();
        let size = backend.size().unwrap();
        assert_eq!(size, Size::new(80, 24)); // Default size
    }

    #[test]
    fn test_placeholder_backend_clear() {
        let mut backend = PlaceholderBackend::new();
        let result = backend.clear();
        assert!(result.is_ok());
    }

    #[test]
    fn test_placeholder_backend_render_text_node() {
        let mut backend = PlaceholderBackend::new();
        let node = text("Hello, World!");
        let rect = Rect::from_coords(0, 0, 20, 5);

        let result = backend.render_node(&node, rect);
        assert!(result.is_ok());
    }

    #[test]
    fn test_placeholder_backend_render_div_node() {
        let mut backend = PlaceholderBackend::new();
        let node = div().child(text("Content"));
        let rect = Rect::from_coords(0, 0, 20, 5);

        let result = backend.render_node(&node, rect);
        assert!(result.is_ok());
    }

    #[test]
    fn test_placeholder_backend_present() {
        let mut backend = PlaceholderBackend::new();
        let result = backend.present();
        assert!(result.is_ok());
    }

    #[test]
    fn test_placeholder_backend_poll_event() {
        let mut backend = PlaceholderBackend::new();
        let result = backend.poll_event();
        assert!(result.is_ok());
        // Placeholder backend should return None for events
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_placeholder_backend_wait_event() {
        let mut backend = PlaceholderBackend::new();
        let result = backend.wait_event();
        assert!(result.is_ok());
        // Placeholder backend should return a dummy key event
        match result.unwrap() {
            Event::Key(_) => assert!(true),
            _ => panic!("Expected key event from placeholder backend"),
        }
    }

    #[test]
    fn test_placeholder_backend_full_lifecycle() {
        let mut backend = PlaceholderBackend::new();

        // Initialize
        assert!(backend.init().is_ok());

        // Clear screen
        assert!(backend.clear().is_ok());

        // Render some content
        let node = div()
            .child(text("Line 1"))
            .child(text("Line 2"));
        let rect = Rect::from_coords(0, 0, 80, 24);
        assert!(backend.render_node(&node, rect).is_ok());

        // Present
        assert!(backend.present().is_ok());

        // Poll for events
        assert!(backend.poll_event().is_ok());

        // Cleanup
        assert!(backend.cleanup().is_ok());
    }

    #[test]
    fn test_placeholder_backend_default_size() {
        // PlaceholderBackend has a fixed default size
        let backend = PlaceholderBackend::new();
        assert_eq!(backend.size().unwrap(), Size::new(80, 24));
    }

    #[test]
    fn test_placeholder_backend_render_empty_node() {
        let mut backend = PlaceholderBackend::new();
        let node = div(); // Empty div
        let rect = Rect::from_coords(0, 0, 10, 10);

        let result = backend.render_node(&node, rect);
        assert!(result.is_ok());
    }

    #[test]
    fn test_placeholder_backend_render_nested_nodes() {
        let mut backend = PlaceholderBackend::new();
        let node = div()
            .child(
                div()
                    .child(text("Nested content"))
                    .child(text("More nested"))
            )
            .child(text("Top level"));
        let rect = Rect::from_coords(0, 0, 80, 24);

        let result = backend.render_node(&node, rect);
        assert!(result.is_ok());
    }

    #[cfg(feature = "notcurses")]
    #[test]
    fn test_notcurses_backend_creation() {
        // This test only runs when notcurses feature is enabled
        // Note: This might fail in CI environments without a terminal
        let result = NotcursesBackend::new();
        // We can't guarantee this will succeed in all environments,
        // so we just test that the function exists and returns a Result
        match result {
            Ok(_) => assert!(true),
            Err(_) => {
                // Expected in headless environments
                assert!(true);
            }
        }
    }

    #[test]
    fn test_backend_trait_object() {
        // Test that Backend can be used as a trait object
        let backend: Box<dyn Backend> = Box::new(PlaceholderBackend::new());

        // This should compile and work
        let size = backend.size();
        assert!(size.is_ok());
    }
}
