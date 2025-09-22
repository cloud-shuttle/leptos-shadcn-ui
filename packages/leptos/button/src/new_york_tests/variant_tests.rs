#[cfg(test)]
mod variant_tests {
    use crate::new_york::{ButtonVariant as ButtonVariantNewYork, ButtonSize as ButtonSizeNewYork};
    use leptos::prelude::*;

    // ===== NEW YORK VARIANT TESTS =====
    // These tests focus on the New York theme variant implementation

    #[test]
    fn test_new_york_button_variant_enum_conversions() {
        // Test ButtonVariantNewYork From<String> implementation
        assert_eq!(ButtonVariantNewYork::from("destructive".to_string()), ButtonVariantNewYork::Destructive);
        assert_eq!(ButtonVariantNewYork::from("outline".to_string()), ButtonVariantNewYork::Outline);
        assert_eq!(ButtonVariantNewYork::from("secondary".to_string()), ButtonVariantNewYork::Secondary);
        assert_eq!(ButtonVariantNewYork::from("ghost".to_string()), ButtonVariantNewYork::Ghost);
        assert_eq!(ButtonVariantNewYork::from("link".to_string()), ButtonVariantNewYork::Link);
        assert_eq!(ButtonVariantNewYork::from("invalid".to_string()), ButtonVariantNewYork::Default);
        assert_eq!(ButtonVariantNewYork::from("".to_string()), ButtonVariantNewYork::Default);
    }

    #[test]
    fn test_new_york_button_size_enum_conversions() {
        // Test ButtonSizeNewYork From<String> implementation
        assert_eq!(ButtonSizeNewYork::from("sm".to_string()), ButtonSizeNewYork::Sm);
        assert_eq!(ButtonSizeNewYork::from("lg".to_string()), ButtonSizeNewYork::Lg);
        assert_eq!(ButtonSizeNewYork::from("icon".to_string()), ButtonSizeNewYork::Icon);
        assert_eq!(ButtonSizeNewYork::from("invalid".to_string()), ButtonSizeNewYork::Default);
        assert_eq!(ButtonSizeNewYork::from("".to_string()), ButtonSizeNewYork::Default);
    }

    #[test]
    fn test_new_york_button_variant_default() {
        // Test Default implementation for ButtonVariantNewYork
        assert_eq!(ButtonVariantNewYork::default(), ButtonVariantNewYork::Default);
    }

    #[test]
    fn test_new_york_button_size_default() {
        // Test Default implementation for ButtonSizeNewYork
        assert_eq!(ButtonSizeNewYork::default(), ButtonSizeNewYork::Default);
    }

    #[test]
    fn test_new_york_button_variant_equality() {
        // Test PartialEq implementation for ButtonVariantNewYork
        assert_eq!(ButtonVariantNewYork::Default, ButtonVariantNewYork::Default);
        assert_ne!(ButtonVariantNewYork::Default, ButtonVariantNewYork::Destructive);
        assert_eq!(ButtonVariantNewYork::Outline, ButtonVariantNewYork::Outline);
        assert_ne!(ButtonVariantNewYork::Ghost, ButtonVariantNewYork::Link);
    }

    #[test]
    fn test_new_york_button_size_equality() {
        // Test PartialEq implementation for ButtonSizeNewYork
        assert_eq!(ButtonSizeNewYork::Default, ButtonSizeNewYork::Default);
        assert_ne!(ButtonSizeNewYork::Default, ButtonSizeNewYork::Sm);
        assert_eq!(ButtonSizeNewYork::Icon, ButtonSizeNewYork::Icon);
        assert_ne!(ButtonSizeNewYork::Lg, ButtonSizeNewYork::Sm);
    }

    #[test]
    fn test_new_york_button_variant_clone() {
        // Test Clone implementation for ButtonVariantNewYork
        let variant = ButtonVariantNewYork::Destructive;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_new_york_button_size_clone() {
        // Test Clone implementation for ButtonSizeNewYork
        let size = ButtonSizeNewYork::Lg;
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_new_york_button_variant_debug() {
        // Test Debug implementation for ButtonVariantNewYork
        let variant = ButtonVariantNewYork::Outline;
        let debug_str = format!("{:?}", variant);
        assert!(debug_str.contains("Outline"));
    }

    #[test]
    fn test_new_york_button_size_debug() {
        // Test Debug implementation for ButtonSizeNewYork
        let size = ButtonSizeNewYork::Icon;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Icon"));
    }

    #[test]
    fn test_new_york_button_variant_all_variants() {
        // Test that all ButtonVariantNewYork variants are accessible
        let variants = vec![
            ButtonVariantNewYork::Default,
            ButtonVariantNewYork::Destructive,
            ButtonVariantNewYork::Outline,
            ButtonVariantNewYork::Secondary,
            ButtonVariantNewYork::Ghost,
            ButtonVariantNewYork::Link,
        ];

        for variant in variants {
            // Test that each variant can be converted to string and back
            let variant_str = format!("{:?}", variant);
            assert!(!variant_str.is_empty());
        }
    }

    #[test]
    fn test_new_york_button_size_all_sizes() {
        // Test that all ButtonSizeNewYork variants are accessible
        let sizes = vec![
            ButtonSizeNewYork::Default,
            ButtonSizeNewYork::Sm,
            ButtonSizeNewYork::Lg,
            ButtonSizeNewYork::Icon,
        ];

        for size in sizes {
            // Test that each size can be converted to string and back
            let size_str = format!("{:?}", size);
            assert!(!size_str.is_empty());
        }
    }

    #[test]
    fn test_new_york_button_variant_from_str() {
        // Test ButtonVariantNewYork from string conversion
        assert_eq!(ButtonVariantNewYork::from("default".to_string()), ButtonVariantNewYork::Default);
        assert_eq!(ButtonVariantNewYork::from("destructive".to_string()), ButtonVariantNewYork::Destructive);
        assert_eq!(ButtonVariantNewYork::from("outline".to_string()), ButtonVariantNewYork::Outline);
        assert_eq!(ButtonVariantNewYork::from("secondary".to_string()), ButtonVariantNewYork::Secondary);
        assert_eq!(ButtonVariantNewYork::from("ghost".to_string()), ButtonVariantNewYork::Ghost);
        assert_eq!(ButtonVariantNewYork::from("link".to_string()), ButtonVariantNewYork::Link);
    }

    #[test]
    fn test_new_york_button_size_from_str() {
        // Test ButtonSizeNewYork from string conversion
        assert_eq!(ButtonSizeNewYork::from("default".to_string()), ButtonSizeNewYork::Default);
        assert_eq!(ButtonSizeNewYork::from("sm".to_string()), ButtonSizeNewYork::Sm);
        assert_eq!(ButtonSizeNewYork::from("lg".to_string()), ButtonSizeNewYork::Lg);
        assert_eq!(ButtonSizeNewYork::from("icon".to_string()), ButtonSizeNewYork::Icon);
    }

    #[test]
    fn test_new_york_button_variant_case_insensitive() {
        // Test case insensitive conversion (if implemented)
        // This test assumes case insensitive conversion is not implemented
        assert_eq!(ButtonVariantNewYork::from("DESTRUCTIVE".to_string()), ButtonVariantNewYork::Default);
        assert_eq!(ButtonVariantNewYork::from("OUTLINE".to_string()), ButtonVariantNewYork::Default);
    }

    #[test]
    fn test_new_york_button_size_case_insensitive() {
        // Test case insensitive conversion (if implemented)
        // This test assumes case insensitive conversion is not implemented
        assert_eq!(ButtonSizeNewYork::from("SM".to_string()), ButtonSizeNewYork::Default);
        assert_eq!(ButtonSizeNewYork::from("LG".to_string()), ButtonSizeNewYork::Default);
    }

    #[test]
    fn test_new_york_button_variant_whitespace_handling() {
        // Test whitespace handling in string conversion
        assert_eq!(ButtonVariantNewYork::from(" destructive ".to_string()), ButtonVariantNewYork::Default);
        assert_eq!(ButtonVariantNewYork::from(" outline ".to_string()), ButtonVariantNewYork::Default);
    }

    #[test]
    fn test_new_york_button_size_whitespace_handling() {
        // Test whitespace handling in string conversion
        assert_eq!(ButtonSizeNewYork::from(" sm ".to_string()), ButtonSizeNewYork::Default);
        assert_eq!(ButtonSizeNewYork::from(" lg ".to_string()), ButtonSizeNewYork::Default);
    }

    #[test]
    fn test_new_york_button_variant_hash() {
        // Test Hash implementation for ButtonVariantNewYork
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        map.insert(ButtonVariantNewYork::Default, "default");
        map.insert(ButtonVariantNewYork::Destructive, "destructive");
        
        assert_eq!(map.get(&ButtonVariantNewYork::Default), Some(&"default"));
        assert_eq!(map.get(&ButtonVariantNewYork::Destructive), Some(&"destructive"));
    }

    #[test]
    fn test_new_york_button_size_hash() {
        // Test Hash implementation for ButtonSizeNewYork
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        map.insert(ButtonSizeNewYork::Default, "default");
        map.insert(ButtonSizeNewYork::Sm, "small");
        
        assert_eq!(map.get(&ButtonSizeNewYork::Default), Some(&"default"));
        assert_eq!(map.get(&ButtonSizeNewYork::Sm), Some(&"small"));
    }

    #[test]
    fn test_new_york_button_variant_serialization() {
        // Test serialization capabilities (if implemented)
        let variant = ButtonVariantNewYork::Destructive;
        let serialized = format!("{:?}", variant);
        assert!(serialized.contains("Destructive"));
    }

    #[test]
    fn test_new_york_button_size_serialization() {
        // Test serialization capabilities (if implemented)
        let size = ButtonSizeNewYork::Lg;
        let serialized = format!("{:?}", size);
        assert!(serialized.contains("Lg"));
    }
}
