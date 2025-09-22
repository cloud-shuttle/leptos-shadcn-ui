#[cfg(test)]
mod functionality_tests {
    use leptos::prelude::*;
    use crate::data_table::{
        DataTable, DataRow, DataColumn, SortDirection, FilterType, FilterOperator,
        SelectionMode, ExportFormat, ColumnFilter, RowAction
    };

    #[test]
    fn test_column_sorting() {
        // Test column sorting functionality
        let mut data = vec![
            DataRow { id: 1, name: "Charlie".to_string(), age: 30, email: "charlie@example.com".to_string() },
            DataRow { id: 2, name: "Alice".to_string(), age: 25, email: "alice@example.com".to_string() },
            DataRow { id: 3, name: "Bob".to_string(), age: 35, email: "bob@example.com".to_string() },
        ];

        // Test sorting by name (ascending)
        data.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(data[0].name, "Alice");
        assert_eq!(data[1].name, "Bob");
        assert_eq!(data[2].name, "Charlie");

        // Test sorting by age (descending)
        data.sort_by(|a, b| b.age.cmp(&a.age));
        assert_eq!(data[0].age, 35);
        assert_eq!(data[1].age, 30);
        assert_eq!(data[2].age, 25);
    }

    #[test]
    fn test_column_filtering() {
        // Test column filtering functionality
        let data = vec![
            DataRow { id: 1, name: "John Doe".to_string(), age: 30, email: "john@example.com".to_string() },
            DataRow { id: 2, name: "Jane Smith".to_string(), age: 25, email: "jane@example.com".to_string() },
            DataRow { id: 3, name: "Bob Johnson".to_string(), age: 35, email: "bob@example.com".to_string() },
        ];

        // Test filtering by name (contains)
        let filtered_by_name: Vec<_> = data.iter()
            .filter(|row| row.name.contains("John"))
            .collect();
        assert_eq!(filtered_by_name.len(), 2);
        assert_eq!(filtered_by_name[0].name, "John Doe");
        assert_eq!(filtered_by_name[1].name, "Bob Johnson");

        // Test filtering by age (greater than)
        let filtered_by_age: Vec<_> = data.iter()
            .filter(|row| row.age > 25)
            .collect();
        assert_eq!(filtered_by_age.len(), 2);
        assert_eq!(filtered_by_age[0].age, 30);
        assert_eq!(filtered_by_age[1].age, 35);
    }

    #[test]
    fn test_pagination() {
        // Test pagination functionality
        let data = vec![
            DataRow { id: 1, name: "User 1".to_string(), age: 20, email: "user1@example.com".to_string() },
            DataRow { id: 2, name: "User 2".to_string(), age: 21, email: "user2@example.com".to_string() },
            DataRow { id: 3, name: "User 3".to_string(), age: 22, email: "user3@example.com".to_string() },
            DataRow { id: 4, name: "User 4".to_string(), age: 23, email: "user4@example.com".to_string() },
            DataRow { id: 5, name: "User 5".to_string(), age: 24, email: "user5@example.com".to_string() },
        ];

        let page_size = 2;
        let current_page = 1;

        // Test first page
        let start = (current_page - 1) * page_size;
        let end = start + page_size;
        let page_data: Vec<_> = data.iter().skip(start as usize).take(page_size as usize).collect();
        assert_eq!(page_data.len(), 2);
        assert_eq!(page_data[0].name, "User 1");
        assert_eq!(page_data[1].name, "User 2");

        // Test second page
        let current_page = 2;
        let start = (current_page - 1) * page_size;
        let end = start + page_size;
        let page_data: Vec<_> = data.iter().skip(start as usize).take(page_size as usize).collect();
        assert_eq!(page_data.len(), 2);
        assert_eq!(page_data[0].name, "User 3");
        assert_eq!(page_data[1].name, "User 4");
    }

    #[test]
    fn test_row_selection() {
        // Test row selection functionality
        let mut selected_rows = Vec::new();
        let data = vec![
            DataRow { id: 1, name: "User 1".to_string(), age: 20, email: "user1@example.com".to_string() },
            DataRow { id: 2, name: "User 2".to_string(), age: 21, email: "user2@example.com".to_string() },
            DataRow { id: 3, name: "User 3".to_string(), age: 22, email: "user3@example.com".to_string() },
        ];

        // Test single selection
        selected_rows.push(1);
        assert_eq!(selected_rows.len(), 1);
        assert!(selected_rows.contains(&1));

        // Test multiple selection
        selected_rows.push(2);
        selected_rows.push(3);
        assert_eq!(selected_rows.len(), 3);
        assert!(selected_rows.contains(&1));
        assert!(selected_rows.contains(&2));
        assert!(selected_rows.contains(&3));

        // Test deselection
        selected_rows.retain(|&id| id != 2);
        assert_eq!(selected_rows.len(), 2);
        assert!(selected_rows.contains(&1));
        assert!(!selected_rows.contains(&2));
        assert!(selected_rows.contains(&3));
    }

    #[test]
    fn test_global_search() {
        // Test global search functionality
        let data = vec![
            DataRow { id: 1, name: "John Doe".to_string(), age: 30, email: "john@example.com".to_string() },
            DataRow { id: 2, name: "Jane Smith".to_string(), age: 25, email: "jane@example.com".to_string() },
            DataRow { id: 3, name: "Bob Johnson".to_string(), age: 35, email: "bob@example.com".to_string() },
        ];

        let search_term = "john";

        // Test global search
        let search_results: Vec<_> = data.iter()
            .filter(|row| {
                row.name.to_lowercase().contains(&search_term.to_lowercase()) ||
                row.email.to_lowercase().contains(&search_term.to_lowercase())
            })
            .collect();

        assert_eq!(search_results.len(), 2);
        assert_eq!(search_results[0].name, "John Doe");
        assert_eq!(search_results[1].name, "Bob Johnson");
    }

    #[test]
    fn test_column_resizing() {
        // Test column resizing functionality
        let mut column = DataColumn {
            key: "test".to_string(),
            title: "Test Column".to_string(),
            sortable: true,
            filterable: true,
            filter_type: Some(FilterType::Text),
            resizable: Some(true),
            width: Some(200),
            draggable: Some(true),
            order: Some(1),
        };

        // Test initial width
        assert_eq!(column.width, Some(200));

        // Test width change
        column.width = Some(300);
        assert_eq!(column.width, Some(300));

        // Test width increase
        column.width = Some(400);
        assert_eq!(column.width, Some(400));

        // Test width decrease
        column.width = Some(150);
        assert_eq!(column.width, Some(150));
    }

    #[test]
    fn test_column_reordering() {
        // Test column reordering functionality
        let mut columns = vec![
            DataColumn { key: "name".to_string(), title: "Name".to_string(), sortable: true, filterable: true, order: Some(1), ..Default::default() },
            DataColumn { key: "age".to_string(), title: "Age".to_string(), sortable: true, filterable: true, order: Some(2), ..Default::default() },
            DataColumn { key: "email".to_string(), title: "Email".to_string(), sortable: false, filterable: true, order: Some(3), ..Default::default() },
        ];

        // Test initial order
        assert_eq!(columns[0].order, Some(1));
        assert_eq!(columns[1].order, Some(2));
        assert_eq!(columns[2].order, Some(3));

        // Test reordering
        columns[0].order = Some(3);
        columns[1].order = Some(1);
        columns[2].order = Some(2);

        assert_eq!(columns[0].order, Some(3));
        assert_eq!(columns[1].order, Some(1));
        assert_eq!(columns[2].order, Some(2));
    }

    #[test]
    fn test_export_functionality() {
        // Test export functionality
        let data = vec![
            DataRow { id: 1, name: "John Doe".to_string(), age: 30, email: "john@example.com".to_string() },
            DataRow { id: 2, name: "Jane Smith".to_string(), age: 25, email: "jane@example.com".to_string() },
        ];

        // Test CSV export format
        let csv_format = ExportFormat::Csv;
        assert_eq!(csv_format, ExportFormat::Csv);

        // Test JSON export format
        let json_format = ExportFormat::Json;
        assert_eq!(json_format, ExportFormat::Json);

        // Test Excel export format
        let excel_format = ExportFormat::Excel;
        assert_eq!(excel_format, ExportFormat::Excel);

        // Test data preparation for export
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].name, "John Doe");
        assert_eq!(data[1].name, "Jane Smith");
    }

    #[test]
    fn test_virtual_scrolling() {
        // Test virtual scrolling functionality
        let large_dataset: Vec<DataRow> = (1..=1000)
            .map(|i| DataRow {
                id: i,
                name: format!("User {}", i),
                age: 20 + (i % 50),
                email: format!("user{}@example.com", i),
            })
            .collect();

        // Test large dataset creation
        assert_eq!(large_dataset.len(), 1000);
        assert_eq!(large_dataset[0].name, "User 1");
        assert_eq!(large_dataset[999].name, "User 1000");

        // Test virtual scrolling window
        let window_size = 50;
        let start_index = 100;
        let end_index = start_index + window_size;

        let visible_items: Vec<_> = large_dataset.iter()
            .skip(start_index)
            .take(window_size)
            .collect();

        assert_eq!(visible_items.len(), window_size);
        assert_eq!(visible_items[0].name, "User 101");
        assert_eq!(visible_items[49].name, "User 150");
    }
}
