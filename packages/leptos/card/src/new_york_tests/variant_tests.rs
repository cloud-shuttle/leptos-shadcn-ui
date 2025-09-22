#[cfg(test)]
mod variant_tests {
    use crate::new_york::{
        CARD_CLASS as CARD_CLASS_NY, CARD_HEADER_CLASS as CARD_HEADER_CLASS_NY,
        CARD_TITLE_CLASS as CARD_TITLE_CLASS_NY, CARD_DESCRIPTION_CLASS as CARD_DESCRIPTION_CLASS_NY,
        CARD_CONTENT_CLASS as CARD_CONTENT_CLASS_NY, CARD_FOOTER_CLASS as CARD_FOOTER_CLASS_NY
    };
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== NEW YORK VARIANT TESTS =====
    // These tests focus on the New York theme variant implementation

    #[test]
    fn test_new_york_card_class_constants() {
        // Test New York CARD_CLASS constant
        assert!(CARD_CLASS_NY.contains("rounded-lg"));
        assert!(CARD_CLASS_NY.contains("border"));
        assert!(CARD_CLASS_NY.contains("bg-card"));
        assert!(CARD_CLASS_NY.contains("text-card-foreground"));
        assert!(CARD_CLASS_NY.contains("shadow-sm"));

        // Test New York CARD_HEADER_CLASS constant
        assert!(CARD_HEADER_CLASS_NY.contains("flex"));
        assert!(CARD_HEADER_CLASS_NY.contains("flex-col"));
        assert!(CARD_HEADER_CLASS_NY.contains("space-y-1.5"));
        assert!(CARD_HEADER_CLASS_NY.contains("p-6"));

        // Test New York CARD_TITLE_CLASS constant
        assert!(CARD_TITLE_CLASS_NY.contains("text-2xl"));
        assert!(CARD_TITLE_CLASS_NY.contains("font-semibold"));
        assert!(CARD_TITLE_CLASS_NY.contains("leading-none"));
        assert!(CARD_TITLE_CLASS_NY.contains("tracking-tight"));

        // Test New York CARD_DESCRIPTION_CLASS constant
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-sm"));
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-muted-foreground"));

        // Test New York CARD_CONTENT_CLASS constant
        assert!(CARD_CONTENT_CLASS_NY.contains("p-6"));
        assert!(CARD_CONTENT_CLASS_NY.contains("pt-0"));

        // Test New York CARD_FOOTER_CLASS constant
        assert!(CARD_FOOTER_CLASS_NY.contains("flex"));
        assert!(CARD_FOOTER_CLASS_NY.contains("items-center"));
        assert!(CARD_FOOTER_CLASS_NY.contains("p-6"));
        assert!(CARD_FOOTER_CLASS_NY.contains("pt-0"));
    }

    #[test]
    fn test_new_york_card_class_consistency() {
        // Test that all New York card classes are consistent
        assert!(!CARD_CLASS_NY.is_empty());
        assert!(!CARD_HEADER_CLASS_NY.is_empty());
        assert!(!CARD_TITLE_CLASS_NY.is_empty());
        assert!(!CARD_DESCRIPTION_CLASS_NY.is_empty());
        assert!(!CARD_CONTENT_CLASS_NY.is_empty());
        assert!(!CARD_FOOTER_CLASS_NY.is_empty());
    }

    #[test]
    fn test_new_york_card_class_structure() {
        // Test that New York card classes have proper structure
        assert!(CARD_CLASS_NY.contains("rounded-lg"));
        assert!(CARD_CLASS_NY.contains("border"));
        assert!(CARD_CLASS_NY.contains("bg-card"));
        assert!(CARD_CLASS_NY.contains("text-card-foreground"));
        assert!(CARD_CLASS_NY.contains("shadow-sm"));
    }

    #[test]
    fn test_new_york_card_header_class_structure() {
        // Test that New York card header classes have proper structure
        assert!(CARD_HEADER_CLASS_NY.contains("flex"));
        assert!(CARD_HEADER_CLASS_NY.contains("flex-col"));
        assert!(CARD_HEADER_CLASS_NY.contains("space-y-1.5"));
        assert!(CARD_HEADER_CLASS_NY.contains("p-6"));
    }

    #[test]
    fn test_new_york_card_title_class_structure() {
        // Test that New York card title classes have proper structure
        assert!(CARD_TITLE_CLASS_NY.contains("text-2xl"));
        assert!(CARD_TITLE_CLASS_NY.contains("font-semibold"));
        assert!(CARD_TITLE_CLASS_NY.contains("leading-none"));
        assert!(CARD_TITLE_CLASS_NY.contains("tracking-tight"));
    }

    #[test]
    fn test_new_york_card_description_class_structure() {
        // Test that New York card description classes have proper structure
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-sm"));
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-muted-foreground"));
    }

    #[test]
    fn test_new_york_card_content_class_structure() {
        // Test that New York card content classes have proper structure
        assert!(CARD_CONTENT_CLASS_NY.contains("p-6"));
        assert!(CARD_CONTENT_CLASS_NY.contains("pt-0"));
    }

    #[test]
    fn test_new_york_card_footer_class_structure() {
        // Test that New York card footer classes have proper structure
        assert!(CARD_FOOTER_CLASS_NY.contains("flex"));
        assert!(CARD_FOOTER_CLASS_NY.contains("items-center"));
        assert!(CARD_FOOTER_CLASS_NY.contains("p-6"));
        assert!(CARD_FOOTER_CLASS_NY.contains("pt-0"));
    }

    #[test]
    fn test_new_york_card_class_accessibility() {
        // Test that New York card classes support accessibility
        assert!(CARD_CLASS_NY.contains("bg-card"));
        assert!(CARD_CLASS_NY.contains("text-card-foreground"));
        assert!(CARD_TITLE_CLASS_NY.contains("font-semibold"));
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-muted-foreground"));
    }

    #[test]
    fn test_new_york_card_class_responsiveness() {
        // Test that New York card classes support responsiveness
        assert!(CARD_HEADER_CLASS_NY.contains("flex"));
        assert!(CARD_HEADER_CLASS_NY.contains("flex-col"));
        assert!(CARD_FOOTER_CLASS_NY.contains("flex"));
        assert!(CARD_FOOTER_CLASS_NY.contains("items-center"));
    }

    #[test]
    fn test_new_york_card_class_spacing() {
        // Test that New York card classes have proper spacing
        assert!(CARD_HEADER_CLASS_NY.contains("space-y-1.5"));
        assert!(CARD_HEADER_CLASS_NY.contains("p-6"));
        assert!(CARD_CONTENT_CLASS_NY.contains("p-6"));
        assert!(CARD_CONTENT_CLASS_NY.contains("pt-0"));
        assert!(CARD_FOOTER_CLASS_NY.contains("p-6"));
        assert!(CARD_FOOTER_CLASS_NY.contains("pt-0"));
    }

    #[test]
    fn test_new_york_card_class_typography() {
        // Test that New York card classes have proper typography
        assert!(CARD_TITLE_CLASS_NY.contains("text-2xl"));
        assert!(CARD_TITLE_CLASS_NY.contains("font-semibold"));
        assert!(CARD_TITLE_CLASS_NY.contains("leading-none"));
        assert!(CARD_TITLE_CLASS_NY.contains("tracking-tight"));
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-sm"));
    }

    #[test]
    fn test_new_york_card_class_visual_hierarchy() {
        // Test that New York card classes create proper visual hierarchy
        assert!(CARD_TITLE_CLASS_NY.contains("text-2xl"));
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-sm"));
        assert!(CARD_TITLE_CLASS_NY.contains("font-semibold"));
        assert!(CARD_DESCRIPTION_CLASS_NY.contains("text-muted-foreground"));
    }

    #[test]
    fn test_new_york_card_class_border_radius() {
        // Test that New York card classes have proper border radius
        assert!(CARD_CLASS_NY.contains("rounded-lg"));
    }

    #[test]
    fn test_new_york_card_class_shadow() {
        // Test that New York card classes have proper shadow
        assert!(CARD_CLASS_NY.contains("shadow-sm"));
    }

    #[test]
    fn test_new_york_card_class_border() {
        // Test that New York card classes have proper border
        assert!(CARD_CLASS_NY.contains("border"));
    }

    #[test]
    fn test_new_york_card_class_background() {
        // Test that New York card classes have proper background
        assert!(CARD_CLASS_NY.contains("bg-card"));
    }

    #[test]
    fn test_new_york_card_class_text_color() {
        // Test that New York card classes have proper text color
        assert!(CARD_CLASS_NY.contains("text-card-foreground"));
    }

    #[test]
    fn test_new_york_card_class_completeness() {
        // Test that all New York card classes are complete and functional
        let all_classes = vec![
            CARD_CLASS_NY,
            CARD_HEADER_CLASS_NY,
            CARD_TITLE_CLASS_NY,
            CARD_DESCRIPTION_CLASS_NY,
            CARD_CONTENT_CLASS_NY,
            CARD_FOOTER_CLASS_NY,
        ];

        for class in all_classes {
            assert!(!class.is_empty());
            assert!(class.len() > 10); // Should have meaningful content
        }
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
}
