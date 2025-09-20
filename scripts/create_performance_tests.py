#!/usr/bin/env python3
"""
Create comprehensive performance tests for large datasets and complex scenarios.
This script generates performance tests that measure rendering time, memory usage, and scalability.
"""

import os
import re
from pathlib import Path

# Performance test scenarios
PERFORMANCE_SCENARIOS = {
    "large_dataset_rendering": {
        "name": "Large Dataset Rendering",
        "description": "Test rendering performance with large datasets",
        "test_file": "large_dataset_performance_tests.rs"
    },
    "memory_usage": {
        "name": "Memory Usage Analysis",
        "description": "Test memory consumption with various component counts",
        "test_file": "memory_usage_tests.rs"
    },
    "scalability": {
        "name": "Component Scalability",
        "description": "Test how components scale with increasing complexity",
        "test_file": "scalability_tests.rs"
    },
    "interaction_performance": {
        "name": "Interaction Performance",
        "description": "Test performance of user interactions",
        "test_file": "interaction_performance_tests.rs"
    }
}

def create_large_dataset_performance_tests():
    """Create performance tests for large datasets"""
    return '''#[cfg(test)]
mod large_dataset_performance_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use std::time::Instant;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    use leptos_shadcn_table::default::Table;
    use leptos_shadcn_button::default::Button;
    use leptos_shadcn_input::default::Input;
    use leptos_shadcn_card::default::{Card, CardHeader, CardTitle, CardContent};
    
    #[derive(Debug, Clone, PartialEq)]
    struct TestData {
        id: usize,
        name: String,
        email: String,
        age: u32,
        department: String,
    }
    
    impl TestData {
        fn new(id: usize) -> Self {
            Self {
                id,
                name: format!("User {}", id),
                email: format!("user{}@example.com", id),
                age: 20 + (id % 50),
                department: match id % 5 {
                    0 => "Engineering".to_string(),
                    1 => "Marketing".to_string(),
                    2 => "Sales".to_string(),
                    3 => "HR".to_string(),
                    _ => "Finance".to_string(),
                },
            }
        }
    }
    
    #[wasm_bindgen_test]
    fn test_large_table_rendering_performance() {
        let dataset_sizes = vec![100, 500, 1000, 2000];
        
        for size in dataset_sizes {
            let start_time = js_sys::Date::now();
            
            mount_to_body(move || {
                let data = (0..size)
                    .map(|i| TestData::new(i))
                    .collect::<Vec<_>>();
                
                view! {
                    <div class="large-table-test">
                        <h2>{format!("Table with {} rows", size)}</h2>
                        <Table>
                            <thead>
                                <tr>
                                    <th>"ID"</th>
                                    <th>"Name"</th>
                                    <th>"Email"</th>
                                    <th>"Age"</th>
                                    <th>"Department"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {data.into_iter().map(|item| {
                                    view! {
                                        <tr key=item.id>
                                            <td>{item.id}</td>
                                            <td>{item.name}</td>
                                            <td>{item.email}</td>
                                            <td>{item.age}</td>
                                            <td>{item.department}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </Table>
                    </div>
                }
            });
            
            let end_time = js_sys::Date::now();
            let render_time = end_time - start_time;
            
            // Verify all rows rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let rows = document.query_selector_all("tbody tr");
            assert_eq!(rows.length(), size, "All {} rows should render", size);
            
            // Performance assertion (adjust thresholds as needed)
            let max_time = match size {
                100 => 100.0,   // 100ms for 100 rows
                500 => 500.0,   // 500ms for 500 rows
                1000 => 1000.0, // 1s for 1000 rows
                2000 => 2000.0, // 2s for 2000 rows
                _ => 5000.0,    // 5s for larger datasets
            };
            
            assert!(
                render_time < max_time,
                "Render time for {} rows should be less than {}ms, got {}ms",
                size, max_time, render_time
            );
            
            println!("✅ Rendered {} rows in {:.2}ms", size, render_time);
        }
    }
    
    #[wasm_bindgen_test]
    fn test_large_card_list_performance() {
        let card_counts = vec![50, 100, 200, 500];
        
        for count in card_counts {
            let start_time = js_sys::Date::now();
            
            mount_to_body(move || {
                view! {
                    <div class="large-card-list">
                        <h2>{format!("Card List with {} items", count)}</h2>
                        <div class="card-grid">
                            {(0..count).map(|i| {
                                view! {
                                    <Card key=i class="performance-card">
                                        <CardHeader>
                                            <CardTitle>{format!("Card {}", i)}</CardTitle>
                                        </CardHeader>
                                        <CardContent>
                                            <p>{format!("This is card number {} with some content.", i)}</p>
                                            <Button>"Action {i}"</Button>
                                        </CardContent>
                                    </Card>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            });
            
            let end_time = js_sys::Date::now();
            let render_time = end_time - start_time;
            
            // Verify all cards rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let cards = document.query_selector_all(".performance-card");
            assert_eq!(cards.length(), count, "All {} cards should render", count);
            
            // Performance assertion
            let max_time = match count {
                50 => 200.0,    // 200ms for 50 cards
                100 => 400.0,   // 400ms for 100 cards
                200 => 800.0,   // 800ms for 200 cards
                500 => 2000.0,  // 2s for 500 cards
                _ => 5000.0,    // 5s for larger counts
            };
            
            assert!(
                render_time < max_time,
                "Render time for {} cards should be less than {}ms, got {}ms",
                count, max_time, render_time
            );
            
            println!("✅ Rendered {} cards in {:.2}ms", count, render_time);
        }
    }
    
    #[wasm_bindgen_test]
    fn test_large_input_form_performance() {
        let input_counts = vec![20, 50, 100, 200];
        
        for count in input_counts {
            let start_time = js_sys::Date::now();
            
            mount_to_body(move || {
                view! {
                    <div class="large-form">
                        <h2>{format!("Form with {} inputs", count)}</h2>
                        <form>
                            {(0..count).map(|i| {
                                view! {
                                    <div key=i class="form-field">
                                        <label>{format!("Field {}", i)}</label>
                                        <Input 
                                            placeholder={format!("Enter value for field {}", i)}
                                            name={format!("field_{}", i)}
                                        />
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                            <Button type="submit">"Submit Form"</Button>
                        </form>
                    </div>
                }
            });
            
            let end_time = js_sys::Date::now();
            let render_time = end_time - start_time;
            
            // Verify all inputs rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let inputs = document.query_selector_all("input");
            assert_eq!(inputs.length(), count, "All {} inputs should render", count);
            
            // Performance assertion
            let max_time = match count {
                20 => 100.0,    // 100ms for 20 inputs
                50 => 250.0,    // 250ms for 50 inputs
                100 => 500.0,   // 500ms for 100 inputs
                200 => 1000.0,  // 1s for 200 inputs
                _ => 2000.0,    // 2s for larger counts
            };
            
            assert!(
                render_time < max_time,
                "Render time for {} inputs should be less than {}ms, got {}ms",
                count, max_time, render_time
            );
            
            println!("✅ Rendered {} inputs in {:.2}ms", count, render_time);
        }
    }
    
    #[wasm_bindgen_test]
    fn test_memory_usage_with_large_datasets() {
        // Test memory usage with progressively larger datasets
        let dataset_sizes = vec![1000, 5000, 10000];
        
        for size in dataset_sizes {
            let start_memory = get_memory_usage();
            
            mount_to_body(move || {
                let data = (0..size)
                    .map(|i| TestData::new(i))
                    .collect::<Vec<_>>();
                
                view! {
                    <div class="memory-test">
                        <h2>{format!("Memory test with {} items", size)}</h2>
                        <div class="data-list">
                            {data.into_iter().map(|item| {
                                view! {
                                    <div key=item.id class="data-item">
                                        <span>{item.name}</span>
                                        <span>{item.email}</span>
                                        <span>{item.department}</span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            });
            
            let end_memory = get_memory_usage();
            let memory_used = end_memory - start_memory;
            
            // Verify all items rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let items = document.query_selector_all(".data-item");
            assert_eq!(items.length(), size, "All {} items should render", size);
            
            // Memory usage should be reasonable (less than 1MB per 1000 items)
            let max_memory_per_item = 1024.0; // 1KB per item
            let max_total_memory = (size as f64 / 1000.0) * max_memory_per_item;
            
            assert!(
                memory_used < max_total_memory,
                "Memory usage for {} items should be less than {}KB, got {}KB",
                size, max_total_memory, memory_used
            );
            
            println!("✅ Memory usage for {} items: {:.2}KB", size, memory_used);
        }
    }
    
    fn get_memory_usage() -> f64 {
        // Get memory usage from performance API
        if let Ok(performance) = web_sys::window().unwrap().performance() {
            if let Ok(memory) = performance.memory() {
                return memory.used_js_heap_size() as f64 / 1024.0; // Convert to KB
            }
        }
        0.0
    }
}
'''

def create_memory_usage_tests():
    """Create memory usage tests"""
    return '''#[cfg(test)]
mod memory_usage_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    use leptos_shadcn_button::default::Button;
    use leptos_shadcn_input::default::Input;
    use leptos_shadcn_card::default::{Card, CardHeader, CardTitle, CardContent};
    
    #[wasm_bindgen_test]
    fn test_component_memory_footprint() {
        let component_counts = vec![10, 50, 100, 500, 1000];
        
        for count in component_counts {
            let start_memory = get_memory_usage();
            
            mount_to_body(move || {
                view! {
                    <div class="memory-footprint-test">
                        <h2>{format!("Memory footprint test with {} components", count)}</h2>
                        <div class="component-grid">
                            {(0..count).map(|i| {
                                view! {
                                    <div key=i class="component-item">
                                        <Card>
                                            <CardHeader>
                                                <CardTitle>{format!("Component {}", i)}</CardTitle>
                                            </CardHeader>
                                            <CardContent>
                                                <Input placeholder={format!("Input {}", i)} />
                                                <Button>"Button {i}"</Button>
                                            </CardContent>
                                        </Card>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            });
            
            let end_memory = get_memory_usage();
            let memory_per_component = (end_memory - start_memory) / count as f64;
            
            // Verify all components rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let components = document.query_selector_all(".component-item");
            assert_eq!(components.length(), count, "All {} components should render", count);
            
            // Memory per component should be reasonable (less than 5KB per component)
            let max_memory_per_component = 5.0; // 5KB per component
            
            assert!(
                memory_per_component < max_memory_per_component,
                "Memory per component should be less than {}KB, got {}KB",
                max_memory_per_component, memory_per_component
            );
            
            println!("✅ Memory per component for {} components: {:.2}KB", count, memory_per_component);
        }
    }
    
    #[wasm_bindgen_test]
    fn test_signal_memory_usage() {
        let signal_counts = vec![100, 500, 1000, 2000];
        
        for count in signal_counts {
            let start_memory = get_memory_usage();
            
            mount_to_body(move || {
                let signals = (0..count)
                    .map(|i| RwSignal::new(format!("Signal value {}", i)))
                    .collect::<Vec<_>>();
                
                view! {
                    <div class="signal-memory-test">
                        <h2>{format!("Signal memory test with {} signals", count)}</h2>
                        <div class="signal-list">
                            {signals.into_iter().enumerate().map(|(i, signal)| {
                                view! {
                                    <div key=i class="signal-item">
                                        <span>{signal.get()}</span>
                                        <Button on_click=move || signal.update(|val| *val = format!("Updated {}", i))>
                                            "Update"
                                        </Button>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            });
            
            let end_memory = get_memory_usage();
            let memory_per_signal = (end_memory - start_memory) / count as f64;
            
            // Verify all signals rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let signal_items = document.query_selector_all(".signal-item");
            assert_eq!(signal_items.length(), count, "All {} signals should render", count);
            
            // Memory per signal should be reasonable (less than 1KB per signal)
            let max_memory_per_signal = 1.0; // 1KB per signal
            
            assert!(
                memory_per_signal < max_memory_per_signal,
                "Memory per signal should be less than {}KB, got {}KB",
                max_memory_per_signal, memory_per_signal
            );
            
            println!("✅ Memory per signal for {} signals: {:.2}KB", count, memory_per_signal);
        }
    }
    
    fn get_memory_usage() -> f64 {
        // Get memory usage from performance API
        if let Ok(performance) = web_sys::window().unwrap().performance() {
            if let Ok(memory) = performance.memory() {
                return memory.used_js_heap_size() as f64 / 1024.0; // Convert to KB
            }
        }
        0.0
    }
}
'''

def create_scalability_tests():
    """Create scalability tests"""
    return '''#[cfg(test)]
mod scalability_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    use leptos_shadcn_button::default::Button;
    use leptos_shadcn_input::default::Input;
    use leptos_shadcn_table::default::Table;
    use leptos_shadcn_card::default::{Card, CardHeader, CardTitle, CardContent};
    
    #[wasm_bindgen_test]
    fn test_component_scalability() {
        let complexity_levels = vec![1, 5, 10, 20, 50];
        
        for level in complexity_levels {
            let start_time = js_sys::Date::now();
            
            mount_to_body(move || {
                view! {
                    <div class="scalability-test">
                        <h2>{format!("Scalability test level {}", level)}</h2>
                        <div class="nested-components">
                            {(0..level).map(|i| {
                                view! {
                                    <div key=i class="nested-level">
                                        <Card>
                                            <CardHeader>
                                                <CardTitle>{format!("Level {}", i)}</CardTitle>
                                            </CardHeader>
                                            <CardContent>
                                                <Input placeholder={format!("Input at level {}", i)} />
                                                <Button>"Button at level {i}"</Button>
                                                <Table>
                                                    <thead>
                                                        <tr>
                                                            <th>"Column 1"</th>
                                                            <th>"Column 2"</th>
                                                            <th>"Column 3"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {(0..5).map(|j| {
                                                            view! {
                                                                <tr key=j>
                                                                    <td>{format!("Row {}-{}", i, j)}</td>
                                                                    <td>{format!("Data {}-{}", i, j)}</td>
                                                                    <td>{format!("Value {}-{}", i, j)}</td>
                                                                </tr>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </tbody>
                                                </Table>
                                            </CardContent>
                                        </Card>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            });
            
            let end_time = js_sys::Date::now();
            let render_time = end_time - start_time;
            
            // Verify all levels rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let levels = document.query_selector_all(".nested-level");
            assert_eq!(levels.length(), level, "All {} levels should render", level);
            
            // Render time should scale reasonably (less than 100ms per level)
            let max_time_per_level = 100.0; // 100ms per level
            let max_total_time = level as f64 * max_time_per_level;
            
            assert!(
                render_time < max_total_time,
                "Render time for level {} should be less than {}ms, got {}ms",
                level, max_total_time, render_time
            );
            
            println!("✅ Rendered complexity level {} in {:.2}ms", level, render_time);
        }
    }
    
    #[wasm_bindgen_test]
    fn test_interaction_scalability() {
        let interaction_counts = vec![10, 50, 100, 200];
        
        for count in interaction_counts {
            let start_time = js_sys::Date::now();
            
            mount_to_body(move || {
                let click_counts = (0..count)
                    .map(|_| RwSignal::new(0))
                    .collect::<Vec<_>>();
                
                view! {
                    <div class="interaction-scalability-test">
                        <h2>{format!("Interaction scalability test with {} buttons", count)}</h2>
                        <div class="button-grid">
                            {click_counts.into_iter().enumerate().map(|(i, click_count)| {
                                view! {
                                    <div key=i class="button-item">
                                        <Button 
                                            on_click=move || click_count.update(|count| *count += 1)
                                        >
                                            {format!("Button {}", i)}
                                        </Button>
                                        <span class="click-count">
                                            "Clicks: " {click_count.get()}
                                        </span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            });
            
            let end_time = js_sys::Date::now();
            let render_time = end_time - start_time;
            
            // Verify all buttons rendered
            let document = web_sys::window().unwrap().document().unwrap();
            let buttons = document.query_selector_all("button");
            assert_eq!(buttons.length(), count, "All {} buttons should render", count);
            
            // Test interaction performance
            let interaction_start = js_sys::Date::now();
            
            // Click all buttons
            for i in 0..count {
                let button = document.query_selector(&format!("button:nth-child({})", i + 1)).unwrap().unwrap();
                let click_event = web_sys::MouseEvent::new("click").unwrap();
                button.dispatch_event(&click_event).unwrap();
            }
            
            let interaction_end = js_sys::Date::now();
            let interaction_time = interaction_end - interaction_start;
            
            // Render time should be reasonable
            let max_render_time = count as f64 * 2.0; // 2ms per button
            assert!(
                render_time < max_render_time,
                "Render time for {} buttons should be less than {}ms, got {}ms",
                count, max_render_time, render_time
            );
            
            // Interaction time should be reasonable
            let max_interaction_time = count as f64 * 1.0; // 1ms per interaction
            assert!(
                interaction_time < max_interaction_time,
                "Interaction time for {} buttons should be less than {}ms, got {}ms",
                count, max_interaction_time, interaction_time
            );
            
            println!("✅ Rendered {} buttons in {:.2}ms, interactions in {:.2}ms", count, render_time, interaction_time);
        }
    }
}
'''

def create_performance_tests_directory():
    """Create the performance tests directory and files"""
    performance_dir = "tests/performance"
    os.makedirs(performance_dir, exist_ok=True)
    
    print(f"📁 Created performance tests directory: {performance_dir}")
    
    # Create large dataset performance tests
    large_dataset_file = os.path.join(performance_dir, "large_dataset_performance_tests.rs")
    with open(large_dataset_file, 'w') as f:
        f.write(create_large_dataset_performance_tests())
    print(f"✅ Created large dataset performance tests: {large_dataset_file}")
    
    # Create memory usage tests
    memory_file = os.path.join(performance_dir, "memory_usage_tests.rs")
    with open(memory_file, 'w') as f:
        f.write(create_memory_usage_tests())
    print(f"✅ Created memory usage tests: {memory_file}")
    
    # Create scalability tests
    scalability_file = os.path.join(performance_dir, "scalability_tests.rs")
    with open(scalability_file, 'w') as f:
        f.write(create_scalability_tests())
    print(f"✅ Created scalability tests: {scalability_file}")

def create_performance_test_runner():
    """Create a performance test runner script"""
    runner_content = '''#!/usr/bin/env python3
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
    
    print("\\n🚀 Running performance tests...")
    
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
        
        print("\\n📊 Test Results:")
        print(result.stdout)
        
        if result.stderr:
            print("\\n❌ Errors:")
            print(result.stderr)
        
        # Save results to JSON file
        results_file = "performance_test_results.json"
        with open(results_file, 'w') as f:
            json.dump(results, f, indent=2)
        
        print(f"\\n💾 Results saved to: {results_file}")
        
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
'''
    
    runner_path = "scripts/run_performance_tests.py"
    with open(runner_path, 'w') as f:
        f.write(runner_content)
    
    os.chmod(runner_path, 0o755)
    print(f"✅ Created performance test runner: {runner_path}")

def main():
    """Main function to create performance tests"""
    print("⚡ Creating Performance Tests for Large Datasets...")
    print("=" * 60)
    
    create_performance_tests_directory()
    create_performance_test_runner()
    
    print("\\n🎉 Performance Tests Created Successfully!")
    print("=" * 60)
    print("📁 Performance tests directory: tests/performance/")
    print("🚀 Test runner: scripts/run_performance_tests.py")
    print("\\n💡 Next steps:")
    print("   1. Run: python3 scripts/run_performance_tests.py")
    print("   2. Review performance results and adjust thresholds")
    print("   3. Add more performance scenarios as needed")
    print("   4. Monitor memory usage and rendering times")

if __name__ == "__main__":
    main()
