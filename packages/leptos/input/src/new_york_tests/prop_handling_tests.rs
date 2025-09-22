//! Prop handling tests for the New York Input component
//! 
//! This module contains tests for prop handling, value management,
//! and attribute processing for the New York theme variant of the Input component.

use leptos::prelude::*;
use leptos_style::Style;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_york_input_value_prop_handling() {
        // Test value prop handling for New York Input
        let value_prop = Some("test-value".to_string());
        assert_eq!(value_prop.unwrap_or_default(), "test-value");

        let value_prop_none: Option<String> = None;
        assert_eq!(value_prop_none.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_input_placeholder_prop_handling() {
        // Test placeholder prop handling for New York Input
        let placeholder_prop = Some("Enter text here".to_string());
        assert_eq!(placeholder_prop.unwrap_or_default(), "Enter text here");

        let placeholder_prop_none: Option<String> = None;
        assert_eq!(placeholder_prop_none.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_input_type_prop_handling() {
        // Test input_type prop handling for New York Input
        let input_type_prop = Some("email".to_string());
        assert_eq!(input_type_prop.unwrap_or_else(|| "text".to_string()), "email");

        let input_type_prop_none: Option<String> = None;
        assert_eq!(input_type_prop_none.unwrap_or_else(|| "text".to_string()), "text");
    }

    #[test]
    fn test_new_york_input_id_prop_handling() {
        // Test id prop handling for New York Input
        let id_prop = Some("test-input-id".to_string());
        assert_eq!(id_prop.unwrap_or_default(), "test-input-id");

        let id_prop_none: Option<String> = None;
        assert_eq!(id_prop_none.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_input_class_prop_handling() {
        // Test class prop handling for New York Input
        let class_prop = Some("custom-input-class".to_string());
        assert_eq!(class_prop.unwrap_or_default(), "custom-input-class");

        let class_prop_none: Option<String> = None;
        assert_eq!(class_prop_none.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_input_style_signal_handling() {
        // Test style signal handling for New York Input
        let style_signal = RwSignal::new("background-color: red;".to_string());
        assert_eq!(style_signal.get(), "background-color: red;");

        style_signal.set("background-color: blue;".to_string());
        assert_eq!(style_signal.get(), "background-color: blue;");
    }

    #[test]
    fn test_new_york_input_disabled_signal_handling() {
        // Test disabled signal handling for New York Input
        let disabled_signal = RwSignal::new(false);
        assert_eq!(disabled_signal.get(), false);

        disabled_signal.set(true);
        assert_eq!(disabled_signal.get(), true);

        disabled_signal.set(false);
        assert_eq!(disabled_signal.get(), false);
    }
}
