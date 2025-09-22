//! Automated testing utilities for Leptos shadcn/ui components.
//!
//! This module provides tools for automatically generating, running, and managing
//! tests across all components in the library.

use crate::test_templates::{TestCodeGenerator, ComponentType};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Automated test generator and executor
pub struct AutomatedTestManager {
    pub workspace_root: PathBuf,
    pub test_results: HashMap<String, TestGenerationResult>,
}

/// Result of test generation for a component
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

impl AutomatedTestManager {
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            test_results: HashMap::new(),
        }
    }
    
    /// Generate tests for all components in the workspace
    pub fn generate_tests_for_all_components(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let components_dir = self.workspace_root.join("packages/leptos");
        
        if !components_dir.exists() {
            return Err("Components directory not found".into());
        }
        
        let components = self.discover_components(&components_dir)?;
        
        for component_name in components {
            let result = self.generate_tests_for_component(&component_name)?;
            self.test_results.insert(component_name, result);
        }
        
        Ok(())
    }
    
    /// Discover all available components
    fn discover_components(&self, components_dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut components = Vec::new();
        
        for entry in fs::read_dir(components_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir()
                && let Some(component_name) = path.file_name() {
                    let component_name = component_name.to_string_lossy();
                    if component_name != "shadcn-ui" { // Skip the main package
                        components.push(component_name.to_string());
                    }
                }
        }
        
        Ok(components)
    }
    
    /// Generate tests for a specific component
    fn generate_tests_for_component(&self, component_name: &str) -> Result<TestGenerationResult, Box<dyn std::error::Error>> {
        let mut result = TestGenerationResult::new(component_name);
        
        // Determine component type
        let component_type = ComponentType::from_name(component_name);
        
        // Generate test code
        let test_code = TestCodeGenerator::generate_comprehensive_tests(component_name, component_type);
        let test_helpers = TestCodeGenerator::generate_test_helpers(component_name);
        
        // Create test files
        let test_files = self.create_test_files(component_name, &test_code, &test_helpers)?;
        result = result.with_test_files(test_files);
        
        // Test compilation
        let compilation_success = self.test_component_compilation(component_name)?;
        result = result.with_compilation_result(compilation_success);
        
        // Test execution
        if compilation_success {
            let test_execution_success = self.test_component_execution(component_name)?;
            result = result.with_test_execution_result(test_execution_success);
        }
        
        Ok(result)
    }
    
    /// Create test files for a component
    fn create_test_files(&self, component_name: &str, test_code: &str, test_helpers: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut created_files = Vec::new();
        let component_dir = self.workspace_root.join("packages/leptos").join(component_name);
        
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
        let config_content = TestCodeGenerator::generate_test_config(component_name);
        fs::write(&config_file, config_content)?;
        created_files.push(config_file.to_string_lossy().to_string());
        
        Ok(created_files)
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
    
    /// Generate comprehensive test report
    pub fn generate_test_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Automated Test Generation Report ===\n\n");
        
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
