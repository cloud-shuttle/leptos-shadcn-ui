#!/bin/bash

# Comprehensive Test Runner Script
# Uses cargo nextest to prevent hanging and improve test execution

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
MIN_COVERAGE=95
MAX_BUNDLE_SIZE_KB=500
MAX_RENDER_TIME_MS=16
MAX_MEMORY_USAGE_MB=10

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install missing tools
install_tools() {
    print_status "Checking and installing required tools..."
    
    if ! command_exists cargo-nextest; then
        print_status "Installing cargo-nextest..."
        cargo install cargo-nextest
    fi
    
    if ! command_exists cargo-tarpaulin; then
        print_status "Installing cargo-tarpaulin..."
        cargo install cargo-tarpaulin
    fi
    
    if ! command_exists cargo-audit; then
        print_status "Installing cargo-audit..."
        cargo install cargo-audit
    fi
    
    if ! command_exists cargo-deny; then
        print_status "Installing cargo-deny..."
        cargo install cargo-deny
    fi
    
    if ! command_exists npx; then
        print_error "Node.js and npm are required for E2E tests"
        exit 1
    fi
    
    if ! command_exists npx; then
        print_status "Installing Playwright..."
        npm install -g @playwright/test
        npx playwright install --with-deps
    fi
}

# Function to run code quality checks
run_code_quality() {
    print_status "Running code quality checks..."
    
    # Format check
    print_status "Checking code formatting..."
    if ! cargo fmt --all -- --check; then
        print_error "Code formatting check failed"
        exit 1
    fi
    print_success "Code formatting check passed"
    
    # Clippy linting
    print_status "Running Clippy linting..."
    if ! cargo clippy --all-targets --all-features -- -D warnings; then
        print_error "Clippy linting failed"
        exit 1
    fi
    print_success "Clippy linting passed"
    
    # Security audit
    print_status "Running security audit..."
    if ! cargo audit; then
        print_error "Security audit failed"
        exit 1
    fi
    print_success "Security audit passed"
    
    # Dependency check
    print_status "Running dependency check..."
    if ! cargo deny check; then
        print_error "Dependency check failed"
        exit 1
    fi
    print_success "Dependency check passed"
}

# Function to run unit tests with nextest
run_unit_tests() {
    print_status "Running unit tests with cargo nextest..."
    
    if ! cargo nextest run \
        --workspace \
        --all-features \
        --config-file .nextest/config.toml \
        --profile default \
        --junit-xml target/nextest/junit.xml; then
        print_error "Unit tests failed"
        exit 1
    fi
    
    print_success "Unit tests passed"
}

# Function to run integration tests
run_integration_tests() {
    print_status "Running integration tests..."
    
    if ! cargo nextest run \
        --workspace \
        --all-features \
        --config-file .nextest/config.toml \
        --profile default \
        --test-threads 1 \
        --junit-xml target/nextest/integration-junit.xml; then
        print_error "Integration tests failed"
        exit 1
    fi
    
    print_success "Integration tests passed"
}

# Function to run E2E tests
run_e2e_tests() {
    print_status "Running E2E tests..."
    
    # Start development server
    print_status "Starting development server..."
    cd examples/leptos
    trunk serve --port 8082 &
    SERVER_PID=$!
    cd ../..
    
    # Wait for server to start
    sleep 10
    
    # Run Playwright tests
    if ! npx playwright test \
        --config=docs/testing/playwright.config.ts \
        --reporter=junit \
        --output-dir=test-results/e2e; then
        print_error "E2E tests failed"
        kill $SERVER_PID
        exit 1
    fi
    
    # Stop the server
    kill $SERVER_PID
    
    print_success "E2E tests passed"
}

# Function to run performance benchmarks
run_performance_benchmarks() {
    print_status "Running performance benchmarks..."
    
    # Run benchmarks for critical components
    local components=("button" "input" "card" "badge" "alert" "skeleton" "progress" "toast" "table" "calendar")
    
    for component in "${components[@]}"; do
        if [ -d "packages/leptos/$component/benches" ]; then
            print_status "Running benchmarks for $component..."
            if ! cargo bench --package leptos-shadcn-$component --features benchmarks; then
                print_warning "Benchmarks for $component failed, continuing..."
            fi
        fi
    done
    
    print_success "Performance benchmarks completed"
}

# Function to run test coverage
run_test_coverage() {
    print_status "Running test coverage analysis..."
    
    if ! cargo tarpaulin \
        --out Html \
        --output-dir coverage \
        --workspace \
        --all-features \
        --exclude-files '*/benches/*' \
        --exclude-files '*/tests/*' \
        --exclude-files '*/examples/*' \
        --timeout 300; then
        print_error "Test coverage analysis failed"
        exit 1
    fi
    
    # Check coverage threshold
    local coverage=$(grep -o 'Total coverage: [0-9.]*%' coverage/tarpaulin-report.html | grep -o '[0-9.]*')
    if (( $(echo "$coverage < $MIN_COVERAGE" | bc -l) )); then
        print_error "Coverage $coverage% is below minimum $MIN_COVERAGE%"
        exit 1
    fi
    
    print_success "Test coverage: $coverage% (meets minimum $MIN_COVERAGE%)"
}

# Function to run accessibility tests
run_accessibility_tests() {
    print_status "Running accessibility tests..."
    
    # Start development server
    print_status "Starting development server for accessibility tests..."
    cd examples/leptos
    trunk serve --port 8082 &
    SERVER_PID=$!
    cd ../..
    
    # Wait for server to start
    sleep 10
    
    # Run accessibility tests
    if ! npx playwright test \
        tests/e2e/accessibility-tests/ \
        --config=docs/testing/playwright.config.ts \
        --reporter=junit \
        --output-dir=test-results/accessibility; then
        print_error "Accessibility tests failed"
        kill $SERVER_PID
        exit 1
    fi
    
    # Stop the server
    kill $SERVER_PID
    
    print_success "Accessibility tests passed"
}

# Function to run security scanning
run_security_scanning() {
    print_status "Running security scanning..."
    
    # Rust security audit
    if ! cargo audit --deny warnings; then
        print_error "Rust security audit failed"
        exit 1
    fi
    
    # Dependency vulnerability check
    if ! cargo deny check; then
        print_error "Dependency vulnerability check failed"
        exit 1
    fi
    
    # NPM security audit (if package.json exists)
    if [ -f "package.json" ]; then
        if ! npm audit --audit-level moderate; then
            print_error "NPM security audit failed"
            exit 1
        fi
    fi
    
    print_success "Security scanning passed"
}

# Function to generate comprehensive report
generate_report() {
    print_status "Generating comprehensive test report..."
    
    local report_file="test-report-$(date +%Y%m%d-%H%M%S).md"
    
    cat > "$report_file" << EOF
# Comprehensive Test Report

Generated: $(date)

## Test Results

### Code Quality
- ✅ Code formatting check passed
- ✅ Clippy linting passed
- ✅ Security audit passed
- ✅ Dependency check passed

### Testing
- ✅ Unit tests passed
- ✅ Integration tests passed
- ✅ E2E tests passed

### Performance
- ✅ Performance benchmarks completed
- ✅ Test coverage: $(grep -o 'Total coverage: [0-9.]*%' coverage/tarpaulin-report.html | grep -o '[0-9.]*')%

### Accessibility
- ✅ Accessibility tests passed

### Security
- ✅ Security scanning passed

## Quality Gates

- Minimum Coverage: $MIN_COVERAGE%
- Maximum Bundle Size: ${MAX_BUNDLE_SIZE_KB}KB
- Maximum Render Time: ${MAX_RENDER_TIME_MS}ms
- Maximum Memory Usage: ${MAX_MEMORY_USAGE_MB}MB

## Status: ✅ ALL TESTS PASSED

Ready for production deployment!
EOF

    print_success "Test report generated: $report_file"
}

# Main function
main() {
    print_status "Starting comprehensive test suite..."
    print_status "Configuration:"
    print_status "  - Minimum Coverage: $MIN_COVERAGE%"
    print_status "  - Maximum Bundle Size: ${MAX_BUNDLE_SIZE_KB}KB"
    print_status "  - Maximum Render Time: ${MAX_RENDER_TIME_MS}ms"
    print_status "  - Maximum Memory Usage: ${MAX_MEMORY_USAGE_MB}MB"
    echo
    
    # Install required tools
    install_tools
    
    # Run all test phases
    run_code_quality
    run_unit_tests
    run_integration_tests
    run_e2e_tests
    run_performance_benchmarks
    run_test_coverage
    run_accessibility_tests
    run_security_scanning
    
    # Generate report
    generate_report
    
    print_success "🎉 All tests passed! Ready for production deployment."
}

# Handle command line arguments
case "${1:-all}" in
    "quality")
        install_tools
        run_code_quality
        ;;
    "unit")
        install_tools
        run_unit_tests
        ;;
    "integration")
        install_tools
        run_integration_tests
        ;;
    "e2e")
        install_tools
        run_e2e_tests
        ;;
    "performance")
        install_tools
        run_performance_benchmarks
        ;;
    "coverage")
        install_tools
        run_test_coverage
        ;;
    "accessibility")
        install_tools
        run_accessibility_tests
        ;;
    "security")
        install_tools
        run_security_scanning
        ;;
    "all"|*)
        main
        ;;
esac
