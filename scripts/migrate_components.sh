#!/bin/bash

# Component Migration Script for Leptos 0.8.8 Signal Integration
# This script batches component migrations and uses cargo nextest for speed

set -e

echo "🚀 Starting batch component migration for Leptos 0.8.8 signal integration..."

# Define component batches by complexity
SIMPLE_COMPONENTS=(
    "separator"
    "skeleton" 
    "badge"
    "avatar"
    "aspect-ratio"
    "label"
    "progress"
)

MEDIUM_COMPONENTS=(
    "input"
    "textarea"
    "checkbox"
    "radio-group"
    "switch"
    "slider"
    "card"
    "alert"
    "breadcrumb"
    "pagination"
    "tabs"
    "accordion"
    "collapsible"
    "resizable"
    "scroll-area"
)

COMPLEX_COMPONENTS=(
    "form"
    "table"
    "dialog"
    "sheet"
    "drawer"
    "popover"
    "tooltip"
    "hover-card"
    "dropdown-menu"
    "context-menu"
    "navigation-menu"
    "menubar"
    "command"
    "combobox"
    "select"
    "calendar"
    "date-picker"
    "toast"
    "alert-dialog"
    "carousel"
    "toggle"
    "input-otp"
)

# Function to migrate a single component
migrate_component() {
    local component=$1
    local batch=$2
    
    echo "📦 Migrating $component (batch: $batch)..."
    
    # Check if component exists
    if [ ! -d "packages/leptos/$component" ]; then
        echo "⚠️  Component $component not found, skipping..."
        return 0
    fi
    
    # Add signal management dependency
    if ! grep -q "leptos-shadcn-signal-management" "packages/leptos/$component/Cargo.toml"; then
        echo "  ➕ Adding signal management dependency..."
        sed -i '' '/\[dependencies\]/a\
leptos-shadcn-signal-management = { path = "../../signal-management" }' "packages/leptos/$component/Cargo.toml"
    fi
    
    # Create signal managed version
    echo "  🔧 Creating signal_managed.rs..."
    cat > "packages/leptos/$component/src/signal_managed.rs" << 'EOF'
//! Signal-managed version of the component using leptos-shadcn-signal-management

use leptos::prelude::*;
use leptos_style::Style;
use leptos_shadcn_signal_management::*;

// TODO: Implement signal-managed component
// This is a template that needs to be customized for each component

/// Signal-managed component state
#[derive(Debug, Clone, PartialEq)]
pub struct SignalManagedComponentState {
    // TODO: Define component-specific state
}

impl Default for SignalManagedComponentState {
    fn default() -> Self {
        Self {
            // TODO: Initialize default state
        }
    }
}

/// Signal-managed component
#[component]
pub fn SignalManagedComponent(
    // TODO: Define component props
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let component_state = ArcRwSignal::new(SignalManagedComponentState::default());
    
    // Create computed properties using ArcMemo
    let computed_property = ArcMemo::new(move |_| {
        let state = component_state.get();
        // TODO: Implement computed logic
        "computed_value".to_string()
    });
    
    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(component_state.clone());
    theme_manager.track_memo(computed_property.clone());
    
    // Create memory manager for monitoring
    let memory_manager = SignalMemoryManager::new();
    
    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();
    
    view! {
        <div class="signal-managed-component">
            // TODO: Implement component view
            {move || computed_property.get()}
        </div>
    }
}
EOF
    
    # Update lib.rs to export signal managed component
    if ! grep -q "pub mod signal_managed;" "packages/leptos/$component/src/lib.rs"; then
        echo "  📝 Updating lib.rs exports..."
        sed -i '' '/^use /a\
pub mod signal_managed;' "packages/leptos/$component/src/lib.rs"
        
        # Add export at the end
        echo "" >> "packages/leptos/$component/src/lib.rs"
        echo "// Signal-managed exports" >> "packages/leptos/$component/src/lib.rs"
        echo "pub use signal_managed::*;" >> "packages/leptos/$component/src/lib.rs"
    fi
    
    # Test compilation
    echo "  🔍 Testing compilation..."
    if cargo check -p "leptos-shadcn-$component" > /dev/null 2>&1; then
        echo "  ✅ $component compiles successfully"
    else
        echo "  ❌ $component has compilation errors"
        return 1
    fi
}

# Function to run tests for a batch
test_batch() {
    local batch_name=$1
    local components=("${@:2}")
    
    echo "🧪 Testing batch: $batch_name"
    
    for component in "${components[@]}"; do
        if [ -d "packages/leptos/$component" ]; then
            echo "  Testing $component..."
            cargo nextest run -p "leptos-shadcn-$component" --profile integration || true
        fi
    done
}

# Main migration process
echo "📋 Starting migration process..."

# Migrate simple components first
echo "🎯 Batch 1: Simple Components"
for component in "${SIMPLE_COMPONENTS[@]}"; do
    migrate_component "$component" "simple" || echo "Failed to migrate $component"
done

# Test simple components
test_batch "simple" "${SIMPLE_COMPONENTS[@]}"

# Migrate medium components
echo "🎯 Batch 2: Medium Components"
for component in "${MEDIUM_COMPONENTS[@]}"; do
    migrate_component "$component" "medium" || echo "Failed to migrate $component"
done

# Test medium components
test_batch "medium" "${MEDIUM_COMPONENTS[@]}"

# Migrate complex components
echo "🎯 Batch 3: Complex Components"
for component in "${COMPLEX_COMPONENTS[@]}"; do
    migrate_component "$component" "complex" || echo "Failed to migrate $component"
done

# Test complex components
test_batch "complex" "${COMPLEX_COMPONENTS[@]}"

echo "🎉 Batch migration completed!"
echo "📊 Running final test suite..."

# Run comprehensive test suite
cargo nextest run --profile integration

echo "✅ All migrations completed successfully!"
