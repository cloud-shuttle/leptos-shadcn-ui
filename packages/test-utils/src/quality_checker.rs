//! Component quality checking utilities for Leptos shadcn/ui components.

use crate::QualityResult;
use std::collections::HashMap;

/// Component API specification for quality validation
#[derive(Debug, Clone)]
pub struct ComponentSpec {
    pub name: String,
    pub props: HashMap<String, PropSpec>,
    pub events: Vec<String>,
    pub variants: Vec<String>,
    pub sizes: Vec<String>,
}

/// Property specification with type and requirements
#[derive(Debug, Clone)]
pub struct PropSpec {
    pub prop_type: String,
    pub required: bool,
    pub default_value: Option<String>,
}

/// Leptos component implementation details
#[derive(Debug, Clone)]
pub struct LeptosImplementation {
    pub component_spec: ComponentSpec,
    pub css_classes: Vec<String>,
    pub dependencies: Vec<String>,
    pub theme_variants: Vec<String>,
}

/// Checks quality of Leptos component implementations
pub struct QualityChecker {
    implementations: HashMap<String, LeptosImplementation>,
}

impl QualityChecker {
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new(),
        }
    }
    
    pub fn add_implementation(mut self, name: String, implementation: LeptosImplementation) -> Self {
        self.implementations.insert(name, implementation);
        self
    }
    
    /// Check overall quality of all registered components
    pub fn check_all_components(&self) -> Vec<QualityResult> {
        self.implementations
            .iter()
            .map(|(name, implementation)| self.check_component_quality(name, implementation))
            .collect()
    }
    
    /// Check quality of a specific component
    pub fn check_component_quality(&self, name: &str, implementation: &LeptosImplementation) -> QualityResult {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Check props consistency
        if implementation.component_spec.props.is_empty() {
            issues.push("No props defined".to_string());
            recommendations.push("Consider adding props for customization".to_string());
        }
        
        // Check theme variants
        if implementation.theme_variants.is_empty() {
            issues.push("No theme variants implemented".to_string());
            recommendations.push("Implement both 'default' and 'new_york' themes".to_string());
        } else if !implementation.theme_variants.contains(&"default".to_string()) {
            issues.push("Missing 'default' theme variant".to_string());
            recommendations.push("Always implement the 'default' theme variant".to_string());
        }
        
        // Check dependencies
        if implementation.dependencies.is_empty() {
            issues.push("No dependencies specified".to_string());
            recommendations.push("Specify required dependencies in Cargo.toml".to_string());
        }
        
        // Check CSS classes
        if implementation.css_classes.is_empty() {
            issues.push("No CSS classes defined".to_string());
            recommendations.push("Implement proper Tailwind CSS classes".to_string());
        }
        
        QualityResult::with_issues(name.to_string(), issues)
            .with_recommendations(recommendations)
    }
    
    /// Check theme consistency across components
    pub fn check_theme_consistency(&self) -> QualityResult {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        let mut theme_counts = HashMap::new();
        for implementation in self.implementations.values() {
            for theme in &implementation.theme_variants {
                *theme_counts.entry(theme.clone()).or_insert(0) += 1;
            }
        }
        
        // Check if all components have consistent theme coverage
        let total_components = self.implementations.len();
        if let Some(default_count) = theme_counts.get("default") {
            if *default_count < total_components {
                issues.push(format!(
                    "Only {}/{} components have 'default' theme",
                    default_count, total_components
                ));
                recommendations.push("Ensure all components implement 'default' theme".to_string());
            }
        }
        
        if let Some(new_york_count) = theme_counts.get("new_york") {
            if *new_york_count < total_components {
                recommendations.push("Consider implementing 'new_york' theme for all components".to_string());
            }
        }
        
        QualityResult::with_issues("theme_consistency".to_string(), issues)
            .with_recommendations(recommendations)
    }
    
    /// Generate quality report for all components
    pub fn generate_quality_report(&self) -> String {
        let component_results = self.check_all_components();
        let theme_result = self.check_theme_consistency();
        
        let mut report = String::new();
        report.push_str("=== Leptos shadcn/ui Component Quality Report ===\n\n");
        
        // Component-specific results
        report.push_str("Component Quality Scores:\n");
        for result in component_results {
            report.push_str(&format!(
                "  {}: {:.1}% ({})\n",
                result.component_name,
                result.quality_score * 100.0,
                if result.quality_score >= 0.9 { "✅" } else if result.quality_score >= 0.7 { "⚠️" } else { "❌" }
            ));
            
            if !result.issues.is_empty() {
                report.push_str("    Issues:\n");
                for issue in &result.issues {
                    report.push_str(&format!("      - {}\n", issue));
                }
            }
            
            if !result.recommendations.is_empty() {
                report.push_str("    Recommendations:\n");
                for rec in &result.recommendations {
                    report.push_str(&format!("      - {}\n", rec));
                }
            }
        }
        
        // Theme consistency results
        report.push_str(&format!(
            "\nTheme Consistency: {:.1}% ({})\n",
            theme_result.quality_score * 100.0,
            if theme_result.quality_score >= 0.9 { "✅" } else if theme_result.quality_score >= 0.7 { "⚠️" } else { "❌" }
        ));
        
        if !theme_result.issues.is_empty() {
            report.push_str("  Issues:\n");
            for issue in &theme_result.issues {
                report.push_str(&format!("    - {}\n", issue));
            }
        }
        
        if !theme_result.recommendations.is_empty() {
            report.push_str("  Recommendations:\n");
            for rec in &theme_result.recommendations {
                report.push_str(&format!("    - {}\n", rec));
            }
        }
        
        report
    }
}