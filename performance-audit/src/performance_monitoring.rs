//! Performance Monitoring Module
//! 
//! This module provides real-time performance monitoring for leptos-shadcn-ui components
//! using TDD principles to ensure optimal runtime performance.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

/// Performance metrics for a single component
#[derive(Debug, Clone)]
pub struct ComponentPerformanceMetrics {
    /// Component name
    pub component_name: String,
    /// Average render time in milliseconds
    pub average_render_time_ms: f64,
    /// Maximum render time in milliseconds
    pub max_render_time_ms: f64,
    /// Minimum render time in milliseconds
    pub min_render_time_ms: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Number of re-renders
    pub rerender_count: u64,
    /// Performance score (0-100)
    pub performance_score: f64,
    /// Meets performance targets
    pub meets_targets: bool,
}

impl ComponentPerformanceMetrics {
    /// Create new performance metrics
    pub fn new(component_name: String) -> Self {
        Self {
            component_name,
            average_render_time_ms: 0.0,
            max_render_time_ms: 0.0,
            min_render_time_ms: 0.0,
            memory_usage_bytes: 0,
            rerender_count: 0,
            performance_score: 0.0,
            meets_targets: false,
        }
    }
    
    /// Update render time metrics
    pub fn update_render_time(&mut self, render_time: Duration) {
        let render_time_ms = render_time.as_secs_f64() * 1000.0;
        
        if self.rerender_count == 0 {
            self.average_render_time_ms = render_time_ms;
            self.max_render_time_ms = render_time_ms;
            self.min_render_time_ms = render_time_ms;
        } else {
            self.average_render_time_ms = (self.average_render_time_ms * self.rerender_count as f64 + render_time_ms) 
                / (self.rerender_count + 1) as f64;
            self.max_render_time_ms = self.max_render_time_ms.max(render_time_ms);
            self.min_render_time_ms = self.min_render_time_ms.min(render_time_ms);
        }
        
        self.rerender_count += 1;
        self.calculate_performance_score();
    }
    
    /// Update memory usage
    pub fn update_memory_usage(&mut self, memory_bytes: u64) {
        self.memory_usage_bytes = memory_bytes;
        self.calculate_performance_score();
    }
    
    /// Calculate performance score based on metrics
    fn calculate_performance_score(&mut self) {
        let render_score = if self.average_render_time_ms <= 16.0 { 100.0 } else {
            (16.0 / self.average_render_time_ms * 100.0).min(100.0)
        };
        
        let memory_score = if self.memory_usage_bytes <= 1024 * 1024 { 100.0 } else { // 1MB
            (1024.0 * 1024.0 / self.memory_usage_bytes as f64 * 100.0).min(100.0)
        };
        
        self.performance_score = (render_score + memory_score) / 2.0;
        self.meets_targets = self.performance_score >= 80.0;
    }
}

/// Performance monitoring results
#[derive(Debug, Clone)]
pub struct PerformanceMonitoringResults {
    /// Individual component metrics
    pub component_metrics: BTreeMap<String, ComponentPerformanceMetrics>,
    /// Total monitoring duration
    pub monitoring_duration: Duration,
    /// Overall performance score
    pub overall_performance_score: f64,
    /// Components failing performance targets
    pub failing_components: Vec<String>,
    /// Performance bottlenecks identified
    pub performance_bottlenecks: Vec<PerformanceBottleneck>,
}

impl Default for PerformanceMonitoringResults {
    fn default() -> Self {
        Self {
            component_metrics: BTreeMap::new(),
            monitoring_duration: Duration::from_secs(0),
            overall_performance_score: 0.0,
            failing_components: Vec::new(),
            performance_bottlenecks: Vec::new(),
        }
    }
}

impl PerformanceMonitoringResults {
    /// Add component metrics
    pub fn add_component_metrics(&mut self, metrics: ComponentPerformanceMetrics) {
        let component_name = metrics.component_name.clone();
        self.component_metrics.insert(component_name.clone(), metrics);
        self.recalculate_overall_metrics();
    }
    
    /// Recalculate overall metrics
    fn recalculate_overall_metrics(&mut self) {
        if self.component_metrics.is_empty() {
            self.overall_performance_score = 0.0;
            self.failing_components.clear();
            return;
        }
        
        self.overall_performance_score = self.component_metrics
            .values()
            .map(|m| m.performance_score)
            .sum::<f64>() / self.component_metrics.len() as f64;
        
        self.failing_components = self.component_metrics
            .iter()
            .filter(|(_, metrics)| !metrics.meets_targets)
            .map(|(name, _)| name.clone())
            .collect();
        
        self.identify_bottlenecks();
    }
    
    /// Identify performance bottlenecks
    fn identify_bottlenecks(&mut self) {
        self.performance_bottlenecks.clear();
        
        for (name, metrics) in &self.component_metrics {
            if metrics.average_render_time_ms > 16.0 {
                self.performance_bottlenecks.push(PerformanceBottleneck {
                    component_name: name.clone(),
                    bottleneck_type: BottleneckType::RenderTime,
                    severity: if metrics.average_render_time_ms > 32.0 { 
                        BottleneckSeverity::High 
                    } else { 
                        BottleneckSeverity::Medium 
                    },
                    description: format!("Render time {}ms exceeds 16ms target", metrics.average_render_time_ms),
                });
            }
            
            if metrics.memory_usage_bytes > 1024 * 1024 { // 1MB
                self.performance_bottlenecks.push(PerformanceBottleneck {
                    component_name: name.clone(),
                    bottleneck_type: BottleneckType::MemoryUsage,
                    severity: if metrics.memory_usage_bytes > 5 * 1024 * 1024 { // 5MB
                        BottleneckSeverity::High 
                    } else { 
                        BottleneckSeverity::Medium 
                    },
                    description: format!("Memory usage {}MB exceeds 1MB target", 
                        metrics.memory_usage_bytes / (1024 * 1024)),
                });
            }
        }
    }
    
    /// Check if monitoring results meet targets
    pub fn meets_targets(&self) -> bool {
        self.overall_performance_score >= 80.0 && self.failing_components.is_empty()
    }
    
    /// Get performance recommendations
    pub fn get_performance_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !self.failing_components.is_empty() {
            recommendations.push(format!(
                "Optimize failing components: {}", 
                self.failing_components.join(", ")
            ));
        }
        
        for bottleneck in &self.performance_bottlenecks {
            match bottleneck.bottleneck_type {
                BottleneckType::RenderTime => {
                    recommendations.push(format!(
                        "Optimize render performance for {}: {}", 
                        bottleneck.component_name, 
                        bottleneck.description
                    ));
                }
                BottleneckType::MemoryUsage => {
                    recommendations.push(format!(
                        "Reduce memory usage for {}: {}", 
                        bottleneck.component_name, 
                        bottleneck.description
                    ));
                }
            }
        }
        
        recommendations
    }
}

/// Performance bottleneck types
#[derive(Debug, Clone)]
pub enum BottleneckType {
    RenderTime,
    MemoryUsage,
}

/// Bottleneck severity levels
#[derive(Debug, Clone)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
}

/// Performance bottleneck information
#[derive(Debug, Clone)]
pub struct PerformanceBottleneck {
    /// Component name
    pub component_name: String,
    /// Type of bottleneck
    pub bottleneck_type: BottleneckType,
    /// Severity level
    pub severity: BottleneckSeverity,
    /// Description of the bottleneck
    pub description: String,
}

/// Performance monitor for leptos-shadcn-ui components
pub struct PerformanceMonitor {
    /// Monitoring configuration
    pub config: PerformanceConfig,
    /// Start time of monitoring
    pub start_time: Option<Instant>,
    /// Component metrics being tracked
    pub tracked_components: BTreeMap<String, ComponentPerformanceMetrics>,
}

/// Performance monitoring configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Maximum render time target (ms)
    pub max_render_time_ms: f64,
    /// Maximum memory usage target (bytes)
    pub max_memory_usage_bytes: u64,
    /// Monitoring duration
    pub monitoring_duration: Duration,
    /// Sample rate (how often to collect metrics)
    pub sample_rate: Duration,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_render_time_ms: 16.0,
            max_memory_usage_bytes: 1024 * 1024, // 1MB
            monitoring_duration: Duration::from_secs(60), // 1 minute
            sample_rate: Duration::from_millis(100), // 100ms
        }
    }
}

impl PerformanceMonitor {
    /// Create new performance monitor
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            config,
            start_time: None,
            tracked_components: BTreeMap::new(),
        }
    }
    
    /// Start monitoring
    pub fn start_monitoring(&mut self) {
        self.start_time = Some(Instant::now());
        self.tracked_components.clear();
    }
    
    /// Stop monitoring and get results
    pub fn stop_monitoring(&mut self) -> PerformanceMonitoringResults {
        let monitoring_duration = self.start_time
            .map(|start| start.elapsed())
            .unwrap_or(Duration::from_secs(0));
        
        let mut results = PerformanceMonitoringResults {
            component_metrics: self.tracked_components.clone(),
            monitoring_duration,
            overall_performance_score: 0.0,
            failing_components: Vec::new(),
            performance_bottlenecks: Vec::new(),
        };
        
        results.recalculate_overall_metrics();
        
        // Clear the start time to indicate monitoring has stopped
        self.start_time = None;
        
        results
    }
    
    /// Record component render time
    pub fn record_render_time(&mut self, component_name: &str, render_time: Duration) {
        let metrics = self.tracked_components
            .entry(component_name.to_string())
            .or_insert_with(|| ComponentPerformanceMetrics::new(component_name.to_string()));
        
        metrics.update_render_time(render_time);
    }
    
    /// Record component memory usage
    pub fn record_memory_usage(&mut self, component_name: &str, memory_bytes: u64) {
        let metrics = self.tracked_components
            .entry(component_name.to_string())
            .or_insert_with(|| ComponentPerformanceMetrics::new(component_name.to_string()));
        
        metrics.update_memory_usage(memory_bytes);
    }
    
    /// Check if monitoring is active
    pub fn is_monitoring(&self) -> bool {
        self.start_time.is_some()
    }
    
    /// Get current monitoring duration
    pub fn get_monitoring_duration(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_performance_metrics_creation() {
        let metrics = ComponentPerformanceMetrics::new("button".to_string());
        
        assert_eq!(metrics.component_name, "button");
        assert_eq!(metrics.average_render_time_ms, 0.0);
        assert_eq!(metrics.rerender_count, 0);
        assert_eq!(metrics.performance_score, 0.0);
        assert!(!metrics.meets_targets);
    }

    #[test]
    fn test_component_performance_metrics_update_render_time() {
        let mut metrics = ComponentPerformanceMetrics::new("button".to_string());
        
        // First render
        metrics.update_render_time(Duration::from_millis(10));
        
        assert_eq!(metrics.average_render_time_ms, 10.0);
        assert_eq!(metrics.max_render_time_ms, 10.0);
        assert_eq!(metrics.min_render_time_ms, 10.0);
        assert_eq!(metrics.rerender_count, 1);
        assert!(metrics.meets_targets); // 10ms < 16ms target
        
        // Second render
        metrics.update_render_time(Duration::from_millis(20));
        
        assert_eq!(metrics.average_render_time_ms, 15.0);
        assert_eq!(metrics.max_render_time_ms, 20.0);
        assert_eq!(metrics.min_render_time_ms, 10.0);
        assert_eq!(metrics.rerender_count, 2);
    }

    #[test]
    fn test_component_performance_metrics_memory_usage() {
        let mut metrics = ComponentPerformanceMetrics::new("button".to_string());
        
        // Low memory usage
        metrics.update_memory_usage(512 * 1024); // 512KB
        
        assert_eq!(metrics.memory_usage_bytes, 512 * 1024);
        assert!(metrics.meets_targets); // < 1MB target
        
        // High memory usage
        metrics.update_memory_usage(2 * 1024 * 1024); // 2MB
        
        assert_eq!(metrics.memory_usage_bytes, 2 * 1024 * 1024);
        assert!(!metrics.meets_targets); // > 1MB target
    }

    #[test]
    fn test_performance_monitoring_results_default() {
        let results = PerformanceMonitoringResults::default();
        
        assert!(results.component_metrics.is_empty());
        assert_eq!(results.monitoring_duration, Duration::from_secs(0));
        assert_eq!(results.overall_performance_score, 0.0);
        assert!(results.failing_components.is_empty());
        assert!(results.performance_bottlenecks.is_empty());
    }

    #[test]
    fn test_performance_monitoring_results_add_metrics() {
        let mut results = PerformanceMonitoringResults::default();
        let mut metrics = ComponentPerformanceMetrics::new("button".to_string());
        metrics.update_render_time(Duration::from_millis(10));
        
        results.add_component_metrics(metrics);
        
        assert_eq!(results.component_metrics.len(), 1);
        assert!(results.failing_components.is_empty());
    }

    #[test]
    fn test_performance_monitoring_results_failing_component() {
        let mut results = PerformanceMonitoringResults::default();
        let mut metrics = ComponentPerformanceMetrics::new("slow-button".to_string());
        metrics.update_render_time(Duration::from_millis(50)); // Exceeds 16ms target
        
        results.add_component_metrics(metrics);
        
        assert_eq!(results.failing_components.len(), 1);
        assert_eq!(results.failing_components[0], "slow-button");
        assert!(!results.performance_bottlenecks.is_empty());
    }

    #[test]
    fn test_performance_monitoring_meets_targets() {
        let mut results = PerformanceMonitoringResults::default();
        let mut metrics = ComponentPerformanceMetrics::new("button".to_string());
        metrics.update_render_time(Duration::from_millis(10));
        
        results.add_component_metrics(metrics);
        
        assert!(results.meets_targets());
    }

    #[test]
    fn test_performance_monitor_creation() {
        let config = PerformanceConfig::default();
        let monitor = PerformanceMonitor::new(config);
        
        assert!(!monitor.is_monitoring());
        assert!(monitor.tracked_components.is_empty());
    }

    #[test]
    fn test_performance_monitor_start_stop() {
        let config = PerformanceConfig::default();
        let mut monitor = PerformanceMonitor::new(config);
        
        assert!(!monitor.is_monitoring());
        
        monitor.start_monitoring();
        assert!(monitor.is_monitoring());
        
        let results = monitor.stop_monitoring();
        assert!(!monitor.is_monitoring());
        assert!(results.monitoring_duration >= Duration::from_secs(0));
    }

    #[test]
    fn test_performance_monitor_record_metrics() {
        let config = PerformanceConfig::default();
        let mut monitor = PerformanceMonitor::new(config);
        
        monitor.start_monitoring();
        monitor.record_render_time("button", Duration::from_millis(10));
        monitor.record_memory_usage("button", 512 * 1024);
        
        let results = monitor.stop_monitoring();
        
        assert_eq!(results.component_metrics.len(), 1);
        let button_metrics = &results.component_metrics["button"];
        assert_eq!(button_metrics.average_render_time_ms, 10.0);
        assert_eq!(button_metrics.memory_usage_bytes, 512 * 1024);
    }

    #[test]
    fn test_performance_config_defaults() {
        let config = PerformanceConfig::default();
        
        assert_eq!(config.max_render_time_ms, 16.0);
        assert_eq!(config.max_memory_usage_bytes, 1024 * 1024);
        assert_eq!(config.monitoring_duration, Duration::from_secs(60));
        assert_eq!(config.sample_rate, Duration::from_millis(100));
    }
}
