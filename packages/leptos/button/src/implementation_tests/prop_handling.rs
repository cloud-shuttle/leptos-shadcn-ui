#[cfg(test)]
mod prop_handling {
    use crate::default::{ButtonChildProps, BUTTON_CLASS};
    use leptos::prelude::*;

    // ===== PROP HANDLING TESTS =====
    // These tests focus on prop validation, combination, and edge case handling

    #[test]
    fn test_button_child_props_creation() {
        // Test ButtonChildProps creation and field access
        let props = ButtonChildProps {
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
    fn test_button_child_props_clone() {
        // Test ButtonChildProps Clone implementation
        let props = ButtonChildProps {
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
    fn test_button_child_props_debug() {
        // Test ButtonChildProps Debug implementation
        let props = ButtonChildProps {
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
    fn test_button_child_props_empty_strings() {
        // Test ButtonChildProps with empty strings
        let props = ButtonChildProps {
            class: "".to_string(),
            id: "".to_string(),
            style: "".to_string(),
            disabled: false,
            r#type: "".to_string(),
            onclick: None,
        };

        assert_eq!(props.class, "");
        assert_eq!(props.id, "");
        assert_eq!(props.style, "");
        assert_eq!(props.r#type, "");
        assert!(!props.disabled);
        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_button_child_props_long_strings() {
        // Test ButtonChildProps with long strings
        let long_class = "a".repeat(1000);
        let long_id = "b".repeat(1000);
        let long_style = "c".repeat(1000);
        let long_type = "d".repeat(1000);

        let props = ButtonChildProps {
            class: long_class.clone(),
            id: long_id.clone(),
            style: long_style.clone(),
            disabled: true,
            r#type: long_type.clone(),
            onclick: Some(Callback::new(|_| {})),
        };

        assert_eq!(props.class, long_class);
        assert_eq!(props.id, long_id);
        assert_eq!(props.style, long_style);
        assert_eq!(props.r#type, long_type);
        assert!(props.disabled);
        assert!(props.onclick.is_some());
    }

    #[test]
    fn test_button_child_props_special_characters() {
        // Test ButtonChildProps with special characters
        let props = ButtonChildProps {
            class: "class-with-special-chars!@#$%^&*()".to_string(),
            id: "id-with-special-chars!@#$%^&*()".to_string(),
            style: "color: red; background: blue; font-size: 12px;".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };

        assert_eq!(props.class, "class-with-special-chars!@#$%^&*()");
        assert_eq!(props.id, "id-with-special-chars!@#$%^&*()");
        assert_eq!(props.style, "color: red; background: blue; font-size: 12px;");
        assert_eq!(props.r#type, "button");
        assert!(!props.disabled);
        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_button_child_props_unicode() {
        // Test ButtonChildProps with unicode characters
        let props = ButtonChildProps {
            class: "class-with-unicode-🚀-🎉".to_string(),
            id: "id-with-unicode-🚀-🎉".to_string(),
            style: "color: red; content: '🚀';".to_string(),
            disabled: true,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(|_| {})),
        };

        assert_eq!(props.class, "class-with-unicode-🚀-🎉");
        assert_eq!(props.id, "id-with-unicode-🚀-🎉");
        assert_eq!(props.style, "color: red; content: '🚀';");
        assert_eq!(props.r#type, "button");
        assert!(props.disabled);
        assert!(props.onclick.is_some());
    }

    #[test]
    fn test_button_child_props_callback_handling() {
        // Test ButtonChildProps callback handling
        let click_count = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            click_count.update(|count| *count += 1);
        });

        let props = ButtonChildProps {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: Some(callback),
        };

        // Test callback execution
        if let Some(cb) = &props.onclick {
            cb.run(());
            assert_eq!(click_count.get(), 1);
            
            cb.run(());
            assert_eq!(click_count.get(), 2);
        }
    }

    #[test]
    fn test_button_child_props_callback_none() {
        // Test ButtonChildProps with no callback
        let props = ButtonChildProps {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };

        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_button_child_props_type_variations() {
        // Test ButtonChildProps with different type values
        let types = vec!["button", "submit", "reset"];

        for button_type in types {
            let props = ButtonChildProps {
                class: "test-class".to_string(),
                id: "test-id".to_string(),
                style: "color: red".to_string(),
                disabled: false,
                r#type: button_type.to_string(),
                onclick: None,
            };

            assert_eq!(props.r#type, button_type);
        }
    }

    #[test]
    fn test_button_child_props_disabled_variations() {
        // Test ButtonChildProps with different disabled states
        let disabled_states = vec![true, false];

        for disabled in disabled_states {
            let props = ButtonChildProps {
                class: "test-class".to_string(),
                id: "test-id".to_string(),
                style: "color: red".to_string(),
                disabled,
                r#type: "button".to_string(),
                onclick: None,
            };

            assert_eq!(props.disabled, disabled);
        }
    }

    #[test]
    fn test_button_class_constant() {
        // Test BUTTON_CLASS constant
        assert!(BUTTON_CLASS.contains("inline-flex"));
        assert!(BUTTON_CLASS.contains("items-center"));
        assert!(BUTTON_CLASS.contains("justify-center"));
        assert!(BUTTON_CLASS.contains("rounded-md"));
        assert!(BUTTON_CLASS.contains("transition-colors"));
    }

    #[test]
    fn test_button_class_constant_not_empty() {
        // Test that BUTTON_CLASS constant is not empty
        assert!(!BUTTON_CLASS.is_empty());
        assert!(BUTTON_CLASS.len() > 0);
    }

    #[test]
    fn test_button_class_constant_contains_expected_classes() {
        // Test that BUTTON_CLASS contains expected CSS classes
        let expected_classes = vec![
            "inline-flex",
            "items-center",
            "justify-center",
            "rounded-md",
            "transition-colors",
        ];

        for expected_class in expected_classes {
            assert!(BUTTON_CLASS.contains(expected_class), 
                "BUTTON_CLASS should contain '{}'", expected_class);
        }
    }

    #[test]
    fn test_button_child_props_equality() {
        // Test ButtonChildProps equality
        let props1 = ButtonChildProps {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };

        let props2 = ButtonChildProps {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };

        assert_eq!(props1, props2);
    }

    #[test]
    fn test_button_child_props_inequality() {
        // Test ButtonChildProps inequality
        let props1 = ButtonChildProps {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };

        let props2 = ButtonChildProps {
            class: "different-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };

        assert_ne!(props1, props2);
    }
}
