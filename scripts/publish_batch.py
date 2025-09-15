#!/usr/bin/env python3
"""
Script to publish component crates in small batches with proper error handling
and disk space management.
"""

import os
import subprocess
import sys
import time
import shutil

def get_disk_space():
    """Get available disk space in GB"""
    try:
        total, used, free = shutil.disk_usage("/")
        return free // (1024**3)  # Convert to GB
    except:
        return 0

def get_component_directories():
    """Get all component directories that have Cargo.toml files"""
    components = []
    leptos_dir = "packages/leptos"
    
    for item in os.listdir(leptos_dir):
        item_path = os.path.join(leptos_dir, item)
        if os.path.isdir(item_path):
            cargo_toml = os.path.join(item_path, "Cargo.toml")
            if os.path.exists(cargo_toml):
                # Check if it's a component crate (has leptos-shadcn- prefix)
                with open(cargo_toml, 'r') as f:
                    content = f.read()
                    if 'name = "leptos-shadcn-' in content:
                        components.append(item)
    
    return sorted(components)

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

def publish_batch(components, batch_num, total_batches):
    """Publish a batch of components"""
    print(f"\n📦 Publishing Batch {batch_num}/{total_batches}")
    print("=" * 50)
    print(f"Components: {', '.join(components)}")
    
    results = []
    successful = 0
    failed = 0
    
    for i, component in enumerate(components, 1):
        print(f"\n[{i}/{len(components)}] Publishing {component}...")
        
        # Check disk space before each publish
        free_space = get_disk_space()
        if free_space < 2:  # Less than 2GB free
            print(f"⚠️  Low disk space: {free_space}GB free. Cleaning up...")
            subprocess.run(["cargo", "clean"], capture_output=True)
            free_space = get_disk_space()
            print(f"✅ Freed up space. Now {free_space}GB free.")
        
        result = publish_component(component)
        results.append(result)
        
        if result["status"] == "success":
            successful += 1
        else:
            failed += 1
        
        # Add delay between publications to respect rate limits
        if i < len(components):
            print("⏳ Waiting 15 seconds before next publication...")
            time.sleep(15)
    
    # Print batch summary
    print(f"\n📊 Batch {batch_num} Summary")
    print("=" * 30)
    print(f"✅ Successful: {successful}")
    print(f"❌ Failed: {failed}")
    print(f"📦 Total: {len(components)}")
    
    if failed > 0:
        print(f"\n❌ Failed Components in Batch {batch_num}:")
        for result in results:
            if result["status"] != "success":
                print(f"  - {result['component']}: {result['error']}")
    
    return results

def main():
    print("🚀 Publishing Component Crates in Batches")
    print("==========================================")
    
    components = get_component_directories()
    print(f"Found {len(components)} component crates to publish")
    
    # Ask for confirmation
    response = input(f"\nProceed with publishing {len(components)} crates in batches? (y/N): ")
    if response.lower() != 'y':
        print("❌ Publishing cancelled by user")
        return
    
    # Check initial disk space
    free_space = get_disk_space()
    print(f"\n💾 Available disk space: {free_space}GB")
    if free_space < 5:
        print("⚠️  Warning: Low disk space. Consider cleaning up first.")
        response = input("Continue anyway? (y/N): ")
        if response.lower() != 'y':
            print("❌ Publishing cancelled due to low disk space")
            return
    
    # Split into batches of 5
    batch_size = 5
    batches = [components[i:i + batch_size] for i in range(0, len(components), batch_size)]
    total_batches = len(batches)
    
    print(f"\n📦 Will publish in {total_batches} batches of up to {batch_size} components each")
    
    all_results = []
    total_successful = 0
    total_failed = 0
    
    for batch_num, batch_components in enumerate(batches, 1):
        batch_results = publish_batch(batch_components, batch_num, total_batches)
        all_results.extend(batch_results)
        
        # Count successes and failures
        for result in batch_results:
            if result["status"] == "success":
                total_successful += 1
            else:
                total_failed += 1
        
        # Add delay between batches
        if batch_num < total_batches:
            print(f"\n⏳ Waiting 30 seconds before next batch...")
            time.sleep(30)
    
    # Print final summary
    print(f"\n🎉 FINAL PUBLICATION SUMMARY")
    print("=" * 40)
    print(f"✅ Total Successful: {total_successful}")
    print(f"❌ Total Failed: {total_failed}")
    print(f"📦 Total Components: {len(components)}")
    
    if total_failed > 0:
        print(f"\n❌ All Failed Components:")
        for result in all_results:
            if result["status"] != "success":
                print(f"  - {result['component']}: {result['error']}")
    
    if total_successful == len(components):
        print(f"\n🎉 ALL {len(components)} COMPONENT CRATES PUBLISHED SUCCESSFULLY!")
        print("🌐 All components are now available on crates.io with signal management features!")
    else:
        print(f"\n⚠️  {total_failed} components failed to publish. Check the errors above.")

if __name__ == "__main__":
    main()
