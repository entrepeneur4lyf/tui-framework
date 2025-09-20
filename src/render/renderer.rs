//! Main renderer implementation.

use crate::component::Component;
use crate::error::Result;
use crate::event::Event;
use crate::layout::Layout;
use crate::render::RenderContext;
use crate::render::backend::Backend;

#[cfg(not(feature = "notcurses"))]
use crate::render::backend::PlaceholderBackend;

#[cfg(feature = "notcurses")]
use crate::render::backend::NotcursesBackend;

/// The main renderer that handles drawing to the terminal.
pub struct Renderer {
    backend: Box<dyn Backend>,
}

impl Renderer {
    /// Create a new renderer with the appropriate backend.
    pub async fn new() -> Result<Self> {
        #[cfg(feature = "notcurses")]
        let backend: Box<dyn Backend> = Box::new(NotcursesBackend::new());

        #[cfg(not(feature = "notcurses"))]
        let backend: Box<dyn Backend> = Box::new(PlaceholderBackend::new());

        Ok(Self { backend })
    }

    /// Initialize the renderer with the given title.
    pub async fn init(&mut self, _title: &str) -> Result<()> {
        self.backend.init()?;
        Ok(())
    }

    /// Render a component to the terminal.
    pub async fn render(
        &mut self,
        component: &dyn Component,
        context: &RenderContext,
    ) -> Result<()> {
        // Get the virtual node from the component
        let vnode = component.render(context).await?;

        // Get terminal size
        let terminal_size = self.backend.size()?;

        // Compute layout
        let mut vnode_mut = vnode;
        let _layout_result = Layout::compute(&mut vnode_mut, terminal_size);

        // Clear the screen
        self.backend.clear()?;

        // For now, just render the root node at the full terminal size
        let root_rect =
            crate::layout::Rect::from_coords(0, 0, terminal_size.width, terminal_size.height);
        self.backend.render_node(&vnode_mut, root_rect)?;

        // Present the rendered content
        self.backend.present()?;

        Ok(())
    }

    /// Poll for the next event.
    pub async fn poll_event(&mut self) -> Result<Option<Event>> {
        self.backend.poll_event()
    }

    /// Clean up the renderer.
    pub async fn cleanup(&mut self) -> Result<()> {
        self.backend.cleanup()?;
        Ok(())
    }
}
