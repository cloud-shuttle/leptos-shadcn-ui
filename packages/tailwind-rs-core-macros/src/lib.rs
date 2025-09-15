//! Procedural macros for tailwind-rs-core.

use quote::quote;
use syn::{parse_macro_input, Expr, Token};

/// Macro for creating type-safe Tailwind classes with compile-time validation.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::classes;
/// 
/// // Basic usage
/// let classes = classes! {
///     base: "px-4 py-2 rounded-md font-medium",
///     variant: "bg-blue-600 text-white hover:bg-blue-700",
///     responsive: "sm:text-sm md:text-base lg:text-lg",
/// };
/// 
/// // Dynamic styling
/// let color = Color::Blue;
/// let classes = classes! {
///     background: color.background(600),
///     text: color.text(),
///     hover: color.hover(700),
/// };
/// ```
#[proc_macro]
pub fn classes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ClassesInput);
    
    let mut base_classes = String::new();
    let mut variant_classes = Vec::new();
    let mut responsive_classes = Vec::new();
    let mut state_classes = Vec::new();
    let mut custom_classes = Vec::new();
    
    for field in input.fields {
        match field.name.as_str() {
            "base" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        base_classes = s.value();
                    }
                }
            }
            "variant" | "variants" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        variant_classes.push(s.value());
                    }
                }
            }
            "responsive" | "responsive_classes" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        responsive_classes.push(s.value());
                    }
                }
            }
            "state" | "states" | "hover" | "focus" | "active" | "disabled" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        state_classes.push(s.value());
                    }
                }
            }
            "custom" | "additional" | "extra" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        custom_classes.push(s.value());
                    }
                }
            }
            _ => {
                // Treat as custom class
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        custom_classes.push(s.value());
                    }
                }
            }
        }
    }
    
    // Generate the final class string
    let mut all_classes = vec![base_classes];
    all_classes.extend(variant_classes);
    all_classes.extend(responsive_classes);
    all_classes.extend(state_classes);
    all_classes.extend(custom_classes);
    
    let final_classes = all_classes.join(" ");
    
    quote! {
        #final_classes
    }.into()
}

/// Macro for creating responsive Tailwind classes.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::responsive;
/// 
/// let responsive_classes = responsive! {
///     sm: "text-sm",
///     md: "text-base",
///     lg: "text-lg",
///     xl: "text-xl",
/// };
/// ```
#[proc_macro]
pub fn responsive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ResponsiveInput);
    
    let mut classes = Vec::new();
    
    for field in input.fields {
        let breakpoint = field.name.as_str();
        if let Expr::Lit(lit) = &field.value {
            if let syn::Lit::Str(s) = &lit.lit {
                let class = format!("{}:{}", breakpoint, s.value());
                classes.push(class);
            }
        }
    }
    
    let final_classes = classes.join(" ");
    
    quote! {
        #final_classes
    }.into()
}

/// Macro for creating theme-based Tailwind classes.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::theme;
/// 
/// let theme_classes = theme! {
///     variant: "primary",
///     size: "md",
///     additional: "rounded-md shadow-lg",
/// };
/// ```
#[proc_macro]
pub fn theme(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ThemeInput);
    
    let mut variant_classes = String::new();
    let mut size_classes = String::new();
    let mut additional_classes = String::new();
    
    for field in input.fields {
        match field.name.as_str() {
            "variant" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        variant_classes = match s.value().as_str() {
                            "primary" => "bg-blue-600 text-white hover:bg-blue-700".to_string(),
                            "secondary" => "bg-gray-200 text-gray-900 hover:bg-gray-300".to_string(),
                            "success" => "bg-green-600 text-white hover:bg-green-700".to_string(),
                            "warning" => "bg-yellow-600 text-white hover:bg-yellow-700".to_string(),
                            "error" => "bg-red-600 text-white hover:bg-red-700".to_string(),
                            "info" => "bg-sky-600 text-white hover:bg-sky-700".to_string(),
                            "outline" => "border-2 border-blue-600 text-blue-600 hover:bg-blue-50".to_string(),
                            "ghost" => "bg-transparent hover:bg-blue-100".to_string(),
                            "link" => "text-blue-600 underline hover:text-blue-700".to_string(),
                            "destructive" => "bg-red-600 text-white hover:bg-red-700".to_string(),
                            _ => String::new(),
                        };
                    }
                }
            }
            "size" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        size_classes = match s.value().as_str() {
                            "xs" => "px-2 py-1 text-xs".to_string(),
                            "sm" => "px-3 py-1.5 text-sm".to_string(),
                            "md" => "px-4 py-2 text-base".to_string(),
                            "lg" => "px-6 py-3 text-lg".to_string(),
                            "xl" => "px-8 py-4 text-xl".to_string(),
                            _ => String::new(),
                        };
                    }
                }
            }
            "additional" | "extra" | "custom" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        additional_classes = s.value();
                    }
                }
            }
            _ => {}
        }
    }
    
    let mut all_classes = vec![variant_classes, size_classes, additional_classes];
    all_classes.retain(|s| !s.is_empty());
    
    let final_classes = all_classes.join(" ");
    
    quote! {
        #final_classes
    }.into()
}

/// Macro for creating color-based Tailwind classes.
/// 
/// # Examples
/// 
/// ```rust
/// use tailwind_rs_core_macros::color;
/// 
/// let color_classes = color! {
///     color: "blue",
///     shade: 600,
///     variants: ["background", "text", "hover"],
/// };
/// ```
#[proc_macro]
pub fn color(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ColorInput);
    
    let mut color_name = String::new();
    let mut shade = 600u16;
    let mut variants = Vec::new();
    
    for field in input.fields {
        match field.name.as_str() {
            "color" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        color_name = s.value();
                    }
                }
            }
            "shade" => {
                if let Expr::Lit(lit) = &field.value {
                    if let syn::Lit::Int(i) = &lit.lit {
                        shade = i.base10_parse::<u16>().unwrap_or(600);
                    }
                }
            }
            "variants" => {
                if let Expr::Array(arr) = &field.value {
                    for elem in &arr.elems {
                        if let Expr::Lit(lit) = elem {
                            if let syn::Lit::Str(s) = &lit.lit {
                                variants.push(s.value());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    let mut classes = Vec::new();
    
    for variant in variants {
        let class = match variant.as_str() {
            "background" | "bg" => format!("bg-{}-{}", color_name, shade),
            "text" => format!("text-{}-{}", color_name, shade),
            "border" => format!("border-{}-{}", color_name, shade),
            "hover" => format!("hover:bg-{}-{}", color_name, shade),
            "focus" => format!("focus:ring-{}-{}", color_name, shade),
            "shadow" => format!("shadow-{}-{}", color_name, shade),
            _ => continue,
        };
        classes.push(class);
    }
    
    let final_classes = classes.join(" ");
    
    quote! {
        #final_classes
    }.into()
}

/// Input structure for the classes macro.
struct ClassesInput {
    fields: Vec<ClassField>,
}

/// Input structure for the responsive macro.
struct ResponsiveInput {
    fields: Vec<ClassField>,
}

/// Input structure for the theme macro.
struct ThemeInput {
    fields: Vec<ClassField>,
}

/// Input structure for the color macro.
struct ColorInput {
    fields: Vec<ClassField>,
}

/// A field in a macro input.
struct ClassField {
    name: String,
    value: Expr,
}

impl syn::parse::Parse for ClassesInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ClassesInput { fields })
    }
}

impl syn::parse::Parse for ResponsiveInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ResponsiveInput { fields })
    }
}

impl syn::parse::Parse for ThemeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ThemeInput { fields })
    }
}

impl syn::parse::Parse for ColorInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(ClassField {
                name: name.to_string(),
                value,
            });
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ColorInput { fields })
    }
}
