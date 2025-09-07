//! # leptos-shadcn Performance Testing Suite
//!
//! Comprehensive performance regression testing system for leptos-shadcn-ui components.
//! Provides automated benchmarking, regression detection, and performance reporting.

use std::path::PathBuf;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod benchmarks;
pub mod regression;
pub mod reporting;
pub mod system_info;

/// Performance test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfTestConfig {
    pub components_dir: PathBuf,
    pub output_dir: PathBuf,
    pub baseline_dir: PathBuf,
    pub thresholds: PerformanceThresholds,
    pub test_iterations: u32,
    pub warmup_iterations: u32,
    pub enable_regression_detection: bool,
    pub enable_memory_profiling: bool,
    pub enable_bundle_analysis: bool,
}

impl Default for PerfTestConfig {
    fn default() -> Self {
        Self {
            components_dir: PathBuf::from("packages/leptos"),
            output_dir: PathBuf::from("performance-results"),
            baseline_dir: PathBuf::from("performance-baselines"),
            thresholds: PerformanceThresholds::default(),
            test_iterations: 1000,
            warmup_iterations: 100,
            enable_regression_detection: true,
            enable_memory_profiling: true,
            enable_bundle_analysis: true,
        }
    }
}

/// Performance thresholds for regression detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable render time in milliseconds
    pub max_render_time_ms: f64,
    /// Maximum acceptable memory usage in MB
    pub max_memory_usage_mb: f64,
    /// Maximum acceptable bundle size in KB
    pub max_bundle_size_kb: f64,
    /// Maximum acceptable regression percentage (e.g., 5.0 for 5%)
    pub max_regression_percent: f64,
    /// Minimum iterations for statistical significance
    pub min_iterations: u32,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_render_time_ms: 16.0, // 60 FPS target
            max_memory_usage_mb: 1.0,  // 1MB per component
            max_bundle_size_kb: 10.0,  // 10KB per component
            max_regression_percent: 5.0, // 5% regression threshold
            min_iterations: 100,
        }
    }
}

/// Performance measurement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMeasurement {
    pub component_name: String,
    pub test_name: String,
    pub render_time_ms: StatisticalData,
    pub memory_usage_mb: Option<f64>,
    pub bundle_size_kb: Option<f64>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub system_info: SystemInfo,
    pub iterations: u32,
}

/// Statistical data for performance measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalData {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub p95: f64,
    pub p99: f64,
}

impl StatisticalData {
    /// Create statistical data from a vector of measurements
    pub fn from_measurements(measurements: &[f64]) -> Self {
        let mut sorted = measurements.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = sorted.iter().sum::<f64>() / sorted.len() as f64;
        let median = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };

        let variance = sorted
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / sorted.len() as f64;
        let std_dev = variance.sqrt();

        let p95_idx = ((sorted.len() as f64 * 0.95) as usize).min(sorted.len() - 1);
        let p99_idx = ((sorted.len() as f64 * 0.99) as usize).min(sorted.len() - 1);

        Self {
            mean,
            median,
            std_dev,
            min: sorted[0],
            max: sorted[sorted.len() - 1],
            p95: sorted[p95_idx],
            p99: sorted[p99_idx],
        }
    }
}

/// System information for performance measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub memory_total_mb: u64,
    pub rust_version: String,
    pub leptos_version: String,
}

/// Performance regression detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionResult {
    pub component_name: String,
    pub test_name: String,
    pub has_regression: bool,
    pub regression_percent: f64,
    pub current_value: f64,
    pub baseline_value: f64,
    pub severity: RegressionSeverity,
    pub recommendation: String,
}

/// Severity of performance regression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegressionSeverity {
    None,     // No regression detected
    Minor,    // 0-5% regression
    Moderate, // 5-15% regression
    Major,    // 15-30% regression
    Critical, // >30% regression
}

impl RegressionSeverity {
    pub fn from_percent(percent: f64) -> Self {
        if percent <= 0.0 {
            Self::None
        } else if percent <= 5.0 {
            Self::Minor
        } else if percent <= 15.0 {
            Self::Moderate
        } else if percent <= 30.0 {
            Self::Major
        } else {
            Self::Critical
        }
    }
}

/// Main performance testing suite
pub struct PerformanceTestSuite {
    config: PerfTestConfig,
    system_info: SystemInfo,
}

impl PerformanceTestSuite {
    /// Create a new performance test suite
    pub fn new(config: PerfTestConfig) -> Result<Self, PerfTestError> {
        let system_info = system_info::gather_system_info()?;
        
        // Create output directories
        std::fs::create_dir_all(&config.output_dir)?;
        std::fs::create_dir_all(&config.baseline_dir)?;
        
        Ok(Self {
            config,
            system_info,
        })
    }

    /// Run complete performance test suite
    pub async fn run_complete_suite(&self) -> Result<PerformanceReport, PerfTestError> {
        log::info!("Starting complete performance test suite");
        
        let mut measurements = Vec::new();
        let mut regressions = Vec::new();

        // Discover and test all components
        let components = self.discover_components().await?;
        log::info!("Found {} components to test", components.len());

        for component in &components {
            // Run performance benchmarks
            let component_measurements = self.benchmark_component(component).await?;
            
            // Check for regressions if enabled
            if self.config.enable_regression_detection {
                for measurement in &component_measurements {
                    if let Ok(regression) = self.check_regression(measurement).await {
                        if regression.has_regression {
                            regressions.push(regression);
                        }
                    }
                }
            }
            
            measurements.extend(component_measurements);
        }

        // Generate comprehensive report
        let report = PerformanceReport {
            measurements,
            regressions,
            system_info: self.system_info.clone(),
            config: self.config.clone(),
            timestamp: chrono::Utc::now(),
            summary: self.generate_summary(&measurements, &regressions),
        };

        // Save report to disk
        self.save_report(&report).await?;
        
        log::info!("Performance test suite completed successfully");
        Ok(report)
    }

    /// Discover all components in the source directory
    async fn discover_components(&self) -> Result<Vec<String>, PerfTestError> {
        let mut components = Vec::new();
        
        for entry in walkdir::WalkDir::new(&self.config.components_dir) {
            let entry = entry.map_err(PerfTestError::FileSystem)?;
            
            if entry.file_type().is_dir() {
                let dir_name = entry.file_name().to_string_lossy();
                if !dir_name.starts_with('.') && entry.path() != self.config.components_dir {
                    components.push(dir_name.to_string());
                }
            }
        }
        
        Ok(components)
    }

    /// Benchmark a specific component
    async fn benchmark_component(&self, component: &str) -> Result<Vec<PerformanceMeasurement>, PerfTestError> {
        log::info!("Benchmarking component: {}", component);
        
        // This would be replaced with actual component rendering and measurement
        // For now, we'll simulate measurements
        let mut measurements = Vec::new();
        
        let test_cases = vec!["basic_render", "with_props", "with_events", "complex_children"];
        
        for test_case in test_cases {
            let render_times = self.measure_render_performance(component, test_case).await?;
            let memory_usage = if self.config.enable_memory_profiling {
                Some(self.measure_memory_usage(component, test_case).await?)
            } else {
                None
            };
            let bundle_size = if self.config.enable_bundle_analysis {
                Some(self.measure_bundle_size(component).await?)
            } else {
                None
            };

            measurements.push(PerformanceMeasurement {
                component_name: component.to_string(),
                test_name: test_case.to_string(),
                render_time_ms: StatisticalData::from_measurements(&render_times),
                memory_usage_mb: memory_usage,
                bundle_size_kb: bundle_size,
                timestamp: chrono::Utc::now(),
                system_info: self.system_info.clone(),
                iterations: self.config.test_iterations,
            });
        }
        
        Ok(measurements)
    }

    /// Measure render performance for a component
    async fn measure_render_performance(&self, _component: &str, _test_case: &str) -> Result<Vec<f64>, PerfTestError> {
        let mut measurements = Vec::new();
        
        // Warmup iterations
        for _ in 0..self.config.warmup_iterations {
            let _ = self.simulate_render().await;
        }
        
        // Actual measurements
        for _ in 0..self.config.test_iterations {
            let start = instant::Instant::now();
            let _ = self.simulate_render().await;
            let duration = start.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
            measurements.push(duration);
        }
        
        Ok(measurements)
    }

    /// Simulate component rendering (placeholder)
    async fn simulate_render(&self) -> Result<(), PerfTestError> {
        // In a real implementation, this would:
        // 1. Create a component instance
        // 2. Render it to a virtual DOM or string
        // 3. Measure the time taken
        
        // For now, simulate some work with a small delay
        tokio::time::sleep(Duration::from_micros(10)).await;
        Ok(())
    }

    /// Measure memory usage for a component
    async fn measure_memory_usage(&self, _component: &str, _test_case: &str) -> Result<f64, PerfTestError> {
        // Placeholder: In a real implementation, this would measure actual memory usage
        Ok(0.5) // 0.5 MB placeholder
    }

    /// Measure bundle size for a component
    async fn measure_bundle_size(&self, _component: &str) -> Result<f64, PerfTestError> {
        // Placeholder: In a real implementation, this would analyze the compiled bundle
        Ok(5.0) // 5 KB placeholder
    }

    /// Check for performance regression
    async fn check_regression(&self, measurement: &PerformanceMeasurement) -> Result<RegressionResult, PerfTestError> {
        let baseline = self.load_baseline(measurement).await?;
        
        let regression_percent = if baseline.render_time_ms.mean > 0.0 {
            ((measurement.render_time_ms.mean - baseline.render_time_ms.mean) / baseline.render_time_ms.mean) * 100.0
        } else {
            0.0
        };
        
        let has_regression = regression_percent > self.config.thresholds.max_regression_percent;
        let severity = RegressionSeverity::from_percent(regression_percent);
        
        let recommendation = match severity {
            RegressionSeverity::None => "No action needed".to_string(),
            RegressionSeverity::Minor => "Consider optimization if trend continues".to_string(),
            RegressionSeverity::Moderate => "Investigate performance degradation".to_string(),
            RegressionSeverity::Major => "Immediate performance review required".to_string(),
            RegressionSeverity::Critical => "Critical performance regression - block deployment".to_string(),
        };
        
        Ok(RegressionResult {
            component_name: measurement.component_name.clone(),
            test_name: measurement.test_name.clone(),
            has_regression,
            regression_percent,
            current_value: measurement.render_time_ms.mean,
            baseline_value: baseline.render_time_ms.mean,
            severity,
            recommendation,
        })
    }

    /// Load baseline performance data
    async fn load_baseline(&self, measurement: &PerformanceMeasurement) -> Result<PerformanceMeasurement, PerfTestError> {
        let baseline_file = self.config.baseline_dir.join(format!(
            "{}_{}_baseline.json",
            measurement.component_name,
            measurement.test_name
        ));
        
        if baseline_file.exists() {
            let content = tokio::fs::read_to_string(&baseline_file).await?;
            let baseline: PerformanceMeasurement = serde_json::from_str(&content)?;
            Ok(baseline)
        } else {
            // No baseline exists, use current measurement as baseline
            self.save_baseline(measurement).await?;
            Ok(measurement.clone())
        }
    }

    /// Save baseline performance data
    async fn save_baseline(&self, measurement: &PerformanceMeasurement) -> Result<(), PerfTestError> {
        let baseline_file = self.config.baseline_dir.join(format!(
            "{}_{}_baseline.json",
            measurement.component_name,
            measurement.test_name
        ));
        
        let content = serde_json::to_string_pretty(measurement)?;
        tokio::fs::write(&baseline_file, content).await?;
        Ok(())
    }

    /// Generate performance summary
    fn generate_summary(&self, measurements: &[PerformanceMeasurement], regressions: &[RegressionResult]) -> PerformanceSummary {
        let total_components = measurements.iter()
            .map(|m| m.component_name.clone())
            .collect::<std::collections::HashSet<_>>()
            .len();

        let avg_render_time = measurements.iter()
            .map(|m| m.render_time_ms.mean)
            .sum::<f64>() / measurements.len() as f64;

        let components_exceeding_threshold = measurements.iter()
            .filter(|m| m.render_time_ms.mean > self.config.thresholds.max_render_time_ms)
            .count();

        let critical_regressions = regressions.iter()
            .filter(|r| r.severity == RegressionSeverity::Critical)
            .count();

        PerformanceSummary {
            total_components,
            total_measurements: measurements.len(),
            avg_render_time_ms: avg_render_time,
            components_exceeding_threshold,
            total_regressions: regressions.len(),
            critical_regressions,
            overall_health: if critical_regressions > 0 {
                HealthStatus::Critical
            } else if regressions.len() > total_components / 2 {
                HealthStatus::Warning
            } else {
                HealthStatus::Good
            },
        }
    }

    /// Save performance report to disk
    async fn save_report(&self, report: &PerformanceReport) -> Result<(), PerfTestError> {
        let report_file = self.config.output_dir.join(format!(
            "performance_report_{}.json",
            report.timestamp.format("%Y%m%d_%H%M%S")
        ));
        
        let content = serde_json::to_string_pretty(report)?;
        tokio::fs::write(&report_file, content).await?;
        
        // Also save as latest report
        let latest_file = self.config.output_dir.join("latest_performance_report.json");
        tokio::fs::write(&latest_file, &content).await?;
        
        Ok(())
    }
}

/// Complete performance test report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub measurements: Vec<PerformanceMeasurement>,
    pub regressions: Vec<RegressionResult>,
    pub system_info: SystemInfo,
    pub config: PerfTestConfig,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub summary: PerformanceSummary,
}

/// Performance summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub total_components: usize,
    pub total_measurements: usize,
    pub avg_render_time_ms: f64,
    pub components_exceeding_threshold: usize,
    pub total_regressions: usize,
    pub critical_regressions: usize,
    pub overall_health: HealthStatus,
}

/// Overall performance health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Good,
    Warning,
    Critical,
}

/// Performance testing errors
#[derive(Debug, thiserror::Error)]
pub enum PerfTestError {
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("System info error: {0}")]
    SystemInfo(String),
    
    #[error("Benchmark error: {0}")]
    Benchmark(String),
    
    #[error("Walk directory error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_statistical_data_calculation() {
        let measurements = vec![10.0, 12.0, 8.0, 15.0, 11.0, 9.0, 13.0, 14.0, 10.0, 11.0];
        let stats = StatisticalData::from_measurements(&measurements);
        
        assert!((stats.mean - 11.3).abs() < 0.1);
        assert!((stats.median - 11.0).abs() < 0.1);
        assert!(stats.min == 8.0);
        assert!(stats.max == 15.0);
    }

    #[test]
    fn test_regression_severity() {
        assert_eq!(RegressionSeverity::from_percent(0.0), RegressionSeverity::None);
        assert_eq!(RegressionSeverity::from_percent(3.0), RegressionSeverity::Minor);
        assert_eq!(RegressionSeverity::from_percent(10.0), RegressionSeverity::Moderate);
        assert_eq!(RegressionSeverity::from_percent(20.0), RegressionSeverity::Major);
        assert_eq!(RegressionSeverity::from_percent(40.0), RegressionSeverity::Critical);
    }

    #[tokio::test]
    async fn test_performance_test_suite_creation() {
        let temp_dir = tempdir().unwrap();
        let config = PerfTestConfig {
            output_dir: temp_dir.path().join("output"),
            baseline_dir: temp_dir.path().join("baselines"),
            ..Default::default()
        };

        let suite = PerformanceTestSuite::new(config);
        assert!(suite.is_ok());
    }

    #[test]
    fn test_performance_thresholds_default() {
        let thresholds = PerformanceThresholds::default();
        
        assert_eq!(thresholds.max_render_time_ms, 16.0);
        assert_eq!(thresholds.max_memory_usage_mb, 1.0);
        assert_eq!(thresholds.max_bundle_size_kb, 10.0);
        assert_eq!(thresholds.max_regression_percent, 5.0);
    }

    #[test]
    fn test_performance_measurement_serialization() {
        let measurement = PerformanceMeasurement {
            component_name: "TestComponent".to_string(),
            test_name: "basic_render".to_string(),
            render_time_ms: StatisticalData::from_measurements(&[10.0, 11.0, 12.0]),
            memory_usage_mb: Some(0.5),
            bundle_size_kb: Some(5.0),
            timestamp: chrono::Utc::now(),
            system_info: SystemInfo {
                os: "Test OS".to_string(),
                cpu_model: "Test CPU".to_string(),
                cpu_cores: 4,
                memory_total_mb: 8192,
                rust_version: "1.70.0".to_string(),
                leptos_version: "0.8.0".to_string(),
            },
            iterations: 1000,
        };

        let json = serde_json::to_string(&measurement).unwrap();
        let deserialized: PerformanceMeasurement = serde_json::from_str(&json).unwrap();
        
        assert_eq!(measurement.component_name, deserialized.component_name);
        assert_eq!(measurement.test_name, deserialized.test_name);
    }
}