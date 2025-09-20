#!/usr/bin/env python3
"""
Advanced Integration Test Runner
Runs complex workflow integration tests
"""

import subprocess
import sys
import os

def run_integration_tests():
    """Run all advanced integration tests"""
    print("🚀 Running Advanced Integration Tests")
    print("=" * 50)
    
    test_files = [
        "tests/integration/ecommerce_workflow_tests.rs",
        "tests/integration/dashboard_workflow_tests.rs", 
        "tests/integration/advanced_form_workflow_tests.rs"
    ]
    
    results = {}
    
    for test_file in test_files:
        if not os.path.exists(test_file):
            print(f"❌ {test_file} not found")
            results[test_file] = False
            continue
            
        print(f"\n🧪 Running {test_file}...")
        
        try:
            # Extract test module name from file
            module_name = os.path.basename(test_file).replace('.rs', '')
            
            result = subprocess.run([
                "cargo", "test", 
                "--test", module_name,
                "--", "--nocapture"
            ], capture_output=True, text=True, timeout=60)
            
            if result.returncode == 0:
                print(f"✅ {test_file}: PASSED")
                results[test_file] = True
            else:
                print(f"❌ {test_file}: FAILED")
                print(f"   Error: {result.stderr[:200]}...")
                results[test_file] = False
                
        except subprocess.TimeoutExpired:
            print(f"⏰ {test_file}: TIMEOUT")
            results[test_file] = False
        except Exception as e:
            print(f"❌ {test_file}: ERROR - {e}")
            results[test_file] = False
    
    # Summary
    passed = sum(1 for success in results.values() if success)
    total = len(results)
    
    print(f"\n📊 Integration Test Results: {passed}/{total} passed")
    
    if passed == total:
        print("🎉 All advanced integration tests passed!")
        return True
    else:
        print("⚠️  Some integration tests failed")
        return False

if __name__ == "__main__":
    success = run_integration_tests()
    sys.exit(0 if success else 1)
