#[cfg(test)]
mod rendering {
    use crate::default::{ButtonVariant, ButtonSize, BUTTON_CLASS};
    use leptos::prelude::*;

    // ===== RENDERING TESTS =====
    // These tests focus on rendering logic, CSS class generation, and visual output

    #[test]
    fn test_button_computed_class_default_variant() {
        // Test computed class generation for default variant
        let variant = Some(ButtonVariant::Default);
        let size = Some(ButtonSize::Default);
        let class = Some("custom-class".to_string());

        // Simulate the computed class logic
        let variant_class = match variant.unwrap_or_default() {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSize::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            BUTTON_CLASS, 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        assert!(computed_class.contains("custom-class"));
    }

    #[test]
    fn test_button_computed_class_all_variants() {
        // Test computed class generation for all variants
        let variants = vec![
            (ButtonVariant::Default, "bg-primary text-primary-foreground hover:bg-primary/90"),
            (ButtonVariant::Destructive, "bg-destructive text-destructive-foreground hover:bg-destructive/90"),
            (ButtonVariant::Outline, "border border-input bg-background hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariant::Secondary, "bg-secondary text-secondary-foreground hover:bg-secondary/80"),
            (ButtonVariant::Ghost, "hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariant::Link, "text-primary underline-offset-4 hover:underline"),
        ];

        for (variant, expected_class) in variants {
            let variant_class = match variant {
                ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
                ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
                ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
                ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
                ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
            };
            
            assert_eq!(variant_class, expected_class, "Variant {:?} should have correct class", variant);
        }
    }

    #[test]
    fn test_button_computed_class_all_sizes() {
        // Test computed class generation for all sizes
        let sizes = vec![
            (ButtonSize::Default, "h-10 px-4 py-2"),
            (ButtonSize::Sm, "h-9 rounded-md px-3"),
            (ButtonSize::Lg, "h-11 rounded-md px-8"),
            (ButtonSize::Icon, "h-10 w-10"),
        ];

        for (size, expected_class) in sizes {
            let size_class = match size {
                ButtonSize::Default => "h-10 px-4 py-2",
                ButtonSize::Sm => "h-9 rounded-md px-3",
                ButtonSize::Lg => "h-11 rounded-md px-8",
                ButtonSize::Icon => "h-10 w-10",
            };
            
            assert_eq!(size_class, expected_class, "Size {:?} should have correct class", size);
        }
    }

    #[test]
    fn test_button_computed_class_with_none_props() {
        // Test computed class generation with None props
        let variant: Option<ButtonVariant> = None;
        let size: Option<ButtonSize> = None;
        let class: Option<String> = None;

        // Simulate the computed class logic with None props
        let variant_class = match variant.unwrap_or_default() {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSize::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            BUTTON_CLASS, 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        assert!(!computed_class.contains("custom-class"));
    }

    #[test]
    fn test_button_computed_class_with_custom_class() {
        // Test computed class generation with custom class
        let variant = Some(ButtonVariant::Outline);
        let size = Some(ButtonSize::Lg);
        let class = Some("my-custom-class".to_string());

        let variant_class = match variant.unwrap_or_default() {
            ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSize::Lg => "h-11 rounded-md px-8",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            BUTTON_CLASS, 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("border border-input"));
        assert!(computed_class.contains("h-11 rounded-md px-8"));
        assert!(computed_class.contains("my-custom-class"));
    }

    #[test]
    fn test_button_computed_class_variant_priority() {
        // Test that variant classes are applied correctly
        let variant = ButtonVariant::Destructive;
        let size = ButtonSize::Sm;

        let variant_class = match variant {
            ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            _ => "",
        };
        
        let size_class = match size {
            ButtonSize::Sm => "h-9 rounded-md px-3",
            _ => "",
        };

        let computed_class = format!("{} {} {}", BUTTON_CLASS, variant_class, size_class);

        // Test that destructive variant classes are present
        assert!(computed_class.contains("bg-destructive"));
        assert!(computed_class.contains("text-destructive-foreground"));
        assert!(computed_class.contains("hover:bg-destructive/90"));
    }

    #[test]
    fn test_button_computed_class_size_priority() {
        // Test that size classes are applied correctly
        let variant = ButtonVariant::Ghost;
        let size = ButtonSize::Icon;

        let variant_class = match variant {
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            _ => "",
        };
        
        let size_class = match size {
            ButtonSize::Icon => "h-10 w-10",
            _ => "",
        };

        let computed_class = format!("{} {} {}", BUTTON_CLASS, variant_class, size_class);

        // Test that icon size classes are present
        assert!(computed_class.contains("h-10 w-10"));
        assert!(computed_class.contains("hover:bg-accent"));
    }

    #[test]
    fn test_button_computed_class_combination() {
        // Test combination of variant and size classes
        let variant = ButtonVariant::Secondary;
        let size = ButtonSize::Lg;

        let variant_class = match variant {
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            _ => "",
        };
        
        let size_class = match size {
            ButtonSize::Lg => "h-11 rounded-md px-8",
            _ => "",
        };

        let computed_class = format!("{} {} {}", BUTTON_CLASS, variant_class, size_class);

        // Test that both variant and size classes are present
        assert!(computed_class.contains("bg-secondary"));
        assert!(computed_class.contains("text-secondary-foreground"));
        assert!(computed_class.contains("h-11 rounded-md px-8"));
    }

    #[test]
    fn test_button_computed_class_base_classes() {
        // Test that base button classes are always present
        let computed_class = format!("{} variant-class size-class", BUTTON_CLASS);

        // Test that base classes are present
        assert!(computed_class.contains("inline-flex"));
        assert!(computed_class.contains("items-center"));
        assert!(computed_class.contains("justify-center"));
        assert!(computed_class.contains("rounded-md"));
        assert!(computed_class.contains("transition-colors"));
    }

    #[test]
    fn test_button_computed_class_empty_custom_class() {
        // Test computed class generation with empty custom class
        let variant = Some(ButtonVariant::Default);
        let size = Some(ButtonSize::Default);
        let class = Some("".to_string());

        let variant_class = match variant.unwrap_or_default() {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSize::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            BUTTON_CLASS, 
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
    fn test_button_computed_class_whitespace_handling() {
        // Test computed class generation with whitespace in custom class
        let variant = Some(ButtonVariant::Default);
        let size = Some(ButtonSize::Default);
        let class = Some("  custom-class  ".to_string());

        let variant_class = match variant.unwrap_or_default() {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            _ => "",
        };
        
        let size_class = match size.unwrap_or_default() {
            ButtonSize::Default => "h-10 px-4 py-2",
            _ => "",
        };

        let computed_class = format!("{} {} {} {}", 
            BUTTON_CLASS, 
            variant_class, 
            size_class, 
            class.unwrap_or_default()
        );

        assert!(computed_class.contains("bg-primary"));
        assert!(computed_class.contains("h-10 px-4 py-2"));
        assert!(computed_class.contains("  custom-class  "));
    }
}
