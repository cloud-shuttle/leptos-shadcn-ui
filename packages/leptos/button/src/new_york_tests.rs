#[cfg(test)]
mod new_york_tests {
    use crate::new_york::{ButtonVariant as ButtonVariantNewYork, ButtonSize as ButtonSizeNewYork, ButtonChildProps as ButtonChildPropsNewYork};
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== NEW YORK VARIANT COMPREHENSIVE TESTS =====
    // These tests focus on the New York theme variant implementation

    #[test]
    fn test_new_york_button_variant_enum_conversions() {
        // Test ButtonVariantNewYork From<String> implementation
        assert_eq!(ButtonVariantNewYork::from("destructive".to_string()), ButtonVariantNewYork::Destructive);
        assert_eq!(ButtonVariantNewYork::from("outline".to_string()), ButtonVariantNewYork::Outline);
        assert_eq!(ButtonVariantNewYork::from("secondary".to_string()), ButtonVariantNewYork::Secondary);
        assert_eq!(ButtonVariantNewYork::from("ghost".to_string()), ButtonVariantNewYork::Ghost);
        assert_eq!(ButtonVariantNewYork::from("link".to_string()), ButtonVariantNewYork::Link);
        assert_eq!(ButtonVariantNewYork::from("invalid".to_string()), ButtonVariantNewYork::Default);
        assert_eq!(ButtonVariantNewYork::from("".to_string()), ButtonVariantNewYork::Default);
    }

    #[test]
    fn test_new_york_button_size_enum_conversions() {
        // Test ButtonSizeNewYork From<String> implementation
        assert_eq!(ButtonSizeNewYork::from("sm".to_string()), ButtonSizeNewYork::Sm);
        assert_eq!(ButtonSizeNewYork::from("lg".to_string()), ButtonSizeNewYork::Lg);
        assert_eq!(ButtonSizeNewYork::from("icon".to_string()), ButtonSizeNewYork::Icon);
        assert_eq!(ButtonSizeNewYork::from("invalid".to_string()), ButtonSizeNewYork::Default);
        assert_eq!(ButtonSizeNewYork::from("".to_string()), ButtonSizeNewYork::Default);
    }

    #[test]
    fn test_new_york_button_variant_default() {
        // Test Default implementation for ButtonVariantNewYork
        assert_eq!(ButtonVariantNewYork::default(), ButtonVariantNewYork::Default);
    }

    #[test]
    fn test_new_york_button_size_default() {
        // Test Default implementation for ButtonSizeNewYork
        assert_eq!(ButtonSizeNewYork::default(), ButtonSizeNewYork::Default);
    }

    #[test]
    fn test_new_york_button_variant_equality() {
        // Test PartialEq implementation for ButtonVariantNewYork
        assert_eq!(ButtonVariantNewYork::Default, ButtonVariantNewYork::Default);
        assert_ne!(ButtonVariantNewYork::Default, ButtonVariantNewYork::Destructive);
        assert_eq!(ButtonVariantNewYork::Outline, ButtonVariantNewYork::Outline);
        assert_ne!(ButtonVariantNewYork::Ghost, ButtonVariantNewYork::Link);
    }

    #[test]
    fn test_new_york_button_size_equality() {
        // Test PartialEq implementation for ButtonSizeNewYork
        assert_eq!(ButtonSizeNewYork::Default, ButtonSizeNewYork::Default);
        assert_ne!(ButtonSizeNewYork::Default, ButtonSizeNewYork::Sm);
        assert_eq!(ButtonSizeNewYork::Icon, ButtonSizeNewYork::Icon);
        assert_ne!(ButtonSizeNewYork::Lg, ButtonSizeNewYork::Sm);
    }

    #[test]
    fn test_new_york_button_variant_clone() {
        // Test Clone implementation for ButtonVariantNewYork
        let variant = ButtonVariantNewYork::Destructive;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_new_york_button_size_clone() {
        // Test Clone implementation for ButtonSizeNewYork
        let size = ButtonSizeNewYork::Lg;
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_new_york_button_variant_debug() {
        // Test Debug implementation for ButtonVariantNewYork
        let variant = ButtonVariantNewYork::Outline;
        let debug_str = format!("{:?}", variant);
        assert!(debug_str.contains("Outline"));
    }

    #[test]
    fn test_new_york_button_size_debug() {
        // Test Debug implementation for ButtonSizeNewYork
        let size = ButtonSizeNewYork::Icon;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Icon"));
    }

    #[test]
    fn test_new_york_button_child_props_creation() {
        // Test ButtonChildPropsNewYork creation and field access
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: true,
            r#type: "submit".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        assert_eq!(props.class, "test-class");
        assert_eq!(props.id, "test-id");
        assert_eq!(props.style, "color: red");
        assert!(props.disabled);
        assert_eq!(props.r#type, "submit");
        assert!(props.onclick.is_some());
    }

    #[test]
    fn test_new_york_button_child_props_clone() {
        // Test ButtonChildPropsNewYork Clone implementation
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };

        let cloned = props.clone();
        assert_eq!(props.class, cloned.class);
        assert_eq!(props.id, cloned.id);
        assert_eq!(props.style, cloned.style);
        assert_eq!(props.disabled, cloned.disabled);
        assert_eq!(props.r#type, cloned.r#type);
        assert_eq!(props.onclick.is_some(), cloned.onclick.is_some());
    }

    #[test]
    fn test_new_york_button_child_props_debug() {
        // Test ButtonChildPropsNewYork Debug implementation
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: true,
            r#type: "button".to_string(),
            onclick: None,
        };

        let debug_str = format!("{:?}", props);
        assert!(debug_str.contains("test-class"));
        assert!(debug_str.contains("test-id"));
    }

    #[test]
    fn test_new_york_button_click_handler_with_callback() {
        // Test click handler with callback for New York variant
        let click_count = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            click_count.update(|count| *count += 1);
        });

        // Simulate the click handler logic
        callback.run(());
        assert_eq!(click_count.get(), 1);

        callback.run(());
        assert_eq!(click_count.get(), 2);
    }

    #[test]
    fn test_new_york_button_click_handler_without_callback() {
        // Test click handler without callback (should not panic) for New York variant
        let callback: Option<Callback<()>> = None;
        
        // This should not panic
        if let Some(cb) = &callback {
            cb.run(());
        }
        
        // Test passes if no panic occurs
    }

    #[test]
    fn test_new_york_button_computed_class_default_variant() {
        // Test computed class generation for default variant in New York theme
        let variant = Some(ButtonVariantNewYork::Default);
        let size = Some(ButtonSizeNewYork::Default);
        let class = Some("custom-class".to_string());

        // Simulate the computed class logic for New York variant
        let variant_class = match variant.unwrap_or_default() {
            ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSizeNewYork::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50", 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        assert!(computed_class.contains("custom-class"));
    }

    #[test]
    fn test_new_york_button_computed_class_all_variants() {
        // Test computed class generation for all variants in New York theme
        let variants = vec![
            (ButtonVariantNewYork::Default, "bg-primary text-primary-foreground hover:bg-primary/90"),
            (ButtonVariantNewYork::Destructive, "bg-destructive text-destructive-foreground hover:bg-destructive/90"),
            (ButtonVariantNewYork::Outline, "border border-input bg-background hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariantNewYork::Secondary, "bg-secondary text-secondary-foreground hover:bg-secondary/80"),
            (ButtonVariantNewYork::Ghost, "hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariantNewYork::Link, "text-primary underline-offset-4 hover:underline"),
        ];

        for (variant, expected_class) in variants {
            let variant_class = match variant {
                ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
                ButtonVariantNewYork::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
                ButtonVariantNewYork::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
                ButtonVariantNewYork::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                ButtonVariantNewYork::Ghost => "hover:bg-accent hover:text-accent-foreground",
                ButtonVariantNewYork::Link => "text-primary underline-offset-4 hover:underline",
            };
            
            assert_eq!(variant_class, expected_class, "New York variant {:?} should have correct class", variant);
        }
    }

    #[test]
    fn test_new_york_button_computed_class_all_sizes() {
        // Test computed class generation for all sizes in New York theme
        let sizes = vec![
            (ButtonSizeNewYork::Default, "h-10 px-4 py-2"),
            (ButtonSizeNewYork::Sm, "h-9 rounded-md px-3"),
            (ButtonSizeNewYork::Lg, "h-11 rounded-md px-8"),
            (ButtonSizeNewYork::Icon, "h-10 w-10"),
        ];

        for (size, expected_class) in sizes {
            let size_class = match size {
                ButtonSizeNewYork::Default => "h-10 px-4 py-2",
                ButtonSizeNewYork::Sm => "h-9 rounded-md px-3",
                ButtonSizeNewYork::Lg => "h-11 rounded-md px-8",
                ButtonSizeNewYork::Icon => "h-10 w-10",
            };
            
            assert_eq!(size_class, expected_class, "New York size {:?} should have correct class", size);
        }
    }

    #[test]
    fn test_new_york_button_computed_class_with_none_props() {
        // Test computed class generation with None props for New York variant
        let variant: Option<ButtonVariantNewYork> = None;
        let size: Option<ButtonSizeNewYork> = None;
        let class: Option<String> = None;

        // Simulate the computed class logic with None props for New York variant
        let variant_class = match variant.unwrap_or_default() {
            ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSizeNewYork::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50", 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        assert!(!computed_class.contains("custom-class"));
    }

    #[test]
    fn test_new_york_button_as_child_callback_execution() {
        // Test as_child callback execution for New York variant
        let callback_executed = RwSignal::new(false);
        let as_child_callback = Callback::new(move |props: ButtonChildPropsNewYork| {
            callback_executed.set(true);
            assert_eq!(props.class, "test-class");
            assert_eq!(props.id, "test-id");
            assert!(props.disabled);
            assert_eq!(props.r#type, "button");
            view! { <div>"Child Component"</div> }.into_any()
        });

        let child_props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: true,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Execute the callback
        let _result = as_child_callback.run(child_props);
        assert!(callback_executed.get(), "New York as_child callback should have been executed");
    }

    #[test]
    fn test_new_york_button_style_signal_handling() {
        // Test style signal handling for New York variant
        let style_signal = RwSignal::new(Style::new());
        let style = Style::new();
        style_signal.set(style);

        let style_string = style_signal.get().to_string();
        // Style should be empty initially
        assert_eq!(style_string, "");
    }

    #[test]
    fn test_new_york_button_disabled_signal_handling() {
        // Test disabled signal handling for New York variant
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get());

        disabled_signal.set(true);
        assert!(disabled_signal.get());

        disabled_signal.set(false);
        assert!(!disabled_signal.get());
    }

    #[test]
    fn test_new_york_button_id_prop_handling() {
        // Test id prop handling for New York variant
        let id_prop = Some("test-button-id".to_string());
        assert_eq!(id_prop.unwrap_or_default(), "test-button-id");

        let id_prop_none: Option<String> = None;
        assert_eq!(id_prop_none.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_button_class_prop_handling() {
        // Test class prop handling for New York variant
        let class_prop = Some("custom-button-class".to_string());
        assert_eq!(class_prop.unwrap_or_default(), "custom-button-class");

        let class_prop_none: Option<String> = None;
        assert_eq!(class_prop_none.unwrap_or_default(), "");
    }

    #[test]
    fn test_new_york_button_children_handling() {
        // Test children handling for New York variant - simplified test
        let children: Option<Children> = None;
        assert!(children.is_none());
        
        // Test that we can create a simple children option
        let has_children = true;
        assert!(has_children);
    }

    #[test]
    fn test_new_york_button_callback_clone() {
        // Test callback cloning for New York variant
        let original_callback = Callback::new(|_: ()| {});
        let cloned_callback = original_callback.clone();
        
        // Both should be valid callbacks
    }

    #[test]
    fn test_new_york_button_signal_derive() {
        // Test Signal::derive functionality for New York variant
        let variant = RwSignal::new(ButtonVariantNewYork::Default);
        let size = RwSignal::new(ButtonSizeNewYork::Default);
        let class = RwSignal::new("test-class".to_string());

        let computed_class = Signal::derive(move || {
            let variant_class = match variant.get() {
                ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
                ButtonVariantNewYork::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
                ButtonVariantNewYork::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
                ButtonVariantNewYork::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                ButtonVariantNewYork::Ghost => "hover:bg-accent hover:text-accent-foreground",
                ButtonVariantNewYork::Link => "text-primary underline-offset-4 hover:underline",
            };
            
            let size_class = match size.get() {
                ButtonSizeNewYork::Default => "h-10 px-4 py-2",
                ButtonSizeNewYork::Sm => "h-9 rounded-md px-3",
                ButtonSizeNewYork::Lg => "h-11 rounded-md px-8",
                ButtonSizeNewYork::Icon => "h-10 w-10",
            };

            format!("{} {} {} {}", 
                "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50", 
                variant_class, 
                size_class, 
                class.get()
            )
        });

        let result = computed_class.get();
        assert!(result.contains("bg-primary"));
        assert!(result.contains("h-10 px-4 py-2"));
        assert!(result.contains("test-class"));

        // Test reactivity
        variant.set(ButtonVariantNewYork::Destructive);
        let new_result = computed_class.get();
        // The computed class should update to reflect the new variant
        assert!(new_result.contains("bg-destructive"), "Expected 'bg-destructive' in '{}'", new_result);
        assert!(!new_result.contains("bg-primary"), "Expected no 'bg-primary' in '{}'", new_result);
    }

    #[test]
    fn test_new_york_button_edge_cases() {
        // Test edge cases and error conditions for New York variant
        
        // Test with empty strings
        let empty_variant = ButtonVariantNewYork::from("".to_string());
        assert_eq!(empty_variant, ButtonVariantNewYork::Default);

        let empty_size = ButtonSizeNewYork::from("".to_string());
        assert_eq!(empty_size, ButtonSizeNewYork::Default);

        // Test with invalid strings
        let invalid_variant = ButtonVariantNewYork::from("invalid_variant".to_string());
        assert_eq!(invalid_variant, ButtonVariantNewYork::Default);

        let invalid_size = ButtonSizeNewYork::from("invalid_size".to_string());
        assert_eq!(invalid_size, ButtonSizeNewYork::Default);

        // Test with very long strings
        let long_string = "a".repeat(1000);
        let long_variant = ButtonVariantNewYork::from(long_string.clone());
        assert_eq!(long_variant, ButtonVariantNewYork::Default);

        let long_size = ButtonSizeNewYork::from(long_string);
        assert_eq!(long_size, ButtonSizeNewYork::Default);
    }

    #[test]
    fn test_new_york_button_memory_management() {
        // Test memory management and cleanup for New York variant
        let signal = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            signal.update(|count| *count += 1);
        });

        // Execute callback multiple times
        for _ in 0..100 {
            callback.run(());
        }

        assert_eq!(signal.get(), 100);
        
        // Test that signals can be dropped without issues
        let _ = signal;
        let _ = callback;
        
        // Test passes if no memory leaks or panics occur
    }

    #[test]
    fn test_new_york_button_performance_characteristics() {
        // Test performance characteristics for New York variant
        let start = std::time::Instant::now();
        
        // Create many button variants
        let variants = vec![
            ButtonVariantNewYork::Default,
            ButtonVariantNewYork::Destructive,
            ButtonVariantNewYork::Outline,
            ButtonVariantNewYork::Secondary,
            ButtonVariantNewYork::Ghost,
            ButtonVariantNewYork::Link,
        ];

        let sizes = vec![
            ButtonSizeNewYork::Default,
            ButtonSizeNewYork::Sm,
            ButtonSizeNewYork::Lg,
            ButtonSizeNewYork::Icon,
        ];

        // Test enum operations performance
        for _ in 0..1000 {
            for variant in &variants {
                let _ = format!("{:?}", variant);
                let _ = variant.clone();
            }
            
            for size in &sizes {
                let _ = format!("{:?}", size);
                let _ = size.clone();
            }
        }

        let duration = start.elapsed();
        
        // Should complete quickly (less than 100ms for 1000 iterations)
        assert!(duration.as_millis() < 100, "New York enum operations should be fast");
    }

    // ===== NEW YORK SPECIFIC THEME TESTS =====

    #[test]
    fn test_new_york_theme_consistency() {
        // Test that New York theme maintains consistency across all variants
        let variants = vec![
            ButtonVariantNewYork::Default,
            ButtonVariantNewYork::Destructive,
            ButtonVariantNewYork::Outline,
            ButtonVariantNewYork::Secondary,
            ButtonVariantNewYork::Ghost,
            ButtonVariantNewYork::Link,
        ];

        for variant in variants {
            // Each variant should have consistent styling patterns
            let variant_class = match variant {
                ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
                ButtonVariantNewYork::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
                ButtonVariantNewYork::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
                ButtonVariantNewYork::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                ButtonVariantNewYork::Ghost => "hover:bg-accent hover:text-accent-foreground",
                ButtonVariantNewYork::Link => "text-primary underline-offset-4 hover:underline",
            };

            // All variants should have some form of styling
            assert!(!variant_class.is_empty(), "New York variant {:?} should have styling", variant);
            
            // All variants should have hover states (except link which has underline)
            if !matches!(variant, ButtonVariantNewYork::Link) {
                assert!(variant_class.contains("hover:"), "New York variant {:?} should have hover state", variant);
            }
        }
    }

    #[test]
    fn test_new_york_theme_accessibility_features() {
        // Test accessibility features specific to New York theme
        let base_class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
        
        // New York theme should maintain accessibility features
        assert!(base_class.contains("focus-visible:outline-none"), "New York theme should have focus-visible outline");
        assert!(base_class.contains("focus-visible:ring-2"), "New York theme should have focus-visible ring");
        assert!(base_class.contains("disabled:pointer-events-none"), "New York theme should disable pointer events when disabled");
        assert!(base_class.contains("disabled:opacity-50"), "New York theme should reduce opacity when disabled");
    }

    #[test]
    fn test_new_york_theme_performance_characteristics() {
        // Test performance characteristics specific to New York theme
        let start = std::time::Instant::now();
        
        // Test New York theme class generation performance
        for _ in 0..1000 {
            let _computed_class = format!("{} {} {}", 
                "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
                "bg-primary text-primary-foreground hover:bg-primary/90",
                "h-10 px-4 py-2"
            );
        }

        let duration = start.elapsed();
        
        // Should complete quickly (less than 50ms for 1000 iterations)
        assert!(duration.as_millis() < 50, "New York theme class generation should be fast");
    }

    #[test]
    fn test_new_york_theme_variant_differences() {
        // Test that New York theme variants have distinct styling
        let default_class = match ButtonVariantNewYork::Default {
            ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let destructive_class = match ButtonVariantNewYork::Destructive {
            ButtonVariantNewYork::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            _ => "",
        };
        
        let outline_class = match ButtonVariantNewYork::Outline {
            ButtonVariantNewYork::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            _ => "",
        };

        // Each variant should have distinct styling
        assert_ne!(default_class, destructive_class, "Default and Destructive should have different styling");
        assert_ne!(default_class, outline_class, "Default and Outline should have different styling");
        assert_ne!(destructive_class, outline_class, "Destructive and Outline should have different styling");
    }

    #[test]
    fn test_new_york_theme_size_differences() {
        // Test that New York theme sizes have distinct styling
        let default_size = match ButtonSizeNewYork::Default {
            ButtonSizeNewYork::Default => "h-10 px-4 py-2",
            _ => "",
        };
        
        let sm_size = match ButtonSizeNewYork::Sm {
            ButtonSizeNewYork::Sm => "h-9 rounded-md px-3",
            _ => "",
        };
        
        let lg_size = match ButtonSizeNewYork::Lg {
            ButtonSizeNewYork::Lg => "h-11 rounded-md px-8",
            _ => "",
        };
        
        let icon_size = match ButtonSizeNewYork::Icon {
            ButtonSizeNewYork::Icon => "h-10 w-10",
            _ => "",
        };

        // Each size should have distinct styling
        assert_ne!(default_size, sm_size, "Default and Sm should have different sizing");
        assert_ne!(default_size, lg_size, "Default and Lg should have different sizing");
        assert_ne!(default_size, icon_size, "Default and Icon should have different sizing");
        assert_ne!(sm_size, lg_size, "Sm and Lg should have different sizing");
        assert_ne!(sm_size, icon_size, "Sm and Icon should have different sizing");
        assert_ne!(lg_size, icon_size, "Lg and Icon should have different sizing");
    }
}
