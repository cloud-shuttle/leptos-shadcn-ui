#!/bin/bash

# 🚀 Batch Publishing Script for leptos-shadcn-ui v0.6.0
# Publishes components in batches to avoid rate limiting

set -e

echo "🚀 Starting batch publishing of leptos-shadcn-ui v0.6.0 components..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DELAY_BETWEEN_PACKAGES=3
DELAY_BETWEEN_BATCHES=10

# Track publishing progress
PUBLISHED_COUNT=0
FAILED_COUNT=0

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

# Batch 1: Basic Form Components (no internal dependencies)
echo -e "${YELLOW}📦 Batch 1: Basic Form Components${NC}"
echo "================================================"

publish_package "leptos-shadcn-separator" "packages/leptos/separator"
publish_package "leptos-shadcn-checkbox" "packages/leptos/checkbox"
publish_package "leptos-shadcn-switch" "packages/leptos/switch"
publish_package "leptos-shadcn-radio-group" "packages/leptos/radio-group"
publish_package "leptos-shadcn-textarea" "packages/leptos/textarea"
publish_package "leptos-shadcn-select" "packages/leptos/select"
publish_package "leptos-shadcn-slider" "packages/leptos/slider"

echo -e "${YELLOW}⏳ Waiting $DELAY_BETWEEN_BATCHES seconds before next batch...${NC}"
sleep $DELAY_BETWEEN_BATCHES
echo ""

# Batch 2: Layout Components
echo -e "${YELLOW}📦 Batch 2: Layout Components${NC}"
echo "================================================"

publish_package "leptos-shadcn-card" "packages/leptos/card"
publish_package "leptos-shadcn-tabs" "packages/leptos/tabs"
publish_package "leptos-shadcn-accordion" "packages/leptos/accordion"
publish_package "leptos-shadcn-collapsible" "packages/leptos/collapsible"
publish_package "leptos-shadcn-scroll-area" "packages/leptos/scroll-area"
publish_package "leptos-shadcn-aspect-ratio" "packages/leptos/aspect-ratio"
publish_package "leptos-shadcn-badge" "packages/leptos/badge"
publish_package "leptos-shadcn-avatar" "packages/leptos/avatar"
publish_package "leptos-shadcn-skeleton" "packages/leptos/skeleton"

echo -e "${YELLOW}⏳ Waiting $DELAY_BETWEEN_BATCHES seconds before next batch...${NC}"
sleep $DELAY_BETWEEN_BATCHES
echo ""

# Batch 3: Overlay Components
echo -e "${YELLOW}📦 Batch 3: Overlay Components${NC}"
echo "================================================"

publish_package "leptos-shadcn-dialog" "packages/leptos/dialog"
publish_package "leptos-shadcn-popover" "packages/leptos/popover"
publish_package "leptos-shadcn-tooltip" "packages/leptos/tooltip"
publish_package "leptos-shadcn-alert-dialog" "packages/leptos/alert-dialog"
publish_package "leptos-shadcn-sheet" "packages/leptos/sheet"
publish_package "leptos-shadcn-drawer" "packages/leptos/drawer"
publish_package "leptos-shadcn-hover-card" "packages/leptos/hover-card"
publish_package "leptos-shadcn-alert" "packages/leptos/alert"
publish_package "leptos-shadcn-progress" "packages/leptos/progress"

echo -e "${YELLOW}⏳ Waiting $DELAY_BETWEEN_BATCHES seconds before next batch...${NC}"
sleep $DELAY_BETWEEN_BATCHES
echo ""

# Batch 4: Navigation & Advanced Components
echo -e "${YELLOW}📦 Batch 4: Navigation & Advanced Components${NC}"
echo "================================================"

publish_package "leptos-shadcn-toast" "packages/leptos/toast"
publish_package "leptos-shadcn-breadcrumb" "packages/leptos/breadcrumb"
publish_package "leptos-shadcn-navigation-menu" "packages/leptos/navigation-menu"
publish_package "leptos-shadcn-context-menu" "packages/leptos/context-menu"
publish_package "leptos-shadcn-dropdown-menu" "packages/leptos/dropdown-menu"
publish_package "leptos-shadcn-menubar" "packages/leptos/menubar"
publish_package "leptos-shadcn-table" "packages/leptos/table"
publish_package "leptos-shadcn-calendar" "packages/leptos/calendar"
publish_package "leptos-shadcn-date-picker" "packages/leptos/date-picker"

echo -e "${YELLOW}⏳ Waiting $DELAY_BETWEEN_BATCHES seconds before next batch...${NC}"
sleep $DELAY_BETWEEN_BATCHES
echo ""

# Batch 5: Remaining Components
echo -e "${YELLOW}📦 Batch 5: Remaining Components${NC}"
echo "================================================"

publish_package "leptos-shadcn-pagination" "packages/leptos/pagination"
publish_package "leptos-shadcn-carousel" "packages/leptos/carousel"
publish_package "leptos-shadcn-form" "packages/leptos/form"
publish_package "leptos-shadcn-combobox" "packages/leptos/combobox"
publish_package "leptos-shadcn-command" "packages/leptos/command"
publish_package "leptos-shadcn-input-otp" "packages/leptos/input-otp"
publish_package "leptos-shadcn-toggle" "packages/leptos/toggle"
publish_package "leptos-shadcn-error-boundary" "packages/leptos/error-boundary"
publish_package "leptos-shadcn-lazy-loading" "packages/leptos/lazy-loading"
publish_package "leptos-shadcn-resizable" "packages/leptos/resizable"

# Summary
echo -e "${YELLOW}📊 Publishing Summary${NC}"
echo "================================================"
echo -e "Successfully Published: ${GREEN}$PUBLISHED_COUNT${NC}"
echo -e "Failed: ${RED}$FAILED_COUNT${NC}"

if [ $FAILED_COUNT -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 ALL COMPONENTS PUBLISHED SUCCESSFULLY!${NC}"
    echo -e "${GREEN}✅ Ready to publish main leptos-shadcn-ui v0.6.0 package!${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}❌ Some components failed to publish. Please review the errors above.${NC}"
    exit 1
fi
