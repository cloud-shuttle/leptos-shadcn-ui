#!/usr/bin/env python3
"""
Performance Test Runner
Runs all performance tests and provides comprehensive reporting.
"""

import subprocess
import sys
import os
import json
import time
from pathlib import Path

def run_performance_tests():
    """Run all performance tests"""
    print("⚡ Running Performance Tests...")
    print("=" * 50)
    
    performance_dir = "tests/performance"
    
    if not os.path.exists(performance_dir):
        print("❌ Performance tests directory not found")
        return False
    
    test_files = [f for f in os.listdir(performance_dir) if f.endswith('.rs')]
    
    if not test_files:
        print("❌ No performance test files found")
        return False
    
    print(f"📁 Found {len(test_files)} performance test files:")
    for test_file in test_files:
        print(f"   - {test_file}")
    
    print("\n🚀 Running performance tests...")
    
    results = {
        "timestamp": time.time(),
        "tests": [],
        "summary": {
            "total_tests": 0,
            "passed": 0,
            "failed": 0,
            "total_time": 0
        }
    }
    
    start_time = time.time()
    
    try:
        # Run performance tests
        result = subprocess.run(
            ['cargo', 'test', '--test', 'performance'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        
        end_time = time.time()
        total_time = end_time - start_time
        
        results["summary"]["total_time"] = total_time
        
        if result.returncode == 0:
            print("✅ All performance tests passed!")
            results["summary"]["passed"] = len(test_files)
            results["summary"]["total_tests"] = len(test_files)
        else:
            print("❌ Some performance tests failed!")
            results["summary"]["failed"] = len(test_files)
            results["summary"]["total_tests"] = len(test_files)
        
        print("\n📊 Test Results:")
        print(result.stdout)
        
        if result.stderr:
            print("\n❌ Errors:")
            print(result.stderr)
        
        # Save results to JSON file
        results_file = "performance_test_results.json"
        with open(results_file, 'w') as f:
            json.dump(results, f, indent=2)
        
        print(f"\n💾 Results saved to: {results_file}")
        
        return result.returncode == 0
        
    except Exception as e:
        print(f"❌ Error running performance tests: {e}")
        return False

def main():
    """Main function"""
    success = run_performance_tests()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
