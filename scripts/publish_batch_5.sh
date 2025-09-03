#!/bin/bash

# 🚀 Publish Batch 5: Interactive Components
# This script publishes the fifth batch of 3 packages efficiently

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to check if package is already published
check_if_published() {
    local package_name="$1"
    if cargo search "$package_name" --limit 1 | grep -q "$package_name"; then
        return 0  # Package exists
    else
        return 1  # Package doesn't exist
    fi
}

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="0.1.0"
DELAY=75  # 75 seconds between packages (conservative for rate limiting)

# Batch 5 packages (Interactive Components)
PACKAGES=(
    "slider"
    "toggle"
    "carousel"
)

echo -e "${GREEN}🚀 Starting Batch 5: Interactive Components${NC}"
echo -e "${BLUE}Total packages: ${#PACKAGES[@]}${NC}"
echo -e "${BLUE}Delay between packages: ${DELAY}s${NC}"
echo -e "${BLUE}Estimated time: 15-20 minutes${NC}"
echo ""

# Check if we're in the right directory
if [[ ! -f "$WORKSPACE_ROOT/Cargo.toml" ]]; then
    echo -e "${RED}❌ Error: Not in workspace root directory${NC}"
    exit 1
fi

# Check if logged in to crates.io
echo -e "${BLUE}🔐 Checking crates.io login status...${NC}"
if ! cargo publish --help >/dev/null 2>&1; then
    echo -e "${RED}❌ Error: Cannot access cargo publish${NC}"
    exit 1
fi

# Confirm before proceeding
echo ""
echo -e "${YELLOW}⚠️  This will publish ${#PACKAGES[@]} packages to crates.io${NC}"
echo -e "${YELLOW}⚠️  Estimated time: $((DELAY * ${#PACKAGES[@]} / 60)) minutes${NC}"
echo ""
read -p "Do you want to continue with Batch 5? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Batch 5 publication cancelled${NC}"
    exit 0
fi

# Start publishing
success_count=0
fail_count=0
total=${#PACKAGES[@]}

echo -e "\n${GREEN}🎯 Starting Batch 5 publication process...${NC}"

for i in "${!PACKAGES[@]}"; do
    package="${PACKAGES[$i]}"
    package_name="leptos-shadcn-$package"
    current=$((i + 1))
    
    echo -e "\n${BLUE}📦 [${current}/${total}] Publishing $package_name...${NC}"
    
    # Check if package is already published
    if check_if_published "$package_name"; then
        echo -e "${YELLOW}⚠️  $package_name is already published, skipping...${NC}"
        ((success_count++))
        continue
    fi
    
    # Verify the package compiles
    echo -e "${BLUE}🔨 Checking if $package_name compiles...${NC}"
    if ! cargo check -p "$package_name" --quiet; then
        echo -e "${RED}❌ $package_name failed to compile${NC}"
        ((fail_count++))
        continue
    fi
    
    # Publish the package
    echo -e "${BLUE}📤 Publishing $package_name to crates.io...${NC}"
    if cargo publish -p "$package_name" --quiet; then
        echo -e "${GREEN}✅ Successfully published $package_name v$VERSION${NC}"
        ((success_count++))
    else
        echo -e "${RED}❌ Failed to publish $package_name${NC}"
        echo -e "${YELLOW}⚠️  This might be due to rate limiting. Check the error message above.${NC}"
        ((fail_count++))
        continue
    fi
    
    # Wait before next package (except for the last one)
    if [[ $i -lt $((total - 1)) ]]; then
        echo -e "${BLUE}⏳ Waiting ${DELAY} seconds before next package...${NC}"
        sleep "$DELAY"
    fi
done

# Final summary
echo -e "\n${GREEN}🎉 Batch 5 completed!${NC}"
echo -e "${GREEN}✅ Successfully published: $success_count packages${NC}"
if [[ $fail_count -gt 0 ]]; then
    echo -e "${RED}❌ Failed to publish: $fail_count packages${NC}"
fi

if [[ $fail_count -eq 0 ]]; then
    echo -e "\n${GREEN}🎯 All Batch 5 packages published successfully!${NC}"
    echo -e "${BLUE}Ready to proceed with Batch 6: Advanced Components${NC}"
else
    echo -e "\n${YELLOW}⚠️  Some packages failed. Please check the errors above.${NC}"
fi
