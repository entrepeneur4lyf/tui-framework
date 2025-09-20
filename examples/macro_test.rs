//! Test the component macro functionality.

use tui_framework::prelude::*;

#[component]
fn hello_world(name: String) -> VirtualNode {
    VirtualNode::element("div").child(VirtualNode::text(format!("Hello, {}!", name)))
}

#[component]
fn counter(initial_value: i32) -> VirtualNode {
    VirtualNode::element("div").child(VirtualNode::text(format!("Count: {}", initial_value)))
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing component macro functionality");

    // Test creating components
    let hello = HelloWorldComponent::new("World".to_string());
    let counter = CounterComponent::new(42);

    // Test rendering (basic test)
    let theme = Theme::dark();
    let context = RenderContext::new(&theme);

    let _hello_vnode = hello.render(&context).await?;
    let _counter_vnode = counter.render(&context).await?;

    println!("Hello component rendered successfully");
    println!("Counter component rendered successfully");

    // Test component IDs
    println!("Hello component ID: {:?}", hello.id());
    println!("Counter component ID: {:?}", counter.id());

    Ok(())
}
