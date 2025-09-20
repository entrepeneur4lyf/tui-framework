# Component Development Guide

This guide covers advanced component development patterns and best practices for building robust TUI applications.

## Component Architecture

### Basic Component Structure

Every component should follow this pattern:

```rust
use tui_framework::prelude::*;
use tui_framework::render::vdom::nodes::*;

#[derive(Clone)]
struct MyComponent {
    base: BaseComponent,
    // Component-specific fields
    title: String,
    config: MyConfig,
}

impl MyComponent {
    fn new(title: String, config: MyConfig) -> Self {
        Self {
            base: BaseComponent::new("MyComponent"),
            title,
            config,
        }
    }
}

#[async_trait]
impl Component for MyComponent {
    fn id(&self) -> ComponentId {
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

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        // Rendering logic here
        Ok(div().child(text(&self.title)))
    }

    // Optional lifecycle methods
    async fn on_mount(&mut self) -> Result<()> {
        // Initialization logic
        Ok(())
    }

    async fn on_unmount(&mut self) -> Result<()> {
        // Cleanup logic
        Ok(())
    }
}
```

## State Management

### Local Component State

Use `use_state` for component-local state:

```rust
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (count, set_count) = use_state(0);
    let (name, set_name) = use_state(String::from(""));
    let (items, set_items) = use_state(Vec::<String>::new());

    // Use state in rendering and event handlers
    Ok(div()
        .child(text(&format!("Count: {}", count.get())))
        .child(input()
            .value(&name.get())
            .on_change(move |new_value| set_name.set(new_value))))
}
```

### Shared State with Context

For state shared between components:

```rust
// Define context type
#[derive(Clone)]
struct AppState {
    user: String,
    theme: Theme,
}

// Provide context at app level
let app_state = Context::new(AppState {
    user: "John".to_string(),
    theme: Theme::default(),
});

// Use context in components
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let app_state = use_context::<AppState>()?;
    
    Ok(div()
        .child(text(&format!("Welcome, {}", app_state.user))))
}
```

### Derived State

Create computed state that updates automatically:

```rust
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (items, set_items) = use_state(vec![1, 2, 3, 4, 5]);
    
    // Derived state - automatically updates when items change
    let total = items.map(|items| items.iter().sum::<i32>());
    let count = items.map(|items| items.len());
    
    Ok(div()
        .child(text(&format!("Items: {}", count.get())))
        .child(text(&format!("Total: {}", total.get()))))
}
```

## Event Handling

### Button Events

```rust
button("Click me")
    .on_click(move |event| {
        println!("Button clicked!");
        // Update state, call functions, etc.
    })
```

### Input Events

```rust
input()
    .placeholder("Enter text...")
    .on_change(move |new_value| {
        set_text.set(new_value);
    })
    .on_focus(move |_| {
        println!("Input focused");
    })
    .on_blur(move |_| {
        println!("Input blurred");
    })
```

### Keyboard Events

```rust
div()
    .on_key(move |key_event| {
        match key_event.key {
            Key::Enter => {
                // Handle enter key
            }
            Key::Escape => {
                // Handle escape key
            }
            Key::Char(c) => {
                // Handle character input
            }
            _ => {}
        }
    })
```

## Component Composition

### Parent-Child Communication

```rust
// Parent component
#[derive(Clone)]
struct ParentComponent {
    base: BaseComponent,
}

impl ParentComponent {
    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let (message, set_message) = use_state(String::from("Hello"));
        
        Ok(div()
            .child(
                ChildComponent::new(message.get().clone())
                    .on_message_change(move |new_message| {
                        set_message.set(new_message);
                    })
                    .render(context).await?
            ))
    }
}

// Child component
#[derive(Clone)]
struct ChildComponent {
    base: BaseComponent,
    message: String,
    on_change: Option<Box<dyn Fn(String) + Send + Sync>>,
}

impl ChildComponent {
    fn new(message: String) -> Self {
        Self {
            base: BaseComponent::new("ChildComponent"),
            message,
            on_change: None,
        }
    }
    
    fn on_message_change<F>(mut self, callback: F) -> Self 
    where 
        F: Fn(String) + Send + Sync + 'static 
    {
        self.on_change = Some(Box::new(callback));
        self
    }
}
```

### Higher-Order Components

```rust
// HOC that adds loading state
fn with_loading<T: Component + Clone>(component: T) -> LoadingWrapper<T> {
    LoadingWrapper::new(component)
}

#[derive(Clone)]
struct LoadingWrapper<T: Component> {
    base: BaseComponent,
    wrapped: T,
}

impl<T: Component + Clone> LoadingWrapper<T> {
    fn new(wrapped: T) -> Self {
        Self {
            base: BaseComponent::new("LoadingWrapper"),
            wrapped,
        }
    }
}

#[async_trait]
impl<T: Component + Clone> Component for LoadingWrapper<T> {
    // Implementation that adds loading overlay
    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let (loading, _) = use_state(false);
        
        if *loading.get() {
            Ok(div()
                .child(text("Loading..."))
                .style("position: absolute; top: 0; left: 0; right: 0; bottom: 0;"))
        } else {
            self.wrapped.render(context).await
        }
    }
}
```

## Lifecycle Management

### Component Mounting

```rust
async fn on_mount(&mut self) -> Result<()> {
    // Initialize resources
    self.start_timer().await?;
    self.load_data().await?;
    
    println!("Component {} mounted", self.name());
    Ok(())
}
```

### Component Unmounting

```rust
async fn on_unmount(&mut self) -> Result<()> {
    // Clean up resources
    self.stop_timer().await?;
    self.close_connections().await?;
    
    println!("Component {} unmounted", self.name());
    Ok(())
}
```

### State Changes

```rust
async fn on_state_changed(&mut self) -> Result<()> {
    // React to state changes
    self.validate_data().await?;
    self.update_cache().await?;
    
    Ok(())
}
```

## Error Handling

### Graceful Error Handling

```rust
async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
    let (error, set_error) = use_state(None::<String>);
    
    if let Some(err) = error.get().as_ref() {
        return Ok(div()
            .style("color: red; border: 1px solid red; padding: 1;")
            .child(text(&format!("Error: {}", err)))
            .child(button("Retry")
                .on_click(move |_| set_error.set(None))));
    }
    
    // Normal rendering
    Ok(div().child(text("Content")))
}
```

### Error Boundaries

```rust
#[derive(Clone)]
struct ErrorBoundary {
    base: BaseComponent,
    child: Box<dyn Component>,
}

#[async_trait]
impl Component for ErrorBoundary {
    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        match self.child.render(context).await {
            Ok(node) => Ok(node),
            Err(error) => {
                Ok(div()
                    .style("color: red; padding: 2; border: 1px solid red;")
                    .child(text("Something went wrong"))
                    .child(text(&format!("Error: {}", error))))
            }
        }
    }
}
```

## Performance Optimization

### Memoization

```rust
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (items, _) = use_state(vec![1, 2, 3, 4, 5]);
    
    // Expensive computation - only recalculates when items change
    let processed_items = use_memo(
        move || {
            items.get().iter()
                .map(|x| expensive_computation(*x))
                .collect::<Vec<_>>()
        },
        vec![items.get().clone()]
    );
    
    Ok(div()
        .children(
            processed_items.get().iter()
                .map(|item| text(&item.to_string()))
                .collect()
        ))
}
```

### Conditional Rendering

```rust
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    let (show_expensive, _) = use_state(false);
    
    let mut children = vec![
        text("Always visible"),
        button("Toggle").on_click(move |_| {
            show_expensive.set(!*show_expensive.get());
        }),
    ];
    
    // Only render expensive component when needed
    if *show_expensive.get() {
        children.push(ExpensiveComponent::new().render(_context).await?);
    }
    
    Ok(div().children(children))
}
```

## Testing Components

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tui_framework::style::Theme;

    #[tokio::test]
    async fn test_component_render() {
        let component = MyComponent::new("Test".to_string());
        let theme = Theme::default();
        let context = RenderContext::new(&theme);
        
        let result = component.render(&context).await;
        assert!(result.is_ok());
        
        let node = result.unwrap();
        // Assert on the rendered node structure
    }
    
    #[tokio::test]
    async fn test_component_lifecycle() {
        let mut component = MyComponent::new("Test".to_string());
        
        // Test mounting
        assert!(component.on_mount().await.is_ok());
        
        // Test unmounting
        assert!(component.on_unmount().await.is_ok());
    }
}
```

## Best Practices

1. **Keep components focused**: Each component should have a single responsibility
2. **Use composition over inheritance**: Combine simple components to build complex UIs
3. **Minimize state**: Only store what you need in component state
4. **Handle errors gracefully**: Always provide fallback UI for error states
5. **Test your components**: Write unit tests for component logic
6. **Use meaningful names**: Component and method names should be descriptive
7. **Document complex logic**: Add comments for non-obvious behavior
8. **Optimize performance**: Use memoization for expensive computations
9. **Clean up resources**: Always clean up in `on_unmount`
10. **Follow Rust conventions**: Use standard Rust naming and patterns
