#[cfg(test)]
mod integration_tests {
    use crate::validation::{validation_builders, ValidationRule, ValidationResult, InputValidator};

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
    fn test_form_validation_integration() {
        // Test complete form validation integration
        let mut validators = std::collections::HashMap::new();
        validators.insert("email".to_string(), validation_builders::email_validator("email"));
        validators.insert("password".to_string(), validation_builders::password_validator("password"));
        validators.insert("username".to_string(), validation_builders::username_validator("username"));
        
        // Test form data
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("email".to_string(), "invalid-email".to_string());
        form_data.insert("password".to_string(), "weak".to_string());
        form_data.insert("username".to_string(), "ab".to_string());
        
        // Test validation
        let mut all_errors = Vec::new();
        for (field, value) in &form_data {
            if let Some(validator) = validators.get(field) {
                let result = validator.validate(value);
                if !result.is_valid {
                    all_errors.extend(result.errors);
                }
            }
        }
        
        assert!(!all_errors.is_empty());
        assert!(all_errors.len() >= 3); // At least one error per field
        
        // Test with valid data
        form_data.insert("email".to_string(), "user@example.com".to_string());
        form_data.insert("password".to_string(), "Password123".to_string());
        form_data.insert("username".to_string(), "user123".to_string());
        
        let mut valid_errors = Vec::new();
        for (field, value) in &form_data {
            if let Some(validator) = validators.get(field) {
                let result = validator.validate(value);
                if !result.is_valid {
                    valid_errors.extend(result.errors);
                }
            }
        }
        
        assert!(valid_errors.is_empty());
    }

    #[test]
    fn test_validation_chain_integration() {
        // Test validation chain integration
        let validator = InputValidator::new("test_field")
            .required()
            .min_length(5)
            .max_length(20)
            .pattern(r"^[a-zA-Z0-9]+$".to_string());
        
        // Test empty value
        let empty_result = validator.validate("");
        assert!(!empty_result.is_valid);
        assert_eq!(empty_result.errors.len(), 1);
        assert_eq!(empty_result.errors[0].message, "This field is required");
        
        // Test too short
        let short_result = validator.validate("ab");
        assert!(!short_result.is_valid);
        assert_eq!(short_result.errors.len(), 1);
        assert!(short_result.errors[0].message.contains("at least 5 characters"));
        
        // Test too long
        let long_result = validator.validate("this_is_too_long_for_validation");
        assert!(!long_result.is_valid);
        assert_eq!(long_result.errors.len(), 1);
        assert!(long_result.errors[0].message.contains("no more than 20 characters"));
        
        // Test invalid pattern
        let pattern_result = validator.validate("invalid-pattern!");
        assert!(!pattern_result.is_valid);
        assert_eq!(pattern_result.errors.len(), 1);
        assert_eq!(pattern_result.errors[0].message, "Invalid format");
        
        // Test valid value
        let valid_result = validator.validate("valid123");
        assert!(valid_result.is_valid);
        assert!(valid_result.errors.is_empty());
    }

    #[test]
    fn test_validation_error_aggregation() {
        // Test error aggregation across multiple validators
        let email_validator = validation_builders::email_validator("email");
        let password_validator = validation_builders::password_validator("password");
        
        let mut aggregated_result = ValidationResult::new();
        
        // Test email validation
        let email_result = email_validator.validate("invalid-email");
        if !email_result.is_valid {
            aggregated_result.errors.extend(email_result.errors);
        }
        
        // Test password validation
        let password_result = password_validator.validate("weak");
        if !password_result.is_valid {
            aggregated_result.errors.extend(password_result.errors);
        }
        
        // Check aggregated result
        assert!(!aggregated_result.is_valid);
        assert!(aggregated_result.errors.len() >= 2); // At least one error per field
        
        // Test with valid data
        let mut valid_aggregated = ValidationResult::new();
        
        let valid_email_result = email_validator.validate("user@example.com");
        if !valid_email_result.is_valid {
            valid_aggregated.errors.extend(valid_email_result.errors);
        }
        
        let valid_password_result = password_validator.validate("Password123");
        if !valid_password_result.is_valid {
            valid_aggregated.errors.extend(valid_password_result.errors);
        }
        
        assert!(valid_aggregated.is_valid);
        assert!(valid_aggregated.errors.is_empty());
    }

    #[test]
    fn test_validation_performance_integration() {
        // Test validation performance with multiple validators
        let validators = vec![
            validation_builders::email_validator("email"),
            validation_builders::password_validator("password"),
            validation_builders::username_validator("username"),
            validation_builders::phone_validator("phone"),
        ];
        
        let test_data = vec![
            ("email", "user@example.com"),
            ("password", "Password123"),
            ("username", "user123"),
            ("phone", "+1 (555) 123-4567"),
        ];
        
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            for (field, value) in &test_data {
                if let Some(validator) = validators.iter().find(|v| v.field_name == *field) {
                    let _result = validator.validate(value);
                }
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[test]
    fn test_validation_memory_integration() {
        // Test memory usage with large validation sets
        let mut validators = std::collections::HashMap::new();
        
        // Create many validators
        for i in 0..100 {
            let field_name = format!("field_{}", i);
            let validator = InputValidator::new(&field_name)
                .required()
                .min_length(3)
                .max_length(50);
            validators.insert(field_name, validator);
        }
        
        // Test validation with many fields
        let mut form_data = std::collections::HashMap::new();
        for i in 0..100 {
            let field_name = format!("field_{}", i);
            let value = format!("value_{}", i);
            form_data.insert(field_name, value);
        }
        
        // Validate all fields
        let mut all_results = Vec::new();
        for (field, value) in &form_data {
            if let Some(validator) = validators.get(field) {
                let result = validator.validate(value);
                all_results.push(result);
            }
        }
        
        assert_eq!(all_results.len(), 100);
        
        // Check memory cleanup
        drop(all_results);
        drop(validators);
        drop(form_data);
        
        // Memory should be cleaned up
    }
}
