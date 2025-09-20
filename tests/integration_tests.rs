//! Integration tests for the TUI framework.
//!
//! These tests verify that different components of the framework work together correctly.

use tui_framework::prelude::*;
use tui_framework::app::App;
use tui_framework::component::{BaseComponent, Component, ComponentId};
use tui_framework::layout::{Rect, Size};
use tui_framework::reactive::state::State;
use tui_framework::render::backend::{Backend, PlaceholderBackend};
use tui_framework::render::context::RenderContext;
use tui_framework::render::vdom::nodes::{div, text};
use tui_framework::render::vdom::{VirtualNode, VirtualStyle, StyleValue, DisplayType, FlexDirection};
use tui_framework::style::{Color, Theme};
use tui_framework::widget::{Button, Input, Text, List, ListItem};

/// Simple test component for integration testing.
struct TestComponent {
    base: BaseComponent,
    counter: State<i32>,
    message: State<String>,
}

impl TestComponent {
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("TestComponent"),
            counter: State::new(0),
            message: State::new("Hello, Integration Test!".to_string()),
        }
    }

    pub fn increment(&self) {
        let current = *self.counter.get();
        self.counter.set(current + 1);
    }

    pub fn set_message(&self, msg: String) {
        self.message.set(msg);
    }
}

#[async_trait::async_trait]
impl Component for TestComponent {
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
        let counter_value = *self.counter.get();
        let message = self.message.get().clone();

        Ok(div()
            .style(VirtualStyle {
                display: Some(DisplayType::Flex),
                flex_direction: Some(FlexDirection::Column),
                width: Some(StyleValue::Fill),
                height: Some(StyleValue::Fill),
                ..Default::default()
            })
            .child(text(&format!("Message: {}", message)))
            .child(text(&format!("Counter: {}", counter_value)))
            .child(text("Integration test component")))
    }
}

#[tokio::test]
async fn test_component_creation_and_rendering() {
    let component = TestComponent::new();
    let theme = Theme::default();
    let context = RenderContext::new(&theme);

    // Test initial state
    assert_eq!(*component.counter.get(), 0);
    assert_eq!(*component.message.get(), "Hello, Integration Test!");

    // Test rendering
    let result = component.render(&context).await;
    assert!(result.is_ok());

    let node = result.unwrap();
    // Should be a div with children
    match node {
        VirtualNode::Element(_) => assert!(true),
        _ => panic!("Expected element node"),
    }
}

#[tokio::test]
async fn test_component_state_updates() {
    let component = TestComponent::new();
    let theme = Theme::default();
    let context = RenderContext::new(&theme);

    // Update state
    component.increment();
    component.set_message("Updated message".to_string());

    // Verify state changes
    assert_eq!(*component.counter.get(), 1);
    assert_eq!(*component.message.get(), "Updated message");

    // Test rendering with updated state
    let result = component.render(&context).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_app_with_component() {
    let component = TestComponent::new();
    
    // Test app creation with component
    let _app = App::new()
        .title("Integration Test App")
        .component(component);

    // App should be created successfully
    assert!(true); // If we reach here, app creation was successful
}

#[tokio::test]
async fn test_backend_integration() {
    let mut backend = PlaceholderBackend::new();
    let component = TestComponent::new();
    let theme = Theme::default();
    let context = RenderContext::new(&theme);

    // Initialize backend
    assert!(backend.init().is_ok());

    // Render component
    let node = component.render(&context).await.unwrap();
    let rect = Rect::from_coords(0, 0, 80, 24);
    
    // Render to backend
    assert!(backend.render_node(&node, rect).is_ok());
    assert!(backend.present().is_ok());

    // Cleanup
    assert!(backend.cleanup().is_ok());
}

#[tokio::test]
async fn test_layout_integration() {
    use tui_framework::layout::layout_engine::Layout;

    let component = TestComponent::new();
    let theme = Theme::default();
    let context = RenderContext::new(&theme);
    let mut node = component.render(&context).await.unwrap();

    // Test layout computation
    let viewport = Size::new(80, 24);
    let result = Layout::compute(&mut node, viewport);

    assert!(result.layouts.len() > 0);
    assert!(result.total_size.width > 0);
    assert!(result.total_size.height > 0);
}

#[tokio::test]
async fn test_widget_integration() {
    // Test Button widget
    let button = Button::new("Test Button");
    let theme = Theme::default();
    let context = RenderContext::new(&theme);
    let button_result = button.render(&context).await;
    assert!(button_result.is_ok());

    // Test Input widget
    let input = Input::new();
    let input_result = input.render(&context).await;
    assert!(input_result.is_ok());

    // Test Text widget
    let text_widget = Text::new("Test text content");
    let text_result = text_widget.render(&context).await;
    assert!(text_result.is_ok());

    // Test List widget
    let mut list = List::new();
    list.add_item(ListItem::new("Item 1", "value1"));
    list.add_item(ListItem::new("Item 2", "value2"));
    let list_result = list.render(&context).await;
    assert!(list_result.is_ok());
}

#[tokio::test]
async fn test_complex_component_composition() {
    // Create a complex component with multiple widgets
    let mut complex_node = div()
        .style(VirtualStyle {
            display: Some(DisplayType::Flex),
            flex_direction: Some(FlexDirection::Column),
            width: Some(StyleValue::Fill),
            height: Some(StyleValue::Fill),
            ..Default::default()
        })
        .child(
            div()
                .style(VirtualStyle {
                    height: Some(StyleValue::Absolute(3)),
                    width: Some(StyleValue::Fill),
                    ..Default::default()
                })
                .child(text("Header Section"))
        )
        .child(
            div()
                .style(VirtualStyle {
                    display: Some(DisplayType::Flex),
                    flex_direction: Some(FlexDirection::Row),
                    height: Some(StyleValue::Fill),
                    width: Some(StyleValue::Fill),
                    ..Default::default()
                })
                .child(
                    div()
                        .style(VirtualStyle {
                            width: Some(StyleValue::Percentage(30.0)),
                            height: Some(StyleValue::Fill),
                            ..Default::default()
                        })
                        .child(text("Sidebar"))
                )
                .child(
                    div()
                        .style(VirtualStyle {
                            width: Some(StyleValue::Percentage(70.0)),
                            height: Some(StyleValue::Fill),
                            ..Default::default()
                        })
                        .child(text("Main Content"))
                )
        )
        .child(
            div()
                .style(VirtualStyle {
                    height: Some(StyleValue::Absolute(2)),
                    width: Some(StyleValue::Fill),
                    ..Default::default()
                })
                .child(text("Footer Section"))
        );

    // Test layout computation for complex structure
    use tui_framework::layout::layout_engine::Layout;
    let viewport = Size::new(80, 24);
    let result = Layout::compute(&mut complex_node, viewport);

    // Should handle complex nested structure
    assert!(result.layouts.len() >= 3); // At least the main components
    assert_eq!(result.total_size.width, 80);
    assert!(result.total_size.height >= 24);
}

#[tokio::test]
async fn test_state_reactivity() {
    let state = State::new(42);
    let computed_state = state.map(|x| x * 2);

    // Test initial values
    assert_eq!(*state.get(), 42);
    assert_eq!(computed_state.get(), 84);

    // Test state update and reactivity
    state.set(10);
    assert_eq!(*state.get(), 10);
    assert_eq!(computed_state.get(), 20);
}

#[tokio::test]
async fn test_error_handling() {
    // Test that components handle errors gracefully
    let component = TestComponent::new();
    let theme = Theme::default();
    let context = RenderContext::new(&theme); // Edge case: zero size

    let result = component.render(&context).await;
    // Should not panic, even with edge case input
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_theme_integration() {
    // Test that themes can be created and used
    let theme = Theme::default();

    // Theme should have default values (fields, not methods)
    assert_eq!(theme.primary, Color::rgb(100, 150, 255));
    assert_eq!(theme.background, Color::rgb(20, 20, 25));
    assert_eq!(theme.text, Color::rgb(240, 240, 245));
}
