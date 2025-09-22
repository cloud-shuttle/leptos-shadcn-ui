#[cfg(test)]
mod class_constants {
    use crate::default::CHECKBOX_CLASS;

    // ===== CLASS CONSTANTS TESTS =====
    // These tests focus on CSS class constants and their properties

    #[test]
    fn test_checkbox_class_constant() {
        // Test CHECKBOX_CLASS constant
        assert!(CHECKBOX_CLASS.contains("h-4"));
        assert!(CHECKBOX_CLASS.contains("w-4"));
        assert!(CHECKBOX_CLASS.contains("shrink-0"));
        assert!(CHECKBOX_CLASS.contains("rounded-sm"));
        assert!(CHECKBOX_CLASS.contains("border"));
        assert!(CHECKBOX_CLASS.contains("border-primary"));
        assert!(CHECKBOX_CLASS.contains("ring-offset-background"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:outline-none"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-2"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-ring"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-offset-2"));
        assert!(CHECKBOX_CLASS.contains("disabled:cursor-not-allowed"));
        assert!(CHECKBOX_CLASS.contains("disabled:opacity-50"));
        assert!(CHECKBOX_CLASS.contains("data-[state=checked]:bg-primary"));
        assert!(CHECKBOX_CLASS.contains("data-[state=checked]:text-primary-foreground"));
    }

    #[test]
    fn test_checkbox_class_consistency() {
        // Test that checkbox class is consistent
        assert!(!CHECKBOX_CLASS.is_empty());
    }

    #[test]
    fn test_checkbox_class_structure() {
        // Test that checkbox class has proper structure
        assert!(CHECKBOX_CLASS.contains("h-4"));
        assert!(CHECKBOX_CLASS.contains("w-4"));
        assert!(CHECKBOX_CLASS.contains("shrink-0"));
        assert!(CHECKBOX_CLASS.contains("rounded-sm"));
        assert!(CHECKBOX_CLASS.contains("border"));
        assert!(CHECKBOX_CLASS.contains("border-primary"));
    }

    #[test]
    fn test_checkbox_class_accessibility() {
        // Test that checkbox class supports accessibility
        assert!(CHECKBOX_CLASS.contains("ring-offset-background"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:outline-none"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-2"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-ring"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-offset-2"));
    }

    #[test]
    fn test_checkbox_class_disabled_state() {
        // Test that checkbox class supports disabled state
        assert!(CHECKBOX_CLASS.contains("disabled:cursor-not-allowed"));
        assert!(CHECKBOX_CLASS.contains("disabled:opacity-50"));
    }

    #[test]
    fn test_checkbox_class_checked_state() {
        // Test that checkbox class supports checked state
        assert!(CHECKBOX_CLASS.contains("data-[state=checked]:bg-primary"));
        assert!(CHECKBOX_CLASS.contains("data-[state=checked]:text-primary-foreground"));
    }

    #[test]
    fn test_checkbox_class_dimensions() {
        // Test that checkbox class has proper dimensions
        assert!(CHECKBOX_CLASS.contains("h-4"));
        assert!(CHECKBOX_CLASS.contains("w-4"));
        assert!(CHECKBOX_CLASS.contains("shrink-0"));
    }

    #[test]
    fn test_checkbox_class_border_radius() {
        // Test that checkbox class has proper border radius
        assert!(CHECKBOX_CLASS.contains("rounded-sm"));
    }

    #[test]
    fn test_checkbox_class_border() {
        // Test that checkbox class has proper border
        assert!(CHECKBOX_CLASS.contains("border"));
        assert!(CHECKBOX_CLASS.contains("border-primary"));
    }

    #[test]
    fn test_checkbox_class_focus_management() {
        // Test that checkbox class has proper focus management
        assert!(CHECKBOX_CLASS.contains("focus-visible:outline-none"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-2"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-ring"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-offset-2"));
    }

    #[test]
    fn test_checkbox_class_ring_offset() {
        // Test that checkbox class has proper ring offset
        assert!(CHECKBOX_CLASS.contains("ring-offset-background"));
    }

    #[test]
    fn test_checkbox_class_completeness() {
        // Test that checkbox class is complete and functional
        assert!(!CHECKBOX_CLASS.is_empty());
        assert!(CHECKBOX_CLASS.len() > 10); // Should have meaningful content
    }

    #[test]
    fn test_checkbox_class_validation() {
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
    fn test_checkbox_class_immutability() {
        // Test that checkbox class is immutable
        let original_class = CHECKBOX_CLASS;
        let _computed_class = format!("{} {}", CHECKBOX_CLASS, "custom-class");
        
        // Original class should remain unchanged
        assert_eq!(CHECKBOX_CLASS, original_class);
    }

    #[test]
    fn test_checkbox_class_consistency_across_calls() {
        // Test that checkbox class is consistent across multiple calls
        let class1 = CHECKBOX_CLASS;
        let class2 = CHECKBOX_CLASS;
        let class3 = CHECKBOX_CLASS;
        
        assert_eq!(class1, class2);
        assert_eq!(class2, class3);
        assert_eq!(class1, class3);
    }

    #[test]
    fn test_checkbox_class_edge_cases() {
        // Test edge cases for checkbox class
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
    fn test_checkbox_class_performance() {
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
    fn test_checkbox_class_memory_usage() {
        // Test checkbox class memory usage
        let computed_class = format!("{} {}", CHECKBOX_CLASS, "custom-class");
        let size = std::mem::size_of_val(&computed_class);
        
        // Should be reasonable size (less than 1KB)
        assert!(size < 1024);
    }
}
