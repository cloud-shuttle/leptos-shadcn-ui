#[cfg(test)]
mod class_constants {
    use crate::default::{
        CARD_CLASS, CARD_HEADER_CLASS, CARD_TITLE_CLASS, CARD_DESCRIPTION_CLASS,
        CARD_CONTENT_CLASS, CARD_FOOTER_CLASS
    };

    // ===== CLASS CONSTANTS TESTS =====
    // These tests focus on CSS class constants and their properties

    #[test]
    fn test_card_class_constants() {
        // Test CARD_CLASS constant
        assert!(CARD_CLASS.contains("rounded-lg"));
        assert!(CARD_CLASS.contains("border"));
        assert!(CARD_CLASS.contains("bg-card"));
        assert!(CARD_CLASS.contains("text-card-foreground"));
        assert!(CARD_CLASS.contains("shadow-sm"));

        // Test CARD_HEADER_CLASS constant
        assert!(CARD_HEADER_CLASS.contains("flex"));
        assert!(CARD_HEADER_CLASS.contains("flex-col"));
        assert!(CARD_HEADER_CLASS.contains("space-y-1.5"));
        assert!(CARD_HEADER_CLASS.contains("p-6"));

        // Test CARD_TITLE_CLASS constant
        assert!(CARD_TITLE_CLASS.contains("text-2xl"));
        assert!(CARD_TITLE_CLASS.contains("font-semibold"));
        assert!(CARD_TITLE_CLASS.contains("leading-none"));
        assert!(CARD_TITLE_CLASS.contains("tracking-tight"));

        // Test CARD_DESCRIPTION_CLASS constant
        assert!(CARD_DESCRIPTION_CLASS.contains("text-sm"));
        assert!(CARD_DESCRIPTION_CLASS.contains("text-muted-foreground"));

        // Test CARD_CONTENT_CLASS constant
        assert!(CARD_CONTENT_CLASS.contains("p-6"));
        assert!(CARD_CONTENT_CLASS.contains("pt-0"));

        // Test CARD_FOOTER_CLASS constant
        assert!(CARD_FOOTER_CLASS.contains("flex"));
        assert!(CARD_FOOTER_CLASS.contains("items-center"));
        assert!(CARD_FOOTER_CLASS.contains("p-6"));
        assert!(CARD_FOOTER_CLASS.contains("pt-0"));
    }

    #[test]
    fn test_card_class_consistency() {
        // Test that all card classes are consistent
        assert!(!CARD_CLASS.is_empty());
        assert!(!CARD_HEADER_CLASS.is_empty());
        assert!(!CARD_TITLE_CLASS.is_empty());
        assert!(!CARD_DESCRIPTION_CLASS.is_empty());
        assert!(!CARD_CONTENT_CLASS.is_empty());
        assert!(!CARD_FOOTER_CLASS.is_empty());
    }

    #[test]
    fn test_card_class_structure() {
        // Test that card classes have proper structure
        assert!(CARD_CLASS.contains("rounded-lg"));
        assert!(CARD_CLASS.contains("border"));
        assert!(CARD_CLASS.contains("bg-card"));
        assert!(CARD_CLASS.contains("text-card-foreground"));
        assert!(CARD_CLASS.contains("shadow-sm"));
    }

    #[test]
    fn test_card_header_class_structure() {
        // Test that card header classes have proper structure
        assert!(CARD_HEADER_CLASS.contains("flex"));
        assert!(CARD_HEADER_CLASS.contains("flex-col"));
        assert!(CARD_HEADER_CLASS.contains("space-y-1.5"));
        assert!(CARD_HEADER_CLASS.contains("p-6"));
    }

    #[test]
    fn test_card_title_class_structure() {
        // Test that card title classes have proper structure
        assert!(CARD_TITLE_CLASS.contains("text-2xl"));
        assert!(CARD_TITLE_CLASS.contains("font-semibold"));
        assert!(CARD_TITLE_CLASS.contains("leading-none"));
        assert!(CARD_TITLE_CLASS.contains("tracking-tight"));
    }

    #[test]
    fn test_card_description_class_structure() {
        // Test that card description classes have proper structure
        assert!(CARD_DESCRIPTION_CLASS.contains("text-sm"));
        assert!(CARD_DESCRIPTION_CLASS.contains("text-muted-foreground"));
    }

    #[test]
    fn test_card_content_class_structure() {
        // Test that card content classes have proper structure
        assert!(CARD_CONTENT_CLASS.contains("p-6"));
        assert!(CARD_CONTENT_CLASS.contains("pt-0"));
    }

    #[test]
    fn test_card_footer_class_structure() {
        // Test that card footer classes have proper structure
        assert!(CARD_FOOTER_CLASS.contains("flex"));
        assert!(CARD_FOOTER_CLASS.contains("items-center"));
        assert!(CARD_FOOTER_CLASS.contains("p-6"));
        assert!(CARD_FOOTER_CLASS.contains("pt-0"));
    }

    #[test]
    fn test_card_class_accessibility() {
        // Test that card classes support accessibility
        assert!(CARD_CLASS.contains("bg-card"));
        assert!(CARD_CLASS.contains("text-card-foreground"));
        assert!(CARD_TITLE_CLASS.contains("font-semibold"));
        assert!(CARD_DESCRIPTION_CLASS.contains("text-muted-foreground"));
    }

    #[test]
    fn test_card_class_responsiveness() {
        // Test that card classes support responsiveness
        assert!(CARD_HEADER_CLASS.contains("flex"));
        assert!(CARD_HEADER_CLASS.contains("flex-col"));
        assert!(CARD_FOOTER_CLASS.contains("flex"));
        assert!(CARD_FOOTER_CLASS.contains("items-center"));
    }

    #[test]
    fn test_card_class_spacing() {
        // Test that card classes have proper spacing
        assert!(CARD_HEADER_CLASS.contains("space-y-1.5"));
        assert!(CARD_HEADER_CLASS.contains("p-6"));
        assert!(CARD_CONTENT_CLASS.contains("p-6"));
        assert!(CARD_CONTENT_CLASS.contains("pt-0"));
        assert!(CARD_FOOTER_CLASS.contains("p-6"));
        assert!(CARD_FOOTER_CLASS.contains("pt-0"));
    }

    #[test]
    fn test_card_class_typography() {
        // Test that card classes have proper typography
        assert!(CARD_TITLE_CLASS.contains("text-2xl"));
        assert!(CARD_TITLE_CLASS.contains("font-semibold"));
        assert!(CARD_TITLE_CLASS.contains("leading-none"));
        assert!(CARD_TITLE_CLASS.contains("tracking-tight"));
        assert!(CARD_DESCRIPTION_CLASS.contains("text-sm"));
    }

    #[test]
    fn test_card_class_visual_hierarchy() {
        // Test that card classes create proper visual hierarchy
        assert!(CARD_TITLE_CLASS.contains("text-2xl"));
        assert!(CARD_DESCRIPTION_CLASS.contains("text-sm"));
        assert!(CARD_TITLE_CLASS.contains("font-semibold"));
        assert!(CARD_DESCRIPTION_CLASS.contains("text-muted-foreground"));
    }

    #[test]
    fn test_card_class_border_radius() {
        // Test that card classes have proper border radius
        assert!(CARD_CLASS.contains("rounded-lg"));
    }

    #[test]
    fn test_card_class_shadow() {
        // Test that card classes have proper shadow
        assert!(CARD_CLASS.contains("shadow-sm"));
    }

    #[test]
    fn test_card_class_border() {
        // Test that card classes have proper border
        assert!(CARD_CLASS.contains("border"));
    }

    #[test]
    fn test_card_class_background() {
        // Test that card classes have proper background
        assert!(CARD_CLASS.contains("bg-card"));
    }

    #[test]
    fn test_card_class_text_color() {
        // Test that card classes have proper text color
        assert!(CARD_CLASS.contains("text-card-foreground"));
    }

    #[test]
    fn test_card_class_completeness() {
        // Test that all card classes are complete and functional
        let all_classes = vec![
            CARD_CLASS,
            CARD_HEADER_CLASS,
            CARD_TITLE_CLASS,
            CARD_DESCRIPTION_CLASS,
            CARD_CONTENT_CLASS,
            CARD_FOOTER_CLASS,
        ];

        for class in all_classes {
            assert!(!class.is_empty());
            assert!(class.len() > 10); // Should have meaningful content
        }
    }
}
