# 📊 Performance Audit System

**Complete performance monitoring and optimization system for leptos-shadcn-ui components**

## 🎯 Overview

The Performance Audit System is a comprehensive tool built with TDD principles to monitor, analyze, and optimize the performance of leptos-shadcn-ui components. It provides real-time monitoring, bundle size analysis, and actionable optimization recommendations.

## ✨ Features

### 📊 Bundle Size Analysis
- **Component Size Tracking** - Monitor individual component bundle sizes
- **Oversized Component Detection** - Identify components exceeding size thresholds
- **Bundle Efficiency Scoring** - Calculate overall bundle efficiency metrics
- **Optimization Recommendations** - Get specific suggestions for size reduction

### ⚡ Real-time Performance Monitoring
- **Render Time Tracking** - Monitor component render performance
- **Memory Usage Monitoring** - Track memory consumption patterns
- **Performance Bottleneck Detection** - Identify slow-performing components
- **Performance Scoring** - Calculate overall performance metrics

### 🗺️ Optimization Roadmap
- **Smart Recommendations** - AI-powered optimization suggestions
- **ROI-based Prioritization** - Rank optimizations by impact vs effort
- **Implementation Planning** - Generate actionable implementation plans
- **Effort Estimation** - Estimate time and resources needed

### 🛠️ CLI Tool
- **Multiple Output Formats** - Text, JSON, HTML, Markdown
- **Progress Indicators** - Visual feedback during long operations
- **Configuration Display** - Show current settings and thresholds
- **Professional Error Handling** - Robust error recovery and reporting

## 🚀 Quick Start

### Installation

```bash
# Install the performance audit tool
cargo install leptos-shadcn-performance-audit
```

### Basic Usage

```bash
# Run complete performance audit
performance-audit audit

# Analyze bundle sizes only
performance-audit bundle --components-path packages/leptos

# Monitor real-time performance
performance-audit monitor --duration 30s --sample-rate 100ms

# Generate optimization roadmap
performance-audit roadmap --output roadmap.json --format json
```

## 📋 CLI Commands

### `audit` - Complete Performance Audit
Runs a comprehensive performance audit including bundle analysis and performance monitoring.

```bash
performance-audit audit [OPTIONS]

Options:
  -c, --components-path <COMPONENTS_PATH>
          Components directory path [default: packages/leptos]
      --max-component-size-kb <MAX_COMPONENT_SIZE_KB>
          Maximum component size in KB [default: 5.0]
      --max-render-time-ms <MAX_RENDER_TIME_MS>
          Maximum render time in milliseconds [default: 16.0]
      --max-memory-usage-mb <MAX_MEMORY_USAGE_MB>
          Maximum memory usage in MB [default: 1.0]
```

### `bundle` - Bundle Size Analysis
Analyzes component bundle sizes and identifies optimization opportunities.

```bash
performance-audit bundle [OPTIONS]

Options:
  -c, --components-path <COMPONENTS_PATH>
          Components directory path [default: packages/leptos]
      --target-size-kb <TARGET_SIZE_KB>
          Target component size in KB [default: 5.0]
```

### `monitor` - Real-time Performance Monitoring
Monitors component performance in real-time.

```bash
performance-audit monitor [OPTIONS]

Options:
  -d, --duration <DURATION>
          Monitoring duration [default: 30s]
      --sample-rate <SAMPLE_RATE>
          Sample rate for monitoring [default: 100ms]
```

### `roadmap` - Optimization Roadmap Generation
Generates actionable optimization recommendations.

```bash
performance-audit roadmap [OPTIONS]

Options:
  -i, --input <INPUT>
          Input file path
  -o, --output <OUTPUT>
          Output file path
      --format <FORMAT>
          Output format [default: text] [possible values: text, json, html, markdown]
```

## 📊 Output Formats

### Text Format (Default)
Human-readable text output with emojis and formatting:

```
🔍 Running comprehensive performance audit...
📊 Configuration:
   Max Component Size: 5.0 KB
   Max Render Time: 16.0 ms
   Max Memory Usage: 1.0 MB
   Output Format: Text

⏳ Analyzing components...
✅ Analysis complete!

📊 Performance Audit Results
Overall Score: 64.0/100 (D)
Meets Targets: ❌ No

📦 Bundle Analysis:
  Overall Efficiency: 44.6%
  Total Size: 23.0 KB
  Average Component Size: 4.6 KB

⚡ Performance Monitoring:
  Overall Score: 83.3%
  Failing Components: 2

🗺️ Optimization Roadmap:
  Total Recommendations: 6
  Estimated Effort: 40.0 hours
  Expected Impact: 470.0%
```

### JSON Format
Structured JSON output for programmatic processing:

```json
{
  "overall_score": 64.0,
  "meets_targets": false,
  "bundle_analysis": {
    "overall_efficiency_score": 44.6,
    "total_bundle_size_bytes": 23552,
    "total_bundle_size_kb": 23.0,
    "average_component_size_kb": 4.6
  },
  "performance_monitoring": {
    "overall_performance_score": 83.3,
    "failing_components": 2
  },
  "optimization_roadmap": {
    "total_recommendations": 6,
    "estimated_effort_hours": 40.0,
    "expected_impact_percent": 470.0
  }
}
```

### HTML Format
Rich HTML output for web display and reporting.

### Markdown Format
Markdown output for documentation and GitHub integration.

## 🧪 Testing

The performance audit system includes comprehensive testing:

```bash
# Run all performance audit tests (53 tests)
cargo test -p leptos-shadcn-performance-audit

# Run specific test suites
cargo test -p leptos-shadcn-performance-audit --lib
cargo test -p leptos-shadcn-performance-audit --test performance_audit_tests

# Test CLI tool
cargo run -p leptos-shadcn-performance-audit --bin performance-audit -- --help
```

### Test Coverage
- **44 Unit Tests** - Individual module testing
- **8 Integration Tests** - End-to-end workflow testing
- **1 Documentation Test** - Example code validation
- **100% Pass Rate** - All tests passing

## 🏗️ Architecture

### Core Modules

#### `bundle_analysis`
- Component bundle size analysis
- Oversized component detection
- Bundle efficiency calculations
- Optimization recommendations

#### `performance_monitoring`
- Real-time performance metrics collection
- Render time and memory usage tracking
- Performance bottleneck detection
- Component performance scoring

#### `optimization_roadmap`
- Smart recommendation generation
- ROI-based prioritization
- Implementation planning
- Effort estimation

#### `benchmarks`
- Performance regression testing
- Benchmark comparison
- Performance trend analysis
- Automated performance validation

### Data Structures

#### `ComponentBundleAnalysis`
```rust
pub struct ComponentBundleAnalysis {
    pub component_name: String,
    pub bundle_size_bytes: u64,
    pub bundle_size_kb: f64,
    pub is_oversized: bool,
    pub performance_score: f64,
    pub optimization_recommendations: Vec<String>,
}
```

#### `ComponentPerformanceMetrics`
```rust
pub struct ComponentPerformanceMetrics {
    pub component_name: String,
    pub render_times: Vec<Duration>,
    pub memory_usage: Vec<u64>,
    pub performance_score: f64,
    pub meets_targets: bool,
}
```

#### `OptimizationRecommendation`
```rust
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

## 🔧 Configuration

### Performance Targets
Default performance targets can be customized:

```rust
pub struct PerformanceConfig {
    pub max_component_size_kb: f64,      // Default: 5.0 KB
    pub max_render_time_ms: f64,         // Default: 16.0 ms
    pub max_memory_usage_mb: f64,        // Default: 1.0 MB
    pub monitoring_enabled: bool,        // Default: true
}
```

### Custom Configuration
```bash
# Custom performance targets
performance-audit audit \
  --max-component-size-kb 3.0 \
  --max-render-time-ms 12.0 \
  --max-memory-usage-mb 0.8
```

## 📈 Use Cases

### Development
- **Performance Monitoring** - Track component performance during development
- **Bundle Size Optimization** - Identify and fix oversized components
- **Performance Regression Detection** - Catch performance issues early
- **Optimization Planning** - Plan performance improvement sprints

### Production
- **Performance Baseline** - Establish performance benchmarks
- **Monitoring Dashboards** - Generate performance reports
- **Optimization Tracking** - Measure optimization impact
- **Performance Auditing** - Regular performance health checks

### CI/CD Integration
- **Automated Performance Testing** - Include in CI/CD pipelines
- **Performance Gates** - Block deployments on performance regressions
- **Performance Reporting** - Generate automated performance reports
- **Optimization Validation** - Verify optimization effectiveness

## 🎯 Best Practices

### Performance Monitoring
1. **Regular Audits** - Run performance audits regularly
2. **Baseline Establishment** - Set performance baselines early
3. **Threshold Management** - Adjust thresholds based on requirements
4. **Trend Analysis** - Monitor performance trends over time

### Bundle Optimization
1. **Size Monitoring** - Track component sizes continuously
2. **Dependency Analysis** - Analyze and optimize dependencies
3. **Code Splitting** - Implement effective code splitting
4. **Tree Shaking** - Ensure proper tree shaking

### Optimization Planning
1. **ROI Focus** - Prioritize high-impact, low-effort optimizations
2. **Incremental Improvements** - Make small, measurable improvements
3. **Performance Budgets** - Set and enforce performance budgets
4. **Continuous Monitoring** - Monitor optimization effectiveness

## 🚀 Future Enhancements

### Planned Features
- **Real Bundle Analysis** - Analyze actual build artifacts
- **Build System Integration** - Integrate with build systems
- **Performance Regression Detection** - Automated regression detection
- **Performance Dashboards** - Web-based performance dashboards

### Community Contributions
- **Custom Optimizers** - Plugin system for custom optimizations
- **Performance Plugins** - Extensible performance monitoring
- **CI/CD Integration** - Enhanced CI/CD pipeline integration
- **Performance Metrics** - Additional performance metrics

## 📚 API Reference

### Core Functions

#### `run_performance_audit`
```rust
pub async fn run_performance_audit(
    config: PerformanceConfig
) -> Result<PerformanceResults, PerformanceAuditError>
```

#### `BundleAnalyzer`
```rust
pub struct BundleAnalyzer {
    pub components_path: PathBuf,
}

impl BundleAnalyzer {
    pub async fn analyze_all_components(&self) -> BundleAnalysisResults;
    pub async fn analyze_component(&self, name: &str) -> ComponentBundleAnalysis;
}
```

#### `PerformanceMonitor`
```rust
pub struct PerformanceMonitor {
    pub config: PerformanceConfig,
    pub start_time: Option<Instant>,
    pub tracked_components: BTreeMap<String, ComponentPerformanceMetrics>,
}

impl PerformanceMonitor {
    pub fn start_monitoring(&mut self);
    pub fn stop_monitoring(&mut self) -> PerformanceMonitoringResults;
    pub fn record_render_time(&mut self, component: &str, duration: Duration);
}
```

## 🤝 Contributing

We welcome contributions to the performance audit system! Please see our [Contributing Guide](../../CONTRIBUTING.md) for details.

### Development Setup
```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-shadcn-ui.git
cd leptos-shadcn-ui

# Test the performance audit system
cargo test -p leptos-shadcn-performance-audit

# Run the CLI tool
cargo run -p leptos-shadcn-performance-audit --bin performance-audit -- --help
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

---

**🎯 Monitor, Optimize, and Scale your Leptos applications with the Performance Audit System!**
