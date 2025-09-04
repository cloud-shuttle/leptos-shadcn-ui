#!/bin/bash

# 🚀 Leptos v0.8 Migration Script
# Automatically migrates all leptos-shadcn-ui components to Leptos v0.8 compatibility

set -e

echo "🚀 Starting Leptos v0.8 Migration for all components..."

# Define the components to migrate
COMPONENTS=(
    "checkbox"
    "switch"
    "radio-group"
    "select"
    "textarea"
    "card"
    "separator"
    "tabs"
    "accordion"
    "dialog"
    "popover"
    "tooltip"
    "alert"
    "badge"
    "skeleton"
    "progress"
    "toast"
    "table"
    "calendar"
    "date-picker"
    "pagination"
    "slider"
    "toggle"
    "carousel"
    "form"
    "combobox"
    "command"
    "input-otp"
    "breadcrumb"
    "navigation-menu"
    "context-menu"
    "dropdown-menu"
    "menubar"
    "hover-card"
    "alert-dialog"
    "sheet"
    "drawer"
    "scroll-area"
    "aspect-ratio"
    "resizable"
    "avatar"
    "collapsible"
)

# Function to migrate a component
migrate_component() {
    local component=$1
    local component_path="packages/leptos/$component"
    
    echo "📦 Migrating $component..."
    
    if [ ! -d "$component_path" ]; then
        echo "⚠️  Component $component not found, skipping..."
        return
    fi
    
    # Migrate default.rs
    if [ -f "$component_path/src/default.rs" ]; then
        echo "  🔧 Updating default.rs..."
        sed -i '' 's/class=computed_class/class=move || computed_class.get()/g' "$component_path/src/default.rs"
        sed -i '' 's/id=id\.get()/id=move || id.get()/g' "$component_path/src/default.rs"
        sed -i '' 's/disabled=disabled/disabled=move || disabled.get()/g' "$component_path/src/default.rs"
        sed -i '' 's/value=value\.get()/value=move || value.get()/g' "$component_path/src/default.rs"
        sed -i '' 's/placeholder=placeholder\.get()/placeholder=move || placeholder.get()/g' "$component_path/src/default.rs"
        sed -i '' 's/input_type=input_type\.get()/input_type=move || input_type.get()/g' "$component_path/src/default.rs"
        sed -i '' 's/r#type=input_type\.get()/r#type=move || input_type.get()/g' "$component_path/src/default.rs"
    fi
    
    # Migrate new_york.rs
    if [ -f "$component_path/src/new_york.rs" ]; then
        echo "  🔧 Updating new_york.rs..."
        sed -i '' 's/class=computed_class/class=move || computed_class.get()/g' "$component_path/src/new_york.rs"
        sed -i '' 's/id=id\.get()/id=move || id.get()/g' "$component_path/src/new_york.rs"
        sed -i '' 's/disabled=disabled/disabled=move || disabled.get()/g' "$component_path/src/new_york.rs"
        sed -i '' 's/value=value\.get()/value=move || value.get()/g' "$component_path/src/new_york.rs"
        sed -i '' 's/placeholder=placeholder\.get()/placeholder=move || placeholder.get()/g' "$component_path/src/new_york.rs"
        sed -i '' 's/input_type=input_type\.get()/input_type=move || input_type.get()/g' "$component_path/src/new_york.rs"
        sed -i '' 's/r#type=input_type\.get()/r#type=move || input_type.get()/g' "$component_path/src/new_york.rs"
    fi
    
    # Test the component
    echo "  🧪 Testing $component..."
    if cargo check -p "leptos-shadcn-$component" > /dev/null 2>&1; then
        echo "  ✅ $component migrated successfully!"
    else
        echo "  ❌ $component failed to compile, manual review needed"
        return 1
    fi
}

# Migrate all components
echo "📋 Migrating ${#COMPONENTS[@]} components..."

for component in "${COMPONENTS[@]}"; do
    migrate_component "$component"
done

echo ""
echo "🎉 Migration complete!"
echo ""
echo "📊 Summary:"
echo "  - Components migrated: ${#COMPONENTS[@]}"
echo "  - Status: All components should now be compatible with Leptos v0.8"
echo ""
echo "🧪 Next steps:"
echo "  1. Run 'cargo test --workspace' to verify all tests pass"
echo "  2. Run 'cargo check --workspace' to verify all components compile"
echo "  3. Test components in a real application"
echo "  4. Update version to v0.6.0"
echo "  5. Publish to crates.io"
echo ""
echo "🚀 Ready for Leptos v0.8!"
