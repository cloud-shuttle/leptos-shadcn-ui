#!/bin/bash

# 📊 Check Published Status of Leptos ShadCN UI Components
# This script checks which packages are already published on crates.io

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="0.1.0"

# Component packages to check
COMPONENTS=(
    "utils" "button" "input" "label" "checkbox" "switch" "radio-group" "select" "textarea"
    "card" "separator" "tabs" "accordion" "dialog" "popover" "tooltip" "sheet" "drawer" "hover-card" "aspect-ratio" "collapsible" "scroll-area"
    "breadcrumb" "navigation-menu" "context-menu" "dropdown-menu" "menubar"
    "alert" "alert-dialog" "badge" "skeleton" "progress" "toast" "table" "calendar" "date-picker" "pagination"
    "slider" "toggle" "carousel"
    "form" "combobox" "command" "input-otp" "lazy-loading" "error-boundary" "registry"
)

echo -e "${BLUE}🔍 Checking published status of Leptos ShadCN UI components...${NC}"
echo -e "${BLUE}Version: $VERSION${NC}"
echo ""

# Check each component
published_count=0
unpublished_count=0

for component in "${COMPONENTS[@]}"; do
    package_name="leptos-shadcn-$component"
    
    if cargo search "$package_name" --limit 1 | grep -q "$package_name"; then
        if cargo search "$package_name" --limit 1 | grep -q "$VERSION"; then
            echo -e "${GREEN}✅ $package_name v$VERSION (Published)${NC}"
            ((published_count++))
        else
            echo -e "${YELLOW}⚠️  $package_name exists but not v$VERSION${NC}"
            ((unpublished_count++))
        fi
    else
        echo -e "${RED}❌ $package_name (Not published)${NC}"
        ((unpublished_count++))
    fi
done

echo ""
echo -e "${BLUE}📊 Summary:${NC}"
echo -e "${GREEN}✅ Published: $published_count packages${NC}"
echo -e "${RED}❌ Unpublished: $unpublished_count packages${NC}"
echo -e "${BLUE}📦 Total: ${#COMPONENTS[@]} packages${NC}"

if [[ $published_count -eq ${#COMPONENTS[@]} ]]; then
    echo -e "\n${GREEN}🎉 All packages are already published!${NC}"
    echo -e "${BLUE}Next step: Update main package to use version dependencies and publish it.${NC}"
elif [[ $published_count -gt 0 ]]; then
    echo -e "\n${YELLOW}⚠️  Some packages are already published.${NC}"
    echo -e "${BLUE}You can run the publishing script to publish the remaining packages.${NC}"
else
    echo -e "\n${BLUE}📤 No packages published yet. Ready to start publishing!${NC}"
fi
