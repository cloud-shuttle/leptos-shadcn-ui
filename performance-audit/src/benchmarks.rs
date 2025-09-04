//! Performance Benchmarks Module
//! 
//! This module provides comprehensive benchmarking for leptos-shadcn-ui components
//! using TDD principles to ensure optimal performance.

use std::collections::HashMap;
use std::time::Duration;

/// Benchmark result for a single test
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Component being benchmarked
    pub component_name: String,
    /// Average execution time
    pub average_time: Duration,
    /// Minimum execution time
    pub min_time: Duration,
    /// Maximum execution time
    pub max_time: Duration,
    /// Standard deviation
    pub std_deviation: Duration,
    /// Number of iterations
    pub iterations: u32,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Performance score (0-100)
    pub performance_score: f64,
    /// Meets performance target
    pub meets_target: bool,
}

impl BenchmarkResult {
    /// Create new benchmark result
    pub fn new(name: String, component_name: String) -> Self {
        Self {
            name,
            component_name,
            average_time: Duration::from_secs(0),
            min_time: Duration::from_secs(0),
            max_time: Duration::from_secs(0),
            std_deviation: Duration::from_secs(0),
            iterations: 0,
            memory_usage_bytes: 0,
            performance_score: 0.0,
            meets_target: false,
        }
    }
    
    /// Calculate performance score based on target time
    pub fn calculate_performance_score(&mut self, target_time: Duration) {
        let target_ms = target_time.as_secs_f64() * 1000.0;
        let actual_ms = self.average_time.as_secs_f64() * 1000.0;
        
        if actual_ms <= target_ms {
            self.performance_score = 100.0;
        } else {
            self.performance_score = (target_ms / actual_ms * 100.0).min(100.0);
        }
        
        self.meets_target = self.performance_score >= 80.0;
    }
}

/// Benchmark suite results
#[derive(Debug, Clone)]
pub struct BenchmarkSuiteResults {
    /// Individual benchmark results
    pub benchmark_results: HashMap<String, BenchmarkResult>,
    /// Overall suite performance score
    pub overall_score: f64,
    /// Components failing benchmarks
    pub failing_components: Vec<String>,
    /// Performance trends
    pub performance_trends: Vec<PerformanceTrend>,
}

impl Default for BenchmarkSuiteResults {
    fn default() -> Self {
        Self {
            benchmark_results: HashMap::new(),
            overall_score: 0.0,
            failing_components: Vec::new(),
            performance_trends: Vec::new(),
        }
    }
}

impl BenchmarkSuiteResults {
    /// Add benchmark result
    pub fn add_result(&mut self, result: BenchmarkResult) {
        let name = result.name.clone();
        let _component_name = result.component_name.clone();
        
        self.benchmark_results.insert(name.clone(), result);
        self.recalculate_overall_metrics();
    }
    
    /// Recalculate overall metrics
    fn recalculate_overall_metrics(&mut self) {
        if self.benchmark_results.is_empty() {
            self.overall_score = 0.0;
            self.failing_components.clear();
            return;
        }
        
        self.overall_score = self.benchmark_results
            .values()
            .map(|r| r.performance_score)
            .sum::<f64>() / self.benchmark_results.len() as f64;
        
        self.failing_components = self.benchmark_results
            .values()
            .filter(|r| !r.meets_target)
            .map(|r| r.component_name.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
    }
    
    /// Check if suite meets performance targets
    pub fn meets_targets(&self) -> bool {
        self.overall_score >= 80.0 && self.failing_components.is_empty()
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
        
        for (name, result) in &self.benchmark_results {
            if !result.meets_target {
                recommendations.push(format!(
                    "Optimize {} benchmark: {:.1}ms exceeds target", 
                    name, 
                    result.average_time.as_secs_f64() * 1000.0
                ));
            }
        }
        
        recommendations
    }
}

/// Performance trend over time
#[derive(Debug, Clone)]
pub struct PerformanceTrend {
    /// Component name
    pub component_name: String,
    /// Benchmark name
    pub benchmark_name: String,
    /// Trend direction
    pub trend_direction: TrendDirection,
    /// Performance change percentage
    pub change_percentage: f64,
    /// Trend confidence (0-100)
    pub confidence: f64,
}

/// Trend direction
#[derive(Debug, Clone)]
pub enum TrendDirection {
    Improving,
    Degrading,
    Stable,
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of warmup iterations
    pub warmup_iterations: u32,
    /// Number of benchmark iterations
    pub benchmark_iterations: u32,
    /// Target execution time per benchmark
    pub target_time: Duration,
    /// Enable memory profiling
    pub enable_memory_profiling: bool,
    /// Enable statistical analysis
    pub enable_statistical_analysis: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 10,
            benchmark_iterations: 100,
            target_time: Duration::from_millis(16), // 60fps target
            enable_memory_profiling: true,
            enable_statistical_analysis: true,
        }
    }
}

/// Benchmark runner for leptos-shadcn-ui components
pub struct BenchmarkRunner {
    /// Benchmark configuration
    pub config: BenchmarkConfig,
    /// Registered benchmarks
    pub benchmarks: HashMap<String, Box<dyn Benchmark>>,
}

/// Trait for benchmark implementations
pub trait Benchmark: Send + Sync {
    /// Get benchmark name
    fn name(&self) -> &str;
    
    /// Get component name
    fn component_name(&self) -> &str;
    
    /// Run the benchmark
    fn run(&self, iterations: u32) -> BenchmarkResult;
    
    /// Setup benchmark (called before running)
    fn setup(&self) -> Result<(), String> {
        Ok(())
    }
    
    /// Teardown benchmark (called after running)
    fn teardown(&self) -> Result<(), String> {
        Ok(())
    }
}

impl BenchmarkRunner {
    /// Create new benchmark runner
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            benchmarks: HashMap::new(),
        }
    }
    
    /// Register a benchmark
    pub fn register_benchmark(&mut self, benchmark: Box<dyn Benchmark>) {
        let name = benchmark.name().to_string();
        self.benchmarks.insert(name, benchmark);
    }
    
    /// Run all registered benchmarks
    pub async fn run_all_benchmarks(&self) -> BenchmarkSuiteResults {
        let mut results = BenchmarkSuiteResults::default();
        
        for (name, benchmark) in &self.benchmarks {
            // Setup benchmark
            if let Err(e) = benchmark.setup() {
                eprintln!("Failed to setup benchmark {}: {}", name, e);
                continue;
            }
            
            // Run benchmark
            let result = benchmark.run(self.config.benchmark_iterations);
            results.add_result(result);
            
            // Teardown benchmark
            if let Err(e) = benchmark.teardown() {
                eprintln!("Failed to teardown benchmark {}: {}", name, e);
            }
        }
        
        results
    }
    
    /// Run specific benchmark
    pub async fn run_benchmark(&self, name: &str) -> Option<BenchmarkResult> {
        let benchmark = self.benchmarks.get(name)?;
        
        // Setup benchmark
        if let Err(e) = benchmark.setup() {
            eprintln!("Failed to setup benchmark {}: {}", name, e);
            return None;
        }
        
        // Run benchmark
        let result = benchmark.run(self.config.benchmark_iterations);
        
        // Teardown benchmark
        if let Err(e) = benchmark.teardown() {
            eprintln!("Failed to teardown benchmark {}: {}", name, e);
        }
        
        Some(result)
    }
    
    /// Get registered benchmark names
    pub fn get_benchmark_names(&self) -> Vec<String> {
        self.benchmarks.keys().cloned().collect()
    }
}

/// Mock benchmark for testing
pub struct MockBenchmark {
    pub name: String,
    pub component_name: String,
    pub execution_time: Duration,
    pub memory_usage: u64,
}

impl Benchmark for MockBenchmark {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn component_name(&self) -> &str {
        &self.component_name
    }
    
    fn run(&self, iterations: u32) -> BenchmarkResult {
        let mut result = BenchmarkResult::new(self.name.clone(), self.component_name.clone());
        result.average_time = self.execution_time;
        result.min_time = self.execution_time;
        result.max_time = self.execution_time;
        result.iterations = iterations;
        result.memory_usage_bytes = self.memory_usage;
        result.calculate_performance_score(Duration::from_millis(16));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_result_creation() {
        let result = BenchmarkResult::new("render-test".to_string(), "button".to_string());
        
        assert_eq!(result.name, "render-test");
        assert_eq!(result.component_name, "button");
        assert_eq!(result.iterations, 0);
        assert_eq!(result.performance_score, 0.0);
        assert!(!result.meets_target);
    }

    #[test]
    fn test_benchmark_result_performance_score() {
        let mut result = BenchmarkResult::new("fast-test".to_string(), "button".to_string());
        result.average_time = Duration::from_millis(8); // Fast execution
        
        result.calculate_performance_score(Duration::from_millis(16));
        
        assert_eq!(result.performance_score, 100.0);
        assert!(result.meets_target);
    }

    #[test]
    fn test_benchmark_result_slow_performance() {
        let mut result = BenchmarkResult::new("slow-test".to_string(), "button".to_string());
        result.average_time = Duration::from_millis(32); // Slow execution
        
        result.calculate_performance_score(Duration::from_millis(16));
        
        assert_eq!(result.performance_score, 50.0); // 16/32 * 100
        assert!(!result.meets_target);
    }

    #[test]
    fn test_benchmark_suite_results_default() {
        let results = BenchmarkSuiteResults::default();
        
        assert!(results.benchmark_results.is_empty());
        assert_eq!(results.overall_score, 0.0);
        assert!(results.failing_components.is_empty());
        assert!(results.performance_trends.is_empty());
    }

    #[test]
    fn test_benchmark_suite_results_add_result() {
        let mut results = BenchmarkSuiteResults::default();
        let mut result = BenchmarkResult::new("test-1".to_string(), "button".to_string());
        result.average_time = Duration::from_millis(8);
        result.calculate_performance_score(Duration::from_millis(16));
        
        results.add_result(result);
        
        assert_eq!(results.benchmark_results.len(), 1);
        assert_eq!(results.overall_score, 100.0);
        assert!(results.failing_components.is_empty());
    }

    #[test]
    fn test_benchmark_suite_results_failing_component() {
        let mut results = BenchmarkSuiteResults::default();
        let mut result = BenchmarkResult::new("slow-test".to_string(), "button".to_string());
        result.average_time = Duration::from_millis(32);
        result.calculate_performance_score(Duration::from_millis(16));
        
        results.add_result(result);
        
        assert_eq!(results.failing_components.len(), 1);
        assert_eq!(results.failing_components[0], "button");
        assert!(!results.meets_targets());
    }

    #[test]
    fn test_benchmark_config_defaults() {
        let config = BenchmarkConfig::default();
        
        assert_eq!(config.warmup_iterations, 10);
        assert_eq!(config.benchmark_iterations, 100);
        assert_eq!(config.target_time, Duration::from_millis(16));
        assert!(config.enable_memory_profiling);
        assert!(config.enable_statistical_analysis);
    }

    #[test]
    fn test_benchmark_runner_creation() {
        let config = BenchmarkConfig::default();
        let runner = BenchmarkRunner::new(config);
        
        assert!(runner.benchmarks.is_empty());
    }

    #[test]
    fn test_benchmark_runner_register_benchmark() {
        let config = BenchmarkConfig::default();
        let mut runner = BenchmarkRunner::new(config);
        
        let benchmark = Box::new(MockBenchmark {
            name: "test-benchmark".to_string(),
            component_name: "button".to_string(),
            execution_time: Duration::from_millis(10),
            memory_usage: 1024,
        });
        
        runner.register_benchmark(benchmark);
        
        assert_eq!(runner.benchmarks.len(), 1);
        assert_eq!(runner.get_benchmark_names(), vec!["test-benchmark"]);
    }

    #[test]
    fn test_mock_benchmark_implementation() {
        let benchmark = MockBenchmark {
            name: "mock-test".to_string(),
            component_name: "button".to_string(),
            execution_time: Duration::from_millis(12),
            memory_usage: 2048,
        };
        
        assert_eq!(benchmark.name(), "mock-test");
        assert_eq!(benchmark.component_name(), "button");
        
        let result = benchmark.run(50);
        
        assert_eq!(result.name, "mock-test");
        assert_eq!(result.component_name, "button");
        assert_eq!(result.average_time, Duration::from_millis(12));
        assert_eq!(result.iterations, 50);
        assert_eq!(result.memory_usage_bytes, 2048);
        assert!(result.meets_target); // 12ms < 16ms target
    }

    #[test]
    fn test_benchmark_suite_recommendations() {
        let mut results = BenchmarkSuiteResults::default();
        
        // Add failing benchmark
        let mut result = BenchmarkResult::new("slow-test".to_string(), "button".to_string());
        result.average_time = Duration::from_millis(32);
        result.calculate_performance_score(Duration::from_millis(16));
        
        results.add_result(result);
        
        let recommendations = results.get_performance_recommendations();
        assert!(!recommendations.is_empty());
        assert!(recommendations[0].contains("button"));
    }
}
