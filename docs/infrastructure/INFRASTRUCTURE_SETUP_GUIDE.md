# 🚀 Infrastructure Setup Guide

> **Complete setup guide for Phase 2 infrastructure systems**

## 📋 Prerequisites

### System Requirements

- **Operating System**: macOS, Linux, or Windows
- **Rust**: 1.70+ with WASM target
- **Node.js**: 18+ with pnpm
- **Git**: Latest version
- **Memory**: 8GB+ RAM recommended
- **Storage**: 10GB+ free space

### Required Tools

```bash
# Install Rust with WASM target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Install Node.js (via nvm recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Install pnpm
npm install -g pnpm

# Install wasm-pack (for WASM builds)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## 🏗️ Installation Steps

### 1. Clone and Setup Project

```bash
# Clone the repository
git clone https://github.com/your-org/leptos-shadcn-ui.git
cd leptos-shadcn-ui

# Install dependencies
pnpm install

# Install Playwright browsers
pnpm playwright install
```

### 2. Build Infrastructure Components

```bash
# Build performance audit system
cd performance-audit
cargo build --release
cd ..

# Build WASM test application
cd minimal-wasm-test
wasm-pack build --target web --out-dir pkg
cd ..

# Build main application
cargo build --workspace
```

### 3. Verify Installation

```bash
# Run basic tests to verify setup
make test

# Check WASM functionality
make test-wasm

# Verify E2E tests
make test-e2e-enhanced

# Check performance benchmarks
make benchmark

# Verify accessibility tests
make accessibility-audit
```

## 🔧 Configuration

### Environment Setup

Create a `.env` file in the project root:

```bash
# Performance thresholds
WASM_MAX_INIT_TIME=5000
WASM_MAX_FIRST_PAINT=3000
WASM_MAX_FCP=4000
WASM_MAX_INTERACTION_LATENCY=100

# Browser selection
WASM_ENABLED_BROWSERS="chromium,firefox,webkit"

# WCAG compliance level
WCAG_LEVEL="AA"

# CI/CD settings (for CI environments)
CI=false
SLACK_WEBHOOK_URL=""
EMAIL_RECIPIENTS=""
```

### Playwright Configuration

The Playwright configuration is automatically set up in `playwright.config.ts`. Key settings:

```typescript
// Browser-specific timeouts
const browserConfigs = {
  chromium: { timeout: 30000, retries: 2 },
  firefox: { timeout: 35000, retries: 2 },
  webkit: { timeout: 40000, retries: 3 },
};

// Performance thresholds
const PERFORMANCE_THRESHOLDS = {
  maxInitializationTime: 5000,
  maxFirstPaint: 3000,
  maxFirstContentfulPaint: 4000,
  maxInteractionLatency: 100,
};
```

### Performance Benchmarking Configuration

Configure performance thresholds in `performance-audit/src/regression_testing.rs`:

```rust
let config = RegressionTestConfig {
    baseline_path: "performance-baseline.json".to_string(),
    results_path: "regression-results.json".to_string(),
    thresholds: RegressionThresholds {
        minor_threshold: 5.0,
        moderate_threshold: 15.0,
        major_threshold: 30.0,
        critical_threshold: 50.0,
    },
    auto_update_baseline: false,
    generate_recommendations: true,
};
```

### Accessibility Configuration

Configure accessibility testing in `tests/e2e/accessibility-automation.ts`:

```typescript
const config: AccessibilityConfig = {
  wcagLevel: WCAGLevel.AA,
  includeScreenReaderTests: true,
  includeKeyboardNavigationTests: true,
  includeColorContrastTests: true,
  includeFocusManagementTests: true,
  customRules: [],
  thresholds: {
    maxViolations: 10,
    maxCriticalViolations: 0,
    maxSeriousViolations: 2,
    minColorContrastRatio: 4.5,
    maxFocusableElementsWithoutLabels: 0,
  },
};
```

## 🧪 Testing Setup

### 1. WASM Testing

```bash
# Run all WASM tests
make test-wasm

# Run on specific browsers
make test-wasm-browsers BROWSERS=chromium,firefox

# Run in headed mode for debugging
make test-wasm-headed

# Run in parallel for faster execution
make test-wasm-parallel
```

### 2. E2E Testing

```bash
# Run enhanced E2E tests
make test-e2e-enhanced

# Run in CI mode
make test-e2e-ci

# Run in debug mode
make test-e2e-debug

# Run specific test categories
make test-e2e-performance
make test-e2e-accessibility
make test-e2e-wasm
```

### 3. Performance Benchmarking

```bash
# Run performance benchmarks
make benchmark

# Run for specific components
make benchmark-components COMPONENTS=button,input

# Run regression tests
make regression-test

# Setup performance baseline
make setup-baseline

# Start automated monitoring
make performance-monitor
```

### 4. Accessibility Testing

```bash
# Run comprehensive accessibility audit
make accessibility-audit

# Run with specific WCAG level
make accessibility-audit-wcag LEVEL=AAA

# Run for specific components
make accessibility-audit-components COMPONENTS=button,input

# Generate HTML report
make accessibility-audit-html
```

## 🚀 CI/CD Setup

### GitHub Actions

The project includes a comprehensive GitHub Actions workflow. To set it up:

1. **Enable GitHub Actions** in your repository settings
2. **Add Secrets** for notifications:
   - `SLACK_WEBHOOK_URL`: Slack webhook for notifications
   - `EMAIL_USERNAME`: Email username for notifications
   - `EMAIL_PASSWORD`: Email password for notifications

3. **Configure Environment Variables**:
   ```yaml
   env:
     WASM_MAX_INIT_TIME: 8000
     WCAG_LEVEL: AA
     CI: true
   ```

### Local CI Simulation

```bash
# Simulate CI environment locally
CI=true make test-e2e-ci
CI=true make test-wasm
CI=true make benchmark
CI=true make accessibility-audit
```

## 📊 Monitoring Setup

### Performance Monitoring

```bash
# Start automated performance monitoring
make performance-monitor

# Start with alerts enabled
make performance-monitor-alerts

# Monitor specific components
./scripts/run-performance-benchmarks.sh monitor -c button,input -a
```

### Accessibility Monitoring

```bash
# Run accessibility audit with monitoring
make accessibility-audit-verbose

# Generate comprehensive report
make accessibility-audit-html
```

## 🔍 Debugging Setup

### Debug Mode

```bash
# Run tests in debug mode
make test-e2e-debug

# Run WASM tests in headed mode
make test-wasm-headed

# Run with verbose output
make test-wasm-verbose
make accessibility-audit-verbose
```

### Browser DevTools

```bash
# Open Playwright inspector
pnpm playwright test --debug

# Run specific test in debug mode
pnpm playwright test --debug --grep "should initialize WASM successfully"
```

### Log Analysis

```bash
# Check test results
ls -la test-results/

# View HTML reports
open test-results/html-report/index.html

# Check performance results
cat test-results/performance/benchmark-results.json

# View accessibility report
open test-results/accessibility/accessibility-report.html
```

## 🛠️ Development Workflow

### Daily Development

```bash
# Start development server
cd examples/leptos
trunk serve --port 8082 &

# Run quick tests
make test

# Run specific component tests
make test-wasm-browsers BROWSERS=chromium
make accessibility-audit-components COMPONENTS=button
```

### Pre-commit Testing

```bash
# Run all tests before commit
make test
make test-wasm
make test-e2e-enhanced
make benchmark
make accessibility-audit
```

### Release Testing

```bash
# Run comprehensive test suite
make test
make test-wasm-parallel
make test-e2e-ci
make regression-test
make accessibility-audit-wcag LEVEL=AA
```

## 📈 Performance Optimization

### Bundle Optimization

```bash
# Analyze bundle sizes
make analyze-bundle

# Run performance benchmarks
make benchmark-html

# Check for performance regressions
make regression-test
```

### Memory Optimization

```bash
# Run memory tests
make test-wasm-verbose

# Monitor memory usage
make performance-monitor
```

## ♿ Accessibility Compliance

### WCAG Compliance

```bash
# Run WCAG AA compliance tests
make accessibility-audit

# Run WCAG AAA compliance tests
make accessibility-audit-wcag LEVEL=AAA

# Focus on specific areas
make accessibility-audit-focus
```

### Screen Reader Testing

```bash
# Run screen reader tests
make accessibility-audit --no-keyboard-nav --no-color-contrast

# Test keyboard navigation
make accessibility-audit --no-screen-reader --no-color-contrast
```

## 🔧 Troubleshooting

### Common Issues

#### 1. WASM Build Failures
```bash
# Check Rust version
rustc --version

# Check WASM target
rustup target list --installed | grep wasm32-unknown-unknown

# Reinstall WASM target
rustup target add wasm32-unknown-unknown

# Clean and rebuild
cargo clean
cargo build --workspace
```

#### 2. Playwright Issues
```bash
# Reinstall Playwright browsers
pnpm playwright install

# Check browser installation
pnpm playwright --version

# Run with system browsers
pnpm playwright test --project=chromium
```

#### 3. Performance Test Failures
```bash
# Check performance baseline
ls -la test-results/performance/

# Update baseline
make setup-baseline

# Run with verbose output
make benchmark-verbose
```

#### 4. Accessibility Test Failures
```bash
# Run with verbose output
make accessibility-audit-verbose

# Check specific components
make accessibility-audit-components COMPONENTS=button

# Generate detailed report
make accessibility-audit-html
```

### Debug Commands

```bash
# Check system resources
free -h
df -h

# Check running processes
ps aux | grep -E "(playwright|chromium|firefox)"

# Check network connectivity
curl -I http://localhost:8082
```

## 📚 Additional Resources

### Documentation

- [WASM Testing Guide](WASM_TESTING_GUIDE.md)
- [E2E Testing Guide](E2E_TESTING_GUIDE.md)
- [Performance Benchmarking Guide](PERFORMANCE_BENCHMARKING_GUIDE.md)
- [Accessibility Testing Guide](ACCESSIBILITY_TESTING_GUIDE.md)

### External Resources

- [Playwright Documentation](https://playwright.dev/)
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [Leptos Documentation](https://leptos.dev/)

### Community

- [GitHub Issues](https://github.com/your-org/leptos-shadcn-ui/issues)
- [Discord Community](https://discord.gg/leptos)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/leptos)

---

**Last Updated**: December 2024  
**Version**: 2.0.0  
**Maintainer**: leptos-shadcn-ui Team
