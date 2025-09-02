#!/bin/bash

# Leptos ShadCN UI Release Script
# This script publishes the 25 ready components to crates.io

set -e

echo "🚀 Starting Leptos ShadCN UI Release Process"
echo "=============================================="

# List of components ready for release
COMPONENTS=(
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
    "calendar"
    "date-picker"
    "pagination"
    "slider"
    "toggle"
)

echo "📦 Components to publish: ${#COMPONENTS[@]}"
echo ""

# Function to check if a crate is already published
check_if_published() {
    local package_name=$1
    local version="0.1.0"
    
    # Check if the crate exists on crates.io
    if cargo search "$package_name" --limit 1 | grep -q "^$package_name"; then
        # Check if our specific version is already published
        if cargo search "$package_name" --limit 10 | grep -q "$package_name = \"$version\""; then
            return 0  # Already published
        fi
    fi
    return 1  # Not published
}

# Function to publish a component
publish_component() {
    local component=$1
    local package_name="leptos-shadcn-${component}"
    
    echo "📤 Publishing ${package_name}..."
    
    # Check if already published
    if check_if_published "$package_name"; then
        echo "  ✅ ${package_name} is already published on crates.io - skipping"
        return 0
    fi
    
    # Navigate to component directory
    cd "packages/leptos/${component}"
    
    # Check if component compiles
    echo "  🔍 Checking compilation..."
    if cargo check --quiet; then
        echo "  ✅ Component compiles successfully"
        
        # Publish to crates.io
        echo "  🚀 Publishing to crates.io..."
        if cargo publish --quiet; then
            echo "  ✅ ${package_name} published successfully!"
        else
            echo "  ❌ Failed to publish ${package_name}"
            return 1
        fi
    else
        echo "  ❌ Component compilation failed"
        return 1
    fi
    
    # Return to root directory
    cd ../../..
    echo ""
}

# Main publishing loop
echo "Starting component publishing..."
echo ""

for component in "${COMPONENTS[@]}"; do
    if ! publish_component "$component"; then
        echo "❌ Release failed at component: ${component}"
        echo "Please fix the issue and run the script again."
        exit 1
    fi
done

echo "🎉 All components published successfully!"
echo ""
echo "📋 Next steps:"
echo "1. Verify all packages are visible on crates.io"
echo "2. Update documentation with crates.io installation instructions"
echo "3. Announce the release to the community"
echo "4. Plan development for the remaining 27 components"
echo ""
echo "✅ Release v0.1.0 complete!"
