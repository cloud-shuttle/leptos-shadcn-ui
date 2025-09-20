//! Performance Regression Testing Module
//! 
//! This module provides automated performance regression testing for leptos-shadcn-ui components,
//! including baseline comparison, trend analysis, and automated optimization recommendations.

use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::benchmarks::{BenchmarkResult, BenchmarkSuiteResults};
use crate::PerformanceAuditError;

/// Performance regression test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTestResult {
    /// Test name
    pub test_name: String,
    /// Component name
    pub component_name: String,
    /// Baseline performance
    pub baseline: PerformanceMetrics,
    /// Current performance
    pub current: PerformanceMetrics,
    /// Performance change
    pub change: PerformanceChange,
    /// Regression detected
    pub regression_detected: bool,
    /// Severity of regression
    pub severity: RegressionSeverity,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Performance metrics for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average execution time
    pub average_time_ms: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Bundle size in bytes
    pub bundle_size_bytes: u64,
    /// Performance score
    pub performance_score: f64,
    /// Number of iterations
    pub iterations: u32,
}

/// Performance change analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceChange {
    /// Time change percentage
    pub time_change_percent: f64,
    /// Memory change percentage
    pub memory_change_percent: f64,
    /// Bundle size change percentage
    pub bundle_size_change_percent: f64,
    /// Performance score change
    pub score_change: f64,
    /// Is improvement
    pub is_improvement: bool,
}

/// Regression severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegressionSeverity {
    /// No regression detected
    None,
    /// Minor regression (5-15% performance loss)
    Minor,
    /// Moderate regression (15-30% performance loss)
    Moderate,
    /// Major regression (30-50% performance loss)
    Major,
    /// Critical regression (>50% performance loss)
    Critical,
}

/// Performance baseline data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// Component baselines
    pub component_baselines: HashMap<String, PerformanceMetrics>,
    /// Overall baseline
    pub overall_baseline: PerformanceMetrics,
    /// Baseline timestamp
    pub timestamp: u64,
    /// Git commit hash
    pub commit_hash: String,
    /// Environment info
    pub environment: EnvironmentInfo,
}

/// Environment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Rust version
    pub rust_version: String,
    /// Target architecture
    pub target_arch: String,
    /// OS information
    pub os_info: String,
    /// CPU information
    pub cpu_info: String,
    /// Memory information
    pub memory_info: String,
}

/// Regression testing configuration
#[derive(Debug, Clone)]
pub struct RegressionTestConfig {
    /// Baseline file path
    pub baseline_path: String,
    /// Results output path
    pub results_path: String,
    /// Regression thresholds
    pub thresholds: RegressionThresholds,
    /// Auto-update baseline
    pub auto_update_baseline: bool,
    /// Generate recommendations
    pub generate_recommendations: bool,
}

/// Regression thresholds
#[derive(Debug, Clone)]
pub struct RegressionThresholds {
    /// Minor regression threshold (percentage)
    pub minor_threshold: f64,
    /// Moderate regression threshold (percentage)
    pub moderate_threshold: f64,
    /// Major regression threshold (percentage)
    pub major_threshold: f64,
    /// Critical regression threshold (percentage)
    pub critical_threshold: f64,
}

impl Default for RegressionThresholds {
    fn default() -> Self {
        Self {
            minor_threshold: 5.0,
            moderate_threshold: 15.0,
            major_threshold: 30.0,
            critical_threshold: 50.0,
        }
    }
}

impl Default for RegressionTestConfig {
    fn default() -> Self {
        Self {
            baseline_path: "performance-baseline.json".to_string(),
            results_path: "performance-regression-results.json".to_string(),
            thresholds: RegressionThresholds::default(),
            auto_update_baseline: false,
            generate_recommendations: true,
        }
    }
}

/// Performance regression tester
pub struct RegressionTester {
    config: RegressionTestConfig,
    baseline: Option<PerformanceBaseline>,
}

impl RegressionTester {
    /// Create new regression tester
    pub fn new(config: RegressionTestConfig) -> Self {
        Self {
            config,
            baseline: None,
        }
    }

    /// Load performance baseline
    pub async fn load_baseline(&mut self) -> Result<(), PerformanceAuditError> {
        let baseline_path = Path::new(&self.config.baseline_path);
        
        if !baseline_path.exists() {
            return Err(PerformanceAuditError::ConfigurationError(
                format!("Baseline file not found: {}", self.config.baseline_path)
            ));
        }

        let baseline_content = std::fs::read_to_string(baseline_path)
            .map_err(|e| PerformanceAuditError::IoError(e))?;
        
        let baseline: PerformanceBaseline = serde_json::from_str(&baseline_content)
            .map_err(|e| PerformanceAuditError::ConfigurationError(
                format!("Failed to parse baseline: {}", e)
            ))?;

        self.baseline = Some(baseline);
        Ok(())
    }

    /// Run regression tests
    pub async fn run_regression_tests(
        &self,
        current_results: &BenchmarkSuiteResults,
    ) -> Result<Vec<RegressionTestResult>, PerformanceAuditError> {
        let baseline = self.baseline.as_ref()
            .ok_or_else(|| PerformanceAuditError::ConfigurationError(
                "Baseline not loaded".to_string()
            ))?;

        let mut regression_results = Vec::new();

        for (test_name, current_result) in &current_results.benchmark_results {
            if let Some(baseline_metrics) = baseline.component_baselines.get(&current_result.component_name) {
                let regression_result = self.analyze_regression(
                    test_name,
                    &current_result,
                    baseline_metrics,
                )?;
                regression_results.push(regression_result);
            }
        }

        Ok(regression_results)
    }

    /// Analyze performance regression for a single component
    fn analyze_regression(
        &self,
        test_name: &str,
        current_result: &BenchmarkResult,
        baseline_metrics: &PerformanceMetrics,
    ) -> Result<RegressionTestResult, PerformanceAuditError> {
        let current_metrics = PerformanceMetrics {
            average_time_ms: current_result.average_time.as_secs_f64() * 1000.0,
            memory_usage_bytes: current_result.memory_usage_bytes,
            bundle_size_bytes: 0, // Would be populated from bundle analysis
            performance_score: current_result.performance_score,
            iterations: current_result.iterations,
        };

        let change = self.calculate_performance_change(baseline_metrics, &current_metrics);
        let severity = self.determine_regression_severity(&change);
        let regression_detected = severity != RegressionSeverity::None;
        
        let recommendations = if self.config.generate_recommendations {
            self.generate_recommendations(&change, &severity)
        } else {
            Vec::new()
        };

        Ok(RegressionTestResult {
            test_name: test_name.to_string(),
            component_name: current_result.component_name.clone(),
            baseline: baseline_metrics.clone(),
            current: current_metrics,
            change,
            regression_detected,
            severity,
            recommendations,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap_or_default().as_secs(),
        })
    }

    /// Calculate performance change between baseline and current
    fn calculate_performance_change(
        &self,
        baseline: &PerformanceMetrics,
        current: &PerformanceMetrics,
    ) -> PerformanceChange {
        let time_change_percent = if baseline.average_time_ms > 0.0 {
            ((current.average_time_ms - baseline.average_time_ms) / baseline.average_time_ms) * 100.0
        } else {
            0.0
        };

        let memory_change_percent = if baseline.memory_usage_bytes > 0 {
            ((current.memory_usage_bytes as f64 - baseline.memory_usage_bytes as f64) 
             / baseline.memory_usage_bytes as f64) * 100.0
        } else {
            0.0
        };

        let bundle_size_change_percent = if baseline.bundle_size_bytes > 0 {
            ((current.bundle_size_bytes as f64 - baseline.bundle_size_bytes as f64) 
             / baseline.bundle_size_bytes as f64) * 100.0
        } else {
            0.0
        };

        let score_change = current.performance_score - baseline.performance_score;
        let is_improvement = time_change_percent < 0.0 && memory_change_percent < 0.0;

        PerformanceChange {
            time_change_percent,
            memory_change_percent,
            bundle_size_change_percent,
            score_change,
            is_improvement,
        }
    }

    /// Determine regression severity based on performance change
    fn determine_regression_severity(&self, change: &PerformanceChange) -> RegressionSeverity {
        let worst_change = change.time_change_percent.max(change.memory_change_percent);

        if worst_change <= self.config.thresholds.minor_threshold {
            RegressionSeverity::None
        } else if worst_change <= self.config.thresholds.moderate_threshold {
            RegressionSeverity::Minor
        } else if worst_change <= self.config.thresholds.major_threshold {
            RegressionSeverity::Moderate
        } else if worst_change <= self.config.thresholds.critical_threshold {
            RegressionSeverity::Major
        } else {
            RegressionSeverity::Critical
        }
    }

    /// Generate optimization recommendations
    fn generate_recommendations(
        &self,
        change: &PerformanceChange,
        severity: &RegressionSeverity,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if change.time_change_percent > 0.0 {
            recommendations.push(format!(
                "Performance regression detected: {:.1}% slower execution time",
                change.time_change_percent
            ));
            
            if change.time_change_percent > 20.0 {
                recommendations.push("Consider optimizing component rendering logic".to_string());
                recommendations.push("Review component lifecycle and state management".to_string());
            }
        }

        if change.memory_change_percent > 0.0 {
            recommendations.push(format!(
                "Memory usage increased by {:.1}%",
                change.memory_change_percent
            ));
            
            if change.memory_change_percent > 20.0 {
                recommendations.push("Check for memory leaks in component cleanup".to_string());
                recommendations.push("Review component state and signal management".to_string());
            }
        }

        if change.bundle_size_change_percent > 0.0 {
            recommendations.push(format!(
                "Bundle size increased by {:.1}%",
                change.bundle_size_change_percent
            ));
            
            if change.bundle_size_change_percent > 10.0 {
                recommendations.push("Consider code splitting and lazy loading".to_string());
                recommendations.push("Review dependencies and remove unused code".to_string());
            }
        }

        match severity {
            RegressionSeverity::Critical => {
                recommendations.push("CRITICAL: Immediate attention required".to_string());
                recommendations.push("Consider reverting recent changes".to_string());
            },
            RegressionSeverity::Major => {
                recommendations.push("MAJOR: High priority optimization needed".to_string());
                recommendations.push("Schedule performance review meeting".to_string());
            },
            RegressionSeverity::Moderate => {
                recommendations.push("MODERATE: Performance optimization recommended".to_string());
            },
            RegressionSeverity::Minor => {
                recommendations.push("MINOR: Monitor performance trends".to_string());
            },
            RegressionSeverity::None => {
                if change.is_improvement {
                    recommendations.push("Performance improvement detected - good work!".to_string());
                }
            },
        }

        recommendations
    }

    /// Save regression test results
    pub async fn save_results(
        &self,
        results: &[RegressionTestResult],
    ) -> Result<(), PerformanceAuditError> {
        let results_path = Path::new(&self.config.results_path);
        
        // Ensure directory exists
        if let Some(parent) = results_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| PerformanceAuditError::IoError(e))?;
        }

        let results_json = serde_json::to_string_pretty(results)
            .map_err(|e| PerformanceAuditError::ConfigurationError(
                format!("Failed to serialize results: {}", e)
            ))?;

        std::fs::write(results_path, results_json)
            .map_err(|e| PerformanceAuditError::IoError(e))?;

        Ok(())
    }

    /// Update performance baseline
    pub async fn update_baseline(
        &mut self,
        current_results: &BenchmarkSuiteResults,
        commit_hash: &str,
    ) -> Result<(), PerformanceAuditError> {
        let mut component_baselines = HashMap::new();
        let mut total_time = 0.0;
        let mut total_memory = 0u64;
        let mut total_score = 0.0;
        let mut component_count = 0;

        for (_, result) in &current_results.benchmark_results {
            let metrics = PerformanceMetrics {
                average_time_ms: result.average_time.as_secs_f64() * 1000.0,
                memory_usage_bytes: result.memory_usage_bytes,
                bundle_size_bytes: 0, // Would be populated from bundle analysis
                performance_score: result.performance_score,
                iterations: result.iterations,
            };

            component_baselines.insert(result.component_name.clone(), metrics.clone());
            
            total_time += metrics.average_time_ms;
            total_memory += metrics.memory_usage_bytes;
            total_score += metrics.performance_score;
            component_count += 1;
        }

        let overall_baseline = if component_count > 0 {
            PerformanceMetrics {
                average_time_ms: total_time / component_count as f64,
                memory_usage_bytes: total_memory / component_count as u64,
                bundle_size_bytes: 0,
                performance_score: total_score / component_count as f64,
                iterations: 0,
            }
        } else {
            PerformanceMetrics {
                average_time_ms: 0.0,
                memory_usage_bytes: 0,
                bundle_size_bytes: 0,
                performance_score: 0.0,
                iterations: 0,
            }
        };

        let baseline = PerformanceBaseline {
            component_baselines,
            overall_baseline,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap_or_default().as_secs(),
            commit_hash: commit_hash.to_string(),
            environment: self.get_environment_info(),
        };

        let baseline_json = serde_json::to_string_pretty(&baseline)
            .map_err(|e| PerformanceAuditError::ConfigurationError(
                format!("Failed to serialize baseline: {}", e)
            ))?;

        std::fs::write(&self.config.baseline_path, baseline_json)
            .map_err(|e| PerformanceAuditError::IoError(e))?;

        self.baseline = Some(baseline);
        Ok(())
    }

    /// Get environment information
    fn get_environment_info(&self) -> EnvironmentInfo {
        EnvironmentInfo {
            rust_version: env!("RUSTC_VERSION").to_string(),
            target_arch: std::env::consts::ARCH.to_string(),
            os_info: format!("{} {}", std::env::consts::OS, std::env::consts::FAMILY),
            cpu_info: "Unknown".to_string(), // Would be populated with actual CPU info
            memory_info: "Unknown".to_string(), // Would be populated with actual memory info
        }
    }

    /// Generate regression test report
    pub fn generate_report(&self, results: &[RegressionTestResult]) -> String {
        let mut report = String::new();
        
        report.push_str("# Performance Regression Test Report\n\n");
        report.push_str(&format!("**Generated**: {}\n\n", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Summary
        let total_tests = results.len();
        let regressions = results.iter().filter(|r| r.regression_detected).count();
        let critical_regressions = results.iter()
            .filter(|r| r.severity == RegressionSeverity::Critical).count();
        let major_regressions = results.iter()
            .filter(|r| r.severity == RegressionSeverity::Major).count();

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Tests**: {}\n", total_tests));
        report.push_str(&format!("- **Regressions Detected**: {}\n", regressions));
        report.push_str(&format!("- **Critical Regressions**: {}\n", critical_regressions));
        report.push_str(&format!("- **Major Regressions**: {}\n", major_regressions));
        report.push_str("\n");

        // Detailed results
        if !results.is_empty() {
            report.push_str("## Detailed Results\n\n");
            
            for result in results {
                report.push_str(&format!("### {}\n", result.component_name));
                report.push_str(&format!("- **Test**: {}\n", result.test_name));
                report.push_str(&format!("- **Severity**: {:?}\n", result.severity));
                report.push_str(&format!("- **Time Change**: {:.1}%\n", result.change.time_change_percent));
                report.push_str(&format!("- **Memory Change**: {:.1}%\n", result.change.memory_change_percent));
                report.push_str(&format!("- **Score Change**: {:.1}\n", result.change.score_change));
                
                if !result.recommendations.is_empty() {
                    report.push_str("\n**Recommendations**:\n");
                    for rec in &result.recommendations {
                        report.push_str(&format!("- {}\n", rec));
                    }
                }
                report.push_str("\n");
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_regression_severity_determination() {
        let config = RegressionTestConfig::default();
        let tester = RegressionTester::new(config);

        let change = PerformanceChange {
            time_change_percent: 3.0,
            memory_change_percent: 2.0,
            bundle_size_change_percent: 1.0,
            score_change: -5.0,
            is_improvement: false,
        };

        let severity = tester.determine_regression_severity(&change);
        assert_eq!(severity, RegressionSeverity::None);

        let change = PerformanceChange {
            time_change_percent: 10.0,
            memory_change_percent: 8.0,
            bundle_size_change_percent: 5.0,
            score_change: -10.0,
            is_improvement: false,
        };

        let severity = tester.determine_regression_severity(&change);
        assert_eq!(severity, RegressionSeverity::Minor);
    }

    #[test]
    fn test_performance_change_calculation() {
        let config = RegressionTestConfig::default();
        let tester = RegressionTester::new(config);

        let baseline = PerformanceMetrics {
            average_time_ms: 100.0,
            memory_usage_bytes: 1024,
            bundle_size_bytes: 2048,
            performance_score: 90.0,
            iterations: 100,
        };

        let current = PerformanceMetrics {
            average_time_ms: 110.0,
            memory_usage_bytes: 1126,
            bundle_size_bytes: 2150,
            performance_score: 85.0,
            iterations: 100,
        };

        let change = tester.calculate_performance_change(&baseline, &current);
        
        assert!((change.time_change_percent - 10.0).abs() < 0.1);
        assert!((change.memory_change_percent - 10.0).abs() < 0.1);
        assert!((change.bundle_size_change_percent - 5.0).abs() < 0.1);
        assert!((change.score_change - (-5.0)).abs() < 0.1);
        assert!(!change.is_improvement);
    }
}
