#!/bin/bash
# Apply TDD to All Workspace Packages
# 
# This script applies TDD principles to all packages in the workspace
# that need it, ensuring consistent quality and testing standards.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Applying TDD Principles to Workspace Packages${NC}"
echo "=================================================="

# Check if we're in the workspace root
if [ ! -f "Cargo.toml" ] || ! grep -q "\[workspace\]" Cargo.toml; then
    echo -e "${RED}❌ Error: Not in workspace root directory${NC}"
    echo "Please run this script from the workspace root (where Cargo.toml with [workspace] exists)"
    exit 1
fi

# Step 1: Scan workspace for packages needing TDD
echo -e "${YELLOW}🔍 Step 1: Scanning workspace for packages needing TDD implementation...${NC}"
cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion scan

# Step 2: Apply TDD to all packages
echo -e "${YELLOW}🧪 Step 2: Applying TDD implementation to all packages...${NC}"
cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion apply

# Step 3: Generate implementation report
echo -e "${YELLOW}📊 Step 3: Generating TDD implementation report...${NC}"
cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion report

# Step 4: Validate implementation
echo -e "${YELLOW}✅ Step 4: Validating TDD implementation...${NC}"
if cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion validate; then
    echo -e "${GREEN}✅ All packages now have adequate TDD implementation!${NC}"
else
    echo -e "${YELLOW}⚠️  Some packages may still need additional TDD work${NC}"
    echo "Check the generated report for details"
fi

# Step 5: Run tests to ensure everything works
echo -e "${YELLOW}🧪 Step 5: Running tests to ensure TDD implementation works...${NC}"
cargo test --workspace

echo ""
echo -e "${GREEN}🎉 TDD Expansion Complete!${NC}"
echo "=================================================="
echo ""
echo -e "${BLUE}📋 What was accomplished:${NC}"
echo "✅ Scanned workspace for packages needing TDD"
echo "✅ Applied TDD principles to all identified packages"
echo "✅ Generated comprehensive implementation report"
echo "✅ Validated TDD implementation across workspace"
echo "✅ Ran tests to ensure everything works"
echo ""
echo -e "${BLUE}📄 Generated Files:${NC}"
echo "📊 tdd_implementation_report.md - Detailed implementation report"
echo ""
echo -e "${BLUE}🔧 Next Steps:${NC}"
echo "1. Review the generated report: cat tdd_implementation_report.md"
echo "2. Run individual package tests: cargo test --package <package-name>"
echo "3. Run performance benchmarks: cargo bench --workspace"
echo "4. Integrate with CI/CD pipeline"
echo ""
echo -e "${YELLOW}💡 Tips:${NC}"
echo "- Use 'cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion scan' to check status"
echo "- Use 'cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion apply-package <name>' for specific packages"
echo "- Check individual package test directories for generated test files"
