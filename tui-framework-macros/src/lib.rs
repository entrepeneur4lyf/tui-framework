//! Procedural macros for the TUI framework.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

/// Macro for creating components.
#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    
    // For now, just return the original function
    // TODO: Implement component transformation
    quote! {
        #input_fn
    }.into()
}

/// Macro for creating CSS-like styles.
///
/// This macro provides a simple way to create styles using CSS-like syntax.
/// It supports basic properties like colors, dimensions, and spacing.
///
/// # Example
/// ```rust,ignore
/// use tui_framework_macros::css;
///
/// let style = css! {
///     "background-color: blue; color: white; width: 100px;"
/// };
/// ```
///
/// Or with utility classes:
/// ```rust,ignore
/// let style = css! {
///     "bg-blue text-white w-full"
/// };
/// ```
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    if input_str.trim().is_empty() {
        // Empty input, return default StyleBuilder
        quote! {
            tui_framework::style::StyleBuilder::new()
        }.into()
    } else {
        // Check if input looks like utility classes (no colons or semicolons)
        let cleaned_input = input_str.trim().trim_matches('"');

        if cleaned_input.contains(':') || cleaned_input.contains(';') {
            // CSS property syntax - parse as CSS properties
            let properties: Vec<&str> = cleaned_input.split(';')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            let mut builder_calls = Vec::new();

            for property in properties {
                if let Some((key, value)) = property.split_once(':') {
                    let key = key.trim();
                    let value = value.trim();
                    builder_calls.push(quote! {
                        builder = tui_framework::style::css::apply_css_property(builder, #key, #value);
                    });
                }
            }

            quote! {
                {
                    let mut builder = tui_framework::style::StyleBuilder::new();
                    #(#builder_calls)*
                    builder
                }
            }.into()
        } else {
            // Utility class syntax
            quote! {
                tui_framework::style::css::apply_utility_classes(
                    tui_framework::style::StyleBuilder::new(),
                    #cleaned_input
                )
            }.into()
        }
    }
}

/// Macro for creating themes.
#[proc_macro_derive(Theme)]
pub fn derive_theme(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    // For now, just implement Default
    // TODO: Implement proper theme derivation
    quote! {
        impl Default for #name {
            fn default() -> Self {
                tui_framework::style::Theme::dark().into()
            }
        }
    }.into()
}
