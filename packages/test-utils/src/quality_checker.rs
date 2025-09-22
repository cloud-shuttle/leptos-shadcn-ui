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
    pub accessibility_features: Vec<String>,
    pub responsive_breakpoints: Vec<String>,
}

/// Property specification with type and requirements
#[derive(Debug, Clone)]
pub struct PropSpec {
    pub prop_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation_rules: Vec<String>,
    pub documentation: Option<String>,
}

/// Leptos component implementation details
#[derive(Debug, Clone)]
pub struct LeptosImplementation {
    pub component_spec: ComponentSpec,
    pub css_classes: Vec<String>,
    pub dependencies: Vec<String>,
    pub theme_variants: Vec<String>,
    pub test_coverage: f64,
    pub documentation_quality: f64,
    pub performance_metrics: HashMap<String, f64>,
}

/// Enhanced quality checking with detailed analysis
pub struct QualityChecker {
    implementations: HashMap<String, LeptosImplementation>,
    quality_thresholds: QualityThresholds,
}

/// Quality thresholds for different aspects
#[derive(Debug, Clone)]
pub struct QualityThresholds {
    pub min_props_count: usize,
    pub min_theme_variants: usize,
    pub min_test_coverage: f64,
    pub min_documentation_quality: f64,
    pub required_accessibility_features: Vec<String>,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            min_props_count: 3,
            min_theme_variants: 2,
            min_test_coverage: 0.8,
            min_documentation_quality: 0.7,
            required_accessibility_features: vec![
                "aria-label".to_string(),
                "keyboard-navigation".to_string(),
                "focus-management".to_string(),
            ],
        }
    }
}

impl Default for QualityChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl QualityChecker {
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new(),
            quality_thresholds: QualityThresholds::default(),
        }
    }
    
    pub fn with_thresholds(mut self, thresholds: QualityThresholds) -> Self {
        self.quality_thresholds = thresholds;
        self
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
    
    /// Check quality of a specific component with enhanced analysis
    pub fn check_component_quality(&self, name: &str, implementation: &LeptosImplementation) -> QualityResult {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut score_components = Vec::new();
        
        // Check props consistency and quality
        let props_score = self.check_props_quality(implementation, &mut issues, &mut recommendations);
        score_components.push(("props", props_score));
        
        // Check theme variants
        let theme_score = self.check_theme_quality(implementation, &mut issues, &mut recommendations);
        score_components.push(("themes", theme_score));
        
        // Check dependencies
        let deps_score = self.check_dependencies_quality(implementation, &mut issues, &mut recommendations);
        score_components.push(("dependencies", deps_score));
        
        // Check CSS classes and styling
        let styling_score = self.check_styling_quality(implementation, &mut issues, &mut recommendations);
        score_components.push(("styling", styling_score));
        
        // Check accessibility features
        let accessibility_score = self.check_accessibility_quality(implementation, &mut issues, &mut recommendations);
        score_components.push(("accessibility", accessibility_score));
        
        // Check test coverage
        let test_score = self.check_test_coverage(implementation, &mut issues, &mut recommendations);
        score_components.push(("testing", test_score));
        
        // Check documentation quality
        let doc_score = self.check_documentation_quality(implementation, &mut issues, &mut recommendations);
        score_components.push(("documentation", doc_score));
        
        // Calculate overall quality score (weighted average)
        let overall_score = self.calculate_weighted_score(&score_components);
        
        QualityResult::with_issues(name.to_string(), issues)
            .with_recommendations(recommendations)
            .with_quality_score(overall_score)
    }
    
    /// Check props quality with detailed analysis
    fn check_props_quality(&self, implementation: &LeptosImplementation, issues: &mut Vec<String>, recommendations: &mut Vec<String>) -> f64 {
        let props = &implementation.component_spec.props;
        let mut score = 1.0;
        
        if props.is_empty() {
            issues.push("No props defined - component lacks customization options".to_string());
            recommendations.push("Add props for common customization needs (class, id, style, children)".to_string());
            score = 0.0;
        } else if props.len() < self.quality_thresholds.min_props_count {
            issues.push(format!("Only {} props defined - consider adding more customization options", props.len()));
            recommendations.push("Add more props for flexibility and customization".to_string());
            score = 0.5;
        }
        
        // Check for required props
        let required_props = props.values().filter(|p| p.required).count();
        if required_props == 0 {
            recommendations.push("Consider adding required props for essential functionality".to_string());
        }
        
        // Check prop documentation
        let documented_props = props.values().filter(|p| p.documentation.is_some()).count();
        if documented_props < props.len() {
            recommendations.push("Document all props for better developer experience".to_string());
            score *= 0.9;
        }
        
        score
    }
    
    /// Check theme quality and consistency
    fn check_theme_quality(&self, implementation: &LeptosImplementation, issues: &mut Vec<String>, recommendations: &mut Vec<String>) -> f64 {
        let themes = &implementation.theme_variants;
        let mut score = 1.0;
        
        if themes.is_empty() {
            issues.push("No theme variants implemented".to_string());
            recommendations.push("Implement both 'default' and 'new_york' themes for consistency".to_string());
            score = 0.0;
        } else if themes.len() < self.quality_thresholds.min_theme_variants {
            issues.push(format!("Only {} theme variant(s) - expected at least {}", themes.len(), self.quality_thresholds.min_theme_variants));
            recommendations.push("Implement both default and new_york themes".to_string());
            score = 0.5;
        }
        
        if !themes.contains(&"default".to_string()) {
            issues.push("Missing 'default' theme variant".to_string());
            recommendations.push("Always implement the 'default' theme variant".to_string());
            score *= 0.7;
        }
        
        if !themes.contains(&"new_york".to_string()) {
            recommendations.push("Consider adding 'new_york' theme variant for design system consistency".to_string());
            score *= 0.9;
        }
        
        score
    }
    
    /// Check dependencies quality
    fn check_dependencies_quality(&self, implementation: &LeptosImplementation, issues: &mut Vec<String>, recommendations: &mut Vec<String>) -> f64 {
        let deps = &implementation.dependencies;
        let mut score = 1.0;
        
        if deps.is_empty() {
            issues.push("No dependencies specified".to_string());
            recommendations.push("Specify required dependencies in Cargo.toml".to_string());
            score = 0.0;
        }
        
        // Check for essential Leptos dependencies
        let has_leptos = deps.iter().any(|d| d.contains("leptos"));
        if !has_leptos {
            issues.push("Missing core Leptos dependency".to_string());
            recommendations.push("Ensure leptos dependency is properly specified".to_string());
            score *= 0.5;
        }
        
        // Check for leptos_style dependency
        let has_leptos_style = deps.iter().any(|d| d.contains("leptos_style"));
        if !has_leptos_style {
            recommendations.push("Consider adding leptos_style for enhanced styling capabilities".to_string());
            score *= 0.9;
        }
        
        score
    }
    
    /// Check styling quality
    fn check_styling_quality(&self, implementation: &LeptosImplementation, issues: &mut Vec<String>, recommendations: &mut Vec<String>) -> f64 {
        let classes = &implementation.css_classes;
        let mut score = 1.0;
        
        if classes.is_empty() {
            issues.push("No CSS classes defined".to_string());
            recommendations.push("Implement proper Tailwind CSS classes for styling".to_string());
            score = 0.0;
        }
        
        // Check for responsive classes
        let has_responsive = classes.iter().any(|c| c.contains("sm:") || c.contains("md:") || c.contains("lg:"));
        if !has_responsive {
            recommendations.push("Consider adding responsive design classes".to_string());
            score *= 0.9;
        }
        
        // Check for dark mode support
        let has_dark_mode = classes.iter().any(|c| c.contains("dark:"));
        if !has_dark_mode {
            recommendations.push("Consider adding dark mode support classes".to_string());
            score *= 0.95;
        }
        
        score
    }
    
    /// Check accessibility quality
    fn check_accessibility_quality(&self, implementation: &LeptosImplementation, issues: &mut Vec<String>, recommendations: &mut Vec<String>) -> f64 {
        let features = &implementation.component_spec.accessibility_features;
        let mut score = 1.0;
        
        for required_feature in &self.quality_thresholds.required_accessibility_features {
            if !features.contains(required_feature) {
                issues.push(format!("Missing required accessibility feature: {}", required_feature));
                recommendations.push(format!("Implement {} for better accessibility", required_feature));
                score *= 0.8;
            }
        }
        
        if features.is_empty() {
            recommendations.push("Add accessibility features like ARIA labels and keyboard navigation".to_string());
            score *= 0.7;
        }
        
        score
    }
    
    /// Check test coverage
    fn check_test_coverage(&self, implementation: &LeptosImplementation, issues: &mut Vec<String>, recommendations: &mut Vec<String>) -> f64 {
        let coverage = implementation.test_coverage;
        let mut score = 1.0;
        
        if coverage < self.quality_thresholds.min_test_coverage {
            issues.push(format!("Test coverage {}% is below threshold of {}%", 
                (coverage * 100.0) as i32, 
                (self.quality_thresholds.min_test_coverage * 100.0) as i32));
            recommendations.push("Increase test coverage by adding more test cases".to_string());
            score = coverage;
        }
        
        if coverage < 0.5 {
            recommendations.push("Consider implementing comprehensive test suite".to_string());
        }
        
        score
    }
    
    /// Check documentation quality
    fn check_documentation_quality(&self, implementation: &LeptosImplementation, issues: &mut Vec<String>, recommendations: &mut Vec<String>) -> f64 {
        let doc_quality = implementation.documentation_quality;
        let mut score = 1.0;
        
        if doc_quality < self.quality_thresholds.min_documentation_quality {
            issues.push(format!("Documentation quality {}% is below threshold of {}%", 
                (doc_quality * 100.0) as i32, 
                (self.quality_thresholds.min_documentation_quality * 100.0) as i32));
            recommendations.push("Improve component documentation with examples and usage patterns".to_string());
            score = doc_quality;
        }
        
        score
    }
    
    /// Calculate weighted quality score
    fn calculate_weighted_score(&self, score_components: &[(&str, f64)]) -> f64 {
        let weights = HashMap::from([
            ("props", 0.15),
            ("themes", 0.15),
            ("dependencies", 0.10),
            ("styling", 0.15),
            ("accessibility", 0.20),
            ("testing", 0.15),
            ("documentation", 0.10),
        ]);
        
        let mut total_weight = 0.0;
        let mut weighted_sum = 0.0;
        
        for (component, score) in score_components {
            if let Some(&weight) = weights.get(component) {
                weighted_sum += score * weight;
                total_weight += weight;
            }
        }
        
        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
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
        let expected_components = self.implementations.len();
        for (theme, count) in theme_counts {
            if count < expected_components {
                issues.push(format!("Theme '{}' missing from {} components", theme, expected_components - count));
                recommendations.push(format!("Ensure all components implement '{}' theme", theme));
            }
        }
        
        QualityResult::with_issues("theme_consistency".to_string(), issues)
            .with_recommendations(recommendations)
    }
    
    /// Generate comprehensive quality report
    pub fn generate_quality_report(&self) -> String {
        let results = self.check_all_components();
        let mut report = String::new();
        
        report.push_str("=== Component Quality Report ===\n\n");
        
        // Overall statistics
        let total_components = results.len();
        let avg_score = results.iter().map(|r| r.quality_score).sum::<f64>() / total_components as f64;
        let high_quality = results.iter().filter(|r| r.quality_score >= 0.8).count();
        let needs_improvement = results.iter().filter(|r| r.quality_score < 0.6).count();
        
        report.push_str(&"📊 Overall Statistics:\n".to_string());
        report.push_str(&format!("  - Total Components: {}\n", total_components));
        report.push_str(&format!("  - Average Quality Score: {:.1}%\n", avg_score * 100.0));
        report.push_str(&format!("  - High Quality (≥80%): {}\n", high_quality));
        report.push_str(&format!("  - Needs Improvement (<60%): {}\n\n", needs_improvement));
        
        // Component breakdown
        report.push_str("🎯 Component Breakdown:\n");
        for result in &results {
            let status = if result.quality_score >= 0.8 { "✅" } else if result.quality_score >= 0.6 { "⚠️" } else { "❌" };
            report.push_str(&format!("  {} {}: {:.1}%\n", status, result.component_name, result.quality_score * 100.0));
            
            if !result.issues.is_empty() {
                for issue in &result.issues {
                    report.push_str(&format!("    - Issue: {}\n", issue));
                }
            }
            
            if !result.recommendations.is_empty() {
                for rec in &result.recommendations {
                    report.push_str(&format!("    - Recommendation: {}\n", rec));
                }
            }
            report.push('\n');
        }
        
        report
    }
}