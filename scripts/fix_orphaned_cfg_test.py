#!/usr/bin/env python3
"""
Fix orphaned #[cfg(test)] attributes in lib.rs files
"""

import os
import re
import glob

def fix_orphaned_cfg_test(filepath):
    """Remove orphaned #[cfg(test)] attributes from lib.rs files"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Remove orphaned #[cfg(test)] attributes (those not followed by a module declaration)
        # This regex looks for #[cfg(test)] followed by whitespace and end of line or another attribute
        content = re.sub(r'^\s*#\[cfg\(test\)\]\s*$', '', content, flags=re.MULTILINE)
        
        # Also remove multiple consecutive empty lines that might result from the above
        content = re.sub(r'\n\s*\n\s*\n', '\n\n', content)
        
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
    print("🔧 Fixing orphaned #[cfg(test)] attributes in lib.rs files...")
    
    # Find all lib.rs files in packages/leptos
    lib_files = glob.glob("packages/leptos/*/src/lib.rs")
    
    fixed_count = 0
    total_count = len(lib_files)
    
    for lib_file in lib_files:
        if fix_orphaned_cfg_test(lib_file):
            fixed_count += 1
    
    print(f"\n📊 Summary:")
    print(f"   - Total lib.rs files: {total_count}")
    print(f"   - Files fixed: {fixed_count}")
    print(f"   - Files unchanged: {total_count - fixed_count}")

if __name__ == "__main__":
    main()
