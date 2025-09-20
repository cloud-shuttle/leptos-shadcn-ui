#!/usr/bin/env python3
"""
Script to fix common compilation errors in test files
"""

import os
import re
import glob

def fix_file(filepath):
    """Fix common compilation errors in a test file"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix data-test attributes (not supported in Leptos)
        content = re.sub(r'data-test="([^"]*)"', r'data_test="\1"', content)
        
        # Fix function names with hyphens
        content = re.sub(r'fn test_([a-zA-Z0-9]+)-([a-zA-Z0-9_]+)\(', r'fn test_\1_\2(', content)
        
        # Remove unsupported children props for specific components
        if 'combobox' in filepath.lower():
            # Remove children from combobox tests
            content = re.sub(r'<Combobox[^>]*>\s*"[^"]*"\s*</Combobox>', 
                           lambda m: m.group(0).replace('"Class Test combobox"', ''), content)
        
        if 'toast' in filepath.lower():
            # Remove children from toast tests
            content = re.sub(r'<Toast[^>]*>\s*"[^"]*"\s*</Toast>', 
                           lambda m: m.group(0).replace('"toast content"', ''), content)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"✅ Fixed: {filepath}")
            return True
        else:
            print(f"⏭️  No changes needed: {filepath}")
            return False
            
    except Exception as e:
        print(f"❌ Error fixing {filepath}: {e}")
        return False

def main():
    print("🔧 Fixing common compilation errors in test files...")
    
    # Find all real_tests.rs files
    test_files = []
    for root, dirs, files in os.walk("packages/leptos"):
        if "real_tests.rs" in files:
            test_files.append(os.path.join(root, "real_tests.rs"))
    
    fixed_count = 0
    total_count = len(test_files)
    
    for test_file in test_files:
        if fix_file(test_file):
            fixed_count += 1
    
    print(f"\n🎉 Fix complete!")
    print(f"📊 Fixed {fixed_count}/{total_count} files")

if __name__ == "__main__":
    main()
