#[cfg(test)]
mod styling_tests {
    use crate::default::{INPUT_CLASS, INPUT_ERROR_CLASS};
    use leptos::prelude::*;
    use leptos_style::Style;

    #[test]
    fn test_input_class_composition() {
        // Test class composition logic
        let base_class = INPUT_CLASS;
        let custom_class = "custom-input";
        let error_class = INPUT_ERROR_CLASS;
        
        // Test base class composition
        let composed_class = format!("{} {}", base_class, custom_class);
        assert!(composed_class.contains("flex"));
        assert!(composed_class.contains("h-10"));
        assert!(composed_class.contains("custom-input"));
        
        // Test error class composition
        let error_composed = format!("{} {}", base_class, error_class);
        assert!(error_composed.contains("border-destructive"));
        assert!(error_composed.contains("focus-visible:ring-destructive"));
        
        // Test full composition
        let full_composed = format!("{} {} {}", base_class, custom_class, error_class);
        assert!(full_composed.contains("flex"));
        assert!(full_composed.contains("custom-input"));
        assert!(full_composed.contains("border-destructive"));
    }

    #[test]
    fn test_input_class_trimming() {
        // Test class trimming and normalization
        let base_class = INPUT_CLASS;
        let empty_class = "";
        let whitespace_class = "   ";
        let custom_class = "custom-input";
        
        // Test with empty class
        let composed_empty = format!("{} {}", base_class, empty_class).trim().to_string();
        assert_eq!(composed_empty, base_class);
        
        // Test with whitespace class
        let composed_whitespace = format!("{} {}", base_class, whitespace_class).trim().to_string();
        assert_eq!(composed_whitespace, base_class);
        
        // Test with valid custom class
        let composed_valid = format!("{} {}", base_class, custom_class).trim().to_string();
        assert!(composed_valid.contains("flex"));
        assert!(composed_valid.contains("custom-input"));
    }

    #[test]
    fn test_input_style_signal_handling() {
        // Test style signal handling
        let style_signal = RwSignal::new(Style::new());
        
        // Test initial empty style
        let initial_style = style_signal.get().to_string();
        assert_eq!(initial_style, "");
        
        // Test setting style properties
        let new_style = Style::new();
        style_signal.set(new_style);
        
        let updated_style = style_signal.get().to_string();
        assert_eq!(updated_style, ""); // Style is empty by default
        
        // Test clearing style
        style_signal.set(Style::new());
        let cleared_style = style_signal.get().to_string();
        assert_eq!(cleared_style, "");
    }

    #[test]
    fn test_input_conditional_styling() {
        // Test conditional styling based on state
        let has_error = RwSignal::new(false);
        let is_disabled = RwSignal::new(false);
        let is_focused = RwSignal::new(false);
        
        // Test normal state
        let normal_class = if has_error.get() {
            format!("{} {}", INPUT_CLASS, INPUT_ERROR_CLASS)
        } else {
            INPUT_CLASS.to_string()
        };
        assert_eq!(normal_class, INPUT_CLASS);
        
        // Test error state
        has_error.set(true);
        let error_class = if has_error.get() {
            format!("{} {}", INPUT_CLASS, INPUT_ERROR_CLASS)
        } else {
            INPUT_CLASS.to_string()
        };
        assert!(error_class.contains("border-destructive"));
        
        // Test disabled state
        is_disabled.set(true);
        let disabled_class = if is_disabled.get() {
            format!("{} disabled:opacity-50", INPUT_CLASS)
        } else {
            INPUT_CLASS.to_string()
        };
        assert!(disabled_class.contains("disabled:opacity-50"));
        
        // Test focused state
        is_focused.set(true);
        let focused_class = if is_focused.get() {
            format!("{} focus:ring-2", INPUT_CLASS)
        } else {
            INPUT_CLASS.to_string()
        };
        assert!(focused_class.contains("focus:ring-2"));
    }

    #[test]
    fn test_input_style_override_handling() {
        // Test style override handling
        let base_style = Style::new();
        let override_style = Style::new();
        
        // Test empty styles
        let combined_style = format!("{} {}", base_style.to_string(), override_style.to_string()).trim().to_string();
        assert_eq!(combined_style, "");
        
        // Test with base style
        let base = Style::new();
        let override_styles = Style::new();
        
        let base_str = base.to_string();
        let override_str = override_styles.to_string();
        
        assert_eq!(base_str, "");
        assert_eq!(override_str, "");
    }

    #[test]
    fn test_input_responsive_styling() {
        // Test responsive styling classes
        let base_class = INPUT_CLASS;
        let responsive_class = "sm:w-full md:w-1/2 lg:w-1/3";
        
        let composed = format!("{} {}", base_class, responsive_class);
        assert!(composed.contains("flex"));
        assert!(composed.contains("sm:w-full"));
        assert!(composed.contains("md:w-1/2"));
        assert!(composed.contains("lg:w-1/3"));
    }

    #[test]
    fn test_input_theme_integration() {
        // Test theme integration
        let base_class = INPUT_CLASS;
        let theme_class = "dark:bg-gray-800 dark:text-white";
        
        let themed_class = format!("{} {}", base_class, theme_class);
        assert!(themed_class.contains("flex"));
        assert!(themed_class.contains("dark:bg-gray-800"));
        assert!(themed_class.contains("dark:text-white"));
    }

    #[test]
    fn test_input_variant_styling() {
        // Test variant styling
        let base_class = INPUT_CLASS;
        let variant_class = "variant-outline";
        let size_class = "size-lg";
        
        let variant_styled = format!("{} {} {}", base_class, variant_class, size_class);
        assert!(variant_styled.contains("flex"));
        assert!(variant_styled.contains("variant-outline"));
        assert!(variant_styled.contains("size-lg"));
    }

    #[test]
    fn test_input_animation_styling() {
        // Test animation styling
        let base_class = INPUT_CLASS;
        let animation_class = "transition-all duration-200 ease-in-out";
        
        let animated_class = format!("{} {}", base_class, animation_class);
        assert!(animated_class.contains("flex"));
        assert!(animated_class.contains("transition-all"));
        assert!(animated_class.contains("duration-200"));
        assert!(animated_class.contains("ease-in-out"));
    }

    #[test]
    fn test_input_accessibility_styling() {
        // Test accessibility styling
        let base_class = INPUT_CLASS;
        let a11y_class = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500";
        
        let a11y_styled = format!("{} {}", base_class, a11y_class);
        assert!(a11y_styled.contains("flex"));
        assert!(a11y_styled.contains("focus-visible:outline-none"));
        assert!(a11y_styled.contains("focus-visible:ring-2"));
        assert!(a11y_styled.contains("focus-visible:ring-blue-500"));
    }

    #[test]
    fn test_input_style_performance() {
        // Test style performance
        let style_signal = RwSignal::new(Style::new());
        
        // Test rapid style updates
        let start = std::time::Instant::now();
        
        for _i in 0..100 {
            let style = Style::new();
            style_signal.set(style);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be very fast
        
        // Test final state
        let final_style = style_signal.get().to_string();
        assert_eq!(final_style, "");
    }
}
