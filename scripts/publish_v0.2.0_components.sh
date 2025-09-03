#!/bin/bash

# Publish all individual components to v0.2.0
# This script addresses the lucide-leptos compatibility issue

set -e

echo "🚀 Publishing all individual components to v0.2.0"
echo "This addresses the lucide-leptos compatibility issue"
echo ""

# List of all component packages
COMPONENTS=(
    "accordion"
    "alert"
    "alert-dialog"
    "aspect-ratio"
    "avatar"
    "badge"
    "breadcrumb"
    "button"
    "calendar"
    "card"
    "carousel"
    "checkbox"
    "collapsible"
    "combobox"
    "command"
    "context-menu"
    "date-picker"
    "dialog"
    "drawer"
    "dropdown-menu"
    "form"
    "hover-card"
    "input"
    "input-otp"
    "label"
    "menubar"
    "navigation-menu"
    "pagination"
    "popover"
    "progress"
    "radio-group"
    "scroll-area"
    "select"
    "separator"
    "sheet"
    "skeleton"
    "slider"
    "switch"
    "table"
    "tabs"
    "textarea"
    "toast"
    "toggle"
    "tooltip"
)

# Function to publish a component
publish_component() {
    local component=$1
    local package_dir="packages/leptos/$component"
    
    echo "📦 Publishing $component..."
    
    if [ ! -d "$package_dir" ]; then
        echo "❌ Package directory not found: $package_dir"
        return 1
    fi
    
    cd "$package_dir"
    
    # Check if package is ready
    echo "  🔍 Checking package readiness..."
    if ! cargo check --quiet; then
        echo "  ❌ Package check failed for $component"
        cd - > /dev/null
        return 1
    fi
    
    # Dry run first
    echo "  🧪 Testing publish (dry run)..."
    if ! cargo publish --dry-run --quiet; then
        echo "  ❌ Dry run failed for $component"
        cd - > /dev/null
        return 1
    fi
    
    # Actual publish
    echo "  🚀 Publishing $component v0.2.0..."
    if cargo publish --quiet; then
        echo "  ✅ Successfully published $component v0.2.0"
    else
        echo "  ❌ Failed to publish $component"
        cd - > /dev/null
        return 1
    fi
    
    cd - > /dev/null
    echo ""
}

# Main execution
echo "📋 Found ${#COMPONENTS[@]} components to publish"
echo ""

# Publish each component
for component in "${COMPONENTS[@]}"; do
    if ! publish_component "$component"; then
        echo "❌ Failed to publish $component. Stopping."
        exit 1
    fi
done

echo "🎉 All components published successfully!"
echo ""
echo "Next steps:"
echo "1. Update main package to use v0.2.0 dependencies"
echo "2. Test compilation: cargo check --workspace"
echo "3. Publish main package v0.2.1"
echo ""
echo "✅ lucide-leptos compatibility issue resolved!"
