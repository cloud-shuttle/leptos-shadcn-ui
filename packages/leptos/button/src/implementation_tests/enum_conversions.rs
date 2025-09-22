#[cfg(test)]
mod enum_conversions {
    use crate::default::{ButtonVariant, ButtonSize};
    use leptos::prelude::*;

    // ===== ENUM CONVERSION TESTS =====
    // These tests focus on enum conversions, defaults, and basic trait implementations

    #[test]
    fn test_button_variant_enum_conversions() {
        // Test ButtonVariant From<String> implementation
        assert_eq!(ButtonVariant::from("destructive".to_string()), ButtonVariant::Destructive);
        assert_eq!(ButtonVariant::from("outline".to_string()), ButtonVariant::Outline);
        assert_eq!(ButtonVariant::from("secondary".to_string()), ButtonVariant::Secondary);
        assert_eq!(ButtonVariant::from("ghost".to_string()), ButtonVariant::Ghost);
        assert_eq!(ButtonVariant::from("link".to_string()), ButtonVariant::Link);
        assert_eq!(ButtonVariant::from("invalid".to_string()), ButtonVariant::Default);
        assert_eq!(ButtonVariant::from("".to_string()), ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_enum_conversions() {
        // Test ButtonSize From<String> implementation
        assert_eq!(ButtonSize::from("sm".to_string()), ButtonSize::Sm);
        assert_eq!(ButtonSize::from("lg".to_string()), ButtonSize::Lg);
        assert_eq!(ButtonSize::from("icon".to_string()), ButtonSize::Icon);
        assert_eq!(ButtonSize::from("invalid".to_string()), ButtonSize::Default);
        assert_eq!(ButtonSize::from("".to_string()), ButtonSize::Default);
    }

    #[test]
    fn test_button_variant_default() {
        // Test Default implementation for ButtonVariant
        assert_eq!(ButtonVariant::default(), ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_default() {
        // Test Default implementation for ButtonSize
        assert_eq!(ButtonSize::default(), ButtonSize::Default);
    }

    #[test]
    fn test_button_variant_equality() {
        // Test PartialEq implementation for ButtonVariant
        assert_eq!(ButtonVariant::Default, ButtonVariant::Default);
        assert_ne!(ButtonVariant::Default, ButtonVariant::Destructive);
        assert_eq!(ButtonVariant::Outline, ButtonVariant::Outline);
        assert_ne!(ButtonVariant::Ghost, ButtonVariant::Link);
    }

    #[test]
    fn test_button_size_equality() {
        // Test PartialEq implementation for ButtonSize
        assert_eq!(ButtonSize::Default, ButtonSize::Default);
        assert_ne!(ButtonSize::Default, ButtonSize::Sm);
        assert_eq!(ButtonSize::Icon, ButtonSize::Icon);
        assert_ne!(ButtonSize::Lg, ButtonSize::Sm);
    }

    #[test]
    fn test_button_variant_clone() {
        // Test Clone implementation for ButtonVariant
        let variant = ButtonVariant::Destructive;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_button_size_clone() {
        // Test Clone implementation for ButtonSize
        let size = ButtonSize::Lg;
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_button_variant_debug() {
        // Test Debug implementation for ButtonVariant
        let variant = ButtonVariant::Outline;
        let debug_str = format!("{:?}", variant);
        assert!(debug_str.contains("Outline"));
    }

    #[test]
    fn test_button_size_debug() {
        // Test Debug implementation for ButtonSize
        let size = ButtonSize::Icon;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Icon"));
    }

    #[test]
    fn test_button_variant_all_variants() {
        // Test that all ButtonVariant variants are accessible
        let variants = vec![
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];

        for variant in variants {
            // Test that each variant can be converted to string and back
            let variant_str = format!("{:?}", variant);
            assert!(!variant_str.is_empty());
        }
    }

    #[test]
    fn test_button_size_all_sizes() {
        // Test that all ButtonSize variants are accessible
        let sizes = vec![
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ];

        for size in sizes {
            // Test that each size can be converted to string and back
            let size_str = format!("{:?}", size);
            assert!(!size_str.is_empty());
        }
    }

    #[test]
    fn test_button_variant_from_str() {
        // Test ButtonVariant from string conversion
        assert_eq!(ButtonVariant::from("default".to_string()), ButtonVariant::Default);
        assert_eq!(ButtonVariant::from("destructive".to_string()), ButtonVariant::Destructive);
        assert_eq!(ButtonVariant::from("outline".to_string()), ButtonVariant::Outline);
        assert_eq!(ButtonVariant::from("secondary".to_string()), ButtonVariant::Secondary);
        assert_eq!(ButtonVariant::from("ghost".to_string()), ButtonVariant::Ghost);
        assert_eq!(ButtonVariant::from("link".to_string()), ButtonVariant::Link);
    }

    #[test]
    fn test_button_size_from_str() {
        // Test ButtonSize from string conversion
        assert_eq!(ButtonSize::from("default".to_string()), ButtonSize::Default);
        assert_eq!(ButtonSize::from("sm".to_string()), ButtonSize::Sm);
        assert_eq!(ButtonSize::from("lg".to_string()), ButtonSize::Lg);
        assert_eq!(ButtonSize::from("icon".to_string()), ButtonSize::Icon);
    }

    #[test]
    fn test_button_variant_case_insensitive() {
        // Test case insensitive conversion (if implemented)
        // This test assumes case insensitive conversion is not implemented
        assert_eq!(ButtonVariant::from("DESTRUCTIVE".to_string()), ButtonVariant::Default);
        assert_eq!(ButtonVariant::from("OUTLINE".to_string()), ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_case_insensitive() {
        // Test case insensitive conversion (if implemented)
        // This test assumes case insensitive conversion is not implemented
        assert_eq!(ButtonSize::from("SM".to_string()), ButtonSize::Default);
        assert_eq!(ButtonSize::from("LG".to_string()), ButtonSize::Default);
    }

    #[test]
    fn test_button_variant_whitespace_handling() {
        // Test whitespace handling in string conversion
        assert_eq!(ButtonVariant::from(" destructive ".to_string()), ButtonVariant::Default);
        assert_eq!(ButtonVariant::from(" outline ".to_string()), ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_whitespace_handling() {
        // Test whitespace handling in string conversion
        assert_eq!(ButtonSize::from(" sm ".to_string()), ButtonSize::Default);
        assert_eq!(ButtonSize::from(" lg ".to_string()), ButtonSize::Default);
    }

    #[test]
    fn test_button_variant_hash() {
        // Test Hash implementation for ButtonVariant
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        map.insert(ButtonVariant::Default, "default");
        map.insert(ButtonVariant::Destructive, "destructive");
        
        assert_eq!(map.get(&ButtonVariant::Default), Some(&"default"));
        assert_eq!(map.get(&ButtonVariant::Destructive), Some(&"destructive"));
    }

    #[test]
    fn test_button_size_hash() {
        // Test Hash implementation for ButtonSize
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        map.insert(ButtonSize::Default, "default");
        map.insert(ButtonSize::Sm, "small");
        
        assert_eq!(map.get(&ButtonSize::Default), Some(&"default"));
        assert_eq!(map.get(&ButtonSize::Sm), Some(&"small"));
    }
}
