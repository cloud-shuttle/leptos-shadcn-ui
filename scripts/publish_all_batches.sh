#!/bin/bash

# 🚀 Master Publishing Script: Execute All Remaining Batches
# This script runs all remaining batches sequentially for maximum efficiency

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BATCH_SCRIPTS=(
    "publish_batch_4.sh"
    "publish_batch_5.sh"
    "publish_batch_6.sh"
    "publish_batch_7.sh"
    "publish_batch_8.sh"
)

echo -e "${GREEN}🚀 Master Publishing Script: Execute All Remaining Batches${NC}"
echo -e "${BLUE}Total batches: ${#BATCH_SCRIPTS[@]}${NC}"
echo -e "${BLUE}Estimated total time: 2-3 hours${NC}"
echo ""
echo -e "${YELLOW}⚠️  This will execute all remaining batches sequentially${NC}"
echo -e "${YELLOW}⚠️  Each batch will ask for confirmation before proceeding${NC}"
echo ""

# Check if we're in the right directory
if [[ ! -f "$WORKSPACE_ROOT/Cargo.toml" ]]; then
    echo -e "${RED}❌ Error: Not in workspace root directory${NC}"
    exit 1
fi

# Check if all batch scripts exist
echo -e "${BLUE}🔍 Checking if all batch scripts exist...${NC}"
for script in "${BATCH_SCRIPTS[@]}"; do
    if [[ ! -f "$WORKSPACE_ROOT/scripts/$script" ]]; then
        echo -e "${RED}❌ Error: Batch script not found: $script${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ Found: $script${NC}"
done

# Confirm before proceeding
echo ""
echo -e "${YELLOW}⚠️  This will execute ${#BATCH_SCRIPTS[@]} batches sequentially${NC}"
echo -e "${YELLOW}⚠️  Each batch will ask for confirmation before proceeding${NC}"
echo -e "${YELLOW}⚠️  You can cancel any individual batch if needed${NC}"
echo ""
read -p "Do you want to start the master publishing process? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Master publishing process cancelled${NC}"
    exit 0
fi

# Start executing batches
echo -e "\n${GREEN}🎯 Starting master publishing process...${NC}"

for i in "${!BATCH_SCRIPTS[@]}"; do
    script="${BATCH_SCRIPTS[$i]}"
    current=$((i + 1))
    total=${#BATCH_SCRIPTS[@]}
    
    echo -e "\n${BLUE}📦 [${current}/${total}] Executing $script...${NC}"
    echo -e "${BLUE}⏳ Starting batch ${current} of ${total}...${NC}"
    
    # Execute the batch script
    if "$WORKSPACE_ROOT/scripts/$script"; then
        echo -e "${GREEN}✅ Batch ${current} completed successfully!${NC}"
    else
        echo -e "${RED}❌ Batch ${current} failed or was cancelled${NC}"
        echo -e "${YELLOW}⚠️  You can continue with the next batch or fix issues and retry${NC}"
        
        # Ask if user wants to continue
        read -p "Do you want to continue with the next batch? (y/N): " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${YELLOW}Master publishing process stopped by user${NC}"
            exit 0
        fi
    fi
    
    # Brief pause between batches (except for the last one)
    if [[ $i -lt $((total - 1)) ]]; then
        echo -e "${BLUE}⏳ Brief pause before next batch...${NC}"
        sleep 10
    fi
done

# Final summary
echo -e "\n${GREEN}🎉🎉🎉 MASTER PUBLISHING PROCESS COMPLETED! 🎉🎉🎉${NC}"
echo -e "${GREEN}🎯 All batches have been executed!${NC}"
echo -e "${BLUE}📊 Check the status of individual packages with: ./scripts/check_published_status.sh${NC}"
echo -e "${BLUE}🚀 Next step: Publish the main leptos-shadcn-ui package${NC}"
