#!/bin/bash

# Script to continue publishing remaining packages after rate limit reset
# This script publishes packages in smaller batches to avoid rate limiting

set -e

echo "🚀 Continuing publication of remaining packages for v0.4.0"
echo "========================================================"

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

# Function to publish packages in small batches
publish_small_batch() {
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
            echo "⏳ Waiting 30 seconds before retrying..."
            sleep 30
            
            # Try once more
            if ! publish_package "$package_dir" "$package_name"; then
                echo "❌ Final attempt failed for $package_name"
                return 1
            fi
        fi
        
        # Add a longer delay to avoid rate limiting
        echo "⏳ Waiting 5 seconds before next package..."
        sleep 5
    done
    
    echo "✅ Batch $batch_name completed successfully"
    echo "⏳ Waiting 15 seconds before next batch..."
    sleep 15
}

# Remaining packages to publish (from where we left off)
# We got through the first 3 batches successfully, so we need to continue from batch 4

# Batch 4: Menu & Interaction Components (remaining)
batch4_remaining=(
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

# Publish remaining batches
publish_small_batch "Menu & Interaction Components (4/5 remaining)" "${batch4_remaining[@]}"
publish_small_batch "Remaining Components (5/5)" "${batch5[@]}"

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
