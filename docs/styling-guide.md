# Styling Guide

This guide covers the styling system in TUI Framework, including CSS-like properties, themes, and responsive design.

## Overview

TUI Framework provides a powerful styling system that brings CSS-like capabilities to terminal applications:

- **CSS-like syntax**: Familiar properties and values
- **Theme system**: Consistent styling across your application
- **Responsive design**: Layouts that adapt to terminal size
- **Color support**: Rich color palette with RGB and named colors
- **Flexbox layout**: Modern layout system for complex UIs

## Basic Styling

### Inline Styles

Apply styles directly to components using the `style()` method:

```rust
use tui_framework::prelude::*;
use tui_framework::render::vdom::nodes::*;

div()
    .style("
        background-color: #1e1e1e;
        color: #ffffff;
        padding: 2;
        border: 1px solid #444444;
    ")
    .child(text("Styled content"))
```

### Style Properties

#### Layout Properties

```rust
div()
    .style("
        width: 50%;
        height: 20;
        margin: 2;
        padding: 1;
        position: absolute;
        top: 5;
        left: 10;
    ")
```

#### Flexbox Properties

```rust
div()
    .style("
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        flex-wrap: wrap;
    ")
```

#### Border Properties

```rust
div()
    .style("
        border: 2px solid #ff0000;
        border-top: 1px dashed #00ff00;
        border-radius: 4px;
    ")
```

#### Text Properties

```rust
text("Styled text")
    .style("
        color: #ffffff;
        background-color: #000000;
        font-weight: bold;
        text-align: center;
        text-decoration: underline;
    ")
```

## Color System

### Color Formats

```rust
// Hex colors
.style("color: #ff0000;")           // Red
.style("color: #00ff00;")           // Green
.style("color: #0000ff;")           // Blue

// RGB colors
.style("color: rgb(255, 0, 0);")    // Red
.style("color: rgb(0, 255, 0);")    // Green
.style("color: rgb(0, 0, 255);")    // Blue

// Named colors
.style("color: red;")               // Red
.style("color: green;")             // Green
.style("color: blue;")              // Blue
```

### Color Constants

```rust
use tui_framework::style::Color;

// Use predefined colors
let red = Color::RED;
let green = Color::GREEN;
let blue = Color::BLUE;
let white = Color::WHITE;
let black = Color::BLACK;

// Create custom colors
let custom = Color::rgb(128, 64, 192);
```

## Theme System

### Creating Themes

```rust
use tui_framework::prelude::*;

#[derive(Theme, Clone)]
struct MyTheme {
    #[theme(primary)]
    primary_color: Color,
    
    #[theme(secondary)]
    secondary_color: Color,
    
    #[theme(background)]
    bg_color: Color,
    
    #[theme(text)]
    text_color: Color,
    
    #[theme(border)]
    border_color: Color,
}

impl Default for MyTheme {
    fn default() -> Self {
        Self {
            primary_color: Color::rgb(100, 150, 255),
            secondary_color: Color::rgb(255, 150, 100),
            bg_color: Color::rgb(20, 20, 25),
            text_color: Color::rgb(240, 240, 245),
            border_color: Color::rgb(60, 60, 70),
        }
    }
}
```

### Using Themes

```rust
// Apply theme to app
let app = App::new()
    .title("Themed App")
    .theme(MyTheme::default())
    .component(MyComponent::new());

// Access theme in components
async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
    let theme = &context.theme;
    
    Ok(div()
        .style(&format!("
            background-color: {};
            color: {};
            border: 1px solid {};
        ", theme.background, theme.text, theme.border))
        .child(text("Themed content")))
}
```

### Theme Variants

```rust
// Dark theme
impl MyTheme {
    fn dark() -> Self {
        Self {
            primary_color: Color::rgb(100, 150, 255),
            secondary_color: Color::rgb(255, 150, 100),
            bg_color: Color::rgb(15, 15, 20),
            text_color: Color::rgb(240, 240, 245),
            border_color: Color::rgb(40, 40, 50),
        }
    }
    
    fn light() -> Self {
        Self {
            primary_color: Color::rgb(50, 100, 200),
            secondary_color: Color::rgb(200, 100, 50),
            bg_color: Color::rgb(250, 250, 255),
            text_color: Color::rgb(20, 20, 25),
            border_color: Color::rgb(200, 200, 210),
        }
    }
}
```

## Responsive Design

### Viewport-Based Sizing

```rust
async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
    let viewport = context.viewport_size;
    
    // Responsive width based on viewport
    let width = if viewport.width > 100 {
        "80%"
    } else if viewport.width > 50 {
        "90%"
    } else {
        "100%"
    };
    
    Ok(div()
        .style(&format!("width: {};", width))
        .child(text("Responsive content")))
}
```

### Breakpoint System

```rust
struct Breakpoints;

impl Breakpoints {
    const SMALL: u16 = 40;
    const MEDIUM: u16 = 80;
    const LARGE: u16 = 120;
}

async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
    let width = context.viewport_size.width;
    
    let (columns, padding) = match width {
        w if w >= Breakpoints::LARGE => (3, 4),
        w if w >= Breakpoints::MEDIUM => (2, 2),
        _ => (1, 1),
    };
    
    Ok(div()
        .style(&format!("
            display: flex;
            flex-direction: row;
            padding: {};
        ", padding))
        .children(
            (0..columns)
                .map(|i| div()
                    .style("flex: 1; margin: 1;")
                    .child(text(&format!("Column {}", i + 1))))
                .collect()
        ))
}
```

## Layout System

### Flexbox Layout

```rust
// Horizontal layout
div()
    .style("
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
    ")
    .child(text("Left"))
    .child(text("Center"))
    .child(text("Right"))

// Vertical layout
div()
    .style("
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: stretch;
        height: 100%;
    ")
    .child(text("Top"))
    .child(text("Middle"))
    .child(text("Bottom"))
```

### Grid-like Layout

```rust
// Create a grid using nested flexbox
div()
    .style("
        display: flex;
        flex-direction: column;
        height: 100%;
    ")
    .child(
        // Header row
        div()
            .style("
                display: flex;
                flex-direction: row;
                height: 3;
                border-bottom: 1px solid #444;
            ")
            .child(text("Header").style("flex: 1;"))
    )
    .child(
        // Content row
        div()
            .style("
                display: flex;
                flex-direction: row;
                flex: 1;
            ")
            .child(
                // Sidebar
                div()
                    .style("
                        width: 20;
                        border-right: 1px solid #444;
                        padding: 1;
                    ")
                    .child(text("Sidebar"))
            )
            .child(
                // Main content
                div()
                    .style("flex: 1; padding: 1;")
                    .child(text("Main content"))
            )
    )
```

## Advanced Styling

### CSS Classes

```rust
// Define reusable style classes
struct StyleClasses;

impl StyleClasses {
    const BUTTON: &'static str = "
        padding: 1 2;
        border: 1px solid #444;
        background-color: #333;
        color: #fff;
        cursor: pointer;
    ";
    
    const BUTTON_PRIMARY: &'static str = "
        padding: 1 2;
        border: 1px solid #0066cc;
        background-color: #0080ff;
        color: #fff;
        cursor: pointer;
    ";
    
    const CARD: &'static str = "
        padding: 2;
        border: 1px solid #444;
        border-radius: 4px;
        background-color: #2a2a2a;
        margin: 1;
    ";
}

// Use style classes
button("Click me").style(StyleClasses::BUTTON_PRIMARY)
div().style(StyleClasses::CARD)
```

### Dynamic Styling

```rust
async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
    let (is_active, set_active) = use_state(false);
    let (hover, set_hover) = use_state(false);
    
    let button_style = format!("
        padding: 1 2;
        border: 1px solid {};
        background-color: {};
        color: {};
    ",
        if *is_active.get() { "#0080ff" } else { "#444" },
        if *hover.get() { "#555" } else { "#333" },
        if *is_active.get() { "#fff" } else { "#ccc" }
    );
    
    Ok(button("Dynamic Button")
        .style(&button_style)
        .on_click(move |_| set_active.set(!*is_active.get()))
        .on_hover(move |_| set_hover.set(true))
        .on_blur(move |_| set_hover.set(false)))
}
```

### Animation (Conceptual)

```rust
// Animated progress bar
async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
    let (progress, set_progress) = use_state(0.0);
    
    // Use effect to animate progress
    use_effect(move || {
        // Animation logic would go here
        // This is a conceptual example
    }, vec![]);
    
    let width = (*progress.get() * 100.0) as u16;
    
    Ok(div()
        .style("
            width: 100%;
            height: 3;
            border: 1px solid #444;
            background-color: #222;
        ")
        .child(
            div()
                .style(&format!("
                    width: {}%;
                    height: 100%;
                    background-color: #0080ff;
                ", width))
        ))
}
```

## Best Practices

### 1. Use Consistent Spacing

```rust
// Define spacing constants
struct Spacing;
impl Spacing {
    const XS: u16 = 1;
    const SM: u16 = 2;
    const MD: u16 = 4;
    const LG: u16 = 8;
    const XL: u16 = 16;
}

// Use consistent spacing
div().style(&format!("padding: {};", Spacing::MD))
```

### 2. Create Reusable Components

```rust
fn styled_button(text: &str, variant: ButtonVariant) -> VirtualNode {
    let style = match variant {
        ButtonVariant::Primary => "background-color: #0080ff; color: white;",
        ButtonVariant::Secondary => "background-color: #666; color: white;",
        ButtonVariant::Danger => "background-color: #ff4444; color: white;",
    };
    
    button(text)
        .style(&format!("
            padding: 1 2;
            border: none;
            border-radius: 2px;
            {}
        ", style))
}
```

### 3. Use Semantic Color Names

```rust
struct Colors;
impl Colors {
    const PRIMARY: Color = Color::rgb(0, 128, 255);
    const SUCCESS: Color = Color::rgb(40, 167, 69);
    const WARNING: Color = Color::rgb(255, 193, 7);
    const DANGER: Color = Color::rgb(220, 53, 69);
    const INFO: Color = Color::rgb(23, 162, 184);
}
```

### 4. Test Across Different Terminal Sizes

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_responsive_layout() {
        let component = MyComponent::new();
        
        // Test small screen
        let small_context = RenderContext::new_with_size(Size::new(40, 20));
        let small_result = component.render(&small_context).await;
        assert!(small_result.is_ok());
        
        // Test large screen
        let large_context = RenderContext::new_with_size(Size::new(120, 40));
        let large_result = component.render(&large_context).await;
        assert!(large_result.is_ok());
    }
}
```

### 5. Optimize for Performance

- Use memoization for expensive style calculations
- Avoid creating new style strings on every render
- Cache computed styles when possible
- Use CSS classes instead of inline styles for repeated patterns

## Common Patterns

### Modal Overlay

```rust
div()
    .style("
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
    ")
    .child(
        div()
            .style("
                background-color: white;
                padding: 4;
                border-radius: 8px;
                max-width: 80%;
                max-height: 80%;
            ")
            .child(text("Modal content"))
    )
```

### Card Layout

```rust
div()
    .style("
        background-color: #2a2a2a;
        border: 1px solid #444;
        border-radius: 4px;
        padding: 3;
        margin: 2;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    ")
    .child(text("Card title").style("font-weight: bold; margin-bottom: 2;"))
    .child(text("Card content"))
```

### Navigation Bar

```rust
div()
    .style("
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        padding: 1 2;
        background-color: #1a1a1a;
        border-bottom: 1px solid #444;
    ")
    .child(text("App Title").style("font-weight: bold;"))
    .child(
        div()
            .style("display: flex; gap: 2;")
            .child(button("Home"))
            .child(button("About"))
            .child(button("Settings"))
    )
```
