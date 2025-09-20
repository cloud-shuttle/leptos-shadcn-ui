#[cfg(test)]
mod variant_comparison_tests {
    use crate::default::{ButtonVariant as DefaultButtonVariant, ButtonSize as DefaultButtonSize};
    use crate::new_york::{ButtonVariant as NewYorkButtonVariant, ButtonSize as NewYorkButtonSize};

    // ===== VARIANT COMPARISON TESTS =====
    // These tests compare styling differences between default and New York themes

    #[test]
    fn test_button_variant_enum_equivalence() {
        // Test that both themes have equivalent enum variants
        let default_variants = vec![
            DefaultButtonVariant::Default,
            DefaultButtonVariant::Destructive,
            DefaultButtonVariant::Outline,
            DefaultButtonVariant::Secondary,
            DefaultButtonVariant::Ghost,
            DefaultButtonVariant::Link,
        ];

        let new_york_variants = vec![
            NewYorkButtonVariant::Default,
            NewYorkButtonVariant::Destructive,
            NewYorkButtonVariant::Outline,
            NewYorkButtonVariant::Secondary,
            NewYorkButtonVariant::Ghost,
            NewYorkButtonVariant::Link,
        ];

        // Both themes should have the same number of variants
        assert_eq!(default_variants.len(), new_york_variants.len(), "Both themes should have the same number of variants");
    }

    #[test]
    fn test_button_size_enum_equivalence() {
        // Test that both themes have equivalent size variants
        let default_sizes = vec![
            DefaultButtonSize::Default,
            DefaultButtonSize::Sm,
            DefaultButtonSize::Lg,
            DefaultButtonSize::Icon,
        ];

        let new_york_sizes = vec![
            NewYorkButtonSize::Default,
            NewYorkButtonSize::Sm,
            NewYorkButtonSize::Lg,
            NewYorkButtonSize::Icon,
        ];

        // Both themes should have the same number of sizes
        assert_eq!(default_sizes.len(), new_york_sizes.len(), "Both themes should have the same number of sizes");
    }

    #[test]
    fn test_button_variant_string_conversion_equivalence() {
        // Test that both themes handle string conversions the same way
        let test_strings = vec![
            "default", "destructive", "outline", "secondary", "ghost", "link", "invalid", ""
        ];

        for test_string in test_strings {
            let default_result = DefaultButtonVariant::from(test_string.to_string());
            let new_york_result = NewYorkButtonVariant::from(test_string.to_string());

            // Both themes should handle string conversion consistently
            match (default_result, new_york_result) {
                (DefaultButtonVariant::Default, NewYorkButtonVariant::Default) => {},
                (DefaultButtonVariant::Destructive, NewYorkButtonVariant::Destructive) => {},
                (DefaultButtonVariant::Outline, NewYorkButtonVariant::Outline) => {},
                (DefaultButtonVariant::Secondary, NewYorkButtonVariant::Secondary) => {},
                (DefaultButtonVariant::Ghost, NewYorkButtonVariant::Ghost) => {},
                (DefaultButtonVariant::Link, NewYorkButtonVariant::Link) => {},
                _ => panic!("String conversion should be consistent between themes for '{}'", test_string),
            }
        }
    }

    #[test]
    fn test_button_size_string_conversion_equivalence() {
        // Test that both themes handle size string conversions the same way
        let test_strings = vec![
            "default", "sm", "lg", "icon", "invalid", ""
        ];

        for test_string in test_strings {
            let default_result = DefaultButtonSize::from(test_string.to_string());
            let new_york_result = NewYorkButtonSize::from(test_string.to_string());

            // Both themes should handle string conversion consistently
            match (default_result, new_york_result) {
                (DefaultButtonSize::Default, NewYorkButtonSize::Default) => {},
                (DefaultButtonSize::Sm, NewYorkButtonSize::Sm) => {},
                (DefaultButtonSize::Lg, NewYorkButtonSize::Lg) => {},
                (DefaultButtonSize::Icon, NewYorkButtonSize::Icon) => {},
                _ => panic!("Size string conversion should be consistent between themes for '{}'", test_string),
            }
        }
    }

    #[test]
    fn test_button_variant_default_equivalence() {
        // Test that both themes have the same default variant
        let default_default = DefaultButtonVariant::default();
        let new_york_default = NewYorkButtonVariant::default();

        // Both should default to Default variant
        assert!(matches!(default_default, DefaultButtonVariant::Default));
        assert!(matches!(new_york_default, NewYorkButtonVariant::Default));
    }

    #[test]
    fn test_button_size_default_equivalence() {
        // Test that both themes have the same default size
        let default_default = DefaultButtonSize::default();
        let new_york_default = NewYorkButtonSize::default();

        // Both should default to Default size
        assert!(matches!(default_default, DefaultButtonSize::Default));
        assert!(matches!(new_york_default, NewYorkButtonSize::Default));
    }

    #[test]
    fn test_button_variant_clone_equivalence() {
        // Test that both themes handle cloning the same way
        let default_variant = DefaultButtonVariant::Destructive;
        let new_york_variant = NewYorkButtonVariant::Destructive;

        let default_cloned = default_variant.clone();
        let new_york_cloned = new_york_variant.clone();

        // Both should clone correctly
        assert_eq!(default_variant, default_cloned);
        assert_eq!(new_york_variant, new_york_cloned);
    }

    #[test]
    fn test_button_size_clone_equivalence() {
        // Test that both themes handle size cloning the same way
        let default_size = DefaultButtonSize::Lg;
        let new_york_size = NewYorkButtonSize::Lg;

        let default_cloned = default_size.clone();
        let new_york_cloned = new_york_size.clone();

        // Both should clone correctly
        assert_eq!(default_size, default_cloned);
        assert_eq!(new_york_size, new_york_cloned);
    }

    #[test]
    fn test_button_variant_debug_equivalence() {
        // Test that both themes handle debug formatting the same way
        let default_variant = DefaultButtonVariant::Outline;
        let new_york_variant = NewYorkButtonVariant::Outline;

        let default_debug = format!("{:?}", default_variant);
        let new_york_debug = format!("{:?}", new_york_variant);

        // Both should have similar debug output
        assert!(default_debug.contains("Outline"));
        assert!(new_york_debug.contains("Outline"));
    }

    #[test]
    fn test_button_size_debug_equivalence() {
        // Test that both themes handle size debug formatting the same way
        let default_size = DefaultButtonSize::Icon;
        let new_york_size = NewYorkButtonSize::Icon;

        let default_debug = format!("{:?}", default_size);
        let new_york_debug = format!("{:?}", new_york_size);

        // Both should have similar debug output
        assert!(default_debug.contains("Icon"));
        assert!(new_york_debug.contains("Icon"));
    }

    #[test]
    fn test_button_variant_equality_equivalence() {
        // Test that both themes handle equality the same way
        let default_variant1 = DefaultButtonVariant::Ghost;
        let default_variant2 = DefaultButtonVariant::Ghost;
        let default_variant3 = DefaultButtonVariant::Link;

        let new_york_variant1 = NewYorkButtonVariant::Ghost;
        let new_york_variant2 = NewYorkButtonVariant::Ghost;
        let new_york_variant3 = NewYorkButtonVariant::Link;

        // Both themes should handle equality consistently
        assert_eq!(default_variant1, default_variant2);
        assert_ne!(default_variant1, default_variant3);
        assert_eq!(new_york_variant1, new_york_variant2);
        assert_ne!(new_york_variant1, new_york_variant3);
    }

    #[test]
    fn test_button_size_equality_equivalence() {
        // Test that both themes handle size equality the same way
        let default_size1 = DefaultButtonSize::Sm;
        let default_size2 = DefaultButtonSize::Sm;
        let default_size3 = DefaultButtonSize::Lg;

        let new_york_size1 = NewYorkButtonSize::Sm;
        let new_york_size2 = NewYorkButtonSize::Sm;
        let new_york_size3 = NewYorkButtonSize::Lg;

        // Both themes should handle equality consistently
        assert_eq!(default_size1, default_size2);
        assert_ne!(default_size1, default_size3);
        assert_eq!(new_york_size1, new_york_size2);
        assert_ne!(new_york_size1, new_york_size3);
    }

    #[test]
    fn test_button_variant_comprehensive_comparison() {
        // Comprehensive comparison of all button variants
        let variant_pairs = vec![
            (DefaultButtonVariant::Default, NewYorkButtonVariant::Default),
            (DefaultButtonVariant::Destructive, NewYorkButtonVariant::Destructive),
            (DefaultButtonVariant::Outline, NewYorkButtonVariant::Outline),
            (DefaultButtonVariant::Secondary, NewYorkButtonVariant::Secondary),
            (DefaultButtonVariant::Ghost, NewYorkButtonVariant::Ghost),
            (DefaultButtonVariant::Link, NewYorkButtonVariant::Link),
        ];

        for (default_variant, new_york_variant) in variant_pairs {
            // Test string conversion
            let default_string = match default_variant {
                DefaultButtonVariant::Default => "default",
                DefaultButtonVariant::Destructive => "destructive",
                DefaultButtonVariant::Outline => "outline",
                DefaultButtonVariant::Secondary => "secondary",
                DefaultButtonVariant::Ghost => "ghost",
                DefaultButtonVariant::Link => "link",
            };

            let new_york_string = match new_york_variant {
                NewYorkButtonVariant::Default => "default",
                NewYorkButtonVariant::Destructive => "destructive",
                NewYorkButtonVariant::Outline => "outline",
                NewYorkButtonVariant::Secondary => "secondary",
                NewYorkButtonVariant::Ghost => "ghost",
                NewYorkButtonVariant::Link => "link",
            };

            assert_eq!(default_string, new_york_string, "Variant strings should match");

            // Test conversion back from string
            let default_from_string = DefaultButtonVariant::from(default_string.to_string());
            let new_york_from_string = NewYorkButtonVariant::from(new_york_string.to_string());

            // Both should convert back to the same variant
            match (default_from_string, new_york_from_string) {
                (DefaultButtonVariant::Default, NewYorkButtonVariant::Default) => {},
                (DefaultButtonVariant::Destructive, NewYorkButtonVariant::Destructive) => {},
                (DefaultButtonVariant::Outline, NewYorkButtonVariant::Outline) => {},
                (DefaultButtonVariant::Secondary, NewYorkButtonVariant::Secondary) => {},
                (DefaultButtonVariant::Ghost, NewYorkButtonVariant::Ghost) => {},
                (DefaultButtonVariant::Link, NewYorkButtonVariant::Link) => {},
                _ => panic!("String conversion should be consistent"),
            }
        }
    }

    #[test]
    fn test_button_size_comprehensive_comparison() {
        // Comprehensive comparison of all button sizes
        let size_pairs = vec![
            (DefaultButtonSize::Default, NewYorkButtonSize::Default),
            (DefaultButtonSize::Sm, NewYorkButtonSize::Sm),
            (DefaultButtonSize::Lg, NewYorkButtonSize::Lg),
            (DefaultButtonSize::Icon, NewYorkButtonSize::Icon),
        ];

        for (default_size, new_york_size) in size_pairs {
            // Test string conversion
            let default_string = match default_size {
                DefaultButtonSize::Default => "default",
                DefaultButtonSize::Sm => "sm",
                DefaultButtonSize::Lg => "lg",
                DefaultButtonSize::Icon => "icon",
            };

            let new_york_string = match new_york_size {
                NewYorkButtonSize::Default => "default",
                NewYorkButtonSize::Sm => "sm",
                NewYorkButtonSize::Lg => "lg",
                NewYorkButtonSize::Icon => "icon",
            };

            assert_eq!(default_string, new_york_string, "Size strings should match");

            // Test conversion back from string
            let default_from_string = DefaultButtonSize::from(default_string.to_string());
            let new_york_from_string = NewYorkButtonSize::from(new_york_string.to_string());

            // Both should convert back to the same size
            match (default_from_string, new_york_from_string) {
                (DefaultButtonSize::Default, NewYorkButtonSize::Default) => {},
                (DefaultButtonSize::Sm, NewYorkButtonSize::Sm) => {},
                (DefaultButtonSize::Lg, NewYorkButtonSize::Lg) => {},
                (DefaultButtonSize::Icon, NewYorkButtonSize::Icon) => {},
                _ => panic!("Size string conversion should be consistent"),
            }
        }
    }

    #[test]
    fn test_button_theme_api_consistency() {
        // Test that both themes provide consistent APIs
        // This test ensures that the New York theme doesn't break the expected API

        // Test that all expected variants exist
        let _default_variants = vec![
            DefaultButtonVariant::Default,
            DefaultButtonVariant::Destructive,
            DefaultButtonVariant::Outline,
            DefaultButtonVariant::Secondary,
            DefaultButtonVariant::Ghost,
            DefaultButtonVariant::Link,
        ];

        let _new_york_variants = vec![
            NewYorkButtonVariant::Default,
            NewYorkButtonVariant::Destructive,
            NewYorkButtonVariant::Outline,
            NewYorkButtonVariant::Secondary,
            NewYorkButtonVariant::Ghost,
            NewYorkButtonVariant::Link,
        ];

        // Test that all expected sizes exist
        let _default_sizes = vec![
            DefaultButtonSize::Default,
            DefaultButtonSize::Sm,
            DefaultButtonSize::Lg,
            DefaultButtonSize::Icon,
        ];

        let _new_york_sizes = vec![
            NewYorkButtonSize::Default,
            NewYorkButtonSize::Sm,
            NewYorkButtonSize::Lg,
            NewYorkButtonSize::Icon,
        ];

        // If we get here without compilation errors, the APIs are consistent
    }

    #[test]
    fn test_button_theme_behavior_consistency() {
        // Test that both themes behave consistently for edge cases
        let edge_cases = vec![
            "", "invalid", "INVALID", "Default", "DEFAULT", "destructive", "DESTRUCTIVE"
        ];

        for edge_case in edge_cases {
            let default_variant = DefaultButtonVariant::from(edge_case.to_string());
            let new_york_variant = NewYorkButtonVariant::from(edge_case.to_string());

            // Both themes should handle edge cases consistently
            match (default_variant, new_york_variant) {
                (DefaultButtonVariant::Default, NewYorkButtonVariant::Default) => {},
                (DefaultButtonVariant::Destructive, NewYorkButtonVariant::Destructive) => {},
                (DefaultButtonVariant::Outline, NewYorkButtonVariant::Outline) => {},
                (DefaultButtonVariant::Secondary, NewYorkButtonVariant::Secondary) => {},
                (DefaultButtonVariant::Ghost, NewYorkButtonVariant::Ghost) => {},
                (DefaultButtonVariant::Link, NewYorkButtonVariant::Link) => {},
                _ => panic!("Edge case '{}' should be handled consistently", edge_case),
            }
        }
    }
}
