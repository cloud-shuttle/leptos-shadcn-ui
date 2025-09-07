//! # leptos-shadcn API Standards Framework
//!
//! This crate provides comprehensive API standardization and validation tools
//! for leptos-shadcn-ui components, ensuring consistent and accessible component APIs.

use std::collections::HashMap;

pub mod props;
pub mod events;
pub mod accessibility;
pub mod css;
pub mod validation;
pub mod linting;
pub mod testing;

/// Standard component variant types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum StandardVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
    Light,
    Dark,
}

impl StandardVariant {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            StandardVariant::Default => "default",
            StandardVariant::Primary => "primary", 
            StandardVariant::Secondary => "secondary",
            StandardVariant::Success => "success",
            StandardVariant::Warning => "warning",
            StandardVariant::Danger => "danger",
            StandardVariant::Info => "info",
            StandardVariant::Light => "light",
            StandardVariant::Dark => "dark",
        }
    }

    pub fn all_variants() -> Vec<StandardVariant> {
        vec![
            StandardVariant::Default,
            StandardVariant::Primary,
            StandardVariant::Secondary,
            StandardVariant::Success,
            StandardVariant::Warning,
            StandardVariant::Danger,
            StandardVariant::Info,
            StandardVariant::Light,
            StandardVariant::Dark,
        ]
    }
}

/// Standard component size types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum StandardSize {
    Xs,
    Sm,
    Default,
    Lg,
    Xl,
    Responsive,
}

impl StandardSize {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            StandardSize::Xs => "xs",
            StandardSize::Sm => "sm",
            StandardSize::Default => "default",
            StandardSize::Lg => "lg",
            StandardSize::Xl => "xl",
            StandardSize::Responsive => "responsive",
        }
    }

    pub fn all_sizes() -> Vec<StandardSize> {
        vec![
            StandardSize::Xs,
            StandardSize::Sm,
            StandardSize::Default,
            StandardSize::Lg,
            StandardSize::Xl,
            StandardSize::Responsive,
        ]
    }
}

/// Component API compliance report
#[derive(Debug, Clone)]
pub struct ApiComplianceReport {
    pub component_name: String,
    pub compliance_score: f64,
    pub issues: Vec<ApiIssue>,
    pub suggestions: Vec<ApiSuggestion>,
    pub test_results: HashMap<String, TestResult>,
}

/// API compliance issues
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ApiIssue {
    MissingCoreProps(Vec<String>),
    InvalidPropType { prop: String, expected: String, actual: String },
    AccessibilityViolation { rule: String, description: String },
    EventHandlerMissing(String),
    CssClassNonCompliant { expected_pattern: String, actual: String },
    PerformanceViolation { metric: String, threshold: f64, actual: f64 },
}

/// API improvement suggestions
#[derive(Debug, Clone)]
pub enum ApiSuggestion {
    AddOptionalProp(String),
    ImproveAccessibility(String),
    OptimizePerformance(String),
    EnhanceDocumentation(String),
    FollowNamingConvention { current: String, suggested: String },
}

/// Test execution results
#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub execution_time_ms: u64,
    pub message: String,
    pub details: HashMap<String, serde_json::Value>,
}

impl TestResult {
    pub fn passed(message: impl Into<String>) -> Self {
        Self {
            passed: true,
            execution_time_ms: 0,
            message: message.into(),
            details: HashMap::new(),
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            execution_time_ms: 0,
            message: message.into(),
            details: HashMap::new(),
        }
    }

    pub fn with_timing(mut self, duration_ms: u64) -> Self {
        self.execution_time_ms = duration_ms;
        self
    }

    pub fn with_detail(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.details.insert(key.into(), value);
        self
    }
}

/// Component API compliance trait
pub trait ApiCompliant {
    type Props;
    
    /// Test basic component rendering
    fn test_basic_rendering(&self) -> TestResult;
    
    /// Test prop handling compliance
    fn test_prop_handling(&self) -> TestResult;
    
    /// Test accessibility compliance
    fn test_accessibility_compliance(&self) -> TestResult;
    
    /// Test event handling compliance
    fn test_event_handling(&self) -> TestResult;
    
    /// Test CSS class generation compliance
    fn test_css_compliance(&self) -> TestResult;
    
    /// Test performance characteristics
    fn test_performance_compliance(&self) -> TestResult;
    
    /// Generate comprehensive compliance report
    fn generate_compliance_report(&self) -> ApiComplianceReport {
        let component_name = std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("Unknown")
            .to_string();

        let mut test_results = HashMap::new();
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Run all compliance tests
        let tests = vec![
            ("basic_rendering", self.test_basic_rendering()),
            ("prop_handling", self.test_prop_handling()),
            ("accessibility", self.test_accessibility_compliance()),
            ("event_handling", self.test_event_handling()),
            ("css_compliance", self.test_css_compliance()),
            ("performance", self.test_performance_compliance()),
        ];

        let mut passed_tests = 0;
        for (test_name, result) in tests {
            if result.passed {
                passed_tests += 1;
            } else {
                issues.push(ApiIssue::PerformanceViolation {
                    metric: test_name.to_string(),
                    threshold: 1.0,
                    actual: 0.0,
                });
            }
            test_results.insert(test_name.to_string(), result);
        }

        let compliance_score = passed_tests as f64 / test_results.len() as f64;

        ApiComplianceReport {
            component_name,
            compliance_score,
            issues,
            suggestions,
            test_results,
        }
    }
}

/// Utility functions for API standardization
pub mod utils {
    use super::*;

    /// Generate unique component ID
    pub fn generate_component_id(component_name: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        component_name.hash(&mut hasher);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        format!("{}-{:x}", component_name.to_lowercase(), hasher.finish() ^ (timestamp as u64))
    }

    /// Generate CSS classes following component standards
    pub fn generate_standard_classes(
        component_name: &str,
        variant: &StandardVariant,
        size: &StandardSize,
        custom_class: Option<&str>,
    ) -> String {
        let mut classes = vec![
            format!("shadcn-{}", component_name.to_lowercase()),
            format!("shadcn-{}--{}", component_name.to_lowercase(), variant.to_css_class()),
            format!("shadcn-{}--{}", component_name.to_lowercase(), size.to_css_class()),
        ];

        if let Some(custom) = custom_class {
            classes.push(custom.to_string());
        }

        classes.join(" ")
    }

    /// Validate CSS class naming convention
    pub fn validate_css_class_name(class_name: &str) -> Result<(), String> {
        let regex = regex::Regex::new(r"^[a-z][a-z0-9-]*[a-z0-9]$").unwrap();
        
        if !regex.is_match(class_name) {
            return Err(format!(
                "CSS class '{}' does not follow naming convention. Should match pattern: ^[a-z][a-z0-9-]*[a-z0-9]$",
                class_name
            ));
        }

        if class_name.contains("--") && !class_name.starts_with("shadcn-") {
            return Err(format!(
                "CSS class '{}' uses BEM modifier syntax but is not a shadcn component class",
                class_name
            ));
        }

        Ok(())
    }

    /// Calculate API compliance score
    pub fn calculate_compliance_score(
        issues: &[ApiIssue], 
        suggestions: &[ApiSuggestion]
    ) -> f64 {
        let critical_issues = issues.iter().filter(|issue| {
            matches!(issue, 
                ApiIssue::AccessibilityViolation { .. } |
                ApiIssue::MissingCoreProps(_) |
                ApiIssue::PerformanceViolation { .. }
            )
        }).count();

        let minor_issues = issues.len() - critical_issues;
        let suggestion_bonus = (suggestions.len() as f64 * 0.05).min(0.2);

        let base_score = 1.0 - (critical_issues as f64 * 0.25) - (minor_issues as f64 * 0.1);
        (base_score + suggestion_bonus).max(0.0).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_variant_css_classes() {
        assert_eq!(StandardVariant::Default.to_css_class(), "default");
        assert_eq!(StandardVariant::Primary.to_css_class(), "primary");
        assert_eq!(StandardVariant::Danger.to_css_class(), "danger");
    }

    #[test]
    fn test_standard_size_css_classes() {
        assert_eq!(StandardSize::Default.to_css_class(), "default");
        assert_eq!(StandardSize::Lg.to_css_class(), "lg");
        assert_eq!(StandardSize::Responsive.to_css_class(), "responsive");
    }

    #[test]
    fn test_generate_standard_classes() {
        let classes = utils::generate_standard_classes(
            "Button",
            &StandardVariant::Primary,
            &StandardSize::Lg,
            Some("custom-class")
        );
        
        assert_eq!(classes, "shadcn-button shadcn-button--primary shadcn-button--lg custom-class");
    }

    #[test]
    fn test_css_class_name_validation() {
        assert!(utils::validate_css_class_name("valid-class-name").is_ok());
        assert!(utils::validate_css_class_name("shadcn-button--primary").is_ok());
        
        assert!(utils::validate_css_class_name("Invalid-Class").is_err());
        assert!(utils::validate_css_class_name("invalid--modifier").is_err());
        assert!(utils::validate_css_class_name("123invalid").is_err());
    }

    #[test]
    fn test_compliance_score_calculation() {
        let issues = vec![
            ApiIssue::AccessibilityViolation { 
                rule: "ARIA".to_string(), 
                description: "Missing label".to_string() 
            },
            ApiIssue::MissingCoreProps(vec!["id".to_string()]),
        ];
        let suggestions = vec![
            ApiSuggestion::ImproveAccessibility("Add aria-label".to_string()),
        ];

        let score = utils::calculate_compliance_score(&issues, &suggestions);
        
        // Should be 1.0 - (2 critical * 0.25) + (1 suggestion * 0.05) = 0.55
        assert!((score - 0.55).abs() < 0.01);
    }

    #[test]
    fn test_generate_component_id() {
        let id1 = utils::generate_component_id("Button");
        let id2 = utils::generate_component_id("Button");
        
        assert!(id1.starts_with("button-"));
        assert!(id2.starts_with("button-"));
        assert_ne!(id1, id2); // Should be unique
    }

    #[test]
    fn test_test_result_creation() {
        let result = TestResult::passed("Test completed successfully")
            .with_timing(150)
            .with_detail("render_time", serde_json::json!(12));
        
        assert!(result.passed);
        assert_eq!(result.execution_time_ms, 150);
        assert_eq!(result.message, "Test completed successfully");
        assert_eq!(result.details["render_time"], serde_json::json!(12));
    }
}