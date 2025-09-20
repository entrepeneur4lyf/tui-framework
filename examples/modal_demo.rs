use tui_framework::prelude::*;
use tui_framework::widget::modal::*;
use tui_framework::widget::button::Button;
use tui_framework::widget::text::Text;
use tui_framework::style::{Color, StyleBuilder};
use tui_framework::render::VirtualNode;
use std::sync::{Arc, Mutex};

/// Modal demo application showcasing various modal types and features.
#[derive(Debug)]
struct ModalDemoApp {
    /// Currently open modal type
    current_modal: Arc<Mutex<Option<String>>>,
    /// Demo state
    counter: Arc<Mutex<i32>>,
}

impl ModalDemoApp {
    fn new() -> Self {
        Self {
            current_modal: Arc::new(Mutex::new(None)),
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl Component for ModalDemoApp {
    fn id(&self) -> ComponentId {
        ComponentId::new()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let current_modal = self.current_modal.lock().unwrap().clone();
        let counter_value = *self.counter.lock().unwrap();

        // Simple demo content
        let title = Text::new("🎭 Modal Demo Application")
            .with_style(
                StyleBuilder::new()
                    .color(Color::rgba(100, 200, 255, 255))
                    .build()
            );

        let description = Text::new("This demo showcases the Modal widget functionality");

        let counter_text = Text::new(format!("Counter: {}", counter_value))
            .with_style(
                StyleBuilder::new()
                    .color(Color::rgba(150, 255, 150, 255))
                    .build()
            );

        // Create a simple button to open a modal
        let current_modal_clone = self.current_modal.clone();
        let open_button = Button::new("Open Modal")
            .on_click(move || {
                *current_modal_clone.lock().unwrap() = Some("dialog".to_string());
            });

        let mut content = VirtualNode::element("div")
            .child(title.render(context).await?)
            .child(description.render(context).await?)
            .child(open_button.render(context).await?)
            .child(counter_text.render(context).await?);

        // Add modal if one is open
        if let Some(modal_type) = current_modal {
            let modal = self.create_modal(&modal_type).await?;
            content = content.child(modal.render(context).await?);
        }

        Ok(content)
    }
}

impl ModalDemoApp {

    /// Create a modal based on the type.
    async fn create_modal(&self, modal_type: &str) -> Result<Modal> {
        let current_modal = self.current_modal.clone();
        let counter = self.counter.clone();

        let modal = match modal_type {
            "dialog" => {
                let current_modal_confirm = current_modal.clone();
                let current_modal_cancel = current_modal.clone();

                Modal::new()
                    .with_title("Simple Dialog")
                    .with_type(ModalType::Dialog)
                    .with_size(ModalSize::Medium)
                    .with_position(ModalPosition::Center)
                    .with_content(VirtualNode::text("This is a simple dialog modal with basic functionality."))
                    .with_confirm_button("OK")
                    .with_cancel_button("Cancel")
                    .on_confirm(move || {
                        *current_modal_confirm.lock().unwrap() = None;
                    })
                    .on_cancel(move || {
                        *current_modal_cancel.lock().unwrap() = None;
                    })
            }
            "confirmation" => {
                let current_modal_confirm = current_modal.clone();
                let current_modal_cancel = current_modal.clone();
                let counter_clone = counter.clone();

                Modal::new()
                    .with_title("Confirm Action")
                    .with_type(ModalType::Confirmation)
                    .with_size(ModalSize::Small)
                    .with_content(VirtualNode::text("Are you sure you want to increment the counter?"))
                    .with_confirm_button("Yes, Increment")
                    .with_cancel_button("Cancel")
                    .on_confirm(move || {
                        *counter_clone.lock().unwrap() += 1;
                        *current_modal_confirm.lock().unwrap() = None;
                    })
                    .on_cancel(move || {
                        *current_modal_cancel.lock().unwrap() = None;
                    })
            }
            "alert" => {
                let current_modal_clone = current_modal.clone();
                Modal::new()
                    .with_title("Alert")
                    .with_type(ModalType::Alert)
                    .with_size(ModalSize::Small)
                    .with_content(VirtualNode::text("⚠️ This is an important alert message!"))
                    .with_confirm_button("Understood")
                    .close_on_backdrop_click(false)
                    .on_confirm(move || {
                        *current_modal_clone.lock().unwrap() = None;
                    })
            }
            "large" => {
                let current_modal_clone = current_modal.clone();
                Modal::new()
                    .with_title("Large Modal")
                    .with_type(ModalType::Dialog)
                    .with_size(ModalSize::Large)
                    .with_content(VirtualNode::text("This is a large modal that demonstrates how modals can be sized differently. It contains more content and provides more space for complex interactions."))
                    .with_confirm_button("Close")
                    .draggable(true)
                    .resizable(true)
                    .on_confirm(move || {
                        *current_modal_clone.lock().unwrap() = None;
                    })
            }
            "custom" => {
                let current_modal_clone = current_modal.clone();
                Modal::new()
                    .with_title("Custom Modal")
                    .with_type(ModalType::Custom)
                    .with_size(ModalSize::Custom { width: 600, height: 400 })
                    .with_position(ModalPosition::TopCenter)
                    .with_content(VirtualNode::text("This is a custom-sized modal positioned at the top center."))
                    .with_confirm_button("Got it")
                    .show_backdrop(true)
                    .on_confirm(move || {
                        *current_modal_clone.lock().unwrap() = None;
                    })
            }
            "animated" => {
                let current_modal_clone = current_modal.clone();
                Modal::new()
                    .with_title("Animated Modal")
                    .with_type(ModalType::Dialog)
                    .with_size(ModalSize::Medium)
                    .with_animation(ModalAnimation::Scale)
                    .with_content(VirtualNode::text("This modal uses scale animation for smooth entrance and exit."))
                    .with_confirm_button("Cool!")
                    .on_confirm(move || {
                        *current_modal_clone.lock().unwrap() = None;
                    })
            }
            _ => {
                let current_modal_clone = current_modal.clone();
                Modal::new()
                    .with_title("Unknown Modal")
                    .with_content(VirtualNode::text("Unknown modal type"))
                    .with_confirm_button("OK")
                    .on_confirm(move || {
                        *current_modal_clone.lock().unwrap() = None;
                    })
            }
        };

        Ok(modal)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎭 Starting Modal Demo Application...");
    
    let _app = ModalDemoApp::new();
    
    // In a real application, this would be handled by the framework's event loop
    println!("Modal Demo Application initialized successfully!");
    println!("This demo showcases:");
    println!("  • Simple Dialog Modals");
    println!("  • Confirmation Modals");
    println!("  • Alert Modals");
    println!("  • Large Modals with drag/resize");
    println!("  • Custom-sized Modals");
    println!("  • Animated Modals");
    println!("  • Focus management and keyboard navigation");
    println!("  • Backdrop interaction");
    println!("  • Modal stacking with z-index management");
    
    Ok(())
}
