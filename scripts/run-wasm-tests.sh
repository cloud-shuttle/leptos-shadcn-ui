#!/bin/bash

# Enhanced WASM Browser Testing Runner
# This script runs comprehensive WASM tests across all supported browsers

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_RESULTS_DIR="$PROJECT_ROOT/test-results/wasm-tests"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Default values
BROWSERS="chromium,firefox,webkit,Mobile Chrome,Mobile Safari"
SCENARIOS="basic-initialization,memory-management,cross-browser-compatibility,performance-monitoring,error-handling,bundle-analysis"
HEADLESS=true
PARALLEL=false
VERBOSE=false
GENERATE_REPORTS=true

# Function to print colored output
print_info() {
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

# Function to show usage
show_usage() {
    echo "Enhanced WASM Browser Testing Runner"
    echo "===================================="
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -b, --browsers BROWSERS     Comma-separated list of browsers to test"
    echo "                              Default: chromium,firefox,webkit,Mobile Chrome,Mobile Safari"
    echo "  -s, --scenarios SCENARIOS   Comma-separated list of test scenarios"
    echo "                              Default: all scenarios"
    echo "  -h, --headless             Run tests in headless mode (default)"
    echo "  -H, --headed               Run tests in headed mode"
    echo "  -p, --parallel             Run tests in parallel"
    echo "  -v, --verbose              Verbose output"
    echo "  -r, --no-reports           Skip report generation"
    echo "  --help                     Show this help message"
    echo ""
    echo "Available browsers:"
    echo "  chromium, firefox, webkit, Mobile Chrome, Mobile Safari"
    echo ""
    echo "Available scenarios:"
    echo "  basic-initialization, memory-management, cross-browser-compatibility,"
    echo "  performance-monitoring, error-handling, bundle-analysis"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Run all tests with default settings"
    echo "  $0 -b chromium,firefox -H            # Run on Chrome and Firefox in headed mode"
    echo "  $0 -s basic-initialization -v        # Run only basic initialization tests with verbose output"
    echo "  $0 -p -r                             # Run in parallel without generating reports"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -b|--browsers)
            BROWSERS="$2"
            shift 2
            ;;
        -s|--scenarios)
            SCENARIOS="$2"
            shift 2
            ;;
        -h|--headless)
            HEADLESS=true
            shift
            ;;
        -H|--headed)
            HEADLESS=false
            shift
            ;;
        -p|--parallel)
            PARALLEL=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -r|--no-reports)
            GENERATE_REPORTS=false
            shift
            ;;
        --help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Validate browsers
validate_browsers() {
    local valid_browsers=("chromium" "firefox" "webkit" "Mobile Chrome" "Mobile Safari")
    IFS=',' read -ra BROWSER_ARRAY <<< "$BROWSERS"
    
    for browser in "${BROWSER_ARRAY[@]}"; do
        browser=$(echo "$browser" | xargs) # Trim whitespace
        if [[ ! " ${valid_browsers[@]} " =~ " ${browser} " ]]; then
            print_error "Invalid browser: $browser"
            print_error "Valid browsers: ${valid_browsers[*]}"
            exit 1
        fi
    done
}

# Validate scenarios
validate_scenarios() {
    local valid_scenarios=("basic-initialization" "memory-management" "cross-browser-compatibility" "performance-monitoring" "error-handling" "bundle-analysis")
    IFS=',' read -ra SCENARIO_ARRAY <<< "$SCENARIOS"
    
    for scenario in "${SCENARIO_ARRAY[@]}"; do
        scenario=$(echo "$scenario" | xargs) # Trim whitespace
        if [[ ! " ${valid_scenarios[@]} " =~ " ${scenario} " ]]; then
            print_error "Invalid scenario: $scenario"
            print_error "Valid scenarios: ${valid_scenarios[*]}"
            exit 1
        fi
    done
}

# Setup test environment
setup_environment() {
    print_info "Setting up WASM testing environment..."
    
    # Create test results directory
    mkdir -p "$TEST_RESULTS_DIR"
    
    # Check if Playwright is installed
    if ! command -v pnpm &> /dev/null; then
        print_error "pnpm is not installed. Please install pnpm first."
        exit 1
    fi
    
    # Check if Playwright browsers are installed
    if ! pnpm playwright --version &> /dev/null; then
        print_warning "Playwright not found. Installing Playwright..."
        cd "$PROJECT_ROOT"
        pnpm install
        pnpm playwright install
    fi
    
    # Check if WASM target is installed
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        print_warning "WASM target not installed. Installing wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi
    
    print_success "Environment setup complete"
}

# Build WASM test application
build_wasm_app() {
    print_info "Building WASM test application..."
    
    cd "$PROJECT_ROOT"
    
    # Build the minimal WASM test
    if [ -d "minimal-wasm-test" ]; then
        cd minimal-wasm-test
        wasm-pack build --target web --out-dir pkg
        cd ..
        print_success "WASM test application built successfully"
    else
        print_warning "minimal-wasm-test directory not found, skipping WASM build"
    fi
}

# Run WASM tests for a specific browser
run_browser_tests() {
    local browser="$1"
    local browser_results_dir="$TEST_RESULTS_DIR/$browser"
    
    print_info "Running WASM tests on $browser..."
    
    # Create browser-specific results directory
    mkdir -p "$browser_results_dir"
    
    # Set up Playwright command
    local playwright_cmd="pnpm playwright test tests/e2e/wasm-browser-testing.spec.ts"
    playwright_cmd="$playwright_cmd --project=$browser"
    
    if [ "$HEADLESS" = true ]; then
        playwright_cmd="$playwright_cmd --headed=false"
    else
        playwright_cmd="$playwright_cmd --headed=true"
    fi
    
    if [ "$VERBOSE" = true ]; then
        playwright_cmd="$playwright_cmd --reporter=list"
    else
        playwright_cmd="$playwright_cmd --reporter=html,json"
    fi
    
    # Add output directory
    playwright_cmd="$playwright_cmd --output-dir=$browser_results_dir"
    
    # Run tests
    cd "$PROJECT_ROOT"
    if eval "$playwright_cmd"; then
        print_success "WASM tests passed on $browser"
        return 0
    else
        print_error "WASM tests failed on $browser"
        return 1
    fi
}

# Run all browser tests
run_all_tests() {
    local failed_browsers=()
    local passed_browsers=()
    
    IFS=',' read -ra BROWSER_ARRAY <<< "$BROWSERS"
    
    if [ "$PARALLEL" = true ]; then
        print_info "Running tests in parallel across all browsers..."
        
        # Run tests in parallel
        local pids=()
        for browser in "${BROWSER_ARRAY[@]}"; do
            browser=$(echo "$browser" | xargs) # Trim whitespace
            run_browser_tests "$browser" &
            pids+=($!)
        done
        
        # Wait for all tests to complete
        for i in "${!pids[@]}"; do
            local browser="${BROWSER_ARRAY[$i]}"
            browser=$(echo "$browser" | xargs) # Trim whitespace
            
            if wait "${pids[$i]}"; then
                passed_browsers+=("$browser")
            else
                failed_browsers+=("$browser")
            fi
        done
    else
        print_info "Running tests sequentially across all browsers..."
        
        # Run tests sequentially
        for browser in "${BROWSER_ARRAY[@]}"; do
            browser=$(echo "$browser" | xargs) # Trim whitespace
            
            if run_browser_tests "$browser"; then
                passed_browsers+=("$browser")
            else
                failed_browsers+=("$browser")
            fi
        done
    fi
    
    # Print summary
    echo ""
    print_info "Test Summary:"
    print_success "Passed browsers: ${passed_browsers[*]}"
    if [ ${#failed_browsers[@]} -gt 0 ]; then
        print_error "Failed browsers: ${failed_browsers[*]}"
    fi
    
    # Return exit code based on results
    if [ ${#failed_browsers[@]} -gt 0 ]; then
        return 1
    else
        return 0
    fi
}

# Generate comprehensive test report
generate_report() {
    if [ "$GENERATE_REPORTS" = false ]; then
        return 0
    fi
    
    print_info "Generating comprehensive WASM test report..."
    
    local report_file="$TEST_RESULTS_DIR/wasm-test-report-$TIMESTAMP.md"
    
    cat > "$report_file" << EOF
# WASM Browser Testing Report

**Generated**: $(date)
**Test Configuration**:
- Browsers: $BROWSERS
- Scenarios: $SCENARIOS
- Headless Mode: $HEADLESS
- Parallel Execution: $PARALLEL

## Test Results Summary

EOF
    
    # Add browser-specific results
    IFS=',' read -ra BROWSER_ARRAY <<< "$BROWSERS"
    for browser in "${BROWSER_ARRAY[@]}"; do
        browser=$(echo "$browser" | xargs) # Trim whitespace
        local browser_results_dir="$TEST_RESULTS_DIR/$browser"
        
        echo "### $browser" >> "$report_file"
        
        if [ -f "$browser_results_dir/results.json" ]; then
            # Parse JSON results and add to report
            local passed=$(jq '.stats.passed // 0' "$browser_results_dir/results.json" 2>/dev/null || echo "0")
            local failed=$(jq '.stats.failed // 0' "$browser_results_dir/results.json" 2>/dev/null || echo "0")
            local skipped=$(jq '.stats.skipped // 0' "$browser_results_dir/results.json" 2>/dev/null || echo "0")
            
            echo "- **Passed**: $passed" >> "$report_file"
            echo "- **Failed**: $failed" >> "$report_file"
            echo "- **Skipped**: $skipped" >> "$report_file"
        else
            echo "- **Status**: No results found" >> "$report_file"
        fi
        
        echo "" >> "$report_file"
    done
    
    echo "## Detailed Results" >> "$report_file"
    echo "" >> "$report_file"
    echo "Detailed test results are available in the following directories:" >> "$report_file"
    echo "" >> "$report_file"
    
    for browser in "${BROWSER_ARRAY[@]}"; do
        browser=$(echo "$browser" | xargs) # Trim whitespace
        echo "- **$browser**: \`$TEST_RESULTS_DIR/$browser/\`" >> "$report_file"
    done
    
    print_success "Report generated: $report_file"
}

# Main execution
main() {
    print_info "Starting Enhanced WASM Browser Testing"
    print_info "======================================"
    
    # Validate inputs
    validate_browsers
    validate_scenarios
    
    # Setup environment
    setup_environment
    
    # Build WASM application
    build_wasm_app
    
    # Run tests
    if run_all_tests; then
        print_success "All WASM tests completed successfully!"
        generate_report
        exit 0
    else
        print_error "Some WASM tests failed!"
        generate_report
        exit 1
    fi
}

# Run main function
main "$@"
