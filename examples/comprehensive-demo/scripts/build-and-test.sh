#!/bin/bash

# Comprehensive Build and Test Script for Leptos ShadCN UI Demo
# Handles port conflicts, builds WASM, serves content, and runs Playwright tests

set -e

echo "🚀 Leptos ShadCN UI Comprehensive Demo - Build & Test"
echo "===================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the examples/comprehensive-demo directory"
    exit 1
fi

# Check for required tools
print_status "Checking dependencies..."

if ! command -v wasm-pack &> /dev/null; then
    print_warning "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

if ! command -v node &> /dev/null; then
    print_error "Node.js is required but not installed"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    print_error "npm is required but not installed"
    exit 1
fi

# Install Node.js dependencies
print_status "Installing Node.js dependencies..."
npm install

# Build WASM package
print_status "Building WASM package..."
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    print_success "WASM build completed successfully"
else
    print_error "WASM build failed"
    exit 1
fi

# Check if pkg directory exists and has content
if [ ! -d "pkg" ] || [ ! -f "pkg/leptos_shadcn_comprehensive_demo.js" ]; then
    print_error "WASM build output not found"
    exit 1
fi

print_success "WASM package built successfully"
print_status "Generated files:"
ls -la pkg/

# Start server in background
print_status "Starting demo server..."
node scripts/serve.js &
SERVER_PID=$!

# Wait for server to start
print_status "Waiting for server to start..."
sleep 5

# Check if server is running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    print_error "Server failed to start"
    exit 1
fi

# Get the port from the server output or use default
DEMO_PORT=${DEMO_PORT:-3000}
print_success "Demo server running on port $DEMO_PORT"

# Show demo information
print_status "Demo Information:"
echo "  🌐 Demo URL: http://localhost:$DEMO_PORT"
echo "  🔍 Health Check: http://localhost:$DEMO_PORT/health"
echo "  📊 API Info: http://localhost:$DEMO_PORT/api/demo-info"
echo "  📱 Mobile Testing: http://localhost:$DEMO_PORT (responsive design)"
echo ""

print_status "Available commands:"
echo "  📱 View demo: open http://localhost:$DEMO_PORT"
echo "  🧪 Run tests: npx playwright test"
echo "  🎭 Test UI: npx playwright test --ui"
echo "  📊 Test report: npx playwright show-report"
echo "  🛑 Stop server: kill $SERVER_PID"
echo ""

print_success "Demo is ready! Press Ctrl+C to stop the server"

# Keep server running until interrupted
trap "print_status 'Stopping server...'; kill $SERVER_PID 2>/dev/null; exit 0" INT TERM

# Wait for server process
wait $SERVER_PID