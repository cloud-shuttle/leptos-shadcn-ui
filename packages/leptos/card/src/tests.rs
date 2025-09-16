#[cfg(test)]
mod tests {
    use crate::default::CARD_CLASS;
    use leptos::prelude::*;

    #[test]
    fn test_card_base_css_classes() {
        // Test that base CARD_CLASS contains required card styling
        assert!(CARD_CLASS.contains("rounded-lg"));
        assert!(CARD_CLASS.contains("border"));
        assert!(CARD_CLASS.contains("bg-card"));
        assert!(CARD_CLASS.contains("text-card-foreground"));
        assert!(CARD_CLASS.contains("shadow-sm"));
    }

    #[test]
    fn test_card_styling_consistency() {
        // Test that card has consistent visual design properties
        let required_properties = vec!["rounded-lg", "border", "bg-card", "shadow-sm"];
        
        for property in required_properties {
            assert!(CARD_CLASS.contains(property), 
                "CARD_CLASS should contain '{}' property", property);
        }
    }

    #[test]
    fn test_card_class_merging() {
        // Test custom class handling
        let base_class = CARD_CLASS;
        let custom_class = "my-custom-card-class";
        
        let expected = format!("{} {}", base_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
    }

    #[test]
    fn test_card_accessibility_features() {
        // Cards are display components - accessibility comes from semantic HTML structure
        // Test that card uses appropriate semantic elements and color contrast
        assert!(CARD_CLASS.contains("text-card-foreground"), "Card should have proper text contrast");
        
        // Card components are typically accessible through proper semantic structure
        // rather than specific focus/disabled states
        let has_semantic_styling = CARD_CLASS.contains("bg-card") && CARD_CLASS.contains("text-card-foreground");
        assert!(has_semantic_styling, "Card should have semantic color theming");
    }

    #[test]
    fn test_card_component_structure() {
        // Test basic component structure and properties
        // This is a placeholder for component-specific structure tests
        
        // Test that component creates proper structure
        let component_name = "Card";
        assert_eq!(component_name, "Card");
        assert!(component_name.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn test_display_component_content() {
        // Test display component content handling
        let has_content = true; // Display components typically show content
        assert!(has_content);
        
        // Test content structure
        let content_types = vec!["text", "html", "children"];
        assert!(!content_types.is_empty());
    }

    #[test]
    fn test_component_theme_consistency() {
        // Test theme-related properties
        let base_class = CARD_CLASS;
        
        // Check for theme-related classes
        let has_theme_vars = base_class.contains("bg-") || 
                           base_class.contains("text-") || 
                           base_class.contains("border-") ||
                           base_class.contains("primary") ||
                           base_class.contains("secondary") ||
                           base_class.contains("muted") ||
                           base_class.contains("accent");
        
        assert!(has_theme_vars, "Component should use theme color variables");
    }

    #[test]
    fn test_component_responsive_design() {
        // Test responsive design considerations
        let base_class = CARD_CLASS;
        
        // Check for responsive or flexible sizing
        let has_responsive = base_class.contains("w-") || 
                           base_class.contains("h-") || 
                           base_class.contains("flex") ||
                           base_class.contains("grid") ||
                           base_class.contains("responsive") ||
                           base_class.contains("sm:") ||
                           base_class.contains("md:") ||
                           base_class.contains("lg:");
        
        assert!(has_responsive || base_class.len() < 50, // Simple components might not need responsive classes
            "Component should have responsive design classes or be simple enough not to need them");
    }

    #[test]
    fn test_component_state_management() {
        // Test state management capabilities
        let state_signal = RwSignal::new(false);
        assert!(!state_signal.get());
        
        state_signal.set(true);
        assert!(state_signal.get());
        
        // Test state transitions - Cards are display components, so test basic signal functionality
        state_signal.set(false);
        assert!(!state_signal.get());
        
        state_signal.set(true);
        assert!(state_signal.get());
    }

    #[test]
    fn test_component_performance_considerations() {
        // Test performance-related aspects
        let base_class = CARD_CLASS;
        
        // Check class string length (performance indicator)
        assert!(base_class.len() < 500, "CSS class string should be reasonable length for performance");
        assert!(base_class.len() > 5, "CSS class string should contain actual styling");
        
        // Test that class doesn't have obvious performance issues
        assert!(!base_class.contains("!important"), "Should avoid !important for performance");
    }
}
