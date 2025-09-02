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

# Function to publish a component
publish_component() {
    local component=$1
    local package_name="leptos-shadcn-${component}"
    
    echo "📤 Publishing ${package_name}..."
    
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
