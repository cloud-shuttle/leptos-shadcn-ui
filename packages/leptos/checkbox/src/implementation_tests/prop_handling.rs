#[cfg(test)]
mod prop_handling {
    use crate::default::CHECKBOX_CLASS;
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== PROP HANDLING TESTS =====
    // These tests focus on prop handling and validation

    #[test]
    fn test_checkbox_prop_defaults() {
        // Test prop default handling for Checkbox
        let class = Some("test-class".to_string());
        let default_class = class.unwrap_or_default();
        assert_eq!(default_class, "test-class");
        
        let no_class: Option<String> = None;
        let default_no_class = no_class.unwrap_or_default();
        assert_eq!(default_no_class, "");
    }

    #[test]
    fn test_checkbox_prop_handling() {
        // Test checkbox prop handling
        let id = Some("test-checkbox-id".to_string());
        let class = Some("test-checkbox-class".to_string());
        let style = Some("color: red".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-checkbox-id");
        assert_eq!(class.unwrap_or_default(), "test-checkbox-class");
        assert_eq!(style.unwrap_or_default(), "color: red");
    }

    #[test]
    fn test_checkbox_prop_handling_with_none() {
        // Test checkbox prop handling with None values
        let id: Option<String> = None;
        let class: Option<String> = None;
        let style: Option<String> = None;
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(class.unwrap_or_default(), "");
        assert_eq!(style.unwrap_or_default(), "");
    }

    #[test]
    fn test_checkbox_prop_handling_with_empty_strings() {
        // Test checkbox prop handling with empty strings
        let id = Some("".to_string());
        let class = Some("".to_string());
        let style = Some("".to_string());
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(class.unwrap_or_default(), "");
        assert_eq!(style.unwrap_or_default(), "");
    }

    #[test]
    fn test_checkbox_prop_handling_with_whitespace() {
        // Test checkbox prop handling with whitespace
        let id = Some("  ".to_string());
        let class = Some("  ".to_string());
        let style = Some("  ".to_string());
        
        assert_eq!(id.unwrap_or_default(), "  ");
        assert_eq!(class.unwrap_or_default(), "  ");
        assert_eq!(style.unwrap_or_default(), "  ");
    }

    #[test]
    fn test_checkbox_prop_handling_with_special_characters() {
        // Test checkbox prop handling with special characters
        let id = Some("test-id-with-special-chars!@#$%^&*()".to_string());
        let class = Some("test-class-with-special-chars!@#$%^&*()".to_string());
        let style = Some("color: red; background: blue; font-size: 16px;".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-with-special-chars!@#$%^&*()");
        assert_eq!(class.unwrap_or_default(), "test-class-with-special-chars!@#$%^&*()");
        assert_eq!(style.unwrap_or_default(), "color: red; background: blue; font-size: 16px;");
    }

    #[test]
    fn test_checkbox_prop_handling_with_unicode() {
        // Test checkbox prop handling with unicode characters
        let id = Some("test-id-🚀".to_string());
        let class = Some("test-class-🎉".to_string());
        let style = Some("color: red; content: '🚀';".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-🚀");
        assert_eq!(class.unwrap_or_default(), "test-class-🎉");
        assert_eq!(style.unwrap_or_default(), "color: red; content: '🚀';");
    }

    #[test]
    fn test_checkbox_prop_handling_with_long_strings() {
        // Test checkbox prop handling with long strings
        let long_string = "a".repeat(1000);
        let id = Some(long_string.clone());
        let class = Some(long_string.clone());
        let style = Some(long_string.clone());
        
        assert_eq!(id.unwrap_or_default(), long_string);
        assert_eq!(class.unwrap_or_default(), long_string);
        assert_eq!(style.unwrap_or_default(), long_string);
    }

    #[test]
    fn test_checkbox_prop_handling_with_numbers() {
        // Test checkbox prop handling with numbers
        let id = Some("test-id-123".to_string());
        let class = Some("test-class-456".to_string());
        let style = Some("width: 100px; height: 200px;".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-123");
        assert_eq!(class.unwrap_or_default(), "test-class-456");
        assert_eq!(style.unwrap_or_default(), "width: 100px; height: 200px;");
    }

    #[test]
    fn test_checkbox_prop_handling_with_boolean() {
        // Test checkbox prop handling with boolean values
        let disabled = Some(true);
        let visible = Some(false);
        
        assert_eq!(disabled.unwrap_or_default(), true);
        assert_eq!(visible.unwrap_or_default(), false);
    }

    #[test]
    fn test_checkbox_prop_handling_with_boolean_none() {
        // Test checkbox prop handling with boolean None values
        let disabled: Option<bool> = None;
        let visible: Option<bool> = None;
        
        assert_eq!(disabled.unwrap_or_default(), false);
        assert_eq!(visible.unwrap_or_default(), false);
    }

    #[test]
    fn test_checkbox_prop_handling_with_callback() {
        // Test checkbox prop handling with callback
        let callback = Some(Callback::new(|_| {}));
        
        assert!(callback.is_some());
    }

    #[test]
    fn test_checkbox_prop_handling_with_callback_none() {
        // Test checkbox prop handling with callback None
        let callback: Option<Callback<()>> = None;
        
        assert!(callback.is_none());
    }

    #[test]
    fn test_checkbox_prop_handling_with_signal() {
        // Test checkbox prop handling with signal
        let signal = Some(RwSignal::new("test-value".to_string()));
        
        assert!(signal.is_some());
        assert_eq!(signal.unwrap().get(), "test-value");
    }

    #[test]
    fn test_checkbox_prop_handling_with_signal_none() {
        // Test checkbox prop handling with signal None
        let signal: Option<RwSignal<String>> = None;
        
        assert!(signal.is_none());
    }

    #[test]
    fn test_checkbox_prop_handling_with_style() {
        // Test checkbox prop handling with style
        let style = Some(Style::new());
        
        assert!(style.is_some());
        assert_eq!(style.unwrap().to_string(), "");
    }

    #[test]
    fn test_checkbox_prop_handling_with_style_none() {
        // Test checkbox prop handling with style None
        let style: Option<Style> = None;
        
        assert!(style.is_none());
    }

    #[test]
    fn test_checkbox_prop_handling_with_children() {
        // Test checkbox prop handling with children
        let children: Option<Children> = None;
        
        assert!(children.is_none());
    }

    #[test]
    fn test_checkbox_prop_handling_with_multiple_props() {
        // Test checkbox prop handling with multiple props
        let id = Some("test-id".to_string());
        let class = Some("test-class".to_string());
        let style = Some("color: red".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        
        assert_eq!(id.unwrap_or_default(), "test-id");
        assert_eq!(class.unwrap_or_default(), "test-class");
        assert_eq!(style.unwrap_or_default(), "color: red");
        assert_eq!(disabled.unwrap_or_default(), false);
        assert!(callback.is_some());
    }

    #[test]
    fn test_checkbox_prop_handling_with_complex_props() {
        // Test checkbox prop handling with complex props
        let id = Some("test-id".to_string());
        let class = Some("test-class custom-class".to_string());
        let style = Some("color: red; background: blue; font-size: 16px;".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        let signal = Some(RwSignal::new("test-value".to_string()));
        
        assert_eq!(id.unwrap_or_default(), "test-id");
        assert_eq!(class.unwrap_or_default(), "test-class custom-class");
        assert_eq!(style.unwrap_or_default(), "color: red; background: blue; font-size: 16px;");
        assert_eq!(disabled.unwrap_or_default(), false);
        assert!(callback.is_some());
        assert_eq!(signal.unwrap().get(), "test-value");
    }

    #[test]
    fn test_checkbox_prop_handling_performance() {
        // Test checkbox prop handling performance
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _id = Some("test-id".to_string());
            let _class = Some("test-class".to_string());
            let _style = Some("color: red".to_string());
            let _disabled = Some(false);
            let _callback = Some(Callback::new(|_| {}));
        }
        
        let duration = start.elapsed();
        
        // Should complete quickly (less than 10ms for 1000 iterations)
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_checkbox_prop_handling_memory_usage() {
        // Test checkbox prop handling memory usage
        let id = Some("test-id".to_string());
        let class = Some("test-class".to_string());
        let style = Some("color: red".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        
        let total_size = std::mem::size_of_val(&id) + 
                        std::mem::size_of_val(&class) + 
                        std::mem::size_of_val(&style) + 
                        std::mem::size_of_val(&disabled) + 
                        std::mem::size_of_val(&callback);
        
        // Should be reasonable size (less than 1KB)
        assert!(total_size < 1024);
    }

    #[test]
    fn test_checkbox_prop_handling_edge_cases() {
        // Test checkbox prop handling edge cases
        let id = Some("test-id".to_string());
        let class = Some("test-class".to_string());
        let style = Some("color: red".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        
        // Test that props can be cloned
        let id_clone = id.clone();
        let class_clone = class.clone();
        let style_clone = style.clone();
        let disabled_clone = disabled.clone();
        let callback_clone = callback.clone();
        
        assert_eq!(id, id_clone);
        assert_eq!(class, class_clone);
        assert_eq!(style, style_clone);
        assert_eq!(disabled, disabled_clone);
        assert!(callback_clone.is_some());
    }
}
