#!/usr/bin/env python3
"""
Script to publish the next batch of 10 component crates.
Batch 2: Components 6-15
"""

import os
import subprocess
import sys
import time

def publish_component(component):
    """Publish a single component crate"""
    component_path = os.path.join("packages/leptos", component)
    
    try:
        print(f"🚀 Publishing {component}...")
        
        # Change to component directory
        original_cwd = os.getcwd()
        os.chdir(component_path)
        
        # Run cargo publish
        result = subprocess.run(
            ["cargo", "publish"],
            capture_output=True,
            text=True,
            timeout=300  # 5 minute timeout
        )
        
        if result.returncode == 0:
            print(f"✅ Successfully published {component}")
            return {"component": component, "status": "success", "error": None}
        else:
            error_msg = result.stderr.strip()
            print(f"❌ Failed to publish {component}: {error_msg}")
            return {"component": component, "status": "failed", "error": error_msg}
            
    except subprocess.TimeoutExpired:
        print(f"⏰ Timeout publishing {component}")
        return {"component": component, "status": "timeout", "error": "Timeout after 5 minutes"}
    except Exception as e:
        print(f"💥 Exception publishing {component}: {str(e)}")
        return {"component": component, "status": "exception", "error": str(e)}
    finally:
        os.chdir(original_cwd)

def main():
    print("🚀 Publishing Batch 2: Components 6-15")
    print("======================================")
    
    # Next 10 components to publish (alphabetically)
    components = [
        "accordion",
        "alert-dialog", 
        "aspect-ratio",
        "avatar",
        "breadcrumb",
        "calendar",
        "carousel",
        "checkbox",
        "collapsible",
        "combobox"
    ]
    
    print(f"Publishing {len(components)} components:")
    for i, comp in enumerate(components, 1):
        print(f"  {i}. {comp}")
    
    print(f"\n📦 Starting publication of {len(components)} crates...")
    
    results = []
    successful = 0
    failed = 0
    
    for i, component in enumerate(components, 1):
        print(f"\n[{i}/{len(components)}] Publishing {component}...")
        result = publish_component(component)
        results.append(result)
        
        if result["status"] == "success":
            successful += 1
        else:
            failed += 1
        
        # Add delay between publications to respect rate limits
        if i < len(components):
            print("⏳ Waiting 10 seconds before next publication...")
            time.sleep(10)
    
    # Print summary
    print(f"\n📊 Batch 2 Summary")
    print("=" * 20)
    print(f"✅ Successful: {successful}")
    print(f"❌ Failed: {failed}")
    print(f"📦 Total: {len(components)}")
    
    if failed > 0:
        print(f"\n❌ Failed Components:")
        for result in results:
            if result["status"] != "success":
                print(f"  - {result['component']}: {result['error']}")
    
    if successful == len(components):
        print(f"\n🎉 ALL {len(components)} COMPONENTS IN BATCH 2 PUBLISHED SUCCESSFULLY!")
        print("🌐 Components 6-15 are now available on crates.io with signal management features!")
    else:
        print(f"\n⚠️  {failed} components failed to publish. Check the errors above.")

if __name__ == "__main__":
    main()
