#!/bin/bash

# 🚀 Publish All 52 Leptos ShadCN UI Components to Crates.io
# This script publishes all individual component packages systematically

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPONENTS_DIR="$WORKSPACE_ROOT/packages/leptos"
VERSION="0.1.0"
DELAY_BETWEEN_PUBLISHES=60  # Seconds to wait between publishes (increased for rate limiting)

# Component packages to publish (in dependency order)
COMPONENTS=(
    # Core form components (no dependencies on other components)
    "utils"
    "button"
    "input"
    "label"
    "checkbox"
    "switch"
    "radio-group"
    "select"
    "textarea"
    
    # Layout components
    "card"
    "separator"
    "tabs"
    "accordion"
    "dialog"
    "popover"
    "tooltip"
    "sheet"
    "drawer"
    "hover-card"
    "aspect-ratio"
    "collapsible"
    "scroll-area"
    
    # Navigation components
    "breadcrumb"
    "navigation-menu"
    "context-menu"
    "dropdown-menu"
    "menubar"
    
    # Feedback & status components
    "alert"
    "alert-dialog"
    "badge"
    "skeleton"
    "progress"
    "toast"
    "table"
    "calendar"
    "date-picker"
    "pagination"
    
    # Interactive components
    "slider"
    "toggle"
    "carousel"
    
    # Advanced components
    "form"
    "combobox"
    "command"
    "input-otp"
    "lazy-loading"
    "error-boundary"
    "registry"
)

# Function to check if a crate is already published
check_if_published() {
    local crate_name="$1"
    local version="$2"
    
    if cargo search "$crate_name" --limit 1 | grep -q "$crate_name"; then
        if cargo search "$crate_name" --limit 1 | grep -q "$version"; then
            return 0  # Already published
        else
            return 1  # Exists but wrong version
        fi
    else
        return 1  # Not published
    fi
}

# Function to publish a single component
publish_component() {
    local component="$1"
    local package_name="leptos-shadcn-$component"
    local component_dir="$COMPONENTS_DIR/$component"
    
    echo -e "\n${BLUE}🚀 Publishing $package_name...${NC}"
    
    # Check if component directory exists
    if [[ ! -d "$component_dir" ]]; then
        echo -e "${RED}❌ Component directory not found: $component_dir${NC}"
        return 1
    fi
    
    # Check if already published
    if check_if_published "$package_name" "$VERSION"; then
        echo -e "${GREEN}⏭️  Skipping $package_name (already published)${NC}"
        return 0
    fi
    
    # Navigate to component directory
    cd "$component_dir"
    
    # Verify the package compiles
    echo -e "${BLUE}🔨 Checking if $package_name compiles...${NC}"
    if ! cargo check --quiet; then
        echo -e "${RED}❌ $package_name failed to compile${NC}"
        cd "$WORKSPACE_ROOT"
        return 1
    fi
    
    # Publish the package
    echo -e "${BLUE}📤 Publishing $package_name to crates.io...${NC}"
    if cargo publish --quiet; then
        echo -e "${GREEN}✅ Successfully published $package_name v$VERSION${NC}"
    else
        # Check if it's a rate limit error
        if cargo publish 2>&1 | grep -q "429 Too Many Requests"; then
            echo -e "${YELLOW}⚠️  Rate limit hit! Waiting 5 minutes before retry...${NC}"
            sleep 300  # Wait 5 minutes
            echo -e "${BLUE}🔄 Retrying publication of $package_name...${NC}"
            if cargo publish --quiet; then
                echo -e "${GREEN}✅ Successfully published $package_name v$VERSION (after retry)${NC}"
            else
                echo -e "${RED}❌ Failed to publish $package_name after retry${NC}"
                cd "$WORKSPACE_ROOT"
                return 1
            fi
        else
            echo -e "${RED}❌ Failed to publish $package_name${NC}"
            cd "$WORKSPACE_ROOT"
            return 1
        fi
    fi
    
    # Return to workspace root
    cd "$WORKSPACE_ROOT"
    
    # Wait before publishing next package
    if [[ "$component" != "${COMPONENTS[-1]}" ]]; then
        echo -e "${BLUE}⏳ Waiting $DELAY_BETWEEN_PUBLISHES seconds before next publish...${NC}"
        sleep "$DELAY_BETWEEN_PUBLISHES"
    fi
}

# Function to show progress
show_progress() {
    local current="$1"
    local total="$2"
    local percentage=$((current * 100 / total))
    local completed=$((current * 50 / total))
    local remaining=$((50 - completed))
    
    printf "\r${BLUE}Progress: ["
    printf "%${completed}s" | tr ' ' '█'
    printf "%${remaining}s" | tr ' ' '░'
    printf "] %d/%d (%d%%)${NC}" "$current" "$total" "$percentage"
}

# Main execution
main() {
    echo -e "${GREEN}🚀 Starting publication of all 52 Leptos ShadCN UI components${NC}"
    echo -e "${BLUE}Workspace: $WORKSPACE_ROOT${NC}"
    echo -e "${BLUE}Version: $VERSION${NC}"
    echo -e "${BLUE}Total components: ${#COMPONENTS[@]}${NC}"
    echo -e "${BLUE}Delay between publishes: ${DELAY_BETWEEN_PUBLISHES}s${NC}"
    echo ""
    
    # Check if we're in the right directory
    if [[ ! -f "$WORKSPACE_ROOT/Cargo.toml" ]]; then
        echo -e "${RED}❌ Error: Not in workspace root directory${NC}"
        exit 1
    fi
    
    # Check if logged in to crates.io
    echo -e "${BLUE}🔐 Checking crates.io login status...${NC}"
    if ! cargo whoami >/dev/null 2>&1; then
        echo -e "${RED}❌ Not logged in to crates.io. Please run 'cargo login' first.${NC}"
        exit 1
    fi
    
    local username=$(cargo whoami)
    echo -e "${GREEN}✅ Logged in as: $username${NC}"
    
    # Confirm before proceeding
    echo ""
    echo -e "${YELLOW}⚠️  This will publish ${#COMPONENTS[@]} packages to crates.io${NC}"
    echo -e "${YELLOW}⚠️  This process will take approximately $((DELAY_BETWEEN_PUBLISHES * ${#COMPONENTS[@]} / 60)) minutes${NC}"
    echo ""
    read -p "Do you want to continue? (y/N): " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Publication cancelled${NC}"
        exit 0
    fi
    
    # Start publishing
    local success_count=0
    local fail_count=0
    local total=${#COMPONENTS[@]}
    
    echo -e "\n${GREEN}🎯 Starting publication process...${NC}"
    
    for i in "${!COMPONENTS[@]}"; do
        local component="${COMPONENTS[$i]}"
        local current=$((i + 1))
        
        show_progress "$current" "$total"
        
        if publish_component "$component"; then
            ((success_count++))
        else
            ((fail_count++))
            echo -e "\n${RED}❌ Failed to publish $component${NC}"
        fi
        
        echo ""  # New line after progress bar
    done
    
    # Final summary
    echo -e "\n${GREEN}🎉 Publication process completed!${NC}"
    echo -e "${GREEN}✅ Successfully published: $success_count packages${NC}"
    if [[ $fail_count -gt 0 ]]; then
        echo -e "${RED}❌ Failed to publish: $fail_count packages${NC}"
    fi
    
    if [[ $fail_count -eq 0 ]]; then
        echo -e "\n${GREEN}🎯 All packages published successfully!${NC}"
        echo -e "${BLUE}Next step: Update main package to use version dependencies and publish it.${NC}"
    else
        echo -e "\n${YELLOW}⚠️  Some packages failed to publish. Please check the errors above.${NC}"
    fi
}

# Run main function
main "$@"
