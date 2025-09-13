#!/usr/bin/env python3
"""
Batch Component Migration Script for Leptos 0.8.8 Signal Integration
This script creates signal-managed versions of all components efficiently
"""

import os
import subprocess
import sys
from pathlib import Path

# Component batches by complexity
SIMPLE_COMPONENTS = [
    "separator", "skeleton", "badge", "avatar", "aspect-ratio", "label", "progress"
]

MEDIUM_COMPONENTS = [
    "textarea", "checkbox", "radio-group", "switch", "slider", "alert", 
    "breadcrumb", "pagination", "tabs", "accordion", "collapsible", 
    "resizable", "scroll-area", "popover", "tooltip", "hover-card"
]

COMPLEX_COMPONENTS = [
    "form", "table", "dialog", "sheet", "drawer", "dropdown-menu", 
    "context-menu", "navigation-menu", "menubar", "command", "combobox", 
    "select", "calendar", "date-picker", "toast", "alert-dialog", 
    "carousel", "toggle", "input-otp"
]

def create_signal_managed_component(component_name, complexity="medium"):
    """Create a signal-managed version of a component"""
    
    component_path = Path(f"packages/leptos/{component_name}")
    if not component_path.exists():
        print(f"⚠️  Component {component_name} not found, skipping...")
        return False
    
    print(f"📦 Migrating {component_name} (complexity: {complexity})...")
    
    # Add signal management dependency to Cargo.toml
    cargo_toml = component_path / "Cargo.toml"
    if cargo_toml.exists():
        with open(cargo_toml, 'r') as f:
            content = f.read()
        
        if "leptos-shadcn-signal-management" not in content:
            # Add dependency
            lines = content.split('\n')
            deps_section = False
            for i, line in enumerate(lines):
                if line.strip() == "[dependencies]":
                    deps_section = True
                elif deps_section and line.strip().startswith('[') and line.strip() != "[dependencies]":
                    lines.insert(i, 'leptos-shadcn-signal-management = { path = "../../signal-management" }')
                    break
                elif deps_section and line.strip() == "":
                    lines.insert(i, 'leptos-shadcn-signal-management = { path = "../../signal-management" }')
                    break
            
            with open(cargo_toml, 'w') as f:
                f.write('\n'.join(lines))
            print(f"  ➕ Added signal management dependency")
    
    # Create signal_managed.rs
    signal_managed_path = component_path / "src" / "signal_managed.rs"
    
    # Generate component-specific template
    template = generate_component_template(component_name, complexity)
    
    with open(signal_managed_path, 'w') as f:
        f.write(template)
    print(f"  🔧 Created signal_managed.rs")
    
    # Update lib.rs
    lib_rs_path = component_path / "src" / "lib.rs"
    if lib_rs_path.exists():
        with open(lib_rs_path, 'r') as f:
            content = f.read()
        
        if "pub mod signal_managed;" not in content:
            # Add module declaration
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.strip().startswith("pub mod ") and not line.strip().startswith("pub mod signal_managed"):
                    lines.insert(i, "pub mod signal_managed;")
                    break
            
            # Add exports at the end
            if "pub use signal_managed::" not in content:
                lines.append("")
                lines.append("// Signal-managed exports")
                lines.append(f"pub use signal_managed::*;")
            
            with open(lib_rs_path, 'w') as f:
                f.write('\n'.join(lines))
            print(f"  📝 Updated lib.rs exports")
    
    return True

def generate_component_template(component_name, complexity):
    """Generate a template for the signal-managed component"""
    
    component_class = component_name.replace('-', '_').title()
    
    template = f'''//! Signal-managed version of the {component_name} component using leptos-shadcn-signal-management

use leptos::prelude::*;
use leptos_style::Style;
use leptos_shadcn_signal_management::*;

/// Signal-managed {component_name} state
#[derive(Debug, Clone, PartialEq)]
pub struct SignalManaged{component_class}State {{
    pub is_active: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub click_count: u32,
}}

impl Default for SignalManaged{component_class}State {{
    fn default() -> Self {{
        Self {{
            is_active: false,
            is_hovered: false,
            is_focused: false,
            click_count: 0,
        }}
    }}
}}

/// Signal-managed {component_name} component
#[component]
pub fn SignalManaged{component_class}(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {{
    // Create persistent state using ArcRwSignal
    let {component_name}_state = ArcRwSignal::new(SignalManaged{component_class}State::default());

    // Create computed class using ArcMemo
    let {component_name}_state_for_class = {component_name}_state.clone();
    let computed_class = ArcMemo::new(move |_| {{
        let state = {component_name}_state_for_class.get();
        let base_class = "component-base-class"; // TODO: Replace with actual base class
        let active_class = if state.is_active {{ "active" }} else {{ "" }};
        let hover_class = if state.is_hovered {{ "hover" }} else {{ "" }};
        let focus_class = if state.is_focused {{ "focus" }} else {{ "" }};
        
        format!("{{}} {{}} {{}} {{}} {{}}", 
            base_class, 
            active_class, 
            hover_class, 
            focus_class,
            class.get().unwrap_or_default()
        )
    }});

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal({component_name}_state.clone());
    theme_manager.track_memo(computed_class.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create event handlers
    let handle_click = {{
        let {component_name}_state = {component_name}_state.clone();
        move |_event: leptos::ev::MouseEvent| {{
            {component_name}_state.update(|state| {{
                state.click_count += 1;
                state.is_active = !state.is_active;
            }});
        }}
    }};

    let handle_mouse_enter = {{
        let {component_name}_state = {component_name}_state.clone();
        move |_event: leptos::ev::MouseEvent| {{
            {component_name}_state.update(|state| {{
                state.is_hovered = true;
            }});
        }}
    }};

    let handle_mouse_leave = {{
        let {component_name}_state = {component_name}_state.clone();
        move |_event: leptos::ev::MouseEvent| {{
            {component_name}_state.update(|state| {{
                state.is_hovered = false;
            }});
        }}
    }};

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    let {component_name}_state_for_disabled = {component_name}_state.clone();
    view! {{
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
            on:click=handle_click
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
        >
            {{children.map(|c| c())}}
        </div>
    }}
}}

/// Enhanced {component_name} component with advanced signal management
#[component]
pub fn Enhanced{component_class}(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {{
    // Create persistent state using ArcRwSignal
    let {component_name}_state = ArcRwSignal::new(SignalManaged{component_class}State::default());

    // Create computed class using ArcMemo
    let {component_name}_state_for_class = {component_name}_state.clone();
    let computed_class = ArcMemo::new(move |_| {{
        let state = {component_name}_state_for_class.get();
        let base_class = "component-base-class"; // TODO: Replace with actual base class
        let active_class = if state.is_active {{ "active transition-all" }} else {{ "" }};
        let hover_class = if state.is_hovered {{ "hover:shadow-md" }} else {{ "" }};
        let focus_class = if state.is_focused {{ "focus:ring-2 focus:ring-ring" }} else {{ "" }};
        
        format!("{{}} {{}} {{}} {{}} {{}}", 
            base_class, 
            active_class, 
            hover_class, 
            focus_class,
            class.get().unwrap_or_default()
        )
    }});

    // Create performance metrics
    let {component_name}_state_for_metrics = {component_name}_state.clone();
    let performance_metrics = ArcMemo::new(move |_| {{
        let state = {component_name}_state_for_metrics.get();
        format!("Clicks: {{}}, Active: {{}}, Hovered: {{}}", 
            state.click_count, 
            state.is_active, 
            state.is_hovered
        )
    }});

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal({component_name}_state.clone());
    theme_manager.track_memo(computed_class.clone());
    theme_manager.track_memo(performance_metrics.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create event handlers with performance monitoring
    let handle_click = {{
        let {component_name}_state = {component_name}_state.clone();
        move |_event: leptos::ev::MouseEvent| {{
            {component_name}_state.update(|state| {{
                state.click_count += 1;
                state.is_active = !state.is_active;
            }});
        }}
    }};

    let handle_mouse_enter = {{
        let {component_name}_state = {component_name}_state.clone();
        move |_event: leptos::ev::MouseEvent| {{
            {component_name}_state.update(|state| {{
                state.is_hovered = true;
            }});
        }}
    }};

    let handle_mouse_leave = {{
        let {component_name}_state = {component_name}_state.clone();
        move |_event: leptos::ev::MouseEvent| {{
            {component_name}_state.update(|state| {{
                state.is_hovered = false;
            }});
        }}
    }};

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    view! {{
        <div class="enhanced-{component_name}-container">
            <div
                class=move || computed_class.get()
                id=move || id.get().unwrap_or_default()
                style=move || style.get().to_string()
                on:click=handle_click
                on:mouseenter=handle_mouse_enter
                on:mouseleave=handle_mouse_leave
            >
                {{children.map(|c| c())}}
            </div>
            
            // Performance monitoring (only in development)
            #[cfg(debug_assertions)]
            <div class="performance-monitor text-xs text-muted-foreground mt-1">
                {{move || performance_metrics.get()}}
            </div>
        </div>
    }}
}}
'''
    
    return template

def test_component(component_name):
    """Test if a component compiles"""
    try:
        result = subprocess.run(
            ["cargo", "check", "-p", f"leptos-shadcn-{component_name}"],
            capture_output=True,
            text=True,
            cwd="."
        )
        return result.returncode == 0
    except Exception as e:
        print(f"Error testing {component_name}: {e}")
        return False

def main():
    """Main migration process"""
    print("🚀 Starting batch component migration for Leptos 0.8.8 signal integration...")
    
    # Change to the project root
    os.chdir(Path(__file__).parent.parent)
    
    success_count = 0
    total_count = 0
    
    # Migrate simple components
    print("\n🎯 Batch 1: Simple Components")
    for component in SIMPLE_COMPONENTS:
        total_count += 1
        if create_signal_managed_component(component, "simple"):
            if test_component(component):
                print(f"  ✅ {component} migrated and compiles successfully")
                success_count += 1
            else:
                print(f"  ❌ {component} has compilation errors")
        else:
            print(f"  ⚠️  {component} migration failed")
    
    # Migrate medium components
    print("\n🎯 Batch 2: Medium Components")
    for component in MEDIUM_COMPONENTS:
        total_count += 1
        if create_signal_managed_component(component, "medium"):
            if test_component(component):
                print(f"  ✅ {component} migrated and compiles successfully")
                success_count += 1
            else:
                print(f"  ❌ {component} has compilation errors")
        else:
            print(f"  ⚠️  {component} migration failed")
    
    # Migrate complex components
    print("\n🎯 Batch 3: Complex Components")
    for component in COMPLEX_COMPONENTS:
        total_count += 1
        if create_signal_managed_component(component, "complex"):
            if test_component(component):
                print(f"  ✅ {component} migrated and compiles successfully")
                success_count += 1
            else:
                print(f"  ❌ {component} has compilation errors")
        else:
            print(f"  ⚠️  {component} migration failed")
    
    print(f"\n🎉 Batch migration completed!")
    print(f"📊 Successfully migrated: {success_count}/{total_count} components")
    
    if success_count == total_count:
        print("✅ All components migrated successfully!")
        return 0
    else:
        print(f"⚠️  {total_count - success_count} components need manual attention")
        return 1

if __name__ == "__main__":
    sys.exit(main())
