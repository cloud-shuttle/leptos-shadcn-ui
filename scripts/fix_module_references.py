#!/usr/bin/env python3
"""
Fix module references in lib.rs files by removing real_tests module declarations
"""

import os
import re
import glob

def fix_lib_rs_file(filepath):
    """Remove real_tests module declarations from lib.rs files"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Remove mod real_tests; declarations
        original_content = content
        content = re.sub(r'^\s*mod real_tests;\s*$', '', content, flags=re.MULTILINE)
        
        # Also remove any conditional mod real_tests; declarations
        content = re.sub(r'^\s*#\[cfg\(test\)\]\s*\n\s*mod real_tests;\s*$', '', content, flags=re.MULTILINE)
        
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
    print("🔧 Fixing module references in lib.rs files...")
    
    # Find all lib.rs files in packages/leptos
    lib_files = glob.glob("packages/leptos/*/src/lib.rs")
    
    fixed_count = 0
    total_count = len(lib_files)
    
    for lib_file in lib_files:
        if fix_lib_rs_file(lib_file):
            fixed_count += 1
    
    print(f"\n📊 Summary:")
    print(f"   - Total lib.rs files: {total_count}")
    print(f"   - Files fixed: {fixed_count}")
    print(f"   - Files unchanged: {total_count - fixed_count}")

if __name__ == "__main__":
    main()
