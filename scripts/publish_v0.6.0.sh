#!/bin/bash

# 🚀 Leptos v0.8 Compatibility Release - v0.6.0 Publishing Script
# Comprehensive publishing of all components with Leptos v0.8 support

set -e

echo "🚀 Starting Leptos v0.8 Compatibility Release (v0.6.0)..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
BATCH_SIZE=10
DELAY_BETWEEN_BATCHES=30
DELAY_BETWEEN_PACKAGES=5

# Track publishing progress
PUBLISHED_COUNT=0
FAILED_COUNT=0
TOTAL_PACKAGES=47  # 46 sub-components + 1 main package

# Function to publish a package
publish_package() {
    local package_name="$1"
    local package_path="$2"
    
    echo -e "${BLUE}📦 Publishing: $package_name${NC}"
    
    if [ -d "$package_path" ]; then
        cd "$package_path"
        
        # Check if package is already published at this version
        if cargo search "$package_name" --limit 1 | grep -q "v0.6.0"; then
            echo -e "${YELLOW}⚠️  $package_name v0.6.0 already published, skipping...${NC}"
            cd - > /dev/null
            return 0
        fi
        
        # Publish the package
        if cargo publish --no-verify; then
            echo -e "${GREEN}✅ Published: $package_name v0.6.0${NC}"
            PUBLISHED_COUNT=$((PUBLISHED_COUNT + 1))
        else
            echo -e "${RED}❌ Failed to publish: $package_name${NC}"
            FAILED_COUNT=$((FAILED_COUNT + 1))
        fi
        
        cd - > /dev/null
        sleep $DELAY_BETWEEN_PACKAGES
    else
        echo -e "${RED}❌ Package path not found: $package_path${NC}"
        FAILED_COUNT=$((FAILED_COUNT + 1))
    fi
}

# Function to publish a batch of packages
publish_batch() {
    local batch_name="$1"
    shift
    local packages=("$@")
    
    echo -e "${PURPLE}📦 Publishing Batch: $batch_name${NC}"
    echo "================================================"
    
    for package_info in "${packages[@]}"; do
        IFS='|' read -r package_name package_path <<< "$package_info"
        publish_package "$package_name" "$package_path"
    done
    
    echo -e "${YELLOW}⏳ Waiting $DELAY_BETWEEN_BATCHES seconds before next batch...${NC}"
    sleep $DELAY_BETWEEN_BATCHES
    echo ""
}

# Phase 1: Version Bump
echo -e "${YELLOW}📋 Phase 1: Version Bump to v0.6.0${NC}"
echo "================================================"

# Update all component versions
echo "Updating component versions..."

# Basic components (no internal dependencies)
BASIC_COMPONENTS=(
    "packages/leptos/button"
    "packages/leptos/input"
    "packages/leptos/label"
    "packages/leptos/separator"
    "packages/leptos/checkbox"
    "packages/leptos/switch"
    "packages/leptos/radio-group"
    "packages/leptos/textarea"
    "packages/leptos/select"
    "packages/leptos/slider"
)

# Update basic components first
for component in "${BASIC_COMPONENTS[@]}"; do
    if [ -f "$component/Cargo.toml" ]; then
        sed -i '' 's/version = "0.4.0"/version = "0.6.0"/g' "$component/Cargo.toml"
        echo "Updated $component to v0.6.0"
    fi
done

# Update remaining components
find packages/leptos -name "Cargo.toml" -not -path "*/button/*" -not -path "*/input/*" -not -path "*/label/*" -not -path "*/separator/*" -not -path "*/checkbox/*" -not -path "*/switch/*" -not -path "*/radio-group/*" -not -path "*/textarea/*" -not -path "*/select/*" -not -path "*/slider/*" -exec sed -i '' 's/version = "0.4.0"/version = "0.6.0"/g' {} \;

# Update main package
sed -i '' 's/version = "0.5.0"/version = "0.6.0"/g' packages/leptos-shadcn-ui/Cargo.toml

echo -e "${GREEN}✅ All versions updated to v0.6.0${NC}"
echo ""

# Phase 2: Publish Sub-Components
echo -e "${YELLOW}📋 Phase 2: Publishing Sub-Components${NC}"
echo "================================================"

# Batch 1: Basic Form Components
publish_batch "Basic Form Components" \
    "leptos-shadcn-button|packages/leptos/button" \
    "leptos-shadcn-input|packages/leptos/input" \
    "leptos-shadcn-label|packages/leptos/label" \
    "leptos-shadcn-checkbox|packages/leptos/checkbox" \
    "leptos-shadcn-switch|packages/leptos/switch" \
    "leptos-shadcn-radio-group|packages/leptos/radio-group" \
    "leptos-shadcn-textarea|packages/leptos/textarea" \
    "leptos-shadcn-select|packages/leptos/select" \
    "leptos-shadcn-slider|packages/leptos/slider" \
    "leptos-shadcn-separator|packages/leptos/separator"

# Batch 2: Layout Components
publish_batch "Layout Components" \
    "leptos-shadcn-card|packages/leptos/card" \
    "leptos-shadcn-tabs|packages/leptos/tabs" \
    "leptos-shadcn-accordion|packages/leptos/accordion" \
    "leptos-shadcn-collapsible|packages/leptos/collapsible" \
    "leptos-shadcn-scroll-area|packages/leptos/scroll-area" \
    "leptos-shadcn-aspect-ratio|packages/leptos/aspect-ratio" \
    "leptos-shadcn-resizable|packages/leptos/resizable" \
    "leptos-shadcn-badge|packages/leptos/badge" \
    "leptos-shadcn-avatar|packages/leptos/avatar" \
    "leptos-shadcn-skeleton|packages/leptos/skeleton"

# Batch 3: Overlay Components
publish_batch "Overlay Components" \
    "leptos-shadcn-dialog|packages/leptos/dialog" \
    "leptos-shadcn-popover|packages/leptos/popover" \
    "leptos-shadcn-tooltip|packages/leptos/tooltip" \
    "leptos-shadcn-alert-dialog|packages/leptos/alert-dialog" \
    "leptos-shadcn-sheet|packages/leptos/sheet" \
    "leptos-shadcn-drawer|packages/leptos/drawer" \
    "leptos-shadcn-hover-card|packages/leptos/hover-card" \
    "leptos-shadcn-alert|packages/leptos/alert" \
    "leptos-shadcn-progress|packages/leptos/progress" \
    "leptos-shadcn-toast|packages/leptos/toast"

# Batch 4: Navigation Components
publish_batch "Navigation Components" \
    "leptos-shadcn-breadcrumb|packages/leptos/breadcrumb" \
    "leptos-shadcn-navigation-menu|packages/leptos/navigation-menu" \
    "leptos-shadcn-context-menu|packages/leptos/context-menu" \
    "leptos-shadcn-dropdown-menu|packages/leptos/dropdown-menu" \
    "leptos-shadcn-menubar|packages/leptos/menubar" \
    "leptos-shadcn-table|packages/leptos/table" \
    "leptos-shadcn-calendar|packages/leptos/calendar" \
    "leptos-shadcn-date-picker|packages/leptos/date-picker" \
    "leptos-shadcn-pagination|packages/leptos/pagination" \
    "leptos-shadcn-carousel|packages/leptos/carousel"

# Batch 5: Advanced Components
publish_batch "Advanced Components" \
    "leptos-shadcn-form|packages/leptos/form" \
    "leptos-shadcn-combobox|packages/leptos/combobox" \
    "leptos-shadcn-command|packages/leptos/command" \
    "leptos-shadcn-input-otp|packages/leptos/input-otp" \
    "leptos-shadcn-toggle|packages/leptos/toggle" \
    "leptos-shadcn-error-boundary|packages/leptos/error-boundary" \
    "leptos-shadcn-lazy-loading|packages/leptos/lazy-loading"

# Phase 3: Publish Main Package
echo -e "${YELLOW}📋 Phase 3: Publishing Main Package${NC}"
echo "================================================"

publish_package "leptos-shadcn-ui" "packages/leptos-shadcn-ui"

# Phase 4: Summary
echo -e "${YELLOW}📊 Publishing Summary${NC}"
echo "================================================"
echo -e "Total Packages: ${BLUE}$TOTAL_PACKAGES${NC}"
echo -e "Successfully Published: ${GREEN}$PUBLISHED_COUNT${NC}"
echo -e "Failed: ${RED}$FAILED_COUNT${NC}"

if [ $FAILED_COUNT -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 ALL PACKAGES PUBLISHED SUCCESSFULLY!${NC}"
    echo -e "${GREEN}✅ leptos-shadcn-ui v0.6.0 with Leptos v0.8 support is now available!${NC}"
    echo ""
    echo -e "${BLUE}🚀 Next Steps:${NC}"
    echo "1. Create GitHub release"
    echo "2. Update documentation"
    echo "3. Announce the release"
    exit 0
else
    echo ""
    echo -e "${RED}❌ Some packages failed to publish. Please review the errors above.${NC}"
    exit 1
fi
