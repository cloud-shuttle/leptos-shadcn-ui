#!/usr/bin/env python3
"""
Script to update all package versions from 0.8.1 to 0.9.0
"""

import os
import re
import glob

def update_version_in_file(filepath):
    """Update version from 0.8.1 to 0.9.0 in a Cargo.toml file"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Update version = "0.8.1" to version = "0.9.0"
        updated_content = re.sub(r'version = "0\.8\.1"', 'version = "0.9.0"', content)
        
        # Also update dependency versions
        updated_content = re.sub(r'version = "0\.8\.1"', 'version = "0.9.0"', updated_content)
        
        if content != updated_content:
            with open(filepath, 'w') as f:
                f.write(updated_content)
            print(f"✅ Updated: {filepath}")
            return True
        else:
            print(f"⏭️  No changes needed: {filepath}")
            return False
            
    except Exception as e:
        print(f"❌ Error updating {filepath}: {e}")
        return False

def main():
    print("🚀 Updating all package versions from 0.8.1 to 0.9.0...")
    
    # Find all Cargo.toml files
    cargo_files = []
    
    # Add root Cargo.toml
    cargo_files.append("Cargo.toml")
    
    # Add all package Cargo.toml files
    for root, dirs, files in os.walk("packages"):
        if "Cargo.toml" in files:
            cargo_files.append(os.path.join(root, "Cargo.toml"))
    
    # Add performance-audit Cargo.toml
    if os.path.exists("performance-audit/Cargo.toml"):
        cargo_files.append("performance-audit/Cargo.toml")
    
    updated_count = 0
    total_count = len(cargo_files)
    
    for cargo_file in cargo_files:
        if os.path.exists(cargo_file):
            if update_version_in_file(cargo_file):
                updated_count += 1
    
    print(f"\n🎉 Version update complete!")
    print(f"📊 Updated {updated_count}/{total_count} files")
    
    if updated_count > 0:
        print("\n🔍 Verifying changes...")
        # Check if any 0.8.1 versions remain
        remaining_versions = []
        for cargo_file in cargo_files:
            if os.path.exists(cargo_file):
                try:
                    with open(cargo_file, 'r') as f:
                        content = f.read()
                        if 'version = "0.8.1"' in content:
                            remaining_versions.append(cargo_file)
                except:
                    pass
        
        if remaining_versions:
            print(f"⚠️  Warning: {len(remaining_versions)} files still contain version 0.8.1:")
            for file in remaining_versions:
                print(f"   - {file}")
        else:
            print("✅ All versions successfully updated to 0.9.0!")

if __name__ == "__main__":
    main()
