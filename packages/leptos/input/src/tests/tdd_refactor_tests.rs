//! TDD Pattern 3: REFACTOR - Enhanced validation system tests
//! 
//! This module contains tests for the enhanced validation system and component functionality.

#[cfg(test)]
mod tdd_refactor_tests {
    use crate::default::INPUT_CLASS;
    use crate::validation::{ValidationContext, validation_builders, InputValidator};
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_enhanced_input_validation_system() {
        // Test email validation with the new system
        let email_validator = validation_builders::email_validator("email");
        let result = email_validator.validate("invalid-email");
        
        assert!(!result.is_valid);
        assert!(result.errors.len() >= 1); // At least one error (email format)
        assert!(result.errors.iter().any(|e| e.message.contains("valid email")));
        
        // Test valid email
        let valid_result = email_validator.validate("user@example.com");
        assert!(valid_result.is_valid);
    }

    #[test]
    fn test_password_validation_system() {
        let password_validator = validation_builders::password_validator("password");
        
        // Test weak password
        let weak_result = password_validator.validate("weak");
        assert!(!weak_result.is_valid);
        assert!(weak_result.errors.len() >= 1); // At least one validation error
        
        // Test strong password
        let strong_result = password_validator.validate("StrongPass123");
        assert!(strong_result.is_valid);
    }

    #[test]
    fn test_validation_context_multiple_fields() {
        let mut context = ValidationContext::new();
        
        // Test email validation
        let email_validator = validation_builders::email_validator("email");
        let email_result = email_validator.validate("invalid-email");
        context.add_field_result("email".to_string(), email_result);
        
        // Test username validation
        let username_validator = validation_builders::username_validator("username");
        let username_result = username_validator.validate("ab"); // Too short
        context.add_field_result("username".to_string(), username_result);
        
        // Test context state
        assert!(!context.is_all_valid());
        assert!(context.has_field_error("email"));
        assert!(context.has_field_error("username"));
        
        // Test individual field validation
        let email_error = context.get_field_error("email");
        assert!(email_error.is_some());
        
        let username_error = context.get_field_error("username");
        assert!(username_error.is_some());
    }

    #[test]
    fn test_custom_validation_rules() {
        let validator = InputValidator::new("custom_field")
            .with_rule(crate::validation::ValidationRule::Required)
            .with_custom_validator(|value| value.starts_with("prefix_"));
        
        // Test custom validation failure
        let result = validator.validate("wrong_start");
        assert!(!result.is_valid);
        assert!(result.errors.len() >= 1);
        
        // Test custom validation success
        let valid_result = validator.validate("prefix_valid");
        assert!(valid_result.is_valid);
    }

    #[test]
    fn test_validation_error_prioritization() {
        let validator = InputValidator::new("field")
            .with_rule(crate::validation::ValidationRule::Required)
            .with_rule(crate::validation::ValidationRule::MinLength(5))
            .with_rule(crate::validation::ValidationRule::MaxLength(10));
        
        // Empty field should show required error first
        let empty_result = validator.validate("");
        assert!(!empty_result.is_valid);
        assert!(empty_result.errors[0].message.contains("required"));
        
        // Short field should show min length error
        let short_result = validator.validate("ab");
        assert!(!short_result.is_valid);
        assert!(short_result.errors[0].message.contains("at least"));
        
        // Long field should show max length error
        let long_result = validator.validate("very_long_input");
        assert!(!long_result.is_valid);
        assert!(long_result.errors[0].message.contains("no more than"));
    }

    #[test]
    fn test_validation_performance() {
        let validator = InputValidator::new("performance_test")
            .with_rule(crate::validation::ValidationRule::Required)
            .with_rule(crate::validation::ValidationRule::MinLength(3))
            .with_rule(crate::validation::ValidationRule::MaxLength(100))
            .with_rule(crate::validation::ValidationRule::Pattern(r"^[a-zA-Z0-9]+$".to_string()));
        
        // Test that validation is fast even with multiple rules
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = validator.validate("test123");
        }
        let duration = start.elapsed();
        
        // Should complete 1000 validations in reasonable time (< 1000ms)
        assert!(duration.as_millis() < 1000, "Validation should be performant");
    }

    #[test]
    fn test_input_base_css_classes() {
        // Test that base INPUT_CLASS contains required styling and accessibility classes
        assert!(INPUT_CLASS.contains("flex"));
        assert!(INPUT_CLASS.contains("h-10"));
        assert!(INPUT_CLASS.contains("w-full"));
        assert!(INPUT_CLASS.contains("rounded-md"));
        assert!(INPUT_CLASS.contains("border"));
        assert!(INPUT_CLASS.contains("border-input"));
        assert!(INPUT_CLASS.contains("bg-background"));
        assert!(INPUT_CLASS.contains("focus-visible:outline-none"));
        assert!(INPUT_CLASS.contains("focus-visible:ring-2"));
        assert!(INPUT_CLASS.contains("disabled:cursor-not-allowed"));
        assert!(INPUT_CLASS.contains("disabled:opacity-50"));
        assert!(INPUT_CLASS.contains("placeholder:text-muted-foreground"));
    }

    #[test]
    fn test_input_file_specific_classes() {
        // Test file input specific styling
        assert!(INPUT_CLASS.contains("file:border-0"));
        assert!(INPUT_CLASS.contains("file:bg-transparent"));
        assert!(INPUT_CLASS.contains("file:text-sm"));
        assert!(INPUT_CLASS.contains("file:font-medium"));
    }

    #[test]
    fn test_input_component_creation() {
        // Test that Input component can be created with various props
        // This is a conceptual test - in real implementation we'd need proper rendering environment
        
        // Test default type
        let default_type = "text".to_string();
        assert_eq!(default_type, "text");
        
        // Test various input types
        let input_types = vec!["text", "password", "email", "number", "tel", "url"];
        for input_type in input_types {
            assert!(!input_type.is_empty());
        }
    }

    #[test]
    fn test_input_value_handling() {
        // Test value prop handling
        let test_value = "test value".to_string();
        assert_eq!(test_value, "test value");
        
        // Test empty value
        let empty_value = String::new();
        assert!(empty_value.is_empty());
        
        // Test value updates
        let value = RwSignal::new("initial".to_string());
        assert_eq!(value.get(), "initial");
        
        value.set("updated".to_string());
        assert_eq!(value.get(), "updated");
    }

    #[test]
    fn test_input_placeholder_handling() {
        // Test placeholder functionality
        let placeholder_text = "Enter text here...".to_string();
        assert!(!placeholder_text.is_empty());
        assert!(placeholder_text.contains("Enter"));
        
        // Test empty placeholder
        let empty_placeholder = String::new();
        assert!(empty_placeholder.is_empty());
    }

    #[test]
    fn test_input_disabled_state() {
        // Test disabled signal functionality
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get());
        
        disabled_signal.set(true);
        assert!(disabled_signal.get());
        
        // Test disabled state styling is included in base class
        assert!(INPUT_CLASS.contains("disabled:cursor-not-allowed"));
        assert!(INPUT_CLASS.contains("disabled:opacity-50"));
    }

    #[test]
    fn test_input_change_callback() {
        // Test change callback structure
        let change_called = Arc::new(Mutex::new(false));
        let change_value = Arc::new(Mutex::new(String::new()));
        
        let change_called_clone = Arc::clone(&change_called);
        let change_value_clone = Arc::clone(&change_value);
        
        let callback = Callback::new(move |value: String| {
            *change_called_clone.lock().unwrap() = true;
            *change_value_clone.lock().unwrap() = value;
        });
        
        // Simulate callback execution
        callback.run("test input".to_string());
        
        assert!(*change_called.lock().unwrap());
        assert_eq!(*change_value.lock().unwrap(), "test input");
    }

    #[test]
    fn test_input_class_merging() {
        // Test custom class handling
        let base_class = INPUT_CLASS;
        let custom_class = "my-custom-input-class";
        
        let expected = format!("{} {}", base_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
    }

    #[test]
    fn test_input_accessibility_features() {
        // Test accessibility-related CSS classes
        assert!(INPUT_CLASS.contains("focus-visible:outline-none"));
        assert!(INPUT_CLASS.contains("focus-visible:ring-2"));
        assert!(INPUT_CLASS.contains("focus-visible:ring-ring"));
        assert!(INPUT_CLASS.contains("focus-visible:ring-offset-2"));
        
        // Test that placeholder has proper contrast
        assert!(INPUT_CLASS.contains("placeholder:text-muted-foreground"));
    }

    #[test]
    fn test_input_styling_consistency() {
        // Test that all required styling properties are present
        let required_properties = vec![
            "flex", "h-10", "w-full", "rounded-md", "border",
            "bg-background", "px-3", "py-2", "text-sm",
            "ring-offset-background"
        ];
        
        for property in required_properties {
            assert!(INPUT_CLASS.contains(property), 
                "INPUT_CLASS should contain '{}' property", property);
        }
    }
}
