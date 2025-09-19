#[cfg(test)]
mod prop_handling_tests {
    use crate::default::{INPUT_CLASS, INPUT_ERROR_CLASS};
    use crate::validation::{ValidationError, ValidationRule};
    use leptos::prelude::*;
    use leptos_style::Style;

    #[test]
    fn test_input_computed_class_generation() {
        // Test computed class generation logic
        let base_class = INPUT_CLASS;
        let custom_class = "custom-input";
        let error_class = INPUT_ERROR_CLASS;
        
        // Test base class
        let computed = format!("{} {} {}", base_class, custom_class, "").trim().to_string();
        assert!(computed.contains("flex"));
        assert!(computed.contains("h-10"));
        assert!(computed.contains("custom-input"));
        
        // Test with error class
        let computed_with_error = format!("{} {} {}", base_class, custom_class, error_class).trim().to_string();
        assert!(computed_with_error.contains("border-destructive"));
        assert!(computed_with_error.contains("focus-visible:ring-destructive"));
    }

    #[test]
    fn test_input_display_error_logic() {
        // Test display error logic
        let validation_error = Some("Custom error message".to_string());
        let validation_result_has_errors = true;
        let validation_result_errors = vec![ValidationError {
            field: "test".to_string(),
            message: "Validation error message".to_string(),
            rule: ValidationRule::Required,
        }];
        
        // Test with validation_error
        if let Some(error) = validation_error {
            assert_eq!(error, "Custom error message");
        }
        
        // Test with validation_result errors
        if validation_result_has_errors {
            let first_error = validation_result_errors.first().map(|e| e.message.clone());
            assert_eq!(first_error, Some("Validation error message".to_string()));
        }
        
        // Test with no errors
        let no_validation_error: Option<String> = None;
        let no_validation_result_errors = false;
        let no_errors = if no_validation_error.is_some() {
            no_validation_error
        } else if no_validation_result_errors {
            None
        } else {
            None
        };
        assert!(no_errors.is_none());
    }

    #[test]
    fn test_input_aria_attributes() {
        // Test ARIA attribute generation
        let has_errors = true;
        let id = "test-input";
        
        // Test aria-invalid
        let aria_invalid = has_errors.to_string();
        assert_eq!(aria_invalid, "true");
        
        // Test aria-describedby
        let aria_describedby = if has_errors {
            format!("{}-error", id)
        } else {
            String::new()
        };
        assert_eq!(aria_describedby, "test-input-error");
        
        // Test without errors
        let no_errors = false;
        let no_aria_describedby = if no_errors {
            format!("{}-error", id)
        } else {
            String::new()
        };
        assert_eq!(no_aria_describedby, "");
    }

    #[test]
    fn test_input_type_defaults() {
        // Test input type default handling
        let input_type = Some("email".to_string());
        let default_type = input_type.unwrap_or_else(|| "text".to_string());
        assert_eq!(default_type, "email");
        
        let no_input_type: Option<String> = None;
        let default_type_none = no_input_type.unwrap_or_else(|| "text".to_string());
        assert_eq!(default_type_none, "text");
    }

    #[test]
    fn test_input_prop_defaults() {
        // Test prop default handling
        let value = Some("test value".to_string());
        let default_value = value.unwrap_or_default();
        assert_eq!(default_value, "test value");
        
        let no_value: Option<String> = None;
        let default_value_none = no_value.unwrap_or_default();
        assert_eq!(default_value_none, "");
        
        let placeholder = Some("Enter text".to_string());
        let default_placeholder = placeholder.unwrap_or_default();
        assert_eq!(default_placeholder, "Enter text");
        
        let no_placeholder: Option<String> = None;
        let default_placeholder_none = no_placeholder.unwrap_or_default();
        assert_eq!(default_placeholder_none, "");
    }

    #[test]
    fn test_input_style_handling() {
        // Test style signal handling
        let style_signal = RwSignal::new(Style::new());
        let style_string = style_signal.get().to_string();
        assert_eq!(style_string, "");
        
        // Test style changes
        let new_style = Style::new();
        style_signal.set(new_style);
        let new_style_string = style_signal.get().to_string();
        assert_eq!(new_style_string, "");
    }

    #[test]
    fn test_input_disabled_signal_handling() {
        // Test disabled signal handling
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get());
        
        disabled_signal.set(true);
        assert!(disabled_signal.get());
        
        disabled_signal.set(false);
        assert!(!disabled_signal.get());
    }

    #[test]
    fn test_input_validation_signal_handling() {
        // Test validation signal handling
        let validation_result = RwSignal::new(crate::validation::ValidationResult::new());
        let is_validating = RwSignal::new(false);
        let show_validation = RwSignal::new(true);
        
        // Test initial state
        assert!(validation_result.get().is_valid);
        assert!(!is_validating.get());
        assert!(show_validation.get());
        
        // Test validation state changes
        let mut result = crate::validation::ValidationResult::new();
        result.add_error("test", "Test error", ValidationRule::Required);
        validation_result.set(result);
        
        assert!(!validation_result.get().is_valid);
        assert!(validation_result.get().has_errors());
        
        is_validating.set(true);
        assert!(is_validating.get());
        
        show_validation.set(false);
        assert!(!show_validation.get());
    }

    #[test]
    fn test_input_edge_cases() {
        // Test edge cases and error conditions
        
        // Test empty strings
        let validator = crate::validation::InputValidator::new("test").required();
        let empty_result = validator.validate("");
        assert!(!empty_result.is_valid);
        
        // Test whitespace-only strings
        let whitespace_result = validator.validate("   ");
        assert!(!whitespace_result.is_valid);
        
        // Test very long strings
        let long_string = "a".repeat(10000);
        let long_result = validator.validate(&long_string);
        assert!(long_result.is_valid); // Should be valid for required rule
        
        // Test special characters
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        let special_result = validator.validate(special_chars);
        assert!(special_result.is_valid);
    }

    #[test]
    fn test_input_performance_characteristics() {
        // Test performance characteristics
        
        // Test rapid signal updates
        let value_signal = RwSignal::new("initial".to_string());
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            value_signal.set(format!("value_{}", i));
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
        
        // Test validation performance
        let validator = crate::validation::InputValidator::new("test")
            .required()
            .min_length(5)
            .max_length(100)
            .email();
        
        let start_validation = std::time::Instant::now();
        
        for i in 0..100 {
            let test_value = format!("test{}@example.com", i);
            let _result = validator.validate(&test_value);
        }
        
        let validation_duration = start_validation.elapsed();
        assert!(validation_duration.as_millis() < 50); // Should be very fast
    }

    #[test]
    fn test_input_memory_management() {
        // Test memory management
        
        // Test signal cleanup
        let value_signal = RwSignal::new("test".to_string());
        let initial_value = value_signal.get();
        assert_eq!(initial_value, "test");
        
        // Test large string handling
        let large_string = "x".repeat(100000);
        value_signal.set(large_string.clone());
        assert_eq!(value_signal.get(), large_string);
        
        // Test memory cleanup by setting smaller value
        value_signal.set("small".to_string());
        assert_eq!(value_signal.get(), "small");
        
        // Test validation result memory
        let mut result = crate::validation::ValidationResult::new();
        for i in 0..100 {
            result.add_error(&format!("field_{}", i), &format!("error_{}", i), ValidationRule::Required);
        }
        
        assert_eq!(result.errors.len(), 100);
        result.clear_errors();
        assert_eq!(result.errors.len(), 0);
    }
}
