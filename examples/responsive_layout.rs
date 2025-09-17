//! Responsive layout example demonstrating the layout engine.
//! 
//! This example shows how the layout engine automatically adapts to terminal size
//! and provides responsive layouts by default.

use tui_framework::prelude::*;
use tui_framework::layout::{Layout, Size};
use tui_framework::render::vdom::{VirtualNode, VirtualStyle, StyleValue, DisplayType, FlexDirection, JustifyContent, AlignItems};
use tui_framework::render::vdom::nodes::{div, text};
use tui_framework::style::{Color, Theme};
use tui_framework::component::BaseComponent;

/// A responsive layout component that demonstrates various layout patterns.
struct ResponsiveLayout {
    base: BaseComponent,
}

impl ResponsiveLayout {
    fn new() -> Self {
        Self {
            base: BaseComponent::new("ResponsiveLayout"),
        }
    }
}

#[async_trait::async_trait]
impl Component for ResponsiveLayout {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "ResponsiveLayout"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        // Create a responsive layout that fills the terminal
        Ok(div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Column),
                width: Some(StyleValue::Fill), // Fill terminal width
                height: Some(StyleValue::Fill), // Fill terminal height
                background_color: Some(Color::rgb(20, 20, 30)),
                ..Default::default()
            })
            .child(
                // Header section - fixed height
                div()
                    .style(VirtualStyle {
                        height: Some(StyleValue::Absolute(3)),
                        width: Some(StyleValue::Fill),
                        background_color: Some(Color::rgb(40, 40, 60)),
                        display: Some(DisplayType::Flex),
                        justify_content: Some(JustifyContent::Center),
                        align_items: Some(AlignItems::Center),
                        ..Default::default()
                    })
                    .child(text("🚀 Responsive TUI Framework - Layout Demo"))
            )
            .child(
                // Main content area - flexible
                div()
                    .style(VirtualStyle {
                        display: Some(DisplayType::Flex),
                        flex_direction: Some(FlexDirection::Row),
                        width: Some(StyleValue::Fill),
                        height: Some(StyleValue::Fill), // Take remaining space
                        ..Default::default()
                    })
                    .child(
                        // Sidebar - 25% width
                        div()
                            .style(VirtualStyle {
                                width: Some(StyleValue::Percentage(25.0)),
                                height: Some(StyleValue::Fill),
                                background_color: Some(Color::rgb(30, 30, 50)),
                                display: Some(DisplayType::Flex),
                                flex_direction: Some(FlexDirection::Column),
                                justify_content: Some(JustifyContent::FlexStart),
                                align_items: Some(AlignItems::Center),
                                ..Default::default()
                            })
                            .child(text("📋 Sidebar"))
                            .child(text("• Navigation"))
                            .child(text("• Settings"))
                            .child(text("• Help"))
                    )
                    .child(
                        // Main content - 75% width
                        div()
                            .style(VirtualStyle {
                                width: Some(StyleValue::Percentage(75.0)),
                                height: Some(StyleValue::Fill),
                                background_color: Some(Color::rgb(25, 25, 40)),
                                display: Some(DisplayType::Flex),
                                flex_direction: Some(FlexDirection::Column),
                                justify_content: Some(JustifyContent::SpaceBetween),
                                align_items: Some(AlignItems::Center),
                                ..Default::default()
                            })
                            .child(
                                div()
                                    .style(VirtualStyle {
                                        display: Some(DisplayType::Flex),
                                        flex_direction: Some(FlexDirection::Row),
                                        justify_content: Some(JustifyContent::SpaceEvenly),
                                        width: Some(StyleValue::Fill),
                                        ..Default::default()
                                    })
                                    .child(
                                        div()
                                            .style(VirtualStyle {
                                                width: Some(StyleValue::Percentage(30.0)),
                                                height: Some(StyleValue::Absolute(8)),
                                                background_color: Some(Color::rgb(60, 80, 100)),
                                                display: Some(DisplayType::Flex),
                                                justify_content: Some(JustifyContent::Center),
                                                align_items: Some(AlignItems::Center),
                                                ..Default::default()
                                            })
                                            .child(text("📊 Chart 1"))
                                    )
                                    .child(
                                        div()
                                            .style(VirtualStyle {
                                                width: Some(StyleValue::Percentage(30.0)),
                                                height: Some(StyleValue::Absolute(8)),
                                                background_color: Some(Color::rgb(80, 60, 100)),
                                                display: Some(DisplayType::Flex),
                                                justify_content: Some(JustifyContent::Center),
                                                align_items: Some(AlignItems::Center),
                                                ..Default::default()
                                            })
                                            .child(text("📈 Chart 2"))
                                    )
                                    .child(
                                        div()
                                            .style(VirtualStyle {
                                                width: Some(StyleValue::Percentage(30.0)),
                                                height: Some(StyleValue::Absolute(8)),
                                                background_color: Some(Color::rgb(100, 80, 60)),
                                                display: Some(DisplayType::Flex),
                                                justify_content: Some(JustifyContent::Center),
                                                align_items: Some(AlignItems::Center),
                                                ..Default::default()
                                            })
                                            .child(text("📉 Chart 3"))
                                    )
                            )
                            .child(text("💡 This layout automatically adapts to terminal size!"))
                            .child(text("🔄 Resize your terminal to see responsive behavior"))
                    )
            )
            .child(
                // Footer - fixed height
                div()
                    .style(VirtualStyle {
                        height: Some(StyleValue::Absolute(2)),
                        width: Some(StyleValue::Fill),
                        background_color: Some(Color::rgb(40, 40, 60)),
                        display: Some(DisplayType::Flex),
                        justify_content: Some(JustifyContent::Center),
                        align_items: Some(AlignItems::Center),
                        ..Default::default()
                    })
                    .child(text("Press Ctrl+C to exit | Framework v0.1.0"))
            ))
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Responsive Layout Demo...");
    println!("This demo shows how the layout engine automatically fills terminal space.");
    println!("Try resizing your terminal to see responsive behavior!");
    println!();

    // Create the responsive layout component
    let layout = ResponsiveLayout::new();

    // Test the layout computation with different terminal sizes
    let sizes = vec![
        Size::new(80, 24),   // Standard terminal
        Size::new(120, 30),  // Wide terminal
        Size::new(60, 20),   // Narrow terminal
        Size::new(200, 50),  // Very wide terminal
    ];

    for (i, terminal_size) in sizes.iter().enumerate() {
        println!("📐 Testing layout computation for terminal size: {}x{}", 
                 terminal_size.width, terminal_size.height);

        // Render the component
        let context = RenderContext::new(&Theme::default());
        let mut vdom = layout.render(&context).await?;

        // Compute layout
        let layout_result = Layout::compute(&mut vdom, *terminal_size);
        
        println!("   ✅ Layout computed successfully!");
        println!("   📏 Total size required: {}x{}", 
                 layout_result.total_size.width, layout_result.total_size.height);
        println!("   🎯 Layout nodes: {}", layout_result.layouts.len());
        
        // Show some layout details
        for (node_id, computed_layout) in layout_result.layouts.iter().take(3) {
            println!("   📦 Node '{}': pos=({}, {}), size={}x{}", 
                     node_id,
                     computed_layout.position.x, computed_layout.position.y,
                     computed_layout.size.width, computed_layout.size.height);
        }
        
        if i < sizes.len() - 1 {
            println!();
        }
    }

    println!();
    println!("🎉 Responsive layout demo completed successfully!");
    println!("💡 Key features demonstrated:");
    println!("   • Automatic terminal space utilization");
    println!("   • Percentage-based responsive widths");
    println!("   • Flexbox-style layout with proper alignment");
    println!("   • Fixed and flexible sizing combinations");
    println!("   • Nested layout containers");

    Ok(())
}
