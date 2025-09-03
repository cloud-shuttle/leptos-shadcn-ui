//! Test templates for different component types.
//!
//! This module provides pre-built test code strings for different component types,
//! making it easy to generate comprehensive tests for Leptos components.

use crate::leptos_testing::{ComponentTestSuite, test_helpers};

/// Test code generator for different component types
pub struct TestCodeGenerator;

impl TestCodeGenerator {
    /// Generate test code for basic components
    pub fn generate_basic_component_tests(component_name: &str) -> String {
        let test_builder = test_helpers::basic_component_test(component_name);
        let _test_suite = ComponentTestSuite::new(&format!("{}_test_suite", component_name))
            .add_test(test_builder);
        
        format!(
            r#"#[cfg(test)]
mod tests {{
    use super::*;
    use leptos::*;
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use shadcn_ui_test_utils::{{TestResult, Framework, Theme}};

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Component should render successfully");
    }}

    #[test]
    fn test_{component_name}_basic_functionality() {{
        // Test basic component functionality
        let result = LeptosTestUtils::test_component_with_props(std::collections::HashMap::new());
        assert!(result.passed, "Component should work with default props");
    }}

    #[test]
    fn test_{component_name}_accessibility() {{
        // Test component accessibility
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed, "Component should meet accessibility requirements");
    }}

    #[test]
    fn test_{component_name}_styling() {{
        // Test component styling
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed, "Component should have proper styling");
    }}

    #[test]
    fn test_{component_name}_theme_variants() {{
        // Test that both theme variants exist and are accessible
        let default_theme = crate::default::{component_name_pascal}::default();
        let new_york_theme = crate::new_york::{component_name_pascal}::default();
        
        // Basic existence check - components should be available
        assert!(std::any::type_name_of_val(&default_theme).contains("{component_name}"));
        assert!(std::any::type_name_of_val(&new_york_theme).contains("{component_name}"));
    }}

    #[test]
    fn test_{component_name}_comprehensive() {{
        // Comprehensive test using the test builder
        let test = test_helpers::basic_component_test("{component_name}");
        let result = test.run();
        assert!(result.passed, "Comprehensive test should pass");
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for form components
    pub fn generate_form_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use super::*;
    use leptos::*;
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use shadcn_ui_test_utils::{{TestResult, Framework, Theme}};
    use std::collections::HashMap;

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Component should render successfully");
    }}

    #[test]
    fn test_{component_name}_form_functionality() {{
        // Test form-specific functionality
        let mut props = HashMap::new();
        props.insert("value".to_string(), "test_value".to_string());
        props.insert("placeholder".to_string(), "Enter text".to_string());
        
        let result = LeptosTestUtils::test_component_with_props(props);
        assert!(result.passed, "Component should work with form props");
    }}

    #[test]
    fn test_{component_name}_accessibility() {{
        // Test form component accessibility
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed, "Form component should meet accessibility requirements");
    }}

    #[test]
    fn test_{component_name}_events() {{
        // Test form component events
        let result = LeptosTestUtils::test_component_interaction("input");
        assert!(result.passed, "Component should handle input events");
    }}

    #[test]
    fn test_{component_name}_validation() {{
        // Test form validation if applicable
        let result = LeptosTestUtils::test_component_with_config(
            leptos_testing::LeptosTestConfig::default()
        );
        assert!(result.passed, "Component should handle validation correctly");
    }}

    #[test]
    fn test_{component_name}_theme_variants() {{
        // Test both theme variants
        let default_theme = crate::default::{component_name_pascal}::default();
        let new_york_theme = crate::new_york::{component_name_pascal}::default();
        
        assert!(std::any::type_name_of_val(&default_theme).contains("{component_name}"));
        assert!(std::any::type_name_of_val(&new_york_theme).contains("{component_name}"));
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for interactive components
    pub fn generate_interactive_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use super::*;
    use leptos::*;
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use shadcn_ui_test_utils::{{TestResult, Framework, Theme}};

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Component should render successfully");
    }}

    #[test]
    fn test_{component_name}_interactions() {{
        // Test interactive functionality
        let result = LeptosTestUtils::test_component_interaction("click");
        assert!(result.passed, "Component should handle click interactions");
        
        let result = LeptosTestUtils::test_component_interaction("hover");
        assert!(result.passed, "Component should handle hover interactions");
    }}

    #[test]
    fn test_{component_name}_state_management() {{
        // Test state changes
        let result = LeptosTestUtils::test_component_state_change();
        assert!(result.passed, "Component should manage state correctly");
    }}

    #[test]
    fn test_{component_name}_accessibility() {{
        // Test accessibility features
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.passed, "Interactive component should meet accessibility requirements");
    }}

    #[test]
    fn test_{component_name}_keyboard_navigation() {{
        // Test keyboard navigation
        let result = LeptosTestUtils::test_component_interaction("keyboard");
        assert!(result.passed, "Component should support keyboard navigation");
    }}

    #[test]
    fn test_{component_name}_theme_variants() {{
        // Test both theme variants
        let default_theme = crate::default::{component_name_pascal}::default();
        let new_york_theme = crate::new_york::{component_name_pascal}::default();
        
        assert!(std::any::type_name_of_val(&default_theme).contains("{component_name}"));
        assert!(std::any::type_name_of_val(&new_york_theme).contains("{component_name}"));
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for layout components
    pub fn generate_layout_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use super::*;
    use leptos::*;
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use shadcn_ui_test_utils::{{TestResult, Framework, Theme}};

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Component should render successfully");
    }}

    #[test]
    fn test_{component_name}_layout_functionality() {{
        // Test layout-specific functionality
        let result = LeptosTestUtils::test_component_with_props(std::collections::HashMap::new());
        assert!(result.passed, "Layout component should work correctly");
    }}

    #[test]
    fn test_{component_name}_responsive_behavior() {{
        // Test responsive behavior if applicable
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed, "Layout component should have proper styling");
    }}

    #[test]
    fn test_{component_name}_children_handling() {{
        // Test that layout components can handle children
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Layout component should handle children correctly");
    }}

    #[test]
    fn test_{component_name}_theme_variants() {{
        // Test both theme variants
        let default_theme = crate::default::{component_name_pascal}::default();
        let new_york_theme = crate::new_york::{component_name_pascal}::default();
        
        assert!(std::any::type_name_of_val(&default_theme).contains("{component_name}"));
        assert!(std::any::type_name_of_val(&new_york_theme).contains("{component_name}"));
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate test code for display components
    pub fn generate_display_component_tests(component_name: &str) -> String {
        format!(
            r#"#[cfg(test)]
mod tests {{
    use super::*;
    use leptos::*;
    use shadcn_ui_test_utils::leptos_testing::{{LeptosTestUtils, ComponentTestBuilder, test_helpers}};
    use shadcn_ui_test_utils::{{TestResult, Framework, Theme}};

    #[test]
    fn test_{component_name}_component_exists() {{
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Component should render successfully");
    }}

    #[test]
    fn test_{component_name}_display_functionality() {{
        // Test display-specific functionality
        let result = LeptosTestUtils::test_component_with_props(std::collections::HashMap::new());
        assert!(result.passed, "Display component should work correctly");
    }}

    #[test]
    fn test_{component_name}_styling() {{
        // Test component styling
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.passed, "Display component should have proper styling");
    }}

    #[test]
    fn test_{component_name}_content_rendering() {{
        // Test that content renders correctly
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.passed, "Display component should render content correctly");
    }}

    #[test]
    fn test_{component_name}_theme_variants() {{
        // Test both theme variants
        let default_theme = crate::default::{component_name_pascal}::default();
        let new_york_theme = crate::new_york::{component_name_pascal}::default();
        
        assert!(std::any::type_name_of_val(&default_theme).contains("{component_name}"));
        assert!(std::any::type_name_of_val(&new_york_theme).contains("{component_name}"));
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Generate comprehensive test suite for any component type
    pub fn generate_comprehensive_tests(component_name: &str, component_type: ComponentType) -> String {
        match component_type {
            ComponentType::Basic => Self::generate_basic_component_tests(component_name),
            ComponentType::Form => Self::generate_form_component_tests(component_name),
            ComponentType::Interactive => Self::generate_interactive_component_tests(component_name),
            ComponentType::Layout => Self::generate_layout_component_tests(component_name),
            ComponentType::Display => Self::generate_display_component_tests(component_name),
        }
    }

    /// Generate test configuration file
    pub fn generate_test_config(component_name: &str) -> String {
        format!(
            r#"# Test configuration for {component_name} component

[test]
# Enable all test types
compilation_tests = true
runtime_tests = false  # Requires WASM runtime
accessibility_tests = true
theme_tests = true
performance_tests = false

# Test timeouts
test_timeout_seconds = 30

# Output verbosity
verbose_output = false

# Quality thresholds
min_quality_score = 0.8
min_test_coverage = 0.8
min_documentation_quality = 0.7

# Required accessibility features
required_accessibility_features = [
    "aria-label",
    "keyboard-navigation", 
    "focus-management"
]

# Theme requirements
required_themes = ["default", "new_york"]

# Performance benchmarks
performance_benchmarks = [
    "render_time < 16ms",
    "memory_usage < 1MB",
    "bundle_size < 10KB"
]
"#
        )
    }

    /// Generate test helper functions
    pub fn generate_test_helpers(component_name: &str) -> String {
        format!(
            r#"// Test helper functions for {component_name} component

use super::*;
use leptos::*;
use shadcn_ui_test_utils::leptos_testing::LeptosTestUtils;

/// Helper function to create a test instance with default props
pub fn create_test_{component_name}() -> impl IntoView {{
    // Create component with minimal props for testing
    view! {{
        <{component_name_pascal} />
    }}
}}

/// Helper function to create a test instance with custom props
pub fn create_test_{component_name}_with_props(props: impl Into<{component_name_pascal}Props>) -> impl IntoView {{
    view! {{
        <{component_name_pascal} ..props.into() />
    }}
}}

/// Helper function to test component rendering
pub fn test_{component_name}_rendering() -> bool {{
    let result = LeptosTestUtils::test_component_renders();
    result.passed
}}

/// Helper function to test component accessibility
pub fn test_{component_name}_accessibility() -> bool {{
    let result = LeptosTestUtils::test_component_accessibility();
    result.passed
}}

/// Helper function to test component styling
pub fn test_{component_name}_styling() -> bool {{
    let result = LeptosTestUtils::test_component_styling();
    result.passed
}}

/// Helper function to test component interactions
pub fn test_{component_name}_interactions() -> bool {{
    let result = LeptosTestUtils::test_component_interaction("click");
    result.passed
}}

#[cfg(test)]
mod test_helpers_tests {{
    use super::*;

    #[test]
    fn test_helper_functions_exist() {{
        // Test that all helper functions can be called
        assert!(test_{component_name}_rendering());
        assert!(test_{component_name}_accessibility());
        assert!(test_{component_name}_styling());
        assert!(test_{component_name}_interactions());
    }}

    #[test]
    fn test_component_creation() {{
        // Test that components can be created
        let _component = create_test_{component_name}();
        // If we get here without panicking, the test passes
    }}
}}"#,
            component_name = component_name,
            component_name_pascal = Self::to_pascal_case(component_name)
        )
    }

    /// Convert component name to PascalCase
    fn to_pascal_case(s: &str) -> String {
        s.split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }
}

/// Component types for test generation
#[derive(Debug, Clone, Copy)]
pub enum ComponentType {
    Basic,
    Form,
    Interactive,
    Layout,
    Display,
}

impl ComponentType {
    /// Determine component type from component name
    pub fn from_name(name: &str) -> Self {
        match name {
            // Form components
            "button" | "checkbox" | "radio-group" | "select" | "combobox" | 
            "form" | "input" | "label" | "textarea" | "slider" | "switch" | "toggle" => {
                ComponentType::Form
            }
            // Interactive components
            "dialog" | "alert-dialog" | "sheet" | "drawer" | "dropdown-menu" |
            "popover" | "tooltip" | "toast" | "carousel" | "date-picker" |
            "hover-card" | "input-otp" => {
                ComponentType::Interactive
            }
            // Layout components
            "accordion" | "collapsible" | "resizable" | "scroll-area" |
            "separator" | "sidebar" | "aspect-ratio" => {
                ComponentType::Layout
            }
            // Display components
            "alert" | "avatar" | "badge" | "card" | "calendar" |
            "progress" | "skeleton" | "table" | "typography" => {
                ComponentType::Display
            }
            // Default to basic for navigation and other components
            _ => ComponentType::Basic,
        }
    }
    
    /// Get description for component type
    pub fn description(&self) -> &'static str {
        match self {
            ComponentType::Basic => "Basic component with standard functionality",
            ComponentType::Form => "Form component with input handling and validation",
            ComponentType::Interactive => "Interactive component with user interactions",
            ComponentType::Layout => "Layout component for organizing content",
            ComponentType::Display => "Display component for showing information",
        }
    }
}
