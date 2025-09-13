#!/usr/bin/env python3
"""
Script to publish all component crates to crates.io
with proper error handling and progress tracking.
"""

import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
import threading

# Thread-safe print function
print_lock = threading.Lock()

def safe_print(message):
    with print_lock:
        print(message)

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
        safe_print(f"🚀 Publishing {component}...")
        
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
            safe_print(f"✅ Successfully published {component}")
            return {"component": component, "status": "success", "error": None}
        else:
            error_msg = result.stderr.strip()
            safe_print(f"❌ Failed to publish {component}: {error_msg}")
            return {"component": component, "status": "failed", "error": error_msg}
            
    except subprocess.TimeoutExpired:
        safe_print(f"⏰ Timeout publishing {component}")
        return {"component": component, "status": "timeout", "error": "Timeout after 5 minutes"}
    except Exception as e:
        safe_print(f"💥 Exception publishing {component}: {str(e)}")
        return {"component": component, "status": "exception", "error": str(e)}
    finally:
        os.chdir(original_cwd)

def main():
    print("🚀 Publishing All Component Crates")
    print("==================================")
    
    components = get_component_directories()
    print(f"Found {len(components)} component crates to publish")
    
    # Ask for confirmation
    response = input(f"\nProceed with publishing {len(components)} crates? (y/N): ")
    if response.lower() != 'y':
        print("❌ Publishing cancelled by user")
        return
    
    print(f"\n📦 Starting publication of {len(components)} crates...")
    print("This may take a while due to crates.io rate limiting...")
    
    # Track results
    results = []
    successful = 0
    failed = 0
    
    # Publish components sequentially to avoid rate limiting
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
    print(f"\n📊 Publication Summary")
    print("=====================")
    print(f"✅ Successful: {successful}")
    print(f"❌ Failed: {failed}")
    print(f"📦 Total: {len(components)}")
    
    if failed > 0:
        print(f"\n❌ Failed Components:")
        for result in results:
            if result["status"] != "success":
                print(f"  - {result['component']}: {result['error']}")
    
    if successful == len(components):
        print(f"\n🎉 All {len(components)} component crates published successfully!")
        print("🌐 All components are now available on crates.io with signal management features!")
    else:
        print(f"\n⚠️  {failed} components failed to publish. Check the errors above.")

if __name__ == "__main__":
    main()
