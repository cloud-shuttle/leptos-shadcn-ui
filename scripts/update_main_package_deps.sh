#!/bin/bash

# Update main package dependencies to use local paths for development

MAIN_CARGO_TOML="packages/leptos-shadcn-ui/Cargo.toml"

# List of all components
COMPONENTS=(
    "button"
    "input"
    "label"
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
    "error-boundary"
    "lazy-loading"
    "alert-dialog"
    "sheet"
    "drawer"
    "hover-card"
    "breadcrumb"
    "navigation-menu"
    "context-menu"
    "dropdown-menu"
    "menubar"
    "collapsible"
    "scroll-area"
    "aspect-ratio"
    "resizable"
    "avatar"
)

# Update each component dependency
for component in "${COMPONENTS[@]}"; do
    echo "Updating leptos-shadcn-$component dependency..."
    sed -i '' "s/leptos-shadcn-$component = { version = \"0.6.0\", optional = true }/leptos-shadcn-$component = { path = \"..\/leptos\/$component\", version = \"0.6.0\", optional = true }/g" "$MAIN_CARGO_TOML"
done

echo "✅ All dependencies updated to use local paths"
