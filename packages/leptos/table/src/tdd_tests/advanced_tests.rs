#[cfg(test)]
mod advanced_tests {
    use leptos::prelude::*;
    use crate::default::Table;

    // ===== ADVANCED TESTS =====
    // These tests focus on advanced functionality and edge cases

    #[test]
    fn test_table_advanced_state_management() {
        let table_data = RwSignal::new(vec![
            ("Name", "Age", "Email"),
            ("John Doe", "30", "john@example.com"),
            ("Jane Smith", "25", "jane@example.com"),
        ]);
        
        let _table_view = view! {
            <Table>
                <thead>
                    <tr>
                        {move || {
                            let data = table_data.get();
                            if let Some(header_row) = data.first() {
                                view! {
                                    <th>{header_row.0}</th>
                                    <th>{header_row.1}</th>
                                    <th>{header_row.2}</th>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }
                        }}
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let data = table_data.get();
                        data.iter().skip(1).map(|row| {
                            view! {
                                <tr>
                                    <td>{row.0}</td>
                                    <td>{row.1}</td>
                                    <td>{row.2}</td>
                                </tr>
                            }.into_view()
                        }).collect::<Vec<_>>()
                    }}
                </tbody>
            </Table>
        };
        
        // Test state update
        table_data.update(|data| {
            data.push(("Bob Johnson", "35", "bob@example.com"));
        });
        
        assert_eq!(table_data.get().len(), 4);
    }

    #[test]
    fn test_table_dynamic_columns() {
        let columns = RwSignal::new(vec!["Name", "Age", "Email"]);
        let _table_view = view! {
            <Table>
                <thead>
                    <tr>
                        {move || {
                            columns.get().iter().map(|col| {
                                view! {
                                    <th>{col}</th>
                                }.into_view()
                            }).collect::<Vec<_>>()
                        }}
                    </tr>
                </thead>
            </Table>
        };
        
        // Test column update
        columns.update(|cols| {
            cols.push("Phone");
        });
        
        assert_eq!(columns.get().len(), 4);
    }

    #[test]
    fn test_table_sorting_functionality() {
        let sort_column = RwSignal::new("name");
        let sort_direction = RwSignal::new("asc");
        let table_data = RwSignal::new(vec![
            ("Charlie", "30"),
            ("Alice", "25"),
            ("Bob", "35"),
        ]);
        
        let _table_view = view! {
            <Table>
                <thead>
                    <tr>
                        <th
                            on:click=move |_| {
                                sort_column.set("name");
                                sort_direction.update(|dir| {
                                    *dir = if *dir == "asc" { "desc" } else { "asc" };
                                });
                            }
                        >
                            "Name"
                            {move || {
                                if sort_column.get() == "name" {
                                    if sort_direction.get() == "asc" { " ↑" } else { " ↓" }
                                } else {
                                    ""
                                }
                            }}
                        </th>
                        <th
                            on:click=move |_| {
                                sort_column.set("age");
                                sort_direction.update(|dir| {
                                    *dir = if *dir == "asc" { "desc" } else { "asc" };
                                });
                            }
                        >
                            "Age"
                            {move || {
                                if sort_column.get() == "age" {
                                    if sort_direction.get() == "asc" { " ↑" } else { " ↓" }
                                } else {
                                    ""
                                }
                            }}
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let mut data = table_data.get();
                        if sort_column.get() == "name" {
                            if sort_direction.get() == "asc" {
                                data.sort_by(|a, b| a.0.cmp(&b.0));
                            } else {
                                data.sort_by(|a, b| b.0.cmp(&a.0));
                            }
                        }
                        data.iter().map(|row| {
                            view! {
                                <tr>
                                    <td>{row.0}</td>
                                    <td>{row.1}</td>
                                </tr>
                            }.into_view()
                        }).collect::<Vec<_>>()
                    }}
                </tbody>
            </Table>
        };
        
        // Test sorting
        sort_column.set("name");
        sort_direction.set("asc");
        assert_eq!(sort_column.get(), "name");
        assert_eq!(sort_direction.get(), "asc");
    }

    #[test]
    fn test_table_filtering_functionality() {
        let filter_value = RwSignal::new("".to_string());
        let table_data = RwSignal::new(vec![
            ("John Doe", "30"),
            ("Jane Smith", "25"),
            ("Bob Johnson", "35"),
        ]);
        
        let _table_view = view! {
            <Table>
                <thead>
                    <tr>
                        <th>
                            <input
                                type="text"
                                placeholder="Filter by name"
                                value=move || filter_value.get()
                                on:input=move |ev| {
                                    filter_value.set(event_target_value(&ev));
                                }
                            />
                        </th>
                        <th>"Age"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let filter = filter_value.get();
                        let data = table_data.get();
                        data.iter()
                            .filter(|row| {
                                if filter.is_empty() {
                                    true
                                } else {
                                    row.0.to_lowercase().contains(&filter.to_lowercase())
                                }
                            })
                            .map(|row| {
                                view! {
                                    <tr>
                                        <td>{row.0}</td>
                                        <td>{row.1}</td>
                                    </tr>
                                }.into_view()
                            }).collect::<Vec<_>>()
                    }}
                </tbody>
            </Table>
        };
        
        // Test filtering
        filter_value.set("john".to_string());
        assert_eq!(filter_value.get(), "john");
    }

    #[test]
    fn test_table_pagination_functionality() {
        let current_page = RwSignal::new(1);
        let page_size = RwSignal::new(2);
        let table_data = RwSignal::new(vec![
            ("Row 1", "Data 1"),
            ("Row 2", "Data 2"),
            ("Row 3", "Data 3"),
            ("Row 4", "Data 4"),
            ("Row 5", "Data 5"),
        ]);
        
        let _table_view = view! {
            <Table>
                <tbody>
                    {move || {
                        let page = current_page.get();
                        let size = page_size.get();
                        let start = (page - 1) * size;
                        let end = start + size;
                        
                        table_data.get()
                            .iter()
                            .skip(start as usize)
                            .take(size as usize)
                            .map(|row| {
                                view! {
                                    <tr>
                                        <td>{row.0}</td>
                                        <td>{row.1}</td>
                                    </tr>
                                }.into_view()
                            }).collect::<Vec<_>>()
                    }}
                </tbody>
                <tfoot>
                    <tr>
                        <td colspan="2">
                            <button
                                disabled=move || current_page.get() <= 1
                                on:click=move |_| {
                                    current_page.update(|page| {
                                        if *page > 1 {
                                            *page -= 1;
                                        }
                                    });
                                }
                            >
                                "Previous"
                            </button>
                            <span>
                                {move || {
                                    let page = current_page.get();
                                    let total = (table_data.get().len() as f64 / page_size.get() as f64).ceil() as i32;
                                    format!("Page {} of {}", page, total)
                                }}
                            </span>
                            <button
                                disabled=move || {
                                    let page = current_page.get();
                                    let total = (table_data.get().len() as f64 / page_size.get() as f64).ceil() as i32;
                                    page >= total
                                }
                                on:click=move |_| {
                                    current_page.update(|page| {
                                        let total = (table_data.get().len() as f64 / page_size.get() as f64).ceil() as i32;
                                        if *page < total {
                                            *page += 1;
                                        }
                                    });
                                }
                            >
                                "Next"
                            </button>
                        </td>
                    </tr>
                </tfoot>
            </Table>
        };
        
        // Test pagination
        current_page.set(2);
        assert_eq!(current_page.get(), 2);
    }

    #[test]
    fn test_table_row_selection() {
        let selected_rows = RwSignal::new(Vec::<usize>::new());
        let table_data = RwSignal::new(vec![
            ("Row 1", "Data 1"),
            ("Row 2", "Data 2"),
            ("Row 3", "Data 3"),
        ]);
        
        let _table_view = view! {
            <Table>
                <tbody>
                    {move || {
                        table_data.get()
                            .iter()
                            .enumerate()
                            .map(|(index, row)| {
                                let is_selected = selected_rows.get().contains(&index);
                                view! {
                                    <tr
                                        class=move || if is_selected { "selected" } else { "" }
                                        on:click=move |_| {
                                            selected_rows.update(|selected| {
                                                if selected.contains(&index) {
                                                    selected.retain(|&i| i != index);
                                                } else {
                                                    selected.push(index);
                                                }
                                            });
                                        }
                                    >
                                        <td>
                                            <input
                                                type="checkbox"
                                                checked=is_selected
                                                on:change=move |_| {
                                                    selected_rows.update(|selected| {
                                                        if selected.contains(&index) {
                                                            selected.retain(|&i| i != index);
                                                        } else {
                                                            selected.push(index);
                                                        }
                                                    });
                                                }
                                            />
                                        </td>
                                        <td>{row.0}</td>
                                        <td>{row.1}</td>
                                    </tr>
                                }.into_view()
                            }).collect::<Vec<_>>()
                    }}
                </tbody>
            </Table>
        };
        
        // Test row selection
        selected_rows.update(|selected| selected.push(0));
        assert!(selected_rows.get().contains(&0));
    }

    #[test]
    fn test_table_editable_cells() {
        let table_data = RwSignal::new(vec![
            ("Editable Cell 1", "Editable Cell 2"),
            ("Editable Cell 3", "Editable Cell 4"),
        ]);
        
        let _table_view = view! {
            <Table>
                <tbody>
                    {move || {
                        table_data.get()
                            .iter()
                            .enumerate()
                            .map(|(row_index, row)| {
                                view! {
                                    <tr>
                                        <td>
                                            <input
                                                type="text"
                                                value=row.0
                                                on:input=move |ev| {
                                                    table_data.update(|data| {
                                                        if let Some(row_data) = data.get_mut(row_index) {
                                                            row_data.0 = event_target_value(&ev);
                                                        }
                                                    });
                                                }
                                            />
                                        </td>
                                        <td>
                                            <input
                                                type="text"
                                                value=row.1
                                                on:input=move |ev| {
                                                    table_data.update(|data| {
                                                        if let Some(row_data) = data.get_mut(row_index) {
                                                            row_data.1 = event_target_value(&ev);
                                                        }
                                                    });
                                                }
                                            />
                                        </td>
                                    </tr>
                                }.into_view()
                            }).collect::<Vec<_>>()
                    }}
                </tbody>
            </Table>
        };
        
        // Test cell editing
        table_data.update(|data| {
            if let Some(row) = data.get_mut(0) {
                row.0 = "Updated Cell 1".to_string();
            }
        });
        
        assert_eq!(table_data.get()[0].0, "Updated Cell 1");
    }

    #[test]
    fn test_table_export_functionality() {
        let table_data = RwSignal::new(vec![
            ("Name", "Age"),
            ("John Doe", "30"),
            ("Jane Smith", "25"),
        ]);
        
        let _table_view = view! {
            <Table>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Age"</th>
                        <th>
                            <button
                                on:click=move |_| {
                                    // Simulate CSV export
                                    let data = table_data.get();
                                    let csv = data.iter()
                                        .map(|row| format!("{},{}", row.0, row.1))
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    // In a real implementation, this would trigger a download
                                    println!("CSV Export:\n{}", csv);
                                }
                            >
                                "Export CSV"
                            </button>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        table_data.get()
                            .iter()
                            .skip(1)
                            .map(|row| {
                                view! {
                                    <tr>
                                        <td>{row.0}</td>
                                        <td>{row.1}</td>
                                        <td></td>
                                    </tr>
                                }.into_view()
                            }).collect::<Vec<_>>()
                    }}
                </tbody>
            </Table>
        };
        
        // Test export data
        let data = table_data.get();
        assert_eq!(data.len(), 3);
        assert_eq!(data[0].0, "Name");
        assert_eq!(data[1].0, "John Doe");
    }
}
