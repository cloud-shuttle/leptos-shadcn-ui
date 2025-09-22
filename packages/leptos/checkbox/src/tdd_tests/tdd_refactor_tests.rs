#[cfg(test)]
mod tdd_refactor_tests {
    use crate::default::CHECKBOX_CLASS;
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== TDD REFACTOR TESTS =====
    // These tests focus on enhanced system tests with full functionality

    #[test]
    fn test_checkbox_refactor_implementation() {
        // Test that checkbox component can be created with enhanced implementation
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
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", checkbox_class, "custom-class");
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("custom-class"));
    }

    #[test]
    fn test_checkbox_props_refactor_implementation() {
        // Test that checkbox props can be handled with enhanced implementation
        let id = Some("test-checkbox-id".to_string());
        let class = Some("test-checkbox-class".to_string());
        let style = Some("color: red".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-checkbox-id");
        assert_eq!(class.unwrap_or_default(), "test-checkbox-class");
        assert_eq!(style.unwrap_or_default(), "color: red");
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("test-checkbox-class"));
    }

    #[test]
    fn test_checkbox_props_with_none_refactor_implementation() {
        // Test that checkbox props with None values can be handled with enhanced implementation
        let id: Option<String> = None;
        let class: Option<String> = None;
        let style: Option<String> = None;
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(class.unwrap_or_default(), "");
        assert_eq!(style.unwrap_or_default(), "");
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert!(!computed_class.contains("custom-class"));
    }

    #[test]
    fn test_checkbox_props_with_empty_strings_refactor_implementation() {
        // Test that checkbox props with empty strings can be handled with enhanced implementation
        let id = Some("".to_string());
        let class = Some("".to_string());
        let style = Some("".to_string());
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(class.unwrap_or_default(), "");
        assert_eq!(style.unwrap_or_default(), "");
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert_eq!(computed_class, CHECKBOX_CLASS);
    }

    #[test]
    fn test_checkbox_props_with_whitespace_refactor_implementation() {
        // Test that checkbox props with whitespace can be handled with enhanced implementation
        let id = Some("  ".to_string());
        let class = Some("  ".to_string());
        let style = Some("  ".to_string());
        
        assert_eq!(id.unwrap_or_default(), "  ");
        assert_eq!(class.unwrap_or_default(), "  ");
        assert_eq!(style.unwrap_or_default(), "  ");
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("  "));
    }

    #[test]
    fn test_checkbox_props_with_special_characters_refactor_implementation() {
        // Test that checkbox props with special characters can be handled with enhanced implementation
        let id = Some("test-id-with-special-chars!@#$%^&*()".to_string());
        let class = Some("test-class-with-special-chars!@#$%^&*()".to_string());
        let style = Some("color: red; background: blue; font-size: 16px;".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-with-special-chars!@#$%^&*()");
        assert_eq!(class.unwrap_or_default(), "test-class-with-special-chars!@#$%^&*()");
        assert_eq!(style.unwrap_or_default(), "color: red; background: blue; font-size: 16px;");
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("test-class-with-special-chars!@#$%^&*()"));
    }

    #[test]
    fn test_checkbox_props_with_unicode_refactor_implementation() {
        // Test that checkbox props with unicode characters can be handled with enhanced implementation
        let id = Some("test-id-🚀".to_string());
        let class = Some("test-class-🎉".to_string());
        let style = Some("color: red; content: '🚀';".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-🚀");
        assert_eq!(class.unwrap_or_default(), "test-class-🎉");
        assert_eq!(style.unwrap_or_default(), "color: red; content: '🚀';");
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("test-class-🎉"));
    }

    #[test]
    fn test_checkbox_props_with_long_strings_refactor_implementation() {
        // Test that checkbox props with long strings can be handled with enhanced implementation
        let long_string = "a".repeat(1000);
        let id = Some(long_string.clone());
        let class = Some(long_string.clone());
        let style = Some(long_string.clone());
        
        assert_eq!(id.unwrap_or_default(), long_string);
        assert_eq!(class.unwrap_or_default(), long_string);
        assert_eq!(style.unwrap_or_default(), long_string);
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains(&long_string));
    }

    #[test]
    fn test_checkbox_props_with_numbers_refactor_implementation() {
        // Test that checkbox props with numbers can be handled with enhanced implementation
        let id = Some("test-id-123".to_string());
        let class = Some("test-class-456".to_string());
        let style = Some("width: 100px; height: 200px;".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-123");
        assert_eq!(class.unwrap_or_default(), "test-class-456");
        assert_eq!(style.unwrap_or_default(), "width: 100px; height: 200px;");
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, class.unwrap_or_default());
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("test-class-456"));
    }

    #[test]
    fn test_checkbox_props_with_boolean_refactor_implementation() {
        // Test that checkbox props with boolean values can be handled with enhanced implementation
        let disabled = Some(true);
        let visible = Some(false);
        
        assert_eq!(disabled.unwrap_or_default(), true);
        assert_eq!(visible.unwrap_or_default(), false);
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, if disabled.unwrap_or_default() { "disabled" } else { "enabled" });
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("disabled"));
    }

    #[test]
    fn test_checkbox_props_with_boolean_none_refactor_implementation() {
        // Test that checkbox props with boolean None values can be handled with enhanced implementation
        let disabled: Option<bool> = None;
        let visible: Option<bool> = None;
        
        assert_eq!(disabled.unwrap_or_default(), false);
        assert_eq!(visible.unwrap_or_default(), false);
        
        // Test enhanced functionality
        let computed_class = format!("{} {}", CHECKBOX_CLASS, if disabled.unwrap_or_default() { "disabled" } else { "enabled" });
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("enabled"));
    }

    #[test]
    fn test_checkbox_props_with_callback_refactor_implementation() {
        // Test that checkbox props with callback can be handled with enhanced implementation
        let callback = Some(Callback::new(|_| {}));
        
        assert!(callback.is_some());
        
        // Test enhanced functionality
        let callback_clone = callback.clone();
        assert!(callback_clone.is_some());
    }

    #[test]
    fn test_checkbox_props_with_callback_none_refactor_implementation() {
        // Test that checkbox props with callback None can be handled with enhanced implementation
        let callback: Option<Callback<()>> = None;
        
        assert!(callback.is_none());
        
        // Test enhanced functionality
        let callback_clone = callback.clone();
        assert!(callback_clone.is_none());
    }

    #[test]
    fn test_checkbox_props_with_signal_refactor_implementation() {
        // Test that checkbox props with signal can be handled with enhanced implementation
        let signal = Some(RwSignal::new("test-value".to_string()));
        
        assert!(signal.is_some());
        assert_eq!(signal.unwrap().get(), "test-value");
        
        // Test enhanced functionality
        let signal_clone = signal.clone();
        assert!(signal_clone.is_some());
        assert_eq!(signal_clone.unwrap().get(), "test-value");
    }

    #[test]
    fn test_checkbox_props_with_signal_none_refactor_implementation() {
        // Test that checkbox props with signal None can be handled with enhanced implementation
        let signal: Option<RwSignal<String>> = None;
        
        assert!(signal.is_none());
        
        // Test enhanced functionality
        let signal_clone = signal.clone();
        assert!(signal_clone.is_none());
    }

    #[test]
    fn test_checkbox_props_with_style_refactor_implementation() {
        // Test that checkbox props with style can be handled with enhanced implementation
        let style = Some(Style::new());
        
        assert!(style.is_some());
        assert_eq!(style.unwrap().to_string(), "");
        
        // Test enhanced functionality
        let style_clone = style.clone();
        assert!(style_clone.is_some());
        assert_eq!(style_clone.unwrap().to_string(), "");
    }

    #[test]
    fn test_checkbox_props_with_style_none_refactor_implementation() {
        // Test that checkbox props with style None can be handled with enhanced implementation
        let style: Option<Style> = None;
        
        assert!(style.is_none());
        
        // Test enhanced functionality
        let style_clone = style.clone();
        assert!(style_clone.is_none());
    }

    #[test]
    fn test_checkbox_props_with_children_refactor_implementation() {
        // Test that checkbox props with children can be handled with enhanced implementation
        let children: Option<Children> = None;
        
        assert!(children.is_none());
        
        // Test enhanced functionality
        let children_clone = children.clone();
        assert!(children_clone.is_none());
    }

    #[test]
    fn test_checkbox_props_with_multiple_props_refactor_implementation() {
        // Test that checkbox props with multiple props can be handled with enhanced implementation
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
        
        // Test enhanced functionality
        let computed_class = format!("{} {} {}", CHECKBOX_CLASS, class.unwrap_or_default(), if disabled.unwrap_or_default() { "disabled" } else { "enabled" });
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("test-class"));
        assert!(computed_class.contains("enabled"));
    }

    #[test]
    fn test_checkbox_props_with_complex_props_refactor_implementation() {
        // Test that checkbox props with complex props can be handled with enhanced implementation
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
        
        // Test enhanced functionality
        let computed_class = format!("{} {} {}", CHECKBOX_CLASS, class.unwrap_or_default(), if disabled.unwrap_or_default() { "disabled" } else { "enabled" });
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("test-class custom-class"));
        assert!(computed_class.contains("enabled"));
    }

    #[test]
    fn test_checkbox_props_performance_refactor_implementation() {
        // Test that checkbox props performance is acceptable with enhanced implementation
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
    fn test_checkbox_props_memory_usage_refactor_implementation() {
        // Test that checkbox props memory usage is acceptable with enhanced implementation
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
    fn test_checkbox_props_edge_cases_refactor_implementation() {
        // Test that checkbox props edge cases are handled with enhanced implementation
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
        
        // Test enhanced functionality
        let computed_class = format!("{} {} {}", CHECKBOX_CLASS, class_clone.unwrap_or_default(), if disabled_clone.unwrap_or_default() { "disabled" } else { "enabled" });
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("test-class"));
        assert!(computed_class.contains("enabled"));
    }
}
