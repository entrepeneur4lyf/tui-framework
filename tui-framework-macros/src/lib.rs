//! Procedural macros for the TUI framework.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect()
}

/// Macro for creating components.
///
/// This macro transforms a function into a Component implementation.
/// The function should return a VirtualNode and can take props as parameters.
///
/// # Example
/// ```rust,ignore
/// #[component]
/// fn MyComponent(name: String) -> VirtualNode {
///     VirtualNode::element("div")
///         .child(VirtualNode::text(format!("Hello, {}!", name)))
/// }
/// ```
#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    // Generate a struct name based on the function name (convert to PascalCase)
    let fn_name_str = fn_name.to_string();
    let pascal_case_name = to_pascal_case(&fn_name_str);
    let struct_name = syn::Ident::new(&format!("{}Component", pascal_case_name), fn_name.span());

    // Extract parameter types for the props struct
    let mut prop_fields = Vec::new();
    let mut prop_names = Vec::new();

    for input in fn_inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                let field_name = &pat_ident.ident;
                let field_type = &pat_type.ty;
                prop_fields.push(quote! { pub #field_name: #field_type });
                prop_names.push(field_name);
            }
        }
    }

    // Generate the component struct and implementation
    let expanded = quote! {
        // Props struct
        #[derive(Debug, Clone)]
        #fn_vis struct #struct_name {
            #(#prop_fields,)*
        }

        impl #struct_name {
            pub fn new(#fn_inputs) -> Self {
                Self {
                    #(#prop_names,)*
                }
            }
        }

        #[async_trait::async_trait]
        impl tui_framework::component::Component for #struct_name {
            fn id(&self) -> tui_framework::component::ComponentId {
                tui_framework::component::ComponentId::new()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            async fn render(&self, _context: &tui_framework::render::RenderContext) -> tui_framework::error::Result<tui_framework::render::vdom::VirtualNode> {
                let #struct_name { #(#prop_names,)* } = self;
                #(let #prop_names = #prop_names.clone();)*

                let result = #fn_block;
                Ok(result)
            }
        }

        // Keep the original function for direct usage
        #fn_vis fn #fn_name(#fn_inputs) #fn_output #fn_block
    };

    expanded.into()
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
        }
        .into()
    } else {
        // Check if input looks like utility classes (no colons or semicolons)
        let cleaned_input = input_str.trim().trim_matches('"');

        if cleaned_input.contains(':') || cleaned_input.contains(';') {
            // CSS property syntax - parse as CSS properties
            let properties: Vec<&str> = cleaned_input
                .split(';')
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
            }
            .into()
        } else {
            // Utility class syntax
            quote! {
                tui_framework::style::css::apply_utility_classes(
                    tui_framework::style::StyleBuilder::new(),
                    #cleaned_input
                )
            }
            .into()
        }
    }
}

/// Macro for creating themes.
///
/// This macro derives theme functionality for structs with color fields.
/// It automatically implements conversion to/from the base Theme type and
/// provides validation and defaults.
///
/// # Example
/// ```rust,ignore
/// use tui_framework_macros::Theme;
/// use tui_framework::style::Color;
///
/// #[derive(Theme)]
/// struct MyTheme {
///     primary: Color,
///     secondary: Color,
///     background: Color,
///     text: Color,
/// }
/// ```
#[proc_macro_derive(Theme)]
pub fn derive_theme(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Extract struct fields
    let fields = match input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "Theme derive only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "Theme derive only supports structs")
                .to_compile_error()
                .into();
        }
    };

    // Generate field assignments for From<Theme> implementation
    let mut from_theme_assignments = Vec::new();
    let mut to_theme_assignments = Vec::new();
    let mut default_assignments = Vec::new();

    for field in &fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        // Map common field names to theme properties
        let theme_property = match field_name_str.as_str() {
            "primary" => quote! { theme.primary },
            "secondary" => quote! { theme.secondary },
            "background" => quote! { theme.background },
            "surface" => quote! { theme.surface },
            "text" => quote! { theme.text },
            "text_on_primary" => quote! { theme.text_on_primary },
            "error" => quote! { theme.error },
            "warning" => quote! { theme.warning },
            "success" => quote! { theme.success },
            "info" => quote! { theme.info },
            _ => {
                // For unknown fields, use a default color
                quote! { tui_framework::style::Color::TRANSPARENT }
            }
        };

        from_theme_assignments.push(quote! {
            #field_name: #theme_property
        });

        // For To<Theme>, only map back known fields
        if matches!(
            field_name_str.as_str(),
            "primary"
                | "secondary"
                | "background"
                | "surface"
                | "text"
                | "text_on_primary"
                | "error"
                | "warning"
                | "success"
                | "info"
        ) {
            to_theme_assignments.push(quote! {
                #field_name: self.#field_name
            });
        }

        // Default assignments use dark theme values
        let default_value = match field_name_str.as_str() {
            "primary" => quote! { tui_framework::style::Color::rgb(100, 150, 255) },
            "secondary" => quote! { tui_framework::style::Color::rgb(150, 100, 255) },
            "background" => quote! { tui_framework::style::Color::rgb(20, 20, 25) },
            "surface" => quote! { tui_framework::style::Color::rgb(30, 30, 35) },
            "text" => quote! { tui_framework::style::Color::rgb(240, 240, 245) },
            "text_on_primary" => quote! { tui_framework::style::Color::WHITE },
            "error" => quote! { tui_framework::style::Color::rgb(255, 100, 100) },
            "warning" => quote! { tui_framework::style::Color::rgb(255, 200, 100) },
            "success" => quote! { tui_framework::style::Color::rgb(100, 255, 100) },
            "info" => quote! { tui_framework::style::Color::rgb(100, 200, 255) },
            _ => quote! { tui_framework::style::Color::TRANSPARENT },
        };

        default_assignments.push(quote! {
            #field_name: #default_value
        });
    }

    // Fill in missing theme fields with defaults for To<Theme>
    let base_theme_fields = [
        (
            "primary",
            quote! { tui_framework::style::Color::rgb(100, 150, 255) },
        ),
        (
            "secondary",
            quote! { tui_framework::style::Color::rgb(150, 100, 255) },
        ),
        (
            "background",
            quote! { tui_framework::style::Color::rgb(20, 20, 25) },
        ),
        (
            "surface",
            quote! { tui_framework::style::Color::rgb(30, 30, 35) },
        ),
        (
            "text",
            quote! { tui_framework::style::Color::rgb(240, 240, 245) },
        ),
        (
            "text_on_primary",
            quote! { tui_framework::style::Color::WHITE },
        ),
        (
            "error",
            quote! { tui_framework::style::Color::rgb(255, 100, 100) },
        ),
        (
            "warning",
            quote! { tui_framework::style::Color::rgb(255, 200, 100) },
        ),
        (
            "success",
            quote! { tui_framework::style::Color::rgb(100, 255, 100) },
        ),
        (
            "info",
            quote! { tui_framework::style::Color::rgb(100, 200, 255) },
        ),
    ];

    let mut complete_theme_assignments = Vec::new();
    for (field_name, default_value) in base_theme_fields {
        let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());

        // Check if this field exists in the custom theme
        let has_field = fields
            .iter()
            .any(|f| f.ident.as_ref().map(|i| i.to_string()) == Some(field_name.to_string()));

        if has_field {
            complete_theme_assignments.push(quote! {
                #field_ident: custom_theme.#field_ident
            });
        } else {
            complete_theme_assignments.push(quote! {
                #field_ident: #default_value
            });
        }
    }

    let expanded = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#default_assignments,)*
                }
            }
        }

        impl From<tui_framework::style::Theme> for #name {
            fn from(theme: tui_framework::style::Theme) -> Self {
                Self {
                    #(#from_theme_assignments,)*
                }
            }
        }

        impl From<#name> for tui_framework::style::Theme {
            fn from(custom_theme: #name) -> Self {
                Self {
                    #(#complete_theme_assignments,)*
                }
            }
        }

        impl #name {
            /// Create a new theme based on the dark theme
            pub fn dark() -> Self {
                tui_framework::style::Theme::dark().into()
            }

            /// Create a new theme based on the light theme
            pub fn light() -> Self {
                tui_framework::style::Theme::light().into()
            }

            /// Convert this theme to the base Theme type
            pub fn to_theme(&self) -> tui_framework::style::Theme {
                self.clone().into()
            }
        }
    };

    expanded.into()
}

/// Macro for JSX-like syntax for creating virtual DOM nodes.
///
/// This macro provides a more React-like syntax for creating UI elements.
/// It supports element creation, attributes, and nested children.
///
/// # Example
/// ```rust,ignore
/// use tui_framework_macros::jsx;
///
/// let node = jsx! {
///     <div style="bg-blue text-white">
///         <text>{"Hello World"}</text>
///         <button onclick={handle_click}>{"Click me"}</button>
///     </div>
/// };
/// ```
#[proc_macro]
pub fn jsx(input: TokenStream) -> TokenStream {
    // For now, provide a simple implementation that creates a basic div
    // This is a placeholder for future JSX-like syntax implementation
    let input_str = input.to_string();

    if input_str.trim().is_empty() {
        // Empty JSX returns empty div
        quote! {
            tui_framework::render::vdom::VirtualNode::element("div")
        }
        .into()
    } else {
        // For now, just create a div with the input as text content
        // TODO: Implement proper JSX parsing
        quote! {
            tui_framework::render::vdom::VirtualNode::element("div")
                .child(tui_framework::render::vdom::VirtualNode::text("JSX placeholder"))
        }
        .into()
    }
}

/// Macro for creating hook-based state management patterns.
///
/// This macro provides convenient patterns for common hook usage.
///
/// # Example
/// ```rust,ignore
/// use tui_framework_macros::use_hooks;
///
/// use_hooks! {
///     let (count, set_count) = use_state(0);
///     let (name, set_name) = use_state("".to_string());
///
///     use_effect(|| {
///         println!("Component mounted");
///     }, []);
/// }
/// ```
#[proc_macro]
pub fn use_hooks(input: TokenStream) -> TokenStream {
    // For now, just pass through the input as-is
    // This allows the macro to be used as a marker for hook blocks
    // TODO: Add validation and optimization for hook usage patterns
    input
}
