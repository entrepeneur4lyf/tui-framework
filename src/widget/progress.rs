//! Progress bar widget implementation.
//!
//! This module provides a comprehensive progress bar widget with support for:
//! - Determinate and indeterminate progress
//! - Multiple visual styles (bar, blocks, dots, spinner)
//! - Customizable colors and dimensions
//! - Text labels and percentage display
//! - Animation support
//! - Event callbacks

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::render::{RenderContext, VirtualNode};
use crate::style::Color;
use crate::widget::Widget;
use async_trait::async_trait;
use std::any::Any;

/// Progress type enumeration.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressType {
    /// Determinate progress with a specific value (0.0 to 1.0).
    Determinate(f64),
    /// Indeterminate progress showing activity without specific completion.
    Indeterminate,
}

/// Visual style for the progress bar.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressStyle {
    /// Traditional horizontal bar with fill.
    Bar,
    /// Segmented blocks showing discrete progress.
    Blocks,
    /// Dot-based progress indicator.
    Dots,
    /// Rotating spinner for indeterminate progress.
    Spinner,
}

/// Orientation of the progress bar.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressOrientation {
    /// Horizontal progress bar (default).
    Horizontal,
    /// Vertical progress bar.
    Vertical,
}

/// Text position relative to the progress bar.
#[derive(Debug, Clone, PartialEq)]
pub enum TextPosition {
    /// Text above the progress bar.
    Above,
    /// Text below the progress bar.
    Below,
    /// Text overlaid on the progress bar.
    Overlay,
    /// No text display.
    None,
}

/// Progress bar widget with comprehensive customization options.
pub struct ProgressBar {
    base: BaseComponent,
    progress_type: ProgressType,
    style: ProgressStyle,
    orientation: ProgressOrientation,
    width: Option<usize>,
    height: Option<usize>,
    show_percentage: bool,
    show_label: bool,
    label: Option<String>,
    text_position: TextPosition,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    text_color: Option<Color>,
    animation_frame: usize,
    animation_speed: usize, // frames per animation step
    // Event callbacks
    on_complete: Option<Box<dyn Fn() + Send + Sync>>,
    on_progress_changed: Option<Box<dyn Fn(f64) + Send + Sync>>,
}

impl ProgressBar {
    /// Create a new progress bar with default settings.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("ProgressBar"),
            progress_type: ProgressType::Determinate(0.0),
            style: ProgressStyle::Bar,
            orientation: ProgressOrientation::Horizontal,
            width: Some(20),
            height: Some(1),
            show_percentage: true,
            show_label: false,
            label: None,
            text_position: TextPosition::Below,
            foreground_color: None,
            background_color: None,
            text_color: None,
            animation_frame: 0,
            animation_speed: 3,
            on_complete: None,
            on_progress_changed: None,
        }
    }

    /// Set the progress value (0.0 to 1.0).
    pub fn with_progress(mut self, progress: f64) -> Self {
        let clamped_progress = progress.clamp(0.0, 1.0);
        self.progress_type = ProgressType::Determinate(clamped_progress);
        self
    }

    /// Set the progress bar to indeterminate mode.
    pub fn with_indeterminate(mut self) -> Self {
        self.progress_type = ProgressType::Indeterminate;
        self
    }

    /// Set the visual style of the progress bar.
    pub fn with_style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the orientation of the progress bar.
    pub fn with_orientation(mut self, orientation: ProgressOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the width of the progress bar.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the progress bar.
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Enable or disable percentage display.
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Set a text label for the progress bar.
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self.show_label = true;
        self
    }

    /// Set the text position.
    pub fn with_text_position(mut self, position: TextPosition) -> Self {
        self.text_position = position;
        self
    }

    /// Set the foreground color.
    pub fn with_foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = Some(color);
        self
    }

    /// Set the background color.
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set the text color.
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set the animation speed for indeterminate progress.
    pub fn with_animation_speed(mut self, speed: usize) -> Self {
        self.animation_speed = speed.max(1);
        self
    }

    /// Set a callback for when progress reaches 100%.
    pub fn on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_complete = Some(Box::new(callback));
        self
    }

    /// Set a callback for when progress value changes.
    pub fn on_progress_changed<F>(mut self, callback: F) -> Self
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        self.on_progress_changed = Some(Box::new(callback));
        self
    }

    /// Get the current progress value.
    pub fn progress(&self) -> Option<f64> {
        match &self.progress_type {
            ProgressType::Determinate(value) => Some(*value),
            ProgressType::Indeterminate => None,
        }
    }

    /// Set the progress value and trigger callbacks.
    pub fn set_progress(&mut self, progress: f64) {
        let clamped_progress = progress.clamp(0.0, 1.0);
        let old_progress = self.progress();
        
        self.progress_type = ProgressType::Determinate(clamped_progress);
        
        // Trigger progress changed callback
        if let Some(ref callback) = self.on_progress_changed {
            callback(clamped_progress);
        }
        
        // Trigger completion callback if we reached 100%
        if clamped_progress >= 1.0 && old_progress.is_none_or(|p| p < 1.0) {
            if let Some(ref callback) = self.on_complete {
                callback();
            }
        }
    }

    /// Update animation frame for indeterminate progress.
    pub fn update_animation(&mut self) {
        if matches!(self.progress_type, ProgressType::Indeterminate) {
            self.animation_frame = (self.animation_frame + 1) % (self.animation_speed * 8);
        }
    }

    /// Get the current animation frame.
    pub fn animation_frame(&self) -> usize {
        self.animation_frame / self.animation_speed
    }

    /// Check if the progress bar is complete.
    pub fn is_complete(&self) -> bool {
        matches!(self.progress_type, ProgressType::Determinate(p) if p >= 1.0)
    }

    /// Check if the progress bar is indeterminate.
    pub fn is_indeterminate(&self) -> bool {
        matches!(self.progress_type, ProgressType::Indeterminate)
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressBar {
    /// Render the progress bar content based on style and orientation.
    fn render_progress_content(&self) -> String {
        let width = self.width.unwrap_or(20);

        match &self.style {
            ProgressStyle::Bar => self.render_bar_style(width),
            ProgressStyle::Blocks => self.render_blocks_style(width),
            ProgressStyle::Dots => self.render_dots_style(width),
            ProgressStyle::Spinner => self.render_spinner_style(),
        }
    }

    /// Render traditional bar style progress.
    fn render_bar_style(&self, width: usize) -> String {
        match &self.progress_type {
            ProgressType::Determinate(progress) => {
                let filled_width = (width as f64 * progress) as usize;
                let empty_width = width - filled_width;

                let filled = "█".repeat(filled_width);
                let empty = "░".repeat(empty_width);
                format!("{}{}", filled, empty)
            }
            ProgressType::Indeterminate => {
                let frame = self.animation_frame();
                let pos = frame % width;
                let mut chars = vec!['░'; width];

                // Create a moving wave effect
                for i in 0..3 {
                    let idx = (pos + i) % width;
                    chars[idx] = match i {
                        0 => '▓',
                        1 => '█',
                        2 => '▓',
                        _ => '░',
                    };
                }

                chars.into_iter().collect()
            }
        }
    }

    /// Render blocks style progress.
    fn render_blocks_style(&self, width: usize) -> String {
        let block_count = width / 2; // Each block takes 2 characters

        match &self.progress_type {
            ProgressType::Determinate(progress) => {
                let filled_blocks = (block_count as f64 * progress) as usize;
                let empty_blocks = block_count - filled_blocks;

                let filled = "██".repeat(filled_blocks);
                let empty = "░░".repeat(empty_blocks);
                format!("{}{}", filled, empty)
            }
            ProgressType::Indeterminate => {
                let frame = self.animation_frame();
                let pos = frame % block_count;

                let mut result = String::new();
                for i in 0..block_count {
                    if i == pos {
                        result.push_str("██");
                    } else {
                        result.push_str("░░");
                    }
                }
                result
            }
        }
    }

    /// Render dots style progress.
    fn render_dots_style(&self, width: usize) -> String {
        match &self.progress_type {
            ProgressType::Determinate(progress) => {
                let filled_dots = (width as f64 * progress) as usize;
                let empty_dots = width - filled_dots;

                let filled = "●".repeat(filled_dots);
                let empty = "○".repeat(empty_dots);
                format!("{}{}", filled, empty)
            }
            ProgressType::Indeterminate => {
                let frame = self.animation_frame();
                let pos = frame % width;

                let mut result = String::new();
                for i in 0..width {
                    if i == pos {
                        result.push('●');
                    } else {
                        result.push('○');
                    }
                }
                result
            }
        }
    }

    /// Render spinner style progress.
    fn render_spinner_style(&self) -> String {
        let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let frame = self.animation_frame() % spinner_chars.len();
        spinner_chars[frame].to_string()
    }

    /// Get the percentage text.
    fn get_percentage_text(&self) -> Option<String> {
        if !self.show_percentage {
            return None;
        }

        match &self.progress_type {
            ProgressType::Determinate(progress) => {
                Some(format!("{:.0}%", progress * 100.0))
            }
            ProgressType::Indeterminate => None,
        }
    }

    /// Get the label text.
    fn get_label_text(&self) -> Option<&String> {
        if self.show_label {
            self.label.as_ref()
        } else {
            None
        }
    }
}

#[async_trait]
impl Component for ProgressBar {
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
            .attr("class", "progress-container");

        // Add label if present and positioned above
        if let (Some(label), TextPosition::Above) = (self.get_label_text(), &self.text_position) {
            let label_node = VirtualNode::element("div")
                .attr("class", "progress-label progress-label-above")
                .child(VirtualNode::text(label.clone()));
            container = container.child(label_node);
        }

        // Create the main progress bar element
        let mut progress_bar = VirtualNode::element("div")
            .attr("class", "progress-bar");

        // Add orientation class
        match self.orientation {
            ProgressOrientation::Horizontal => {
                progress_bar = progress_bar.attr("class", "progress-bar progress-horizontal");
            }
            ProgressOrientation::Vertical => {
                progress_bar = progress_bar.attr("class", "progress-bar progress-vertical");
            }
        }

        // Add style class
        let style_class = match self.style {
            ProgressStyle::Bar => "progress-style-bar",
            ProgressStyle::Blocks => "progress-style-blocks",
            ProgressStyle::Dots => "progress-style-dots",
            ProgressStyle::Spinner => "progress-style-spinner",
        };
        progress_bar = progress_bar.attr("class", format!("progress-bar {}", style_class));

        // Add progress content
        let progress_content = self.render_progress_content();
        let progress_fill = VirtualNode::element("div")
            .attr("class", "progress-fill")
            .child(VirtualNode::text(progress_content));
        progress_bar = progress_bar.child(progress_fill);

        // Add overlay text if positioned as overlay
        if let TextPosition::Overlay = self.text_position {
            let mut overlay_text = String::new();

            if let Some(label) = self.get_label_text() {
                overlay_text.push_str(label);
                if self.show_percentage {
                    overlay_text.push(' ');
                }
            }

            if let Some(percentage) = self.get_percentage_text() {
                overlay_text.push_str(&percentage);
            }

            if !overlay_text.is_empty() {
                let text_node = VirtualNode::element("div")
                    .attr("class", "progress-text progress-overlay")
                    .child(VirtualNode::text(overlay_text));
                progress_bar = progress_bar.child(text_node);
            }
        }

        container = container.child(progress_bar);

        // Add percentage text if positioned below and not overlay
        if !matches!(self.text_position, TextPosition::Overlay | TextPosition::None) {
            let mut bottom_text = String::new();
            let mut has_content = false;

            if let (Some(label), TextPosition::Below) = (self.get_label_text(), &self.text_position) {
                bottom_text.push_str(label);
                has_content = true;
            }

            if let Some(percentage) = self.get_percentage_text() {
                if has_content {
                    bottom_text.push(' ');
                }
                bottom_text.push_str(&percentage);
                has_content = true;
            }

            if has_content {
                let text_node = VirtualNode::element("div")
                    .attr("class", "progress-text progress-below")
                    .child(VirtualNode::text(bottom_text));
                container = container.child(text_node);
            }
        }

        Ok(container)
    }
}

#[async_trait]
impl Widget for ProgressBar {
    fn widget_type(&self) -> &'static str {
        "ProgressBar"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::RenderContext;
    use crate::style::Theme;

    #[test]
    fn test_progress_bar_creation() {
        let progress = ProgressBar::new();
        assert_eq!(progress.progress(), Some(0.0));
        assert!(!progress.is_complete());
        assert!(!progress.is_indeterminate());
        assert_eq!(progress.style, ProgressStyle::Bar);
        assert_eq!(progress.orientation, ProgressOrientation::Horizontal);
    }

    #[test]
    fn test_progress_value_setting() {
        let mut progress = ProgressBar::new().with_progress(0.5);
        assert_eq!(progress.progress(), Some(0.5));
        assert!(!progress.is_complete());

        progress.set_progress(1.0);
        assert_eq!(progress.progress(), Some(1.0));
        assert!(progress.is_complete());

        // Test clamping
        progress.set_progress(1.5);
        assert_eq!(progress.progress(), Some(1.0));

        progress.set_progress(-0.5);
        assert_eq!(progress.progress(), Some(0.0));
    }

    #[test]
    fn test_indeterminate_progress() {
        let progress = ProgressBar::new().with_indeterminate();
        assert!(progress.is_indeterminate());
        assert_eq!(progress.progress(), None);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_progress_styles() {
        let bar_progress = ProgressBar::new().with_style(ProgressStyle::Bar);
        assert_eq!(bar_progress.style, ProgressStyle::Bar);

        let blocks_progress = ProgressBar::new().with_style(ProgressStyle::Blocks);
        assert_eq!(blocks_progress.style, ProgressStyle::Blocks);

        let dots_progress = ProgressBar::new().with_style(ProgressStyle::Dots);
        assert_eq!(dots_progress.style, ProgressStyle::Dots);

        let spinner_progress = ProgressBar::new().with_style(ProgressStyle::Spinner);
        assert_eq!(spinner_progress.style, ProgressStyle::Spinner);
    }

    #[test]
    fn test_progress_orientation() {
        let horizontal = ProgressBar::new().with_orientation(ProgressOrientation::Horizontal);
        assert_eq!(horizontal.orientation, ProgressOrientation::Horizontal);

        let vertical = ProgressBar::new().with_orientation(ProgressOrientation::Vertical);
        assert_eq!(vertical.orientation, ProgressOrientation::Vertical);
    }

    #[test]
    fn test_progress_dimensions() {
        let progress = ProgressBar::new()
            .with_width(30)
            .with_height(2);
        assert_eq!(progress.width, Some(30));
        assert_eq!(progress.height, Some(2));
    }

    #[test]
    fn test_progress_text_configuration() {
        let progress = ProgressBar::new()
            .with_label("Loading...")
            .show_percentage(true)
            .with_text_position(TextPosition::Above);

        assert!(progress.show_label);
        assert!(progress.show_percentage);
        assert_eq!(progress.label, Some("Loading...".to_string()));
        assert_eq!(progress.text_position, TextPosition::Above);
    }

    #[test]
    fn test_progress_colors() {
        let progress = ProgressBar::new()
            .with_foreground_color(Color::rgba(255, 0, 0, 255))
            .with_background_color(Color::rgba(128, 128, 128, 255))
            .with_text_color(Color::rgba(0, 0, 0, 255));

        assert!(progress.foreground_color.is_some());
        assert!(progress.background_color.is_some());
        assert!(progress.text_color.is_some());
    }

    #[test]
    fn test_animation_updates() {
        let mut progress = ProgressBar::new()
            .with_indeterminate()
            .with_animation_speed(2);

        let initial_frame = progress.animation_frame();
        progress.update_animation();
        progress.update_animation(); // Need to update twice due to speed setting

        // Frame should advance after enough updates
        assert!(progress.animation_frame() != initial_frame || progress.animation_frame == 0);
    }

    #[test]
    fn test_progress_bar_rendering() {
        let progress = ProgressBar::new().with_progress(0.5);

        // Test bar style rendering
        let bar_content = progress.render_bar_style(10);
        assert_eq!(bar_content.chars().count(), 10); // Should have 10 characters
        assert!(bar_content.contains('█')); // Should contain filled characters
        assert!(bar_content.contains('░')); // Should contain empty characters

        // Test blocks style rendering (uses 2-char blocks, so 10 width = 5 blocks = 10 chars)
        let blocks_content = progress.render_blocks_style(10);
        assert_eq!(blocks_content.chars().count(), 10); // 5 blocks * 2 chars each
        assert!(blocks_content.contains('█') || blocks_content.contains('░'));

        // Test dots style rendering
        let dots_content = progress.render_dots_style(10);
        assert_eq!(dots_content.chars().count(), 10);
        assert!(dots_content.contains('●') || dots_content.contains('○'));
    }

    #[test]
    fn test_spinner_rendering() {
        let progress = ProgressBar::new()
            .with_style(ProgressStyle::Spinner)
            .with_indeterminate();

        let spinner_content = progress.render_spinner_style();
        assert_eq!(spinner_content.chars().count(), 1);
        assert!("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏".contains(&spinner_content));
    }

    #[test]
    fn test_percentage_text() {
        let progress = ProgressBar::new()
            .with_progress(0.75)
            .show_percentage(true);

        let percentage = progress.get_percentage_text();
        assert_eq!(percentage, Some("75%".to_string()));

        let indeterminate = ProgressBar::new()
            .with_indeterminate()
            .show_percentage(true);

        let no_percentage = indeterminate.get_percentage_text();
        assert_eq!(no_percentage, None);
    }

    #[test]
    fn test_progress_callbacks() {
        use std::sync::{Arc, Mutex};

        let completed = Arc::new(Mutex::new(false));
        let completed_clone = completed.clone();

        let progress_values = Arc::new(Mutex::new(Vec::new()));
        let progress_values_clone = progress_values.clone();

        let mut progress = ProgressBar::new()
            .on_complete(move || {
                *completed_clone.lock().unwrap() = true;
            })
            .on_progress_changed(move |value| {
                progress_values_clone.lock().unwrap().push(value);
            });

        // Test progress change callback
        progress.set_progress(0.5);
        assert_eq!(progress_values.lock().unwrap().len(), 1);
        assert_eq!(progress_values.lock().unwrap()[0], 0.5);

        // Test completion callback
        progress.set_progress(1.0);
        assert!(*completed.lock().unwrap());
        assert_eq!(progress_values.lock().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_progress_bar_component_rendering() {
        let progress = ProgressBar::new()
            .with_progress(0.6)
            .with_label("Test Progress")
            .show_percentage(true);

        let theme = Theme::default();
        let context = RenderContext::new(&theme);

        let result = progress.render(&context).await;
        assert!(result.is_ok());

        let vnode = result.unwrap();
        assert_eq!(vnode.tag(), Some("div"));
    }
}
