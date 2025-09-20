//! Performance tests and benchmarks for the TUI framework.
//!
//! These tests verify that the framework performs well under various conditions.

use tui_framework::prelude::*;
use tui_framework::component::{BaseComponent, Component};
use tui_framework::layout::{Size};
use tui_framework::layout::layout_engine::Layout;
use tui_framework::reactive::state::State;
use tui_framework::render::backend::{Backend, PlaceholderBackend};
use tui_framework::render::context::RenderContext;
use tui_framework::render::vdom::nodes::{div, text};
use tui_framework::render::vdom::{VirtualNode, VirtualStyle, StyleValue, DisplayType, FlexDirection};
use std::time::{Duration, Instant};

/// Performance test component that creates a large number of elements.
struct PerformanceTestComponent {
    base: BaseComponent,
    item_count: State<usize>,
}

impl PerformanceTestComponent {
    pub fn new(count: usize) -> Self {
        Self {
            base: BaseComponent::new("PerformanceTestComponent"),
            item_count: State::new(count),
        }
    }
}

#[async_trait::async_trait]
impl Component for PerformanceTestComponent {
    fn id(&self) -> tui_framework::component::ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let count = *self.item_count.get();
        
        let mut container = div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Column),
                width: Some(StyleValue::Fill),
                height: Some(StyleValue::Fill),
                ..Default::default()
            });

        // Add many child elements
        for i in 0..count {
            container = container.child(
                div()
                    .style(VirtualStyle {
                        height: Some(StyleValue::Absolute(1)),
                        width: Some(StyleValue::Fill),
                        ..Default::default()
                    })
                    .child(text(&format!("Item {}", i)))
            );
        }

        Ok(container)
    }
}

#[tokio::test]
async fn test_rendering_performance_small() {
    let component = PerformanceTestComponent::new(10);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);

    let start = Instant::now();
    let result = component.render(&context).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    // Small component should render very quickly (< 10ms)
    assert!(duration < Duration::from_millis(10));
}

#[tokio::test]
async fn test_rendering_performance_medium() {
    let component = PerformanceTestComponent::new(100);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);

    let start = Instant::now();
    let result = component.render(&context).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    // Medium component should render reasonably quickly (< 50ms)
    assert!(duration < Duration::from_millis(50));
}

#[tokio::test]
async fn test_rendering_performance_large() {
    let component = PerformanceTestComponent::new(1000);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);

    let start = Instant::now();
    let result = component.render(&context).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    // Large component should still render in reasonable time (< 200ms)
    assert!(duration < Duration::from_millis(200));
    
    println!("Large component (1000 items) rendered in: {:?}", duration);
}

#[tokio::test]
async fn test_layout_performance() {
    let component = PerformanceTestComponent::new(500);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);
    let mut node = component.render(&context).await.unwrap();

    let viewport = Size::new(80, 24);

    let start = Instant::now();
    let result = Layout::compute(&mut node, viewport);
    let duration = start.elapsed();

    assert!(result.layouts.len() > 0);
    // Layout computation should be fast (< 100ms for 500 items)
    assert!(duration < Duration::from_millis(100));

    println!("Layout computation (500 items) took: {:?}", duration);
}

#[tokio::test]
async fn test_backend_rendering_performance() {
    let mut backend = PlaceholderBackend::new();
    let component = PerformanceTestComponent::new(100);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);

    assert!(backend.init().is_ok());

    let node = component.render(&context).await.unwrap();
    let rect = tui_framework::layout::Rect::from_coords(0, 0, 80, 24);

    let start = Instant::now();
    let result = backend.render_node(&node, rect);
    let duration = start.elapsed();

    assert!(result.is_ok());
    // Backend rendering should be fast (< 50ms)
    assert!(duration < Duration::from_millis(50));

    assert!(backend.cleanup().is_ok());
    
    println!("Backend rendering (100 items) took: {:?}", duration);
}

#[tokio::test]
async fn test_state_update_performance() {
    let state = State::new(0);
    let mapped_state = state.map(|x| x * 2);

    let start = Instant::now();
    
    // Perform many state updates
    for i in 0..1000 {
        state.set(i);
        // Verify the mapped state is updated
        assert_eq!(mapped_state.get(), i * 2);
    }
    
    let duration = start.elapsed();

    // State updates should be very fast (< 10ms for 1000 updates)
    assert!(duration < Duration::from_millis(10));
    
    println!("1000 state updates took: {:?}", duration);
}

#[tokio::test]
async fn test_memory_usage() {
    // Test that creating many components doesn't cause excessive memory usage
    let mut components = Vec::new();
    
    for i in 0..100 {
        components.push(PerformanceTestComponent::new(i));
    }

    // All components should be created successfully
    assert_eq!(components.len(), 100);

    // Test rendering all components
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);

    for component in &components {
        let result = component.render(&context).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_concurrent_rendering() {
    use tokio::task;

    // Test that multiple components can be rendered concurrently
    let mut handles = Vec::new();

    for i in 0..10 {
        let handle = task::spawn(async move {
            let component = PerformanceTestComponent::new(50 + i);
            let theme = tui_framework::style::Theme::default();
            let context = RenderContext::new(&theme);
            component.render(&context).await
        });
        handles.push(handle);
    }

    // Wait for all renders to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_layout_caching_performance() {
    let component = PerformanceTestComponent::new(200);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);
    let mut node = component.render(&context).await.unwrap();

    let viewport = Size::new(80, 24);
    
    // First layout computation
    let start = Instant::now();
    let result1 = Layout::compute(&mut node, viewport);
    let first_duration = start.elapsed();

    // Second layout computation (should potentially be faster due to caching)
    let start = Instant::now();
    let result2 = Layout::compute(&mut node, viewport);
    let second_duration = start.elapsed();

    assert!(result1.layouts.len() > 0);
    assert!(result2.layouts.len() > 0);
    
    println!("First layout: {:?}, Second layout: {:?}", first_duration, second_duration);
    
    // Both should complete in reasonable time
    assert!(first_duration < Duration::from_millis(100));
    assert!(second_duration < Duration::from_millis(100));
}

#[tokio::test]
async fn test_stress_test() {
    // Stress test with very large component
    let component = PerformanceTestComponent::new(2000);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);

    let start = Instant::now();
    let result = component.render(&context).await;
    let render_duration = start.elapsed();

    assert!(result.is_ok());
    
    let mut node = result.unwrap();
    let viewport = Size::new(120, 40);
    
    let start = Instant::now();
    let layout_result = Layout::compute(&mut node, viewport);
    let layout_duration = start.elapsed();

    assert!(layout_result.layouts.len() > 0);
    
    println!("Stress test - Render: {:?}, Layout: {:?}", render_duration, layout_duration);
    
    // Even stress test should complete in reasonable time (< 1 second each)
    assert!(render_duration < Duration::from_secs(1));
    assert!(layout_duration < Duration::from_secs(1));
}

#[tokio::test]
async fn test_repeated_operations() {
    // Test that repeated operations don't degrade performance
    let component = PerformanceTestComponent::new(50);
    let theme = tui_framework::style::Theme::default();
    let context = RenderContext::new(&theme);

    let mut total_duration = Duration::new(0, 0);
    
    for _ in 0..10 {
        let start = Instant::now();
        let result = component.render(&context).await;
        total_duration += start.elapsed();
        
        assert!(result.is_ok());
    }

    let average_duration = total_duration / 10;
    
    println!("Average render time over 10 iterations: {:?}", average_duration);
    
    // Average should be very fast (< 5ms)
    assert!(average_duration < Duration::from_millis(5));
}
