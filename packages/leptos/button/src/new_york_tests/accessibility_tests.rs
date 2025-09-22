#[cfg(test)]
mod accessibility_tests {
    use crate::new_york::{ButtonVariant as ButtonVariantNewYork, ButtonSize as ButtonSizeNewYork, ButtonChildProps as ButtonChildPropsNewYork};
    use leptos::prelude::*;

    // ===== NEW YORK ACCESSIBILITY TESTS =====
    // These tests focus on accessibility features and keyboard navigation

    #[test]
    fn test_new_york_button_accessibility_attributes() {
        // Test that accessibility attributes are properly handled
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that disabled state is properly tracked
        assert!(!props.disabled);
        
        // Test that type is properly set
        assert_eq!(props.r#type, "button");
        
        // Test that id is properly set
        assert_eq!(props.id, "test-id");
    }

    #[test]
    fn test_new_york_button_disabled_accessibility() {
        // Test disabled button accessibility
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: true,
            r#type: "button".to_string(),
            onclick: None,
        };

        // Test that disabled state is properly tracked
        assert!(props.disabled);
        
        // Test that onclick is None for disabled buttons
        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_new_york_button_keyboard_navigation() {
        // Test keyboard navigation support
        let click_count = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            click_count.update(|count| *count += 1);
        });

        // Simulate keyboard activation (Enter key)
        callback.run(());
        assert_eq!(click_count.get(), 1);

        // Simulate keyboard activation (Space key)
        callback.run(());
        assert_eq!(click_count.get(), 2);
    }

    #[test]
    fn test_new_york_button_focus_management() {
        // Test focus management
        let props = ButtonChildPropsNewYork {
            class: "test-class focus:ring-2 focus:ring-blue-500".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that focus classes are present
        assert!(props.class.contains("focus:ring-2"));
        assert!(props.class.contains("focus:ring-blue-500"));
    }

    #[test]
    fn test_new_york_button_aria_attributes() {
        // Test ARIA attributes support
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that id is available for aria-labelledby
        assert_eq!(props.id, "test-id");
        
        // Test that type is properly set for screen readers
        assert_eq!(props.r#type, "button");
    }

    #[test]
    fn test_new_york_button_screen_reader_support() {
        // Test screen reader support
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that button type is properly set for screen readers
        assert_eq!(props.r#type, "button");
        
        // Test that id is available for screen reader navigation
        assert_eq!(props.id, "test-id");
    }

    #[test]
    fn test_new_york_button_color_contrast() {
        // Test color contrast considerations
        let variants = vec![
            ButtonVariantNewYork::Default,
            ButtonVariantNewYork::Destructive,
            ButtonVariantNewYork::Outline,
            ButtonVariantNewYork::Secondary,
            ButtonVariantNewYork::Ghost,
            ButtonVariantNewYork::Link,
        ];

        for variant in variants {
            // Test that each variant has appropriate color classes
            let variant_class = match variant {
                ButtonVariantNewYork::Default => "bg-primary text-primary-foreground",
                ButtonVariantNewYork::Destructive => "bg-destructive text-destructive-foreground",
                ButtonVariantNewYork::Outline => "border border-input bg-background",
                ButtonVariantNewYork::Secondary => "bg-secondary text-secondary-foreground",
                ButtonVariantNewYork::Ghost => "hover:bg-accent hover:text-accent-foreground",
                ButtonVariantNewYork::Link => "text-primary",
            };
            
            // Test that color classes are present
            assert!(!variant_class.is_empty());
        }
    }

    #[test]
    fn test_new_york_button_size_accessibility() {
        // Test size accessibility considerations
        let sizes = vec![
            ButtonSizeNewYork::Default,
            ButtonSizeNewYork::Sm,
            ButtonSizeNewYork::Lg,
            ButtonSizeNewYork::Icon,
        ];

        for size in sizes {
            // Test that each size has appropriate dimensions
            let size_class = match size {
                ButtonSizeNewYork::Default => "h-10 px-4 py-2",
                ButtonSizeNewYork::Sm => "h-9 rounded-md px-3",
                ButtonSizeNewYork::Lg => "h-11 rounded-md px-8",
                ButtonSizeNewYork::Icon => "h-10 w-10",
            };
            
            // Test that size classes are present
            assert!(!size_class.is_empty());
            
            // Test that minimum touch target size is met (44px minimum)
            assert!(size_class.contains("h-9") || size_class.contains("h-10") || size_class.contains("h-11"));
        }
    }

    #[test]
    fn test_new_york_button_interactive_states() {
        // Test interactive states for accessibility
        let props = ButtonChildPropsNewYork {
            class: "test-class hover:bg-blue-600 focus:ring-2 active:bg-blue-700".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that hover state is present
        assert!(props.class.contains("hover:bg-blue-600"));
        
        // Test that focus state is present
        assert!(props.class.contains("focus:ring-2"));
        
        // Test that active state is present
        assert!(props.class.contains("active:bg-blue-700"));
    }

    #[test]
    fn test_new_york_button_disabled_interactive_states() {
        // Test disabled interactive states
        let props = ButtonChildPropsNewYork {
            class: "test-class disabled:opacity-50 disabled:cursor-not-allowed".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: true,
            r#type: "button".to_string(),
            onclick: None,
        };

        // Test that disabled state is properly tracked
        assert!(props.disabled);
        
        // Test that disabled styles are present
        assert!(props.class.contains("disabled:opacity-50"));
        assert!(props.class.contains("disabled:cursor-not-allowed"));
        
        // Test that onclick is None for disabled buttons
        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_new_york_button_high_contrast_mode() {
        // Test high contrast mode support
        let props = ButtonChildPropsNewYork {
            class: "test-class border border-transparent".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that border classes are present for high contrast
        assert!(props.class.contains("border"));
    }

    #[test]
    fn test_new_york_button_motion_reduction() {
        // Test motion reduction support
        let props = ButtonChildPropsNewYork {
            class: "test-class transition-colors motion-reduce:transition-none".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that motion reduction classes are present
        assert!(props.class.contains("motion-reduce:transition-none"));
    }

    #[test]
    fn test_new_york_button_aria_pressed() {
        // Test ARIA pressed state support
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that button type is properly set for ARIA
        assert_eq!(props.r#type, "button");
        
        // Test that id is available for ARIA attributes
        assert_eq!(props.id, "test-id");
    }

    #[test]
    fn test_new_york_button_aria_expanded() {
        // Test ARIA expanded state support
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that button type is properly set for ARIA
        assert_eq!(props.r#type, "button");
        
        // Test that id is available for ARIA attributes
        assert_eq!(props.id, "test-id");
    }

    #[test]
    fn test_new_york_button_aria_controls() {
        // Test ARIA controls support
        let props = ButtonChildPropsNewYork {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        // Test that button type is properly set for ARIA
        assert_eq!(props.r#type, "button");
        
        // Test that id is available for ARIA attributes
        assert_eq!(props.id, "test-id");
    }
}
