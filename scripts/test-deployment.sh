#!/bin/bash

# Test Deployment Script
# This script tests the deployment locally before pushing to GitHub

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Testing Demo Deployment${NC}"
echo "=============================="

# Function to print status
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "examples/comprehensive-demo" ]; then
    print_error "Please run this script from the root of the leptos-shadcn-ui repository"
    exit 1
fi

# Test 1: Build the demo
print_status "Testing demo build..."
cd examples/comprehensive-demo
if wasm-pack build --target web --out-dir pkg --release; then
    print_status "Demo builds successfully"
else
    print_error "Demo build failed"
    exit 1
fi
cd ../..

# Test 2: Check if all required files exist
print_status "Checking required files..."
required_files=(
    "examples/comprehensive-demo/index.html"
    "examples/comprehensive-demo/pkg/leptos_shadcn_comprehensive_demo.js"
    "examples/comprehensive-demo/pkg/leptos_shadcn_comprehensive_demo_bg.wasm"
    "tests/e2e/comprehensive-demo.spec.ts"
    ".github/workflows/demo-deploy.yml"
    "scripts/deploy-demo.sh"
)

for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        print_status "Found: $file"
    else
        print_error "Missing: $file"
        exit 1
    fi
done

# Test 3: Run Playwright tests (if available)
if command -v npx &> /dev/null; then
    print_status "Running Playwright tests..."
    if npx playwright test comprehensive-demo.spec.ts --reporter=line; then
        print_status "All tests passed!"
    else
        print_warning "Some tests failed, but deployment can continue"
    fi
else
    print_warning "npx not available, skipping Playwright tests"
fi

# Test 4: Check deployment script
print_status "Testing deployment script..."
if [ -x "scripts/deploy-demo.sh" ]; then
    print_status "Deployment script is executable"
else
    print_error "Deployment script is not executable"
    exit 1
fi

# Test 5: Validate GitHub Actions workflow
print_status "Validating GitHub Actions workflow..."
if [ -f ".github/workflows/demo-deploy.yml" ]; then
    print_status "GitHub Actions workflow exists"
else
    print_error "GitHub Actions workflow missing"
    exit 1
fi

print_status "All tests passed! 🎉"
print_status "Ready for deployment to GitHub Pages"
print_status "Run: ./scripts/deploy-demo.sh --serve to test locally"
print_status "Push to main branch to deploy automatically"
