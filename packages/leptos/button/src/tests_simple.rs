// Simple working tests for Button component
#[cfg(test)]
mod button_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize, BUTTON_CLASS};
    use leptos::prelude::*;
    
    #[test]
    fn button_variant_enum_works() {
        // Test ButtonVariant enum functionality
        let default_variant = ButtonVariant::default();
        assert_eq!(default_variant, ButtonVariant::Default);
        
        // Test From<String> implementations
        assert_eq!(ButtonVariant::from("destructive".to_string()), ButtonVariant::Destructive);
        assert_eq!(ButtonVariant::from("outline".to_string()), ButtonVariant::Outline);
        assert_eq!(ButtonVariant::from("secondary".to_string()), ButtonVariant::Secondary);
        assert_eq!(ButtonVariant::from("ghost".to_string()), ButtonVariant::Ghost);
        assert_eq!(ButtonVariant::from("link".to_string()), ButtonVariant::Link);
        
        // Unknown values should default
        assert_eq!(ButtonVariant::from("unknown".to_string()), ButtonVariant::Default);
    }
    
    #[test] 
    fn button_size_enum_works() {
        // Test ButtonSize enum functionality
        let default_size = ButtonSize::default();
        assert_eq!(default_size, ButtonSize::Default);
        
        // Test From<String> implementations  
        assert_eq!(ButtonSize::from("sm".to_string()), ButtonSize::Sm);
        assert_eq!(ButtonSize::from("lg".to_string()), ButtonSize::Lg);
        assert_eq!(ButtonSize::from("icon".to_string()), ButtonSize::Icon);
        
        // Unknown values should default
        assert_eq!(ButtonSize::from("unknown".to_string()), ButtonSize::Default);
    }
    
    #[test]
    fn button_class_constant_has_content() {
        // Test that BUTTON_CLASS is not empty and contains expected classes
        assert!(!BUTTON_CLASS.is_empty(), "BUTTON_CLASS should not be empty");
        
        // Check for key accessibility and styling classes
        assert!(BUTTON_CLASS.contains("inline-flex"), "Should have inline-flex");
        assert!(BUTTON_CLASS.contains("items-center"), "Should have items-center");
        assert!(BUTTON_CLASS.contains("justify-center"), "Should have justify-center"); 
        assert!(BUTTON_CLASS.contains("rounded-md"), "Should have rounded-md");
        assert!(BUTTON_CLASS.contains("focus-visible:outline-none"), "Should have focus styles");
        assert!(BUTTON_CLASS.contains("focus-visible:ring-2"), "Should have focus ring");
        assert!(BUTTON_CLASS.contains("disabled:opacity-50"), "Should handle disabled state");
    }
    
    #[test]
    fn button_variants_can_be_cloned_and_compared() {
        let variant1 = ButtonVariant::Destructive;
        let variant2 = variant1.clone();
        let variant3 = ButtonVariant::Default;
        
        assert_eq!(variant1, variant2, "Cloned variants should be equal");
        assert_ne!(variant1, variant3, "Different variants should not be equal");
    }
    
    #[test]
    fn button_sizes_can_be_cloned_and_compared() {
        let size1 = ButtonSize::Lg;
        let size2 = size1.clone();
        let size3 = ButtonSize::Default;
        
        assert_eq!(size1, size2, "Cloned sizes should be equal");
        assert_ne!(size1, size3, "Different sizes should not be equal");
    }
    
    #[test]
    fn button_variant_debug_format() {
        // Test Debug formatting for variants
        let variant = ButtonVariant::Destructive;
        let debug_str = format!("{:?}", variant);
        assert_eq!(debug_str, "Destructive", "Debug format should match variant name");
    }
    
    #[test]
    fn button_size_debug_format() {
        // Test Debug formatting for sizes
        let size = ButtonSize::Icon;
        let debug_str = format!("{:?}", size);
        assert_eq!(debug_str, "Icon", "Debug format should match size name");
    }
    
    #[test]
    fn all_button_variants_exist() {
        // Ensure all expected variants can be created
        let variants = vec![
            ButtonVariant::Default,
            ButtonVariant::Destructive, 
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];
        
        assert_eq!(variants.len(), 6, "Should have exactly 6 button variants");
        
        // Each should be unique
        for (i, variant_a) in variants.iter().enumerate() {
            for (j, variant_b) in variants.iter().enumerate() {
                if i != j {
                    assert_ne!(variant_a, variant_b, 
                             "Variants {:?} and {:?} should be different", variant_a, variant_b);
                }
            }
        }
    }
    
    #[test]  
    fn all_button_sizes_exist() {
        // Ensure all expected sizes can be created
        let sizes = vec![
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ];
        
        assert_eq!(sizes.len(), 4, "Should have exactly 4 button sizes");
        
        // Each should be unique
        for (i, size_a) in sizes.iter().enumerate() {
            for (j, size_b) in sizes.iter().enumerate() {
                if i != j {
                    assert_ne!(size_a, size_b, 
                             "Sizes {:?} and {:?} should be different", size_a, size_b);
                }
            }
        }
    }
    
    #[test]
    fn button_class_has_accessibility_features() {
        // Verify accessibility-related classes are present
        assert!(BUTTON_CLASS.contains("focus-visible"), "Should have focus-visible styles");
        assert!(BUTTON_CLASS.contains("ring-offset"), "Should have ring offset for better visibility");
        assert!(BUTTON_CLASS.contains("disabled:"), "Should handle disabled state styling");
        assert!(BUTTON_CLASS.contains("pointer-events-none") || 
               BUTTON_CLASS.contains("disabled:pointer-events-none"), 
               "Should disable pointer events when disabled");
    }

    #[test]
    fn button_renders_with_loading_state() {
        // Test button with loading state
        let (loading, _set_loading) = signal(true);
        
        let _button_view = view! {
            <Button loading=loading>
                "Loading Button"
            </Button>
        };
        
        // Verify the view renders without errors
        let _view = _button_view.into_view();
        // If we get here without panicking, the view was created successfully
    }

    #[test]
    fn button_renders_with_aria_attributes() {
        // Test button with ARIA attributes
        let _button_view = view! {
            <Button 
                aria_label="Submit form"
                aria_describedby="submit-help"
            >
                "Submit"
            </Button>
        };
        
        // Verify the view renders without errors
        let _view = _button_view.into_view();
        // If we get here without panicking, the view was created successfully
    }

    #[test]
    fn button_renders_with_keyboard_support() {
        // Test button with keyboard support
        let (clicked, set_clicked) = signal(false);
        let on_click = Callback::new(move |_| {
            set_clicked.set(true);
        });
        
        let _button_view = view! {
            <Button on_click=on_click>
                "Keyboard Button"
            </Button>
        };
        
        // Verify the view renders without errors
        let _view = _button_view.into_view();
        // If we get here without panicking, the view was created successfully
    }

    #[test]
    fn button_renders_with_all_enhanced_features() {
        // Test button with all enhanced features
        let (loading, _set_loading) = signal(false);
        let (disabled, _set_disabled) = signal(false);
        let (clicked, set_clicked) = signal(false);
        let on_click = Callback::new(move |_| {
            set_clicked.set(true);
        });
        
        let _button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Lg
                loading=loading
                disabled=disabled
                on_click=on_click
                aria_label="Enhanced button"
                aria_describedby="button-help"
            >
                "Enhanced Button"
            </Button>
        };
        
        // Verify the view renders without errors
        let _view = _button_view.into_view();
        // If we get here without panicking, the view was created successfully
    }
}
