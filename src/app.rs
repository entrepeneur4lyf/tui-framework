//! Application management and event loop.

use crate::component::Component;
use crate::error::Result;
use crate::event::types::Event;
use crate::layout::Rect;
use crate::render::backend::{Backend, PlaceholderBackend};
use crate::render::context::RenderContext;
use crate::style::Theme;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "notcurses")]
use crate::render::backend::NotcursesBackend;

/// The main application struct that manages the component tree and event loop.
pub struct App {
    title: String,
    theme: Theme,
    root_component: Option<Box<dyn Component>>,
    backend: Box<dyn Backend>,
    running: Arc<RwLock<bool>>,
}

impl App {
    /// Create a new application instance with placeholder backend.
    pub fn new() -> Self {
        Self {
            title: "TUI App".to_string(),
            theme: Theme::default(),
            root_component: None,
            backend: Box::new(PlaceholderBackend::new()),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new application instance with libnotcurses backend.
    #[cfg(feature = "notcurses")]
    pub fn with_notcurses() -> Self {
        Self {
            title: "TUI App".to_string(),
            theme: Theme::default(),
            root_component: None,
            backend: Box::new(NotcursesBackend::new()),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Set the application title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the application theme.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Set the root component.
    pub fn component<C: Component + 'static>(mut self, component: C) -> Self {
        self.root_component = Some(Box::new(component));
        self
    }

    /// Initialize the application.
    pub async fn init(&mut self) -> Result<()> {
        // Initialize the backend
        self.backend.init()?;
        Ok(())
    }

    /// Run the application event loop.
    pub async fn run(mut self) -> Result<()> {
        self.init().await?;

        {
            let mut running = self.running.write().await;
            *running = true;
        }

        // Main event loop
        while *self.running.read().await {
            // Handle events
            if let Some(event) = self.poll_event().await? {
                self.handle_event(event).await?;
            }

            // Render frame
            self.render_frame().await?;

            // Small delay to prevent busy waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await; // ~60 FPS
        }

        self.cleanup().await?;
        Ok(())
    }

    /// Stop the application.
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
    }

    /// Poll for the next event.
    async fn poll_event(&mut self) -> Result<Option<Event>> {
        self.backend.poll_event()
    }

    /// Handle an event.
    async fn handle_event(&self, event: Event) -> Result<()> {
        // Handle built-in events (like quit)
        if let Event::Key(key_event) = &event {
            if key_event.key == crate::event::types::NcKey::Esc {
                self.stop().await;
                return Ok(());
            }
        }

        Ok(())
    }

    /// Render a frame.
    async fn render_frame(&mut self) -> Result<()> {
        if let Some(root_component) = &self.root_component {
            // Get terminal size
            let terminal_size = self.backend.size()?;

            // Create render context
            let context = RenderContext::new(&self.theme).with_viewport_size(terminal_size);

            // Render the component to get virtual DOM
            let vdom = root_component.render(&context).await?;

            // Compute layout
            let layout_rect = Rect::new(crate::layout::Position::new(0, 0), terminal_size);

            // Clear the screen
            self.backend.clear()?;

            // Render the virtual DOM
            self.backend.render_node(&vdom, layout_rect)?;

            // Present to screen
            self.backend.present()?;
        }
        Ok(())
    }

    /// Clean up resources.
    async fn cleanup(&mut self) -> Result<()> {
        self.backend.cleanup()?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_creation() {
        let app = App::new().title("Test App");
        assert_eq!(app.title, "Test App");
    }

    #[tokio::test]
    async fn test_app_init() {
        let _app = App::new();
        // Note: This will fail without a proper terminal, but tests the structure
        // In a real test environment, we'd mock the renderer
    }
}
