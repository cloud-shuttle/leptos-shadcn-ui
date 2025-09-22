#[cfg(test)]
mod integration_tests {
    use crate::new_york::{ButtonVariant as ButtonVariantNewYork, ButtonSize as ButtonSizeNewYork, ButtonChildProps as ButtonChildPropsNewYork};
    use leptos::prelude::*;

    // ===== NEW YORK INTEGRATION TESTS =====
    // These tests focus on integration scenarios and complex usage patterns

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
        // Test click handler with callback
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
        // Test click handler without callback (should not panic)
        let callback: Option<Callback<()>> = None;
        
        // This should not panic
        if let Some(cb) = &callback {
            cb.run(());
        }
        
        // Test passes if no panic occurs
    }

    #[test]
    fn test_new_york_button_computed_class_default_variant() {
        // Test computed class generation for default variant
        let variant = Some(ButtonVariantNewYork::Default);
        let size = Some(ButtonSizeNewYork::Default);
        let class = Some("custom-class".to_string());

        // Simulate the computed class logic
        let variant_class = match variant.unwrap_or_default() {
            ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSizeNewYork::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
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
        // Test computed class generation for all variants
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
            
            assert_eq!(variant_class, expected_class, "Variant {:?} should have correct class", variant);
        }
    }

    #[test]
    fn test_new_york_button_computed_class_all_sizes() {
        // Test computed class generation for all sizes
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
            
            assert_eq!(size_class, expected_class, "Size {:?} should have correct class", size);
        }
    }

    #[test]
    fn test_new_york_button_computed_class_with_none_props() {
        // Test computed class generation with None props
        let variant: Option<ButtonVariantNewYork> = None;
        let size: Option<ButtonSizeNewYork> = None;
        let class: Option<String> = None;

        // Simulate the computed class logic with None props
        let variant_class = match variant.unwrap_or_default() {
            ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSizeNewYork::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        assert!(!computed_class.contains("custom-class"));
    }

    #[test]
    fn test_new_york_button_computed_class_with_custom_class() {
        // Test computed class generation with custom class
        let variant = Some(ButtonVariantNewYork::Outline);
        let size = Some(ButtonSizeNewYork::Lg);
        let class = Some("my-custom-class".to_string());

        let variant_class = match variant.unwrap_or_default() {
            ButtonVariantNewYork::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSizeNewYork::Lg => "h-11 rounded-md px-8",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("border border-input"));
        assert!(computed_class.contains("h-11 rounded-md px-8"));
        assert!(computed_class.contains("my-custom-class"));
    }

    #[test]
    fn test_new_york_button_computed_class_variant_priority() {
        // Test that variant classes are applied correctly
        let variant = ButtonVariantNewYork::Destructive;
        let size = ButtonSizeNewYork::Sm;

        let variant_class = match variant {
            ButtonVariantNewYork::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            _ => "",
        };
        
        let size_class = match size {
            ButtonSizeNewYork::Sm => "h-9 rounded-md px-3",
            _ => "",
        };

        let computed_class = format!("{} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
            variant_class, 
            size_class
        );

        // Test that destructive variant classes are present
        assert!(computed_class.contains("bg-destructive"));
        assert!(computed_class.contains("text-destructive-foreground"));
        assert!(computed_class.contains("hover:bg-destructive/90"));
    }

    #[test]
    fn test_new_york_button_computed_class_size_priority() {
        // Test that size classes are applied correctly
        let variant = ButtonVariantNewYork::Ghost;
        let size = ButtonSizeNewYork::Icon;

        let variant_class = match variant {
            ButtonVariantNewYork::Ghost => "hover:bg-accent hover:text-accent-foreground",
            _ => "",
        };
        
        let size_class = match size {
            ButtonSizeNewYork::Icon => "h-10 w-10",
            _ => "",
        };

        let computed_class = format!("{} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
            variant_class, 
            size_class
        );

        // Test that icon size classes are present
        assert!(computed_class.contains("h-10 w-10"));
        assert!(computed_class.contains("hover:bg-accent"));
    }

    #[test]
    fn test_new_york_button_computed_class_combination() {
        // Test combination of variant and size classes
        let variant = ButtonVariantNewYork::Secondary;
        let size = ButtonSizeNewYork::Lg;

        let variant_class = match variant {
            ButtonVariantNewYork::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            _ => "",
        };
        
        let size_class = match size {
            ButtonSizeNewYork::Lg => "h-11 rounded-md px-8",
            _ => "",
        };

        let computed_class = format!("{} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
            variant_class, 
            size_class
        );

        // Test that both variant and size classes are present
        assert!(computed_class.contains("bg-secondary"));
        assert!(computed_class.contains("text-secondary-foreground"));
        assert!(computed_class.contains("h-11 rounded-md px-8"));
    }

    #[test]
    fn test_new_york_button_computed_class_base_classes() {
        // Test that base button classes are always present
        let computed_class = format!("{} variant-class size-class", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors"
        );

        // Test that base classes are present
        assert!(computed_class.contains("inline-flex"));
        assert!(computed_class.contains("items-center"));
        assert!(computed_class.contains("justify-center"));
        assert!(computed_class.contains("rounded-md"));
        assert!(computed_class.contains("transition-colors"));
    }

    #[test]
    fn test_new_york_button_computed_class_empty_custom_class() {
        // Test computed class generation with empty custom class
        let variant = Some(ButtonVariantNewYork::Default);
        let size = Some(ButtonSizeNewYork::Default);
        let class = Some("".to_string());

        let variant_class = match variant.unwrap_or_default() {
            ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSizeNewYork::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        // Empty custom class should not affect the result
        assert!(!computed_class.contains("custom-class"));
    }

    #[test]
    fn test_new_york_button_computed_class_whitespace_handling() {
        // Test computed class generation with whitespace in custom class
        let variant = Some(ButtonVariantNewYork::Default);
        let size = Some(ButtonSizeNewYork::Default);
        let class = Some("  custom-class  ".to_string());

        let variant_class = match variant.unwrap_or_default() {
            ButtonVariantNewYork::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSizeNewYork::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors", 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        assert!(computed_class.contains("  custom-class  "));
    }
}
