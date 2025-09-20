#[cfg(test)]
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
