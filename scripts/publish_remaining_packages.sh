#!/bin/bash

# Script to publish remaining packages after rate limit resets
# Run this after the rate limit resets (around 14:12:20 GMT on Sep 20, 2025)

echo "🚀 Publishing remaining packages to crates.io..."

# List of packages that failed due to rate limit
remaining_packages=(
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

# Publish each remaining package
for package in "${remaining_packages[@]}"; do
    echo "📦 Publishing $package..."
    (cd "packages/leptos/$package" && cargo publish --no-verify --allow-dirty) || echo "❌ Failed to publish $package"
done

echo "✅ Publishing complete!"
