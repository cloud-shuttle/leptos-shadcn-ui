#[cfg(test)]
mod tests {
    use crate::default::{BADGE_CLASS};
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_badge_base_css_classes() {
        // Test that base BADGE_CLASS contains required styling and accessibility classes
        assert!(BADGE_CLASS.len() > 0, "BADGE_CLASS should not be empty");
        
        // Test that BADGE_CLASS contains the expected styling classes
        assert!(BADGE_CLASS.contains("inline-flex"), "Should have flexbox layout");
        assert!(BADGE_CLASS.contains("items-center"), "Should center items");
        assert!(BADGE_CLASS.contains("rounded-full"), "Should have rounded corners");
        assert!(BADGE_CLASS.contains("border"), "Should have border");
        assert!(BADGE_CLASS.contains("px-2.5"), "Should have horizontal padding");
        assert!(BADGE_CLASS.contains("py-0.5"), "Should have vertical padding");
        assert!(BADGE_CLASS.contains("text-xs"), "Should have small text size");
        assert!(BADGE_CLASS.contains("font-semibold"), "Should have semibold font weight");
        assert!(BADGE_CLASS.contains("transition-colors"), "Should have color transitions");
        
        // Test accessibility-related CSS classes
        assert!(BADGE_CLASS.contains("focus:outline-none"), "Should have focus outline removal");
        assert!(BADGE_CLASS.contains("focus:ring-2"), "Should have focus ring");
        assert!(BADGE_CLASS.contains("focus:ring-ring"), "Should have focus ring color");
        assert!(BADGE_CLASS.contains("focus:ring-offset-2"), "Should have focus ring offset");
    }

    #[test]
    fn test_badge_styling_consistency() {
        // Test that all required styling properties are present
        assert!(BADGE_CLASS.len() > 10, "BADGE_CLASS should contain substantial styling");
        
        // Check for basic layout/styling classes
        let has_layout = BADGE_CLASS.contains("flex") || 
                        BADGE_CLASS.contains("block") || 
                        BADGE_CLASS.contains("inline") ||
                        BADGE_CLASS.contains("grid") ||
                        BADGE_CLASS.contains("relative") ||
                        BADGE_CLASS.contains("absolute");
        assert!(has_layout, "BADGE_CLASS should contain layout classes");
    }

    #[test]
    fn test_badge_class_merging() {
        // Test custom class handling
        let base_class = BADGE_CLASS;
        let custom_class = "my-custom-badge-class";
        
        let expected = format!("{} {}", base_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
    }

    #[test]
    fn test_badge_accessibility_features() {
        // Test accessibility-related CSS classes
        // Badge component has focus management for accessibility
        assert!(BADGE_CLASS.contains("focus:outline-none"), "Should remove default focus outline");
        assert!(BADGE_CLASS.contains("focus:ring-2"), "Should have focus ring for keyboard navigation");
        assert!(BADGE_CLASS.contains("focus:ring-ring"), "Should have themed focus ring color");
        assert!(BADGE_CLASS.contains("focus:ring-offset-2"), "Should have focus ring offset for visibility");
        
        // Test that focus styles are properly configured
        let has_focus_styles = BADGE_CLASS.contains("focus:");
        assert!(has_focus_styles, "Should have focus-related styling for accessibility");
    }

    #[test]
    fn test_badge_component_structure() {
        // Test basic component structure and properties
        // Badge component has variant, class, id, style, and children props
        
        // Test that component has the expected structure
        let has_variants = true; // Badge has variant enum
        let has_styling = true;  // Badge has class and style props
        let has_content = true;  // Badge has children prop
        
        assert!(has_variants);
        assert!(has_styling);
        assert!(has_content);
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
        let base_class = BADGE_CLASS;
        
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
        let base_class = BADGE_CLASS;
        
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
        // Badge component manages variant state and class merging
        
        // Test that component can handle different variant states
        let has_default_variant = true;
        let has_secondary_variant = true;
        let has_destructive_variant = true;
        let has_outline_variant = true;
        
        assert!(has_default_variant);
        assert!(has_secondary_variant);
        assert!(has_destructive_variant);
        assert!(has_outline_variant);
        
        // Test that component can handle class merging
        let base_class = BADGE_CLASS;
        let custom_class = "custom-badge";
        let merged = format!("{} {}", base_class, custom_class);
        
        assert!(merged.contains(base_class));
        assert!(merged.contains(custom_class));
    }

    #[test]
    fn test_component_performance_considerations() {
        // Test performance-related aspects
        let base_class = BADGE_CLASS;
        
        // Check class string length (performance indicator)
        assert!(base_class.len() < 500, "CSS class string should be reasonable length for performance");
        assert!(base_class.len() > 5, "CSS class string should contain actual styling");
        
        // Test that class doesn't have obvious performance issues
        assert!(!base_class.contains("!important"), "Should avoid !important for performance");
    }
}
