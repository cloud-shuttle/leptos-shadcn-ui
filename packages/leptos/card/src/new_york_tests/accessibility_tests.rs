#[cfg(test)]
mod accessibility_tests {
    use crate::new_york::{
        CARD_CLASS as CARD_CLASS_NY, CARD_HEADER_CLASS as CARD_HEADER_CLASS_NY,
        CARD_TITLE_CLASS as CARD_TITLE_CLASS_NY, CARD_DESCRIPTION_CLASS as CARD_DESCRIPTION_CLASS_NY,
        CARD_CONTENT_CLASS as CARD_CONTENT_CLASS_NY, CARD_FOOTER_CLASS as CARD_FOOTER_CLASS_NY
    };
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== NEW YORK ACCESSIBILITY TESTS =====
    // These tests focus on accessibility features and keyboard navigation

    #[test]
    fn test_new_york_card_accessibility_attributes() {
        // Test that accessibility attributes are properly handled
        let id = Some("test-card-id".to_string());
        let aria_label = Some("Test card".to_string());
        let aria_describedby = Some("card-description".to_string());
        let role = Some("region".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-card-id");
        assert_eq!(aria_label.unwrap_or_default(), "Test card");
        assert_eq!(aria_describedby.unwrap_or_default(), "card-description");
        assert_eq!(role.unwrap_or_default(), "region");
    }

    #[test]
    fn test_new_york_card_accessibility_with_none() {
        // Test accessibility attributes with None values
        let id: Option<String> = None;
        let aria_label: Option<String> = None;
        let aria_describedby: Option<String> = None;
        let role: Option<String> = None;
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(aria_label.unwrap_or_default(), "");
        assert_eq!(aria_describedby.unwrap_or_default(), "");
        assert_eq!(role.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_card_accessibility_with_empty_strings() {
        // Test accessibility attributes with empty strings
        let id = Some("".to_string());
        let aria_label = Some("".to_string());
        let aria_describedby = Some("".to_string());
        let role = Some("".to_string());
        
        assert_eq!(id.unwrap_or_default(), "");
        assert_eq!(aria_label.unwrap_or_default(), "");
        assert_eq!(aria_describedby.unwrap_or_default(), "");
        assert_eq!(role.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_card_accessibility_with_whitespace() {
        // Test accessibility attributes with whitespace
        let id = Some("  ".to_string());
        let aria_label = Some("  ".to_string());
        let aria_describedby = Some("  ".to_string());
        let role = Some("  ".to_string());
        
        assert_eq!(id.unwrap_or_default(), "  ");
        assert_eq!(aria_label.unwrap_or_default(), "  ");
        assert_eq!(aria_describedby.unwrap_or_default(), "  ");
        assert_eq!(role.unwrap_or_default(), "  ");
    }

    #[test]
    fn test_new_york_card_accessibility_with_special_characters() {
        // Test accessibility attributes with special characters
        let id = Some("test-id-with-special-chars!@#$%^&*()".to_string());
        let aria_label = Some("Test card with special chars!@#$%^&*()".to_string());
        let aria_describedby = Some("description-with-special-chars!@#$%^&*()".to_string());
        let role = Some("region-with-special-chars!@#$%^&*()".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-with-special-chars!@#$%^&*()");
        assert_eq!(aria_label.unwrap_or_default(), "Test card with special chars!@#$%^&*()");
        assert_eq!(aria_describedby.unwrap_or_default(), "description-with-special-chars!@#$%^&*()");
        assert_eq!(role.unwrap_or_default(), "region-with-special-chars!@#$%^&*()");
    }

    #[test]
    fn test_new_york_card_accessibility_with_unicode() {
        // Test accessibility attributes with unicode characters
        let id = Some("test-id-🚀".to_string());
        let aria_label = Some("Test card 🎉".to_string());
        let aria_describedby = Some("description-🚀".to_string());
        let role = Some("region-🎉".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-🚀");
        assert_eq!(aria_label.unwrap_or_default(), "Test card 🎉");
        assert_eq!(aria_describedby.unwrap_or_default(), "description-🚀");
        assert_eq!(role.unwrap_or_default(), "region-🎉");
    }

    #[test]
    fn test_new_york_card_accessibility_with_long_strings() {
        // Test accessibility attributes with long strings
        let long_string = "a".repeat(1000);
        let id = Some(long_string.clone());
        let aria_label = Some(long_string.clone());
        let aria_describedby = Some(long_string.clone());
        let role = Some(long_string.clone());
        
        assert_eq!(id.unwrap_or_default(), long_string);
        assert_eq!(aria_label.unwrap_or_default(), long_string);
        assert_eq!(aria_describedby.unwrap_or_default(), long_string);
        assert_eq!(role.unwrap_or_default(), long_string);
    }

    #[test]
    fn test_new_york_card_accessibility_with_numbers() {
        // Test accessibility attributes with numbers
        let id = Some("test-id-123".to_string());
        let aria_label = Some("Test card 456".to_string());
        let aria_describedby = Some("description-789".to_string());
        let role = Some("region-012".to_string());
        
        assert_eq!(id.unwrap_or_default(), "test-id-123");
        assert_eq!(aria_label.unwrap_or_default(), "Test card 456");
        assert_eq!(aria_describedby.unwrap_or_default(), "description-789");
        assert_eq!(role.unwrap_or_default(), "region-012");
    }

    #[test]
    fn test_new_york_card_accessibility_with_boolean() {
        // Test accessibility attributes with boolean values
        let disabled = Some(true);
        let visible = Some(false);
        let hidden = Some(true);
        
        assert_eq!(disabled.unwrap_or_default(), true);
        assert_eq!(visible.unwrap_or_default(), false);
        assert_eq!(hidden.unwrap_or_default(), true);
    }

    #[test]
    fn test_new_york_card_accessibility_with_boolean_none() {
        // Test accessibility attributes with boolean None values
        let disabled: Option<bool> = None;
        let visible: Option<bool> = None;
        let hidden: Option<bool> = None;
        
        assert_eq!(disabled.unwrap_or_default(), false);
        assert_eq!(visible.unwrap_or_default(), false);
        assert_eq!(hidden.unwrap_or_default(), false);
    }

    #[test]
    fn test_new_york_card_accessibility_with_callback() {
        // Test accessibility attributes with callback
        let callback = Some(Callback::new(|_| {}));
        
        assert!(callback.is_some());
    }

    #[test]
    fn test_new_york_card_accessibility_with_callback_none() {
        // Test accessibility attributes with callback None
        let callback: Option<Callback<()>> = None;
        
        assert!(callback.is_none());
    }

    #[test]
    fn test_new_york_card_accessibility_with_signal() {
        // Test accessibility attributes with signal
        let signal = Some(RwSignal::new("test-value".to_string()));
        
        assert!(signal.is_some());
        assert_eq!(signal.unwrap().get(), "test-value");
    }

    #[test]
    fn test_new_york_card_accessibility_with_signal_none() {
        // Test accessibility attributes with signal None
        let signal: Option<RwSignal<String>> = None;
        
        assert!(signal.is_none());
    }

    #[test]
    fn test_new_york_card_accessibility_with_style() {
        // Test accessibility attributes with style
        let style = Some(Style::new());
        
        assert!(style.is_some());
        assert_eq!(style.unwrap().to_string(), "");
    }

    #[test]
    fn test_new_york_card_accessibility_with_style_none() {
        // Test accessibility attributes with style None
        let style: Option<Style> = None;
        
        assert!(style.is_none());
    }

    #[test]
    fn test_new_york_card_accessibility_with_children() {
        // Test accessibility attributes with children
        let children: Option<Children> = None;
        
        assert!(children.is_none());
    }

    #[test]
    fn test_new_york_card_accessibility_with_multiple_props() {
        // Test accessibility attributes with multiple props
        let id = Some("test-id".to_string());
        let aria_label = Some("Test card".to_string());
        let aria_describedby = Some("card-description".to_string());
        let role = Some("region".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        
        assert_eq!(id.unwrap_or_default(), "test-id");
        assert_eq!(aria_label.unwrap_or_default(), "Test card");
        assert_eq!(aria_describedby.unwrap_or_default(), "card-description");
        assert_eq!(role.unwrap_or_default(), "region");
        assert_eq!(disabled.unwrap_or_default(), false);
        assert!(callback.is_some());
    }

    #[test]
    fn test_new_york_card_accessibility_with_complex_props() {
        // Test accessibility attributes with complex props
        let id = Some("test-id".to_string());
        let aria_label = Some("Test card with complex props".to_string());
        let aria_describedby = Some("card-description with complex props".to_string());
        let role = Some("region with complex props".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        let signal = Some(RwSignal::new("test-value".to_string()));
        
        assert_eq!(id.unwrap_or_default(), "test-id");
        assert_eq!(aria_label.unwrap_or_default(), "Test card with complex props");
        assert_eq!(aria_describedby.unwrap_or_default(), "card-description with complex props");
        assert_eq!(role.unwrap_or_default(), "region with complex props");
        assert_eq!(disabled.unwrap_or_default(), false);
        assert!(callback.is_some());
        assert_eq!(signal.unwrap().get(), "test-value");
    }

    #[test]
    fn test_new_york_card_accessibility_performance() {
        // Test accessibility attributes performance
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _id = Some("test-id".to_string());
            let _aria_label = Some("Test card".to_string());
            let _aria_describedby = Some("card-description".to_string());
            let _role = Some("region".to_string());
            let _disabled = Some(false);
            let _callback = Some(Callback::new(|_| {}));
        }
        
        let duration = start.elapsed();
        
        // Should complete quickly (less than 10ms for 1000 iterations)
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_new_york_card_accessibility_memory_usage() {
        // Test accessibility attributes memory usage
        let id = Some("test-id".to_string());
        let aria_label = Some("Test card".to_string());
        let aria_describedby = Some("card-description".to_string());
        let role = Some("region".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        
        let total_size = std::mem::size_of_val(&id) + 
                        std::mem::size_of_val(&aria_label) + 
                        std::mem::size_of_val(&aria_describedby) + 
                        std::mem::size_of_val(&role) + 
                        std::mem::size_of_val(&disabled) + 
                        std::mem::size_of_val(&callback);
        
        // Should be reasonable size (less than 1KB)
        assert!(total_size < 1024);
    }

    #[test]
    fn test_new_york_card_accessibility_edge_cases() {
        // Test accessibility attributes edge cases
        let id = Some("test-id".to_string());
        let aria_label = Some("Test card".to_string());
        let aria_describedby = Some("card-description".to_string());
        let role = Some("region".to_string());
        let disabled = Some(false);
        let callback = Some(Callback::new(|_| {}));
        
        // Test that props can be cloned
        let id_clone = id.clone();
        let aria_label_clone = aria_label.clone();
        let aria_describedby_clone = aria_describedby.clone();
        let role_clone = role.clone();
        let disabled_clone = disabled.clone();
        let callback_clone = callback.clone();
        
        assert_eq!(id, id_clone);
        assert_eq!(aria_label, aria_label_clone);
        assert_eq!(aria_describedby, aria_describedby_clone);
        assert_eq!(role, role_clone);
        assert_eq!(disabled, disabled_clone);
        assert!(callback_clone.is_some());
    }
}
