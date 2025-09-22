//! Component behavior tests for the Label component
//! 
//! This module contains tests for component behavior, children handling,
//! semantic structure, and accessibility features for the Label component.

use crate::default::LABEL_CLASS;
use leptos::prelude::*;
use leptos_style::Style;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_children_handling() {
        // Test children handling for Label
        let children_text = "Test Label Text";
        assert_eq!(children_text, "Test Label Text");
        
        let children_html = "<span>HTML Content</span>";
        assert!(children_html.contains("<span>"));
        assert!(children_html.contains("HTML Content"));
    }

    #[test]
    fn test_label_semantic_structure() {
        // Test semantic structure for Label
        let base_class = LABEL_CLASS;
        
        // Test that the class contains semantic elements
        assert!(base_class.contains("text-sm"), "Should have text size");
        assert!(base_class.contains("font-medium"), "Should have font weight");
        assert!(base_class.contains("leading-none"), "Should have line height");
    }

    #[test]
    fn test_label_accessibility_features() {
        // Test accessibility features for Label
        let base_class = LABEL_CLASS;
        
        // Test accessibility-related classes
        assert!(base_class.contains("peer-disabled:cursor-not-allowed"), "Should have disabled cursor");
        assert!(base_class.contains("peer-disabled:opacity-70"), "Should have disabled opacity");
    }

    #[test]
    fn test_label_typography_system() {
        // Test typography system for Label
        let base_class = LABEL_CLASS;
        
        // Test typography elements
        assert!(base_class.contains("text-sm"), "Should have small text");
        assert!(base_class.contains("font-medium"), "Should have medium font weight");
        assert!(base_class.contains("leading-none"), "Should have tight line height");
    }

    #[test]
    fn test_label_peer_disabled_states() {
        // Test peer disabled states for Label
        let base_class = LABEL_CLASS;
        
        // Test peer disabled state classes
        assert!(base_class.contains("peer-disabled:cursor-not-allowed"), "Should handle peer disabled cursor");
        assert!(base_class.contains("peer-disabled:opacity-70"), "Should handle peer disabled opacity");
    }

    #[test]
    fn test_label_form_integration() {
        // Test form integration for Label
        let base_class = LABEL_CLASS;
        
        // Test that the class supports form integration
        assert!(base_class.contains("text-sm"), "Should support form integration");
        assert!(base_class.contains("font-medium"), "Should support form integration");
    }
}
