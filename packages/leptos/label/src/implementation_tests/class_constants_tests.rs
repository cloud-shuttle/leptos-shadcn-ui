//! Class constants tests for the Label component
//! 
//! This module contains tests for CSS class constants, computed class generation,
//! and basic styling functionality for the Label component.

use crate::default::LABEL_CLASS;
use leptos::prelude::*;
use leptos_style::Style;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_class_constant() {
        // Test LABEL_CLASS constant
        assert!(LABEL_CLASS.contains("text-sm"));
        assert!(LABEL_CLASS.contains("font-medium"));
        assert!(LABEL_CLASS.contains("leading-none"));
        assert!(LABEL_CLASS.contains("peer-disabled:cursor-not-allowed"));
        assert!(LABEL_CLASS.contains("peer-disabled:opacity-70"));
    }

    #[test]
    fn test_label_computed_class_generation() {
        // Test Label computed class generation
        let base_class = LABEL_CLASS;
        let custom_class = "custom-label";
        let computed = format!("{} {}", base_class, custom_class);
        
        assert!(computed.contains("text-sm"));
        assert!(computed.contains("font-medium"));
        assert!(computed.contains("custom-label"));
    }

    #[test]
    fn test_label_prop_defaults() {
        // Test prop default handling for Label
        let class = Some("test-class".to_string());
        let default_class = class.unwrap_or_default();
        assert_eq!(default_class, "test-class");
        
        let no_class: Option<String> = None;
        let default_no_class = no_class.unwrap_or_default();
        assert_eq!(default_no_class, "");
        
        let id = Some("test-id".to_string());
        let default_id = id.unwrap_or_default();
        assert_eq!(default_id, "test-id");
        
        let no_id: Option<String> = None;
        let default_no_id = no_id.unwrap_or_default();
        assert_eq!(default_no_id, "");
    }

    #[test]
    fn test_label_style_handling() {
        // Test style handling for Label
        let style = Style::new();
        assert!(style.to_string().is_empty());
        
        let custom_style = Style::new()
            .set("color", "red")
            .set("font-size", "16px");
        
        let style_string = custom_style.to_string();
        assert!(style_string.contains("color: red"));
        assert!(style_string.contains("font-size: 16px"));
    }
}
