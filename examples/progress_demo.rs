//! Progress bar widget demonstration.
//!
//! This example showcases the comprehensive progress bar widget with:
//! - Different visual styles (bar, blocks, dots, spinner)
//! - Determinate and indeterminate progress
//! - Various text positions and labels
//! - Color customization
//! - Animation effects

#![allow(dead_code)]

use std::any::Any;
use tui_framework::component::BaseComponent;
use tui_framework::prelude::*;
use tui_framework::widget::{
    ProgressBar, ProgressOrientation, ProgressStyle, TextPosition,
};

/// Progress bar demonstration component.
struct ProgressDemo {
    base: BaseComponent,
    progress_values: Vec<f64>,
    current_style: usize,
    animation_counter: usize,
}

impl ProgressDemo {
    /// Create a new progress demo.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("ProgressDemo"),
            progress_values: vec![0.0, 0.25, 0.5, 0.75, 1.0],
            current_style: 0,
            animation_counter: 0,
        }
    }

    /// Get the current progress value for animated demos.
    fn get_animated_progress(&self) -> f64 {
        let cycle = (self.animation_counter / 10) % 100;
        cycle as f64 / 100.0
    }

    /// Update animation state.
    pub fn update_animation(&mut self) {
        self.animation_counter = (self.animation_counter + 1) % 1000;
    }

    /// Cycle through different styles.
    pub fn next_style(&mut self) {
        self.current_style = (self.current_style + 1) % 4;
    }
}

#[async_trait]
impl Component for ProgressDemo {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let mut container = VirtualNode::element("div")
            .attr("class", "progress-demo-container");

        // Title
        container = container.child(
            VirtualNode::element("h1")
                .attr("class", "demo-title")
                .child(VirtualNode::text("Progress Bar Widget Demo")),
        );

        // Section 1: Basic Progress Bars
        container = container.child(
            VirtualNode::element("h2")
                .attr("class", "section-title")
                .child(VirtualNode::text("1. Basic Progress Bars")),
        );

        for (i, &progress) in self.progress_values.iter().enumerate() {
            let progress_bar = ProgressBar::new()
                .with_progress(progress)
                .with_label(format!("Task {}", i + 1))
                .with_width(30)
                .show_percentage(true);

            let progress_node = progress_bar.render(_context).await?;
            container = container.child(progress_node);
        }

        // Section 2: Different Styles
        container = container.child(
            VirtualNode::element("h2")
                .attr("class", "section-title")
                .child(VirtualNode::text("2. Different Visual Styles")),
        );

        let styles = [
            (ProgressStyle::Bar, "Bar Style"),
            (ProgressStyle::Blocks, "Blocks Style"),
            (ProgressStyle::Dots, "Dots Style"),
            (ProgressStyle::Spinner, "Spinner Style"),
        ];

        for (style, name) in styles.iter() {
            let progress_bar = if matches!(style, ProgressStyle::Spinner) {
                ProgressBar::new()
                    .with_indeterminate()
                    .with_style(style.clone())
                    .with_label(name.to_string())
                    .with_width(20)
            } else {
                ProgressBar::new()
                    .with_progress(0.7)
                    .with_style(style.clone())
                    .with_label(name.to_string())
                    .with_width(25)
                    .show_percentage(true)
            };

            let progress_node = progress_bar.render(_context).await?;
            container = container.child(progress_node);
        }

        // Section 3: Text Positions
        container = container.child(
            VirtualNode::element("h2")
                .attr("class", "section-title")
                .child(VirtualNode::text("3. Text Positioning")),
        );

        let text_positions = [
            (TextPosition::Above, "Text Above"),
            (TextPosition::Below, "Text Below"),
            (TextPosition::Overlay, "Text Overlay"),
        ];

        for (position, name) in text_positions.iter() {
            let progress_bar = ProgressBar::new()
                .with_progress(0.6)
                .with_label(name.to_string())
                .with_text_position(position.clone())
                .with_width(25)
                .show_percentage(true);

            let progress_node = progress_bar.render(_context).await?;
            container = container.child(progress_node);
        }

        // Section 4: Animated Progress
        container = container.child(
            VirtualNode::element("h2")
                .attr("class", "section-title")
                .child(VirtualNode::text("4. Animated Progress")),
        );

        let animated_progress = ProgressBar::new()
            .with_progress(self.get_animated_progress())
            .with_label("Animated Progress")
            .with_width(30)
            .show_percentage(true);

        let animated_node = animated_progress.render(_context).await?;
        container = container.child(animated_node);

        // Section 5: Indeterminate Progress
        container = container.child(
            VirtualNode::element("h2")
                .attr("class", "section-title")
                .child(VirtualNode::text("5. Indeterminate Progress")),
        );

        let indeterminate_styles = [
            ProgressStyle::Bar,
            ProgressStyle::Blocks,
            ProgressStyle::Dots,
            ProgressStyle::Spinner,
        ];

        for style in indeterminate_styles.iter() {
            let mut progress_bar = ProgressBar::new()
                .with_indeterminate()
                .with_style(style.clone())
                .with_label(format!("Loading ({:?})", style))
                .with_width(25);

            // Simulate animation frame updates
            for _ in 0..self.animation_counter % 10 {
                progress_bar.update_animation();
            }

            let progress_node = progress_bar.render(_context).await?;
            container = container.child(progress_node);
        }

        // Section 6: Vertical Progress Bars
        container = container.child(
            VirtualNode::element("h2")
                .attr("class", "section-title")
                .child(VirtualNode::text("6. Vertical Orientation")),
        );

        let vertical_progress = ProgressBar::new()
            .with_progress(0.8)
            .with_orientation(ProgressOrientation::Vertical)
            .with_label("Vertical Progress")
            .with_height(10)
            .show_percentage(true);

        let vertical_node = vertical_progress.render(_context).await?;
        container = container.child(vertical_node);

        // Section 7: Custom Colors (conceptual - colors would be applied via CSS)
        container = container.child(
            VirtualNode::element("h2")
                .attr("class", "section-title")
                .child(VirtualNode::text("7. Custom Styling")),
        );

        let custom_progress = ProgressBar::new()
            .with_progress(0.9)
            .with_label("Custom Styled Progress")
            .with_width(30)
            .show_percentage(true)
            .with_text_position(TextPosition::Overlay);

        let custom_node = custom_progress.render(_context).await?;
        container = container.child(custom_node);

        // Instructions
        container = container.child(
            VirtualNode::element("div")
                .attr("class", "instructions")
                .child(VirtualNode::text(
                    "Progress bars support various styles, orientations, and text positions. \
                     Animation is handled automatically for indeterminate progress.",
                )),
        );

        Ok(container)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut demo = ProgressDemo::new();

    // Simulate some animation updates
    for _ in 0..5 {
        demo.update_animation();
    }

    let theme = Theme::default();
    let context = RenderContext::new(&theme);

    let result = demo.render(&context).await?;
    println!("Progress Demo rendered successfully!");
    println!("VirtualNode: {:?}", result);

    Ok(())
}
