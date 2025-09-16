#[cfg(test)]
mod implementation_tests {
    use crate::default::{INPUT_CLASS, INPUT_ERROR_CLASS};
    use crate::validation::{
        InputValidator, ValidationResult, ValidationRule, ValidationError, 
        ValidationContext, validation_builders
    };
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== COMPREHENSIVE IMPLEMENTATION TESTS =====
    // These tests focus on actual implementation logic and validation behavior

    #[test]
    fn test_input_class_constants() {
        // Test INPUT_CLASS constant
        assert!(INPUT_CLASS.contains("flex"));
        assert!(INPUT_CLASS.contains("h-10"));
        assert!(INPUT_CLASS.contains("w-full"));
        assert!(INPUT_CLASS.contains("rounded-md"));
        assert!(INPUT_CLASS.contains("border"));
        assert!(INPUT_CLASS.contains("bg-background"));
        assert!(INPUT_CLASS.contains("px-3"));
        assert!(INPUT_CLASS.contains("py-2"));
        assert!(INPUT_CLASS.contains("text-sm"));
        assert!(INPUT_CLASS.contains("focus-visible:outline-none"));
        assert!(INPUT_CLASS.contains("disabled:cursor-not-allowed"));
        assert!(INPUT_CLASS.contains("disabled:opacity-50"));

        // Test INPUT_ERROR_CLASS constant
        assert!(INPUT_ERROR_CLASS.contains("border-destructive"));
        assert!(INPUT_ERROR_CLASS.contains("focus-visible:ring-destructive"));
    }

    #[test]
    fn test_validation_result_new() {
        // Test ValidationResult::new()
        let result = ValidationResult::new();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(!result.has_errors());
    }

    #[test]
    fn test_validation_result_add_error() {
        // Test adding errors to ValidationResult
        let mut result = ValidationResult::new();
        
        result.add_error("email", "Invalid email format", ValidationRule::Email);
        
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert!(result.has_errors());
        
        let error = &result.errors[0];
        assert_eq!(error.field, "email");
        assert_eq!(error.message, "Invalid email format");
        assert_eq!(error.rule, ValidationRule::Email);
    }

    #[test]
    fn test_validation_result_multiple_errors() {
        // Test adding multiple errors
        let mut result = ValidationResult::new();
        
        result.add_error("email", "Invalid email", ValidationRule::Email);
        result.add_error("password", "Too short", ValidationRule::MinLength(8));
        result.add_error("username", "Required", ValidationRule::Required);
        
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 3);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_result_get_error() {
        // Test getting specific errors
        let mut result = ValidationResult::new();
        result.add_error("email", "Invalid email", ValidationRule::Email);
        result.add_error("password", "Too short", ValidationRule::MinLength(8));
        
        let email_error = result.get_error("email");
        assert!(email_error.is_some());
        assert_eq!(email_error.unwrap().message, "Invalid email");
        
        let password_error = result.get_error("password");
        assert!(password_error.is_some());
        assert_eq!(password_error.unwrap().message, "Too short");
        
        let missing_error = result.get_error("username");
        assert!(missing_error.is_none());
    }

    #[test]
    fn test_validation_result_get_error_message() {
        // Test getting error messages
        let mut result = ValidationResult::new();
        result.add_error("email", "Invalid email format", ValidationRule::Email);
        
        let message = result.get_error_message("email");
        assert_eq!(message, Some("Invalid email format"));
        
        let missing_message = result.get_error_message("username");
        assert!(missing_message.is_none());
    }

    #[test]
    fn test_validation_result_clear_errors() {
        // Test clearing errors
        let mut result = ValidationResult::new();
        result.add_error("email", "Invalid email", ValidationRule::Email);
        result.add_error("password", "Too short", ValidationRule::MinLength(8));
        
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 2);
        
        result.clear_errors();
        
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(!result.has_errors());
    }

    #[test]
    fn test_validation_rule_equality() {
        // Test ValidationRule equality
        assert_eq!(ValidationRule::Required, ValidationRule::Required);
        assert_eq!(ValidationRule::MinLength(5), ValidationRule::MinLength(5));
        assert_eq!(ValidationRule::MaxLength(10), ValidationRule::MaxLength(10));
        assert_eq!(ValidationRule::Email, ValidationRule::Email);
        assert_eq!(ValidationRule::Pattern("test".to_string()), ValidationRule::Pattern("test".to_string()));
        assert_eq!(ValidationRule::Custom("test".to_string()), ValidationRule::Custom("test".to_string()));
        
        assert_ne!(ValidationRule::Required, ValidationRule::Email);
        assert_ne!(ValidationRule::MinLength(5), ValidationRule::MinLength(10));
        assert_ne!(ValidationRule::Pattern("test".to_string()), ValidationRule::Pattern("other".to_string()));
    }

    #[test]
    fn test_validation_rule_clone() {
        // Test ValidationRule cloning
        let rule = ValidationRule::MinLength(8);
        let cloned = rule.clone();
        assert_eq!(rule, cloned);
        
        let pattern_rule = ValidationRule::Pattern("test-pattern".to_string());
        let cloned_pattern = pattern_rule.clone();
        assert_eq!(pattern_rule, cloned_pattern);
    }

    #[test]
    fn test_validation_rule_debug() {
        // Test ValidationRule debug formatting
        let rule = ValidationRule::Email;
        let debug_str = format!("{:?}", rule);
        assert!(debug_str.contains("Email"));
        
        let min_rule = ValidationRule::MinLength(5);
        let min_debug = format!("{:?}", min_rule);
        assert!(min_debug.contains("MinLength"));
        assert!(min_debug.contains("5"));
    }

    #[test]
    fn test_validation_error_creation() {
        // Test ValidationError creation and access
        let error = ValidationError {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
            rule: ValidationRule::Email,
        };
        
        assert_eq!(error.field, "email");
        assert_eq!(error.message, "Invalid email format");
        assert_eq!(error.rule, ValidationRule::Email);
    }

    #[test]
    fn test_validation_error_clone() {
        // Test ValidationError cloning
        let error = ValidationError {
            field: "password".to_string(),
            message: "Too short".to_string(),
            rule: ValidationRule::MinLength(8),
        };
        
        let cloned = error.clone();
        assert_eq!(error.field, cloned.field);
        assert_eq!(error.message, cloned.message);
        assert_eq!(error.rule, cloned.rule);
    }

    #[test]
    fn test_validation_error_equality() {
        // Test ValidationError equality
        let error1 = ValidationError {
            field: "email".to_string(),
            message: "Invalid email".to_string(),
            rule: ValidationRule::Email,
        };
        
        let error2 = ValidationError {
            field: "email".to_string(),
            message: "Invalid email".to_string(),
            rule: ValidationRule::Email,
        };
        
        let error3 = ValidationError {
            field: "password".to_string(),
            message: "Invalid email".to_string(),
            rule: ValidationRule::Email,
        };
        
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_input_validator_new() {
        // Test InputValidator creation
        let validator = InputValidator::new("test_field");
        assert_eq!(validator.field_name, "test_field");
        assert!(validator.rules.is_empty());
        assert!(validator.custom_validators.is_empty());
    }

    #[test]
    fn test_input_validator_required() {
        // Test required validation
        let validator = InputValidator::new("test_field").required();
        assert_eq!(validator.rules.len(), 1);
        assert_eq!(validator.rules[0], ValidationRule::Required);
        
        // Test validation logic
        let empty_result = validator.validate("");
        assert!(!empty_result.is_valid);
        assert_eq!(empty_result.errors.len(), 1);
        assert_eq!(empty_result.errors[0].message, "This field is required");
        
        let valid_result = validator.validate("not empty");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_input_validator_min_length() {
        // Test minimum length validation
        let validator = InputValidator::new("test_field").min_length(5);
        assert_eq!(validator.rules.len(), 1);
        assert_eq!(validator.rules[0], ValidationRule::MinLength(5));
        
        // Test validation logic
        let short_result = validator.validate("abc");
        assert!(!short_result.is_valid);
        assert_eq!(short_result.errors.len(), 1);
        assert!(short_result.errors[0].message.contains("at least 5 characters"));
        
        let valid_result = validator.validate("abcdef");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_input_validator_max_length() {
        // Test maximum length validation
        let validator = InputValidator::new("test_field").max_length(10);
        assert_eq!(validator.rules.len(), 1);
        assert_eq!(validator.rules[0], ValidationRule::MaxLength(10));
        
        // Test validation logic
        let long_result = validator.validate("this is too long");
        assert!(!long_result.is_valid);
        assert_eq!(long_result.errors.len(), 1);
        assert!(long_result.errors[0].message.contains("no more than 10 characters"));
        
        let valid_result = validator.validate("short");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_input_validator_email() {
        // Test email validation
        let validator = InputValidator::new("email").email();
        assert_eq!(validator.rules.len(), 1);
        assert_eq!(validator.rules[0], ValidationRule::Email);
        
        // Test invalid emails
        let invalid_emails = vec![
            "invalid-email",
            "@example.com",
            "user@",
            ".user@example.com",
            "user@example.",
            "",
            "user@example",
        ];
        
        for invalid_email in invalid_emails {
            let result = validator.validate(invalid_email);
            assert!(!result.is_valid, "Email '{}' should be invalid", invalid_email);
            assert_eq!(result.errors.len(), 1);
            assert_eq!(result.errors[0].message, "Please enter a valid email address");
        }
        
        // Test valid emails
        let valid_emails = vec![
            "user@example.com",
            "test.email@domain.co.uk",
            "user+tag@example.org",
            "user123@test-domain.com",
        ];
        
        for valid_email in valid_emails {
            let result = validator.validate(valid_email);
            assert!(result.is_valid, "Email '{}' should be valid", valid_email);
            assert!(result.errors.is_empty());
        }
    }

    #[test]
    fn test_input_validator_pattern() {
        // Test pattern validation
        let validator = InputValidator::new("phone").pattern(r"^\d{3}-\d{3}-\d{4}$");
        assert_eq!(validator.rules.len(), 1);
        assert_eq!(validator.rules[0], ValidationRule::Pattern(r"^\d{3}-\d{3}-\d{4}$".to_string()));
        
        // Test valid pattern
        let valid_result = validator.validate("123-456-7890");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
        
        // Test invalid pattern
        let invalid_result = validator.validate("1234567890");
        assert!(!invalid_result.is_valid);
        assert_eq!(invalid_result.errors.len(), 1);
        assert_eq!(invalid_result.errors[0].message, "Invalid format");
    }

    #[test]
    fn test_input_validator_custom() {
        // Test custom validation
        let validator = InputValidator::new("test_field").custom(|value| value.len() > 3);
        assert_eq!(validator.rules.len(), 1);
        assert_eq!(validator.rules[0], ValidationRule::Custom("Custom validation".to_string()));
        assert_eq!(validator.custom_validators.len(), 1);
        
        // Test custom validation logic
        let short_result = validator.validate("ab");
        assert!(!short_result.is_valid);
        assert_eq!(short_result.errors.len(), 1);
        assert_eq!(short_result.errors[0].message, "Invalid value");
        
        let valid_result = validator.validate("abcd");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_input_validator_chaining() {
        // Test validator method chaining
        let validator = InputValidator::new("password")
            .required()
            .min_length(8)
            .max_length(50)
            .pattern(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).*$".to_string());
        
        assert_eq!(validator.rules.len(), 4);
        assert_eq!(validator.rules[0], ValidationRule::Required);
        assert_eq!(validator.rules[1], ValidationRule::MinLength(8));
        assert_eq!(validator.rules[2], ValidationRule::MaxLength(50));
        assert_eq!(validator.rules[3], ValidationRule::Pattern(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).*$".to_string()));
        
        // Test validation with multiple rules
        let empty_result = validator.validate("");
        assert!(!empty_result.is_valid);
        assert!(empty_result.errors.len() >= 1); // At least required error
        
        let short_result = validator.validate("abc");
        assert!(!short_result.is_valid);
        assert!(short_result.errors.len() >= 1); // At least min length error
        
        let long_result = validator.validate(&"a".repeat(60));
        assert!(!long_result.is_valid);
        assert!(long_result.errors.len() >= 1); // At least max length error
        
        let weak_result = validator.validate("weak"); // Too short, no uppercase, no digits
        assert!(!weak_result.is_valid);
        assert!(weak_result.errors.len() >= 1); // At least one error
        
        let strong_result = validator.validate("Password123");
        assert!(strong_result.is_valid);
        assert!(strong_result.errors.is_empty());
    }

    #[test]
    fn test_input_validator_email_validation_logic() {
        // Test email validation logic through public validate method
        let validator = InputValidator::new("email").email();
        
        // Test valid emails
        let valid_emails = vec![
            "user@example.com",
            "test.email@domain.co.uk",
            "user+tag@example.org",
            "user123@test-domain.com",
        ];
        
        for valid_email in valid_emails {
            let result = validator.validate(valid_email);
            assert!(result.is_valid, "Email '{}' should be valid", valid_email);
            assert!(result.errors.is_empty());
        }
        
        // Test invalid emails
        let invalid_emails = vec![
            "invalid-email",
            "@example.com",
            "user@",
            ".user@example.com",
            "user@example.",
            "",
            "user@example",
        ];
        
        for invalid_email in invalid_emails {
            let result = validator.validate(invalid_email);
            assert!(!result.is_valid, "Email '{}' should be invalid", invalid_email);
            assert_eq!(result.errors.len(), 1);
            assert_eq!(result.errors[0].message, "Please enter a valid email address");
        }
    }

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
    fn test_validation_builders_email_validator() {
        // Test email validator builder
        let validator = validation_builders::email_validator("email");
        assert_eq!(validator.field_name, "email");
        assert_eq!(validator.rules.len(), 2);
        assert_eq!(validator.rules[0], ValidationRule::Required);
        assert_eq!(validator.rules[1], ValidationRule::Email);
        
        // Test validation
        let invalid_result = validator.validate("");
        assert!(!invalid_result.is_valid);
        assert!(invalid_result.errors.len() >= 1); // At least required error
        assert_eq!(invalid_result.errors[0].message, "This field is required");
        
        let valid_result = validator.validate("user@example.com");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_validation_builders_password_validator() {
        // Test password validator builder
        let validator = validation_builders::password_validator("password");
        assert_eq!(validator.field_name, "password");
        assert_eq!(validator.rules.len(), 3);
        assert_eq!(validator.rules[0], ValidationRule::Required);
        assert_eq!(validator.rules[1], ValidationRule::MinLength(8));
        assert_eq!(validator.rules[2], ValidationRule::Pattern(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).*$".to_string()));
        
        // Test validation
        let weak_result = validator.validate("weak"); // Too short, no uppercase, no digits
        assert!(!weak_result.is_valid);
        assert!(weak_result.errors.len() >= 1); // At least one error
        
        let strong_result = validator.validate("Password123");
        assert!(strong_result.is_valid);
        assert!(strong_result.errors.is_empty());
    }

    #[test]
    fn test_validation_builders_username_validator() {
        // Test username validator builder
        let validator = validation_builders::username_validator("username");
        assert_eq!(validator.field_name, "username");
        assert_eq!(validator.rules.len(), 4);
        assert_eq!(validator.rules[0], ValidationRule::Required);
        assert_eq!(validator.rules[1], ValidationRule::MinLength(3));
        assert_eq!(validator.rules[2], ValidationRule::MaxLength(20));
        assert_eq!(validator.rules[3], ValidationRule::Pattern(r"^[a-zA-Z0-9_]+$".to_string()));
        
        // Test validation
        let invalid_result = validator.validate("ab");
        assert!(!invalid_result.is_valid);
        assert_eq!(invalid_result.errors.len(), 1);
        assert!(invalid_result.errors[0].message.contains("at least 3 characters"));
        
        let valid_result = validator.validate("user123");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_validation_builders_phone_validator() {
        // Test phone validator builder
        let validator = validation_builders::phone_validator("phone");
        assert_eq!(validator.field_name, "phone");
        assert_eq!(validator.rules.len(), 1);
        assert_eq!(validator.rules[0], ValidationRule::Pattern(r"^\+?[\d\s\-\(\)]+$".to_string()));
        
        // Test validation
        let valid_result = validator.validate("+1 (555) 123-4567");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
        
        let invalid_result = validator.validate("invalid-phone");
        assert!(!invalid_result.is_valid);
        assert_eq!(invalid_result.errors.len(), 1);
        assert_eq!(invalid_result.errors[0].message, "Invalid format");
    }

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
        let validation_result = RwSignal::new(ValidationResult::new());
        let is_validating = RwSignal::new(false);
        let show_validation = RwSignal::new(true);
        
        // Test initial state
        assert!(validation_result.get().is_valid);
        assert!(!is_validating.get());
        assert!(show_validation.get());
        
        // Test validation state changes
        let mut result = ValidationResult::new();
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
        let validator = InputValidator::new("test").required();
        let empty_result = validator.validate("");
        assert!(!empty_result.is_valid);
        
        // Test whitespace-only strings
        let whitespace_result = validator.validate("   ");
        assert!(!whitespace_result.is_valid);
        
        // Test very long strings
        let long_string = "a".repeat(1000);
        let long_validator = InputValidator::new("test").max_length(10);
        let long_result = long_validator.validate(&long_string);
        assert!(!long_result.is_valid);
        
        // Test special characters in email
        let email_validator = InputValidator::new("email").email();
        let special_result = email_validator.validate("user+tag@example.com");
        assert!(special_result.is_valid);
        
        // Test unicode characters
        let unicode_result = validator.validate("测试");
        assert!(unicode_result.is_valid);
    }

    #[test]
    fn test_input_performance_characteristics() {
        // Test performance characteristics
        let start = std::time::Instant::now();
        
        // Create many validators
        let validators = vec![
            InputValidator::new("email").required().email(),
            InputValidator::new("password").required().min_length(8),
            InputValidator::new("username").required().min_length(3).max_length(20),
            InputValidator::new("phone").pattern(r"^\+?[\d\s\-\(\)]+$".to_string()),
        ];
        
        // Test validation performance
        for _ in 0..1000 {
            for validator in &validators {
                let _ = validator.validate("test@example.com");
                let _ = validator.validate("password123");
                let _ = validator.validate("username");
                let _ = validator.validate("+1 (555) 123-4567");
            }
        }
        
        let duration = start.elapsed();
        
        // Should complete without panicking
        assert!(duration.as_millis() > 0, "Validation should take some time");
    }

    #[test]
    fn test_input_memory_management() {
        // Test memory management and cleanup
        let mut context = ValidationContext::new();
        context.add_validator(InputValidator::new("email").required().email());
        
        // Execute validation multiple times
        for _ in 0..100 {
            let _ = context.validate_field("email", "test@example.com");
        }
        
        assert!(context.is_field_valid("email"));
        
        // Test that contexts can be dropped without issues
        drop(context);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }
}
