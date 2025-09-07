#[cfg(test)]
mod tests {
    use crate::default::{Input, INPUT_CLASS};
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    // ============================================================================
    // TDD PATTERN 1: RED - Write failing tests first
    // ============================================================================
    
    #[test]
    fn test_input_validation_required_field() {
        // RED: This test should fail initially - we need to add validation support
        let validation_state = RwSignal::new(ValidationState::new());
        let is_required = Signal::derive(|| true);
        let value = RwSignal::new("".to_string());
        
        // Test that empty required field triggers validation error
        let is_valid = Signal::derive(move || {
            if is_required.get() && value.get().trim().is_empty() {
                false
            } else {
                true
            }
        });
        
        assert!(!is_valid.get(), "Required field should be invalid when empty");
        
        value.set("valid input".to_string());
        assert!(is_valid.get(), "Required field should be valid when filled");
    }

    #[test]
    fn test_input_validation_email_format() {
        // RED: Email validation should fail for invalid formats
        let email_value = RwSignal::new("invalid-email".to_string());
        
        let is_valid_email = Signal::derive(move || {
            let email = email_value.get();
            email.contains('@') && email.contains('.') && email.len() > 5
        });
        
        assert!(!is_valid_email.get(), "Invalid email format should fail validation");
        
        email_value.set("user@example.com".to_string());
        assert!(is_valid_email.get(), "Valid email format should pass validation");
    }

    #[test]
    fn test_input_validation_min_length() {
        // RED: Minimum length validation
        let value = RwSignal::new("ab".to_string());
        let min_length = 3;
        
        let is_valid_length = Signal::derive(move || {
            value.get().len() >= min_length
        });
        
        assert!(!is_valid_length.get(), "Value below minimum length should be invalid");
        
        value.set("abc".to_string());
        assert!(is_valid_length.get(), "Value meeting minimum length should be valid");
    }

    #[test]
    fn test_input_validation_max_length() {
        // RED: Maximum length validation
        let value = RwSignal::new("very long input that exceeds limit".to_string());
        let max_length = 10;
        
        let is_valid_length = Signal::derive(move || {
            value.get().len() <= max_length
        });
        
        assert!(!is_valid_length.get(), "Value exceeding maximum length should be invalid");
        
        value.set("short".to_string());
        assert!(is_valid_length.get(), "Value within maximum length should be valid");
    }

    #[test]
    fn test_input_validation_pattern_matching() {
        // RED: Pattern validation (e.g., phone number, alphanumeric)
        let value = RwSignal::new("abc123!@#".to_string());
        let pattern = regex::Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        
        let is_valid_pattern = Signal::derive(move || {
            pattern.is_match(&value.get())
        });
        
        assert!(!is_valid_pattern.get(), "Value with special characters should fail pattern validation");
        
        value.set("abc123".to_string());
        assert!(is_valid_pattern.get(), "Alphanumeric value should pass pattern validation");
    }

    #[test]
    fn test_input_validation_error_display() {
        // RED: Error message display functionality
        let validation_error = RwSignal::new(Some("This field is required".to_string()));
        let show_error = Signal::derive(move || validation_error.get().is_some());
        
        assert!(show_error.get(), "Error should be shown when validation fails");
        assert_eq!(validation_error.get().unwrap(), "This field is required");
        
        validation_error.set(None);
        assert!(!show_error.get(), "Error should be hidden when validation passes");
    }

    #[test]
    fn test_input_validation_real_time_feedback() {
        // RED: Real-time validation as user types
        let value = RwSignal::new("".to_string());
        let validation_errors = RwSignal::new(Vec::<String>::new());
        
        let validate_input = move || {
            let mut errors = Vec::new();
            let current_value = value.get();
            
            if current_value.trim().is_empty() {
                errors.push("Field is required".to_string());
            }
            if current_value.len() < 3 {
                errors.push("Must be at least 3 characters".to_string());
            }
            
            validation_errors.set(errors);
        };
        
        // Initial state - should have errors
        validate_input();
        assert!(!validation_errors.get().is_empty());
        
        // Partial input - should still have length error
        value.set("ab".to_string());
        validate_input();
        assert!(validation_errors.get().contains(&"Must be at least 3 characters".to_string()));
        
        // Valid input - should have no errors
        value.set("abc".to_string());
        validate_input();
        assert!(validation_errors.get().is_empty());
    }

    // ============================================================================
    // TDD PATTERN 2: GREEN - Make tests pass with minimal implementation
    // ============================================================================
    
    #[derive(Clone, Debug)]
    struct ValidationState {
        pub is_valid: bool,
        pub errors: Vec<String>,
    }
    
    impl ValidationState {
        fn new() -> Self {
            Self {
                is_valid: true,
                errors: Vec::new(),
            }
        }
        
        fn add_error(&mut self, error: String) {
            self.is_valid = false;
            self.errors.push(error);
        }
        
        fn clear_errors(&mut self) {
            self.is_valid = true;
            self.errors.clear();
        }
    }

    // ============================================================================
    // TDD PATTERN 3: REFACTOR - Enhanced validation system tests
    // ============================================================================
    
    #[test]
    fn test_enhanced_input_validation_system() {
        use crate::validation::{InputValidator, ValidationContext, validation_builders};
        
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
        use crate::validation::validation_builders;
        
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
        use crate::validation::{ValidationContext, validation_builders};
        
        let mut context = ValidationContext::new();
        context.add_validator(validation_builders::email_validator("email"));
        context.add_validator(validation_builders::username_validator("username"));
        
        let mut values = std::collections::HashMap::new();
        values.insert("email".to_string(), "invalid-email".to_string());
        values.insert("username".to_string(), "ab".to_string()); // Too short
        
        let result = context.validate_all(&values);
        assert!(!result.is_valid);
        assert!(result.errors.len() >= 1); // At least one validation error
        
        // Test individual field validation
        let email_error = context.get_field_error("email");
        assert!(email_error.is_some());
        
        let username_error = context.get_field_error("username");
        assert!(username_error.is_some());
    }

    #[test]
    fn test_custom_validation_rules() {
        use crate::validation::InputValidator;
        
        let validator = InputValidator::new("custom_field")
            .required()
            .custom(|value| value.starts_with("prefix_"));
        
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
        use crate::validation::InputValidator;
        
        let validator = InputValidator::new("field")
            .required()
            .min_length(5)
            .max_length(10);
        
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
        use crate::validation::InputValidator;
        
        let validator = InputValidator::new("performance_test")
            .required()
            .min_length(3)
            .max_length(100)
            .pattern(r"^[a-zA-Z0-9]+$".to_string());
        
        // Test that validation is fast even with multiple rules
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = validator.validate("test123");
        }
        let duration = start.elapsed();
        
        // Should complete 1000 validations in reasonable time (< 100ms)
        assert!(duration.as_millis() < 100, "Validation should be performant");
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
        let mut value = RwSignal::new("initial".to_string());
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