#!/usr/bin/env python3
"""
Remove placeholder assert!(true) tests from components that now have real tests.
This cleans up the codebase by removing the old placeholder tests.
"""

import os
import re
import subprocess
from pathlib import Path

# Components that now have working real tests
WORKING_COMPONENTS = [
    "avatar", "button", "card", "separator", "badge", "accordion", "alert", 
    "calendar", "carousel", "collapsible", "form", "label", "popover", 
    "resizable", "sheet", "table", "tabs", "toast", "toggle"
]

def remove_placeholder_tests_from_file(file_path):
    """Remove placeholder tests from a specific file"""
    if not os.path.exists(file_path):
        return 0
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Remove lines with assert!(true
        lines = content.split('\n')
        new_lines = []
        removed_count = 0
        
        for line in lines:
            if 'assert!(true' in line:
                removed_count += 1
                # Skip this line (remove it)
                continue
            new_lines.append(line)
        
        if removed_count > 0:
            new_content = '\n'.join(new_lines)
            with open(file_path, 'w') as f:
                f.write(new_content)
            print(f"    Removed {removed_count} placeholder tests from {file_path}")
        
        return removed_count
    except Exception as e:
        print(f"    Error processing {file_path}: {e}")
        return 0

def remove_placeholder_tests_from_component(component_name):
    """Remove placeholder tests from all test files in a component"""
    component_dir = f"packages/leptos/{component_name}/src"
    
    if not os.path.exists(component_dir):
        return 0
    
    total_removed = 0
    
    # Find all test files in the component
    for root, dirs, files in os.walk(component_dir):
        for file in files:
            if file.endswith('.rs') and ('test' in file.lower() or 'tdd' in file.lower()):
                file_path = os.path.join(root, file)
                removed = remove_placeholder_tests_from_file(file_path)
                total_removed += removed
    
    return total_removed

def count_placeholder_tests():
    """Count total placeholder tests in the codebase"""
    try:
        result = subprocess.run(
            ['grep', '-r', 'assert!(true', 'packages/leptos/'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        if result.returncode == 0:
            return len(result.stdout.split('\n')) - 1  # -1 for empty line at end
        else:
            return 0
    except Exception as e:
        print(f"Error counting placeholder tests: {e}")
        return 0

def main():
    """Main function to remove placeholder tests from working components"""
    print("🧹 Removing placeholder tests from working components...")
    print(f"📦 Processing {len(WORKING_COMPONENTS)} components")
    
    initial_count = count_placeholder_tests()
    print(f"📊 Initial placeholder test count: {initial_count}")
    
    total_removed = 0
    
    for component_name in WORKING_COMPONENTS:
        print(f"\n🔨 Removing placeholder tests from {component_name}...")
        removed = remove_placeholder_tests_from_component(component_name)
        total_removed += removed
        if removed > 0:
            print(f"  ✅ Removed {removed} placeholder tests from {component_name}")
        else:
            print(f"  ℹ️  No placeholder tests found in {component_name}")
    
    final_count = count_placeholder_tests()
    
    print(f"\n🎉 Summary:")
    print(f"✅ Removed {total_removed} placeholder tests")
    print(f"📊 Before: {initial_count} placeholder tests")
    print(f"📊 After: {final_count} placeholder tests")
    print(f"📊 Reduction: {initial_count - final_count} tests ({((initial_count - final_count)/initial_count)*100:.1f}%)")
    
    return 0

if __name__ == "__main__":
    exit(main())
