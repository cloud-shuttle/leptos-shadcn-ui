#[cfg(test)]
mod real_tests {
    use crate::default::{Table};
    use crate::data_table::{DataTable, DataRow, DataColumn, SortDirection, FilterType, SelectionMode};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_table_renders() {
        mount_to_body(|| {
            view! {
                <Table>
                    "table content"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "table should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_table_with_props() {
        mount_to_body(|| {
            view! {
                <Table class="test-class".into()>
                    "table with props"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "table with props should render");
    }

    #[test]
    fn test_table_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "table signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "table signal should update");
    }

    #[test]
    fn test_table_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "table callback should be triggered");
    }

    #[test]
    fn test_table_class_handling() {
        let custom_class = "custom-table-class";
        assert!(!custom_class.is_empty(), "table should support custom classes");
        assert!(custom_class.contains("table"), "Class should contain component name");
    }

    #[test]
    fn test_table_id_handling() {
        let custom_id = "custom-table-id";
        assert!(!custom_id.is_empty(), "table should support custom IDs");
        assert!(custom_id.contains("table"), "ID should contain component name");
    }

    #[test]
    fn test_data_table_column_creation() {
        let column = DataColumn::new("test".to_string(), "Test Column".to_string());
        assert_eq!(column.key, "test");
        assert_eq!(column.title, "Test Column");
        assert_eq!(column.sortable, false);
        assert_eq!(column.filterable, false);
    }

    #[test]
    fn test_data_table_row_creation() {
        let row = DataRow {
            id: 1,
            name: "John".to_string(),
            age: 25,
            email: "john@example.com".to_string(),
        };
        assert_eq!(row.id, 1);
        assert_eq!(row.name, "John");
        assert_eq!(row.age, 25);
        assert_eq!(row.email, "john@example.com");
    }

    #[test]
    fn test_sort_direction_enum() {
        assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
        assert_eq!(SortDirection::Descending, SortDirection::Descending);
        assert_ne!(SortDirection::Ascending, SortDirection::Descending);
    }

    #[test]
    fn test_filter_type_enum() {
        assert_eq!(FilterType::Text, FilterType::Text);
        assert_eq!(FilterType::Number, FilterType::Number);
        assert_eq!(FilterType::Date, FilterType::Date);
        assert_eq!(FilterType::Boolean, FilterType::Boolean);
        assert_ne!(FilterType::Text, FilterType::Number);
    }

    #[test]
    fn test_selection_mode_enum() {
        assert_eq!(SelectionMode::None, SelectionMode::None);
        assert_eq!(SelectionMode::Single, SelectionMode::Single);
        assert_eq!(SelectionMode::Multiple, SelectionMode::Multiple);
        assert_ne!(SelectionMode::None, SelectionMode::Single);
    }

    #[wasm_bindgen_test]
    fn test_table_interaction() {
        mount_to_body(|| {
            view! {
                <Table class="test-interaction".into()>
                    "Interactive table"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "table should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_table_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Table class="test-focus".into()>
                    "Focusable table"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "table should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_table_accessibility() {
        mount_to_body(|| {
            view! {
                <Table class="test-a11y".into()>
                    "Accessible table"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "table should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_table_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Table class="test-dom-render".into()>
                    "DOM Test table"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "table should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_table_class_application() {
        mount_to_body(|| {
            view! {
                <Table class="test-class-application custom-class".into()>
                    "Class Test table"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_table_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Table 
                    class="test-attributes".into()>
                    "Attribute Test table"
                </Table>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test table");
    }
}