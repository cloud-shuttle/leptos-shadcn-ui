#!/bin/bash

# Script to check which packages have been successfully published to crates.io

echo "🔍 Checking published packages..."
echo "================================="

# List of all packages to check
packages=(
    "leptos-shadcn-button"
    "leptos-shadcn-input"
    "leptos-shadcn-label"
    "leptos-shadcn-checkbox"
    "leptos-shadcn-switch"
    "leptos-shadcn-radio-group"
    "leptos-shadcn-select"
    "leptos-shadcn-textarea"
    "leptos-shadcn-card"
    "leptos-shadcn-separator"
    "leptos-shadcn-tabs"
    "leptos-shadcn-accordion"
    "leptos-shadcn-dialog"
    "leptos-shadcn-popover"
    "leptos-shadcn-tooltip"
    "leptos-shadcn-alert"
    "leptos-shadcn-badge"
    "leptos-shadcn-skeleton"
    "leptos-shadcn-progress"
    "leptos-shadcn-toast"
    "leptos-shadcn-table"
    "leptos-shadcn-slider"
    "leptos-shadcn-toggle"
    "leptos-shadcn-carousel"
    "leptos-shadcn-form"
    "leptos-shadcn-combobox"
    "leptos-shadcn-command"
    "leptos-shadcn-input-otp"
    "leptos-shadcn-breadcrumb"
    "leptos-shadcn-navigation-menu"
    "leptos-shadcn-context-menu"
    "leptos-shadcn-dropdown-menu"
    "leptos-shadcn-menubar"
    "leptos-shadcn-hover-card"
    "leptos-shadcn-aspect-ratio"
    "leptos-shadcn-collapsible"
    "leptos-shadcn-scroll-area"
    "leptos-shadcn-sheet"
    "leptos-shadcn-drawer"
    "leptos-shadcn-alert-dialog"
    "leptos-shadcn-avatar"
    "leptos-shadcn-resizable"
    "leptos-shadcn-calendar"
    "leptos-shadcn-date-picker"
    "leptos-shadcn-pagination"
    "leptos-shadcn-error-boundary"
    "leptos-shadcn-lazy-loading"
)

published_count=0
not_published=()

for package in "${packages[@]}"; do
    if cargo search "$package" --limit 1 | grep -q "version = \"0.4.0\""; then
        echo "✅ $package v0.4.0 - Published"
        ((published_count++))
    else
        echo "❌ $package v0.4.0 - Not published"
        not_published+=("$package")
    fi
done

echo ""
echo "📊 Summary:"
echo "==========="
echo "✅ Published: $published_count packages"
echo "❌ Not published: ${#not_published[@]} packages"

if [ ${#not_published[@]} -gt 0 ]; then
    echo ""
    echo "📋 Packages still needing publication:"
    for package in "${not_published[@]}"; do
        echo "  - $package"
    done
fi

echo ""
echo "⏰ Rate limit resets: Thu, 04 Sep 2025 10:30:20 GMT"
echo "   (Check current time and wait if needed)"
