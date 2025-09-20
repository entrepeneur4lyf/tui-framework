//! Error types and handling for the TUI framework.

use std::fmt;
use thiserror::Error;

/// The main error type for the TUI framework.
#[derive(Error, Debug)]
pub enum Error {
    /// Error from the underlying notcurses library
    #[error("Notcurses error: {message}")]
    #[cfg(feature = "notcurses")]
    /// Notcurses error
    Notcurses {
        /// Error message
        message: String,
    },

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Component not found
    #[error("Component with id '{id}' not found")]
    ComponentNotFound {
        /// The ID of the component that was not found
        id: String,
    },

    /// Invalid component state
    #[error("Invalid component state: {message}")]
    InvalidState {
        /// Description of the invalid state
        message: String,
    },

    /// Rendering error
    #[error("Rendering error: {message}")]
    Render {
        /// Description of the rendering error
        message: String,
    },

    /// Layout error
    #[error("Layout error: {message}")]
    Layout {
        /// Description of the layout error
        message: String,
    },

    /// Style parsing error
    #[error("Style parsing error: {message}")]
    StyleParsing {
        /// Description of the style parsing error
        message: String,
    },

    /// Event handling error
    #[error("Event handling error: {message}")]
    EventHandling {
        /// Description of the event handling error
        message: String,
    },

    /// Context error
    #[error("Context error: {message}")]
    Context {
        /// Description of the context error
        message: String,
    },

    /// Hook error (e.g., using hooks outside of component)
    #[error("Hook error: {message}")]
    Hook {
        /// Description of the hook error
        message: String,
    },

    /// Generic framework error
    #[error("Framework error: {message}")]
    Framework {
        /// Description of the framework error
        message: String,
    },

    /// Async runtime error
    #[error("Async runtime error: {0}")]
    Runtime(#[from] tokio::task::JoinError),

    /// Custom error for user-defined errors
    #[error("Custom error: {message}")]
    Custom {
        /// Description of the custom error
        message: String,
    },
}

impl Error {
    /// Create a new component not found error
    pub fn component_not_found(id: impl Into<String>) -> Self {
        Self::ComponentNotFound { id: id.into() }
    }

    /// Create a new invalid state error
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState {
            message: message.into(),
        }
    }

    /// Create a new render error
    pub fn render(message: impl Into<String>) -> Self {
        Self::Render {
            message: message.into(),
        }
    }

    /// Create a new layout error
    pub fn layout(message: impl Into<String>) -> Self {
        Self::Layout {
            message: message.into(),
        }
    }

    /// Create a new style parsing error
    pub fn style_parsing(message: impl Into<String>) -> Self {
        Self::StyleParsing {
            message: message.into(),
        }
    }

    /// Create a new event handling error
    pub fn event_handling(message: impl Into<String>) -> Self {
        Self::EventHandling {
            message: message.into(),
        }
    }

    /// Create a new context error
    pub fn context(message: impl Into<String>) -> Self {
        Self::Context {
            message: message.into(),
        }
    }

    /// Create a new hook error
    pub fn hook(message: impl Into<String>) -> Self {
        Self::Hook {
            message: message.into(),
        }
    }

    /// Create a new framework error
    pub fn framework(message: impl Into<String>) -> Self {
        Self::Framework {
            message: message.into(),
        }
    }

    /// Create a new custom error
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }
}

/// Result type alias for the TUI framework.
pub type Result<T> = std::result::Result<T, Error>;

/// Extension trait for converting results to framework results.
pub trait ResultExt<T> {
    /// Convert to a framework result with context
    fn with_context(self, message: impl Into<String>) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: fmt::Display,
{
    fn with_context(self, message: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::framework(format!("{}: {}", message.into(), e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::component_not_found("test-id");
        assert!(matches!(err, Error::ComponentNotFound { .. }));

        let err = Error::invalid_state("test message");
        assert!(matches!(err, Error::InvalidState { .. }));
    }

    #[test]
    fn test_result_ext() {
        let result: std::result::Result<i32, &str> = Err("test error");
        let framework_result = result.with_context("operation failed");
        assert!(framework_result.is_err());
    }
}
