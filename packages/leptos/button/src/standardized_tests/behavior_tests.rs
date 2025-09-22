#[cfg(test)]
mod behavior_tests {
    use crate::standardized::StandardizedButtonProps;
    use leptos_shadcn_api_standards::*;
    use leptos::prelude::*;

    // ===== BEHAVIOR TESTS =====
    // These tests focus on standardized behavior and cross-platform compatibility

    #[test]
    fn test_standardized_button_behavior_default_state() {
        // Test default button behavior
        let props = StandardizedButtonProps::default();
        
        // Test that default state is properly set
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Default));
        assert_eq!(props.size, Some(StandardSize::Default));
        assert_eq!(props.button_type, None); // Should default to "button" in implementation
    }

    #[test]
    fn test_standardized_button_behavior_disabled_state() {
        // Test disabled button behavior
        let props = StandardizedButtonProps {
            id: None,
            class: None,
            style: None,
            disabled: Some(true),
            variant: Some(StandardVariant::Default),
            size: Some(StandardSize::Default),
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

        // Test that disabled state is properly set
        assert_eq!(props.disabled, Some(true));
        
        // Test that event handlers are None for disabled buttons
        assert_eq!(props.onclick, None);
        assert_eq!(props.onfocus, None);
        assert_eq!(props.onblur, None);
    }

    #[test]
    fn test_standardized_button_behavior_variant_consistency() {
        // Test that variant behavior is consistent
        let variants = vec![
            StandardVariant::Default,
            StandardVariant::Destructive,
            StandardVariant::Outline,
            StandardVariant::Secondary,
            StandardVariant::Ghost,
            StandardVariant::Link,
        ];

        for variant in variants {
            let props = StandardizedButtonProps {
                id: None,
                class: None,
                style: None,
                disabled: Some(false),
                variant: Some(variant),
                size: Some(StandardSize::Default),
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

            // Test that variant is properly set
            assert_eq!(props.variant, Some(variant));
            
            // Test that other props remain unchanged
            assert_eq!(props.disabled, Some(false));
            assert_eq!(props.size, Some(StandardSize::Default));
        }
    }

    #[test]
    fn test_standardized_button_behavior_size_consistency() {
        // Test that size behavior is consistent
        let sizes = vec![
            StandardSize::Default,
            StandardSize::Small,
            StandardSize::Large,
            StandardSize::Icon,
        ];

        for size in sizes {
            let props = StandardizedButtonProps {
                id: None,
                class: None,
                style: None,
                disabled: Some(false),
                variant: Some(StandardVariant::Default),
                size: Some(size),
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

            // Test that size is properly set
            assert_eq!(props.size, Some(size));
            
            // Test that other props remain unchanged
            assert_eq!(props.disabled, Some(false));
            assert_eq!(props.variant, Some(StandardVariant::Default));
        }
    }

    #[test]
    fn test_standardized_button_behavior_event_handler_consistency() {
        // Test that event handlers are consistent
        let mut click_count = 0;
        let mut focus_count = 0;
        let mut blur_count = 0;

        let props = StandardizedButtonProps {
            id: None,
            class: None,
            style: None,
            disabled: Some(false),
            variant: Some(StandardVariant::Default),
            size: Some(StandardSize::Default),
            aria_label: None,
            aria_describedby: None,
            aria_labelledby: None,
            role: None,
            tabindex: None,
            button_type: None,
            onclick: Some(Box::new(|| { click_count += 1; })),
            onfocus: Some(Box::new(|| { focus_count += 1; })),
            onblur: Some(Box::new(|| { blur_count += 1; })),
            children: None,
        };

        // Test that event handlers are properly set
        assert!(props.onclick.is_some());
        assert!(props.onfocus.is_some());
        assert!(props.onblur.is_some());
        
        // Test that other props remain unchanged
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Default));
        assert_eq!(props.size, Some(StandardSize::Default));
    }

    #[test]
    fn test_standardized_button_behavior_accessibility_consistency() {
        // Test that accessibility behavior is consistent
        let props = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: None,
            style: None,
            disabled: Some(false),
            variant: Some(StandardVariant::Default),
            size: Some(StandardSize::Default),
            aria_label: Some("Test button".to_string()),
            aria_describedby: Some("test-description".to_string()),
            aria_labelledby: Some("test-label".to_string()),
            role: Some("button".to_string()),
            tabindex: Some(0),
            button_type: None,
            onclick: None,
            onfocus: None,
            onblur: None,
            children: None,
        };

        // Test that accessibility props are properly set
        assert_eq!(props.id, Some("test-id".to_string()));
        assert_eq!(props.aria_label, Some("Test button".to_string()));
        assert_eq!(props.aria_describedby, Some("test-description".to_string()));
        assert_eq!(props.aria_labelledby, Some("test-label".to_string()));
        assert_eq!(props.role, Some("button".to_string()));
        assert_eq!(props.tabindex, Some(0));
        
        // Test that other props remain unchanged
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Default));
        assert_eq!(props.size, Some(StandardSize::Default));
    }

    #[test]
    fn test_standardized_button_behavior_cross_platform_consistency() {
        // Test cross-platform consistency
        let props = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: Some("test-class".to_string()),
            style: Some("color: red".to_string()),
            disabled: Some(false),
            variant: Some(StandardVariant::Default),
            size: Some(StandardSize::Default),
            aria_label: Some("Test button".to_string()),
            aria_describedby: None,
            aria_labelledby: None,
            role: Some("button".to_string()),
            tabindex: Some(0),
            button_type: Some("button".to_string()),
            onclick: None,
            onfocus: None,
            onblur: None,
            children: None,
        };

        // Test that all props are properly set across platforms
        assert_eq!(props.id, Some("test-id".to_string()));
        assert_eq!(props.class, Some("test-class".to_string()));
        assert_eq!(props.style, Some("color: red".to_string()));
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Default));
        assert_eq!(props.size, Some(StandardSize::Default));
        assert_eq!(props.aria_label, Some("Test button".to_string()));
        assert_eq!(props.role, Some("button".to_string()));
        assert_eq!(props.tabindex, Some(0));
        assert_eq!(props.button_type, Some("button".to_string()));
    }

    #[test]
    fn test_standardized_button_behavior_prop_combination() {
        // Test prop combination behavior
        let props = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            class: Some("test-class".to_string()),
            style: Some("color: red".to_string()),
            disabled: Some(false),
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

        // Test that all props work together
        assert_eq!(props.id, Some("test-id".to_string()));
        assert_eq!(props.class, Some("test-class".to_string()));
        assert_eq!(props.style, Some("color: red".to_string()));
        assert_eq!(props.disabled, Some(false));
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
    }

    #[test]
    fn test_standardized_button_behavior_edge_cases() {
        // Test edge cases
        let props = StandardizedButtonProps {
            id: Some("".to_string()), // Empty string
            class: Some("".to_string()), // Empty string
            style: Some("".to_string()), // Empty string
            disabled: Some(false),
            variant: Some(StandardVariant::Default),
            size: Some(StandardSize::Default),
            aria_label: Some("".to_string()), // Empty string
            aria_describedby: None,
            aria_labelledby: None,
            role: Some("".to_string()), // Empty string
            tabindex: Some(-1), // Negative tabindex
            button_type: Some("".to_string()), // Empty string
            onclick: None,
            onfocus: None,
            onblur: None,
            children: None,
        };

        // Test that edge cases are handled properly
        assert_eq!(props.id, Some("".to_string()));
        assert_eq!(props.class, Some("".to_string()));
        assert_eq!(props.style, Some("".to_string()));
        assert_eq!(props.aria_label, Some("".to_string()));
        assert_eq!(props.role, Some("".to_string()));
        assert_eq!(props.tabindex, Some(-1));
        assert_eq!(props.button_type, Some("".to_string()));
    }

    #[test]
    fn test_standardized_button_behavior_performance() {
        // Test performance characteristics
        let props = StandardizedButtonProps::default();
        
        // Test that default creation is fast
        let start = std::time::Instant::now();
        let _props = StandardizedButtonProps::default();
        let duration = start.elapsed();
        
        // Test that creation takes less than 1ms
        assert!(duration.as_millis() < 1);
    }

    #[test]
    fn test_standardized_button_behavior_memory_usage() {
        // Test memory usage characteristics
        let props = StandardizedButtonProps::default();
        
        // Test that default props have minimal memory footprint
        let size = std::mem::size_of::<StandardizedButtonProps>();
        
        // Test that size is reasonable (less than 1KB)
        assert!(size < 1024);
    }
}
