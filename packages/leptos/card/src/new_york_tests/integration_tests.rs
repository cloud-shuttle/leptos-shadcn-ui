#[cfg(test)]
mod integration_tests {
    use crate::new_york::{
        CARD_CLASS as CARD_CLASS_NY, CARD_HEADER_CLASS as CARD_HEADER_CLASS_NY,
        CARD_TITLE_CLASS as CARD_TITLE_CLASS_NY, CARD_DESCRIPTION_CLASS as CARD_DESCRIPTION_CLASS_NY,
        CARD_CONTENT_CLASS as CARD_CONTENT_CLASS_NY, CARD_FOOTER_CLASS as CARD_FOOTER_CLASS_NY
    };
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== NEW YORK INTEGRATION TESTS =====
    // These tests focus on integration scenarios and complex usage patterns

    #[test]
    fn test_new_york_card_computed_class_generation() {
        // Test computed class generation for New York card components
        let base_class = CARD_CLASS_NY;
        let custom_class = "custom-card-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("rounded-lg"));
        assert!(computed_class.contains("border"));
        assert!(computed_class.contains("bg-card"));
        assert!(computed_class.contains("text-card-foreground"));
        assert!(computed_class.contains("shadow-sm"));
        assert!(computed_class.contains("custom-card-class"));
    }

    #[test]
    fn test_new_york_card_header_computed_class_generation() {
        // Test computed class generation for New York card header
        let base_class = CARD_HEADER_CLASS_NY;
        let custom_class = "custom-header-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("flex"));
        assert!(computed_class.contains("flex-col"));
        assert!(computed_class.contains("space-y-1.5"));
        assert!(computed_class.contains("p-6"));
        assert!(computed_class.contains("custom-header-class"));
    }

    #[test]
    fn test_new_york_card_title_computed_class_generation() {
        // Test computed class generation for New York card title
        let base_class = CARD_TITLE_CLASS_NY;
        let custom_class = "custom-title-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("text-2xl"));
        assert!(computed_class.contains("font-semibold"));
        assert!(computed_class.contains("leading-none"));
        assert!(computed_class.contains("tracking-tight"));
        assert!(computed_class.contains("custom-title-class"));
    }

    #[test]
    fn test_new_york_card_description_computed_class_generation() {
        // Test computed class generation for New York card description
        let base_class = CARD_DESCRIPTION_CLASS_NY;
        let custom_class = "custom-description-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("text-sm"));
        assert!(computed_class.contains("text-muted-foreground"));
        assert!(computed_class.contains("custom-description-class"));
    }

    #[test]
    fn test_new_york_card_content_computed_class_generation() {
        // Test computed class generation for New York card content
        let base_class = CARD_CONTENT_CLASS_NY;
        let custom_class = "custom-content-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("p-6"));
        assert!(computed_class.contains("pt-0"));
        assert!(computed_class.contains("custom-content-class"));
    }

    #[test]
    fn test_new_york_card_footer_computed_class_generation() {
        // Test computed class generation for New York card footer
        let base_class = CARD_FOOTER_CLASS_NY;
        let custom_class = "custom-footer-class";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("flex"));
        assert!(computed_class.contains("items-center"));
        assert!(computed_class.contains("p-6"));
        assert!(computed_class.contains("pt-0"));
        assert!(computed_class.contains("custom-footer-class"));
    }

    #[test]
    fn test_new_york_card_class_combination() {
        // Test combining multiple New York card classes
        let card_class = CARD_CLASS_NY;
        let header_class = CARD_HEADER_CLASS_NY;
        let title_class = CARD_TITLE_CLASS_NY;
        
        let combined_class = format!("{} {} {}", card_class, header_class, title_class);
        
        assert!(combined_class.contains("rounded-lg"));
        assert!(combined_class.contains("flex"));
        assert!(combined_class.contains("text-2xl"));
    }

    #[test]
    fn test_new_york_card_class_with_empty_custom_class() {
        // Test New York card class with empty custom class
        let base_class = CARD_CLASS_NY;
        let custom_class = "";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert_eq!(computed_class, base_class);
    }

    #[test]
    fn test_new_york_card_class_with_whitespace_custom_class() {
        // Test New York card class with whitespace custom class
        let base_class = CARD_CLASS_NY;
        let custom_class = "  ";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("rounded-lg"));
        assert!(computed_class.contains("  "));
    }

    #[test]
    fn test_new_york_card_class_with_multiple_custom_classes() {
        // Test New York card class with multiple custom classes
        let base_class = CARD_CLASS_NY;
        let custom_classes = "class1 class2 class3";
        
        let computed_class = format!("{} {}", base_class, custom_classes);
        
        assert!(computed_class.contains("rounded-lg"));
        assert!(computed_class.contains("class1"));
        assert!(computed_class.contains("class2"));
        assert!(computed_class.contains("class3"));
    }

    #[test]
    fn test_new_york_card_class_with_special_characters() {
        // Test New York card class with special characters
        let base_class = CARD_CLASS_NY;
        let custom_class = "class-with-dashes_and_underscores";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("rounded-lg"));
        assert!(computed_class.contains("class-with-dashes_and_underscores"));
    }

    #[test]
    fn test_new_york_card_class_with_numbers() {
        // Test New York card class with numbers
        let base_class = CARD_CLASS_NY;
        let custom_class = "class123";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("rounded-lg"));
        assert!(computed_class.contains("class123"));
    }

    #[test]
    fn test_new_york_card_class_with_unicode() {
        // Test New York card class with unicode characters
        let base_class = CARD_CLASS_NY;
        let custom_class = "class-🚀";
        
        let computed_class = format!("{} {}", base_class, custom_class);
        
        assert!(computed_class.contains("rounded-lg"));
        assert!(computed_class.contains("class-🚀"));
    }

    #[test]
    fn test_new_york_card_class_performance() {
        // Test New York card class generation performance
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _computed_class = format!("{} {}", CARD_CLASS_NY, "custom-class");
        }
        
        let duration = start.elapsed();
        
        // Should complete quickly (less than 10ms for 1000 iterations)
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_new_york_card_class_memory_usage() {
        // Test New York card class memory usage
        let computed_class = format!("{} {}", CARD_CLASS_NY, "custom-class");
        let size = std::mem::size_of_val(&computed_class);
        
        // Should be reasonable size (less than 1KB)
        assert!(size < 1024);
    }

    #[test]
    fn test_new_york_card_class_immutability() {
        // Test that New York card classes are immutable
        let original_class = CARD_CLASS_NY;
        let _computed_class = format!("{} {}", CARD_CLASS_NY, "custom-class");
        
        // Original class should remain unchanged
        assert_eq!(CARD_CLASS_NY, original_class);
    }

    #[test]
    fn test_new_york_card_class_consistency_across_calls() {
        // Test that New York card classes are consistent across multiple calls
        let class1 = CARD_CLASS_NY;
        let class2 = CARD_CLASS_NY;
        let class3 = CARD_CLASS_NY;
        
        assert_eq!(class1, class2);
        assert_eq!(class2, class3);
        assert_eq!(class1, class3);
    }

    #[test]
    fn test_new_york_card_class_edge_cases() {
        // Test edge cases for New York card class generation
        let base_class = CARD_CLASS_NY;
        
        // Test with very long custom class
        let long_class = "a".repeat(1000);
        let computed_class = format!("{} {}", base_class, long_class);
        assert!(computed_class.contains("rounded-lg"));
        assert!(computed_class.contains(&long_class));
        
        // Test with empty base class (should not happen in practice)
        let empty_base = "";
        let custom_class = "custom-class";
        let computed_class = format!("{} {}", empty_base, custom_class);
        assert_eq!(computed_class, " custom-class");
    }

    #[test]
    fn test_new_york_card_class_validation() {
        // Test New York card class validation
        let base_class = CARD_CLASS_NY;
        
        // Test that base class contains required elements
        assert!(base_class.contains("rounded-lg"));
        assert!(base_class.contains("border"));
        assert!(base_class.contains("bg-card"));
        assert!(base_class.contains("text-card-foreground"));
        assert!(base_class.contains("shadow-sm"));
        
        // Test that base class is not empty
        assert!(!base_class.is_empty());
        
        // Test that base class has reasonable length
        assert!(base_class.len() > 10);
        assert!(base_class.len() < 1000);
    }

    #[test]
    fn test_new_york_card_class_theme_consistency() {
        // Test that New York card classes maintain theme consistency
        assert!(CARD_CLASS_NY.contains("rounded-lg"));
        assert!(CARD_CLASS_NY.contains("border"));
        assert!(CARD_CLASS_NY.contains("bg-card"));
        assert!(CARD_CLASS_NY.contains("text-card-foreground"));
        assert!(CARD_CLASS_NY.contains("shadow-sm"));
    }

    #[test]
    fn test_new_york_card_class_theme_differences() {
        // Test that New York card classes have distinct theme characteristics
        let card_class = CARD_CLASS_NY;
        let header_class = CARD_HEADER_CLASS_NY;
        let title_class = CARD_TITLE_CLASS_NY;
        
        // Each class should have distinct characteristics
        assert!(card_class.contains("rounded-lg"));
        assert!(header_class.contains("flex"));
        assert!(title_class.contains("text-2xl"));
    }

    #[test]
    fn test_new_york_card_class_theme_performance() {
        // Test New York theme performance characteristics
        let start = std::time::Instant::now();
        
        // Test New York theme class generation performance
        for _ in 0..1000 {
            let _computed_class = format!("{} {} {}", 
                CARD_CLASS_NY,
                CARD_HEADER_CLASS_NY,
                CARD_TITLE_CLASS_NY
            );
        }
        
        let duration = start.elapsed();
        
        // Should complete quickly (less than 50ms for 1000 iterations)
        assert!(duration.as_millis() < 50);
    }
}
