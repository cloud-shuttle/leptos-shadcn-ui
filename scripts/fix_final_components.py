#!/usr/bin/env python3
"""
Script to fix the final components with proper variable naming
"""

import os
import re

def fix_component(component_name):
    """Fix a single component"""
    print(f"Fixing {component_name}")
    
    file_path = f"packages/leptos/{component_name}/src/signal_managed.rs"
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix struct names
    old_struct = f"SignalManaged{component_name.replace('-', '_').title()}State"
    new_struct = f"SignalManaged{component_name.replace('-', '').title()}State"
    content = content.replace(old_struct, new_struct)
    
    # Fix function names
    old_func = f"SignalManaged{component_name.replace('-', '_').title()}"
    new_func = f"SignalManaged{component_name.replace('-', '').title()}"
    content = content.replace(old_func, new_func)
    
    old_enhanced = f"Enhanced{component_name.replace('-', '_').title()}"
    new_enhanced = f"Enhanced{component_name.replace('-', '').title()}"
    content = content.replace(old_enhanced, new_enhanced)
    
    # Fix variable names
    old_var = f"{component_name}_state"
    new_var = f"{component_name.replace('-', '_')}_state"
    content = content.replace(old_var, new_var)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed {component_name}")

def main():
    """Main function"""
    components = [
        'radio-group', 'context-menu', 'navigation-menu', 
        'dropdown-menu', 'scroll-area', 'hover-card'
    ]
    
    for component in components:
        fix_component(component)
    
    print("Done!")

if __name__ == "__main__":
    main()
