#!/usr/bin/env python3
"""
Script to publish the final batches of component crates.
Batch 4: Components 26-35 (10 components)
Batch 5: Components 36-49 (14 components)
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

def run_batch(batch_name, components, batch_num):
    """Run a batch of component publications"""
    print(f"\n🚀 {batch_name}: Components {batch_num}")
    print("=" * 50)
    
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
    
    # Print batch summary
    print(f"\n📊 {batch_name} Summary")
    print("=" * 30)
    print(f"✅ Successful: {successful}")
    print(f"❌ Failed: {failed}")
    print(f"📦 Total: {len(components)}")
    
    if failed > 0:
        print(f"\n❌ Failed Components:")
        for result in results:
            if result["status"] != "success":
                print(f"  - {result['component']}: {result['error']}")
    
    if successful == len(components):
        print(f"\n🎉 ALL {len(components)} COMPONENTS IN {batch_name.upper()} PUBLISHED SUCCESSFULLY!")
    else:
        print(f"\n⚠️  {failed} components failed to publish. Check the errors above.")
    
    return results

def main():
    print("🚀 Publishing Final Batches: Components 26-49")
    print("=============================================")
    
    # Batch 4: Components 26-35 (10 components)
    batch_4_components = [
        "menubar",
        "navigation-menu",
        "pagination",
        "popover",
        "progress",
        "radio-group",
        "resizable",
        "scroll-area",
        "select",
        "separator"
    ]
    
    # Batch 5: Components 36-49 (14 components)
    batch_5_components = [
        "sheet",
        "skeleton",
        "slider",
        "sonner",
        "switch",
        "table",
        "tabs",
        "textarea",
        "toast",
        "toggle",
        "toggle-group",
        "tooltip",
        "tree"
    ]
    
    all_results = []
    
    # Run Batch 4
    batch_4_results = run_batch("Batch 4", batch_4_components, "26-35")
    all_results.extend(batch_4_results)
    
    # Clean up between batches to prevent disk space issues
    print(f"\n🧹 Cleaning up build artifacts between batches...")
    try:
        subprocess.run(["cargo", "clean"], capture_output=True, text=True)
        print("✅ Cleanup completed")
    except Exception as e:
        print(f"⚠️  Cleanup failed: {e}")
    
    # Run Batch 5
    batch_5_results = run_batch("Batch 5", batch_5_components, "36-49")
    all_results.extend(batch_5_results)
    
    # Final summary
    total_successful = sum(1 for r in all_results if r["status"] == "success")
    total_failed = sum(1 for r in all_results if r["status"] != "success")
    total_components = len(all_results)
    
    print(f"\n🎯 FINAL SUMMARY")
    print("=" * 50)
    print(f"✅ Total Successful: {total_successful}")
    print(f"❌ Total Failed: {total_failed}")
    print(f"📦 Total Components: {total_components}")
    
    if total_failed == 0:
        print(f"\n🏆 MISSION ACCOMPLISHED!")
        print("🎉 ALL 49 COMPONENT CRATES PUBLISHED SUCCESSFULLY!")
        print("🌐 The entire leptos-shadcn-ui ecosystem is now available on crates.io!")
        print("🚀 All components include signal management features for Leptos 0.8.8!")
    else:
        print(f"\n⚠️  {total_failed} components failed to publish.")
        print("Failed components:")
        for result in all_results:
            if result["status"] != "success":
                print(f"  - {result['component']}: {result['error']}")

if __name__ == "__main__":
    main()
