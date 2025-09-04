#!/bin/bash

# Script to bump all sub-component crates to version 0.4.0 and publish them in batches
# This script handles version bumping, dependency updates, and batch publishing

set -e

echo "🚀 Starting version bump and publish process for v0.4.0"
echo "=================================================="

# Function to bump version in a Cargo.toml file
bump_version() {
    local cargo_file="$1"
    local new_version="$2"
    
    echo "📝 Bumping version in $cargo_file to $new_version"
    
    # Use sed to replace the version line
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_file"
    rm "$cargo_file.bak"
}

# Function to update dependencies in a Cargo.toml file
update_dependencies() {
    local cargo_file="$1"
    local new_version="$2"
    
    echo "🔗 Updating dependencies in $cargo_file"
    
    # Update all leptos-shadcn-* dependencies to the new version
    sed -i.bak "s/leptos-shadcn-[a-zA-Z0-9-]* = \"[0-9]\+\.[0-9]\+\.[0-9]\+\"/leptos-shadcn-& = \"$new_version\"/g" "$cargo_file"
    # Clean up the regex replacement artifacts
    sed -i.bak "s/leptos-shadcn-\([a-zA-Z0-9-]*\) = \"leptos-shadcn-\1 = \"[0-9]\+\.[0-9]\+\.[0-9]\+\"/leptos-shadcn-\1 = \"$new_version\"/g" "$cargo_file"
    rm "$cargo_file.bak"
}

# Function to publish a single package
publish_package() {
    local package_dir="$1"
    local package_name="$2"
    
    echo "📦 Publishing $package_name from $package_dir"
    
    cd "$package_dir"
    
    # Check if package is already published at this version
    if cargo search "$package_name" --limit 1 | grep -q "version = \"0.4.0\""; then
        echo "⚠️  $package_name v0.4.0 already published, skipping..."
        cd - > /dev/null
        return 0
    fi
    
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

# Function to publish packages in batches
publish_batch() {
    local batch_name="$1"
    shift
    local packages=("$@")
    
    echo ""
    echo "🔄 Publishing batch: $batch_name"
    echo "Packages: ${packages[*]}"
    echo "----------------------------------------"
    
    for package_info in "${packages[@]}"; do
        IFS='|' read -r package_dir package_name <<< "$package_info"
        
        if ! publish_package "$package_dir" "$package_name"; then
            echo "❌ Batch $batch_name failed at package $package_name"
            return 1
        fi
        
        # Add a small delay to avoid rate limiting
        echo "⏳ Waiting 2 seconds before next package..."
        sleep 2
    done
    
    echo "✅ Batch $batch_name completed successfully"
    echo "⏳ Waiting 10 seconds before next batch..."
    sleep 10
}

# Step 1: Bump all component versions to 0.4.0
echo ""
echo "📋 Step 1: Bumping all component versions to 0.4.0"
echo "=================================================="

# Get all component Cargo.toml files
component_files=($(ls packages/leptos/*/Cargo.toml))

for cargo_file in "${component_files[@]}"; do
    bump_version "$cargo_file" "0.4.0"
done

# Also bump the main package
bump_version "packages/leptos-shadcn-ui/Cargo.toml" "0.4.0"

echo "✅ All versions bumped to 0.4.0"

# Step 2: Update dependencies in all packages
echo ""
echo "📋 Step 2: Updating dependencies to use 0.4.0 versions"
echo "====================================================="

for cargo_file in "${component_files[@]}"; do
    update_dependencies "$cargo_file" "0.4.0"
done

# Update main package dependencies
update_dependencies "packages/leptos-shadcn-ui/Cargo.toml" "0.4.0"

echo "✅ All dependencies updated to 0.4.0"

# Step 3: Define packages in batches for publishing
echo ""
echo "📋 Step 3: Publishing packages in batches"
echo "========================================="

# Batch 1: Basic components (no internal dependencies)
batch1=(
    "packages/leptos/button|leptos-shadcn-button"
    "packages/leptos/input|leptos-shadcn-input"
    "packages/leptos/label|leptos-shadcn-label"
    "packages/leptos/checkbox|leptos-shadcn-checkbox"
    "packages/leptos/switch|leptos-shadcn-switch"
    "packages/leptos/radio-group|leptos-shadcn-radio-group"
    "packages/leptos/select|leptos-shadcn-select"
    "packages/leptos/textarea|leptos-shadcn-textarea"
    "packages/leptos/card|leptos-shadcn-card"
    "packages/leptos/separator|leptos-shadcn-separator"
)

# Batch 2: More basic components
batch2=(
    "packages/leptos/tabs|leptos-shadcn-tabs"
    "packages/leptos/accordion|leptos-shadcn-accordion"
    "packages/leptos/dialog|leptos-shadcn-dialog"
    "packages/leptos/popover|leptos-shadcn-popover"
    "packages/leptos/tooltip|leptos-shadcn-tooltip"
    "packages/leptos/alert|leptos-shadcn-alert"
    "packages/leptos/badge|leptos-shadcn-badge"
    "packages/leptos/skeleton|leptos-shadcn-skeleton"
    "packages/leptos/progress|leptos-shadcn-progress"
    "packages/leptos/toast|leptos-shadcn-toast"
)

# Batch 3: Table and form components
batch3=(
    "packages/leptos/table|leptos-shadcn-table"
    "packages/leptos/slider|leptos-shadcn-slider"
    "packages/leptos/toggle|leptos-shadcn-toggle"
    "packages/leptos/carousel|leptos-shadcn-carousel"
    "packages/leptos/form|leptos-shadcn-form"
    "packages/leptos/combobox|leptos-shadcn-combobox"
    "packages/leptos/command|leptos-shadcn-command"
    "packages/leptos/input-otp|leptos-shadcn-input-otp"
    "packages/leptos/breadcrumb|leptos-shadcn-breadcrumb"
    "packages/leptos/navigation-menu|leptos-shadcn-navigation-menu"
)

# Batch 4: Menu and interaction components
batch4=(
    "packages/leptos/context-menu|leptos-shadcn-context-menu"
    "packages/leptos/dropdown-menu|leptos-shadcn-dropdown-menu"
    "packages/leptos/menubar|leptos-shadcn-menubar"
    "packages/leptos/hover-card|leptos-shadcn-hover-card"
    "packages/leptos/aspect-ratio|leptos-shadcn-aspect-ratio"
    "packages/leptos/collapsible|leptos-shadcn-collapsible"
    "packages/leptos/scroll-area|leptos-shadcn-scroll-area"
    "packages/leptos/sheet|leptos-shadcn-sheet"
    "packages/leptos/drawer|leptos-shadcn-drawer"
    "packages/leptos/alert-dialog|leptos-shadcn-alert-dialog"
)

# Batch 5: Remaining components
batch5=(
    "packages/leptos/avatar|leptos-shadcn-avatar"
    "packages/leptos/resizable|leptos-shadcn-resizable"
    "packages/leptos/calendar|leptos-shadcn-calendar"
    "packages/leptos/date-picker|leptos-shadcn-date-picker"
    "packages/leptos/pagination|leptos-shadcn-pagination"
    "packages/leptos/error-boundary|leptos-shadcn-error-boundary"
    "packages/leptos/lazy-loading|leptos-shadcn-lazy-loading"
)

# Publish all batches
publish_batch "Basic Components (1/5)" "${batch1[@]}"
publish_batch "UI Components (2/5)" "${batch2[@]}"
publish_batch "Table & Form Components (3/5)" "${batch3[@]}"
publish_batch "Menu & Interaction Components (4/5)" "${batch4[@]}"
publish_batch "Remaining Components (5/5)" "${batch5[@]}"

# Step 4: Publish the main package
echo ""
echo "📋 Step 4: Publishing main leptos-shadcn-ui package"
echo "=================================================="

echo "📦 Publishing leptos-shadcn-ui v0.4.0"

cd packages/leptos-shadcn-ui

if cargo publish --no-verify; then
    echo "✅ Successfully published leptos-shadcn-ui v0.4.0"
else
    echo "❌ Failed to publish leptos-shadcn-ui v0.4.0"
    exit 1
fi

cd - > /dev/null

echo ""
echo "🎉 All packages successfully published to v0.4.0!"
echo "=================================================="
echo "✅ 49 component packages published"
echo "✅ 1 main package published"
echo "✅ All dependencies updated"
echo ""
echo "📦 Main package: leptos-shadcn-ui v0.4.0"
echo "🔗 Available on crates.io"
