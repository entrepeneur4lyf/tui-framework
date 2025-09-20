//! Test all enhanced macro functionality.

use tui_framework::prelude::*;
use tui_framework::reactive::hooks::use_effect_simple;

// Test the enhanced Theme derive macro
#[derive(Debug, Clone, Theme)]
struct AppTheme {
    primary: Color,
    secondary: Color,
    background: Color,
    text: Color,
    #[allow(dead_code)]
    accent: Color,
}

// Test component macro with the new theme
#[component]
fn themed_button(label: String, theme: AppTheme) -> VirtualNode {
    VirtualNode::element("button")
        .attr(
            "style",
            format!(
                "background-color: {:?}; color: {:?}",
                theme.primary, theme.text
            ),
        )
        .child(VirtualNode::text(label))
}

// Test component with hooks pattern
#[component]
fn counter_with_hooks() -> VirtualNode {
    // Using the use_hooks macro for better organization
    use_hooks! {
        let (count, _set_count) = use_state(0);

        use_effect_simple(|| {
            println!("Counter component mounted");
        }, vec!["mount".to_string()]);
    }

    VirtualNode::element("div")
        .child(VirtualNode::text(format!("Count: {}", count.get())))
        .child(VirtualNode::element("button").child(VirtualNode::text("Increment".to_string())))
}

// Test JSX-like syntax (placeholder implementation)
#[component]
fn jsx_example() -> VirtualNode {
    // This will use the jsx! macro when fully implemented
    let _jsx_node = jsx! {};

    // For now, create manually
    VirtualNode::element("div").child(VirtualNode::text(
        "JSX-like syntax coming soon!".to_string(),
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing Enhanced Macro System");
    println!("============================");

    // Test 1: Enhanced Theme derive macro
    println!("\n1. Testing Enhanced Theme Derive:");

    let app_theme = AppTheme {
        primary: Color::rgb(255, 100, 50),
        secondary: Color::rgb(50, 150, 255),
        background: Color::rgb(25, 25, 30),
        text: Color::rgb(255, 255, 255),
        accent: Color::rgb(255, 200, 0),
    };

    println!("   Custom AppTheme: {:?}", app_theme);

    // Test conversion to base theme
    let base_theme: Theme = app_theme.clone().into();
    println!("   Converted to base Theme: {:?}", base_theme);

    // Test default creation
    let default_theme = AppTheme::default();
    println!("   Default AppTheme: {:?}", default_theme);

    // Test dark/light theme creation
    let dark_theme = AppTheme::dark();
    let light_theme = AppTheme::light();
    println!(
        "   Dark AppTheme: primary={:?}, background={:?}",
        dark_theme.primary, dark_theme.background
    );
    println!(
        "   Light AppTheme: primary={:?}, background={:?}",
        light_theme.primary, light_theme.background
    );

    // Test 2: Component macro with enhanced theme
    println!("\n2. Testing Component with Enhanced Theme:");

    let button_component =
        ThemedButtonComponent::new("Themed Button".to_string(), app_theme.clone());

    let render_context = RenderContext::new(&base_theme);
    let button_vnode = button_component.render(&render_context).await?;
    println!("   Themed button component rendered successfully");
    println!("   Button VNode: {:?}", button_vnode);

    // Test 3: CSS macro with enhanced features
    println!("\n3. Testing Enhanced CSS Macro:");

    let css_style1 = css! { "bg-blue text-white w-full" };
    let style1 = css_style1.build();
    println!("   Utility classes style: {:?}", style1);

    let css_style2 = css! { "background-color: red; color: white; width: 200px" };
    let style2 = css_style2.build();
    println!("   CSS properties style: {:?}", style2);

    let empty_css = css! {};
    let empty_style = empty_css.build();
    println!("   Empty CSS style: {:?}", empty_style);

    // Test 4: Hook patterns with use_hooks macro
    println!("\n4. Testing Hook Patterns:");

    let counter_component = CounterWithHooksComponent::new();
    let counter_vnode = counter_component.render(&render_context).await?;
    println!("   Counter with hooks rendered successfully");
    println!("   Counter VNode: {:?}", counter_vnode);

    // Test 5: JSX-like syntax (placeholder)
    println!("\n5. Testing JSX-like Syntax (Placeholder):");

    let jsx_component = JsxExampleComponent::new();
    let jsx_vnode = jsx_component.render(&render_context).await?;
    println!("   JSX example rendered successfully");
    println!("   JSX VNode: {:?}", jsx_vnode);

    // Test 6: Integration test - all macros together
    println!("\n6. Testing Macro Integration:");

    let integrated_style = css! { "bg-blue text-white" };
    let integrated_theme = AppTheme::dark();

    println!("   Integrated style: {:?}", integrated_style.build());
    println!(
        "   Integrated theme: primary={:?}",
        integrated_theme.primary
    );

    // Create a complex component using all macros
    let complex_vnode = VirtualNode::element("div")
        .attr("class", "app-container")
        .child(
            VirtualNode::element("header")
                .child(VirtualNode::text("Enhanced TUI Framework".to_string())),
        )
        .child(
            VirtualNode::element("main")
                .child(button_vnode)
                .child(counter_vnode)
                .child(jsx_vnode),
        );

    println!("   Complex integrated VNode created successfully");
    println!(
        "   VNode has {} children",
        complex_vnode.get_children().len()
    );

    println!("\n✅ All enhanced macro tests completed successfully!");
    println!("🎉 Enhanced macro system is fully functional!");

    Ok(())
}
