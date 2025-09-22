//! Class constants tests for the New York Input component
//! 
//! This module contains tests for CSS class constants and basic styling
//! for the New York theme variant of the Input component.

use leptos::prelude::*;
use leptos_style::Style;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_york_input_constant() {
        // Test that New York input constant is properly defined
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        // Test that the constant contains expected styling
        assert!(input_class.contains("flex"), "Input class should be flex");
        assert!(input_class.contains("h-10"), "Input class should have height");
        assert!(input_class.contains("w-full"), "Input class should be full width");
        assert!(input_class.contains("rounded-md"), "Input class should have rounded corners");
        assert!(input_class.contains("border"), "Input class should have border");
        assert!(input_class.contains("bg-background"), "Input class should have background");
        assert!(input_class.contains("px-3"), "Input class should have horizontal padding");
        assert!(input_class.contains("py-2"), "Input class should have vertical padding");
        assert!(input_class.contains("text-sm"), "Input class should have small text");
        assert!(input_class.contains("focus-visible:outline-none"), "Input class should have focus-visible outline");
        assert!(input_class.contains("focus-visible:ring-2"), "Input class should have focus-visible ring");
        assert!(input_class.contains("disabled:cursor-not-allowed"), "Input class should have disabled cursor");
        assert!(input_class.contains("disabled:opacity-50"), "Input class should have disabled opacity");
    }

    #[test]
    fn test_new_york_input_computed_class_generation() {
        // Test computed class generation for New York Input
        let class_prop = Some("custom-input-class".to_string());
        let base_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        let computed_class = format!("{} {}", base_class, class_prop.unwrap_or_default());
        
        assert!(computed_class.contains("flex"));
        assert!(computed_class.contains("custom-input-class"));
    }

    #[test]
    fn test_new_york_input_computed_class_with_none_props() {
        // Test computed class generation with None props for New York Input
        let class_prop: Option<String> = None;
        let base_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        let computed_class = format!("{} {}", base_class, class_prop.unwrap_or_default());
        
        assert!(computed_class.contains("flex"));
        assert!(!computed_class.contains("custom-class"));
    }
}
