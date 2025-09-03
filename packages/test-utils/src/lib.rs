//! Testing utilities for shadcn-ui components.
//!
//! This package provides shared testing infrastructure for validating component
//! implementations for the Leptos framework, with support for future framework expansion.

pub mod component_tester;
pub mod theme_validator;
pub mod quality_checker;
pub mod visual_regression;
pub mod leptos_testing;
pub mod test_templates;

use std::collections::HashMap;

/// Framework types for testing (currently Leptos-focused)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Framework {
    Leptos,
    // Future frameworks can be added here
    // Yew,      // Removed - focusing on Leptos completion
    // Dioxus,   // Planned for future expansion
}

/// Theme variants supported by components
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Default,
    NewYork,
}

/// Test execution results
#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub message: String,
    pub details: HashMap<String, String>,
}

/// Component quality test results
#[derive(Debug, Clone)]
pub struct QualityResult {
    pub component_name: String,
    pub quality_score: f64, // 0.0-1.0 quality score
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

impl TestResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            passed: true,
            message: message.into(),
            details: HashMap::new(),
        }
    }
    
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            message: message.into(),
            details: HashMap::new(),
        }
    }
    
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}

impl QualityResult {
    pub fn perfect_score(component_name: impl Into<String>) -> Self {
        Self {
            component_name: component_name.into(),
            quality_score: 1.0,
            issues: vec![],
            recommendations: vec![],
        }
    }
    
    pub fn with_issues(component_name: impl Into<String>, issues: Vec<String>) -> Self {
        let score = if issues.is_empty() {
            1.0
        } else {
            // Simple scoring: reduce by 0.1 per issue, minimum 0.0
            (1.0 - (issues.len() as f64 * 0.1)).max(0.0)
        };
        
        Self {
            component_name: component_name.into(),
            quality_score: score,
            issues,
            recommendations: vec![],
        }
    }
    
    pub fn with_recommendations(mut self, recommendations: Vec<String>) -> Self {
        self.recommendations = recommendations;
        self
    }
}