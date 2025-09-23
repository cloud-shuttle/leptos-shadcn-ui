#!/bin/bash

# Leptos ShadCN UI Comprehensive Demo Build Script
# This script builds and serves the comprehensive demo showcasing all refactored components

echo "🚀 Building Leptos ShadCN UI Comprehensive Demo v0.9.1"
echo "=================================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the comprehensive-demo directory"
    exit 1
fi

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the demo
echo "🔨 Building WASM package..."
wasm-pack build --target web --out-dir pkg --dev

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🌐 Starting local server..."
    echo "📱 Open your browser and go to: http://localhost:8000"
    echo "🛑 Press Ctrl+C to stop the server"
    echo ""
    
    # Start a simple HTTP server
    python3 -m http.server 8000
else
    echo "❌ Build failed!"
    exit 1
fi
