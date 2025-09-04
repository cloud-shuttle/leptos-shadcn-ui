#!/bin/bash

# 🚀 Comprehensive leptos-shadcn-ui Package Publisher
# This script updates all packages to v0.3.0 and publishes them to crates.io

set -e  # Exit on any error

echo "🚀 leptos-shadcn-ui Package Publisher v0.3.0"
echo "📦 Publishing 100% TDD Implementation to crates.io"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

# Step 1: Update all package versions to 0.3.0
print_status "Step 1: Updating all package versions to 0.3.0..."

# Find all Cargo.toml files in packages/leptos
find packages/leptos -name "Cargo.toml" -type f | while read -r cargo_file; do
    # Check if the file contains version = "0.2.0"
    if grep -q 'version = "0.2.0"' "$cargo_file"; then
        print_status "Updating version in $cargo_file"
        sed -i '' 's/version = "0.2.0"/version = "0.3.0"/' "$cargo_file"
    fi
done

print_success "All package versions updated to 0.3.0"

# Step 2: Commit version changes
print_status "Step 2: Committing version changes..."
git add packages/leptos/*/Cargo.toml
git commit -m "🚀 Bump all packages to v0.3.0 for latest TDD implementation" || {
    print_warning "No changes to commit (versions already updated)"
}

# Step 3: Push changes to GitHub
print_status "Step 3: Pushing changes to GitHub..."
git push origin main

# Step 4: Publish packages
print_status "Step 4: Publishing packages to crates.io..."

# List of packages to publish (in dependency order)
packages=(
    "button"      # Already published
    "input"       # Already published  
    "card"        # Already published
    "checkbox"    # Already published
    "label"
    "switch"
    "radio-group"
    "select"
    "textarea"
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
    "calendar"
    "date-picker"
    "pagination"
    "error-boundary"
    "lazy-loading"
)

# Track published packages
published_count=0
total_count=${#packages[@]}

for package in "${packages[@]}"; do
    package_dir="packages/leptos/$package"
    
    if [ ! -d "$package_dir" ]; then
        print_warning "Package directory $package_dir not found, skipping..."
        continue
    fi
    
    print_status "Publishing leptos-shadcn-$package v0.3.0... ($((published_count + 1))/$total_count)"
    
    cd "$package_dir"
    
    # Check if package is already published at 0.3.0
    if cargo search "leptos-shadcn-$package" | grep -q "0.3.0"; then
        print_success "leptos-shadcn-$package v0.3.0 already published, skipping..."
        cd ../../..
        continue
    fi
    
    # Publish the package
    if cargo publish; then
        print_success "✅ Successfully published leptos-shadcn-$package v0.3.0"
        published_count=$((published_count + 1))
    else
        print_error "❌ Failed to publish leptos-shadcn-$package"
        cd ../../..
        exit 1
    fi
    
    cd ../../..
    
    # Wait between publishes to avoid rate limiting
    if [ $published_count -lt $total_count ]; then
        print_status "Waiting 3 seconds before next publish..."
        sleep 3
    fi
done

# Final summary
echo ""
echo "🎉 Package Publishing Complete!"
echo "📊 Summary:"
echo "   - Total packages: $total_count"
echo "   - Successfully published: $published_count"
echo "   - Already published: $((total_count - published_count))"
echo ""
echo "🔗 Your packages are now available on crates.io!"
echo "📚 Developers can now install your 100% TDD implementation:"
echo ""
echo "   [dependencies]"
echo "   leptos-shadcn-button = \"0.3.0\""
echo "   leptos-shadcn-input = \"0.3.0\""
echo "   leptos-shadcn-card = \"0.3.0\""
echo "   # ... and many more!"
echo ""
echo "🏆 Congratulations on publishing your comprehensive TDD implementation!"
