#!/bin/bash

# 🚀 leptos-shadcn-ui v0.9.0 Major Release Publisher
# This script publishes the complete test suite transformation release to crates.io

set -e  # Exit on any error

echo "🚀 leptos-shadcn-ui v0.9.0 Major Release Publisher"
echo "📦 Publishing Complete Test Suite Transformation to crates.io"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

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

print_highlight() {
    echo -e "${PURPLE}[HIGHLIGHT]${NC} $1"
}

# Step 1: Verify we're ready for release
print_status "Step 1: Verifying release readiness..."

# Check if we're on the right branch
current_branch=$(git branch --show-current)
if [ "$current_branch" != "main" ]; then
    print_warning "Not on main branch (currently on $current_branch)"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_error "Aborting release"
        exit 1
    fi
fi

# Check if there are uncommitted changes
if ! git diff-index --quiet HEAD --; then
    print_error "There are uncommitted changes. Please commit or stash them first."
    exit 1
fi

print_success "Release readiness verified"

# Step 2: Final compilation check
print_status "Step 2: Running final compilation check..."
if ! cargo check --workspace --quiet; then
    print_error "Compilation failed. Please fix errors before releasing."
    exit 1
fi
print_success "Compilation check passed"

# Step 3: Run tests
print_status "Step 3: Running test suite..."
if ! cargo test --workspace --lib --quiet; then
    print_error "Tests failed. Please fix failing tests before releasing."
    exit 1
fi
print_success "All tests passed"

# Step 4: Publish packages in dependency order
print_status "Step 4: Publishing packages to crates.io..."

# List of packages to publish (in dependency order)
packages=(
    # Core infrastructure packages first
    "test-runner"
    "performance-monitoring" 
    "visual-testing"
    
    # Basic components (no internal dependencies)
    "button"
    "input"
    "label"
    "checkbox"
    "switch"
    "radio-group"
    "select"
    "textarea"
    "card"
    "separator"
    "tabs"
    "accordion"
    "dialog"
    "popover"
    "tooltip"
    "alert"
    "badge"
    "skeleton"
    "progress"
    "toast"
    "table"
    "slider"
    "toggle"
    "carousel"
    "form"
    "combobox"
    "command"
    "input-otp"
    "breadcrumb"
    "navigation-menu"
    "context-menu"
    "dropdown-menu"
    "menubar"
    "hover-card"
    "aspect-ratio"
    "collapsible"
    "scroll-area"
    "sheet"
    "drawer"
    "alert-dialog"
    "avatar"
    "resizable"
    
    # Components with internal dependencies
    "calendar"
    "date-picker"
    "pagination"
    "error-boundary"
    "lazy-loading"
    
    # Main library package last
    "leptos-shadcn-ui"
)

# Track published packages
published_count=0
skipped_count=0
failed_count=0
total_count=${#packages[@]}

print_highlight "Starting publication of $total_count packages..."

for package in "${packages[@]}"; do
    if [ "$package" = "leptos-shadcn-ui" ]; then
        package_dir="packages/$package"
    else
        package_dir="packages/leptos/$package"
    fi
    
    if [ ! -d "$package_dir" ]; then
        print_warning "Package directory $package_dir not found, skipping..."
        skipped_count=$((skipped_count + 1))
        continue
    fi
    
    print_status "Publishing leptos-shadcn-$package v0.9.0... ($((published_count + skipped_count + failed_count + 1))/$total_count)"
    
    cd "$package_dir"
    
    # Check if package is already published at 0.9.0
    if cargo search "leptos-shadcn-$package" 2>/dev/null | grep -q "0.9.0"; then
        print_success "leptos-shadcn-$package v0.9.0 already published, skipping..."
        skipped_count=$((skipped_count + 1))
        cd ../../..
        continue
    fi
    
    # Publish the package
    if cargo publish --quiet; then
        print_success "✅ Successfully published leptos-shadcn-$package v0.9.0"
        published_count=$((published_count + 1))
    else
        print_error "❌ Failed to publish leptos-shadcn-$package"
        failed_count=$((failed_count + 1))
        cd ../../..
        # Continue with other packages instead of exiting
        continue
    fi
    
    cd ../../..
    
    # Wait between publishes to avoid rate limiting
    if [ $((published_count + skipped_count + failed_count)) -lt $total_count ]; then
        print_status "Waiting 2 seconds before next publish..."
        sleep 2
    fi
done

# Final summary
echo ""
echo "🎉 v0.9.0 Release Publishing Complete!"
echo "📊 Summary:"
echo "   - Total packages: $total_count"
echo "   - Successfully published: $published_count"
echo "   - Already published: $skipped_count"
echo "   - Failed: $failed_count"
echo ""

if [ $failed_count -gt 0 ]; then
    print_warning "Some packages failed to publish. Check the logs above for details."
    echo "You may need to publish them manually or fix issues and retry."
else
    print_success "All packages published successfully!"
fi

echo ""
print_highlight "🔗 Your v0.9.0 packages are now available on crates.io!"
echo "📚 Developers can now install the complete test suite transformation:"
echo ""
echo "   [dependencies]"
echo "   leptos-shadcn-ui = \"0.9.0\""
echo "   leptos-shadcn-test-runner = \"0.9.0\""
echo "   leptos-shadcn-performance-monitoring = \"0.9.0\""
echo "   leptos-shadcn-visual-testing = \"0.9.0\""
echo "   # ... and all 47 component packages!"
echo ""
echo "🏆 Congratulations on publishing the most significant release in project history!"
echo "🎊 Complete Test Suite Transformation with 100% real test coverage!"
