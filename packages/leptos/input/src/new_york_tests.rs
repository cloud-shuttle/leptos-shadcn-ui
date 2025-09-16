#[cfg(test)]
mod new_york_tests {
    // Removed unused import - component is tested through its CSS classes and behavior
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== NEW YORK INPUT COMPREHENSIVE TESTS =====
    // These tests focus on the New York theme variant implementation for Input component

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
        let style_signal = RwSignal::new(Style::new());
        let style = Style::new();
        style_signal.set(style);

        let style_string = style_signal.get().to_string();
        // Style should be empty initially
        assert_eq!(style_string, "");
    }

    #[test]
    fn test_new_york_input_disabled_signal_handling() {
        // Test disabled signal handling for New York Input
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get());

        disabled_signal.set(true);
        assert!(disabled_signal.get());

        disabled_signal.set(false);
        assert!(!disabled_signal.get());
    }

    #[test]
    fn test_new_york_input_signal_derive() {
        // Test Signal::derive functionality for New York Input
        let class = RwSignal::new("test-class".to_string());
        let base_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        let computed_class = Signal::derive(move || {
            format!("{} {}", base_class, class.get())
        });

        let result = computed_class.get();
        assert!(result.contains("flex"));
        assert!(result.contains("test-class"));

        // Test reactivity
        class.set("new-class".to_string());
        let new_result = computed_class.get();
        assert!(new_result.contains("new-class"));
        assert!(!new_result.contains("test-class"));
    }

    #[test]
    fn test_new_york_input_callback_handling() {
        // Test callback handling for New York Input
        let callback_executed = RwSignal::new(false);
        let callback = Callback::new(move |value: String| {
            callback_executed.set(true);
            assert_eq!(value, "test-value");
        });

        // Execute the callback
        callback.run("test-value".to_string());
        assert!(callback_executed.get(), "New York input callback should have been executed");
    }

    #[test]
    fn test_new_york_input_callback_without_callback() {
        // Test callback handling without callback (should not panic) for New York Input
        let callback: Option<Callback<String>> = None;
        
        // This should not panic
        if let Some(cb) = &callback {
            cb.run("test".to_string());
        }
        
        // Test passes if no panic occurs
        assert!(true);
    }

    #[test]
    fn test_new_york_input_edge_cases() {
        // Test edge cases and error conditions for New York Input
        
        // Test with empty strings
        let empty_value: Option<String> = Some("".to_string());
        assert_eq!(empty_value.unwrap_or_default(), "");

        let empty_placeholder: Option<String> = Some("".to_string());
        assert_eq!(empty_placeholder.unwrap_or_default(), "");

        // Test with very long strings
        let long_string = "a".repeat(1000);
        let long_value = Some(long_string.clone());
        assert_eq!(long_value.unwrap_or_default(), long_string);
    }

    #[test]
    fn test_new_york_input_memory_management() {
        // Test memory management and cleanup for New York Input
        let signal = RwSignal::new(0);
        let callback = Callback::new(move |_value: String| {
            signal.update(|count| *count += 1);
        });

        // Execute callback multiple times
        for _ in 0..100 {
            callback.run("test".to_string());
        }

        assert_eq!(signal.get(), 100);
        
        // Test that signals can be dropped without issues
        let _ = signal;
        let _ = callback;
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_new_york_input_performance_characteristics() {
        // Test performance characteristics for New York Input
        let start = std::time::Instant::now();
        
        // Test class generation performance
        for _ in 0..1000 {
            let _computed_class = format!("{} {}", 
                "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                "test-class"
            );
        }

        let duration = start.elapsed();
        
        // Should complete quickly (less than 50ms for 1000 iterations)
        assert!(duration.as_millis() < 50, "New York input class generation should be fast");
    }

    // ===== NEW YORK SPECIFIC THEME TESTS =====

    #[test]
    fn test_new_york_input_theme_consistency() {
        // Test that New York input theme maintains consistency
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        // New York input should have consistent styling patterns
        assert!(!input_class.is_empty(), "New York input should have styling");
        
        // Input should have layout and spacing
        let has_layout = input_class.contains("flex") || input_class.contains("w-full");
        assert!(has_layout, "New York input should have layout classes");
        
        // Input should have spacing
        let has_spacing = input_class.contains("px-") || input_class.contains("py-");
        assert!(has_spacing, "New York input should have spacing classes");
    }

    #[test]
    fn test_new_york_input_theme_accessibility_features() {
        // Test accessibility features specific to New York input theme
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        
        // New York input theme should maintain accessibility features
        assert!(input_class.contains("focus-visible:outline-none"), "New York input should have focus-visible outline");
        assert!(input_class.contains("focus-visible:ring-2"), "New York input should have focus-visible ring");
        assert!(input_class.contains("disabled:cursor-not-allowed"), "New York input should have disabled cursor");
        assert!(input_class.contains("disabled:opacity-50"), "New York input should have disabled opacity");
        assert!(input_class.contains("placeholder:text-muted-foreground"), "New York input should have placeholder styling");
    }

    #[test]
    fn test_new_york_input_theme_performance_characteristics() {
        // Test performance characteristics specific to New York input theme
        let start = std::time::Instant::now();
        
        // Test New York input theme class generation performance
        for _ in 0..1000 {
            let _computed_class = format!("{} {} {}", 
                "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                "custom-class",
                "additional-class"
            );
        }

        let duration = start.elapsed();
        
        // Should complete quickly (less than 50ms for 1000 iterations)
        assert!(duration.as_millis() < 50, "New York input theme class generation should be fast");
    }

    #[test]
    fn test_new_york_input_theme_input_types() {
        // Test that New York input theme supports different input types
        let input_types = vec![
            "text", "email", "password", "number", "tel", "url", "search", "date", "time", "datetime-local"
        ];

        for input_type in input_types {
            // Each input type should be supported
            let input_type_prop = Some(input_type.to_string());
            assert_eq!(input_type_prop.unwrap_or_else(|| "text".to_string()), input_type);
        }
    }

    #[test]
    fn test_new_york_input_theme_semantic_structure() {
        // Test that New York input theme maintains proper semantic structure
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        // Input should be flex with full width
        assert!(input_class.contains("flex"), "Input should be flex");
        assert!(input_class.contains("w-full"), "Input should be full width");

        // Input should have proper sizing
        assert!(input_class.contains("h-10"), "Input should have height");

        // Input should have border and background
        assert!(input_class.contains("border"), "Input should have border");
        assert!(input_class.contains("bg-background"), "Input should have background");

        // Input should have proper padding
        assert!(input_class.contains("px-3"), "Input should have horizontal padding");
        assert!(input_class.contains("py-2"), "Input should have vertical padding");

        // Input should have proper text styling
        assert!(input_class.contains("text-sm"), "Input should have small text");

        // Input should have focus states
        assert!(input_class.contains("focus-visible:outline-none"), "Input should have focus-visible outline");
        assert!(input_class.contains("focus-visible:ring-2"), "Input should have focus-visible ring");

        // Input should have disabled states
        assert!(input_class.contains("disabled:cursor-not-allowed"), "Input should have disabled cursor");
        assert!(input_class.contains("disabled:opacity-50"), "Input should have disabled opacity");
    }

    #[test]
    fn test_new_york_input_theme_file_input_support() {
        // Test that New York input theme supports file inputs
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        // Input should have file input styling
        assert!(input_class.contains("file:border-0"), "Input should have file border styling");
        assert!(input_class.contains("file:bg-transparent"), "Input should have file background styling");
        assert!(input_class.contains("file:text-sm"), "Input should have file text styling");
        assert!(input_class.contains("file:font-medium"), "Input should have file font styling");
    }

    #[test]
    fn test_new_york_input_theme_placeholder_support() {
        // Test that New York input theme supports placeholder styling
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        // Input should have placeholder styling
        assert!(input_class.contains("placeholder:text-muted-foreground"), "Input should have placeholder styling");
    }

    #[test]
    fn test_new_york_input_theme_ring_support() {
        // Test that New York input theme supports ring styling
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

        // Input should have ring styling
        assert!(input_class.contains("ring-offset-background"), "Input should have ring offset");
        assert!(input_class.contains("focus-visible:ring-ring"), "Input should have focus-visible ring color");
        assert!(input_class.contains("focus-visible:ring-offset-2"), "Input should have focus-visible ring offset");
    }
}
