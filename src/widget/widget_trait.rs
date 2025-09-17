//! Widget trait and base functionality.

use crate::component::Component;
use crate::error::Result;
use async_trait::async_trait;

/// Base trait for all widgets.
#[async_trait]
pub trait Widget: Component {
    /// Get the widget type name.
    fn widget_type(&self) -> &'static str;

    /// Handle widget-specific events.
    async fn handle_widget_event(&mut self, _event: &str) -> Result<()> {
        Ok(())
    }
}
