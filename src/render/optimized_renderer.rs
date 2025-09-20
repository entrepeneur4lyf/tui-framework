//! Performance-optimized renderer with dirty tracking, caching, and batching.

use crate::component::{Component, ComponentId};
use crate::error::Result;
use crate::event::Event;
use crate::layout::Layout;
use crate::layout::cache::{LayoutCache, LayoutCacheKey};
use crate::render::backend::Backend;
use crate::render::batch::{BatchedRenderer, RenderOperation, RenderCoordinator};
use crate::render::context::RenderContext;
use crate::render::dirty_tracking::DirtyTracker;

use crate::utils::memory_pool::pools;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(not(feature = "notcurses"))]
use crate::render::backend::PlaceholderBackend;

#[cfg(feature = "notcurses")]
use crate::render::backend::NotcursesBackend;

/// Performance-optimized renderer with advanced caching and batching.
pub struct OptimizedRenderer {
    /// The rendering backend
    backend: Box<dyn Backend>,
    /// Dirty tracking system
    dirty_tracker: Arc<DirtyTracker>,
    /// Layout cache for performance
    layout_cache: LayoutCache,
    /// Batched renderer for efficient operations
    batched_renderer: BatchedRenderer,
    /// Render coordinator for frame management
    render_coordinator: RenderCoordinator,
    /// Memory pools for object reuse
    element_pool: pools::VirtualElementPool,
    /// Performance metrics
    metrics: PerformanceMetrics,
    /// Last render time for profiling
    last_render_time: Option<Instant>,
}

impl OptimizedRenderer {
    /// Create a new optimized renderer.
    pub async fn new() -> Result<Self> {
        #[cfg(feature = "notcurses")]
        let backend: Box<dyn Backend> = Box::new(NotcursesBackend::new());

        #[cfg(not(feature = "notcurses"))]
        let backend: Box<dyn Backend> = Box::new(PlaceholderBackend::new());

        Ok(Self {
            backend,
            dirty_tracker: Arc::new(DirtyTracker::new()),
            layout_cache: LayoutCache::new(1000, Duration::from_secs(60)),
            batched_renderer: BatchedRenderer::new(Duration::from_millis(16), 100),
            render_coordinator: RenderCoordinator::new(60), // 60 FPS target
            element_pool: pools::create_virtual_element_pool(),
            metrics: PerformanceMetrics::default(),
            last_render_time: None,
        })
    }

    /// Initialize the renderer.
    pub async fn init(&mut self, title: &str) -> Result<()> {
        self.backend.init()?;
        self.metrics.initialization_time = Some(Instant::now());
        // Log initialization (using println for now since log crate not available)
        println!("Optimized renderer initialized: {}", title);
        Ok(())
    }

    /// Render a component with performance optimizations.
    pub async fn render(
        &mut self,
        component: &dyn Component,
        context: &RenderContext,
    ) -> Result<()> {
        let render_start = Instant::now();
        
        // Get terminal size
        let terminal_size = self.backend.size()?;
        
        // Check if viewport changed
        if self.dirty_tracker.viewport_changed(terminal_size) {
            self.dirty_tracker.update_viewport(terminal_size);
            self.dirty_tracker.mark_dirty(component.id());
            self.layout_cache.clear(); // Clear cache on viewport change
        }

        // Get the virtual node from the component
        let vnode = component.render(context).await?;
        
        // Check if VDOM changed significantly
        let vdom_changed = self.dirty_tracker.vdom_changed(&vnode);
        
        if !vdom_changed && !self.dirty_tracker.is_dirty(component.id()) {
            // No changes, skip rendering
            self.metrics.skipped_frames += 1;
            return Ok(());
        }

        // Try to get cached layout
        let cache_key = LayoutCacheKey::new(terminal_size, &vnode, Some(component.id()));
        let _layout_result = if let Some(cached) = self.layout_cache.get(&cache_key) {
            self.metrics.layout_cache_hits += 1;
            cached
        } else {
            // Compute new layout
            let layout_start = Instant::now();
            let mut vnode_mut = vnode.clone();
            let result = Layout::compute(&mut vnode_mut, terminal_size);
            self.metrics.layout_computation_time += layout_start.elapsed();
            self.metrics.layout_cache_misses += 1;
            
            // Cache the result
            self.layout_cache.put(cache_key, result.clone());
            result
        };

        // Add render operations to batch
        self.batched_renderer.add_operation(
            RenderOperation::Clear(crate::layout::Rect::from_coords(
                0, 0, terminal_size.width, terminal_size.height
            )),
            255, // High priority
        );

        self.batched_renderer.add_operation(
            RenderOperation::RenderNode {
                node: vnode.clone(),
                rect: crate::layout::Rect::from_coords(
                    0, 0, terminal_size.width, terminal_size.height
                ),
                component_id: Some(component.id()),
            },
            128, // Medium priority
        );

        self.batched_renderer.add_operation(
            RenderOperation::Present,
            255, // High priority
        );

        // Execute batched operations
        self.batched_renderer.flush_current_batch();
        self.batched_renderer.execute_batches(&mut *self.backend)?;

        // Update tracking
        self.dirty_tracker.update_vdom(vnode);
        self.dirty_tracker.clear_dirty(component.id());

        // Update metrics
        self.metrics.total_frames += 1;
        self.metrics.total_render_time += render_start.elapsed();
        self.last_render_time = Some(render_start);

        Ok(())
    }

    /// Render multiple components efficiently.
    pub async fn render_batch(
        &mut self,
        components: &[(ComponentId, &dyn Component)],
        context: &RenderContext,
    ) -> Result<()> {
        let batch_start = Instant::now();
        
        // Mark all components as potentially dirty
        for &(component_id, _) in components {
            if self.dirty_tracker.is_dirty(component_id) {
                continue; // Already marked
            }
        }

        // Render each component
        for &(_, component) in components {
            self.render(component, context).await?;
        }

        self.metrics.batch_render_time += batch_start.elapsed();
        Ok(())
    }

    /// Poll for events.
    pub async fn poll_event(&mut self) -> Result<Option<Event>> {
        self.backend.poll_event()
    }

    /// Force a full re-render (clears all caches).
    pub async fn force_refresh(&mut self) -> Result<()> {
        self.dirty_tracker.clear_all_dirty();
        self.layout_cache.clear();
        self.batched_renderer.clear_pending();
        self.metrics.forced_refreshes += 1;
        Ok(())
    }

    /// Get performance metrics.
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Get cache statistics.
    pub fn get_cache_stats(&self) -> (f64, usize) {
        (self.layout_cache.hit_ratio(), self.layout_cache.size())
    }

    /// Get dirty tracking statistics.
    pub fn get_dirty_stats(&self) -> crate::render::dirty_tracking::DirtyStats {
        self.dirty_tracker.get_stats()
    }

    /// Cleanup stale cache entries.
    pub fn cleanup_caches(&mut self) {
        self.layout_cache.cleanup_stale();
        self.element_pool.cleanup_stale();
    }

    /// Set target frame rate.
    pub fn set_target_fps(&mut self, fps: u32) {
        self.render_coordinator.set_target_fps(fps);
    }

    /// Clean up the renderer.
    pub async fn cleanup(&mut self) -> Result<()> {
        self.backend.cleanup()?;
        self.cleanup_caches();
        
        // Log final metrics
        // Log final metrics (using println for now since log crate not available)
        println!("Renderer cleanup - Final metrics: {:?}", self.metrics);
        Ok(())
    }
}

/// Performance metrics for the optimized renderer.
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// When the renderer was initialized
    pub initialization_time: Option<Instant>,
    /// Total number of frames rendered
    pub total_frames: u64,
    /// Number of frames skipped due to no changes
    pub skipped_frames: u64,
    /// Total time spent rendering
    pub total_render_time: Duration,
    /// Time spent on layout computation
    pub layout_computation_time: Duration,
    /// Time spent on batch rendering
    pub batch_render_time: Duration,
    /// Number of layout cache hits
    pub layout_cache_hits: u64,
    /// Number of layout cache misses
    pub layout_cache_misses: u64,
    /// Number of forced refreshes
    pub forced_refreshes: u64,
}

impl PerformanceMetrics {
    /// Get average frame time.
    pub fn avg_frame_time(&self) -> Duration {
        if self.total_frames == 0 {
            Duration::ZERO
        } else {
            self.total_render_time / self.total_frames as u32
        }
    }

    /// Get frames per second.
    pub fn fps(&self) -> f64 {
        let avg_frame_time = self.avg_frame_time();
        if avg_frame_time.is_zero() {
            0.0
        } else {
            1.0 / avg_frame_time.as_secs_f64()
        }
    }

    /// Get cache hit ratio.
    pub fn cache_hit_ratio(&self) -> f64 {
        let total_requests = self.layout_cache_hits + self.layout_cache_misses;
        if total_requests == 0 {
            0.0
        } else {
            self.layout_cache_hits as f64 / total_requests as f64
        }
    }

    /// Get skip ratio (efficiency metric).
    pub fn skip_ratio(&self) -> f64 {
        let total_attempts = self.total_frames + self.skipped_frames;
        if total_attempts == 0 {
            0.0
        } else {
            self.skipped_frames as f64 / total_attempts as f64
        }
    }

    /// Reset all metrics.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::BaseComponent;
    use crate::render::vdom::{VirtualNode, nodes::text};
    use crate::style::Theme;

    struct TestComponent {
        base: BaseComponent,
    }

    impl TestComponent {
        fn new() -> Self {
            Self {
                base: BaseComponent::new("test"),
            }
        }
    }

    #[async_trait::async_trait]
    impl Component for TestComponent {
        fn id(&self) -> ComponentId { self.base.id() }
        fn name(&self) -> &str { self.base.name() }
        
        async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
            Ok(text("Test"))
        }
        
        fn as_any(&self) -> &dyn std::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    }

    #[tokio::test]
    async fn test_optimized_renderer_creation() {
        let renderer = OptimizedRenderer::new().await;
        assert!(renderer.is_ok());
    }

    #[tokio::test]
    async fn test_render_with_caching() {
        let mut renderer = OptimizedRenderer::new().await.unwrap();
        let component = TestComponent::new();
        let theme = Theme::default();
        let context = RenderContext::new(&theme);

        // First render
        let result = renderer.render(&component, &context).await;
        assert!(result.is_ok());
        assert_eq!(renderer.metrics.total_frames, 1);

        // Second render (should be skipped due to no changes)
        let result = renderer.render(&component, &context).await;
        assert!(result.is_ok());
        assert_eq!(renderer.metrics.skipped_frames, 1);
    }
}
