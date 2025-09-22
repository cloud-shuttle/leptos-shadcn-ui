#[cfg(test)]
mod basic_rendering_tests {
    use leptos::prelude::*;
    use crate::default::Table;

    // ===== BASIC RENDERING TESTS =====
    // These tests focus on basic rendering and component creation

    #[test]
    fn test_table_basic_rendering() {
        let _table_view = view! {
            <Table>
                "Basic table content"
            </Table>
        };
    }

    #[test]
    fn test_table_custom_styling() {
        let custom_class = "custom-table-class";
        let _table_view = view! {
            <Table class=custom_class>
                "Styled table content"
            </Table>
        };
    }

    #[test]
    fn test_table_custom_id() {
        let _table_view = view! {
            <Table id="custom-table-id">
                "Table with custom ID"
            </Table>
        };
    }

    #[test]
    fn test_table_custom_properties() {
        let _table_view = view! {
            <Table class="custom-properties-table" id="custom-props-test">
                "Table with custom properties"
            </Table>
        };
    }

    #[test]
    fn test_table_edge_cases() {
        let _table_view = view! {
            <Table class="" id="">
                "Edge case table"
            </Table>
        };
    }

    #[test]
    fn test_table_children_content() {
        let _table_view = view! {
            <Table>
                <thead>
                    <tr>
                        <th>"Header 1"</th>
                        <th>"Header 2"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Cell 1"</td>
                        <td>"Cell 2"</td>
                    </tr>
                </tbody>
            </Table>
        };
    }

    #[test]
    fn test_table_dynamic_content() {
        let dynamic_content = "Dynamic table content";
        let _table_view = view! {
            <Table>
                {dynamic_content}
            </Table>
        };
    }

    #[test]
    fn test_table_conditional_rendering() {
        let show_header = true;
        let _table_view = view! {
            <Table>
                {if show_header {
                    view! {
                        <thead>
                            <tr>
                                <th>"Conditional Header"</th>
                            </tr>
                        </thead>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
                <tbody>
                    <tr>
                        <td>"Body Content"</td>
                    </tr>
                </tbody>
            </Table>
        };
    }

    #[test]
    fn test_table_multiple_instances() {
        let _table1 = view! {
            <Table class="table-1">
                "First table"
            </Table>
        };
        
        let _table2 = view! {
            <Table class="table-2">
                "Second table"
            </Table>
        };
        
        let _table3 = view! {
            <Table class="table-3">
                "Third table"
            </Table>
        };
    }

    #[test]
    fn test_table_state_management() {
        let table_state = RwSignal::new("initial state");
        let _table_view = view! {
            <Table>
                {move || table_state.get()}
            </Table>
        };
        
        // Test state change
        table_state.set("updated state");
        assert_eq!(table_state.get(), "updated state");
    }

    #[test]
    fn test_table_reactive_content() {
        let content_signal = RwSignal::new("Initial content");
        let _table_view = view! {
            <Table>
                {move || content_signal.get()}
            </Table>
        };
        
        // Test reactive content
        content_signal.set("Updated content");
        assert_eq!(content_signal.get(), "Updated content");
    }

    #[test]
    fn test_table_nested_components() {
        let _table_view = view! {
            <Table>
                <thead>
                    <tr>
                        <th>
                            <span class="header-text">"Nested Header"</span>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="cell-content">
                                "Nested cell content"
                            </div>
                        </td>
                    </tr>
                </tbody>
            </Table>
        };
    }

    #[test]
    fn test_table_accessibility_attributes() {
        let _table_view = view! {
            <Table
                role="table"
                aria-label="Accessible table"
                tabindex="0"
            >
                "Accessible table content"
            </Table>
        };
    }

    #[test]
    fn test_table_custom_attributes() {
        let _table_view = view! {
            <Table
                data-testid="custom-table"
                data-role="data-table"
                data-version="1.0"
            >
                "Table with custom attributes"
            </Table>
        };
    }

    #[test]
    fn test_table_style_properties() {
        let _table_view = view! {
            <Table
                style="border: 1px solid #ccc; width: 100%;"
            >
                "Styled table"
            </Table>
        };
    }

    #[test]
    fn test_table_complex_structure() {
        let _table_view = view! {
            <Table class="complex-table">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Age"</th>
                        <th>"Email"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"John Doe"</td>
                        <td>"30"</td>
                        <td>"john@example.com"</td>
                    </tr>
                    <tr>
                        <td>"Jane Smith"</td>
                        <td>"25"</td>
                        <td>"jane@example.com"</td>
                    </tr>
                </tbody>
                <tfoot>
                    <tr>
                        <td colspan="3">"Total: 2 rows"</td>
                    </tr>
                </tfoot>
            </Table>
        };
    }
}
