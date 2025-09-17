//! Reactive state management system.

pub mod context;
pub mod hooks;
pub mod state;

pub use context::{Context, ContextProvider, use_context};
pub use hooks::{use_effect, use_memo};
pub use state::{State, use_state};

// Re-export for convenience
pub use context::Context as ReactiveContext;
