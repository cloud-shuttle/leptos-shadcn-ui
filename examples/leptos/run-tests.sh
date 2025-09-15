#!/bin/bash

# Leptos ShadCN UI Demo Test Runner
# This script runs comprehensive Playwright tests to verify the demo functionality

set -e

echo "🚀 Starting Leptos ShadCN UI Demo Tests"
echo "========================================"

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "❌ Error: trunk is not installed. Please install it first:"
    echo "   cargo install trunk"
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Error: Node.js is not installed. Please install it first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ Error: npm is not installed. Please install it first."
    exit 1
fi

echo "✅ Prerequisites check passed"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing npm dependencies..."
    npm install
fi

# Install Playwright browsers if needed
if [ ! -d "node_modules/@playwright/test" ]; then
    echo "🎭 Installing Playwright browsers..."
    npx playwright install
fi

# Build the project
echo "🔨 Building the project..."
cargo build

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please fix the build errors first."
    exit 1
fi

echo "✅ Build successful"

# Start the server in the background
echo "🌐 Starting development server..."
trunk serve --port 8080 &
SERVER_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 10

# Check if server is running
if ! curl -s http://localhost:8080 > /dev/null; then
    echo "❌ Server failed to start. Please check the logs."
    kill $SERVER_PID 2>/dev/null || true
    exit 1
fi

echo "✅ Server is running on http://localhost:8080"

# Run the tests
echo "🧪 Running Playwright tests..."
echo ""

# Run different test suites
echo "📊 Running Visual Regression Tests..."
npx playwright test --grep "@visual" --reporter=line

echo ""
echo "🔄 Running Interaction Tests..."
npx playwright test --grep "@interaction" --reporter=line

echo ""
echo "🎨 Running Tailwind-RS-Core Tests..."
npx playwright test --grep "@tailwind-rs-core" --reporter=line

echo ""
echo "⚡ Running Performance Tests..."
npx playwright test --grep "@performance" --reporter=line

echo ""
echo "🎯 Running All Tests..."
npx playwright test --reporter=html

# Stop the server
echo ""
echo "🛑 Stopping server..."
kill $SERVER_PID 2>/dev/null || true

echo ""
echo "✅ All tests completed!"
echo "📊 Test results are available in the playwright-report directory"
echo "🌐 Open playwright-report/index.html to view detailed results"

