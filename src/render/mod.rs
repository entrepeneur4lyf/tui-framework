//! Rendering system and virtual DOM.

pub mod backend;
pub mod batch;
pub mod context;
pub mod dirty_tracking;
pub mod optimized_renderer;
pub mod renderer;
pub mod vdom;

pub use backend::Backend;
pub use context::RenderContext;
pub use renderer::Renderer;
pub use vdom::{VirtualElement, VirtualNode};
