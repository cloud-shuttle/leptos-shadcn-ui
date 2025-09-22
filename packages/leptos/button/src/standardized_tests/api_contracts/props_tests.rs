#[cfg(test)]
mod props_tests {
    use crate::standardized::StandardizedButtonProps;
    use leptos_shadcn_api_standards::*;
    use leptos::prelude::*;

    // ===== PROPS TESTS =====
    // These tests focus on props validation and creation

    #[test]
    fn test_standardized_button_props_default() {
        // Test Default implementation for StandardizedButtonProps
        let props = StandardizedButtonProps::default();
        
        assert_eq!(props.id, None);
        assert_eq!(props.class, None);
        assert_eq!(props.style, None);
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Default));
        assert_eq!(props.size, Some(StandardSize::Default));
        assert_eq!(props.aria_label, None);
        assert_eq!(props.aria_describedby, None);
        assert_eq!(props.aria_labelledby, None);
        assert_eq!(props.role, None);
        assert_eq!(props.tabindex, None);
        assert_eq!(props.button_type, None);
        assert_eq!(props.onclick, None);
        assert_eq!(props.onfocus, None);
        assert_eq!(props.onblur, None);
        assert_eq!(props.children, None);
    }

    #[test]
    fn test_standardized_button_props_creation() {
        // Test StandardizedButtonProps creation with all fields
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
        
        assert_eq!(props.id, Some("test-id".to_string()));
        assert_eq!(props.class, Some("test-class".to_string()));
        assert_eq!(props.style, Some("color: red".to_string()));
        assert_eq!(props.disabled, Some(true));
        assert_eq!(props.variant, Some(StandardVariant::Destructive));
        assert_eq!(props.size, Some(StandardSize::Large));
        assert_eq!(props.aria_label, Some("Test button".to_string()));
        assert_eq!(props.aria_describedby, Some("test-description".to_string()));
        assert_eq!(props.aria_labelledby, Some("test-label".to_string()));
        assert_eq!(props.role, Some("button".to_string()));
        assert_eq!(props.tabindex, Some(0));
        assert_eq!(props.button_type, Some("submit".to_string()));
        assert!(props.onclick.is_some());
        assert!(props.onfocus.is_some());
        assert!(props.onblur.is_some());
        assert_eq!(props.children, None);
    }

    #[test]
    fn test_standardized_button_props_partial_creation() {
        // Test StandardizedButtonProps creation with partial fields
        let props = StandardizedButtonProps {
            id: Some("partial-id".to_string()),
            class: Some("partial-class".to_string()),
            disabled: Some(false),
            variant: Some(StandardVariant::Secondary),
            size: Some(StandardSize::Small),
            ..Default::default()
        };
        
        assert_eq!(props.id, Some("partial-id".to_string()));
        assert_eq!(props.class, Some("partial-class".to_string()));
        assert_eq!(props.style, None);
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Secondary));
        assert_eq!(props.size, Some(StandardSize::Small));
        assert_eq!(props.aria_label, None);
        assert_eq!(props.aria_describedby, None);
        assert_eq!(props.aria_labelledby, None);
        assert_eq!(props.role, None);
        assert_eq!(props.tabindex, None);
        assert_eq!(props.button_type, None);
        assert_eq!(props.onclick, None);
        assert_eq!(props.onfocus, None);
        assert_eq!(props.onblur, None);
        assert_eq!(props.children, None);
    }

    #[test]
    fn test_standardized_button_props_with_none_values() {
        // Test StandardizedButtonProps with explicit None values
        let props = StandardizedButtonProps {
            id: None,
            class: None,
            style: None,
            disabled: None,
            variant: None,
            size: None,
            aria_label: None,
            aria_describedby: None,
            aria_labelledby: None,
            role: None,
            tabindex: None,
            button_type: None,
            onclick: None,
            onfocus: None,
            onblur: None,
            children: None,
        };
        
        assert_eq!(props.id, None);
        assert_eq!(props.class, None);
        assert_eq!(props.style, None);
        assert_eq!(props.disabled, None);
        assert_eq!(props.variant, None);
        assert_eq!(props.size, None);
        assert_eq!(props.aria_label, None);
        assert_eq!(props.aria_describedby, None);
        assert_eq!(props.aria_labelledby, None);
        assert_eq!(props.role, None);
        assert_eq!(props.tabindex, None);
        assert_eq!(props.button_type, None);
        assert_eq!(props.onclick, None);
        assert_eq!(props.onfocus, None);
        assert_eq!(props.onblur, None);
        assert_eq!(props.children, None);
    }

    #[test]
    fn test_standardized_button_props_with_empty_strings() {
        // Test StandardizedButtonProps with empty string values
        let props = StandardizedButtonProps {
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
        
        assert_eq!(props.id, Some("".to_string()));
        assert_eq!(props.class, Some("".to_string()));
        assert_eq!(props.style, Some("".to_string()));
        assert_eq!(props.aria_label, Some("".to_string()));
        assert_eq!(props.aria_describedby, Some("".to_string()));
        assert_eq!(props.aria_labelledby, Some("".to_string()));
        assert_eq!(props.role, Some("".to_string()));
        assert_eq!(props.button_type, Some("".to_string()));
    }

    #[test]
    fn test_standardized_button_props_with_whitespace() {
        // Test StandardizedButtonProps with whitespace values
        let props = StandardizedButtonProps {
            id: Some("  ".to_string()),
            class: Some("  ".to_string()),
            style: Some("  ".to_string()),
            aria_label: Some("  ".to_string()),
            aria_describedby: Some("  ".to_string()),
            aria_labelledby: Some("  ".to_string()),
            role: Some("  ".to_string()),
            button_type: Some("  ".to_string()),
            ..Default::default()
        };
        
        assert_eq!(props.id, Some("  ".to_string()));
        assert_eq!(props.class, Some("  ".to_string()));
        assert_eq!(props.style, Some("  ".to_string()));
        assert_eq!(props.aria_label, Some("  ".to_string()));
        assert_eq!(props.aria_describedby, Some("  ".to_string()));
        assert_eq!(props.aria_labelledby, Some("  ".to_string()));
        assert_eq!(props.role, Some("  ".to_string()));
        assert_eq!(props.button_type, Some("  ".to_string()));
    }

    #[test]
    fn test_standardized_button_props_with_special_characters() {
        // Test StandardizedButtonProps with special characters
        let props = StandardizedButtonProps {
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
        
        assert_eq!(props.id, Some("test-id!@#$%^&*()".to_string()));
        assert_eq!(props.class, Some("test-class!@#$%^&*()".to_string()));
        assert_eq!(props.style, Some("color: red; background: blue; font-size: 16px;".to_string()));
        assert_eq!(props.aria_label, Some("Test button!@#$%^&*()".to_string()));
        assert_eq!(props.aria_describedby, Some("test-description!@#$%^&*()".to_string()));
        assert_eq!(props.aria_labelledby, Some("test-label!@#$%^&*()".to_string()));
        assert_eq!(props.role, Some("button!@#$%^&*()".to_string()));
        assert_eq!(props.button_type, Some("submit!@#$%^&*()".to_string()));
    }

    #[test]
    fn test_standardized_button_props_with_unicode() {
        // Test StandardizedButtonProps with unicode characters
        let props = StandardizedButtonProps {
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
        
        assert_eq!(props.id, Some("test-id-🚀".to_string()));
        assert_eq!(props.class, Some("test-class-🎉".to_string()));
        assert_eq!(props.style, Some("color: red; content: '🚀';".to_string()));
        assert_eq!(props.aria_label, Some("Test button 🚀".to_string()));
        assert_eq!(props.aria_describedby, Some("test-description-🎉".to_string()));
        assert_eq!(props.aria_labelledby, Some("test-label-🚀".to_string()));
        assert_eq!(props.role, Some("button-🎉".to_string()));
        assert_eq!(props.button_type, Some("submit-🚀".to_string()));
    }

    #[test]
    fn test_standardized_button_props_with_long_strings() {
        // Test StandardizedButtonProps with long strings
        let long_string = "a".repeat(1000);
        let props = StandardizedButtonProps {
            id: Some(long_string.clone()),
            class: Some(long_string.clone()),
            style: Some(long_string.clone()),
            aria_label: Some(long_string.clone()),
            aria_describedby: Some(long_string.clone()),
            aria_labelledby: Some(long_string.clone()),
            role: Some(long_string.clone()),
            button_type: Some(long_string.clone()),
            ..Default::default()
        };
        
        assert_eq!(props.id, Some(long_string.clone()));
        assert_eq!(props.class, Some(long_string.clone()));
        assert_eq!(props.style, Some(long_string.clone()));
        assert_eq!(props.aria_label, Some(long_string.clone()));
        assert_eq!(props.aria_describedby, Some(long_string.clone()));
        assert_eq!(props.aria_labelledby, Some(long_string.clone()));
        assert_eq!(props.role, Some(long_string.clone()));
        assert_eq!(props.button_type, Some(long_string.clone()));
    }

    #[test]
    fn test_standardized_button_props_with_numbers() {
        // Test StandardizedButtonProps with numeric values
        let props = StandardizedButtonProps {
            tabindex: Some(0),
            ..Default::default()
        };
        
        assert_eq!(props.tabindex, Some(0));
        
        let props_negative = StandardizedButtonProps {
            tabindex: Some(-1),
            ..Default::default()
        };
        
        assert_eq!(props_negative.tabindex, Some(-1));
        
        let props_large = StandardizedButtonProps {
            tabindex: Some(999),
            ..Default::default()
        };
        
        assert_eq!(props_large.tabindex, Some(999));
    }

    #[test]
    fn test_standardized_button_props_with_boolean_values() {
        // Test StandardizedButtonProps with boolean values
        let props_enabled = StandardizedButtonProps {
            disabled: Some(false),
            ..Default::default()
        };
        
        assert_eq!(props_enabled.disabled, Some(false));
        
        let props_disabled = StandardizedButtonProps {
            disabled: Some(true),
            ..Default::default()
        };
        
        assert_eq!(props_disabled.disabled, Some(true));
    }

    #[test]
    fn test_standardized_button_props_with_variant_values() {
        // Test StandardizedButtonProps with different variant values
        let props_default = StandardizedButtonProps {
            variant: Some(StandardVariant::Default),
            ..Default::default()
        };
        
        assert_eq!(props_default.variant, Some(StandardVariant::Default));
        
        let props_destructive = StandardizedButtonProps {
            variant: Some(StandardVariant::Destructive),
            ..Default::default()
        };
        
        assert_eq!(props_destructive.variant, Some(StandardVariant::Destructive));
        
        let props_outline = StandardizedButtonProps {
            variant: Some(StandardVariant::Outline),
            ..Default::default()
        };
        
        assert_eq!(props_outline.variant, Some(StandardVariant::Outline));
        
        let props_secondary = StandardizedButtonProps {
            variant: Some(StandardVariant::Secondary),
            ..Default::default()
        };
        
        assert_eq!(props_secondary.variant, Some(StandardVariant::Secondary));
        
        let props_ghost = StandardizedButtonProps {
            variant: Some(StandardVariant::Ghost),
            ..Default::default()
        };
        
        assert_eq!(props_ghost.variant, Some(StandardVariant::Ghost));
        
        let props_link = StandardizedButtonProps {
            variant: Some(StandardVariant::Link),
            ..Default::default()
        };
        
        assert_eq!(props_link.variant, Some(StandardVariant::Link));
    }

    #[test]
    fn test_standardized_button_props_with_size_values() {
        // Test StandardizedButtonProps with different size values
        let props_default = StandardizedButtonProps {
            size: Some(StandardSize::Default),
            ..Default::default()
        };
        
        assert_eq!(props_default.size, Some(StandardSize::Default));
        
        let props_small = StandardizedButtonProps {
            size: Some(StandardSize::Small),
            ..Default::default()
        };
        
        assert_eq!(props_small.size, Some(StandardSize::Small));
        
        let props_large = StandardizedButtonProps {
            size: Some(StandardSize::Large),
            ..Default::default()
        };
        
        assert_eq!(props_large.size, Some(StandardSize::Large));
        
        let props_icon = StandardizedButtonProps {
            size: Some(StandardSize::Icon),
            ..Default::default()
        };
        
        assert_eq!(props_icon.size, Some(StandardSize::Icon));
    }

    #[test]
    fn test_standardized_button_props_with_callbacks() {
        // Test StandardizedButtonProps with callback functions
        let click_callback = Box::new(|| {});
        let focus_callback = Box::new(|| {});
        let blur_callback = Box::new(|| {});
        
        let props = StandardizedButtonProps {
            onclick: Some(click_callback),
            onfocus: Some(focus_callback),
            onblur: Some(blur_callback),
            ..Default::default()
        };
        
        assert!(props.onclick.is_some());
        assert!(props.onfocus.is_some());
        assert!(props.onblur.is_some());
    }

    #[test]
    fn test_standardized_button_props_with_children() {
        // Test StandardizedButtonProps with children
        let props = StandardizedButtonProps {
            children: Some(Children::new(|_| view! { <span>"Child content"</span> })),
            ..Default::default()
        };
        
        assert!(props.children.is_some());
    }

    #[test]
    fn test_standardized_button_props_clone() {
        // Test that StandardizedButtonProps can be cloned
        let props = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: Some("test-class".to_string()),
            disabled: Some(true),
            variant: Some(StandardVariant::Destructive),
            size: Some(StandardSize::Large),
            ..Default::default()
        };
        
        let cloned_props = props.clone();
        
        assert_eq!(props.id, cloned_props.id);
        assert_eq!(props.class, cloned_props.class);
        assert_eq!(props.disabled, cloned_props.disabled);
        assert_eq!(props.variant, cloned_props.variant);
        assert_eq!(props.size, cloned_props.size);
    }

    #[test]
    fn test_standardized_button_props_debug() {
        // Test that StandardizedButtonProps can be debug printed
        let props = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: Some("test-class".to_string()),
            disabled: Some(true),
            variant: Some(StandardVariant::Destructive),
            size: Some(StandardSize::Large),
            ..Default::default()
        };
        
        let debug_string = format!("{:?}", props);
        assert!(debug_string.contains("test-id"));
        assert!(debug_string.contains("test-class"));
        assert!(debug_string.contains("true"));
        assert!(debug_string.contains("Destructive"));
        assert!(debug_string.contains("Large"));
    }

    #[test]
    fn test_standardized_button_props_equality() {
        // Test that StandardizedButtonProps can be compared for equality
        let props1 = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: Some("test-class".to_string()),
            disabled: Some(true),
            variant: Some(StandardVariant::Destructive),
            size: Some(StandardSize::Large),
            ..Default::default()
        };
        
        let props2 = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: Some("test-class".to_string()),
            disabled: Some(true),
            variant: Some(StandardVariant::Destructive),
            size: Some(StandardSize::Large),
            ..Default::default()
        };
        
        assert_eq!(props1, props2);
        
        let props3 = StandardizedButtonProps {
            id: Some("different-id".to_string()),
            class: Some("test-class".to_string()),
            disabled: Some(true),
            variant: Some(StandardVariant::Destructive),
            size: Some(StandardSize::Large),
            ..Default::default()
        };
        
        assert_ne!(props1, props3);
    }

    #[test]
    fn test_standardized_button_props_performance() {
        // Test that StandardizedButtonProps creation is performant
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
    fn test_standardized_button_props_memory_usage() {
        // Test that StandardizedButtonProps memory usage is reasonable
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
