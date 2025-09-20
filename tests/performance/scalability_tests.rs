#[cfg(test)]
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
