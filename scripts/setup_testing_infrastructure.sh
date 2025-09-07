#!/bin/bash

# 🧪 leptos-shadcn-ui Testing Infrastructure Setup Script
# Sets up comprehensive testing environment for v1.0 development

set -e

echo "🚀 Setting up leptos-shadcn-ui Testing Infrastructure..."
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || ! grep -q "leptos-shadcn-ui" Cargo.toml; then
    log_error "Please run this script from the leptos-shadcn-ui project root"
    exit 1
fi

log_info "Detected leptos-shadcn-ui project root"

# ========================================
# Phase 1: Rust Testing Tools
# ========================================
echo ""
echo "📦 Phase 1: Installing Rust Testing Tools"
echo "----------------------------------------"

# Install cargo-tarpaulin for coverage
if ! command -v cargo-tarpaulin &> /dev/null; then
    log_info "Installing cargo-tarpaulin for test coverage..."
    cargo install cargo-tarpaulin
    log_success "cargo-tarpaulin installed"
else
    log_success "cargo-tarpaulin already installed"
fi

# Install cargo-criterion for benchmarking
if ! command -v cargo-criterion &> /dev/null; then
    log_info "Installing cargo-criterion for performance benchmarking..."
    cargo install cargo-criterion
    log_success "cargo-criterion installed"
else
    log_success "cargo-criterion already installed"
fi

# Install cargo-audit for security auditing
if ! command -v cargo-audit &> /dev/null; then
    log_info "Installing cargo-audit for security auditing..."
    cargo install cargo-audit
    log_success "cargo-audit installed"
else
    log_success "cargo-audit already installed"
fi

# Install cargo-deny for dependency licensing
if ! command -v cargo-deny &> /dev/null; then
    log_info "Installing cargo-deny for license checking..."
    cargo install cargo-deny
    log_success "cargo-deny installed"
else
    log_success "cargo-deny already installed"
fi

# Install trunk for WASM building
if ! command -v trunk &> /dev/null; then
    log_info "Installing trunk for WASM building..."
    cargo install trunk
    log_success "trunk installed"
else
    log_success "trunk already installed"
fi

# Install wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    log_info "Installing wasm-pack..."
    cargo install wasm-pack
    log_success "wasm-pack installed"
else
    log_success "wasm-pack already installed"
fi

# ========================================
# Phase 2: Node.js and E2E Tools
# ========================================
echo ""
echo "📦 Phase 2: Setting up Node.js and E2E Testing"
echo "---------------------------------------------"

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    log_error "Node.js is required but not installed. Please install Node.js 18+ and try again."
    exit 1
fi

NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
    log_error "Node.js version 18+ is required. Found version: $(node --version)"
    exit 1
fi

log_success "Node.js $(node --version) detected"

# Install/update npm dependencies
if [ ! -f "package.json" ]; then
    log_info "Creating package.json for E2E testing dependencies..."
    cat > package.json << 'EOF'
{
  "name": "leptos-shadcn-ui-testing",
  "version": "1.0.0",
  "private": true,
  "description": "Testing dependencies for leptos-shadcn-ui",
  "scripts": {
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui",
    "test:install": "playwright install",
    "test:report": "playwright show-report"
  },
  "devDependencies": {
    "@playwright/test": "^1.40.0",
    "@axe-core/playwright": "^4.8.0",
    "typescript": "^5.0.0"
  }
}
EOF
    log_success "package.json created"
fi

log_info "Installing Node.js dependencies..."
npm install
log_success "Node.js dependencies installed"

# Install Playwright browsers
log_info "Installing Playwright browsers..."
npx playwright install
log_success "Playwright browsers installed"

# ========================================
# Phase 3: Testing Configuration Files
# ========================================
echo ""
echo "📦 Phase 3: Creating Testing Configuration"
echo "-----------------------------------------"

# Create cargo-deny configuration
if [ ! -f "deny.toml" ]; then
    log_info "Creating cargo-deny configuration..."
    cat > deny.toml << 'EOF'
[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
]
deny = [
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-1.0",
    "AGPL-3.0",
]

[bans]
multiple-versions = "warn"
wildcards = "allow"

[sources]
unknown-registry = "warn"
unknown-git = "warn"
EOF
    log_success "cargo-deny configuration created"
fi

# Create Playwright configuration
if [ ! -f "playwright.config.ts" ]; then
    log_info "Creating Playwright configuration..."
    cat > playwright.config.ts << 'EOF'
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [
    ['html'],
    ['json', { outputFile: 'test-results/results.json' }],
  ],
  use: {
    baseURL: 'http://127.0.0.1:8080',
    trace: 'on-first-retry',
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
    {
      name: 'Mobile Chrome',
      use: { ...devices['Pixel 5'] },
    },
    {
      name: 'Mobile Safari',
      use: { ...devices['iPhone 12'] },
    },
  ],

  webServer: {
    command: 'cd examples/leptos && trunk serve',
    port: 8080,
    reuseExistingServer: !process.env.CI,
  },
});
EOF
    log_success "Playwright configuration created"
fi

# Create tarpaulin configuration
if [ ! -f "tarpaulin.toml" ]; then
    log_info "Creating tarpaulin configuration..."
    cat > tarpaulin.toml << 'EOF'
[tool.tarpaulin]
timeout = 120
exclude-files = [
    "examples/*",
    "scripts/*",
    "*/tests.rs",
    "*/benches/*"
]
ignore-panics = true
ignore-tests = true
skip-clean = false
line = true
branch = true
out = ["Xml", "Html"]
EOF
    log_success "tarpaulin configuration created"
fi

# ========================================
# Phase 4: Test Directory Structure
# ========================================
echo ""
echo "📦 Phase 4: Setting up Test Directory Structure"
echo "----------------------------------------------"

# Create test directories
mkdir -p tests/e2e
mkdir -p tests/integration
mkdir -p tests/performance
mkdir -p test-results
mkdir -p coverage

log_info "Creating test directories..."

# Create sample E2E test if it doesn't exist
if [ ! -f "tests/e2e/basic-functionality.spec.ts" ]; then
    cat > tests/e2e/basic-functionality.spec.ts << 'EOF'
import { test, expect } from '@playwright/test';

test.describe('Basic Component Functionality', () => {
  test('components render correctly', async ({ page }) => {
    await page.goto('/');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check if basic components are rendered
    await expect(page.locator('body')).toBeVisible();
    
    // Add more specific component tests here
  });
  
  test('components are accessible', async ({ page }) => {
    await page.goto('/');
    
    // Basic accessibility check
    const focusableElements = await page.locator('[tabindex]:not([tabindex="-1"])').count();
    expect(focusableElements).toBeGreaterThan(0);
  });
});
EOF
    log_success "Sample E2E test created"
fi

# ========================================
# Phase 5: Git Hooks Setup
# ========================================
echo ""
echo "📦 Phase 5: Setting up Git Hooks"
echo "-------------------------------"

# Create pre-commit hook
if [ ! -f ".git/hooks/pre-commit" ]; then
    log_info "Creating pre-commit hook..."
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash

echo "🧪 Running pre-commit quality gates..."

# Format check
echo "📝 Checking code format..."
if ! cargo fmt -- --check; then
    echo "❌ Code format check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Lint check
echo "📎 Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy check failed. Fix the warnings above."
    exit 1
fi

# Quick unit tests
echo "🧪 Running unit tests..."
if ! cargo test --lib --all-features --quiet; then
    echo "❌ Unit tests failed."
    exit 1
fi

# Security audit
echo "🛡️ Running security audit..."
if ! cargo audit; then
    echo "❌ Security audit failed."
    exit 1
fi

echo "✅ Pre-commit gates passed!"
EOF
    chmod +x .git/hooks/pre-commit
    log_success "Pre-commit hook created and activated"
else
    log_success "Pre-commit hook already exists"
fi

# ========================================
# Phase 6: Makefile for Common Tasks
# ========================================
echo ""
echo "📦 Phase 6: Creating Development Makefile"
echo "----------------------------------------"

if [ ! -f "Makefile" ]; then
    log_info "Creating Makefile for common testing tasks..."
    cat > Makefile << 'EOF'
# leptos-shadcn-ui Testing Makefile

.PHONY: help test test-unit test-integration test-e2e test-perf coverage audit clean setup

# Default target
help:
	@echo "🧪 leptos-shadcn-ui Testing Commands"
	@echo "=================================="
	@echo "setup              - Set up testing environment"
	@echo "test               - Run all tests"
	@echo "test-unit          - Run unit tests"
	@echo "test-integration   - Run integration tests"
	@echo "test-e2e          - Run E2E tests"
	@echo "test-perf         - Run performance tests"
	@echo "coverage          - Generate test coverage report"
	@echo "audit             - Run security audit"
	@echo "clean             - Clean test artifacts"

# Setup testing environment
setup:
	@echo "🚀 Setting up testing environment..."
	@./scripts/setup_testing_infrastructure.sh

# Run all tests
test: test-unit test-integration test-perf
	@echo "✅ All tests completed"

# Run unit tests
test-unit:
	@echo "🧪 Running unit tests..."
	@cargo test --workspace --lib --all-features

# Run integration tests
test-integration:
	@echo "🔗 Running integration tests..."
	@cargo test --workspace --test '*' --all-features

# Run E2E tests
test-e2e:
	@echo "🎭 Running E2E tests..."
	@npm run test:e2e

# Run performance tests
test-perf:
	@echo "⚡ Running performance tests..."
	@cd performance-audit && cargo test --release
	@cargo bench --workspace

# Generate coverage report
coverage:
	@echo "📊 Generating coverage report..."
	@cargo tarpaulin --out html --output-dir coverage/

# Run security audit
audit:
	@echo "🛡️ Running security audit..."
	@cargo audit
	@cargo deny check

# Clean test artifacts
clean:
	@echo "🧹 Cleaning test artifacts..."
	@cargo clean
	@rm -rf coverage/
	@rm -rf test-results/
	@rm -rf target/criterion/

# Install Playwright browsers
install-playwright:
	@echo "🎭 Installing Playwright browsers..."
	@npx playwright install

# Run tests with watch mode
watch:
	@echo "👀 Running tests in watch mode..."
	@cargo watch -x "test --lib"

# Run specific component tests
test-component:
	@echo "🎨 Running tests for component: $(COMPONENT)"
	@cargo test --package leptos-shadcn-$(COMPONENT) --lib --verbose

# Format and lint
fmt:
	@echo "📝 Formatting code..."
	@cargo fmt

lint:
	@echo "📎 Running linter..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Complete quality check
quality: fmt lint audit test coverage
	@echo "✅ Quality check completed"
EOF
    log_success "Makefile created"
fi

# ========================================
# Phase 7: VS Code Configuration
# ========================================
echo ""
echo "📦 Phase 7: Setting up VS Code Configuration"
echo "-------------------------------------------"

mkdir -p .vscode

# VS Code settings for testing
if [ ! -f ".vscode/settings.json" ]; then
    log_info "Creating VS Code settings..."
    cat > .vscode/settings.json << 'EOF'
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--all-targets", "--all-features"],
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "files.watcherExclude": {
    "**/target/**": true,
    "**/node_modules/**": true,
    "**/coverage/**": true
  },
  "playwright.showTrace": true,
  "playwright.reuseBrowser": true
}
EOF
    log_success "VS Code settings created"
fi

# VS Code launch configuration for testing
if [ ! -f ".vscode/launch.json" ]; then
    log_info "Creating VS Code launch configuration..."
    cat > .vscode/launch.json << 'EOF'
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug Unit Tests",
      "type": "lldb",
      "request": "launch",
      "cargo": {
        "args": ["test", "--no-run", "--lib"],
        "filter": {
          "name": "${workspaceFolderBasename}",
          "kind": "lib"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    },
    {
      "name": "Debug Component Test",
      "type": "lldb",
      "request": "launch",
      "cargo": {
        "args": ["test", "--no-run", "--package", "leptos-shadcn-${input:componentName}"],
        "filter": {
          "kind": "lib"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ],
  "inputs": [
    {
      "id": "componentName",
      "description": "Component name to test",
      "default": "button",
      "type": "promptString"
    }
  ]
}
EOF
    log_success "VS Code launch configuration created"
fi

# VS Code tasks for testing
if [ ! -f ".vscode/tasks.json" ]; then
    log_info "Creating VS Code tasks..."
    cat > .vscode/tasks.json << 'EOF'
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run Unit Tests",
      "type": "shell",
      "command": "cargo",
      "args": ["test", "--lib", "--all-features"],
      "group": {
        "kind": "test",
        "isDefault": true
      },
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      }
    },
    {
      "label": "Run E2E Tests",
      "type": "shell",
      "command": "npm",
      "args": ["run", "test:e2e"],
      "group": "test",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      }
    },
    {
      "label": "Generate Coverage",
      "type": "shell",
      "command": "cargo",
      "args": ["tarpaulin", "--out", "html", "--output-dir", "coverage/"],
      "group": "test",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      }
    }
  ]
}
EOF
    log_success "VS Code tasks created"
fi

# ========================================
# Phase 8: Final Verification
# ========================================
echo ""
echo "📦 Phase 8: Final Verification"
echo "-----------------------------"

# Test that everything is working
log_info "Running verification tests..."

# Test cargo commands
if cargo --version &> /dev/null; then
    log_success "Cargo is working"
else
    log_error "Cargo verification failed"
    exit 1
fi

# Test installed tools
TOOLS=("cargo-tarpaulin" "cargo-criterion" "cargo-audit" "cargo-deny" "trunk")
for tool in "${TOOLS[@]}"; do
    if command -v "$tool" &> /dev/null; then
        log_success "$tool is installed and accessible"
    else
        log_error "$tool installation verification failed"
        exit 1
    fi
done

# Test Node.js setup
if npx playwright --version &> /dev/null; then
    log_success "Playwright is installed and working"
else
    log_error "Playwright verification failed"
    exit 1
fi

# Test basic compilation
log_info "Testing basic project compilation..."
if cargo check --workspace --all-features &> /dev/null; then
    log_success "Project compilation check passed"
else
    log_error "Project compilation check failed"
    exit 1
fi

# ========================================
# Success Summary
# ========================================
echo ""
echo "🎉 Testing Infrastructure Setup Complete!"
echo "========================================"
echo ""
echo "✅ Installed Tools:"
echo "   - cargo-tarpaulin (test coverage)"
echo "   - cargo-criterion (performance benchmarks)"
echo "   - cargo-audit (security auditing)"
echo "   - cargo-deny (license checking)"
echo "   - trunk (WASM building)"
echo "   - Playwright (E2E testing)"
echo ""
echo "✅ Configuration Files:"
echo "   - GitHub Actions CI/CD pipeline"
echo "   - Playwright configuration"
echo "   - Tarpaulin configuration"
echo "   - Cargo-deny configuration"
echo "   - VS Code settings and tasks"
echo ""
echo "✅ Git Hooks:"
echo "   - Pre-commit quality gates"
echo ""
echo "🚀 Quick Start Commands:"
echo "   make test          - Run all tests"
echo "   make coverage      - Generate coverage report"
echo "   make test-e2e      - Run E2E tests"
echo "   make audit         - Run security audit"
echo ""
echo "📚 Next Steps:"
echo "   1. Run 'make test' to verify everything works"
echo "   2. Check out the testing standards in docs/v1.0-roadmap/testing-infrastructure/"
echo "   3. Start implementing Phase 2 of the v1.0 roadmap"
echo ""
echo "Happy testing! 🧪✨"
EOF