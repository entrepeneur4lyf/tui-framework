//! Batched rendering system for performance optimization.
//!
//! This module provides batching capabilities to group multiple rendering
//! operations together, reducing the number of expensive backend calls.

use crate::component::ComponentId;
use crate::layout::{Rect, Size};
use crate::render::{RenderContext, VirtualNode};
use crate::render::backend::Backend;
use crate::error::Result;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A batch of rendering operations to be executed together.
#[derive(Debug, Clone)]
pub struct RenderBatch {
    /// Operations in this batch
    pub operations: Vec<RenderOperation>,
    /// When this batch was created
    pub created_at: Instant,
    /// Priority of this batch (higher = more urgent)
    pub priority: u8,
}

impl RenderBatch {
    /// Create a new render batch.
    pub fn new(priority: u8) -> Self {
        Self {
            operations: Vec::new(),
            created_at: Instant::now(),
            priority,
        }
    }

    /// Add an operation to this batch.
    pub fn add_operation(&mut self, operation: RenderOperation) {
        self.operations.push(operation);
    }

    /// Check if this batch is empty.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Get the age of this batch.
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
}

/// A single rendering operation.
#[derive(Debug, Clone)]
pub enum RenderOperation {
    /// Clear a specific region
    Clear(Rect),
    /// Render a virtual node at a specific location
    RenderNode {
        /// The virtual node to render
        node: VirtualNode,
        /// The rectangle to render within
        rect: Rect,
        /// Optional component ID for tracking
        component_id: Option<ComponentId>,
    },
    /// Update a specific component
    UpdateComponent {
        /// The component ID to update
        component_id: ComponentId,
        /// The new virtual node
        node: VirtualNode,
        /// The rectangle to render within
        rect: Rect,
    },
    /// Present the rendered content to screen
    Present,
    /// Set cursor position
    SetCursor {
        /// X coordinate
        x: u16,
        /// Y coordinate
        y: u16
    },
}

/// Batched renderer that groups operations for efficiency.
pub struct BatchedRenderer {
    /// Pending batches waiting to be executed
    pending_batches: Arc<Mutex<VecDeque<RenderBatch>>>,
    /// Current batch being built
    current_batch: Arc<Mutex<Option<RenderBatch>>>,
    /// Maximum time to wait before flushing a batch
    max_batch_time: Duration,
    /// Maximum number of operations per batch
    max_batch_size: usize,
    /// Rendering statistics
    stats: Arc<Mutex<BatchStats>>,
}

impl Default for BatchedRenderer {
    fn default() -> Self {
        Self::new(Duration::from_millis(16), 100) // ~60 FPS, 100 ops per batch
    }
}

impl BatchedRenderer {
    /// Create a new batched renderer.
    pub fn new(max_batch_time: Duration, max_batch_size: usize) -> Self {
        Self {
            pending_batches: Arc::new(Mutex::new(VecDeque::new())),
            current_batch: Arc::new(Mutex::new(Some(RenderBatch::new(0)))),
            max_batch_time,
            max_batch_size,
            stats: Arc::new(Mutex::new(BatchStats::default())),
        }
    }

    /// Add a render operation to the current batch.
    pub fn add_operation(&self, operation: RenderOperation, priority: u8) {
        let mut current_batch = self.current_batch.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        stats.total_operations += 1;

        if let Some(ref mut batch) = *current_batch {
            // Check if we need to start a new batch due to priority change
            if batch.priority != priority && !batch.is_empty() {
                self.flush_current_batch_internal(&mut current_batch, &mut stats);
                *current_batch = Some(RenderBatch::new(priority));
            }

            if let Some(ref mut batch) = *current_batch {
                batch.add_operation(operation);
                batch.priority = batch.priority.max(priority);

                // Check if batch is full or too old
                if batch.operations.len() >= self.max_batch_size 
                    || batch.age() >= self.max_batch_time {
                    self.flush_current_batch_internal(&mut current_batch, &mut stats);
                }
            }
        }
    }

    /// Flush the current batch to the pending queue.
    pub fn flush_current_batch(&self) {
        let mut current_batch = self.current_batch.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        self.flush_current_batch_internal(&mut current_batch, &mut stats);
    }

    /// Internal method to flush current batch.
    fn flush_current_batch_internal(
        &self,
        current_batch: &mut Option<RenderBatch>,
        stats: &mut BatchStats,
    ) {
        if let Some(batch) = current_batch.take() {
            if !batch.is_empty() {
                let mut pending = self.pending_batches.lock().unwrap();
                
                // Insert batch in priority order
                let insert_pos = pending
                    .iter()
                    .position(|b| b.priority < batch.priority)
                    .unwrap_or(pending.len());
                
                pending.insert(insert_pos, batch);
                stats.batches_created += 1;
            }
            *current_batch = Some(RenderBatch::new(0));
        }
    }

    /// Execute all pending batches using the provided backend.
    pub fn execute_batches(&self, backend: &mut dyn Backend) -> Result<()> {
        let mut pending = self.pending_batches.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        while let Some(batch) = pending.pop_front() {
            self.execute_batch(backend, &batch)?;
            stats.batches_executed += 1;
            stats.operations_executed += batch.operations.len() as u64;
        }

        Ok(())
    }

    /// Execute a single batch.
    fn execute_batch(&self, backend: &mut dyn Backend, batch: &RenderBatch) -> Result<()> {
        for operation in &batch.operations {
            match operation {
                RenderOperation::Clear(_rect) => {
                    backend.clear()?;
                }
                RenderOperation::RenderNode { node, rect, .. } => {
                    backend.render_node(node, *rect)?;
                }
                RenderOperation::UpdateComponent { node, rect, .. } => {
                    backend.render_node(node, *rect)?;
                }
                RenderOperation::Present => {
                    backend.present()?;
                }
                RenderOperation::SetCursor { x: _x, y: _y } => {
                    // Cursor positioning not implemented in current backend
                    // This is a placeholder for future cursor support
                }
            }
        }
        Ok(())
    }

    /// Get the number of pending batches.
    pub fn pending_batch_count(&self) -> usize {
        let pending = self.pending_batches.lock().unwrap();
        pending.len()
    }

    /// Get the number of operations in the current batch.
    pub fn current_batch_size(&self) -> usize {
        let current = self.current_batch.lock().unwrap();
        current.as_ref().map_or(0, |b| b.operations.len())
    }

    /// Clear all pending batches.
    pub fn clear_pending(&self) {
        let mut pending = self.pending_batches.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        stats.batches_discarded += pending.len() as u64;
        pending.clear();
    }

    /// Get rendering statistics.
    pub fn get_stats(&self) -> BatchStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Reset statistics.
    pub fn reset_stats(&self) {
        let mut stats = self.stats.lock().unwrap();
        *stats = BatchStats::default();
    }
}

/// Statistics for batched rendering.
#[derive(Debug, Clone, Default)]
pub struct BatchStats {
    /// Total number of operations added
    pub total_operations: u64,
    /// Number of batches created
    pub batches_created: u64,
    /// Number of batches executed
    pub batches_executed: u64,
    /// Number of batches discarded
    pub batches_discarded: u64,
    /// Total operations executed
    pub operations_executed: u64,
}

impl BatchStats {
    /// Get the average operations per batch.
    pub fn avg_operations_per_batch(&self) -> f64 {
        if self.batches_executed == 0 {
            0.0
        } else {
            self.operations_executed as f64 / self.batches_executed as f64
        }
    }

    /// Get the batch execution ratio.
    pub fn batch_execution_ratio(&self) -> f64 {
        if self.batches_created == 0 {
            0.0
        } else {
            self.batches_executed as f64 / self.batches_created as f64
        }
    }
}

/// High-level batched rendering coordinator.
pub struct RenderCoordinator {
    /// The batched renderer
    renderer: BatchedRenderer,
    /// Component update tracking
    dirty_components: HashMap<ComponentId, VirtualNode>,
    /// Last render time for frame rate limiting
    last_render: Option<Instant>,
    /// Target frame rate
    target_fps: u32,
}

impl RenderCoordinator {
    /// Create a new render coordinator.
    pub fn new(target_fps: u32) -> Self {
        Self {
            renderer: BatchedRenderer::default(),
            dirty_components: HashMap::new(),
            last_render: None,
            target_fps,
        }
    }

    /// Mark a component as needing re-render.
    pub fn mark_component_dirty(&mut self, component_id: ComponentId, node: VirtualNode) {
        self.dirty_components.insert(component_id, node);
    }

    /// Render all dirty components and execute batches.
    pub fn render_frame<B: Backend>(
        &mut self,
        backend: &mut B,
        viewport: Size,
        _context: &RenderContext,
    ) -> Result<()> {
        // Check frame rate limiting
        if let Some(last) = self.last_render {
            let frame_time = Duration::from_secs_f64(1.0 / self.target_fps as f64);
            let elapsed = last.elapsed();
            if elapsed < frame_time {
                return Ok(());
            }
        }

        // Clear screen
        let full_rect = Rect::new(
            crate::layout::Position::new(0, 0),
            viewport,
        );
        self.renderer.add_operation(
            RenderOperation::Clear(full_rect),
            255, // High priority
        );

        // Render dirty components
        for (component_id, node) in self.dirty_components.drain() {
            self.renderer.add_operation(
                RenderOperation::UpdateComponent {
                    component_id,
                    node,
                    rect: full_rect, // TODO: Use actual component bounds
                },
                128, // Medium priority
            );
        }

        // Present frame
        self.renderer.add_operation(
            RenderOperation::Present,
            255, // High priority
        );

        // Flush and execute
        self.renderer.flush_current_batch();
        self.renderer.execute_batches(backend)?;

        self.last_render = Some(Instant::now());
        Ok(())
    }

    /// Get rendering statistics.
    pub fn get_stats(&self) -> BatchStats {
        self.renderer.get_stats()
    }

    /// Set target frame rate.
    pub fn set_target_fps(&mut self, fps: u32) {
        self.target_fps = fps;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::backend::PlaceholderBackend;

    #[test]
    fn test_render_batch_creation() {
        let mut batch = RenderBatch::new(128);
        assert!(batch.is_empty());
        assert_eq!(batch.priority, 128);

        let operation = RenderOperation::Present;
        batch.add_operation(operation);
        assert!(!batch.is_empty());
        assert_eq!(batch.operations.len(), 1);
    }

    #[test]
    fn test_batched_renderer() {
        let renderer = BatchedRenderer::new(Duration::from_millis(100), 5);
        
        // Add some operations
        renderer.add_operation(RenderOperation::Present, 128);
        renderer.add_operation(RenderOperation::Present, 128);
        
        assert_eq!(renderer.current_batch_size(), 2);
        assert_eq!(renderer.pending_batch_count(), 0);

        // Flush current batch
        renderer.flush_current_batch();
        assert_eq!(renderer.current_batch_size(), 0);
        assert_eq!(renderer.pending_batch_count(), 1);

        // Execute batches
        let mut backend = PlaceholderBackend::new();
        let result = renderer.execute_batches(&mut backend);
        assert!(result.is_ok());
        assert_eq!(renderer.pending_batch_count(), 0);
    }

    #[test]
    fn test_render_coordinator() {
        let mut coordinator = RenderCoordinator::new(60);
        let component_id = ComponentId::new();
        let node = VirtualNode::text("test");
        
        coordinator.mark_component_dirty(component_id, node);
        assert_eq!(coordinator.dirty_components.len(), 1);

        let mut backend = PlaceholderBackend::new();
        let context = RenderContext::new(&crate::style::Theme::default());
        let viewport = Size::new(80, 24);
        
        let result = coordinator.render_frame(&mut backend, viewport, &context);
        assert!(result.is_ok());
        assert_eq!(coordinator.dirty_components.len(), 0);
    }
}
