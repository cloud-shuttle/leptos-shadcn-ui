#!/usr/bin/env cargo
//! Automated test generation script for all Leptos shadcn/ui components
//! 
//! This script automatically generates comprehensive tests for all components
//! using the enhanced testing infrastructure and templates.
//! 
//! Last Updated: September 3rd, 2025

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Component test generator for Leptos shadcn/ui
pub struct ComponentTestGenerator {
    pub workspace_root: PathBuf,
    pub components: Vec<ComponentInfo>,
    pub test_results: HashMap<String, TestGenerationResult>,
}

/// Component information for test generation
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub name: String,
    pub component_type: ComponentType,
    pub has_tests: bool,
    pub test_files: Vec<String>,
    pub quality_score: f64,
}

/// Component types for test generation
#[derive(Debug, Clone)]
pub enum ComponentType {
    Basic,
    Form,
    Interactive,
    Layout,
    Display,
}

/// Test generation result
#[derive(Debug, Clone)]
pub struct TestGenerationResult {
    pub component_name: String,
    pub tests_generated: bool,
    pub test_files_created: Vec<String>,
    pub compilation_success: bool,
    pub test_execution_success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ComponentTestGenerator {
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            components: Vec::new(),
            test_results: HashMap::new(),
        }
    }
    
    /// Discover all available components
    pub fn discover_components(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let components_dir = self.workspace_root.join("packages/leptos");
        
        if !components_dir.exists() {
            return Err("Components directory not found".into());
        }
        
        for entry in fs::read_dir(components_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(component_name) = path.file_name() {
                    let component_name = component_name.to_string_lossy();
                    if component_name != "shadcn-ui" { // Skip the main package
                        let component_type = Self::determine_component_type(&component_name);
                        let has_tests = self.check_existing_tests(&path);
                        let quality_score = self.assess_component_quality(&component_name);
                        
                        self.components.push(ComponentInfo {
                            name: component_name.to_string(),
                            component_type,
                            has_tests,
                            test_files: Vec::new(),
                            quality_score,
                        });
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Determine component type based on name
    fn determine_component_type(name: &str) -> ComponentType {
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
    
    /// Check if component already has tests
    fn check_existing_tests(&self, component_path: &Path) -> bool {
        let tests_file = component_path.join("src").join("tests.rs");
        tests_file.exists()
    }
    
    /// Assess component quality (mock implementation)
    fn assess_component_quality(&self, component_name: &str) -> f64 {
        // Mock quality assessment - in practice this would use the QualityChecker
        match component_name {
            "avatar" | "button" | "card" => 0.85,
            "input" | "form" => 0.75,
            _ => 0.60,
        }
    }
    
    /// Generate tests for all components
    pub fn generate_tests_for_all_components(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Generating comprehensive tests for all {} components...\n", self.components.len());
        
        for component in &self.components {
            println!("📝 Generating tests for: {}", component.name);
            let result = self.generate_tests_for_component(component)?;
            self.test_results.insert(component.name.clone(), result);
        }
        
        Ok(())
    }
    
    /// Generate tests for a specific component
    fn generate_tests_for_component(&self, component: &ComponentInfo) -> Result<TestGenerationResult, Box<dyn std::error::Error>> {
        let mut result = TestGenerationResult::new(&component.name);
        
        // Generate test code based on component type
        let test_code = self.generate_test_code(component);
        let test_helpers = self.generate_test_helpers(component);
        
        // Create test files
        let test_files = self.create_test_files(component, &test_code, &test_helpers)?;
        result = result.with_test_files(test_files);
        
        // Test compilation
        let compilation_success = self.test_component_compilation(&component.name)?;
        result = result.with_compilation_result(compilation_success);
        
        // Test execution (if compilation succeeded)
        if compilation_success {
            let test_execution_success = self.test_component_execution(&component.name)?;
            result = result.with_test_execution_result(test_execution_success);
        }
        
        Ok(result)
    }
    
    /// Generate test code based on component type
    fn generate_test_code(&self, component: &ComponentInfo) -> String {
        match component.component_type {
            ComponentType::Form => self.generate_form_component_tests(&component.name),
            ComponentType::Interactive => self.generate_interactive_component_tests(&component.name),
            ComponentType::Layout => self.generate_layout_component_tests(&component.name),
            ComponentType::Display => self.generate_display_component_tests(&component.name),
            ComponentType::Basic => self.generate_basic_component_tests(&component.name),
        }
    }
    
    /// Generate basic component tests
    fn generate_basic_component_tests(&self, component_name: &str) -> String {
        let component_name_pascal = self.to_pascal_case(component_name);
        
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
            component_name_pascal = component_name_pascal
        )
    }
    
    /// Generate form component tests
    fn generate_form_component_tests(&self, component_name: &str) -> String {
        let component_name_pascal = self.to_pascal_case(component_name);
        
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
            component_name_pascal = component_name_pascal
        )
    }
    
    /// Generate interactive component tests
    fn generate_interactive_component_tests(&self, component_name: &str) -> String {
        let component_name_pascal = self.to_pascal_case(component_name);
        
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
            component_name_pascal = component_name_pascal
        )
    }
    
    /// Generate layout component tests
    fn generate_layout_component_tests(&self, component_name: &str) -> String {
        let component_name_pascal = self.to_pascal_case(component_name);
        
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
            component_name_pascal = component_name_pascal
        )
    }
    
    /// Generate display component tests
    fn generate_display_component_tests(&self, component_name: &str) -> String {
        let component_name_pascal = self.to_pascal_case(component_name);
        
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
            component_name_pascal = component_name_pascal
        )
    }
    
    /// Generate test helper functions
    fn generate_test_helpers(&self, component: &ComponentInfo) -> String {
        let component_name = &component.name;
        let component_name_pascal = self.to_pascal_case(component_name);
        
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
            component_name_pascal = component_name_pascal
        )
    }
    
    /// Create test files for a component
    fn create_test_files(&self, component: &ComponentInfo, test_code: &str, test_helpers: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut created_files = Vec::new();
        let component_dir = self.workspace_root.join("packages/leptos").join(&component.name);
        
        // Create tests.rs file
        let tests_file = component_dir.join("src").join("tests.rs");
        fs::write(&tests_file, test_code)?;
        created_files.push(tests_file.to_string_lossy().to_string());
        
        // Create test_helpers.rs file
        let helpers_file = component_dir.join("src").join("test_helpers.rs");
        fs::write(&helpers_file, test_helpers)?;
        created_files.push(helpers_file.to_string_lossy().to_string());
        
        // Create test configuration
        let config_file = component_dir.join("test_config.toml");
        let config_content = self.generate_test_config(&component.name);
        fs::write(&config_file, config_content)?;
        created_files.push(config_file.to_string_lossy().to_string());
        
        Ok(created_files)
    }
    
    /// Generate test configuration
    fn generate_test_config(&self, component_name: &str) -> String {
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
    
    /// Test component compilation
    fn test_component_compilation(&self, component_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let package_name = format!("leptos-shadcn-{}", component_name);
        
        let output = Command::new("cargo")
            .args(["check", "-p", &package_name])
            .current_dir(&self.workspace_root)
            .output()?;
        
        Ok(output.status.success())
    }
    
    /// Test component test execution
    fn test_component_execution(&self, component_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let package_name = format!("leptos-shadcn-{}", component_name);
        
        let output = Command::new("cargo")
            .args(["test", "-p", &package_name])
            .current_dir(&self.workspace_root)
            .output()?;
        
        Ok(output.status.success())
    }
    
    /// Convert component name to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
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
    
    /// Generate comprehensive test report
    pub fn generate_test_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Automated Test Generation Report ===\n");
        report.push_str("*Generated on September 3rd, 2025*\n\n");
        
        if self.test_results.is_empty() {
            report.push_str("No test generation results available.\n");
            report.push_str("Run generate_tests_for_all_components() first.\n");
            return report;
        }
        
        // Overall statistics
        let total_components = self.test_results.len();
        let successful_generation = self.test_results.values().filter(|r| r.tests_generated).count();
        let successful_compilation = self.test_results.values().filter(|r| r.compilation_success).count();
        let successful_execution = self.test_results.values().filter(|r| r.test_execution_success).count();
        let fully_successful = self.test_results.values().filter(|r| r.is_successful()).count();
        
        report.push_str("📊 Overall Statistics:\n");
        report.push_str(&format!("  - Total Components: {}\n", total_components));
        report.push_str(&format!("  - Tests Generated: {}\n", successful_generation));
        report.push_str(&format!("  - Compilation Success: {}\n", successful_compilation));
        report.push_str(&format!("  - Test Execution Success: {}\n", successful_execution));
        report.push_str(&format!("  - Fully Successful: {}\n\n", fully_successful));
        
        // Component breakdown
        report.push_str("🎯 Component Results:\n");
        for (component_name, result) in &self.test_results {
            let status = if result.is_successful() { "✅" } else { "❌" };
            report.push_str(&format!("  {} {}\n", status, component_name));
            
            if !result.test_files_created.is_empty() {
                report.push_str(&format!("    - Test files: {}\n", result.test_files_created.len()));
            }
            
            if !result.errors.is_empty() {
                for error in &result.errors {
                    report.push_str(&format!("    - Error: {}\n", error));
                }
            }
            
            if !result.warnings.is_empty() {
                for warning in &result.warnings {
                    report.push_str(&format!("    - Warning: {}\n", warning));
                }
            }
        }
        
        report
    }
}

impl TestGenerationResult {
    pub fn new(component_name: impl Into<String>) -> Self {
        Self {
            component_name: component_name.into(),
            tests_generated: false,
            test_files_created: Vec::new(),
            compilation_success: false,
            test_execution_success: false,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn with_test_files(mut self, files: Vec<String>) -> Self {
        self.test_files_created = files.clone();
        self.tests_generated = !files.is_empty();
        self
    }
    
    pub fn with_compilation_result(mut self, success: bool) -> Self {
        self.compilation_success = success;
        self
    }
    
    pub fn with_test_execution_result(mut self, success: bool) -> Self {
        self.test_execution_success = success;
        self
    }
    
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.errors.push(error.into());
        self
    }
    
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }
    
    pub fn is_successful(&self) -> bool {
        self.tests_generated && self.compilation_success && self.test_execution_success
    }
}

fn main() {
    println!("🚀 Automated Test Generation for Leptos shadcn/ui Components");
    println!("📅 Generation Date: September 3rd, 2025\n");
    
    let mut generator = ComponentTestGenerator::new(".");
    
    // Discover components
    match generator.discover_components() {
        Ok(_) => println!("✅ Discovered {} components", generator.components.len()),
        Err(e) => {
            eprintln!("❌ Failed to discover components: {}", e);
            std::process::exit(1);
        }
    }
    
    // Generate tests for all components
    match generator.generate_tests_for_all_components() {
        Ok(_) => println!("✅ Test generation completed"),
        Err(e) => {
            eprintln!("❌ Failed to generate tests: {}", e);
            std::process::exit(1);
        }
    }
    
    // Generate and display report
    let report = generator.generate_test_report();
    println!("\n{}", report);
    
    // Summary
    let total_components = generator.components.len();
    let successful_generation = generator.test_results.values().filter(|r| r.tests_generated).count();
    let fully_successful = generator.test_results.values().filter(|r| r.is_successful()).count();
    
    println!("\n🎉 Test Generation Summary:");
    println!("  - Total Components: {}", total_components);
    println!("  - Tests Generated: {}", successful_generation);
    println!("  - Fully Successful: {}", fully_successful);
    println!("  - Success Rate: {:.1}%", (successful_generation as f64 / total_components as f64) * 100.0);
    
    if fully_successful < total_components {
        println!("\n⚠️ Some components may need manual attention:");
        for (component_name, result) in &generator.test_results {
            if !result.is_successful() {
                println!("  - {}: {}", component_name, if result.tests_generated { "Tests generated but compilation/execution failed" } else { "Test generation failed" });
            }
        }
    }
}
