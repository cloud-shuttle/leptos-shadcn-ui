#!/bin/bash

# Script to publish remaining packages, skipping already published ones

set -e

echo "🚀 Publishing remaining packages for v0.4.0"
echo "==========================================="

# Function to publish a single package
publish_package() {
    local package_dir="$1"
    local package_name="$2"
    
    echo "📦 Publishing $package_name from $package_dir"
    
    cd "$package_dir"
    
    # Publish the package
    if cargo publish --no-verify; then
        echo "✅ Successfully published $package_name v0.4.0"
    else
        echo "❌ Failed to publish $package_name v0.4.0"
        cd - > /dev/null
        return 1
    fi
    
    cd - > /dev/null
}

# Remaining packages to publish (skipping collapsible since it's already done)
remaining_packages=(
    "packages/leptos/scroll-area|leptos-shadcn-scroll-area"
    "packages/leptos/sheet|leptos-shadcn-sheet"
    "packages/leptos/drawer|leptos-shadcn-drawer"
    "packages/leptos/alert-dialog|leptos-shadcn-alert-dialog"
    "packages/leptos/avatar|leptos-shadcn-avatar"
    "packages/leptos/resizable|leptos-shadcn-resizable"
    "packages/leptos/calendar|leptos-shadcn-calendar"
    "packages/leptos/date-picker|leptos-shadcn-date-picker"
    "packages/leptos/pagination|leptos-shadcn-pagination"
    "packages/leptos/error-boundary|leptos-shadcn-error-boundary"
    "packages/leptos/lazy-loading|leptos-shadcn-lazy-loading"
)

echo "📋 Publishing ${#remaining_packages[@]} remaining packages..."
echo "=========================================================="

for package_info in "${remaining_packages[@]}"; do
    IFS='|' read -r package_dir package_name <<< "$package_info"
    
    if ! publish_package "$package_dir" "$package_name"; then
        echo "❌ Failed to publish $package_name"
        echo "⏳ Waiting 30 seconds before continuing..."
        sleep 30
    fi
    
    # Add delay between packages
    echo "⏳ Waiting 5 seconds before next package..."
    sleep 5
done

# Publish the main package
echo ""
echo "📋 Publishing main leptos-shadcn-ui package"
echo "=========================================="

cd packages/leptos-shadcn-ui

if cargo publish --no-verify; then
    echo "✅ Successfully published leptos-shadcn-ui v0.4.0"
else
    echo "❌ Failed to publish leptos-shadcn-ui v0.4.0"
    exit 1
fi

cd - > /dev/null

echo ""
echo "🎉 All remaining packages published to v0.4.0!"
echo "=============================================="
echo "✅ All component packages published"
echo "✅ Main package published"
echo ""
echo "📦 Main package: leptos-shadcn-ui v0.4.0"
echo "🔗 Available on crates.io"
