# TUI Framework

A React-like TUI (Terminal User Interface) framework for Rust that combines the developer experience of React with the performance of libnotcurses.

## 🚀 Features

- **React-like API**: Familiar hooks, components, and patterns from React
- **High Performance**: Built on libnotcurses for efficient terminal rendering
- **Type Safety**: Leverages Rust's type system for safe, reliable applications
- **Modern Styling**: CSS-like styling system with themes and responsive design
- **Rich Components**: Comprehensive set of built-in widgets and components
- **Async/Await**: Full async support for modern Rust applications

## 🎯 Goals

This framework aims to bring the best of web development to terminal applications:

1. **Familiar Developer Experience**: React developers should feel at home
2. **High Performance**: Leverage libnotcurses for advanced terminal features
3. **Type Safety**: Compile-time guarantees for UI correctness
4. **Modern Patterns**: Hooks, context, and component composition
5. **Rich Ecosystem**: Extensible widget system and theming

## 📦 Installation

### Prerequisites

This framework requires **notcurses 3.0.11+**. Choose your preferred installation method:

**Python installer (recommended):**
```bash
pip install tui-framework-installer
tui-install
```

**One-line script:**
```bash
curl -sSL https://raw.githubusercontent.com/entrepeneur4lyf/tui-framework/main/install-notcurses.sh | bash
```

**More options:** See [EASY_INSTALL.md](EASY_INSTALL.md) for all installation methods.

### Add to your project

```toml
[dependencies]
tui-framework = { version = "0.1.0", features = ["notcurses"] }
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 Quick Start

```rust
use tui_framework::prelude::*;

#[component]
fn Counter() -> impl Component {
    let (count, set_count) = use_state(0);
    
    div()
        .child(text(&format!("Count: {}", count)))
        .child(button("Increment")
            .on_click(move |_| set_count(count + 1)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()
        .title("Counter App")
        .component(Counter);
    
    app.run().await?;
    Ok(())
}
```

## 🏗️ Architecture

The framework is built around several core concepts:

### Components
Reusable UI elements that can have state and props:

```rust
#[component]
fn TodoItem(props: TodoItemProps) -> impl Component {
    let (completed, set_completed) = use_state(props.todo.completed);
    
    div()
        .class("todo-item")
        .child(checkbox().checked(completed))
        .child(text(&props.todo.text))
}
```

### Hooks
Functions that let you use state and lifecycle features:

```rust
fn MyComponent() -> impl Component {
    let (state, set_state) = use_state(0);
    let (data, set_data) = use_state(Vec::new());
    
    use_effect(move || {
        // Side effects, API calls, etc.
    }, vec![state]);
    
    // Component JSX...
}
```

### Styling
CSS-like styling with themes and responsive design:

```rust
button("Click me")
    .style(|s| s
        .background_color(Color::Blue)
        .color(Color::White)
        .padding(Padding::all(8))
        .hover(|s| s.background_color(Color::DarkBlue))
    )
```

## 🎨 Built-in Components

- **Layout**: `div`, `container`, `flex`, `grid`
- **Text**: `text`, `heading`, `paragraph`
- **Input**: `button`, `input`, `checkbox`, `radio`, `select`
- **Display**: `table`, `list`, `tree`, `progress`
- **Navigation**: `tabs`, `menu`, `breadcrumb`
- **Feedback**: `modal`, `tooltip`, `notification`

## 🎭 Theming

```rust
#[derive(Theme)]
struct DarkTheme {
    primary: Color::Blue,
    secondary: Color::Gray,
    background: Color::Black,
    surface: Color::DarkGray,
    text: Color::White,
}

app.set_theme(DarkTheme::default());
```

## 📚 Examples

Check out the `examples/` directory for comprehensive examples:

- `hello_world.rs` - Basic framework usage
- `counter.rs` - State management with hooks
- `calculator.rs` - Complex component composition
- `todo_app.rs` - Full application with CRUD operations

## 🔧 Development Status

This framework is currently in early development. The core architecture is in place, and we're actively implementing:

- [x] Project structure and core traits
- [ ] Reactive system (hooks)
- [ ] libnotcurses integration
- [ ] Basic widgets
- [ ] Event system
- [ ] Virtual DOM
- [ ] Layout engine
- [ ] Styling system
- [ ] Example applications

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- [Textual](https://github.com/Textualize/textual) - Inspiration for the component model
- [libnotcurses](https://github.com/dankamongmen/notcurses) - High-performance terminal rendering
- [React](https://reactjs.org/) - Component and hooks patterns
- [Taffy](https://github.com/DioxusLabs/taffy) - Flexbox layout engine
