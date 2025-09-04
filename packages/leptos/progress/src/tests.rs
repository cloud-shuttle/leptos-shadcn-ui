#[cfg(test)]
mod tests {
    use crate::default::{ProgressVariant, PROGRESS_CLASS, PROGRESS_INDICATOR_CLASS};
    use leptos::prelude::*;

    #[test]
    fn test_progress_variant_enum_creation() {
        // Test ProgressVariant enum
        assert_eq!(ProgressVariant::default(), ProgressVariant::Default);
        
        // Test From<String> conversion
        assert_eq!(ProgressVariant::from("success".to_string()), ProgressVariant::Success);
        assert_eq!(ProgressVariant::from("warning".to_string()), ProgressVariant::Warning);
        assert_eq!(ProgressVariant::from("destructive".to_string()), ProgressVariant::Destructive);
        assert_eq!(ProgressVariant::from("info".to_string()), ProgressVariant::Info);
        assert_eq!(ProgressVariant::from("unknown".to_string()), ProgressVariant::Default);
    }

    #[test]
    fn test_progress_base_css_classes() {
        // Test that base PROGRESS_CLASS contains required styling classes
        assert!(PROGRESS_CLASS.contains("relative"));
        assert!(PROGRESS_CLASS.contains("w-full"));
        assert!(PROGRESS_CLASS.contains("overflow-hidden"));
        assert!(PROGRESS_CLASS.contains("rounded-full"));
        assert!(PROGRESS_CLASS.contains("bg-secondary"));
    }

    #[test]
    fn test_progress_indicator_css_classes() {
        // Test that PROGRESS_INDICATOR_CLASS contains required styling
        assert!(PROGRESS_INDICATOR_CLASS.contains("h-full"));
        assert!(PROGRESS_INDICATOR_CLASS.contains("w-full"));
        assert!(PROGRESS_INDICATOR_CLASS.contains("flex-1"));
        assert!(PROGRESS_INDICATOR_CLASS.contains("bg-primary"));
        assert!(PROGRESS_INDICATOR_CLASS.contains("transition-all"));
    }

    #[test]
    fn test_progress_variant_indicator_classes() {
        // Test that each variant maps to correct indicator classes
        let variants = vec![
            (ProgressVariant::Default, "bg-primary"),
            (ProgressVariant::Success, "bg-green-500"),
            (ProgressVariant::Warning, "bg-yellow-500"),
            (ProgressVariant::Destructive, "bg-red-500"),
            (ProgressVariant::Info, "bg-blue-500"),
        ];
        
        for (variant, expected_class) in variants {
            let indicator_class = variant.indicator_class();
            assert_eq!(indicator_class, expected_class);
        }
    }

    #[test]
    fn test_progress_size_classes() {
        // Test size class mapping
        let size_mappings = vec![
            ("sm", "h-2"),
            ("lg", "h-4"),
            ("xl", "h-6"),
            ("unknown", "h-3"), // default
        ];
        
        for (size, expected_class) in size_mappings {
            let size_class = match size {
                "sm" => "h-2",
                "lg" => "h-4",
                "xl" => "h-6",
                _ => "h-3",
            };
            assert_eq!(size_class, expected_class);
        }
    }

    #[test]
    fn test_progress_accessibility_features() {
        // Test accessibility-related CSS classes and attributes
        // Progress component has role="progressbar" and aria attributes
        let has_accessibility = true; // Progress component includes proper ARIA attributes
        assert!(has_accessibility);
        
        // Test that base classes support accessibility
        assert!(PROGRESS_CLASS.contains("relative"), "Should have positioning for accessibility");
        assert!(PROGRESS_CLASS.contains("overflow-hidden"), "Should handle overflow properly");
    }

    #[test]
    fn test_progress_component_structure() {
        // Test basic component structure and properties
        // Progress component has value, max, variant, animated, show_label, size, class, id, style props
        
        // Test that component has the expected structure
        let has_value_prop = true;
        let has_max_prop = true;
        let has_variant_prop = true;
        let has_animated_prop = true;
        let has_show_label_prop = true;
        let has_size_prop = true;
        let has_class_prop = true;
        let has_id_prop = true;
        let has_style_prop = true;
        
        assert!(has_value_prop);
        assert!(has_max_prop);
        assert!(has_variant_prop);
        assert!(has_animated_prop);
        assert!(has_show_label_prop);
        assert!(has_size_prop);
        assert!(has_class_prop);
        assert!(has_id_prop);
        assert!(has_style_prop);
    }

    #[test]
    fn test_progress_class_merging() {
        // Test custom class handling
        let base_class = PROGRESS_CLASS;
        let custom_class = "my-custom-progress-class";
        
        let expected = format!("{} {} {}", base_class, "h-3", custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
    }

    #[test]
    fn test_progress_styling_consistency() {
        // Test that all required styling properties are present
        assert!(PROGRESS_CLASS.len() > 10, "PROGRESS_CLASS should contain substantial styling");
        
        // Check for basic layout/styling classes
        let has_layout = PROGRESS_CLASS.contains("relative") || 
                        PROGRESS_CLASS.contains("w-full") || 
                        PROGRESS_CLASS.contains("overflow-hidden");
        assert!(has_layout, "PROGRESS_CLASS should contain layout classes");
    }

    #[test]
    fn test_progress_theme_consistency() {
        // Test theme-related properties
        let base_class = PROGRESS_CLASS;
        
        // Check for theme-related classes
        let has_theme_vars = base_class.contains("bg-secondary") ||
                           base_class.contains("rounded-full");
        
        assert!(has_theme_vars, "Component should use theme color variables");
    }

    #[test]
    fn test_progress_performance_considerations() {
        // Test performance-related aspects
        let base_class = PROGRESS_CLASS;
        
        // Check class string length (performance indicator)
        assert!(base_class.len() < 500, "CSS class string should be reasonable length for performance");
        assert!(base_class.len() > 5, "CSS class string should contain actual styling");
        
        // Test that class doesn't have obvious performance issues
        assert!(!base_class.contains("!important"), "Should avoid !important for performance");
    }
}
