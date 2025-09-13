#!/bin/bash

# Serve Demo Page Script
# Serves the demo page locally for testing and development

echo "🚀 Starting leptos-shadcn-ui Demo Server"
echo "📁 Serving demo page from: $(pwd)/demo"
echo "🌐 Demo will be available at: http://localhost:8080"
echo "📊 Performance Champion showcase ready!"
echo ""

# Check if Python is available
if command -v python3 &> /dev/null; then
    echo "🐍 Using Python 3 HTTP server"
    cd demo && python3 -m http.server 8080
elif command -v python &> /dev/null; then
    echo "🐍 Using Python HTTP server"
    cd demo && python -m SimpleHTTPServer 8080
elif command -v node &> /dev/null; then
    echo "🟢 Using Node.js HTTP server"
    npx http-server demo -p 8080 -o
else
    echo "❌ No suitable HTTP server found. Please install Python or Node.js"
    echo "   Or use any other HTTP server to serve the demo directory"
    exit 1
fi

