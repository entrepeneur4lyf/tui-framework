//! Test the Theme derive macro functionality.

use tui_framework::style::{Color, Theme};
use tui_framework_macros::Theme as ThemeDerive;

// Test 1: Full theme with all standard fields
#[derive(Debug, Clone, ThemeDerive)]
struct FullTheme {
    primary: Color,
    secondary: Color,
    background: Color,
    surface: Color,
    text: Color,
    text_on_primary: Color,
    error: Color,
    warning: Color,
    success: Color,
    info: Color,
}

// Test 2: Minimal theme with only essential fields
#[derive(Debug, Clone, ThemeDerive)]
struct MinimalTheme {
    primary: Color,
    background: Color,
    text: Color,
}

// Test 3: Custom theme with additional fields
#[derive(Debug, Clone, ThemeDerive)]
struct CustomTheme {
    primary: Color,
    secondary: Color,
    background: Color,
    text: Color,
    #[allow(dead_code)]
    accent: Color, // Custom field
    #[allow(dead_code)]
    highlight: Color, // Custom field
}

// Test 4: Gaming theme with specific colors
#[derive(Debug, Clone, ThemeDerive)]
struct GamingTheme {
    primary: Color,
    secondary: Color,
    background: Color,
    surface: Color,
    text: Color,
    error: Color,
    success: Color,
}

fn main() {
    println!("Testing Theme derive macro functionality");
    println!("=======================================");

    // Test 1: Full theme
    println!("\n1. Testing FullTheme:");
    let full_theme = FullTheme::default();
    println!("   Default FullTheme: {:?}", full_theme);

    let full_dark = FullTheme::dark();
    println!("   Dark FullTheme: {:?}", full_dark);

    let full_light = FullTheme::light();
    println!("   Light FullTheme: {:?}", full_light);

    // Test conversion to base Theme
    let base_theme: Theme = full_theme.clone().into();
    println!("   Converted to base Theme: {:?}", base_theme);

    // Test 2: Minimal theme
    println!("\n2. Testing MinimalTheme:");
    let minimal_theme = MinimalTheme::default();
    println!("   Default MinimalTheme: {:?}", minimal_theme);

    let minimal_base: Theme = minimal_theme.clone().into();
    println!("   Converted to base Theme: {:?}", minimal_base);

    // Test 3: Custom theme
    println!("\n3. Testing CustomTheme:");
    let custom_theme = CustomTheme::default();
    println!("   Default CustomTheme: {:?}", custom_theme);

    // Test 4: Gaming theme with custom colors
    println!("\n4. Testing GamingTheme with custom colors:");
    let gaming_theme = GamingTheme {
        primary: Color::rgb(255, 0, 128),   // Hot pink
        secondary: Color::rgb(0, 255, 255), // Cyan
        background: Color::rgb(10, 10, 15), // Very dark
        surface: Color::rgb(20, 20, 30),    // Dark surface
        text: Color::rgb(255, 255, 255),    // White text
        error: Color::rgb(255, 50, 50),     // Bright red
        success: Color::rgb(50, 255, 50),   // Bright green
    };
    println!("   Gaming theme: {:?}", gaming_theme);

    let gaming_base: Theme = gaming_theme.clone().into();
    println!("   Gaming theme as base: {:?}", gaming_base);

    // Test 5: Conversion from base Theme
    println!("\n5. Testing conversion from base Theme:");
    let base_dark = Theme::dark();
    let converted_full: FullTheme = base_dark.clone().into();
    println!("   Base dark -> FullTheme: {:?}", converted_full);

    let converted_minimal: MinimalTheme = base_dark.clone().into();
    println!("   Base dark -> MinimalTheme: {:?}", converted_minimal);

    // Test 6: Round-trip conversion
    println!("\n6. Testing round-trip conversion:");
    let original = FullTheme::light();
    let base: Theme = original.clone().into();
    let back: FullTheme = base.into();

    println!(
        "   Original: primary={:?}, background={:?}",
        original.primary, original.background
    );
    println!(
        "   Round-trip: primary={:?}, background={:?}",
        back.primary, back.background
    );

    // Verify they match
    let colors_match = original.primary == back.primary
        && original.background == back.background
        && original.text == back.text;
    println!("   Colors match after round-trip: {}", colors_match);

    println!("\nTheme derive macro test completed successfully!");
}
