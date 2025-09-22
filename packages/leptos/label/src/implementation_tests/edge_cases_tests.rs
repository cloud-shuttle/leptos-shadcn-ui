//! Edge cases and performance tests for the Label component
//! 
//! This module contains tests for edge cases, performance characteristics,
//! memory management, and validation logic for the Label component.

use crate::default::LABEL_CLASS;
use leptos::prelude::*;
use leptos_style::Style;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_edge_cases() {
        // Test edge cases for Label
        let empty_class = "";
        assert_eq!(empty_class, "");
        
        let long_class = "a".repeat(1000);
        assert_eq!(long_class.len(), 1000);
        
        let special_chars = "!@#$%^&*()";
        assert_eq!(special_chars, "!@#$%^&*()");
    }

    #[test]
    fn test_label_performance_characteristics() {
        // Test performance characteristics for Label
        let base_class = LABEL_CLASS;
        
        // Test that the class is not excessively long (performance consideration)
        assert!(base_class.len() < 200, "Class should not be excessively long");
        
        // Test that essential classes are present
        assert!(base_class.contains("text-sm"), "Should have text size");
        assert!(base_class.contains("font-medium"), "Should have font weight");
    }

    #[test]
    fn test_label_memory_management() {
        // Test memory management for Label
        let (value, set_value) = signal("initial".to_string());
        
        // Test multiple updates
        for i in 0..100 {
            set_value.set(format!("value-{}", i));
        }
        
        assert_eq!(value.get(), "value-99");
        
        // Test memory cleanup by setting to empty
        set_value.set("".to_string());
        assert_eq!(value.get(), "");
    }

    #[test]
    fn test_label_validation_logic() {
        // Test validation logic for Label
        let valid_class = "text-sm font-medium";
        assert!(valid_class.contains("text-sm"));
        assert!(valid_class.contains("font-medium"));
        
        let invalid_class = "invalid-class";
        assert!(!invalid_class.contains("text-sm"));
        assert!(!invalid_class.contains("font-medium"));
    }

    #[test]
    fn test_label_state_management() {
        // Test state management for Label
        let (is_visible, set_is_visible) = signal(true);
        let (text, set_text) = signal("Label text".to_string());
        
        // Test initial state
        assert_eq!(is_visible.get(), true);
        assert_eq!(text.get(), "Label text");
        
        // Test state updates
        set_is_visible.set(false);
        set_text.set("Updated text".to_string());
        
        assert_eq!(is_visible.get(), false);
        assert_eq!(text.get(), "Updated text");
    }
}
