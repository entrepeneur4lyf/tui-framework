# Performance Guide

This guide covers performance optimization techniques for building fast and responsive TUI applications.

## Overview

TUI Framework is designed for high performance, but there are several techniques you can use to optimize your applications:

- **Efficient rendering**: Minimize unnecessary re-renders
- **State optimization**: Use appropriate state management patterns
- **Memory management**: Avoid memory leaks and excessive allocations
- **Layout optimization**: Efficient layout calculations
- **Event handling**: Optimize event processing

## Rendering Performance

### Minimize Re-renders

Use memoization to prevent unnecessary re-renders:

```rust
use tui_framework::prelude::*;
use tui_framework::render::vdom::nodes::*;

async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (items, set_items) = use_state(vec![1, 2, 3, 4, 5]);
    let (filter, set_filter) = use_state(String::new());
    
    // Memoize expensive filtering operation
    let filtered_items = use_memo(
        move || {
            items.get().iter()
                .filter(|&&x| x.to_string().contains(&*filter.get()))
                .cloned()
                .collect::<Vec<_>>()
        },
        vec![items.get().clone(), filter.get().clone()]
    );
    
    Ok(div()
        .child(input()
            .value(&filter.get())
            .on_change(move |value| set_filter.set(value)))
        .children(
            filtered_items.get().iter()
                .map(|&item| text(&item.to_string()))
                .collect()
        ))
}
```

### Conditional Rendering

Only render components when necessary:

```rust
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (show_expensive, set_show_expensive) = use_state(false);
    let (data, _) = use_state(vec![0; 1000]); // Large dataset
    
    Ok(div()
        .child(button("Toggle Expensive View")
            .on_click(move |_| set_show_expensive.set(!*show_expensive.get())))
        .child(
            if *show_expensive.get() {
                // Only render when needed
                ExpensiveListComponent::new(data.get().clone())
                    .render(_context).await?
            } else {
                text("Click to show expensive view")
            }
        ))
}
```

### Virtual Scrolling

For large lists, implement virtual scrolling:

```rust
#[derive(Clone)]
struct VirtualList {
    base: BaseComponent,
    items: Vec<String>,
    item_height: u16,
}

impl VirtualList {
    fn new(items: Vec<String>) -> Self {
        Self {
            base: BaseComponent::new("VirtualList"),
            items,
            item_height: 1,
        }
    }
}

#[async_trait]
impl Component for VirtualList {
    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let (scroll_offset, set_scroll_offset) = use_state(0);
        
        let viewport_height = context.viewport_size.height;
        let visible_count = (viewport_height / self.item_height) as usize;
        let start_index = *scroll_offset.get();
        let end_index = (start_index + visible_count).min(self.items.len());
        
        // Only render visible items
        let visible_items: Vec<VirtualNode> = self.items[start_index..end_index]
            .iter()
            .enumerate()
            .map(|(i, item)| {
                div()
                    .style(&format!("height: {};", self.item_height))
                    .child(text(&format!("{}: {}", start_index + i, item)))
            })
            .collect();
        
        Ok(div()
            .style("overflow-y: scroll;")
            .on_scroll(move |scroll_event| {
                let new_offset = (scroll_event.delta / self.item_height as i32) as usize;
                set_scroll_offset.set(new_offset.min(self.items.len().saturating_sub(visible_count)));
            })
            .children(visible_items))
    }
}
```

## State Management Performance

### Use Appropriate State Granularity

Break down large state objects into smaller, focused pieces:

```rust
// Instead of one large state object
struct AppState {
    user: User,
    settings: Settings,
    data: Vec<Item>,
    ui: UiState,
}

// Use separate state for different concerns
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (user, set_user) = use_state(User::default());
    let (settings, set_settings) = use_state(Settings::default());
    let (data, set_data) = use_state(Vec::<Item>::new());
    let (ui_state, set_ui_state) = use_state(UiState::default());
    
    // Components only re-render when their specific state changes
    Ok(div()
        .child(UserComponent::new(user.get().clone()).render(_context).await?)
        .child(SettingsComponent::new(settings.get().clone()).render(_context).await?)
        .child(DataComponent::new(data.get().clone()).render(_context).await?))
}
```

### Optimize State Updates

Batch state updates when possible:

```rust
async fn handle_bulk_update(&self) {
    let (items, set_items) = use_state(Vec::<Item>::new());
    
    // Instead of multiple individual updates
    // for item in new_items {
    //     set_items.update(|items| items.push(item));
    // }
    
    // Batch the update
    set_items.update(|items| {
        items.extend(new_items);
    });
}
```

### Use Derived State Efficiently

Create derived state that only updates when dependencies change:

```rust
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (items, set_items) = use_state(vec![1, 2, 3, 4, 5]);
    let (multiplier, set_multiplier) = use_state(2);
    
    // Derived state - only recalculates when dependencies change
    let processed_items = use_memo(
        move || {
            items.get().iter()
                .map(|x| x * *multiplier.get())
                .collect::<Vec<_>>()
        },
        vec![items.get().clone(), *multiplier.get()]
    );
    
    // Statistics only recalculate when processed_items change
    let stats = use_memo(
        move || {
            let sum: i32 = processed_items.get().iter().sum();
            let avg = sum as f64 / processed_items.get().len() as f64;
            (sum, avg)
        },
        vec![processed_items.get().clone()]
    );
    
    Ok(div()
        .child(text(&format!("Sum: {}, Average: {:.2}", stats.get().0, stats.get().1))))
}
```

## Memory Management

### Avoid Memory Leaks

Clean up resources in component lifecycle methods:

```rust
#[derive(Clone)]
struct TimerComponent {
    base: BaseComponent,
    timer_handle: Option<tokio::task::JoinHandle<()>>,
}

#[async_trait]
impl Component for TimerComponent {
    async fn on_mount(&mut self) -> Result<()> {
        // Start timer
        let handle = tokio::spawn(async {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                // Timer logic
            }
        });
        self.timer_handle = Some(handle);
        Ok(())
    }
    
    async fn on_unmount(&mut self) -> Result<()> {
        // Clean up timer
        if let Some(handle) = self.timer_handle.take() {
            handle.abort();
        }
        Ok(())
    }
}
```

### Efficient String Handling

Minimize string allocations:

```rust
// Instead of creating new strings repeatedly
fn format_item(item: &Item, index: usize) -> String {
    format!("{}: {}", index, item.name)
}

// Use string caching or pre-computed strings
struct ItemFormatter {
    cache: HashMap<(usize, String), String>,
}

impl ItemFormatter {
    fn format_item(&mut self, item: &Item, index: usize) -> &str {
        let key = (index, item.name.clone());
        self.cache.entry(key.clone()).or_insert_with(|| {
            format!("{}: {}", key.0, key.1)
        })
    }
}
```

### Object Pooling

Reuse expensive objects:

```rust
struct ComponentPool<T> {
    available: Vec<T>,
    in_use: Vec<T>,
}

impl<T: Clone + Default> ComponentPool<T> {
    fn new() -> Self {
        Self {
            available: Vec::new(),
            in_use: Vec::new(),
        }
    }
    
    fn acquire(&mut self) -> T {
        self.available.pop().unwrap_or_else(|| {
            T::default()
        })
    }
    
    fn release(&mut self, item: T) {
        self.available.push(item);
    }
}
```

## Layout Performance

### Cache Layout Calculations

```rust
struct LayoutCache {
    cache: HashMap<(Size, Vec<VirtualNode>), LayoutResult>,
}

impl LayoutCache {
    fn get_or_compute(&mut self, viewport: Size, nodes: &[VirtualNode]) -> LayoutResult {
        let key = (viewport, nodes.to_vec());
        
        if let Some(cached) = self.cache.get(&key) {
            return cached.clone();
        }
        
        let result = compute_layout(viewport, nodes);
        self.cache.insert(key, result.clone());
        result
    }
}
```

### Optimize Layout Algorithms

Use efficient layout algorithms for complex UIs:

```rust
// Instead of recalculating entire layout
fn incremental_layout_update(
    previous_layout: &LayoutResult,
    changes: &[LayoutChange],
) -> LayoutResult {
    let mut new_layout = previous_layout.clone();
    
    for change in changes {
        match change {
            LayoutChange::Resize(node_id, new_size) => {
                // Only update affected nodes
                update_node_and_children(&mut new_layout, *node_id, *new_size);
            }
            LayoutChange::Move(node_id, new_position) => {
                // Only update position
                update_node_position(&mut new_layout, *node_id, *new_position);
            }
        }
    }
    
    new_layout
}
```

## Event Handling Performance

### Debounce Expensive Operations

```rust
use std::time::{Duration, Instant};

struct Debouncer {
    last_call: Option<Instant>,
    delay: Duration,
}

impl Debouncer {
    fn new(delay: Duration) -> Self {
        Self {
            last_call: None,
            delay,
        }
    }
    
    fn should_execute(&mut self) -> bool {
        let now = Instant::now();
        
        if let Some(last) = self.last_call {
            if now.duration_since(last) < self.delay {
                return false;
            }
        }
        
        self.last_call = Some(now);
        true
    }
}

async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (search_term, set_search_term) = use_state(String::new());
    let (debouncer, _) = use_state(Debouncer::new(Duration::from_millis(300)));
    
    Ok(input()
        .value(&search_term.get())
        .on_change(move |value| {
            set_search_term.set(value.clone());
            
            // Only perform expensive search after debounce delay
            if debouncer.get().should_execute() {
                perform_expensive_search(value);
            }
        }))
}
```

### Optimize Event Propagation

```rust
// Stop event propagation when handled
button("Click me")
    .on_click(move |event| {
        handle_click();
        event.stop_propagation(); // Prevent further processing
    })
```

## Profiling and Monitoring

### Performance Metrics

```rust
use std::time::Instant;

struct PerformanceMetrics {
    render_times: Vec<Duration>,
    layout_times: Vec<Duration>,
    event_times: Vec<Duration>,
}

impl PerformanceMetrics {
    fn record_render_time<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        self.render_times.push(start.elapsed());
        result
    }
    
    fn average_render_time(&self) -> Duration {
        if self.render_times.is_empty() {
            return Duration::ZERO;
        }
        
        let total: Duration = self.render_times.iter().sum();
        total / self.render_times.len() as u32
    }
}
```

### Memory Usage Monitoring

```rust
#[cfg(debug_assertions)]
fn log_memory_usage() {
    use std::alloc::{GlobalAlloc, Layout, System};
    
    struct TrackingAllocator;
    
    unsafe impl GlobalAlloc for TrackingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = System.alloc(layout);
            println!("Allocated {} bytes", layout.size());
            ptr
        }
        
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            println!("Deallocated {} bytes", layout.size());
            System.dealloc(ptr, layout);
        }
    }
}
```

## Performance Testing

### Benchmark Components

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[tokio::test]
    async fn benchmark_component_render() {
        let component = LargeListComponent::new(1000);
        let theme = Theme::default();
        let context = RenderContext::new(&theme);
        
        let start = Instant::now();
        let result = component.render(&context).await;
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        assert!(duration < Duration::from_millis(100), "Render took too long: {:?}", duration);
    }
    
    #[tokio::test]
    async fn benchmark_state_updates() {
        let (state, set_state) = use_state(0);
        
        let start = Instant::now();
        for i in 0..1000 {
            set_state.set(i);
        }
        let duration = start.elapsed();
        
        assert!(duration < Duration::from_millis(50), "State updates took too long: {:?}", duration);
    }
}
```

### Load Testing

```rust
#[tokio::test]
async fn stress_test_large_dataset() {
    let large_dataset: Vec<Item> = (0..10000)
        .map(|i| Item::new(format!("Item {}", i)))
        .collect();
    
    let component = DataTableComponent::new(large_dataset);
    let theme = Theme::default();
    let context = RenderContext::new(&theme);
    
    // Test rendering performance with large dataset
    let start = Instant::now();
    let result = component.render(&context).await;
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    assert!(duration < Duration::from_millis(500), "Large dataset render took too long");
}
```

## Best Practices

1. **Profile Early**: Use profiling tools to identify bottlenecks
2. **Measure Everything**: Track render times, memory usage, and event handling
3. **Optimize Hot Paths**: Focus on code that runs frequently
4. **Use Memoization**: Cache expensive computations
5. **Batch Updates**: Group state changes together
6. **Clean Up Resources**: Always clean up in lifecycle methods
7. **Test Performance**: Include performance tests in your test suite
8. **Monitor in Production**: Track performance metrics in deployed applications
9. **Optimize Gradually**: Don't optimize prematurely, measure first
10. **Consider Trade-offs**: Balance performance with code maintainability

## Common Performance Pitfalls

### 1. Excessive Re-renders

```rust
// Bad: Creates new closure on every render
button("Click").on_click(|| println!("Clicked"))

// Good: Use stable references
let handler = use_callback(|| println!("Clicked"), vec![]);
button("Click").on_click(handler)
```

### 2. Large State Objects

```rust
// Bad: Large monolithic state
let (app_state, set_app_state) = use_state(LargeAppState::default());

// Good: Split into focused state
let (user, set_user) = use_state(User::default());
let (settings, set_settings) = use_state(Settings::default());
```

### 3. Inefficient List Rendering

```rust
// Bad: Renders all items
items.iter().map(|item| render_item(item)).collect()

// Good: Virtual scrolling for large lists
render_visible_items(items, viewport, scroll_offset)
```

### 4. Memory Leaks

```rust
// Bad: No cleanup
async fn on_mount(&mut self) -> Result<()> {
    start_timer();
    Ok(())
}

// Good: Proper cleanup
async fn on_unmount(&mut self) -> Result<()> {
    stop_timer();
    Ok(())
}
```
