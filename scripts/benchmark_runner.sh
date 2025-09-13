#!/bin/bash

# Benchmark Runner Script for leptos-shadcn-ui
# This script runs comprehensive benchmarks and generates reports

set -e

echo "🚀 Starting leptos-shadcn-ui Benchmark Suite"
echo "=============================================="

# Create benchmark results directory
mkdir -p benchmark-results/$(date +%Y-%m-%d)

# Run signal management benchmarks
echo "📊 Running Signal Management Benchmarks..."
cargo bench --package leptos-shadcn-signal-management --bench signal_management_benchmarks > benchmark-results/$(date +%Y-%m-%d)/signal-management-$(date +%H-%M-%S).txt 2>&1

# Run component benchmarks (if they exist)
echo "📊 Running Component Benchmarks..."
if [ -d "packages/leptos/button/benches" ]; then
    cargo bench --package leptos-shadcn-button --bench button_benchmarks > benchmark-results/$(date +%Y-%m-%d)/button-$(date +%H-%M-%S).txt 2>&1
fi

# Run memory usage benchmarks
echo "📊 Running Memory Usage Benchmarks..."
cargo bench --package leptos-shadcn-signal-management --bench memory_benchmarks > benchmark-results/$(date +%Y-%m-%d)/memory-$(date +%H-%M-%S).txt 2>&1

# Generate summary report
echo "📋 Generating Benchmark Summary..."
cat > benchmark-results/$(date +%Y-%m-%d)/summary.md << EOF
# Benchmark Results - $(date +%Y-%m-%d)

## Signal Management Performance
- ArcRwSignal creation/access performance
- ArcMemo computation performance
- Batched update performance
- Memory management efficiency

## Component Performance
- Button component rendering performance
- Input component validation performance
- Card component layout performance

## Memory Management
- Memory leak detection accuracy
- Memory pressure monitoring
- Signal cleanup efficiency

## Recommendations
- Monitor signal creation patterns
- Optimize batched updates for large datasets
- Track memory usage in production

Generated: $(date)
EOF

echo "✅ Benchmark suite completed!"
echo "📁 Results saved to: benchmark-results/$(date +%Y-%m-%d)/"
echo "📊 View summary: benchmark-results/$(date +%Y-%m-%d)/summary.md"
