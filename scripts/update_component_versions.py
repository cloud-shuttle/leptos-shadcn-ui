#!/usr/bin/env python3
"""
Script to update all component crate versions from 0.7.0 to 0.8.0
to reflect the new signal management integration features.
"""

import os
import re
import subprocess
import sys

def update_cargo_toml_version(filepath, old_version, new_version):
    """Update version in Cargo.toml file"""
    print(f"Updating {filepath} from {old_version} to {new_version}")
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Update version line
    content = re.sub(
        rf'^version = "{old_version}"',
        f'version = "{new_version}"',
        content,
        flags=re.MULTILINE
    )
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    print(f"✅ Updated {filepath}")

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
    print("🚀 Updating Component Crate Versions")
    print("====================================")
    
    old_version = "0.7.0"
    new_version = "0.8.0"
    
    components = get_component_directories()
    print(f"Found {len(components)} component crates to update:")
    
    updated_count = 0
    
    for component in components:
        cargo_toml_path = os.path.join("packages/leptos", component, "Cargo.toml")
        
        if os.path.exists(cargo_toml_path):
            try:
                update_cargo_toml_version(cargo_toml_path, old_version, new_version)
                updated_count += 1
            except Exception as e:
                print(f"❌ Error updating {component}: {e}")
        else:
            print(f"⚠️  Cargo.toml not found for {component}")
    
    print(f"\n✅ Successfully updated {updated_count} component crates")
    print(f"📦 All components now at version {new_version}")
    
    # Also update the main workspace Cargo.toml if needed
    workspace_cargo = "Cargo.toml"
    if os.path.exists(workspace_cargo):
        print(f"\n🔄 Checking workspace version...")
        with open(workspace_cargo, 'r') as f:
            content = f.read()
            if f'version = "{old_version}"' in content:
                update_cargo_toml_version(workspace_cargo, old_version, new_version)
                print("✅ Updated workspace version")
            else:
                print("ℹ️  Workspace version already up to date")

if __name__ == "__main__":
    main()
