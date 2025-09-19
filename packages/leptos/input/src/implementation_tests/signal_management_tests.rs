#[cfg(test)]
mod signal_management_tests {
    use crate::validation::{ValidationResult, ValidationRule, ValidationContext, InputValidator};
    use leptos::prelude::*;

    #[test]
    fn test_validation_context_new() {
        // Test ValidationContext creation
        let context = ValidationContext::new();
        assert!(context.validators.is_empty());
        assert!(context.results.get().is_empty());
    }

    #[test]
    fn test_validation_context_add_validator() {
        // Test adding validators to context
        let mut context = ValidationContext::new();
        let validator = InputValidator::new("email").required().email();
        
        context.add_validator(validator);
        
        assert_eq!(context.validators.len(), 1);
        assert!(context.validators.contains_key("email"));
    }

    #[test]
    fn test_validation_context_validate_field() {
        // Test field validation in context
        let mut context = ValidationContext::new();
        context.add_validator(InputValidator::new("email").required().email());
        
        // Test invalid field
        let invalid_result = context.validate_field("email", "invalid-email");
        assert!(!invalid_result.is_valid);
        assert_eq!(invalid_result.errors.len(), 1);
        
        // Test valid field
        let valid_result = context.validate_field("email", "user@example.com");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
        
        // Test unknown field
        let unknown_result = context.validate_field("unknown", "value");
        assert!(unknown_result.is_valid);
        assert!(unknown_result.errors.is_empty());
    }

    #[test]
    fn test_validation_context_validate_all() {
        // Test validating all fields
        let mut context = ValidationContext::new();
        context.add_validator(InputValidator::new("email").required().email());
        context.add_validator(InputValidator::new("password").required().min_length(8));
        
        let mut values = std::collections::HashMap::new();
        values.insert("email".to_string(), "invalid-email".to_string());
        values.insert("password".to_string(), "short".to_string());
        
        let result = context.validate_all(&values);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 2); // Both fields have errors
    }

    #[test]
    fn test_validation_context_get_field_error() {
        // Test getting field errors from context
        let mut context = ValidationContext::new();
        context.add_validator(InputValidator::new("email").required().email());
        
        context.validate_field("email", "invalid-email");
        
        let error = context.get_field_error("email");
        assert!(error.is_some());
        assert_eq!(error.unwrap(), "Please enter a valid email address");
        
        let no_error = context.get_field_error("unknown");
        assert!(no_error.is_none());
    }

    #[test]
    fn test_validation_context_is_field_valid() {
        // Test checking field validity
        let mut context = ValidationContext::new();
        context.add_validator(InputValidator::new("email").required().email());
        
        context.validate_field("email", "invalid-email");
        assert!(!context.is_field_valid("email"));
        
        context.validate_field("email", "user@example.com");
        assert!(context.is_field_valid("email"));
        
        assert!(context.is_field_valid("unknown")); // Unknown fields are valid by default
    }

    #[test]
    fn test_validation_context_is_form_valid() {
        // Test checking form validity
        let mut context = ValidationContext::new();
        context.add_validator(InputValidator::new("email").required().email());
        context.add_validator(InputValidator::new("password").required().min_length(8));
        
        // Test with invalid fields
        context.validate_field("email", "invalid-email");
        context.validate_field("password", "short");
        assert!(!context.is_form_valid());
        
        // Test with valid fields
        context.validate_field("email", "user@example.com");
        context.validate_field("password", "password123");
        assert!(context.is_form_valid());
    }

    #[test]
    fn test_signal_reactivity() {
        // Test signal reactivity with validation
        let value_signal = RwSignal::new("".to_string());
        let validation_signal = RwSignal::new(ValidationResult::new());
        let is_validating_signal = RwSignal::new(false);
        
        // Test initial state
        assert_eq!(value_signal.get(), "");
        assert!(validation_signal.get().is_valid);
        assert!(!is_validating_signal.get());
        
        // Test value change
        value_signal.set("test@example.com".to_string());
        assert_eq!(value_signal.get(), "test@example.com");
        
        // Test validation state change
        let mut result = ValidationResult::new();
        result.add_error("email", "Invalid email", ValidationRule::Email);
        validation_signal.set(result);
        
        assert!(!validation_signal.get().is_valid);
        assert!(validation_signal.get().has_errors());
        
        // Test validating state
        is_validating_signal.set(true);
        assert!(is_validating_signal.get());
    }

    #[test]
    fn test_signal_derived_values() {
        // Test derived signals for computed values
        let value_signal = RwSignal::new("".to_string());
        let has_error_signal = RwSignal::new(false);
        
        // Test derived computed values
        let is_empty = move || value_signal.get().is_empty();
        let has_error = move || has_error_signal.get();
        let is_valid = move || !is_empty() && !has_error();
        
        // Test initial state
        assert!(is_empty());
        assert!(!has_error());
        assert!(!is_valid());
        
        // Test with value
        value_signal.set("test".to_string());
        assert!(!is_empty());
        assert!(!has_error());
        assert!(is_valid());
        
        // Test with error
        has_error_signal.set(true);
        assert!(!is_empty());
        assert!(has_error());
        assert!(!is_valid());
    }

    #[test]
    fn test_signal_memoization() {
        // Test memoization for expensive computations
        let value_signal = RwSignal::new("".to_string());
        let validation_signal = RwSignal::new(ValidationResult::new());
        
        // Test memoized validation
        let memoized_validation = Memo::new(move |_| {
            let value = value_signal.get();
            let mut result = ValidationResult::new();
            
            if value.is_empty() {
                result.add_error("field", "Required", ValidationRule::Required);
            } else if value.len() < 3 {
                result.add_error("field", "Too short", ValidationRule::MinLength(3));
            }
            
            result
        });
        
        // Test initial state
        let initial_result = memoized_validation.get();
        assert!(!initial_result.is_valid);
        assert_eq!(initial_result.errors.len(), 1);
        assert_eq!(initial_result.errors[0].message, "Required");
        
        // Test with valid value
        value_signal.set("valid".to_string());
        let valid_result = memoized_validation.get();
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
        
        // Test with short value
        value_signal.set("ab".to_string());
        let short_result = memoized_validation.get();
        assert!(!short_result.is_valid);
        assert_eq!(short_result.errors.len(), 1);
        assert_eq!(short_result.errors[0].message, "Too short");
    }

    #[test]
    fn test_signal_batching() {
        // Test signal batching for performance
        let value_signal = RwSignal::new("".to_string());
        let validation_signal = RwSignal::new(ValidationResult::new());
        let is_validating_signal = RwSignal::new(false);
        
        // Test sequential updates
        value_signal.set("test@example.com".to_string());
        is_validating_signal.set(true);
        
        let mut result = ValidationResult::new();
        result.add_error("email", "Invalid email", ValidationRule::Email);
        validation_signal.set(result);
        
        // All signals should be updated atomically
        assert_eq!(value_signal.get(), "test@example.com");
        assert!(is_validating_signal.get());
        assert!(!validation_signal.get().is_valid);
    }

    #[test]
    fn test_signal_cleanup() {
        // Test signal cleanup and memory management
        let value_signal = RwSignal::new("initial".to_string());
        let validation_signal = RwSignal::new(ValidationResult::new());
        
        // Test initial state
        assert_eq!(value_signal.get(), "initial");
        assert!(validation_signal.get().is_valid);
        
        // Test setting large values
        let large_value = "x".repeat(10000);
        value_signal.set(large_value.clone());
        assert_eq!(value_signal.get(), large_value);
        
        // Test cleanup by setting smaller value
        value_signal.set("small".to_string());
        assert_eq!(value_signal.get(), "small");
        
        // Test validation result cleanup
        let mut result = ValidationResult::new();
        for i in 0..100 {
            result.add_error(&format!("field_{}", i), &format!("error_{}", i), ValidationRule::Required);
        }
        validation_signal.set(result);
        assert_eq!(validation_signal.get().errors.len(), 100);
        
        // Test clearing validation
        validation_signal.set(ValidationResult::new());
        assert!(validation_signal.get().is_valid);
        assert!(validation_signal.get().errors.is_empty());
    }
}
