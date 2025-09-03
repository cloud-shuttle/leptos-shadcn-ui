#!/bin/bash

# 🔍 Verify Batch Readiness for Publishing
# This script checks that all packages in a batch compile and are ready

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Batch definitions
BATCH_1=("tooltip" "sheet" "drawer" "hover-card" "aspect-ratio" "collapsible" "scroll-area")
BATCH_2=("breadcrumb" "navigation-menu" "context-menu" "dropdown-menu" "menubar")
BATCH_3=("alert" "alert-dialog" "badge" "skeleton" "progress" "toast")
BATCH_4=("table" "calendar")
BATCH_5=("slider" "toggle" "carousel")
BATCH_6=("command" "input-otp" "lazy-loading" "error-boundary" "registry")
BATCH_7=("date-picker" "pagination" "form" "combobox")
BATCH_8=("utils")

# Function to verify a single package
verify_package() {
    local package="$1"
    local package_name="leptos-shadcn-$package"
    
    echo -e "${BLUE}🔍 Checking $package_name...${NC}"
    
    # Check if package compiles
    if cargo check -p "$package_name" --quiet; then
        echo -e "${GREEN}✅ $package_name compiles successfully${NC}"
        return 0
    else
        echo -e "${RED}❌ $package_name failed to compile${NC}"
        return 1
    fi
}

# Function to verify a batch
verify_batch() {
    local batch_name="$1"
    shift
    local packages=("$@")
    
    echo -e "\n${BLUE}🎯 Verifying $batch_name (${#packages[@]} packages)${NC}"
    echo -e "${BLUE}Packages: ${packages[*]}${NC}"
    echo ""
    
    local success_count=0
    local fail_count=0
    
    for package in "${packages[@]}"; do
        if verify_package "$package"; then
            ((success_count++))
        else
            ((fail_count++))
        fi
    done
    
    echo -e "\n${BLUE}📊 $batch_name Results:${NC}"
    echo -e "${GREEN}✅ Ready: $success_count packages${NC}"
    if [[ $fail_count -gt 0 ]]; then
        echo -e "${RED}❌ Issues: $fail_count packages${NC}"
    fi
    
    return $fail_count
}

# Main execution
main() {
    echo -e "${GREEN}🔍 Verifying Batch Readiness for Publishing${NC}"
    echo -e "${BLUE}Workspace: $WORKSPACE_ROOT${NC}"
    echo ""
    
    # Check if we're in the right directory
    if [[ ! -f "$WORKSPACE_ROOT/Cargo.toml" ]]; then
        echo -e "${RED}❌ Error: Not in workspace root directory${NC}"
        exit 1
    fi
    
    local total_ready=0
    local total_issues=0
    
    # Verify each batch
    verify_batch "Batch 1: Independent Layout Components" "${BATCH_1[@]}"
    local batch1_issues=$?
    
    verify_batch "Batch 2: Navigation Components" "${BATCH_2[@]}"
    local batch2_issues=$?
    
    verify_batch "Batch 3: Feedback & Status Components" "${BATCH_3[@]}"
    local batch3_issues=$?
    
    verify_batch "Batch 4: Data Display Components" "${BATCH_4[@]}"
    local batch4_issues=$?
    
    verify_batch "Batch 5: Interactive Components" "${BATCH_5[@]}"
    local batch5_issues=$?
    
    verify_batch "Batch 6: Advanced Components" "${BATCH_6[@]}"
    local batch6_issues=$?
    
    verify_batch "Batch 7: Dependent Components" "${BATCH_7[@]}"
    local batch7_issues=$?
    
    verify_batch "Batch 8: Utility Package" "${BATCH_8[@]}"
    local batch8_issues=$?
    
    # Calculate totals
    total_issues=$((batch1_issues + batch2_issues + batch3_issues + batch4_issues + batch5_issues + batch6_issues + batch7_issues + batch8_issues))
    total_ready=$((47 - total_issues))
    
    # Final summary
    echo -e "\n${GREEN}🎉 Batch Readiness Verification Complete!${NC}"
    echo -e "${GREEN}✅ Total Ready: $total_ready packages${NC}"
    if [[ $total_issues -gt 0 ]]; then
        echo -e "${RED}❌ Total Issues: $total_issues packages${NC}"
        echo -e "${YELLOW}⚠️  Please fix issues before publishing${NC}"
    else
        echo -e "${GREEN}🎯 All packages are ready for publishing!${NC}"
    fi
    
    # Recommendations
    if [[ $total_issues -eq 0 ]]; then
        echo -e "\n${BLUE}📋 Next Steps:${NC}"
        echo -e "${BLUE}1. Wait for rate limit to reset (23:05 GMT)${NC}"
        echo -e "${BLUE}2. Execute Batch 1: ./scripts/publish_batch_1.sh${NC}"
        echo -e "${BLUE}3. Continue through batches systematically${NC}"
    else
        echo -e "\n${YELLOW}🔧 Action Required:${NC}"
        echo -e "${YELLOW}Fix compilation issues before proceeding with publishing${NC}"
    fi
}

# Run main function
main "$@"
