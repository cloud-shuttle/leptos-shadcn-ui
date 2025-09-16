# 🚀 Performance Audit Quick Start Guide

**Get up and running with the Performance Audit System in 5 minutes**

## 📦 Installation

### Install the CLI Tool
```bash
cargo install leptos-shadcn-performance-audit
```

### Add to Your Project
```toml
[dependencies]
leptos-shadcn-ui = { version = "0.5.0", features = ["performance-audit"] }
```

## 🎯 Your First Performance Audit

### 1. Run a Complete Audit
```bash
performance-audit audit
```

This will:
- ✅ Analyze all component bundle sizes
- ✅ Monitor component performance
- ✅ Generate optimization recommendations
- ✅ Display results with performance score

### 2. Analyze Bundle Sizes
```bash
performance-audit bundle --components-path packages/leptos
```

### 3. Monitor Real-time Performance
```bash
performance-audit monitor --duration 30s
```

## 📊 Understanding the Output

### Performance Score
- **A (90-100)**: Excellent performance
- **B (80-89)**: Good performance
- **C (70-79)**: Acceptable performance
- **D (60-69)**: Needs improvement
- **F (0-59)**: Poor performance

### Bundle Analysis
- **Overall Efficiency**: Percentage of components meeting size targets
- **Total Size**: Combined size of all components
- **Average Size**: Mean component size
- **Oversized Components**: Components exceeding size thresholds

### Performance Monitoring
- **Render Times**: Component rendering performance
- **Memory Usage**: Memory consumption patterns
- **Failing Components**: Components not meeting performance targets

## 🛠️ Common Commands

### Basic Audits
```bash
# Quick audit with default settings
performance-audit audit

# Custom performance targets
performance-audit audit \
  --max-component-size-kb 3.0 \
  --max-render-time-ms 12.0 \
  --max-memory-usage-mb 0.8
```

### Output Formats
```bash
# Text output (default)
performance-audit audit

# JSON output for automation
performance-audit audit --format json

# HTML report
performance-audit audit --format html --output report.html

# Markdown documentation
performance-audit audit --format markdown --output report.md
```

### Specific Analysis
```bash
# Bundle size analysis only
performance-audit bundle

# Performance monitoring only
performance-audit monitor --duration 60s

# Generate optimization roadmap
performance-audit roadmap --output roadmap.json
```

## 🎯 Optimization Workflow

### 1. Establish Baseline
```bash
# Run initial audit
performance-audit audit --output baseline.json --format json
```

### 2. Identify Issues
```bash
# Focus on oversized components
performance-audit bundle --target-size-kb 3.0
```

### 3. Monitor Improvements
```bash
# Track performance over time
performance-audit monitor --duration 120s --sample-rate 50ms
```

### 4. Generate Roadmap
```bash
# Get optimization recommendations
performance-audit roadmap --input baseline.json --output roadmap.md
```

## 🔧 Configuration

### Custom Performance Targets
Create a configuration file or use command-line options:

```bash
performance-audit audit \
  --max-component-size-kb 4.0 \
  --max-render-time-ms 16.0 \
  --max-memory-usage-mb 1.0
```

### Component Path Configuration
```bash
# Analyze specific component directory
performance-audit audit --components-path src/components

# Analyze multiple directories
performance-audit audit --components-path packages/leptos
```

## 📈 Integration Examples

### CI/CD Pipeline
```yaml
# GitHub Actions example
- name: Performance Audit
  run: |
    cargo install leptos-shadcn-performance-audit
    performance-audit audit --format json --output audit-results.json
    
- name: Check Performance
  run: |
    # Check if performance meets targets
    if ! performance-audit audit --format json | jq '.meets_targets'; then
      echo "Performance audit failed!"
      exit 1
    fi
```

### Development Workflow
```bash
# Pre-commit performance check
performance-audit audit --max-component-size-kb 5.0

# Post-optimization validation
performance-audit audit --format json --output after-optimization.json
```

### Monitoring Dashboard
```bash
# Generate HTML report for dashboard
performance-audit audit --format html --output dashboard.html

# JSON data for custom dashboards
performance-audit audit --format json --output metrics.json
```

## 🚨 Troubleshooting

### Common Issues

#### "Command not found"
```bash
# Ensure the tool is installed
cargo install leptos-shadcn-performance-audit

# Check installation
which performance-audit
```

#### "No components found"
```bash
# Specify correct component path
performance-audit audit --components-path packages/leptos

# Check directory structure
ls -la packages/leptos/
```

#### "Performance targets not met"
```bash
# Adjust targets based on requirements
performance-audit audit --max-component-size-kb 8.0

# Focus on specific optimizations
performance-audit roadmap --output recommendations.md
```

### Getting Help
```bash
# Show help for any command
performance-audit --help
performance-audit audit --help
performance-audit bundle --help
performance-audit monitor --help
performance-audit roadmap --help
```

## 🎉 Next Steps

### Advanced Usage
- **[Complete Documentation](README.md)** - Full system documentation
- **[API Reference](API.md)** - Programmatic usage
- **[Integration Guide](INTEGRATION.md)** - CI/CD and tool integration
- **[Best Practices](BEST_PRACTICES.md)** - Optimization strategies

### Community
- **[GitHub Issues](https://github.com/cloud-shuttle/leptos-shadcn-ui/issues)** - Report bugs or request features
- **[Discussions](https://github.com/cloud-shuttle/leptos-shadcn-ui/discussions)** - Community discussions
- **[Examples](https://github.com/cloud-shuttle/leptos-shadcn-ui/tree/main/examples)** - Working examples

---

**🎯 You're now ready to monitor and optimize your Leptos application performance!**

**Next: Try running your first audit and explore the optimization recommendations.**
