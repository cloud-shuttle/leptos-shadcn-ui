#!/usr/bin/env python3
"""
Script to fix signal management dependencies in all component crates
by replacing path dependencies with version dependencies for publishing.
"""

import os
import re

def fix_cargo_toml_dependencies(filepath):
    """Fix signal management dependency in Cargo.toml file"""
    print(f"Fixing dependencies in {filepath}")
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Replace path dependency with version dependency
    old_dependency = 'leptos-shadcn-signal-management = { path = "../../signal-management" }'
    new_dependency = 'leptos-shadcn-signal-management = "0.1.0"'
    
    if old_dependency in content:
        content = content.replace(old_dependency, new_dependency)
        print(f"✅ Updated signal management dependency in {filepath}")
    else:
        print(f"ℹ️  No signal management dependency found in {filepath}")
    
    with open(filepath, 'w') as f:
        f.write(content)

def get_component_directories():
    """Get all component directories that have Cargo.toml files"""
    components = []
    leptos_dir = "packages/leptos"
    
    for item in os.listdir(leptos_dir):
        item_path = os.path.join(leptos_dir, item)
        if os.path.isdir(item_path):
            cargo_toml = os.path.join(item_path, "Cargo.toml")
            if os.path.exists(cargo_toml):
                # Check if it's a component crate (has leptos-shadcn- prefix)
                with open(cargo_toml, 'r') as f:
                    content = f.read()
                    if 'name = "leptos-shadcn-' in content:
                        components.append(item)
    
    return sorted(components)

def main():
    print("🔧 Fixing Signal Management Dependencies")
    print("=======================================")
    
    components = get_component_directories()
    print(f"Found {len(components)} component crates to fix")
    
    updated_count = 0
    
    for component in components:
        cargo_toml_path = os.path.join("packages/leptos", component, "Cargo.toml")
        
        if os.path.exists(cargo_toml_path):
            try:
                fix_cargo_toml_dependencies(cargo_toml_path)
                updated_count += 1
            except Exception as e:
                print(f"❌ Error fixing {component}: {e}")
        else:
            print(f"⚠️  Cargo.toml not found for {component}")
    
    print(f"\n✅ Successfully fixed dependencies in {updated_count} component crates")
    print("📦 All components now reference the published signal management crate")

if __name__ == "__main__":
    main()
