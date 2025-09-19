#[cfg(test)]
mod performance_tests {
    use leptos::prelude::*;
    use leptos_style::Style;
    use std::collections::HashMap;

    // Mock types for testing
    #[derive(Debug, Clone)]
    struct FormData {
        pub fields: HashMap<String, String>,
    }

    impl FormData {
        fn new() -> Self {
            Self {
                fields: HashMap::new(),
            }
        }
    }

    #[test]
    fn test_form_performance_characteristics() {
        // Test form performance characteristics
        
        // Test rapid signal updates
        let form_data_signal = RwSignal::new(FormData::new());
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let mut form_data = FormData::new();
            form_data.fields.insert("field".to_string(), format!("value_{}", i));
            form_data_signal.set(form_data);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
        
        // Test form data operations performance
        let start_operations = std::time::Instant::now();
        
        for i in 0..100 {
            let mut form_data = FormData::new();
            form_data.fields.insert("email".to_string(), format!("user{}@example.com", i));
            form_data.fields.insert("password".to_string(), format!("password{}", i));
            form_data.fields.insert("username".to_string(), format!("user{}", i));
        }
        
        let operations_duration = start_operations.elapsed();
        assert!(operations_duration.as_millis() < 50); // Should be very fast
    }

    #[test]
    fn test_form_memory_management() {
        // Test memory management
        
        // Test form data cleanup
        let form_data_signal = RwSignal::new(FormData::new());
        let initial_data = form_data_signal.get();
        assert!(initial_data.fields.is_empty());
        
        // Test large form data handling
        let mut large_form_data = FormData::new();
        for i in 0..1000 {
            large_form_data.fields.insert(format!("field_{}", i), format!("value_{}", i));
        }
        form_data_signal.set(large_form_data);
        assert_eq!(form_data_signal.get().fields.len(), 1000);
        
        // Test memory cleanup by setting smaller form data
        let small_form_data = FormData::new();
        form_data_signal.set(small_form_data);
        assert_eq!(form_data_signal.get().fields.len(), 0);
        
        // Test HashMap memory management
        let mut test_map = HashMap::new();
        for i in 0..100 {
            test_map.insert(format!("key_{}", i), format!("value_{}", i));
        }
        assert_eq!(test_map.len(), 100);
        
        // Test HashMap cleanup
        test_map.clear();
        assert_eq!(test_map.len(), 0);
    }

    #[test]
    fn test_form_style_performance() {
        // Test style performance
        let style_signal = RwSignal::new(Style::new());
        
        // Test rapid style updates
        let start = std::time::Instant::now();
        
        for _i in 0..100 {
            let style = Style::new();
            style_signal.set(style);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be very fast
        
        // Test final state
        let final_style = style_signal.get().to_string();
        assert_eq!(final_style, "");
    }

    #[test]
    fn test_form_callback_performance() {
        // Test callback performance
        let callback_count = RwSignal::new(0);
        let callback = Callback::new(move |_form_data: FormData| {
            callback_count.update(|count| *count += 1);
        });
        
        let start = std::time::Instant::now();
        
        // Test rapid callback execution
        for _i in 0..1000 {
            callback.run(FormData::new());
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
        
        // Test callback count
        assert_eq!(callback_count.get(), 1000);
    }

    #[test]
    fn test_form_validation_performance() {
        // Test validation performance
        let mut form_data = FormData::new();
        
        // Test validation with many fields
        for i in 0..100 {
            form_data.fields.insert(format!("field_{}", i), format!("value_{}", i));
        }
        
        let start = std::time::Instant::now();
        
        // Test validation logic
        for (field, value) in &form_data.fields {
            // Simulate validation
            let is_valid = !field.is_empty() && !value.is_empty();
            assert!(is_valid);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be very fast
    }

    #[test]
    fn test_form_large_dataset_performance() {
        // Test performance with large datasets
        let mut form_data = FormData::new();
        
        // Create large form data
        for i in 0..1000 {
            form_data.fields.insert(format!("field_{}", i), format!("value_{}", i));
        }
        
        let start = std::time::Instant::now();
        
        // Test operations on large dataset
        for (field, value) in &form_data.fields {
            // Simulate field processing
            let processed_field = field.to_uppercase();
            let processed_value = value.to_uppercase();
            assert!(!processed_field.is_empty());
            assert!(!processed_value.is_empty());
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200); // Should be reasonable for large dataset
    }

    #[test]
    fn test_form_memory_cleanup_performance() {
        // Test memory cleanup performance
        let start = std::time::Instant::now();
        
        // Test memory cleanup
        for _i in 0..100 {
            let mut form_data = FormData::new();
            
            // Add many fields
            for j in 0..100 {
                form_data.fields.insert(format!("field_{}", j), format!("value_{}", j));
            }
            
            // Clear fields
            form_data.fields.clear();
            
            // Drop form_data
            drop(form_data);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be fast
    }

    #[test]
    fn test_form_concurrent_performance() {
        // Test concurrent-like performance
        let form_data_signal = RwSignal::new(FormData::new());
        let callback_count = RwSignal::new(0);
        
        let callback = Callback::new(move |_form_data: FormData| {
            callback_count.update(|count| *count += 1);
        });
        
        let start = std::time::Instant::now();
        
        // Test concurrent-like operations
        for _i in 0..100 {
            // Update form data
            let mut form_data = FormData::new();
            form_data.fields.insert("field".to_string(), "value".to_string());
            form_data_signal.set(form_data);
            
            // Execute callback
            callback.run(FormData::new());
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be fast
    }

    #[test]
    fn test_form_string_handling_performance() {
        // Test string handling performance
        let start = std::time::Instant::now();
        
        // Test string operations
        for i in 0..1000 {
            let field_name = format!("field_{}", i);
            let field_value = format!("value_{}", i);
            
            // Test string operations
            let combined = format!("{}:{}", field_name, field_value);
            assert!(combined.contains(&field_name));
            assert!(combined.contains(&field_value));
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be very fast
    }

    #[test]
    fn test_form_hashmap_performance() {
        // Test HashMap performance
        let start = std::time::Instant::now();
        
        // Test HashMap operations
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(format!("key_{}", i), format!("value_{}", i));
        }
        
        // Test HashMap lookups
        for i in 0..1000 {
            let key = format!("key_{}", i);
            let value = map.get(&key);
            assert!(value.is_some());
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be very fast
    }
}
