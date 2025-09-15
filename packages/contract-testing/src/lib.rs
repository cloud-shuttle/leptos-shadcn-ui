//! # Contract Testing Framework for Leptos ShadCN UI
//!
//! Test-driven development framework ensuring API compatibility across components.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Validation failed: {message}")]
    ValidationError { message: String },
    #[error("API compatibility broken: {details}")]
    CompatibilityError { details: String },
    #[error("Performance requirement not met: {requirement}")]
    PerformanceError { requirement: String },
}

/// Core contract trait that all components must implement
pub trait ComponentContract {
    type Props: Clone + PartialEq + Send + Sync;
    type Events: Clone + Send + Sync;

    /// Validate component props meet contract requirements
    fn validate_props(props: &Self::Props) -> Result<(), ContractError>;

    /// Required CSS classes for proper styling
    fn required_css_classes() -> Vec<&'static str>;

    /// Accessibility contract requirements
    fn accessibility_requirements() -> AccessibilityContract;

    /// Performance contract requirements
    fn performance_requirements() -> PerformanceContract;

    /// API version for compatibility checking
    fn api_version() -> semver::Version;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityContract {
    pub required_aria_attributes: Vec<String>,
    pub keyboard_navigation: bool,
    pub screen_reader_support: bool,
    pub color_contrast_compliance: bool,
    pub focus_management: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceContract {
    pub max_render_time_ms: u64,
    pub max_bundle_size_kb: u64,
    pub max_memory_usage_mb: u64,
    pub supports_ssr: bool,
    pub supports_hydration: bool,
}

/// Contract testing runner
pub struct ContractTester {
    components: HashMap<String, Box<dyn ContractTestable>>,
}

pub trait ContractTestable {
    fn run_validation_tests(&self) -> Result<(), ContractError>;
    fn run_performance_tests(&self) -> Result<(), ContractError>;
    fn run_accessibility_tests(&self) -> Result<(), ContractError>;
    fn run_compatibility_tests(&self, other: &dyn ContractTestable) -> Result<(), ContractError>;
}

impl ContractTester {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register_component<T: ContractTestable + 'static>(&mut self, name: String, component: T) {
        self.components.insert(name, Box::new(component));
    }

    /// Run all contract tests with nextest parallel execution
    pub async fn run_all_tests(&self) -> Result<(), ContractError> {
        for (name, component) in &self.components {
            println!("🧪 Testing component: {}", name);

            component.run_validation_tests()?;
            component.run_performance_tests()?;
            component.run_accessibility_tests()?;
        }

        // Run compatibility matrix
        self.run_compatibility_matrix().await?;

        Ok(())
    }

    async fn run_compatibility_matrix(&self) -> Result<(), ContractError> {
        let components: Vec<_> = self.components.values().collect();

        for (i, component_a) in components.iter().enumerate() {
            for component_b in components.iter().skip(i + 1) {
                component_a.run_compatibility_tests(component_b.as_ref())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_contract_tester_creation() {
        let tester = ContractTester::new();
        assert_eq!(tester.components.len(), 0);
    }

    #[tokio::test]
    async fn test_accessibility_contract_creation() {
        let contract = AccessibilityContract {
            required_aria_attributes: vec!["role".to_string(), "aria-label".to_string()],
            keyboard_navigation: true,
            screen_reader_support: true,
            color_contrast_compliance: true,
            focus_management: true,
        };

        assert_eq!(contract.required_aria_attributes.len(), 2);
        assert!(contract.keyboard_navigation);
    }

    #[tokio::test]
    async fn test_performance_contract_creation() {
        let contract = PerformanceContract {
            max_render_time_ms: 16, // 60fps
            max_bundle_size_kb: 50,
            max_memory_usage_mb: 10,
            supports_ssr: true,
            supports_hydration: true,
        };

        assert_eq!(contract.max_render_time_ms, 16);
        assert!(contract.supports_ssr);
    }
}

// Sub-modules
pub mod dependency_contracts;
pub mod dependency_fixer;
pub mod wasm_performance;
pub mod tdd_expansion;

// Re-export commonly used items
pub use semver;
pub use dependency_contracts::DependencyContractTester;
pub use dependency_fixer::DependencyFixer;
pub use tdd_expansion::TddExpansionManager;
pub use wasm_performance::WasmPerformanceTester;