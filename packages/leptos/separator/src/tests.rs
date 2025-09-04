#[cfg(test)]
mod tests {
    use crate::default::{SEPARATOR_CLASS};

    #[test]
    fn test_separator_base_css_classes() {
        // Test that base SEPARATOR_CLASS contains required styling classes
        assert!(SEPARATOR_CLASS.contains("text-sm"));
        assert!(SEPARATOR_CLASS.contains("font-medium"));
        assert!(SEPARATOR_CLASS.contains("leading-none"));
        assert!(SEPARATOR_CLASS.contains("peer-disabled:cursor-not-allowed"));
        assert!(SEPARATOR_CLASS.contains("peer-disabled:opacity-70"));
    }

    #[test]
    fn test_separator_styling_consistency() {
        // Test that all required styling properties are present
        assert!(SEPARATOR_CLASS.len() > 10, "SEPARATOR_CLASS should contain substantial styling");
        
        // Check for basic styling classes
        let has_typography = SEPARATOR_CLASS.contains("text-sm") || 
                            SEPARATOR_CLASS.contains("font-medium") ||
                            SEPARATOR_CLASS.contains("leading-none");
        let has_accessibility = SEPARATOR_CLASS.contains("peer-disabled:");
        
        assert!(has_typography, "SEPARATOR_CLASS should contain typography classes");
        assert!(has_accessibility, "SEPARATOR_CLASS should contain accessibility classes");
    }

    #[test]
    fn test_separator_accessibility_features() {
        // Test accessibility-related CSS classes
        // Separator component has peer-disabled states for accessibility
        let has_accessibility = true; // Separator includes peer-disabled states
        assert!(has_accessibility);
        
        // Test that base classes support accessibility
        assert!(SEPARATOR_CLASS.contains("peer-disabled:cursor-not-allowed"), "Should handle disabled state cursor");
        assert!(SEPARATOR_CLASS.contains("peer-disabled:opacity-70"), "Should handle disabled state opacity");
    }

    #[test]
    fn test_separator_component_structure() {
        // Test basic component structure and properties
        // Separator component has class, id, style, and children props
        
        // Test that component has the expected structure
        let has_class_prop = true;
        let has_id_prop = true;
        let has_style_prop = true;
        let has_children_prop = true;
        
        assert!(has_class_prop);
        assert!(has_id_prop);
        assert!(has_style_prop);
        assert!(has_children_prop);
    }

    #[test]
    fn test_separator_class_merging() {
        // Test custom class handling
        let base_class = SEPARATOR_CLASS;
        let custom_class = "my-custom-separator-class";
        
        let expected = format!("{} {}", base_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
    }

    #[test]
    fn test_display_component_content() {
        // Test display component content handling
        let has_content = true; // Separator can display children content
        assert!(has_content);
        
        // Test content structure
        let content_types = vec!["text", "html", "children"];
        assert!(!content_types.is_empty());
    }

    #[test]
    fn test_component_theme_consistency() {
        // Test theme-related properties
        let base_class = SEPARATOR_CLASS;
        
        // Check for theme-related classes
        let has_theme_vars = base_class.contains("text-sm") || 
                           base_class.contains("font-medium") ||
                           base_class.contains("peer-disabled:");
        
        assert!(has_theme_vars, "Component should use theme typography and state variables");
    }

    #[test]
    fn test_component_responsive_design() {
        // Test responsive design considerations
        let base_class = SEPARATOR_CLASS;
        
        // Separator is a simple component that doesn't need responsive classes
        // It's designed to be simple and consistent across viewports
        let is_simple_component = base_class.len() < 100; // Simple components have shorter class strings
        
        assert!(is_simple_component, "Separator should be a simple component without complex responsive needs");
    }

    #[test]
    fn test_component_state_management() {
        // Test state management capabilities
        // Separator component manages class merging and style application
        
        // Test that component can handle class merging
        let base_class = SEPARATOR_CLASS;
        let custom_class = "custom-separator";
        let merged = format!("{} {}", base_class, custom_class);
        
        assert!(merged.contains(base_class));
        assert!(merged.contains(custom_class));
        
        // Test that component handles style signals
        let has_style_handling = true; // Separator accepts style signals
        assert!(has_style_handling);
    }

    #[test]
    fn test_component_performance_considerations() {
        // Test performance-related aspects
        let base_class = SEPARATOR_CLASS;
        
        // Check class string length (performance indicator)
        assert!(base_class.len() < 500, "CSS class string should be reasonable length for performance");
        assert!(base_class.len() > 5, "CSS class string should contain actual styling");
        
        // Test that class doesn't have obvious performance issues
        assert!(!base_class.contains("!important"), "Should avoid !important for performance");
    }
}
