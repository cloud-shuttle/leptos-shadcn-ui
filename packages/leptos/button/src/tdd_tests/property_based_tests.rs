//! Property-Based Testing Examples for Button Component
//! 
//! This module contains property-based tests that validate component behavior
//! across a wide range of inputs and scenarios.

#[cfg(test)]
mod property_based_tests {
    use crate::default::{ButtonVariant, ButtonSize};

    #[test]
    fn test_button_variant_string_conversion_properties() {
        // TDD: Property-based test for variant string conversion
        let test_cases = vec![
            ("default", ButtonVariant::Default),
            ("destructive", ButtonVariant::Destructive),
            ("outline", ButtonVariant::Outline),
            ("secondary", ButtonVariant::Secondary),
            ("ghost", ButtonVariant::Ghost),
            ("link", ButtonVariant::Link),
            ("unknown", ButtonVariant::Default),
            ("DESTRUCTIVE", ButtonVariant::Default), // Case sensitive
            ("", ButtonVariant::Default),
        ];
        
        for (input, expected) in test_cases {
            let result = ButtonVariant::from(input.to_string());
            assert_eq!(result, expected, "Input '{}' should convert to {:?}", input, expected);
        }
    }

    #[test]
    fn test_button_size_string_conversion_properties() {
        // TDD: Property-based test for size string conversion
        let test_cases = vec![
            ("default", ButtonSize::Default),
            ("sm", ButtonSize::Sm),
            ("lg", ButtonSize::Lg),  
            ("icon", ButtonSize::Icon),
            ("unknown", ButtonSize::Default),
            ("SM", ButtonSize::Default), // Case sensitive
            ("large", ButtonSize::Default),
        ];
        
        for (input, expected) in test_cases {
            let result = ButtonSize::from(input.to_string());
            assert_eq!(result, expected, "Input '{}' should convert to {:?}", input, expected);
        }
    }
}
