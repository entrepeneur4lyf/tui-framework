# Getting Started with TUI Framework

Welcome to TUI Framework! This guide will help you build your first terminal user interface application using our React-like framework for Rust.

## Prerequisites

- Rust 1.70 or later
- Basic familiarity with Rust and async programming
- Terminal that supports modern features (most terminals work)

## Installation

Add TUI Framework to your `Cargo.toml`:

```toml
[dependencies]
tui-framework = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Your First Application

Let's create a simple "Hello World" application:

```rust
use tui_framework::prelude::*;
use tui_framework::render::vdom::nodes::*;

#[derive(Clone)]
struct HelloWorld {
    base: BaseComponent,
}

impl HelloWorld {
    fn new() -> Self {
        Self {
            base: BaseComponent::new("HelloWorld"),
        }
    }
}

#[async_trait]
impl Component for HelloWorld {
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

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        Ok(div()
            .style("padding: 2; border: 1px solid blue; text-align: center;")
            .child(text("Hello, TUI Framework!"))
            .child(text("Press 'q' to quit")))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new()
        .title("Hello World")
        .component(HelloWorld::new());

    app.run().await
}
```

Run your application:

```bash
cargo run
```

## Adding Interactivity

Let's create a counter that responds to user input:

```rust
use tui_framework::prelude::*;
use tui_framework::render::vdom::nodes::*;

#[derive(Clone)]
struct Counter {
    base: BaseComponent,
}

impl Counter {
    fn new() -> Self {
        Self {
            base: BaseComponent::new("Counter"),
        }
    }
}

#[async_trait]
impl Component for Counter {
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

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        // Use reactive state
        let (count, set_count) = use_state(0);

        Ok(div()
            .style("padding: 2; border: 1px solid green;")
            .child(text(&format!("Count: {}", count.get())))
            .child(
                div()
                    .style("margin-top: 1;")
                    .child(
                        button("Increment")
                            .style("margin-right: 1;")
                            .on_click(move |_| {
                                set_count.set(*count.get() + 1);
                            })
                    )
                    .child(
                        button("Decrement")
                            .style("margin-right: 1;")
                            .on_click(move |_| {
                                set_count.set(*count.get() - 1);
                            })
                    )
                    .child(
                        button("Reset")
                            .on_click(move |_| {
                                set_count.set(0);
                            })
                    )
            ))
    }
}
```

## Key Concepts

### Components

Components are the building blocks of your application. They:
- Implement the `Component` trait
- Have a unique ID and name
- Can maintain state
- Render to virtual DOM nodes
- Respond to events

### State Management

Use the `use_state` hook for reactive state:

```rust
let (value, set_value) = use_state(initial_value);

// Read state
println!("Current value: {}", value.get());

// Update state (triggers re-render)
set_value.set(new_value);
```

### Virtual DOM

Build your UI using virtual DOM nodes:

```rust
div()                                    // Container
    .style("padding: 1;")               // Add styling
    .child(text("Hello"))               // Add text
    .child(button("Click me")           // Add button
        .on_click(|_| { /* handler */ })) // Add event handler
```

### Styling

Style components with CSS-like syntax:

```rust
div()
    .style("
        background-color: #1e1e1e;
        color: #ffffff;
        padding: 2;
        border: 1px solid #444444;
        text-align: center;
    ")
```

## Common Patterns

### Component Composition

```rust
async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
    Ok(div()
        .child(HeaderComponent::new().render(_context).await?)
        .child(MainComponent::new().render(_context).await?)
        .child(FooterComponent::new().render(_context).await?))
}
```

### Conditional Rendering

```rust
let (show_details, set_show_details) = use_state(false);

let content = if *show_details.get() {
    div().child(text("Detailed information here"))
} else {
    div().child(text("Summary view"))
};

Ok(div()
    .child(button("Toggle Details")
        .on_click(move |_| set_show_details.set(!*show_details.get())))
    .child(content))
```

### Lists and Collections

```rust
let items = vec!["Item 1", "Item 2", "Item 3"];

Ok(div()
    .children(
        items.into_iter()
            .map(|item| text(item))
            .collect()
    ))
```

## Next Steps

- Read the [Component Development Guide](component-guide.md)
- Learn about [Styling](styling-guide.md)
- Explore [Performance Optimization](performance-guide.md)
- Check out the example applications in the `examples/` directory

## Examples

The framework includes several example applications:

```bash
# Basic examples
cargo run --example hello_world
cargo run --example counter

# Advanced examples
cargo run --example calculator
cargo run --example todo_app
cargo run --example responsive_layout
```

Each example demonstrates different aspects of the framework and can serve as a starting point for your own applications.
