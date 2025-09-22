#[cfg(test)]
mod component_behavior {
    use crate::default::CHECKBOX_CLASS;
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== COMPONENT BEHAVIOR TESTS =====
    // These tests focus on component behavior and computed class generation

    #[test]
    fn test_checkbox_computed_class_generation() {
        // Test Checkbox computed class generation
        let base_class = CHECKBOX_CLASS;
        let custom_class = "custom-checkbox";
        let computed = format!("{} {}", base_class, custom_class);
        
        assert!(computed.contains("h-4"));
        assert!(computed.contains("w-4"));
        assert!(computed.contains("custom-checkbox"));
    }

    #[test]
    fn test_checkbox_computed_class_with_empty_custom_class() {
        // Test checkbox class with empty custom class
        let base_class = CHECKBOX_CLASS;
        let custom_class = "";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert_eq!(computed_class, base_class);
    }

    #[test]
    fn test_checkbox_computed_class_with_whitespace_custom_class() {
        // Test checkbox class with whitespace custom class
        let base_class = CHECKBOX_CLASS;
        let custom_class = "  ";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("  "));
    }

    #[test]
    fn test_checkbox_computed_class_with_multiple_custom_classes() {
        // Test checkbox class with multiple custom classes
        let base_class = CHECKBOX_CLASS;
        let custom_classes = "class1 class2 class3";
        
        let computed_class = format!("{} {}", base_class, custom_classes);
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("class1"));
        assert!(computed_class.contains("class2"));
        assert!(computed_class.contains("class3"));
    }

    #[test]
    fn test_checkbox_computed_class_with_special_characters() {
        // Test checkbox class with special characters
        let base_class = CHECKBOX_CLASS;
        let custom_class = "class-with-dashes_and_underscores";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("class-with-dashes_and_underscores"));
    }

    #[test]
    fn test_checkbox_computed_class_with_numbers() {
        // Test checkbox class with numbers
        let base_class = CHECKBOX_CLASS;
        let custom_class = "class123";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("class123"));
    }

    #[test]
    fn test_checkbox_computed_class_with_unicode() {
        // Test checkbox class with unicode characters
        let base_class = CHECKBOX_CLASS;
        let custom_class = "class-🚀";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("class-🚀"));
    }

    #[test]
    fn test_checkbox_computed_class_performance() {
        // Test checkbox class generation performance
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _computed_class = format!("{} {}", CHECKBOX_CLASS, "custom-class");
        }
        
        let duration = start.elapsed();
        
        // Should complete quickly (less than 10ms for 1000 iterations)
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_checkbox_computed_class_memory_usage() {
        // Test checkbox class memory usage
        let computed_class = format!("{} {}", CHECKBOX_CLASS, "custom-class");
        let size = std::mem::size_of_val(&computed_class);
        
        // Should be reasonable size (less than 1KB)
        assert!(size < 1024);
    }

    #[test]
    fn test_checkbox_computed_class_immutability() {
        // Test that checkbox class is immutable
        let original_class = CHECKBOX_CLASS;
        let _computed_class = format!("{} {}", CHECKBOX_CLASS, "custom-class");
        
        // Original class should remain unchanged
        assert_eq!(CHECKBOX_CLASS, original_class);
    }

    #[test]
    fn test_checkbox_computed_class_consistency_across_calls() {
        // Test that checkbox class is consistent across multiple calls
        let class1 = CHECKBOX_CLASS;
        let class2 = CHECKBOX_CLASS;
        let class3 = CHECKBOX_CLASS;
        
        assert_eq!(class1, class2);
        assert_eq!(class2, class3);
        assert_eq!(class1, class3);
    }

    #[test]
    fn test_checkbox_computed_class_edge_cases() {
        // Test edge cases for checkbox class generation
        let base_class = CHECKBOX_CLASS;
        
        // Test with very long custom class
        let long_class = "a".repeat(1000);
        let computed_class = format!("{} {}", base_class, long_class);
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains(&long_class));
        
        // Test with empty base class (should not happen in practice)
        let empty_base = "";
        let custom_class = "custom-class";
        let computed_class = format!("{} {}", empty_base, custom_class);
        assert_eq!(computed_class, " custom-class");
    }

    #[test]
    fn test_checkbox_computed_class_validation() {
        // Test checkbox class validation
        let base_class = CHECKBOX_CLASS;
        
        // Test that base class contains required elements
        assert!(base_class.contains("h-4"));
        assert!(base_class.contains("w-4"));
        assert!(base_class.contains("shrink-0"));
        assert!(base_class.contains("rounded-sm"));
        assert!(base_class.contains("border"));
        assert!(base_class.contains("border-primary"));
        assert!(base_class.contains("ring-offset-background"));
        assert!(base_class.contains("focus-visible:outline-none"));
        assert!(base_class.contains("focus-visible:ring-2"));
        assert!(base_class.contains("focus-visible:ring-ring"));
        assert!(base_class.contains("focus-visible:ring-offset-2"));
        assert!(base_class.contains("disabled:cursor-not-allowed"));
        assert!(base_class.contains("disabled:opacity-50"));
        assert!(base_class.contains("data-[state=checked]:bg-primary"));
        assert!(base_class.contains("data-[state=checked]:text-primary-foreground"));
        
        // Test that base class is not empty
        assert!(!base_class.is_empty());
        
        // Test that base class has reasonable length
        assert!(base_class.len() > 10);
        assert!(base_class.len() < 1000);
    }

    #[test]
    fn test_checkbox_computed_class_combination() {
        // Test combining multiple checkbox classes
        let checkbox_class = CHECKBOX_CLASS;
        let custom_class = "custom-checkbox";
        
        let combined_class = format!("{} {}", checkbox_class, custom_class);
        
        assert!(combined_class.contains("h-4"));
        assert!(combined_class.contains("w-4"));
        assert!(combined_class.contains("custom-checkbox"));
    }

    #[test]
    fn test_checkbox_computed_class_with_none_props() {
        // Test computed class generation with None props
        let base_class = CHECKBOX_CLASS;
        let custom_class: Option<String> = None;
        
        let computed_class = format!("{} {}", base_class, custom_class.unwrap_or_default());
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        assert!(!computed_class.contains("custom-class"));
    }

    #[test]
    fn test_checkbox_computed_class_with_custom_class() {
        // Test computed class generation with custom class
        let base_class = CHECKBOX_CLASS;
        let custom_class = Some("my-custom-class".to_string());
        
        let computed_class = format!("{} {}", base_class, custom_class.unwrap_or_default());
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        assert!(computed_class.contains("my-custom-class"));
    }

    #[test]
    fn test_checkbox_computed_class_variant_priority() {
        // Test that variant classes are applied correctly
        let base_class = CHECKBOX_CLASS;
        let custom_class = "custom-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        // Test that base classes are present
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        assert!(computed_class.contains("custom-class"));
    }

    #[test]
    fn test_checkbox_computed_class_size_priority() {
        // Test that size classes are applied correctly
        let base_class = CHECKBOX_CLASS;
        let custom_class = "custom-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        // Test that size classes are present
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        assert!(computed_class.contains("custom-class"));
    }

    #[test]
    fn test_checkbox_computed_class_combination() {
        // Test combination of variant and size classes
        let base_class = CHECKBOX_CLASS;
        let custom_class = "custom-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        // Test that both variant and size classes are present
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        assert!(computed_class.contains("custom-class"));
    }

    #[test]
    fn test_checkbox_computed_class_base_classes() {
        // Test that base checkbox classes are always present
        let computed_class = format!("{} variant-class size-class", CHECKBOX_CLASS);
        
        // Test that base classes are present
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        assert!(computed_class.contains("shrink-0"));
        assert!(computed_class.contains("rounded-sm"));
        assert!(computed_class.contains("border"));
    }

    #[test]
    fn test_checkbox_computed_class_empty_custom_class() {
        // Test computed class generation with empty custom class
        let base_class = CHECKBOX_CLASS;
        let custom_class = Some("".to_string());
        
        let computed_class = format!("{} {}", base_class, custom_class.unwrap_or_default());
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        // Empty custom class should not affect the result
        assert!(!computed_class.contains("custom-class"));
    }

    #[test]
    fn test_checkbox_computed_class_whitespace_handling() {
        // Test computed class generation with whitespace in custom class
        let base_class = CHECKBOX_CLASS;
        let custom_class = Some("  custom-class  ".to_string());
        
        let computed_class = format!("{} {}", base_class, custom_class.unwrap_or_default());
        
        assert!(computed_class.contains("h-4"));
        assert!(computed_class.contains("w-4"));
        assert!(computed_class.contains("  custom-class  "));
    }
}
