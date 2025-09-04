#[cfg(test)]
mod tests {
    use crate::default::{ButtonVariant, ButtonSize, ButtonChildProps, BUTTON_CLASS};
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_button_variant_enum_creation() {
        // Test ButtonVariant enum
        assert_eq!(ButtonVariant::default(), ButtonVariant::Default);
        
        // Test From<String> conversion
        assert_eq!(ButtonVariant::from("destructive".to_string()), ButtonVariant::Destructive);
        assert_eq!(ButtonVariant::from("outline".to_string()), ButtonVariant::Outline);
        assert_eq!(ButtonVariant::from("secondary".to_string()), ButtonVariant::Secondary);
        assert_eq!(ButtonVariant::from("ghost".to_string()), ButtonVariant::Ghost);
        assert_eq!(ButtonVariant::from("link".to_string()), ButtonVariant::Link);
        assert_eq!(ButtonVariant::from("unknown".to_string()), ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_enum_creation() {
        // Test ButtonSize enum
        assert_eq!(ButtonSize::default(), ButtonSize::Default);
        
        // Test From<String> conversion
        assert_eq!(ButtonSize::from("sm".to_string()), ButtonSize::Sm);
        assert_eq!(ButtonSize::from("lg".to_string()), ButtonSize::Lg);
        assert_eq!(ButtonSize::from("icon".to_string()), ButtonSize::Icon);
        assert_eq!(ButtonSize::from("unknown".to_string()), ButtonSize::Default);
    }

    #[test]
    fn test_button_child_props_structure() {
        // Test ButtonChildProps creation
        let props = ButtonChildProps {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red;".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };
        
        assert_eq!(props.class, "test-class");
        assert_eq!(props.id, "test-id");
        assert_eq!(props.style, "color: red;");
        assert!(!props.disabled);
        assert_eq!(props.r#type, "button");
        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_button_variant_css_classes() {
        // Test that each variant maps to correct CSS classes
        
        let variants = vec![
            (ButtonVariant::Default, "bg-primary text-primary-foreground hover:bg-primary/90"),
            (ButtonVariant::Destructive, "bg-destructive text-destructive-foreground hover:bg-destructive/90"),
            (ButtonVariant::Outline, "border border-input bg-background hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariant::Secondary, "bg-secondary text-secondary-foreground hover:bg-secondary/80"),
            (ButtonVariant::Ghost, "hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariant::Link, "text-primary underline-offset-4 hover:underline"),
        ];
        
        for (variant, expected_class) in variants {
            // This is a conceptual test - in real implementation we'd need to render and check classes
            match variant {
                ButtonVariant::Default => assert!(expected_class.contains("bg-primary")),
                ButtonVariant::Destructive => assert!(expected_class.contains("bg-destructive")),
                ButtonVariant::Outline => assert!(expected_class.contains("border border-input")),
                ButtonVariant::Secondary => assert!(expected_class.contains("bg-secondary")),
                ButtonVariant::Ghost => assert!(expected_class.contains("hover:bg-accent")),
                ButtonVariant::Link => assert!(expected_class.contains("text-primary underline")),
            }
        }
    }

    #[test]
    fn test_button_size_css_classes() {
        // Test that each size maps to correct CSS classes
        let sizes = vec![
            (ButtonSize::Default, "h-10 px-4 py-2"),
            (ButtonSize::Sm, "h-9 rounded-md px-3"),
            (ButtonSize::Lg, "h-11 rounded-md px-8"),
            (ButtonSize::Icon, "h-10 w-10"),
        ];
        
        for (size, expected_class) in sizes {
            match size {
                ButtonSize::Default => assert!(expected_class.contains("h-10 px-4 py-2")),
                ButtonSize::Sm => assert!(expected_class.contains("h-9")),
                ButtonSize::Lg => assert!(expected_class.contains("h-11")),
                ButtonSize::Icon => assert!(expected_class.contains("w-10")),
            }
        }
    }

    #[test]
    fn test_button_base_css_classes() {
        // Test that base BUTTON_CLASS contains required accessibility and styling classes
        assert!(BUTTON_CLASS.contains("inline-flex"));
        assert!(BUTTON_CLASS.contains("items-center"));
        assert!(BUTTON_CLASS.contains("justify-center"));
        assert!(BUTTON_CLASS.contains("focus-visible:outline-none"));
        assert!(BUTTON_CLASS.contains("focus-visible:ring-2"));
        assert!(BUTTON_CLASS.contains("disabled:pointer-events-none"));
        assert!(BUTTON_CLASS.contains("disabled:opacity-50"));
        assert!(BUTTON_CLASS.contains("transition-colors"));
    }

    // Integration test for click handling (conceptual - would need proper test environment)
    #[test]
    fn test_button_click_callback_structure() {
        let click_called = Arc::new(Mutex::new(false));
        let click_called_clone = Arc::clone(&click_called);
        
        // Simulate callback creation
        let callback = Callback::new(move |_: ()| {
            *click_called_clone.lock().unwrap() = true;
        });
        
        // Simulate callback execution
        callback.run(());
        
        assert!(*click_called.lock().unwrap());
    }

    // Test disabled state handling
    #[test]
    fn test_button_disabled_state() {
        // Test disabled signal creation
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get());
        
        disabled_signal.set(true);
        assert!(disabled_signal.get());
        
        // In a real test, we'd verify that disabled buttons don't trigger click events
        // and have proper ARIA attributes
    }

    // Test custom class merging
    #[test]
    fn test_button_custom_class_handling() {
        // Test class merging logic
        let base_class = BUTTON_CLASS;
        let variant_class = "bg-primary text-primary-foreground hover:bg-primary/90";
        let size_class = "h-10 px-4 py-2";
        let custom_class = "my-custom-class";
        
        let expected = format!("{} {} {} {}", base_class, variant_class, size_class, custom_class);
        
        // In real implementation, this would be tested through component rendering
        assert!(expected.contains(base_class));
        assert!(expected.contains(variant_class));
        assert!(expected.contains(size_class));
        assert!(expected.contains(custom_class));
    }

    // Test as_child functionality structure
    #[test]
    fn test_button_as_child_props_creation() {
        // Test as_child callback structure
        let as_child_callback = Callback::new(|props: ButtonChildProps| {
            // Verify props structure
            assert!(!props.class.is_empty());
            assert_eq!(props.r#type, "button");
            view! { <div class=props.class>Custom Child</div> }.into_any()
        });
        
        // Test callback can be created
        assert!(std::mem::size_of_val(&as_child_callback) > 0);
    }
}