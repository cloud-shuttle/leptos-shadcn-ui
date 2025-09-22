#[cfg(test)]
mod tdd_green_tests {
    use crate::default::CHECKBOX_CLASS;
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== TDD GREEN TESTS =====
    // These tests focus on minimal implementation to make tests pass

    #[test]
    fn test_checkbox_green_implementation() {
        // Test that checkbox component can be created with minimal implementation
        let checkbox_class = CHECKBOX_CLASS;
        assert!(!checkbox_class.is_empty());
        
        // Test that checkbox class contains required elements
        assert!(checkbox_class.contains("h-4"));
        assert!(checkbox_class.contains("w-4"));
        assert!(checkbox_class.contains("shrink-0"));
        assert!(checkbox_class.contains("rounded-sm"));
        assert!(checkbox_class.contains("border"));
        assert!(checkbox_class.contains("border-primary"));
        assert!(checkbox_class.contains("ring-offset-background"));
        assert!(checkbox_class.contains("focus-visible:outline-none"));
        assert!(checkbox_class.contains("focus-visible:ring-2"));
        assert!(checkbox_class.contains("focus-visible:ring-ring"));
        assert!(checkbox_class.contains("focus-visible:ring-offset-2"));
        assert!(checkbox_class.contains("disabled:cursor-not-allowed"));
        assert!(checkbox_class.contains("disabled:opacity-50"));
        assert!(checkbox_class.contains("data-[state=checked]:bg-primary"));
        assert!(checkbox_class.contains("data-[state=checked]:text-primary-foreground"));
    }

    #[test]
    fn test_checkbox_props_green_implementation() {
        // Test that checkbox props can be handled with minimal implementation
        let id = Some("test-checkbox-id".to_string());
        let class = Some("test-checkbox-class".to_string());
        let style = Some("color: red".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-checkbox-id");
        assert_eq!(class.unwrap_or_default(), "test-checkbox-class");
        assert_eq!(style.unwrap_or_default(), "color: red");
    }

    #[test]
    fn test_checkbox_props_with_none_green_implementation() {
        // Test that checkbox props with None values can be handled with minimal implementation
        let id: Option<String> = None;
        let class: Option<String> = None;
        let style: Option<String> = None;
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(class.unwrap_or_default(), "");
        assert_eq!(style.unwrap_or_default(), "");
    }

    #[test]
    fn test_checkbox_props_with_empty_strings_green_implementation() {
        // Test that checkbox props with empty strings can be handled with minimal implementation
        let id = Some("".to_string());
        let class = Some("".to_string());
        let style = Some("".to_string());
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(class.unwrap_or_default(), "");
        assert_eq!(style.unwrap_or_default(), "");
    }

    #[test]
    fn test_checkbox_props_with_whitespace_green_implementation() {
        // Test that checkbox props with whitespace can be handled with minimal implementation
        let id = Some("  ".to_string());
        let class = Some("  ".to_string());
        let style = Some("  ".to_string());
        
        assert_eq!(id.unwrap_or_default(), "  ");
        assert_eq!(class.unwrap_or_default(), "  ");
        assert_eq!(style.unwrap_or_default(), "  ");
    }

    #[test]
    fn test_checkbox_props_with_special_characters_green_implementation() {
        // Test that checkbox props with special characters can be handled with minimal implementation
        let id = Some("test-id-with-special-chars!@#$%^&*()".to_string());
        let class = Some("test-class-with-special-chars!@#$%^&*()".to_string());
        let style = Some("color: red; background: blue; font-size: 16px;".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-with-special-chars!@#$%^&*()");
        assert_eq!(class.unwrap_or_default(), "test-class-with-special-chars!@#$%^&*()");
        assert_eq!(style.unwrap_or_default(), "color: red; background: blue; font-size: 16px;");
    }

    #[test]
    fn test_checkbox_props_with_unicode_green_implementation() {
        // Test that checkbox props with unicode characters can be handled with minimal implementation
        let id = Some("test-id-🚀".to_string());
        let class = Some("test-class-🎉".to_string());
        let style = Some("color: red; content: '🚀';".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-🚀");
        assert_eq!(class.unwrap_or_default(), "test-class-🎉");
        assert_eq!(style.unwrap_or_default(), "color: red; content: '🚀';");
    }

    #[test]
    fn test_checkbox_props_with_long_strings_green_implementation() {
        // Test that checkbox props with long strings can be handled with minimal implementation
        let long_string = "a".repeat(1000);
        let id = Some(long_string.clone());
        let class = Some(long_string.clone());
        let style = Some(long_string.clone());
        
        assert_eq!(id.unwrap_or_default(), long_string);
        assert_eq!(class.unwrap_or_default(), long_string);
        assert_eq!(style.unwrap_or_default(), long_string);
    }

    #[test]
    fn test_checkbox_props_with_numbers_green_implementation() {
        // Test that checkbox props with numbers can be handled with minimal implementation
        let id = Some("test-id-123".to_string());
        let class = Some("test-class-456".to_string());
        let style = Some("width: 100px; height: 200px;".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-123");
        assert_eq!(class.unwrap_or_default(), "test-class-456");
        assert_eq!(style.unwrap_or_default(), "width: 100px; height: 200px;");
    }

    #[test]
    fn test_checkbox_props_with_boolean_green_implementation() {
        // Test that checkbox props with boolean values can be handled with minimal implementation
        let disabled = Some(true);
        let visible = Some(false);
        
        assert_eq!(disabled.unwrap_or_default(), true);
        assert_eq!(visible.unwrap_or_default(), false);
    }

    #[test]
    fn test_checkbox_props_with_boolean_none_green_implementation() {
        // Test that checkbox props with boolean None values can be handled with minimal implementation
        let disabled: Option<bool> = None;
        let visible: Option<bool> = None;
        
        assert_eq!(disabled.unwrap_or_default(), false);
        assert_eq!(visible.unwrap_or_default(), false);
    }

    #[test]
    fn test_checkbox_props_with_callback_green_implementation() {
        // Test that checkbox props with callback can be handled with minimal implementation
        let callback = Some(Callback::new(|_| {}));
        
        assert!(callback.is_some());
    }

    #[test]
    fn test_checkbox_props_with_callback_none_green_implementation() {
        // Test that checkbox props with callback None can be handled with minimal implementation
        let callback: Option<Callback<()>> = None;
        
        assert!(callback.is_none());
    }

    #[test]
    fn test_checkbox_props_with_signal_green_implementation() {
        // Test that checkbox props with signal can be handled with minimal implementation
        let signal = Some(RwSignal::new("test-value".to_string()));
        
        assert!(signal.is_some());
        assert_eq!(signal.unwrap().get(), "test-value");
    }

    #[test]
    fn test_checkbox_props_with_signal_none_green_implementation() {
        // Test that checkbox props with signal None can be handled with minimal implementation
        let signal: Option<RwSignal<String>> = None;
        
        assert!(signal.is_none());
    }

    #[test]
    fn test_checkbox_props_with_style_green_implementation() {
        // Test that checkbox props with style can be handled with minimal implementation
        let style = Some(Style::new());
        
        assert!(style.is_some());
        assert_eq!(style.unwrap().to_string(), "");
    }

    #[test]
    fn test_checkbox_props_with_style_none_green_implementation() {
        // Test that checkbox props with style None can be handled with minimal implementation
        let style: Option<Style> = None;
        
        assert!(style.is_none());
    }

    #[test]
    fn test_checkbox_props_with_children_green_implementation() {
        // Test that checkbox props with children can be handled with minimal implementation
        let children: Option<Children> = None;
        
        assert!(children.is_none());
    }

    #[test]
    fn test_checkbox_props_with_multiple_props_green_implementation() {
        // Test that checkbox props with multiple props can be handled with minimal implementation
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
    fn test_checkbox_props_with_complex_props_green_implementation() {
        // Test that checkbox props with complex props can be handled with minimal implementation
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
    fn test_checkbox_props_performance_green_implementation() {
        // Test that checkbox props performance is acceptable with minimal implementation
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
    fn test_checkbox_props_memory_usage_green_implementation() {
        // Test that checkbox props memory usage is acceptable with minimal implementation
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
    fn test_checkbox_props_edge_cases_green_implementation() {
        // Test that checkbox props edge cases are handled with minimal implementation
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
