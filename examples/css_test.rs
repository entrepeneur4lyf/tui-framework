//! Test the CSS macro functionality.

use tui_framework::style::{Color, StyleBuilder, StyleValue};
use tui_framework_macros::css;

fn main() {
    println!("Testing CSS macro functionality");
    println!("================================");

    // Test 1: Empty CSS
    println!("\n1. Testing empty CSS:");
    let empty_style = css! {};
    let style = empty_style.build();
    println!("   Empty style created: {:?}", style);

    // Test 2: Utility classes
    println!("\n2. Testing utility classes:");
    let utility_style = css! { "bg-blue text-white w-full" };
    let style = utility_style.build();
    println!("   Utility style: {:?}", style);
    println!("   Background: {:?}", style.background_color);
    println!("   Color: {:?}", style.color);
    println!("   Width: {:?}", style.width);

    // Test 3: CSS properties
    println!("\n3. Testing CSS properties:");
    let css_style = css! { "background-color: red; color: white; width: 100;" };
    let style = css_style.build();
    println!("   CSS property style: {:?}", style);
    println!("   Background: {:?}", style.background_color);
    println!("   Color: {:?}", style.color);
    println!("   Width: {:?}", style.width);

    // Test 4: Manual StyleBuilder for comparison
    println!("\n4. Manual StyleBuilder for comparison:");
    let manual_style = StyleBuilder::new()
        .background_color(Color::GREEN)
        .color(Color::BLACK)
        .width(StyleValue::Percentage(0.5))
        .build();
    println!("   Manual style: {:?}", manual_style);

    // Test 5: Individual utility classes
    println!("\n5. Testing individual utility classes:");

    let bg_test = css! { "bg-red" };
    let bg_style = bg_test.build();
    println!("   bg-red: {:?}", bg_style.background_color);

    let text_test = css! { "text-green" };
    let text_style = text_test.build();
    println!("   text-green: {:?}", text_style.color);

    let width_test = css! { "w-auto" };
    let width_style = width_test.build();
    println!("   w-auto: {:?}", width_style.width);

    println!("\nCSS macro test completed successfully!");
}
