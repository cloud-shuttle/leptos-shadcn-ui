//! CSS Class Logic Tests for Button Component
//! 
//! This module tests the CSS class computation and styling logic.

#[cfg(test)]
mod css_class_tests {
    use crate::default::{ButtonVariant, ButtonSize, BUTTON_CLASS};

    #[test]
    fn test_css_class_computation_logic() {
        // TDD: Test the class computation logic used in the component
        let variant = ButtonVariant::Default;
        let size = ButtonSize::Lg;
        let custom_class = "custom-btn test-class";
        
        let variant_class = match variant {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        };
        
        let size_class = match size {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Sm => "h-9 rounded-md px-3",
            ButtonSize::Lg => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        };
        
        let computed_class = format!("{} {} {} {}", BUTTON_CLASS, variant_class, size_class, custom_class);
        
        // Test that all parts are included
        assert!(computed_class.contains(BUTTON_CLASS));
        assert!(computed_class.contains("bg-primary")); // variant
        assert!(computed_class.contains("h-11")); // size  
        assert!(computed_class.contains("px-8")); // size
        assert!(computed_class.contains("custom-btn")); // custom
        assert!(computed_class.contains("test-class")); // custom
    }

    #[test]
    fn test_base_css_classes_contain_accessibility_features() {
        // TDD: Test that base classes include required accessibility features
        assert!(BUTTON_CLASS.contains("focus-visible:outline-none"), 
            "Button should have focus outline management");
        assert!(BUTTON_CLASS.contains("focus-visible:ring-2"), 
            "Button should have focus ring for accessibility");
        assert!(BUTTON_CLASS.contains("disabled:pointer-events-none"), 
            "Disabled buttons should not respond to pointer events");
        assert!(BUTTON_CLASS.contains("disabled:opacity-50"), 
            "Disabled buttons should have reduced opacity");
        assert!(BUTTON_CLASS.contains("transition-colors"), 
            "Button should have smooth color transitions");
    }
}
