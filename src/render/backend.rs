//! Backend abstraction for different rendering targets.

use crate::error::Result;
use crate::layout::{Rect, Size};
use crate::render::vdom::{VirtualNode, VirtualElement, VirtualText};
use crate::style::Color;
use crate::event::types::{Event, KeyEvent, NcKey, KeyModifiers};

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
            size: Size { width: 80, height: 24 },
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
        Self {
            initialized: false,
        }
    }

    /// Convert our Color to libnotcurses channel.
    fn color_to_channel(color: &Color) -> u32 {
        let (r, g, b, _a) = (color.r, color.g, color.b, color.a);
        let mut channel = 0u32;
        libnotcurses_sys::c_api::ncchannel_set_rgb8(&mut channel, r, g, b);
        channel
    }

    /// Convert libnotcurses key to our key type.
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
                message: "Backend not initialized".to_string()
            });
        }

        // Create a temporary Nc instance to get terminal size
        let nc = unsafe {
            Nc::with_flags(NcFlag::SuppressBanners | NcFlag::NoAlternateScreen)
                .map_err(|e| crate::error::Error::Framework {
                    message: format!("Failed to initialize libnotcurses: {:?}", e)
                })?
        };

        let stdplane = unsafe { nc.stdplane() };
        let (rows, cols) = stdplane.dim_yx();

        // Clean up
        unsafe { nc.stop() }.map_err(|e| crate::error::Error::Framework {
            message: format!("Failed to stop libnotcurses: {:?}", e)
        })?;

        Ok(Size {
            width: cols as u32,
            height: rows as u32
        })
    }

    fn clear(&mut self) -> Result<()> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string()
            });
        }

        // Create a temporary Nc instance to clear screen
        let nc = unsafe {
            Nc::with_flags(NcFlag::SuppressBanners | NcFlag::NoAlternateScreen)
                .map_err(|e| crate::error::Error::Framework {
                    message: format!("Failed to initialize libnotcurses: {:?}", e)
                })?
        };

        let stdplane = unsafe { nc.stdplane() };
        stdplane.erase();
        nc.render().map_err(|e| crate::error::Error::Framework {
            message: format!("Failed to render: {:?}", e)
        })?;

        // Clean up
        unsafe { nc.stop() }.map_err(|e| crate::error::Error::Framework {
            message: format!("Failed to stop libnotcurses: {:?}", e)
        })?;

        Ok(())
    }

    fn render_node(&mut self, _node: &VirtualNode, _rect: Rect) -> Result<()> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string()
            });
        }

        // For now, just return Ok - rendering will be handled in present()
        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string()
            });
        }

        // For now, just return Ok - actual rendering would happen here
        Ok(())
    }

    fn poll_event(&mut self) -> Result<Option<Event>> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string()
            });
        }

        // For now, just return None - no events available
        Ok(None)
    }

    fn wait_event(&mut self) -> Result<Event> {
        if !self.initialized {
            return Err(crate::error::Error::Framework {
                message: "Backend not initialized".to_string()
            });
        }

        // For now, just return a dummy escape key event
        Ok(Event::Key(KeyEvent::new(NcKey::Esc)))
    }
}

#[cfg(feature = "notcurses")]
impl NotcursesBackend {
    /// Recursively render a virtual node and its children.
    fn render_node_recursive(
        &mut self,
        plane: &mut libnotcurses_sys::NcPlane,
        node: &VirtualNode,
        rect: Rect
    ) -> Result<()> {
        match node {
            VirtualNode::Element(element) => {
                self.render_element(plane, element, rect)?;
            },
            VirtualNode::Text(text) => {
                self.render_text(plane, text, rect)?;
            },
            VirtualNode::Empty => {
                // Nothing to render
            },
        }
        Ok(())
    }

    /// Render a virtual element.
    fn render_element(
        &mut self,
        plane: &mut libnotcurses_sys::NcPlane,
        element: &VirtualElement,
        rect: Rect
    ) -> Result<()> {
        // Set background color if specified
        if let Some(ref bg_color) = element.style.background_color {
            let channel = Self::color_to_channel(bg_color);
            plane.set_bg_rgb((
                (channel >> 16) as u8,
                (channel >> 8) as u8,
                channel as u8,
            ));
        }

        // Set foreground color if specified
        if let Some(ref fg_color) = element.style.color {
            let channel = Self::color_to_channel(fg_color);
            plane.set_fg_rgb((
                (channel >> 16) as u8,
                (channel >> 8) as u8,
                channel as u8,
            ));
        }

        // Render children if we have computed layout
        if let Some(ref _layout) = element.layout {
            let mut child_y = rect.position.y;

            for child in &element.children {
                // Calculate child rect based on layout
                let child_rect = Rect::new(
                    crate::layout::Position::new(rect.position.x, child_y),
                    crate::layout::Size::new(rect.size.width, 1) // Simple line-by-line for now
                );

                self.render_node_recursive(plane, child, child_rect)?;
                child_y += 1;
            }
        }

        Ok(())
    }

    /// Render a text node.
    fn render_text(
        &mut self,
        plane: &mut libnotcurses_sys::NcPlane,
        text: &VirtualText,
        rect: Rect
    ) -> Result<()> {
        // Move cursor to position
        plane.cursor_move_yx(rect.position.y as u32, rect.position.x as u32)
            .map_err(|e| crate::error::Error::Framework {
                message: format!("Failed to move cursor: {:?}", e)
            })?;

        // Put the text
        plane.putstr(&text.content)
            .map_err(|e| crate::error::Error::Framework {
                message: format!("Failed to put text: {:?}", e)
            })?;

        Ok(())
    }
}
