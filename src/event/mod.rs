//! Event system and handling.

pub mod handler;
pub mod types;

pub use handler::EventHandler;
pub use types::{Event, FocusEvent, KeyEvent, MouseEvent};
