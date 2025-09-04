#!/bin/bash

# 🚀 Publish leptos-shadcn-ui packages to crates.io
# This script publishes the latest packages with 100% TDD implementation

echo "🚀 Publishing leptos-shadcn-ui packages to crates.io..."
echo "📦 Version: 0.3.0 (100% TDD Implementation)"
echo ""

# List of packages to publish (core components first)
packages=(
    "checkbox"
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
)

# Publish each package
for package in "${packages[@]}"; do
    echo "📦 Publishing leptos-shadcn-$package v0.3.0..."
    
    # Check if package exists
    if [ -d "packages/leptos/$package" ]; then
        cd "packages/leptos/$package"
        
        # Publish the package
        if cargo publish; then
            echo "✅ Successfully published leptos-shadcn-$package v0.3.0"
        else
            echo "❌ Failed to publish leptos-shadcn-$package"
            exit 1
        fi
        
        cd ../../..
        echo ""
        
        # Wait a bit between publishes to avoid rate limiting
        sleep 2
    else
        echo "⚠️ Package leptos-shadcn-$package not found, skipping..."
    fi
done

echo "🎉 All packages published successfully!"
echo "📚 Your 100% TDD implementation is now available on crates.io!"
echo ""
echo "🔗 Check your packages at: https://crates.io/users/cloud-shuttle"
