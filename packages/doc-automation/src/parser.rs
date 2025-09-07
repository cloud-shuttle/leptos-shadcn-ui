//! Component source code parsing for documentation extraction

use crate::{ComponentMetadata, PropMetadata, EventMetadata, ExampleMetadata, TestMetadata, AccessibilityInfo, PerformanceInfo, DocError};
use std::path::Path;
use syn::{File, Item, ItemStruct, ItemFn, Attribute, Expr, Lit};

/// Parse a Rust component file to extract documentation metadata
pub async fn parse_component_file(file_path: &Path) -> Result<Option<ComponentMetadata>, DocError> {
    let content = tokio::fs::read_to_string(file_path)
        .await
        .map_err(DocError::FileSystem)?;

    let syntax_tree = syn::parse_file(&content)
        .map_err(|e| DocError::Parse(format!("Failed to parse {}: {}", file_path.display(), e)))?;

    // Look for component function or struct
    let mut component_name = None;
    let mut component_description = None;
    let mut props = Vec::new();
    let mut events = Vec::new();
    let mut examples = Vec::new();
    let mut tests = Vec::new();

    for item in syntax_tree.items {
        match item {
            Item::Fn(ref func) => {
                // Check if this is a component function
                if is_component_function(func) {
                    component_name = Some(func.sig.ident.to_string());
                    component_description = extract_doc_comment(&func.attrs);
                    examples.extend(extract_examples_from_docs(&func.attrs));
                }
                
                // Check if this is a test function
                if is_test_function(func) {
                    tests.push(extract_test_metadata(func));
                }
            }
            Item::Struct(ref struct_item) => {
                // Check if this is a props struct
                if struct_item.ident.to_string().ends_with("Props") {
                    props.extend(extract_props_from_struct(struct_item));
                }
            }
            _ => {}
        }
    }

    if let Some(name) = component_name {
        Ok(Some(ComponentMetadata {
            name,
            description: component_description,
            props,
            events,
            examples,
            file_path: file_path.to_path_buf(),
            tests,
            accessibility: extract_accessibility_info(&content),
            performance: extract_performance_info(&content),
        }))
    } else {
        Ok(None)
    }
}

/// Check if a function is a Leptos component
fn is_component_function(func: &ItemFn) -> bool {
    // Look for #[component] attribute
    func.attrs.iter().any(|attr| {
        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::Path(path) = meta {
                return path.is_ident("component");
            }
        }
        false
    })
}

/// Check if a function is a test
fn is_test_function(func: &ItemFn) -> bool {
    func.attrs.iter().any(|attr| {
        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::Path(path) = meta {
                return path.is_ident("test");
            }
        }
        false
    })
}

/// Extract documentation comment from attributes
fn extract_doc_comment(attrs: &[Attribute]) -> Option<String> {
    let mut doc_lines = Vec::new();
    
    for attr in attrs {
        if attr.path.is_ident("doc") {
            if let Ok(syn::Meta::NameValue(meta)) = attr.parse_meta() {
                if let syn::Lit::Str(lit_str) = meta.lit {
                    let line = lit_str.value();
                    // Remove leading space if present
                    let trimmed = if line.starts_with(' ') {
                        &line[1..]
                    } else {
                        &line
                    };
                    doc_lines.push(trimmed.to_string());
                }
            }
        }
    }
    
    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join("\n"))
    }
}

/// Extract code examples from doc comments
fn extract_examples_from_docs(attrs: &[Attribute]) -> Vec<ExampleMetadata> {
    let mut examples = Vec::new();
    let doc_comment = extract_doc_comment(attrs).unwrap_or_default();
    
    // Look for code blocks in documentation
    let mut in_code_block = false;
    let mut current_code = Vec::new();
    let mut current_title = "Example".to_string();
    
    for line in doc_comment.lines() {
        if line.trim().starts_with("```rust") {
            in_code_block = true;
            current_code.clear();
        } else if line.trim().starts_with("```") && in_code_block {
            in_code_block = false;
            if !current_code.is_empty() {
                examples.push(ExampleMetadata {
                    title: current_title.clone(),
                    description: None,
                    code: current_code.join("\n"),
                    category: "usage".to_string(),
                });
            }
        } else if in_code_block {
            current_code.push(line.to_string());
        } else if line.trim().starts_with("# ") {
            current_title = line.trim().trim_start_matches("# ").to_string();
        }
    }
    
    examples
}

/// Extract props from a struct definition
fn extract_props_from_struct(struct_item: &ItemStruct) -> Vec<PropMetadata> {
    let mut props = Vec::new();
    
    if let syn::Fields::Named(fields) = &struct_item.fields {
        for field in &fields.named {
            if let Some(ident) = &field.ident {
                let prop_name = ident.to_string();
                let prop_type = quote::quote!(#(&field.ty)).to_string();
                let description = extract_doc_comment(&field.attrs);
                let required = !is_option_type(&field.ty);
                
                props.push(PropMetadata {
                    name: prop_name,
                    prop_type,
                    description,
                    default_value: None, // TODO: Extract from Default impl
                    required,
                    examples: Vec::new(), // TODO: Extract from docs
                });
            }
        }
    }
    
    props
}

/// Check if a type is Option<T>
fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Extract test metadata from a test function
fn extract_test_metadata(func: &ItemFn) -> TestMetadata {
    let name = func.sig.ident.to_string();
    let description = extract_doc_comment(&func.attrs);
    
    // Determine test type based on name patterns
    let test_type = if name.contains("integration") {
        "integration"
    } else if name.contains("e2e") {
        "e2e"
    } else if name.contains("performance") || name.contains("bench") {
        "performance"
    } else {
        "unit"
    }.to_string();
    
    TestMetadata {
        name,
        test_type,
        description,
        coverage: None, // TODO: Extract from coverage data
    }
}

/// Extract accessibility information from source code
fn extract_accessibility_info(content: &str) -> AccessibilityInfo {
    let keyboard_support = content.contains("onkeydown") || 
                          content.contains("onkeyup") || 
                          content.contains("onkeypress") ||
                          content.contains("tabindex");
    
    let screen_reader_support = content.contains("aria-") || 
                               content.contains("role=");
    
    let mut aria_attributes = Vec::new();
    
    // Extract ARIA attributes (simple pattern matching)
    let aria_patterns = [
        "aria-label", "aria-labelledby", "aria-describedby", 
        "aria-expanded", "aria-selected", "aria-disabled",
        "aria-hidden", "aria-live", "aria-atomic"
    ];
    
    for pattern in &aria_patterns {
        if content.contains(pattern) {
            aria_attributes.push(pattern.to_string());
        }
    }
    
    AccessibilityInfo {
        wcag_level: if screen_reader_support && keyboard_support { 
            "AA".to_string() 
        } else { 
            "A".to_string() 
        },
        keyboard_support,
        screen_reader_support,
        aria_attributes,
    }
}

/// Extract performance information from source code and tests
fn extract_performance_info(content: &str) -> PerformanceInfo {
    // Look for performance-related comments or benchmarks
    let has_benchmarks = content.contains("criterion") || 
                        content.contains("benchmark") ||
                        content.contains("#[bench]");
    
    PerformanceInfo {
        render_time_ms: if has_benchmarks { Some(15.0) } else { None }, // Placeholder
        bundle_size_kb: None, // TODO: Extract from build analysis
        memory_usage_mb: None, // TODO: Extract from profiling
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_parse_simple_component() {
        let component_code = r#"
/// A simple button component
/// 
/// # Example
/// 
/// ```rust
/// use leptos::*;
/// 
/// view! {
///     <Button variant="primary">"Click me"</Button>
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> impl IntoView {
    view! {
        <button class="btn">{props.children}</button>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonProps {
    /// The button variant
    pub variant: Option<String>,
    /// The button content
    pub children: leptos::View,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_button_renders() {
        // Test implementation
    }
}
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", component_code).unwrap();

        let result = parse_component_file(temp_file.path()).await.unwrap();
        assert!(result.is_some());

        let component = result.unwrap();
        assert_eq!(component.name, "Button");
        assert!(component.description.is_some());
        assert!(component.description.unwrap().contains("simple button component"));
        assert_eq!(component.props.len(), 2);
        assert_eq!(component.tests.len(), 1);
        assert_eq!(component.examples.len(), 1);
    }

    #[tokio::test]
    async fn test_parse_non_component_file() {
        let non_component_code = r#"
pub struct SomeStruct {
    pub field: String,
}

pub fn some_function() {
    println!("Hello");
}
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", non_component_code).unwrap();

        let result = parse_component_file(temp_file.path()).await.unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_doc_comment() {
        use syn::parse_quote;
        
        let attrs: Vec<Attribute> = vec![
            parse_quote!(#[doc = " First line"]),
            parse_quote!(#[doc = " Second line"]),
            parse_quote!(#[doc = ""]),
            parse_quote!(#[doc = " Third line"]),
        ];

        let result = extract_doc_comment(&attrs);
        assert_eq!(result, Some("First line\nSecond line\n\nThird line".to_string()));
    }

    #[test]
    fn test_extract_props_from_struct() {
        use syn::parse_quote;
        
        let struct_item: ItemStruct = parse_quote! {
            pub struct TestProps {
                /// Required property
                pub required_prop: String,
                /// Optional property  
                pub optional_prop: Option<i32>,
            }
        };

        let props = extract_props_from_struct(&struct_item);
        assert_eq!(props.len(), 2);
        
        assert_eq!(props[0].name, "required_prop");
        assert!(props[0].required);
        assert!(props[0].description.is_some());
        
        assert_eq!(props[1].name, "optional_prop");
        assert!(!props[1].required);
    }

    #[test]
    fn test_is_option_type() {
        use syn::parse_quote;
        
        let option_type: syn::Type = parse_quote!(Option<String>);
        let regular_type: syn::Type = parse_quote!(String);
        
        assert!(is_option_type(&option_type));
        assert!(!is_option_type(&regular_type));
    }

    #[test]
    fn test_extract_accessibility_info() {
        let content_with_aria = r#"
            view! {
                <button 
                    aria-label="Close dialog"
                    onclick=handle_click
                    onkeydown=handle_keydown
                >
                    "X"
                </button>
            }
        "#;

        let info = extract_accessibility_info(content_with_aria);
        assert_eq!(info.wcag_level, "AA");
        assert!(info.keyboard_support);
        assert!(info.screen_reader_support);
        assert!(info.aria_attributes.contains(&"aria-label".to_string()));
    }

    #[test]
    fn test_extract_performance_info() {
        let content_with_benchmarks = r#"
            use criterion::Criterion;
            
            fn benchmark_component_render(c: &mut Criterion) {
                c.bench_function("render", |b| {
                    b.iter(|| render_component())
                });
            }
        "#;

        let info = extract_performance_info(content_with_benchmarks);
        assert!(info.render_time_ms.is_some());
    }
}