#!/usr/bin/env python3
"""
Integration Test Runner
Runs all integration tests and provides comprehensive reporting.
"""

import subprocess
import sys
import os

def run_integration_tests():
    """Run all integration tests"""
    print("🧪 Running Integration Tests...")
    print("=" * 50)
    
    integration_dir = "tests/integration"
    
    if not os.path.exists(integration_dir):
        print("❌ Integration tests directory not found")
        return False
    
    test_files = [f for f in os.listdir(integration_dir) if f.endswith('.rs')]
    
    if not test_files:
        print("❌ No integration test files found")
        return False
    
    print(f"📁 Found {len(test_files)} integration test files:")
    for test_file in test_files:
        print(f"   - {test_file}")
    
    print("\n🚀 Running integration tests...")
    
    try:
        # Run integration tests
        result = subprocess.run(
            ['cargo', 'test', '--test', 'integration'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        
        if result.returncode == 0:
            print("✅ All integration tests passed!")
            print("\n📊 Test Results:")
            print(result.stdout)
            return True
        else:
            print("❌ Some integration tests failed!")
            print("\n📊 Test Results:")
            print(result.stdout)
            print("\n❌ Errors:")
            print(result.stderr)
            return False
            
    except Exception as e:
        print(f"❌ Error running integration tests: {e}")
        return False

def main():
    """Main function"""
    success = run_integration_tests()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
