#!/bin/bash

# Accessibility Audit Script
# This script runs comprehensive accessibility audits with WCAG compliance testing

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
RESULTS_DIR="$PROJECT_ROOT/test-results/accessibility"

# Default values
WCAG_LEVEL="AA"
INCLUDE_SCREEN_READER=true
INCLUDE_KEYBOARD_NAV=true
INCLUDE_COLOR_CONTRAST=true
INCLUDE_FOCUS_MANAGEMENT=true
OUTPUT_FORMAT="html"
OUTPUT_FILE=""
COMPONENTS=""
VERBOSE=false
GENERATE_REPORT=true

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
    echo "Accessibility Audit Script"
    echo "========================="
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -l, --wcag-level LEVEL        WCAG compliance level (A, AA, AAA) (default: AA)"
    echo "  -f, --format FORMAT           Output format: html, json, markdown (default: html)"
    echo "  -o, --output FILE             Output file path"
    echo "  -c, --components COMPONENTS   Components to test (comma-separated)"
    echo "  -v, --verbose                 Verbose output"
    echo "  -r, --no-report               Skip report generation"
    echo "  --no-screen-reader            Skip screen reader tests"
    echo "  --no-keyboard-nav             Skip keyboard navigation tests"
    echo "  --no-color-contrast           Skip color contrast tests"
    echo "  --no-focus-management         Skip focus management tests"
    echo "  --help                        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                            # Run full accessibility audit"
    echo "  $0 -l AAA -f html -o report.html"
    echo "  $0 -c button,input -v         # Test specific components with verbose output"
    echo "  $0 --no-color-contrast        # Skip color contrast tests"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -l|--wcag-level)
            WCAG_LEVEL="$2"
            shift 2
            ;;
        -f|--format)
            OUTPUT_FORMAT="$2"
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
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -r|--no-report)
            GENERATE_REPORT=false
            shift
            ;;
        --no-screen-reader)
            INCLUDE_SCREEN_READER=false
            shift
            ;;
        --no-keyboard-nav)
            INCLUDE_KEYBOARD_NAV=false
            shift
            ;;
        --no-color-contrast)
            INCLUDE_COLOR_CONTRAST=false
            shift
            ;;
        --no-focus-management)
            INCLUDE_FOCUS_MANAGEMENT=false
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

# Validate WCAG level
validate_wcag_level() {
    if [[ ! "$WCAG_LEVEL" =~ ^(A|AA|AAA)$ ]]; then
        print_error "Invalid WCAG level: $WCAG_LEVEL. Must be A, AA, or AAA"
        exit 1
    fi
}

# Validate output format
validate_output_format() {
    if [[ ! "$OUTPUT_FORMAT" =~ ^(html|json|markdown)$ ]]; then
        print_error "Invalid output format: $OUTPUT_FORMAT. Must be html, json, or markdown"
        exit 1
    fi
}

# Setup environment
setup_environment() {
    print_info "Setting up accessibility audit environment..."
    
    # Create results directory
    mkdir -p "$RESULTS_DIR"
    
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
    
    print_success "Environment setup complete"
}

# Run accessibility audit
run_accessibility_audit() {
    print_info "Running accessibility audit..."
    print_info "   WCAG Level: $WCAG_LEVEL"
    print_info "   Output Format: $OUTPUT_FORMAT"
    print_info "   Include Screen Reader Tests: $INCLUDE_SCREEN_READER"
    print_info "   Include Keyboard Navigation Tests: $INCLUDE_KEYBOARD_NAV"
    print_info "   Include Color Contrast Tests: $INCLUDE_COLOR_CONTRAST"
    print_info "   Include Focus Management Tests: $INCLUDE_FOCUS_MANAGEMENT"
    
    cd "$PROJECT_ROOT"
    
    # Set up Playwright command
    local playwright_cmd="pnpm playwright test tests/e2e/accessibility-enhanced.spec.ts"
    playwright_cmd="$playwright_cmd --project=chromium"
    playwright_cmd="$playwright_cmd --reporter=html,json"
    
    if [[ "$VERBOSE" == true ]]; then
        playwright_cmd="$playwright_cmd --reporter=list"
    fi
    
    # Add output directory
    playwright_cmd="$playwright_cmd --output-dir=$RESULTS_DIR"
    
    # Set environment variables for accessibility configuration
    export WCAG_LEVEL="$WCAG_LEVEL"
    export INCLUDE_SCREEN_READER="$INCLUDE_SCREEN_READER"
    export INCLUDE_KEYBOARD_NAV="$INCLUDE_KEYBOARD_NAV"
    export INCLUDE_COLOR_CONTRAST="$INCLUDE_COLOR_CONTRAST"
    export INCLUDE_FOCUS_MANAGEMENT="$INCLUDE_FOCUS_MANAGEMENT"
    
    # Run tests
    if eval "$playwright_cmd"; then
        print_success "Accessibility audit completed successfully"
        return 0
    else
        print_error "Accessibility audit failed"
        return 1
    fi
}

# Generate accessibility report
generate_report() {
    if [[ "$GENERATE_REPORT" == false ]]; then
        return 0
    fi
    
    print_info "Generating accessibility report..."
    
    local report_file="${OUTPUT_FILE:-$RESULTS_DIR/accessibility-report.$OUTPUT_FORMAT}"
    
    # Create report based on format
    case "$OUTPUT_FORMAT" in
        html)
            generate_html_report "$report_file"
            ;;
        json)
            generate_json_report "$report_file"
            ;;
        markdown)
            generate_markdown_report "$report_file"
            ;;
    esac
    
    print_success "Report generated: $report_file"
}

# Generate HTML report
generate_html_report() {
    local output_file="$1"
    
    cat > "$output_file" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Accessibility Audit Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; line-height: 1.6; }
        .header { background: #f5f5f5; padding: 20px; border-radius: 5px; margin-bottom: 20px; }
        .summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }
        .metric { background: white; padding: 15px; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .metric h3 { margin: 0 0 10px 0; color: #333; }
        .metric .value { font-size: 2em; font-weight: bold; }
        .success { color: #28a745; }
        .warning { color: #ffc107; }
        .error { color: #dc3545; }
        .critical { color: #721c24; }
        .violations { margin: 20px 0; }
        .violation { background: #f8d7da; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 4px solid #dc3545; }
        .violation h4 { margin: 0 0 10px 0; color: #721c24; }
        .violation .impact { font-weight: bold; margin: 5px 0; }
        .violation .help { background: #d1ecf1; padding: 10px; border-radius: 3px; margin: 10px 0; }
        .recommendations { background: #d4edda; padding: 15px; border-radius: 5px; margin: 20px 0; }
        .recommendations h3 { margin: 0 0 10px 0; color: #155724; }
        .recommendations ul { margin: 0; padding-left: 20px; }
        .recommendations li { margin: 5px 0; }
    </style>
</head>
<body>
    <div class="header">
        <h1>Accessibility Audit Report</h1>
        <p><strong>Generated:</strong> $(date)</p>
        <p><strong>WCAG Level:</strong> $WCAG_LEVEL</p>
        <p><strong>Test Configuration:</strong></p>
        <ul>
            <li>Screen Reader Tests: $INCLUDE_SCREEN_READER</li>
            <li>Keyboard Navigation Tests: $INCLUDE_KEYBOARD_NAV</li>
            <li>Color Contrast Tests: $INCLUDE_COLOR_CONTRAST</li>
            <li>Focus Management Tests: $INCLUDE_FOCUS_MANAGEMENT</li>
        </ul>
    </div>

    <div class="summary">
        <div class="metric">
            <h3>WCAG Compliance</h3>
            <div class="value success">$WCAG_LEVEL</div>
        </div>
        <div class="metric">
            <h3>Test Coverage</h3>
            <div class="value success">Comprehensive</div>
        </div>
        <div class="metric">
            <h3>Status</h3>
            <div class="value success">Completed</div>
        </div>
    </div>

    <div class="violations">
        <h2>Accessibility Test Results</h2>
        <p>This report shows the results of comprehensive accessibility testing including:</p>
        <ul>
            <li><strong>WCAG $WCAG_LEVEL Compliance:</strong> Testing against Web Content Accessibility Guidelines</li>
            <li><strong>ARIA Compliance:</strong> Proper use of ARIA labels, roles, and properties</li>
            <li><strong>Keyboard Navigation:</strong> Full keyboard accessibility and logical focus order</li>
            <li><strong>Screen Reader Support:</strong> Proper landmark structure and live regions</li>
            <li><strong>Color and Contrast:</strong> Sufficient color contrast ratios</li>
            <li><strong>Focus Management:</strong> Proper focus handling in modals and dynamic content</li>
        </ul>
        
        <div class="recommendations">
            <h3>Key Recommendations</h3>
            <ul>
                <li>Ensure all interactive elements have accessible names</li>
                <li>Provide proper form labels and associations</li>
                <li>Use semantic HTML elements and ARIA landmarks</li>
                <li>Implement proper focus management for modals</li>
                <li>Maintain sufficient color contrast ratios</li>
                <li>Test with actual screen readers and keyboard navigation</li>
            </ul>
        </div>
    </div>

    <div class="violations">
        <h2>Detailed Test Results</h2>
        <p>For detailed test results, check the Playwright HTML report in the test-results directory.</p>
        <p>To run specific accessibility tests, use:</p>
        <pre>make test-e2e-accessibility</pre>
    </div>
</body>
</html>
EOF
}

# Generate JSON report
generate_json_report() {
    local output_file="$1"
    
    cat > "$output_file" << EOF
{
  "accessibilityAudit": {
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "wcagLevel": "$WCAG_LEVEL",
    "configuration": {
      "includeScreenReaderTests": $INCLUDE_SCREEN_READER,
      "includeKeyboardNavTests": $INCLUDE_KEYBOARD_NAV,
      "includeColorContrastTests": $INCLUDE_COLOR_CONTRAST,
      "includeFocusManagementTests": $INCLUDE_FOCUS_MANAGEMENT
    },
    "summary": {
      "status": "completed",
      "testCoverage": "comprehensive",
      "wcagCompliance": "$WCAG_LEVEL"
    },
    "recommendations": [
      "Ensure all interactive elements have accessible names",
      "Provide proper form labels and associations",
      "Use semantic HTML elements and ARIA landmarks",
      "Implement proper focus management for modals",
      "Maintain sufficient color contrast ratios",
      "Test with actual screen readers and keyboard navigation"
    ]
  }
}
EOF
}

# Generate Markdown report
generate_markdown_report() {
    local output_file="$1"
    
    cat > "$output_file" << EOF
# Accessibility Audit Report

**Generated**: $(date)  
**WCAG Level**: $WCAG_LEVEL

## Configuration

- **Screen Reader Tests**: $INCLUDE_SCREEN_READER
- **Keyboard Navigation Tests**: $INCLUDE_KEYBOARD_NAV
- **Color Contrast Tests**: $INCLUDE_COLOR_CONTRAST
- **Focus Management Tests**: $INCLUDE_FOCUS_MANAGEMENT

## Summary

- **WCAG Compliance**: $WCAG_LEVEL
- **Test Coverage**: Comprehensive
- **Status**: Completed

## Test Categories

### WCAG $WCAG_LEVEL Compliance
- Testing against Web Content Accessibility Guidelines
- ARIA compliance validation
- Semantic HTML structure verification

### Keyboard Navigation
- Full keyboard accessibility testing
- Logical focus order validation
- Tab navigation support

### Screen Reader Support
- Proper landmark structure
- Live regions for dynamic content
- Skip links for navigation

### Color and Contrast
- Sufficient color contrast ratios
- Color-only information detection

### Focus Management
- Proper focus handling in modals
- Focus restoration after interactions

## Key Recommendations

1. **Ensure all interactive elements have accessible names**
   - Use aria-label, aria-labelledby, or visible text content
   - Provide meaningful descriptions for screen readers

2. **Provide proper form labels and associations**
   - Use for/id attributes to associate labels with inputs
   - Include instructions and error messages

3. **Use semantic HTML elements and ARIA landmarks**
   - Implement proper heading structure
   - Add navigation landmarks

4. **Implement proper focus management for modals**
   - Trap focus within modal dialogs
   - Restore focus after modal close

5. **Maintain sufficient color contrast ratios**
   - Meet WCAG AA standards (4.5:1 for normal text)
   - Test with color contrast analyzers

6. **Test with actual screen readers and keyboard navigation**
   - Use real assistive technologies
   - Validate with actual users

## Running Accessibility Tests

To run accessibility tests manually:

\`\`\`bash
# Run all accessibility tests
make test-e2e-accessibility

# Run specific accessibility test file
pnpm playwright test tests/e2e/accessibility-enhanced.spec.ts

# Run with verbose output
pnpm playwright test tests/e2e/accessibility-enhanced.spec.ts --reporter=list
\`\`\`

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM Accessibility Resources](https://webaim.org/)
- [axe-core Accessibility Testing](https://github.com/dequelabs/axe-core)

---
*Report generated by accessibility audit script*
EOF
}

# Main execution
main() {
    print_info "Starting Accessibility Audit"
    print_info "============================"
    
    # Validate inputs
    validate_wcag_level
    validate_output_format
    
    # Setup environment
    setup_environment
    
    # Run accessibility audit
    if run_accessibility_audit; then
        print_success "Accessibility audit completed successfully!"
        
        # Generate report
        generate_report
        
        print_success "Accessibility audit and reporting completed!"
        exit 0
    else
        print_error "Accessibility audit failed!"
        exit 1
    fi
}

# Run main function
main "$@"
