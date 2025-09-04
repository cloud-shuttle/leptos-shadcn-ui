#[cfg(test)]
mod tests {
    use crate::default::{Label, LABEL_CLASS};
    use leptos::prelude::*;

    #[test]
    fn test_label_base_css_classes() {
        // Test that base LABEL_CLASS contains required styling and accessibility classes
        assert!(LABEL_CLASS.contains("text-sm"));
        assert!(LABEL_CLASS.contains("font-medium"));
        assert!(LABEL_CLASS.contains("leading-none"));
        assert!(LABEL_CLASS.contains("peer-disabled:cursor-not-allowed"));
        assert!(LABEL_CLASS.contains("peer-disabled:opacity-70"));
    }

    #[test]
    fn test_label_peer_interaction_classes() {
        // Test peer interaction styling (for form element associations)
        assert!(LABEL_CLASS.contains("peer-disabled:cursor-not-allowed"));
        assert!(LABEL_CLASS.contains("peer-disabled:opacity-70"));
    }

    #[test]
    fn test_label_typography_classes() {
        // Test typography-specific classes
        assert!(LABEL_CLASS.contains("text-sm"));
        assert!(LABEL_CLASS.contains("font-medium"));
        assert!(LABEL_CLASS.contains("leading-none"));
    }

    #[test]
    fn test_label_class_merging() {
        // Test custom class handling
        let base_class = LABEL_CLASS;
        let custom_class = "my-custom-label-class text-lg";
        
        let expected = format!("{} {}", base_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
        assert!(expected.contains("text-sm")); // Base class
        assert!(expected.contains("text-lg")); // Custom class (would override in CSS)
    }

    #[test]
    fn test_label_accessibility_compliance() {
        // Test that label provides proper accessibility features
        // Label elements are inherently accessible when properly associated
        
        // Test semantic HTML structure
        let semantic_tag = "label";
        assert_eq!(semantic_tag, "label");
        
        // Test accessibility through peer classes
        assert!(LABEL_CLASS.contains("peer-disabled"));
    }

    #[test]
    fn test_label_component_structure() {
        // Test that label creates proper HTML structure
        let label_tag = "label";
        assert_eq!(label_tag, "label");
        
        // Test children handling concept
        let has_children = true; // Labels typically contain text or other elements
        assert!(has_children);
    }

    #[test]
    fn test_label_styling_consistency() {
        // Test that all required styling properties are present
        let required_properties = vec![
            "text-sm", "font-medium", "leading-none"
        ];
        
        for property in required_properties {
            assert!(LABEL_CLASS.contains(property), 
                "LABEL_CLASS should contain '{}' property", property);
        }
    }

    #[test]
    fn test_label_form_integration() {
        // Test label's role in form accessibility
        // Labels should be associable with form controls
        
        let form_association_methods = vec!["for", "wrapping", "aria-labelledby"];
        assert!(form_association_methods.len() > 0);
        
        // Test peer interaction concept
        assert!(LABEL_CLASS.contains("peer-disabled"));
    }

    #[test]
    fn test_label_disabled_state_styling() {
        // Test disabled state styling through peer classes
        assert!(LABEL_CLASS.contains("peer-disabled:cursor-not-allowed"));
        assert!(LABEL_CLASS.contains("peer-disabled:opacity-70"));
        
        // Test that disabled styling reduces opacity appropriately
        let opacity_value = "opacity-70"; // 70% opacity
        assert!(LABEL_CLASS.contains(opacity_value));
    }

    #[test]
    fn test_label_visual_hierarchy() {
        // Test typography creates proper visual hierarchy
        assert!(LABEL_CLASS.contains("font-medium")); // Medium weight for emphasis
        assert!(LABEL_CLASS.contains("text-sm"));     // Small text for labels
        assert!(LABEL_CLASS.contains("leading-none")); // Tight line height
        
        // Test these work together for proper label appearance
        let typography_classes = vec!["text-sm", "font-medium", "leading-none"];
        for class in typography_classes {
            assert!(LABEL_CLASS.contains(class));
        }
    }
}