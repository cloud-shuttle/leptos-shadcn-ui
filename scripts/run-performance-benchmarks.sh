#!/bin/bash

# Performance Benchmarking Script
# This script runs comprehensive performance benchmarks and regression tests

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
PERFORMANCE_AUDIT_DIR="$PROJECT_ROOT/performance-audit"
RESULTS_DIR="$PROJECT_ROOT/test-results/performance"

# Default values
ITERATIONS=100
TARGET_TIME=16
FORMAT="text"
COMPONENTS=""
UPDATE_BASELINE=false
COMMIT_HASH=""
MONITOR_DURATION=0
ENABLE_ALERTS=false

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
    echo "Performance Benchmarking Script"
    echo "==============================="
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  benchmark     Run performance benchmarks"
    echo "  regression    Run regression tests"
    echo "  monitor       Start automated monitoring"
    echo "  setup         Setup performance baseline"
    echo "  report        Generate performance report"
    echo ""
    echo "Options:"
    echo "  -i, --iterations ITERATIONS     Number of benchmark iterations (default: 100)"
    echo "  -t, --target-time TIME          Target time in milliseconds (default: 16)"
    echo "  -f, --format FORMAT             Output format: text, json, html (default: text)"
    echo "  -o, --output FILE               Output file path"
    echo "  -c, --components COMPONENTS     Components to test (comma-separated)"
    echo "  -u, --update-baseline           Update baseline after regression tests"
    echo "  --commit COMMIT                 Git commit hash"
    echo "  -d, --duration SECONDS          Monitoring duration (0 = infinite)"
    echo "  -a, --enable-alerts             Enable alerts during monitoring"
    echo "  --webhook-url URL               Webhook URL for alerts"
    echo "  --email-recipients EMAILS       Email recipients (comma-separated)"
    echo "  --help                          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 benchmark -i 200 -f html -o results.html"
    echo "  $0 regression -u --commit abc123"
    echo "  $0 monitor -d 300 -a --webhook-url https://hooks.slack.com/..."
    echo "  $0 setup --commit abc123"
}

# Parse command line arguments
COMMAND=""
OUTPUT_FILE=""
WEBHOOK_URL=""
EMAIL_RECIPIENTS=""

while [[ $# -gt 0 ]]; do
    case $1 in
        benchmark|regression|monitor|setup|report)
            COMMAND="$1"
            shift
            ;;
        -i|--iterations)
            ITERATIONS="$2"
            shift 2
            ;;
        -t|--target-time)
            TARGET_TIME="$2"
            shift 2
            ;;
        -f|--format)
            FORMAT="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        -c|--components)
            COMPONENTS="$2"
            shift 2
            ;;
        -u|--update-baseline)
            UPDATE_BASELINE=true
            shift
            ;;
        --commit)
            COMMIT_HASH="$2"
            shift 2
            ;;
        -d|--duration)
            MONITOR_DURATION="$2"
            shift 2
            ;;
        -a|--enable-alerts)
            ENABLE_ALERTS=true
            shift
            ;;
        --webhook-url)
            WEBHOOK_URL="$2"
            shift 2
            ;;
        --email-recipients)
            EMAIL_RECIPIENTS="$2"
            shift 2
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

# Validate command
if [[ -z "$COMMAND" ]]; then
    print_error "No command specified"
    show_usage
    exit 1
fi

# Setup environment
setup_environment() {
    print_info "Setting up performance benchmarking environment..."
    
    # Create results directory
    mkdir -p "$RESULTS_DIR"
    
    # Check if performance-audit directory exists
    if [[ ! -d "$PERFORMANCE_AUDIT_DIR" ]]; then
        print_error "Performance audit directory not found: $PERFORMANCE_AUDIT_DIR"
        exit 1
    fi
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    # Check if performance-audit binary exists
    if [[ ! -f "$PERFORMANCE_AUDIT_DIR/target/release/performance-benchmark" ]] && 
       [[ ! -f "$PERFORMANCE_AUDIT_DIR/target/debug/performance-benchmark" ]]; then
        print_info "Building performance benchmark tool..."
        cd "$PERFORMANCE_AUDIT_DIR"
        cargo build --release
        cd "$PROJECT_ROOT"
    fi
    
    print_success "Environment setup complete"
}

# Run benchmarks
run_benchmarks() {
    print_info "Running performance benchmarks..."
    print_info "   Iterations: $ITERATIONS"
    print_info "   Target time: ${TARGET_TIME}ms"
    print_info "   Format: $FORMAT"
    
    cd "$PERFORMANCE_AUDIT_DIR"
    
    local cmd="cargo run --release --bin performance-benchmark benchmark"
    cmd="$cmd --iterations $ITERATIONS"
    cmd="$cmd --target-time $TARGET_TIME"
    cmd="$cmd --format $FORMAT"
    
    if [[ -n "$OUTPUT_FILE" ]]; then
        cmd="$cmd --output $OUTPUT_FILE"
    fi
    
    if [[ -n "$COMPONENTS" ]]; then
        cmd="$cmd --components $COMPONENTS"
    fi
    
    if eval "$cmd"; then
        print_success "Benchmarks completed successfully"
        return 0
    else
        print_error "Benchmarks failed"
        return 1
    fi
}

# Run regression tests
run_regression_tests() {
    print_info "Running performance regression tests..."
    
    cd "$PERFORMANCE_AUDIT_DIR"
    
    local cmd="cargo run --release --bin performance-benchmark regression"
    cmd="$cmd --baseline $RESULTS_DIR/performance-baseline.json"
    cmd="$cmd --output $RESULTS_DIR/regression-results.json"
    
    if [[ "$UPDATE_BASELINE" == true ]]; then
        cmd="$cmd --update-baseline"
    fi
    
    if [[ -n "$COMMIT_HASH" ]]; then
        cmd="$cmd --commit $COMMIT_HASH"
    fi
    
    if eval "$cmd"; then
        print_success "Regression tests completed successfully"
        return 0
    else
        print_error "Regression tests failed"
        return 1
    fi
}

# Run monitoring
run_monitoring() {
    print_info "Starting automated performance monitoring..."
    print_info "   Duration: ${MONITOR_DURATION}s"
    print_info "   Alerts enabled: $ENABLE_ALERTS"
    
    cd "$PERFORMANCE_AUDIT_DIR"
    
    local cmd="cargo run --release --bin performance-benchmark monitor"
    cmd="$cmd --interval 30"
    cmd="$cmd --duration $MONITOR_DURATION"
    
    if [[ "$ENABLE_ALERTS" == true ]]; then
        cmd="$cmd --enable-alerts"
    fi
    
    if [[ -n "$WEBHOOK_URL" ]]; then
        cmd="$cmd --webhook-url $WEBHOOK_URL"
    fi
    
    if [[ -n "$EMAIL_RECIPIENTS" ]]; then
        cmd="$cmd --email-recipients $EMAIL_RECIPIENTS"
    fi
    
    if eval "$cmd"; then
        print_success "Monitoring completed successfully"
        return 0
    else
        print_error "Monitoring failed"
        return 1
    fi
}

# Setup baseline
setup_baseline() {
    print_info "Setting up performance baseline..."
    
    cd "$PERFORMANCE_AUDIT_DIR"
    
    local cmd="cargo run --release --bin performance-benchmark setup"
    cmd="$cmd --output $RESULTS_DIR/performance-baseline.json"
    
    if [[ -n "$COMMIT_HASH" ]]; then
        cmd="$cmd --commit $COMMIT_HASH"
    else
        # Get current commit hash
        local current_commit=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
        cmd="$cmd --commit $current_commit"
    fi
    
    if eval "$cmd"; then
        print_success "Performance baseline established"
        return 0
    else
        print_error "Failed to setup baseline"
        return 1
    fi
}

# Generate report
generate_report() {
    print_info "Generating performance report..."
    
    cd "$PERFORMANCE_AUDIT_DIR"
    
    local input_file="$RESULTS_DIR/benchmark-results.json"
    local output_file="${OUTPUT_FILE:-$RESULTS_DIR/performance-report.html}"
    
    if [[ ! -f "$input_file" ]]; then
        print_error "Input file not found: $input_file"
        print_info "Run benchmarks first to generate input data"
        return 1
    fi
    
    local cmd="cargo run --release --bin performance-benchmark report"
    cmd="$cmd --input $input_file"
    cmd="$cmd --output $output_file"
    cmd="$cmd --format $FORMAT"
    
    if eval "$cmd"; then
        print_success "Report generated: $output_file"
        return 0
    else
        print_error "Failed to generate report"
        return 1
    fi
}

# Main execution
main() {
    print_info "Starting Performance Benchmarking"
    print_info "=================================="
    
    # Setup environment
    setup_environment
    
    # Execute command
    case "$COMMAND" in
        benchmark)
            if run_benchmarks; then
                print_success "Benchmarking completed successfully!"
                exit 0
            else
                print_error "Benchmarking failed!"
                exit 1
            fi
            ;;
        regression)
            if run_regression_tests; then
                print_success "Regression testing completed successfully!"
                exit 0
            else
                print_error "Regression testing failed!"
                exit 1
            fi
            ;;
        monitor)
            if run_monitoring; then
                print_success "Monitoring completed successfully!"
                exit 0
            else
                print_error "Monitoring failed!"
                exit 1
            fi
            ;;
        setup)
            if setup_baseline; then
                print_success "Baseline setup completed successfully!"
                exit 0
            else
                print_error "Baseline setup failed!"
                exit 1
            fi
            ;;
        report)
            if generate_report; then
                print_success "Report generation completed successfully!"
                exit 0
            else
                print_error "Report generation failed!"
                exit 1
            fi
            ;;
        *)
            print_error "Unknown command: $COMMAND"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
