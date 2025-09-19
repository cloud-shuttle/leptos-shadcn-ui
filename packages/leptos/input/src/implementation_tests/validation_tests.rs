#[cfg(test)]
mod validation_tests {
    use crate::default::{INPUT_CLASS, INPUT_ERROR_CLASS};
    use crate::validation::{
        InputValidator, ValidationResult, ValidationRule, ValidationError
    };

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
}
