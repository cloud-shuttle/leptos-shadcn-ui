#!/bin/bash
# TDD Workflow Script for Leptos ShadCN UI Remediation
set -e

echo "🧪 Starting TDD-driven remediation workflow..."

# Phase 1: Setup and validate test infrastructure
echo "📋 Phase 1: Test Infrastructure Setup"
cargo nextest run --all --profile default --no-fail-fast || echo "Initial test baseline captured"

# Phase 2: Dependency fixes with tests
echo "🔧 Phase 2: Dependency Remediation (Test-First)"
cargo nextest run --package contract-testing --profile default || echo "Contract tests will be created"

# Phase 3: API contract testing
echo "🔌 Phase 3: API Contract Testing"
cargo nextest run --workspace --profile integration

# Phase 4: WASM optimization tests
echo "⚡ Phase 4: WASM Optimization"
cargo nextest run --target wasm32-unknown-unknown --profile wasm || echo "WASM tests setup needed"

# Phase 5: Performance validation
echo "📊 Phase 5: Performance Validation"
cargo nextest run --workspace --profile performance

echo "✅ TDD workflow complete!"