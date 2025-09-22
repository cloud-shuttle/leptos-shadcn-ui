//! Optimization Roadmap Module
//! 
//! This module provides optimization recommendations and roadmap generation
//! for leptos-shadcn-ui components using TDD principles.

use std::collections::HashMap;

/// Optimization priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Optimization category
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptimizationCategory {
    BundleSize,
    RenderPerformance,
    MemoryUsage,
    Accessibility,
    DeveloperExperience,
    CodeQuality,
}

/// Individual optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Recommendation ID
    pub id: String,
    /// Component name (empty for global recommendations)
    pub component_name: String,
    /// Optimization category
    pub category: OptimizationCategory,
    /// Priority level
    pub priority: OptimizationPriority,
    /// Short description
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Expected impact (0-100)
    pub expected_impact: f64,
    /// Estimated effort (hours)
    pub estimated_effort_hours: f64,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
    /// Success criteria
    pub success_criteria: Vec<String>,
}

impl OptimizationRecommendation {
    /// Create new optimization recommendation
    pub fn new(
        id: String,
        component_name: String,
        category: OptimizationCategory,
        priority: OptimizationPriority,
        title: String,
        description: String,
    ) -> Self {
        Self {
            id,
            component_name,
            category,
            priority,
            title,
            description,
            expected_impact: 0.0,
            estimated_effort_hours: 0.0,
            implementation_steps: Vec::new(),
            success_criteria: Vec::new(),
        }
    }
    
    /// Set expected impact
    pub fn with_impact(mut self, impact: f64) -> Self {
        self.expected_impact = impact;
        self
    }
    
    /// Set estimated effort
    pub fn with_effort(mut self, hours: f64) -> Self {
        self.estimated_effort_hours = hours;
        self
    }
    
    /// Add implementation step
    pub fn add_implementation_step(mut self, step: String) -> Self {
        self.implementation_steps.push(step);
        self
    }
    
    /// Add success criteria
    pub fn add_success_criteria(mut self, criteria: String) -> Self {
        self.success_criteria.push(criteria);
        self
    }
    
    /// Calculate ROI (Return on Investment)
    pub fn calculate_roi(&self) -> f64 {
        if self.estimated_effort_hours == 0.0 {
            return 0.0;
        }
        self.expected_impact / self.estimated_effort_hours
    }
    
    /// Check if recommendation is high priority
    pub fn is_high_priority(&self) -> bool {
        matches!(self.priority, OptimizationPriority::High | OptimizationPriority::Critical)
    }
}

/// Optimization roadmap
#[derive(Debug, Clone)]
pub struct OptimizationRoadmap {
    /// All optimization recommendations
    pub recommendations: HashMap<String, OptimizationRecommendation>,
    /// Recommendations grouped by priority
    pub recommendations_by_priority: HashMap<OptimizationPriority, Vec<String>>,
    /// Recommendations grouped by category
    pub recommendations_by_category: HashMap<OptimizationCategory, Vec<String>>,
    /// Total estimated effort (hours)
    pub total_estimated_effort_hours: f64,
    /// Overall expected impact
    pub overall_expected_impact: f64,
    /// Roadmap completion percentage
    pub completion_percentage: f64,
}

impl Default for OptimizationRoadmap {
    fn default() -> Self {
        Self {
            recommendations: HashMap::new(),
            recommendations_by_priority: HashMap::new(),
            recommendations_by_category: HashMap::new(),
            total_estimated_effort_hours: 0.0,
            overall_expected_impact: 0.0,
            completion_percentage: 0.0,
        }
    }
}

impl OptimizationRoadmap {
    /// Add optimization recommendation
    pub fn add_recommendation(&mut self, recommendation: OptimizationRecommendation) {
        let id = recommendation.id.clone();
        let priority = recommendation.priority.clone();
        let category = recommendation.category.clone();
        
        // Add to main recommendations
        self.recommendations.insert(id.clone(), recommendation);
        
        // Add to priority groups
        self.recommendations_by_priority
            .entry(priority)
            .or_default()
            .push(id.clone());
        
        // Add to category groups
        self.recommendations_by_category
            .entry(category)
            .or_default()
            .push(id);
        
        self.recalculate_totals();
    }
    
    /// Recalculate totals and statistics
    fn recalculate_totals(&mut self) {
        self.total_estimated_effort_hours = self.recommendations
            .values()
            .map(|r| r.estimated_effort_hours)
            .sum();
        
        self.overall_expected_impact = self.recommendations
            .values()
            .map(|r| r.expected_impact)
            .sum();
        
        // Calculate completion percentage (placeholder - would need actual completion tracking)
        self.completion_percentage = 0.0;
    }
    
    /// Get recommendations by priority
    pub fn get_recommendations_by_priority(&self, priority: OptimizationPriority) -> Vec<&OptimizationRecommendation> {
        self.recommendations_by_priority
            .get(&priority)
            .map(|ids| ids.iter().filter_map(|id| self.recommendations.get(id)).collect())
            .unwrap_or_default()
    }
    
    /// Get recommendations by category
    pub fn get_recommendations_by_category(&self, category: &OptimizationCategory) -> Vec<&OptimizationRecommendation> {
        self.recommendations_by_category
            .get(category)
            .map(|ids| ids.iter().filter_map(|id| self.recommendations.get(id)).collect())
            .unwrap_or_default()
    }
    
    /// Get high priority recommendations
    pub fn get_high_priority_recommendations(&self) -> Vec<&OptimizationRecommendation> {
        let mut high_priority = Vec::new();
        high_priority.extend(self.get_recommendations_by_priority(OptimizationPriority::Critical));
        high_priority.extend(self.get_recommendations_by_priority(OptimizationPriority::High));
        high_priority
    }
    
    /// Get recommendations sorted by ROI
    pub fn get_recommendations_by_roi(&self) -> Vec<&OptimizationRecommendation> {
        let mut recommendations: Vec<&OptimizationRecommendation> = self.recommendations.values().collect();
        recommendations.sort_by(|a, b| b.calculate_roi().partial_cmp(&a.calculate_roi()).unwrap());
        recommendations
    }
    
    /// Get next recommended action
    pub fn get_next_recommended_action(&self) -> Option<&OptimizationRecommendation> {
        self.get_recommendations_by_roi()
            .into_iter()
            .find(|r| r.is_high_priority())
    }
    
    /// Generate implementation plan
    pub fn generate_implementation_plan(&self) -> ImplementationPlan {
        let mut plan = ImplementationPlan::new();
        
        // Add critical recommendations first
        for rec in self.get_recommendations_by_priority(OptimizationPriority::Critical) {
            plan.add_phase("Critical Fixes", rec.clone());
        }
        
        // Add high priority recommendations
        for rec in self.get_recommendations_by_priority(OptimizationPriority::High) {
            plan.add_phase("High Priority", rec.clone());
        }
        
        // Add medium priority recommendations
        for rec in self.get_recommendations_by_priority(OptimizationPriority::Medium) {
            plan.add_phase("Medium Priority", rec.clone());
        }
        
        // Add low priority recommendations
        for rec in self.get_recommendations_by_priority(OptimizationPriority::Low) {
            plan.add_phase("Low Priority", rec.clone());
        }
        
        plan
    }
}

/// Implementation plan with phases
#[derive(Debug, Clone)]
pub struct ImplementationPlan {
    /// Implementation phases
    pub phases: Vec<ImplementationPhase>,
    /// Total estimated effort
    pub total_effort_hours: f64,
    /// Total expected impact
    pub total_expected_impact: f64,
}

/// Implementation phase
#[derive(Debug, Clone)]
pub struct ImplementationPhase {
    /// Phase name
    pub name: String,
    /// Recommendations in this phase
    pub recommendations: Vec<OptimizationRecommendation>,
    /// Phase effort estimate
    pub effort_hours: f64,
    /// Phase expected impact
    pub expected_impact: f64,
}

impl Default for ImplementationPlan {
    fn default() -> Self {
        Self::new()
    }
}

impl ImplementationPlan {
    /// Create new implementation plan
    pub fn new() -> Self {
        Self {
            phases: Vec::new(),
            total_effort_hours: 0.0,
            total_expected_impact: 0.0,
        }
    }
    
    /// Add recommendation to a phase
    pub fn add_phase(&mut self, phase_name: &str, recommendation: OptimizationRecommendation) {
        // Find existing phase or create new one
        let phase_index = self.phases
            .iter()
            .position(|p| p.name == phase_name);
        
        if let Some(index) = phase_index {
            self.phases[index].recommendations.push(recommendation);
        } else {
            self.phases.push(ImplementationPhase {
                name: phase_name.to_string(),
                recommendations: vec![recommendation],
                effort_hours: 0.0,
                expected_impact: 0.0,
            });
        }
        
        self.recalculate_totals();
    }
    
    /// Recalculate totals
    fn recalculate_totals(&mut self) {
        for phase in &mut self.phases {
            phase.effort_hours = phase.recommendations
                .iter()
                .map(|r| r.estimated_effort_hours)
                .sum();
            phase.expected_impact = phase.recommendations
                .iter()
                .map(|r| r.expected_impact)
                .sum();
        }
        
        self.total_effort_hours = self.phases
            .iter()
            .map(|p| p.effort_hours)
            .sum();
        self.total_expected_impact = self.phases
            .iter()
            .map(|p| p.expected_impact)
            .sum();
    }
}

/// Optimization roadmap generator
pub struct OptimizationRoadmapGenerator;

impl OptimizationRoadmapGenerator {
    /// Generate optimization roadmap from performance results
    pub fn generate_roadmap(
        bundle_results: &crate::bundle_analysis::BundleAnalysisResults,
        performance_results: &crate::performance_monitoring::PerformanceMonitoringResults,
    ) -> OptimizationRoadmap {
        let mut roadmap = OptimizationRoadmap::default();
        
        // Handle empty data case - return empty roadmap
        if bundle_results.component_analyses.is_empty() && performance_results.component_metrics.is_empty() {
            return roadmap;
        }
        
        // Generate bundle size optimizations
        Self::add_bundle_size_optimizations(&mut roadmap, bundle_results);
        
        // Generate performance optimizations
        Self::add_performance_optimizations(&mut roadmap, performance_results);
        
        // Generate general optimizations
        Self::add_general_optimizations(&mut roadmap);
        
        roadmap
    }
    
    /// Add bundle size optimization recommendations
    fn add_bundle_size_optimizations(
        roadmap: &mut OptimizationRoadmap,
        bundle_results: &crate::bundle_analysis::BundleAnalysisResults,
    ) {
        // Add recommendations for oversized components
        for component_name in &bundle_results.oversized_components {
            let recommendation = OptimizationRecommendation::new(
                format!("bundle-size-{}", component_name),
                component_name.clone(),
                OptimizationCategory::BundleSize,
                OptimizationPriority::High,
                format!("Optimize bundle size for {}", component_name),
                format!("Component {} exceeds 5KB target with {:.1}KB bundle size", 
                    component_name, 
                    bundle_results.component_analyses[component_name].bundle_size_kb),
            )
            .with_impact(80.0)
            .with_effort(4.0)
            .add_implementation_step("Analyze component dependencies".to_string())
            .add_implementation_step("Implement code splitting".to_string())
            .add_implementation_step("Optimize imports and exports".to_string())
            .add_success_criteria("Bundle size < 5KB".to_string())
            .add_success_criteria("No performance regression".to_string());
            
            roadmap.add_recommendation(recommendation);
        }
    }
    
    /// Add performance optimization recommendations
    fn add_performance_optimizations(
        roadmap: &mut OptimizationRoadmap,
        performance_results: &crate::performance_monitoring::PerformanceMonitoringResults,
    ) {
        // Add recommendations for failing components
        for component_name in &performance_results.failing_components {
            let recommendation = OptimizationRecommendation::new(
                format!("performance-{}", component_name),
                component_name.clone(),
                OptimizationCategory::RenderPerformance,
                OptimizationPriority::High,
                format!("Optimize render performance for {}", component_name),
                format!("Component {} fails performance targets with {:.1}ms render time", 
                    component_name, 
                    performance_results.component_metrics[component_name].average_render_time_ms),
            )
            .with_impact(90.0)
            .with_effort(6.0)
            .add_implementation_step("Profile component render cycle".to_string())
            .add_implementation_step("Optimize reactive updates".to_string())
            .add_implementation_step("Implement memoization".to_string())
            .add_success_criteria("Render time < 16ms".to_string())
            .add_success_criteria("No memory leaks".to_string());
            
            roadmap.add_recommendation(recommendation);
        }
    }
    
    /// Add general optimization recommendations
    fn add_general_optimizations(roadmap: &mut OptimizationRoadmap) {
        // Add general recommendations
        let general_recommendations = vec![
            OptimizationRecommendation::new(
                "general-accessibility".to_string(),
                "".to_string(),
                OptimizationCategory::Accessibility,
                OptimizationPriority::Medium,
                "Enhance accessibility compliance".to_string(),
                "Improve WCAG 2.1 AAA compliance across all components".to_string(),
            )
            .with_impact(70.0)
            .with_effort(8.0)
            .add_implementation_step("Audit current accessibility".to_string())
            .add_implementation_step("Implement ARIA improvements".to_string())
            .add_success_criteria("WCAG 2.1 AAA compliance".to_string()),
            
            OptimizationRecommendation::new(
                "general-documentation".to_string(),
                "".to_string(),
                OptimizationCategory::DeveloperExperience,
                OptimizationPriority::Low,
                "Enhance developer documentation".to_string(),
                "Improve component documentation and examples".to_string(),
            )
            .with_impact(60.0)
            .with_effort(12.0)
            .add_implementation_step("Create interactive examples".to_string())
            .add_implementation_step("Add performance best practices".to_string())
            .add_success_criteria("Comprehensive documentation".to_string()),
        ];
        
        for recommendation in general_recommendations {
            roadmap.add_recommendation(recommendation);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_recommendation_creation() {
        let rec = OptimizationRecommendation::new(
            "test-1".to_string(),
            "button".to_string(),
            OptimizationCategory::BundleSize,
            OptimizationPriority::High,
            "Test optimization".to_string(),
            "Test description".to_string(),
        );
        
        assert_eq!(rec.id, "test-1");
        assert_eq!(rec.component_name, "button");
        assert_eq!(rec.priority, OptimizationPriority::High);
        assert_eq!(rec.expected_impact, 0.0);
        assert_eq!(rec.estimated_effort_hours, 0.0);
    }

    #[test]
    fn test_optimization_recommendation_builder() {
        let rec = OptimizationRecommendation::new(
            "test-2".to_string(),
            "input".to_string(),
            OptimizationCategory::RenderPerformance,
            OptimizationPriority::Critical,
            "Critical fix".to_string(),
            "Critical description".to_string(),
        )
        .with_impact(95.0)
        .with_effort(2.0)
        .add_implementation_step("Step 1".to_string())
        .add_success_criteria("Success 1".to_string());
        
        assert_eq!(rec.expected_impact, 95.0);
        assert_eq!(rec.estimated_effort_hours, 2.0);
        assert_eq!(rec.implementation_steps.len(), 1);
        assert_eq!(rec.success_criteria.len(), 1);
        assert!(rec.is_high_priority());
    }

    #[test]
    fn test_optimization_recommendation_roi() {
        let rec = OptimizationRecommendation::new(
            "test-3".to_string(),
            "card".to_string(),
            OptimizationCategory::MemoryUsage,
            OptimizationPriority::Medium,
            "Memory optimization".to_string(),
            "Memory description".to_string(),
        )
        .with_impact(80.0)
        .with_effort(4.0);
        
        assert_eq!(rec.calculate_roi(), 20.0); // 80.0 / 4.0
    }

    #[test]
    fn test_optimization_roadmap_default() {
        let roadmap = OptimizationRoadmap::default();
        
        assert!(roadmap.recommendations.is_empty());
        assert_eq!(roadmap.total_estimated_effort_hours, 0.0);
        assert_eq!(roadmap.overall_expected_impact, 0.0);
        assert_eq!(roadmap.completion_percentage, 0.0);
    }

    #[test]
    fn test_optimization_roadmap_add_recommendation() {
        let mut roadmap = OptimizationRoadmap::default();
        let rec = OptimizationRecommendation::new(
            "test-4".to_string(),
            "button".to_string(),
            OptimizationCategory::BundleSize,
            OptimizationPriority::High,
            "Test optimization".to_string(),
            "Test description".to_string(),
        )
        .with_impact(80.0)
        .with_effort(4.0);
        
        roadmap.add_recommendation(rec);
        
        assert_eq!(roadmap.recommendations.len(), 1);
        assert_eq!(roadmap.total_estimated_effort_hours, 4.0);
        assert_eq!(roadmap.overall_expected_impact, 80.0);
        assert_eq!(roadmap.get_recommendations_by_priority(OptimizationPriority::High).len(), 1);
    }

    #[test]
    fn test_optimization_roadmap_high_priority() {
        let mut roadmap = OptimizationRoadmap::default();
        
        // Add high priority recommendation
        roadmap.add_recommendation(OptimizationRecommendation::new(
            "high-1".to_string(),
            "button".to_string(),
            OptimizationCategory::BundleSize,
            OptimizationPriority::High,
            "High priority".to_string(),
            "High description".to_string(),
        ));
        
        // Add low priority recommendation
        roadmap.add_recommendation(OptimizationRecommendation::new(
            "low-1".to_string(),
            "input".to_string(),
            OptimizationCategory::DeveloperExperience,
            OptimizationPriority::Low,
            "Low priority".to_string(),
            "Low description".to_string(),
        ));
        
        let high_priority = roadmap.get_high_priority_recommendations();
        assert_eq!(high_priority.len(), 1);
        assert_eq!(high_priority[0].id, "high-1");
    }

    #[test]
    fn test_optimization_roadmap_by_roi() {
        let mut roadmap = OptimizationRoadmap::default();
        
        // Add recommendation with high ROI
        roadmap.add_recommendation(OptimizationRecommendation::new(
            "high-roi".to_string(),
            "button".to_string(),
            OptimizationCategory::BundleSize,
            OptimizationPriority::High,
            "High ROI".to_string(),
            "High ROI description".to_string(),
        )
        .with_impact(80.0)
        .with_effort(2.0)); // ROI = 40.0
        
        // Add recommendation with low ROI
        roadmap.add_recommendation(OptimizationRecommendation::new(
            "low-roi".to_string(),
            "input".to_string(),
            OptimizationCategory::RenderPerformance,
            OptimizationPriority::Medium,
            "Low ROI".to_string(),
            "Low ROI description".to_string(),
        )
        .with_impact(40.0)
        .with_effort(4.0)); // ROI = 10.0
        
        let by_roi = roadmap.get_recommendations_by_roi();
        assert_eq!(by_roi.len(), 2);
        assert_eq!(by_roi[0].id, "high-roi"); // Higher ROI first
        assert_eq!(by_roi[1].id, "low-roi");
    }

    #[test]
    fn test_implementation_plan_creation() {
        let plan = ImplementationPlan::new();
        
        assert!(plan.phases.is_empty());
        assert_eq!(plan.total_effort_hours, 0.0);
        assert_eq!(plan.total_expected_impact, 0.0);
    }

    #[test]
    fn test_implementation_plan_add_phase() {
        let mut plan = ImplementationPlan::new();
        let rec = OptimizationRecommendation::new(
            "test-5".to_string(),
            "button".to_string(),
            OptimizationCategory::BundleSize,
            OptimizationPriority::High,
            "Test optimization".to_string(),
            "Test description".to_string(),
        )
        .with_impact(80.0)
        .with_effort(4.0);
        
        plan.add_phase("Phase 1", rec);
        
        assert_eq!(plan.phases.len(), 1);
        assert_eq!(plan.phases[0].name, "Phase 1");
        assert_eq!(plan.phases[0].recommendations.len(), 1);
        assert_eq!(plan.total_effort_hours, 4.0);
        assert_eq!(plan.total_expected_impact, 80.0);
    }
}
