#!/usr/bin/env python3
"""
Script to fix the remaining components with variable naming issues
"""

import os
import re
import glob

def fix_component_variables(file_path):
    """Fix variable names in a component file"""
    print(f"Fixing variables in {file_path}")
    
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
    
    # Fix ALL variable names with hyphens - this is the key fix
    var_name = component_name.replace('-', '_')
    
    # Replace all instances of component-name_state with component_name_state
    content = re.sub(rf'{re.escape(component_name)}_state', f'{var_name}_state', content)
    content = re.sub(rf'{re.escape(component_name)}_state_for_class', f'{var_name}_state_for_class', content)
    content = re.sub(rf'{re.escape(component_name)}_state_for_metrics', f'{var_name}_state_for_metrics', content)
    content = re.sub(rf'{re.escape(component_name)}_state_for_disabled', f'{var_name}_state_for_disabled', content)
    
    # Also fix any remaining hyphens in variable names
    content = re.sub(r'let ([a-zA-Z_]+)-([a-zA-Z_]+) =', r'let \1_\2 =', content)
    content = re.sub(r'let ([a-zA-Z_]+)-([a-zA-Z_]+)-([a-zA-Z_]+) =', r'let \1_\2_\3 =', content)
    
    with open(file_path, 'w') as f:
        f.write(content)

def add_missing_dependencies(component_name):
    """Add missing dependencies to Cargo.toml"""
    cargo_path = f"packages/leptos/{component_name}/Cargo.toml"
    if not os.path.exists(cargo_path):
        return
    
    with open(cargo_path, 'r') as f:
        content = f.read()
    
    # Check if leptos-style is already present
    if 'leptos-style' not in content:
        # Add leptos-style dependency
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.startswith('leptos = { workspace = true'):
                lines.insert(i + 1, 'leptos-style = { workspace = true }')
                break
        
        with open(cargo_path, 'w') as f:
            f.write('\n'.join(lines))
        print(f"Added leptos-style dependency to {cargo_path}")

def add_missing_module_declaration(component_name):
    """Add missing module declaration to lib.rs"""
    lib_path = f"packages/leptos/{component_name}/src/lib.rs"
    if not os.path.exists(lib_path):
        return
    
    with open(lib_path, 'r') as f:
        content = f.read()
    
    # Check if module declaration is missing
    if 'pub mod signal_managed;' not in content and 'pub use signal_managed::*;' in content:
        # Add module declaration before the use statement
        content = content.replace(
            'pub use signal_managed::*;',
            'pub mod signal_managed;\npub use signal_managed::*;'
        )
        
        with open(lib_path, 'w') as f:
            f.write(content)
        print(f"Added module declaration to {lib_path}")

def main():
    """Main function to fix all remaining components"""
    # Components that need fixing
    components = [
        'input-otp', 'radio-group', 'context-menu', 'navigation-menu', 
        'dropdown-menu', 'scroll-area', 'hover-card'
    ]
    
    for component in components:
        print(f"\n=== Fixing {component} ===")
        
        # Fix variables in signal_managed.rs
        signal_managed_path = f"packages/leptos/{component}/src/signal_managed.rs"
        if os.path.exists(signal_managed_path):
            fix_component_variables(signal_managed_path)
        
        # Add missing dependencies
        add_missing_dependencies(component)
        
        # Add missing module declarations
        add_missing_module_declaration(component)
    
    print("\nDone fixing all remaining components!")

if __name__ == "__main__":
    main()
