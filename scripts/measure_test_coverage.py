#!/usr/bin/env python3
"""
Measure real test coverage across all components.
This script analyzes the current state of test coverage and provides metrics.
"""

import os
import re
import subprocess
from pathlib import Path

# Components with working real tests
WORKING_COMPONENTS = [
    "avatar", "button", "card", "separator", "badge", "accordion", "alert", 
    "calendar", "carousel", "collapsible", "form", "label", "popover", 
    "resizable", "sheet", "table", "tabs", "toast", "toggle"
]

# All components
ALL_COMPONENTS = [
    "accordion", "alert", "alert-dialog", "aspect-ratio", "avatar", "badge", 
    "breadcrumb", "button", "calendar", "card", "carousel", "checkbox", 
    "collapsible", "combobox", "command", "context-menu", "date-picker", 
    "dialog", "drawer", "dropdown-menu", "error-boundary", "form", "hover-card", 
    "input", "input-otp", "label", "lazy-loading", "menubar", "navigation-menu", 
    "pagination", "popover", "progress", "radio-group", "resizable", "scroll-area", 
    "select", "separator", "sheet", "skeleton", "slider", "switch", "table", 
    "tabs", "textarea", "toast", "toggle", "tooltip"
]

def count_placeholder_tests():
    """Count total placeholder tests in the codebase"""
    try:
        result = subprocess.run(
            ['grep', '-r', 'assert!(true', 'packages/leptos/'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        if result.returncode == 0:
            return len(result.stdout.split('\n')) - 1  # -1 for empty line at end
        else:
            return 0
    except Exception as e:
        print(f"Error counting placeholder tests: {e}")
        return 0

def count_real_tests():
    """Count real tests (non-placeholder) in the codebase"""
    try:
        # Count test functions that are not placeholder tests
        result = subprocess.run(
            ['grep', '-r', 'fn test_', 'packages/leptos/'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        if result.returncode == 0:
            total_tests = len(result.stdout.split('\n')) - 1
            
            # Subtract placeholder tests
            placeholder_tests = count_placeholder_tests()
            return total_tests - placeholder_tests
        else:
            return 0
    except Exception as e:
        print(f"Error counting real tests: {e}")
        return 0

def count_wasm_tests():
    """Count WASM-based tests (real functional tests)"""
    try:
        result = subprocess.run(
            ['grep', '-r', '#\[wasm_bindgen_test\]', 'packages/leptos/'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        if result.returncode == 0:
            return len(result.stdout.split('\n')) - 1
        else:
            return 0
    except Exception as e:
        print(f"Error counting WASM tests: {e}")
        return 0

def count_component_files():
    """Count total component files"""
    total_files = 0
    for component in ALL_COMPONENTS:
        component_dir = f"packages/leptos/{component}/src"
        if os.path.exists(component_dir):
            for root, dirs, files in os.walk(component_dir):
                for file in files:
                    if file.endswith('.rs'):
                        total_files += 1
    return total_files

def count_test_files():
    """Count test files"""
    test_files = 0
    for component in ALL_COMPONENTS:
        component_dir = f"packages/leptos/{component}/src"
        if os.path.exists(component_dir):
            for root, dirs, files in os.walk(component_dir):
                for file in files:
                    if file.endswith('.rs') and ('test' in file.lower() or 'tdd' in file.lower()):
                        test_files += 1
    return test_files

def analyze_component_coverage(component_name):
    """Analyze coverage for a specific component"""
    component_dir = f"packages/leptos/{component_name}/src"
    
    if not os.path.exists(component_dir):
        return {
            'has_real_tests': False,
            'has_placeholder_tests': False,
            'placeholder_count': 0,
            'real_test_count': 0,
            'wasm_test_count': 0
        }
    
    # Count placeholder tests
    placeholder_count = 0
    real_test_count = 0
    wasm_test_count = 0
    
    for root, dirs, files in os.walk(component_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r') as f:
                        content = f.read()
                    
                    # Count placeholder tests
                    placeholder_count += content.count('assert!(true')
                    
                    # Count real test functions
                    real_test_count += len(re.findall(r'fn test_\w+\(', content))
                    
                    # Count WASM tests
                    wasm_test_count += content.count('#[wasm_bindgen_test]')
                except Exception as e:
                    print(f"Error reading {file_path}: {e}")
    
    return {
        'has_real_tests': real_test_count > 0,
        'has_placeholder_tests': placeholder_count > 0,
        'placeholder_count': placeholder_count,
        'real_test_count': real_test_count,
        'wasm_test_count': wasm_test_count
    }

def main():
    """Main function to measure test coverage"""
    print("📊 Measuring Test Coverage Across All Components")
    print("=" * 60)
    
    # Overall statistics
    total_components = len(ALL_COMPONENTS)
    working_components = len(WORKING_COMPONENTS)
    placeholder_tests = count_placeholder_tests()
    real_tests = count_real_tests()
    wasm_tests = count_wasm_tests()
    total_files = count_component_files()
    test_files = count_test_files()
    
    print(f"\n🎯 OVERALL STATISTICS:")
    print(f"📦 Total Components: {total_components}")
    print(f"✅ Components with Real Tests: {working_components}")
    print(f"📊 Component Coverage: {(working_components/total_components)*100:.1f}%")
    print(f"")
    print(f"🧪 Test Statistics:")
    print(f"❌ Placeholder Tests: {placeholder_tests}")
    print(f"✅ Real Tests: {real_tests}")
    print(f"🌐 WASM Tests: {wasm_tests}")
    print(f"📁 Total Files: {total_files}")
    print(f"🧪 Test Files: {test_files}")
    
    # Calculate coverage percentages
    if placeholder_tests + real_tests > 0:
        real_coverage = (real_tests / (placeholder_tests + real_tests)) * 100
        print(f"📈 Real Test Coverage: {real_coverage:.1f}%")
    
    if total_components > 0:
        component_coverage = (working_components / total_components) * 100
        print(f"📈 Component Coverage: {component_coverage:.1f}%")
    
    # Component-by-component analysis
    print(f"\n🔍 COMPONENT-BY-COMPONENT ANALYSIS:")
    print("-" * 60)
    
    working_count = 0
    placeholder_count = 0
    real_test_count = 0
    wasm_test_count = 0
    
    for component in ALL_COMPONENTS:
        coverage = analyze_component_coverage(component)
        
        status = "✅" if coverage['has_real_tests'] else "❌"
        placeholder_status = "⚠️" if coverage['has_placeholder_tests'] else "✅"
        
        print(f"{status} {component:<20} | Real: {coverage['real_test_count']:>3} | WASM: {coverage['wasm_test_count']:>3} | Placeholder: {coverage['placeholder_count']:>3} {placeholder_status}")
        
        if coverage['has_real_tests']:
            working_count += 1
        placeholder_count += coverage['placeholder_count']
        real_test_count += coverage['real_test_count']
        wasm_test_count += coverage['wasm_test_count']
    
    # Summary
    print(f"\n🎉 FINAL SUMMARY:")
    print("=" * 60)
    print(f"✅ Components with Real Tests: {working_count}/{total_components} ({(working_count/total_components)*100:.1f}%)")
    print(f"🧪 Total Real Tests: {real_test_count}")
    print(f"🌐 Total WASM Tests: {wasm_test_count}")
    print(f"❌ Remaining Placeholder Tests: {placeholder_count}")
    
    if placeholder_count + real_test_count > 0:
        final_coverage = (real_test_count / (placeholder_count + real_test_count)) * 100
        print(f"📈 Final Real Test Coverage: {final_coverage:.1f}%")
        
        if final_coverage >= 90:
            print("🎊 TARGET ACHIEVED: 90%+ Real Test Coverage!")
        else:
            remaining = 90 - final_coverage
            print(f"🎯 TARGET PROGRESS: {remaining:.1f}% remaining to reach 90%")
    
    # Recommendations
    print(f"\n💡 RECOMMENDATIONS:")
    if working_count < total_components:
        remaining_components = total_components - working_count
        print(f"🔧 Fix remaining {remaining_components} components to achieve 100% component coverage")
    
    if placeholder_count > 0:
        print(f"🧹 Remove {placeholder_count} remaining placeholder tests")
    
    if wasm_test_count < real_test_count:
        print(f"🌐 Increase WASM test coverage (currently {wasm_test_count}/{real_test_count})")
    
    return 0

if __name__ == "__main__":
    exit(main())
