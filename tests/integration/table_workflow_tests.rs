#[cfg(test)]
mod table_workflow_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    use leptos_shadcn_table::default::Table;
    use leptos_shadcn_button::default::{Button, ButtonVariant};
    use leptos_shadcn_input::default::Input;
    use leptos_shadcn_card::default::{Card, CardHeader, CardTitle, CardContent};

    #[derive(Debug, Clone, PartialEq)]
    struct TestData {
        id: usize,
        name: String,
        email: String,
        department: String,
    }

    impl TestData {
        fn new(id: usize) -> Self {
            Self {
                id,
                name: format!("User {}", id),
                email: format!("user{}@example.com", id),
                department: match id % 3 {
                    0 => "Engineering".to_string(),
                    1 => "Marketing".to_string(),
                    _ => "Sales".to_string(),
                },
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_table_workflow_integration() {
        let selected_items = RwSignal::new(Vec::<usize>::new());
        let filter_text = RwSignal::new(String::new());
        
        mount_to_body(move || {
            let data = (0..10).map(|i| TestData::new(i)).collect::<Vec<_>>();
            let filtered_data = data.into_iter()
                .filter(|item| item.name.contains(&filter_text.get()))
                .collect::<Vec<_>>();
            
            view! {
                <div class="table-workflow-test">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Table Workflow Test"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <Input 
                                value=filter_text.get()
                                on_change=move |value| filter_text.set(value)
                                placeholder="Filter by name"
                            />
                            
                            <Table>
                                <thead>
                                    <tr>
                                        <th>"ID"</th>
                                        <th>"Name"</th>
                                        <th>"Email"</th>
                                        <th>"Department"</th>
                                        <th>"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {filtered_data.into_iter().map(|item| {
                                        let is_selected = selected_items.get().contains(&item.id);
                                        view! {
                                            <tr key=item.id class=if is_selected { "selected" } else { "" }>
                                                <td>{item.id}</td>
                                                <td>{item.name}</td>
                                                <td>{item.email}</td>
                                                <td>{item.department}</td>
                                                <td>
                                                    <Button 
                                                        variant=if is_selected { ButtonVariant::Secondary } else { ButtonVariant::Default }
                                                        on_click=move || {
                                                            let mut items = selected_items.get();
                                                            if items.contains(&item.id) {
                                                                items.retain(|&x| x != item.id);
                                                            } else {
                                                                items.push(item.id);
                                                            }
                                                            selected_items.set(items);
                                                        }
                                                    >
                                                        {if is_selected { "Deselect" } else { "Select" }}
                                                    </Button>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </Table>
                            
                            <div class="selection-status">
                                "Selected items: " {selected_items.get().len()}
                            </div>
                        </CardContent>
                    </Card>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test filtering
        let input = document.query_selector("input").unwrap().unwrap();
        let html_input = input.unchecked_into::<web_sys::HtmlInputElement>();
        html_input.set_value("User 1");
        
        // Test selection
        let buttons = document.query_selector_all("button");
        if buttons.length() > 0 {
            let first_button = buttons.get(0).unwrap();
            let click_event = web_sys::MouseEvent::new("click").unwrap();
            first_button.dispatch_event(&click_event).unwrap();
        }
        
        // Verify table functionality
        let table = document.query_selector("table").unwrap();
        assert!(table.is_some(), "Table should render");
        
        let rows = document.query_selector_all("tbody tr");
        assert!(rows.length() > 0, "Table should have rows");
    }

    #[wasm_bindgen_test]
    fn test_table_workflow_performance() {
        let start_time = js_sys::Date::now();
        
        mount_to_body(|| {
            let data = (0..100).map(|i| TestData::new(i)).collect::<Vec<_>>();
            
            view! {
                <div class="table-performance-test">
                    <Table>
                        <thead>
                            <tr>
                                <th>"ID"</th>
                                <th>"Name"</th>
                                <th>"Email"</th>
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
        assert_eq!(rows.length(), 100, "All 100 rows should render");
        
        // Performance should be reasonable (less than 500ms for 100 rows)
        assert!(render_time < 500.0, "Render time should be less than 500ms, got {}ms", render_time);
        
        println!("✅ Rendered 100 table rows in {:.2}ms", render_time);
    }
}
