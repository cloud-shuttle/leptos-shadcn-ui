#!/usr/bin/env python3
"""
Script to fix invalid variable names in signal_managed.rs files
"""

import os
import re
import glob

def fix_component_file(file_path):
    """Fix invalid variable names in a component file"""
    print(f"Fixing {file_path}")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Extract component name from path
    component_name = os.path.basename(os.path.dirname(file_path))
    
    # Fix struct names (replace underscores with proper camelCase)
    struct_name = f"SignalManaged{component_name.replace('-', '').title()}State"
    old_struct_name = f"SignalManaged{component_name.replace('-', '_').title()}State"
    content = content.replace(old_struct_name, struct_name)
    
    # Fix function names
    func_name = f"SignalManaged{component_name.replace('-', '').title()}"
    old_func_name = f"SignalManaged{component_name.replace('-', '_').title()}"
    content = content.replace(old_func_name, func_name)
    
    enhanced_func_name = f"Enhanced{component_name.replace('-', '').title()}"
    old_enhanced_func_name = f"Enhanced{component_name.replace('-', '_').title()}"
    content = content.replace(old_enhanced_func_name, enhanced_func_name)
    
    # Fix variable names (replace hyphens with underscores)
    var_name = component_name.replace('-', '_')
    content = content.replace(f"{component_name}_state", f"{var_name}_state")
    content = content.replace(f"{component_name}_state_for_class", f"{var_name}_state_for_class")
    content = content.replace(f"{component_name}_state_for_metrics", f"{var_name}_state_for_metrics")
    content = content.replace(f"{component_name}_state_for_disabled", f"{var_name}_state_for_disabled")
    
    with open(file_path, 'w') as f:
        f.write(content)

def main():
    """Main function to fix all component files"""
    # Find all signal_managed.rs files
    pattern = "packages/leptos/*/src/signal_managed.rs"
    files = glob.glob(pattern)
    
    print(f"Found {len(files)} signal_managed.rs files")
    
    for file_path in files:
        try:
            fix_component_file(file_path)
        except Exception as e:
            print(f"Error fixing {file_path}: {e}")
    
    print("Done fixing component names!")

if __name__ == "__main__":
    main()
