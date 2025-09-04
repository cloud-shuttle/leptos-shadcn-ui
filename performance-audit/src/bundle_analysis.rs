//! Bundle Size Analysis Module
//! 
//! This module provides comprehensive bundle size analysis for leptos-shadcn-ui components
//! using TDD principles to ensure optimal performance.

use std::collections::BTreeMap;
use std::path::PathBuf;

/// Bundle size analysis results for a single component
#[derive(Debug, Clone)]
pub struct ComponentBundleAnalysis {
    /// Component name
    pub component_name: String,
    /// Bundle size in bytes
    pub bundle_size_bytes: u64,
    /// Bundle size in KB
    pub bundle_size_kb: f64,
    /// Gzipped size in bytes
    pub gzipped_size_bytes: u64,
    /// Gzipped size in KB
    pub gzipped_size_kb: f64,
    /// Dependencies count
    pub dependencies_count: usize,
    /// Tree-shaking efficiency (0-100%)
    pub tree_shaking_efficiency: f64,
    /// Meets size target
    pub meets_size_target: bool,
}

impl ComponentBundleAnalysis {
    /// Create new component bundle analysis
    pub fn new(component_name: String, bundle_size_bytes: u64) -> Self {
        let bundle_size_kb = bundle_size_bytes as f64 / 1024.0;
        let gzipped_size_bytes = (bundle_size_bytes as f64 * 0.3) as u64; // Estimate 30% compression
        let gzipped_size_kb = gzipped_size_bytes as f64 / 1024.0;
        
        Self {
            component_name,
            bundle_size_bytes,
            bundle_size_kb,
            gzipped_size_bytes,
            gzipped_size_kb,
            dependencies_count: 0,
            tree_shaking_efficiency: 0.0,
            meets_size_target: bundle_size_kb <= 5.0, // Target: < 5KB
        }
    }
    
    /// Calculate performance score for this component
    pub fn performance_score(&self) -> f64 {
        let size_score = if self.meets_size_target { 100.0 } else { 
            (5.0 / self.bundle_size_kb * 100.0).min(100.0) 
        };
        let efficiency_score = self.tree_shaking_efficiency;
        
        (size_score + efficiency_score) / 2.0
    }
}

/// Overall bundle analysis results
#[derive(Debug, Clone)]
pub struct BundleAnalysisResults {
    /// Individual component analyses (using BTreeMap for sorted iteration)
    pub component_analyses: BTreeMap<String, ComponentBundleAnalysis>,
    /// Total bundle size in bytes
    pub total_bundle_size_bytes: u64,
    /// Total bundle size in KB
    pub total_bundle_size_kb: f64,
    /// Average component size in KB
    pub average_component_size_kb: f64,
    /// Largest component size in KB
    pub largest_component_size_kb: f64,
    /// Components exceeding size target
    pub oversized_components: Vec<String>,
    /// Overall bundle efficiency score (0-100)
    pub overall_efficiency_score: f64,
}

impl Default for BundleAnalysisResults {
    fn default() -> Self {
        Self {
            component_analyses: BTreeMap::new(),
            total_bundle_size_bytes: 0,
            total_bundle_size_kb: 0.0,
            average_component_size_kb: 0.0,
            largest_component_size_kb: 0.0,
            oversized_components: Vec::new(),
            overall_efficiency_score: 0.0,
        }
    }
}

impl BundleAnalysisResults {
    /// Add component analysis
    pub fn add_component(&mut self, analysis: ComponentBundleAnalysis) {
        let component_name = analysis.component_name.clone();
        self.component_analyses.insert(component_name.clone(), analysis);
        self.recalculate_totals();
    }
    
    /// Recalculate totals and statistics
    fn recalculate_totals(&mut self) {
        self.total_bundle_size_bytes = self.component_analyses
            .values()
            .map(|a| a.bundle_size_bytes)
            .sum();
        
        self.total_bundle_size_kb = self.total_bundle_size_bytes as f64 / 1024.0;
        
        if !self.component_analyses.is_empty() {
            self.average_component_size_kb = self.total_bundle_size_kb / self.component_analyses.len() as f64;
            
            self.largest_component_size_kb = self.component_analyses
                .values()
                .map(|a| a.bundle_size_kb)
                .fold(0.0, f64::max);
            
            self.oversized_components = self.component_analyses
                .iter()
                .filter(|(_, analysis)| !analysis.meets_size_target)
                .map(|(name, _)| name.clone())
                .collect();
            
            self.overall_efficiency_score = self.component_analyses
                .values()
                .map(|a| a.performance_score())
                .sum::<f64>() / self.component_analyses.len() as f64;
        }
    }
    
    /// Check if bundle analysis meets targets
    pub fn meets_targets(&self) -> bool {
        self.overall_efficiency_score >= 80.0 && self.oversized_components.is_empty()
    }
    
    /// Get optimization recommendations
    pub fn get_optimization_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !self.oversized_components.is_empty() {
            recommendations.push(format!(
                "Optimize oversized components: {}", 
                self.oversized_components.join(", ")
            ));
        }
        
        if self.average_component_size_kb > 3.0 {
            recommendations.push("Reduce average component size through code splitting".to_string());
        }
        
        if self.overall_efficiency_score < 70.0 {
            recommendations.push("Improve tree-shaking efficiency across components".to_string());
        }
        
        recommendations
    }
}

/// Bundle analyzer for leptos-shadcn-ui components
pub struct BundleAnalyzer {
    /// Components directory path
    pub components_path: PathBuf,
    /// Target bundle size per component (KB)
    pub target_size_kb: f64,
}

impl BundleAnalyzer {
    /// Create new bundle analyzer
    pub fn new(components_path: PathBuf) -> Self {
        Self {
            components_path,
            target_size_kb: 5.0,
        }
    }
    
    /// Analyze all components
    pub async fn analyze_all_components(&self) -> BundleAnalysisResults {
        // This will be implemented in the Green phase
        todo!("Implement component bundle analysis")
    }
    
    /// Analyze single component
    pub async fn analyze_component(&self, _component_name: &str) -> ComponentBundleAnalysis {
        // This will be implemented in the Green phase
        todo!("Implement single component analysis")
    }
    
    /// Get component bundle size from build artifacts
    pub async fn get_component_bundle_size(&self, _component_name: &str) -> u64 {
        // This will be implemented in the Green phase
        todo!("Implement bundle size extraction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_bundle_analysis_creation() {
        let analysis = ComponentBundleAnalysis::new("button".to_string(), 2048); // 2KB
        
        assert_eq!(analysis.component_name, "button");
        assert_eq!(analysis.bundle_size_bytes, 2048);
        assert_eq!(analysis.bundle_size_kb, 2.0);
        assert!(analysis.meets_size_target);
    }

    #[test]
    fn test_component_bundle_analysis_oversized() {
        let analysis = ComponentBundleAnalysis::new("large-component".to_string(), 8192); // 8KB
        
        assert_eq!(analysis.bundle_size_kb, 8.0);
        assert!(!analysis.meets_size_target);
    }

    #[test]
    fn test_component_performance_score() {
        let small_analysis = ComponentBundleAnalysis::new("small".to_string(), 1024); // 1KB
        let large_analysis = ComponentBundleAnalysis::new("large".to_string(), 10240); // 10KB
        
        assert!(small_analysis.performance_score() > large_analysis.performance_score());
    }

    #[test]
    fn test_bundle_analysis_results_default() {
        let results = BundleAnalysisResults::default();
        
        assert_eq!(results.total_bundle_size_bytes, 0);
        assert_eq!(results.total_bundle_size_kb, 0.0);
        assert_eq!(results.average_component_size_kb, 0.0);
        assert!(results.oversized_components.is_empty());
        assert_eq!(results.overall_efficiency_score, 0.0);
    }

    #[test]
    fn test_bundle_analysis_results_add_component() {
        let mut results = BundleAnalysisResults::default();
        let analysis = ComponentBundleAnalysis::new("button".to_string(), 2048);
        
        results.add_component(analysis);
        
        assert_eq!(results.component_analyses.len(), 1);
        assert_eq!(results.total_bundle_size_bytes, 2048);
        assert_eq!(results.total_bundle_size_kb, 2.0);
        assert_eq!(results.average_component_size_kb, 2.0);
        assert_eq!(results.largest_component_size_kb, 2.0);
        assert!(results.oversized_components.is_empty());
    }

    #[test]
    fn test_bundle_analysis_results_multiple_components() {
        let mut results = BundleAnalysisResults::default();
        
        // Add small component
        results.add_component(ComponentBundleAnalysis::new("button".to_string(), 2048));
        // Add large component
        results.add_component(ComponentBundleAnalysis::new("large".to_string(), 8192));
        
        assert_eq!(results.component_analyses.len(), 2);
        assert_eq!(results.total_bundle_size_bytes, 10240);
        assert_eq!(results.total_bundle_size_kb, 10.0);
        assert_eq!(results.average_component_size_kb, 5.0);
        assert_eq!(results.largest_component_size_kb, 8.0);
        assert_eq!(results.oversized_components.len(), 1);
        assert_eq!(results.oversized_components[0], "large");
    }

    #[test]
    fn test_bundle_analysis_meets_targets() {
        let mut results = BundleAnalysisResults::default();
        
        // Add components that meet targets
        results.add_component(ComponentBundleAnalysis::new("button".to_string(), 2048));
        results.add_component(ComponentBundleAnalysis::new("input".to_string(), 1536));
        
        // Should meet targets if efficiency score is high enough
        // (This test will need to be updated when we implement the actual scoring)
        assert!(results.oversized_components.is_empty());
    }

    #[test]
    fn test_bundle_analysis_optimization_recommendations() {
        let mut results = BundleAnalysisResults::default();
        
        // Add oversized component
        results.add_component(ComponentBundleAnalysis::new("large".to_string(), 8192));
        
        let recommendations = results.get_optimization_recommendations();
        assert!(!recommendations.is_empty());
        assert!(recommendations[0].contains("large"));
    }

    #[test]
    fn test_bundle_analyzer_creation() {
        let analyzer = BundleAnalyzer::new(PathBuf::from("packages/leptos"));
        
        assert_eq!(analyzer.target_size_kb, 5.0);
        assert_eq!(analyzer.components_path, PathBuf::from("packages/leptos"));
    }
}
