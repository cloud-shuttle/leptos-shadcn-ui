#[cfg(test)]
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
