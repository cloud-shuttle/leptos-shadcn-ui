# 📊 Performance Audit API Reference

**Complete API documentation for the Performance Audit System**

## 🎯 Overview

The Performance Audit System provides both a command-line interface and a programmatic API for monitoring and optimizing leptos-shadcn-ui component performance.

## 📦 Package Information

- **Package**: `leptos-shadcn-performance-audit`
- **Version**: `0.1.0`
- **Crates.io**: [leptos-shadcn-performance-audit](https://crates.io/crates/leptos-shadcn-performance-audit)

## 🚀 Quick Start

### Installation
```toml
[dependencies]
leptos-shadcn-performance-audit = "0.1.0"
```

### Basic Usage
```rust
use leptos_shadcn_performance_audit::{run_performance_audit, PerformanceConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PerformanceConfig::default();
    let results = run_performance_audit(config).await?;
    
    println!("Overall Score: {:.1}/100", results.overall_score);
    println!("Grade: {}", results.get_grade());
    
    Ok(())
}
```

## 🔧 Core API

### Main Functions

#### `run_performance_audit`
Runs a comprehensive performance audit.

```rust
pub async fn run_performance_audit(
    config: PerformanceConfig
) -> Result<PerformanceResults, PerformanceAuditError>
```

**Parameters:**
- `config: PerformanceConfig` - Performance audit configuration

**Returns:**
- `Result<PerformanceResults, PerformanceAuditError>` - Audit results or error

**Example:**
```rust
use leptos_shadcn_performance_audit::{run_performance_audit, PerformanceConfig};

let config = PerformanceConfig {
    max_component_size_kb: 5.0,
    max_render_time_ms: 16.0,
    max_memory_usage_mb: 1.0,
    monitoring_enabled: true,
};

let results = run_performance_audit(config).await?;
```

## 📊 Data Structures

### `PerformanceConfig`
Configuration for performance audits.

```rust
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub max_component_size_kb: f64,      // Default: 5.0
    pub max_render_time_ms: f64,         // Default: 16.0
    pub max_memory_usage_mb: f64,        // Default: 1.0
    pub monitoring_enabled: bool,        // Default: true
}
```

### `PerformanceResults`
Complete performance audit results.

```rust
#[derive(Debug, Clone)]
pub struct PerformanceResults {
    pub bundle_analysis: BundleAnalysisResults,
    pub performance_monitoring: PerformanceMonitoringResults,
    pub optimization_roadmap: OptimizationRoadmap,
    pub overall_score: f64,
}

impl PerformanceResults {
    pub fn meets_targets(&self) -> bool;
    pub fn get_grade(&self) -> char;
}
```

### `BundleAnalysisResults`
Bundle size analysis results.

```rust
#[derive(Debug, Clone)]
pub struct BundleAnalysisResults {
    pub component_analyses: BTreeMap<String, ComponentBundleAnalysis>,
    pub total_bundle_size_bytes: u64,
    pub total_bundle_size_kb: f64,
    pub average_component_size_kb: f64,
    pub largest_component_size_kb: f64,
    pub oversized_components: Vec<String>,
    pub overall_efficiency_score: f64,
}
```

### `ComponentBundleAnalysis`
Individual component bundle analysis.

```rust
#[derive(Debug, Clone)]
pub struct ComponentBundleAnalysis {
    pub component_name: String,
    pub bundle_size_bytes: u64,
    pub bundle_size_kb: f64,
    pub is_oversized: bool,
    pub performance_score: f64,
    pub optimization_recommendations: Vec<String>,
}
```

### `PerformanceMonitoringResults`
Performance monitoring results.

```rust
#[derive(Debug, Clone)]
pub struct PerformanceMonitoringResults {
    pub component_metrics: BTreeMap<String, ComponentPerformanceMetrics>,
    pub monitoring_duration: Duration,
    pub overall_performance_score: f64,
    pub failing_components: Vec<String>,
    pub performance_bottlenecks: Vec<String>,
}
```

### `ComponentPerformanceMetrics`
Individual component performance metrics.

```rust
#[derive(Debug, Clone)]
pub struct ComponentPerformanceMetrics {
    pub component_name: String,
    pub render_times: Vec<Duration>,
    pub memory_usage: Vec<u64>,
    pub performance_score: f64,
    pub meets_targets: bool,
}
```

### `OptimizationRoadmap`
Optimization recommendations and roadmap.

```rust
#[derive(Debug, Clone)]
pub struct OptimizationRoadmap {
    pub recommendations: Vec<OptimizationRecommendation>,
    pub total_estimated_effort_hours: f64,
    pub total_expected_impact_percent: f64,
    pub overall_roi_score: f64,
}
```

### `OptimizationRecommendation`
Individual optimization recommendation.

```rust
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub title: String,
    pub description: String,
    pub category: OptimizationCategory,
    pub priority: OptimizationPriority,
    pub estimated_effort_hours: f64,
    pub expected_impact_percent: f64,
    pub roi_score: f64,
}
```

### Enums

#### `OptimizationCategory`
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptimizationCategory {
    BundleSize,
    Performance,
    MemoryUsage,
    CodeQuality,
    Dependencies,
}
```

#### `OptimizationPriority`
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OptimizationPriority {
    Critical,
    High,
    Medium,
    Low,
}
```

## 🛠️ Bundle Analysis API

### `BundleAnalyzer`
Analyzes component bundle sizes.

```rust
pub struct BundleAnalyzer {
    pub components_path: PathBuf,
}

impl BundleAnalyzer {
    pub fn new(components_path: PathBuf) -> Self;
    
    pub async fn analyze_all_components(&self) -> BundleAnalysisResults;
    
    pub async fn analyze_component(&self, name: &str) -> ComponentBundleAnalysis;
    
    pub async fn get_component_bundle_size(&self, name: &str) -> u64;
}
```

**Example:**
```rust
use leptos_shadcn_performance_audit::bundle_analysis::BundleAnalyzer;
use std::path::PathBuf;

let analyzer = BundleAnalyzer::new(PathBuf::from("packages/leptos"));
let results = analyzer.analyze_all_components().await;
```

## ⚡ Performance Monitoring API

### `PerformanceMonitor`
Monitors component performance in real-time.

```rust
pub struct PerformanceMonitor {
    pub config: PerformanceConfig,
    pub start_time: Option<Instant>,
    pub tracked_components: BTreeMap<String, ComponentPerformanceMetrics>,
}

impl PerformanceMonitor {
    pub fn new(config: PerformanceConfig) -> Self;
    
    pub fn start_monitoring(&mut self);
    
    pub fn stop_monitoring(&mut self) -> PerformanceMonitoringResults;
    
    pub fn record_render_time(&mut self, component: &str, duration: Duration);
    
    pub fn record_memory_usage(&mut self, component: &str, usage: u64);
    
    pub fn is_monitoring(&self) -> bool;
}
```

**Example:**
```rust
use leptos_shadcn_performance_audit::performance_monitoring::PerformanceMonitor;
use std::time::Duration;

let mut monitor = PerformanceMonitor::new(PerformanceConfig::default());
monitor.start_monitoring();

// Record some metrics
monitor.record_render_time("button", Duration::from_millis(8));
monitor.record_memory_usage("button", 512 * 1024);

let results = monitor.stop_monitoring();
```

## 🗺️ Optimization Roadmap API

### `OptimizationRoadmapGenerator`
Generates optimization recommendations.

```rust
pub struct OptimizationRoadmapGenerator;

impl OptimizationRoadmapGenerator {
    pub fn generate_roadmap(
        bundle_results: &BundleAnalysisResults,
        performance_results: &PerformanceMonitoringResults,
    ) -> OptimizationRoadmap;
}
```

**Example:**
```rust
use leptos_shadcn_performance_audit::optimization_roadmap::OptimizationRoadmapGenerator;

let roadmap = OptimizationRoadmapGenerator::generate_roadmap(
    &bundle_results,
    &performance_results,
);
```

## 📈 Benchmarking API

### `BenchmarkRunner`
Runs performance benchmarks.

```rust
pub struct BenchmarkRunner {
    pub config: BenchmarkConfig,
    pub benchmarks: HashMap<String, Box<dyn Benchmark>>,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self;
    
    pub fn register_benchmark(&mut self, name: String, benchmark: Box<dyn Benchmark>);
    
    pub async fn run_benchmark(&self, name: &str) -> BenchmarkResult;
    
    pub async fn run_all_benchmarks(&self) -> BenchmarkSuiteResults;
}
```

### `BenchmarkConfig`
```rust
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,           // Default: 100
    pub warmup_iterations: usize,    // Default: 10
    pub timeout: Duration,           // Default: 30s
}
```

## ❌ Error Handling

### `PerformanceAuditError`
Custom error types for the performance audit system.

```rust
#[derive(Error, Debug)]
pub enum PerformanceAuditError {
    #[error("Bundle analysis failed: {0}")]
    BundleAnalysisError(String),
    
    #[error("Performance monitoring failed: {0}")]
    PerformanceMonitoringError(String),
    
    #[error("Optimization roadmap generation failed: {0}")]
    OptimizationRoadmapError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

**Example:**
```rust
use leptos_shadcn_performance_audit::PerformanceAuditError;

match run_performance_audit(config).await {
    Ok(results) => println!("Audit completed: {:.1}/100", results.overall_score),
    Err(PerformanceAuditError::BundleAnalysisError(msg)) => {
        eprintln!("Bundle analysis failed: {}", msg);
    },
    Err(e) => eprintln!("Audit failed: {}", e),
}
```

## 🧪 Testing API

### Test Utilities
The performance audit system includes comprehensive test utilities.

```rust
// Test configuration
let config = PerformanceConfig::default();

// Test with mock data
let results = run_performance_audit(config).await?;

// Assertions
assert!(results.overall_score >= 0.0 && results.overall_score <= 100.0);
assert!(results.bundle_analysis.overall_efficiency_score >= 0.0);
assert!(results.performance_monitoring.overall_performance_score >= 0.0);
```

## 🔧 CLI Integration

### Programmatic CLI Usage
You can also use the CLI programmatically:

```rust
use std::process::Command;

let output = Command::new("performance-audit")
    .arg("audit")
    .arg("--format")
    .arg("json")
    .output()?;

let results: serde_json::Value = serde_json::from_slice(&output.stdout)?;
```

## 📚 Examples

### Complete Audit Example
```rust
use leptos_shadcn_performance_audit::{
    run_performance_audit, 
    PerformanceConfig,
    PerformanceAuditError
};

#[tokio::main]
async fn main() -> Result<(), PerformanceAuditError> {
    // Configure performance targets
    let config = PerformanceConfig {
        max_component_size_kb: 5.0,
        max_render_time_ms: 16.0,
        max_memory_usage_mb: 1.0,
        monitoring_enabled: true,
    };
    
    // Run comprehensive audit
    let results = run_performance_audit(config).await?;
    
    // Display results
    println!("📊 Performance Audit Results");
    println!("Overall Score: {:.1}/100 ({})", results.overall_score, results.get_grade());
    println!("Meets Targets: {}", if results.meets_targets() { "✅ Yes" } else { "❌ No" });
    
    // Bundle analysis
    println!("\n📦 Bundle Analysis:");
    println!("  Overall Efficiency: {:.1}%", results.bundle_analysis.overall_efficiency_score);
    println!("  Total Size: {:.1} KB", results.bundle_analysis.total_bundle_size_kb);
    println!("  Average Component Size: {:.1} KB", results.bundle_analysis.average_component_size_kb);
    
    // Performance monitoring
    println!("\n⚡ Performance Monitoring:");
    println!("  Overall Score: {:.1}%", results.performance_monitoring.overall_performance_score);
    println!("  Failing Components: {}", results.performance_monitoring.failing_components.len());
    
    // Optimization roadmap
    println!("\n🗺️ Optimization Roadmap:");
    println!("  Total Recommendations: {}", results.optimization_roadmap.recommendations.len());
    println!("  Estimated Effort: {:.1} hours", results.optimization_roadmap.total_estimated_effort_hours);
    println!("  Expected Impact: {:.1}%", results.optimization_roadmap.total_expected_impact_percent);
    
    Ok(())
}
```

### Custom Monitoring Example
```rust
use leptos_shadcn_performance_audit::performance_monitoring::PerformanceMonitor;
use std::time::{Duration, Instant};

let mut monitor = PerformanceMonitor::new(PerformanceConfig::default());
monitor.start_monitoring();

// Simulate component rendering
let start = Instant::now();
// ... render component ...
let render_time = start.elapsed();

monitor.record_render_time("my-component", render_time);
monitor.record_memory_usage("my-component", 1024 * 1024); // 1MB

let results = monitor.stop_monitoring();
println!("Component performance score: {:.1}", results.overall_performance_score);
```

## 📖 Additional Resources

- **[Quick Start Guide](QUICK_START.md)** - Get started in 5 minutes
- **[Complete Documentation](README.md)** - Full system documentation
- **[CLI Reference](CLI.md)** - Command-line interface documentation
- **[Examples](../examples/)** - Working code examples

## 🤝 Contributing

We welcome contributions to the API! Please see our [Contributing Guide](../../CONTRIBUTING.md) for details.

---

**🎯 Build powerful performance monitoring into your Leptos applications with the Performance Audit API!**
