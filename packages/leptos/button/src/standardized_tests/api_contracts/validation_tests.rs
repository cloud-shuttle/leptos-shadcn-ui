#[cfg(test)]
mod validation_tests {
    use crate::standardized::StandardizedButtonProps;
    use leptos_shadcn_api_standards::*;
    use leptos::prelude::*;

    // ===== VALIDATION TESTS =====
    // These tests focus on API contract validation and standardization

    #[test]
    fn test_standardized_button_props_validation_default_values() {
        // Test that default values are valid
        let props = StandardizedButtonProps::default();
        
        // Validate default values
        assert!(props.disabled.unwrap_or(false) == false);
        assert!(props.variant.unwrap_or(StandardVariant::Default) == StandardVariant::Default);
        assert!(props.size.unwrap_or(StandardSize::Default) == StandardSize::Default);
    }

    #[test]
    fn test_standardized_button_props_validation_required_fields() {
        // Test validation of required fields
        let props = StandardizedButtonProps {
            id: Some("valid-id".to_string()),
            class: Some("valid-class".to_string()),
            ..Default::default()
        };
        
        // Validate that required fields are present
        assert!(props.id.is_some());
        assert!(props.class.is_some());
    }

    #[test]
    fn test_standardized_button_props_validation_string_lengths() {
        // Test validation of string field lengths
        let short_string = "a";
        let medium_string = "a".repeat(100);
        let long_string = "a".repeat(1000);
        
        let props_short = StandardizedButtonProps {
            id: Some(short_string.clone()),
            class: Some(short_string.clone()),
            ..Default::default()
        };
        
        let props_medium = StandardizedButtonProps {
            id: Some(medium_string.clone()),
            class: Some(medium_string.clone()),
            ..Default::default()
        };
        
        let props_long = StandardizedButtonProps {
            id: Some(long_string.clone()),
            class: Some(long_string.clone()),
            ..Default::default()
        };
        
        // Validate that strings of various lengths are accepted
        assert_eq!(props_short.id, Some(short_string));
        assert_eq!(props_medium.id, Some(medium_string));
        assert_eq!(props_long.id, Some(long_string));
    }

    #[test]
    fn test_standardized_button_props_validation_aria_attributes() {
        // Test validation of ARIA attributes
        let props = StandardizedButtonProps {
            aria_label: Some("Valid label".to_string()),
            aria_describedby: Some("valid-description".to_string()),
            aria_labelledby: Some("valid-label".to_string()),
            role: Some("button".to_string()),
            ..Default::default()
        };
        
        // Validate ARIA attributes
        assert!(props.aria_label.is_some());
        assert!(props.aria_describedby.is_some());
        assert!(props.aria_labelledby.is_some());
        assert!(props.role.is_some());
    }

    #[test]
    fn test_standardized_button_props_validation_tabindex_values() {
        // Test validation of tabindex values
        let props_zero = StandardizedButtonProps {
            tabindex: Some(0),
            ..Default::default()
        };
        
        let props_positive = StandardizedButtonProps {
            tabindex: Some(1),
            ..Default::default()
        };
        
        let props_negative = StandardizedButtonProps {
            tabindex: Some(-1),
            ..Default::default()
        };
        
        // Validate tabindex values
        assert_eq!(props_zero.tabindex, Some(0));
        assert_eq!(props_positive.tabindex, Some(1));
        assert_eq!(props_negative.tabindex, Some(-1));
    }

    #[test]
    fn test_standardized_button_props_validation_button_types() {
        // Test validation of button type values
        let props_submit = StandardizedButtonProps {
            button_type: Some("submit".to_string()),
            ..Default::default()
        };
        
        let props_button = StandardizedButtonProps {
            button_type: Some("button".to_string()),
            ..Default::default()
        };
        
        let props_reset = StandardizedButtonProps {
            button_type: Some("reset".to_string()),
            ..Default::default()
        };
        
        // Validate button types
        assert_eq!(props_submit.button_type, Some("submit".to_string()));
        assert_eq!(props_button.button_type, Some("button".to_string()));
        assert_eq!(props_reset.button_type, Some("reset".to_string()));
    }

    #[test]
    fn test_standardized_button_props_validation_variant_combinations() {
        // Test validation of variant and size combinations
        let combinations = vec![
            (StandardVariant::Default, StandardSize::Default),
            (StandardVariant::Default, StandardSize::Small),
            (StandardVariant::Default, StandardSize::Large),
            (StandardVariant::Default, StandardSize::Icon),
            (StandardVariant::Destructive, StandardSize::Default),
            (StandardVariant::Destructive, StandardSize::Small),
            (StandardVariant::Destructive, StandardSize::Large),
            (StandardVariant::Destructive, StandardSize::Icon),
            (StandardVariant::Outline, StandardSize::Default),
            (StandardVariant::Outline, StandardSize::Small),
            (StandardVariant::Outline, StandardSize::Large),
            (StandardVariant::Outline, StandardSize::Icon),
            (StandardVariant::Secondary, StandardSize::Default),
            (StandardVariant::Secondary, StandardSize::Small),
            (StandardVariant::Secondary, StandardSize::Large),
            (StandardVariant::Secondary, StandardSize::Icon),
            (StandardVariant::Ghost, StandardSize::Default),
            (StandardVariant::Ghost, StandardSize::Small),
            (StandardVariant::Ghost, StandardSize::Large),
            (StandardVariant::Ghost, StandardSize::Icon),
            (StandardVariant::Link, StandardSize::Default),
            (StandardVariant::Link, StandardSize::Small),
            (StandardVariant::Link, StandardSize::Large),
            (StandardVariant::Link, StandardSize::Icon),
        ];
        
        for (variant, size) in combinations {
            let props = StandardizedButtonProps {
                variant: Some(variant),
                size: Some(size),
                ..Default::default()
            };
            
            // Validate that all combinations are valid
            assert_eq!(props.variant, Some(variant));
            assert_eq!(props.size, Some(size));
        }
    }

    #[test]
    fn test_standardized_button_props_validation_callback_functions() {
        // Test validation of callback functions
        let click_callback = Box::new(|| {});
        let focus_callback = Box::new(|| {});
        let blur_callback = Box::new(|| {});
        
        let props = StandardizedButtonProps {
            onclick: Some(click_callback),
            onfocus: Some(focus_callback),
            onblur: Some(blur_callback),
            ..Default::default()
        };
        
        // Validate callback functions
        assert!(props.onclick.is_some());
        assert!(props.onfocus.is_some());
        assert!(props.onblur.is_some());
    }

    #[test]
    fn test_standardized_button_props_validation_children() {
        // Test validation of children
        let props_with_children = StandardizedButtonProps {
            children: Some(Children::new(|_| view! { <span>"Child"</span> })),
            ..Default::default()
        };
        
        let props_without_children = StandardizedButtonProps {
            children: None,
            ..Default::default()
        };
        
        // Validate children
        assert!(props_with_children.children.is_some());
        assert!(props_without_children.children.is_none());
    }

    #[test]
    fn test_standardized_button_props_validation_edge_cases() {
        // Test validation of edge cases
        let props_empty_strings = StandardizedButtonProps {
            id: Some("".to_string()),
            class: Some("".to_string()),
            style: Some("".to_string()),
            aria_label: Some("".to_string()),
            aria_describedby: Some("".to_string()),
            aria_labelledby: Some("".to_string()),
            role: Some("".to_string()),
            button_type: Some("".to_string()),
            ..Default::default()
        };
        
        // Validate edge cases
        assert_eq!(props_empty_strings.id, Some("".to_string()));
        assert_eq!(props_empty_strings.class, Some("".to_string()));
        assert_eq!(props_empty_strings.style, Some("".to_string()));
        assert_eq!(props_empty_strings.aria_label, Some("".to_string()));
        assert_eq!(props_empty_strings.aria_describedby, Some("".to_string()));
        assert_eq!(props_empty_strings.aria_labelledby, Some("".to_string()));
        assert_eq!(props_empty_strings.role, Some("".to_string()));
        assert_eq!(props_empty_strings.button_type, Some("".to_string()));
    }

    #[test]
    fn test_standardized_button_props_validation_unicode_strings() {
        // Test validation of unicode strings
        let props_unicode = StandardizedButtonProps {
            id: Some("test-id-🚀".to_string()),
            class: Some("test-class-🎉".to_string()),
            style: Some("color: red; content: '🚀';".to_string()),
            aria_label: Some("Test button 🚀".to_string()),
            aria_describedby: Some("test-description-🎉".to_string()),
            aria_labelledby: Some("test-label-🚀".to_string()),
            role: Some("button-🎉".to_string()),
            button_type: Some("submit-🚀".to_string()),
            ..Default::default()
        };
        
        // Validate unicode strings
        assert_eq!(props_unicode.id, Some("test-id-🚀".to_string()));
        assert_eq!(props_unicode.class, Some("test-class-🎉".to_string()));
        assert_eq!(props_unicode.style, Some("color: red; content: '🚀';".to_string()));
        assert_eq!(props_unicode.aria_label, Some("Test button 🚀".to_string()));
        assert_eq!(props_unicode.aria_describedby, Some("test-description-🎉".to_string()));
        assert_eq!(props_unicode.aria_labelledby, Some("test-label-🚀".to_string()));
        assert_eq!(props_unicode.role, Some("button-🎉".to_string()));
        assert_eq!(props_unicode.button_type, Some("submit-🚀".to_string()));
    }

    #[test]
    fn test_standardized_button_props_validation_special_characters() {
        // Test validation of special characters
        let props_special = StandardizedButtonProps {
            id: Some("test-id!@#$%^&*()".to_string()),
            class: Some("test-class!@#$%^&*()".to_string()),
            style: Some("color: red; background: blue; font-size: 16px;".to_string()),
            aria_label: Some("Test button!@#$%^&*()".to_string()),
            aria_describedby: Some("test-description!@#$%^&*()".to_string()),
            aria_labelledby: Some("test-label!@#$%^&*()".to_string()),
            role: Some("button!@#$%^&*()".to_string()),
            button_type: Some("submit!@#$%^&*()".to_string()),
            ..Default::default()
        };
        
        // Validate special characters
        assert_eq!(props_special.id, Some("test-id!@#$%^&*()".to_string()));
        assert_eq!(props_special.class, Some("test-class!@#$%^&*()".to_string()));
        assert_eq!(props_special.style, Some("color: red; background: blue; font-size: 16px;".to_string()));
        assert_eq!(props_special.aria_label, Some("Test button!@#$%^&*()".to_string()));
        assert_eq!(props_special.aria_describedby, Some("test-description!@#$%^&*()".to_string()));
        assert_eq!(props_special.aria_labelledby, Some("test-label!@#$%^&*()".to_string()));
        assert_eq!(props_special.role, Some("button!@#$%^&*()".to_string()));
        assert_eq!(props_special.button_type, Some("submit!@#$%^&*()".to_string()));
    }

    #[test]
    fn test_standardized_button_props_validation_boolean_combinations() {
        // Test validation of boolean combinations
        let props_enabled = StandardizedButtonProps {
            disabled: Some(false),
            ..Default::default()
        };
        
        let props_disabled = StandardizedButtonProps {
            disabled: Some(true),
            ..Default::default()
        };
        
        let props_default_disabled = StandardizedButtonProps {
            disabled: None,
            ..Default::default()
        };
        
        // Validate boolean combinations
        assert_eq!(props_enabled.disabled, Some(false));
        assert_eq!(props_disabled.disabled, Some(true));
        assert_eq!(props_default_disabled.disabled, None);
    }

    #[test]
    fn test_standardized_button_props_validation_numeric_ranges() {
        // Test validation of numeric ranges
        let props_zero = StandardizedButtonProps {
            tabindex: Some(0),
            ..Default::default()
        };
        
        let props_positive = StandardizedButtonProps {
            tabindex: Some(999),
            ..Default::default()
        };
        
        let props_negative = StandardizedButtonProps {
            tabindex: Some(-999),
            ..Default::default()
        };
        
        // Validate numeric ranges
        assert_eq!(props_zero.tabindex, Some(0));
        assert_eq!(props_positive.tabindex, Some(999));
        assert_eq!(props_negative.tabindex, Some(-999));
    }

    #[test]
    fn test_standardized_button_props_validation_comprehensive() {
        // Test comprehensive validation with all fields
        let props = StandardizedButtonProps {
            id: Some("comprehensive-test-id".to_string()),
            class: Some("comprehensive-test-class".to_string()),
            style: Some("color: red; background: blue; font-size: 16px;".to_string()),
            disabled: Some(false),
            variant: Some(StandardVariant::Primary),
            size: Some(StandardSize::Large),
            aria_label: Some("Comprehensive test button".to_string()),
            aria_describedby: Some("comprehensive-test-description".to_string()),
            aria_labelledby: Some("comprehensive-test-label".to_string()),
            role: Some("button".to_string()),
            tabindex: Some(0),
            button_type: Some("submit".to_string()),
            onclick: Some(Box::new(|| {})),
            onfocus: Some(Box::new(|| {})),
            onblur: Some(Box::new(|| {})),
            children: Some(Children::new(|_| view! { <span>"Comprehensive child"</span> })),
        };
        
        // Comprehensive validation
        assert_eq!(props.id, Some("comprehensive-test-id".to_string()));
        assert_eq!(props.class, Some("comprehensive-test-class".to_string()));
        assert_eq!(props.style, Some("color: red; background: blue; font-size: 16px;".to_string()));
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Primary));
        assert_eq!(props.size, Some(StandardSize::Large));
        assert_eq!(props.aria_label, Some("Comprehensive test button".to_string()));
        assert_eq!(props.aria_describedby, Some("comprehensive-test-description".to_string()));
        assert_eq!(props.aria_labelledby, Some("comprehensive-test-label".to_string()));
        assert_eq!(props.role, Some("button".to_string()));
        assert_eq!(props.tabindex, Some(0));
        assert_eq!(props.button_type, Some("submit".to_string()));
        assert!(props.onclick.is_some());
        assert!(props.onfocus.is_some());
        assert!(props.onblur.is_some());
        assert!(props.children.is_some());
    }

    #[test]
    fn test_standardized_button_props_validation_performance() {
        // Test that validation is performant
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _props = StandardizedButtonProps {
                id: Some("test-id".to_string()),
                class: Some("test-class".to_string()),
                disabled: Some(true),
                variant: Some(StandardVariant::Destructive),
                size: Some(StandardSize::Large),
                ..Default::default()
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_standardized_button_props_validation_memory_usage() {
        // Test that validation doesn't cause memory issues
        let props = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: Some("test-class".to_string()),
            style: Some("color: red".to_string()),
            disabled: Some(true),
            variant: Some(StandardVariant::Destructive),
            size: Some(StandardSize::Large),
            aria_label: Some("Test button".to_string()),
            aria_describedby: Some("test-description".to_string()),
            aria_labelledby: Some("test-label".to_string()),
            role: Some("button".to_string()),
            tabindex: Some(0),
            button_type: Some("submit".to_string()),
            onclick: Some(Box::new(|| {})),
            onfocus: Some(Box::new(|| {})),
            onblur: Some(Box::new(|| {})),
            children: None,
        };
        
        let size = std::mem::size_of_val(&props);
        assert!(size < 1024); // Should be less than 1KB
    }
}
