//! Input validation system for TDD implementation
//! 
//! This module provides comprehensive validation functionality for the Input component
//! following TDD patterns with proper error handling and real-time feedback.

pub mod types;
pub mod rules;
pub mod validator;

// Re-export commonly used types and functions
pub use types::{
    ValidationRule,
    ValidationError,
    ValidationResult,
    ValidationContext,
};

pub use validator::{
    InputValidator,
    validation_builders,
};

pub use rules::{
    validate_rule,
    validate_rules,
};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_validation_system_integration() {
        let mut context = ValidationContext::new();
        
        let email_validator = validation_builders::email_validator("email");
        let password_validator = validation_builders::password_validator("password");
        
        email_validator.validate_with_context("invalid-email", &mut context);
        password_validator.validate_with_context("short", &mut context);
        
        assert!(!context.is_all_valid());
        assert!(context.has_field_error("email"));
        assert!(context.has_field_error("password"));
    }

    #[test]
    fn test_validation_context_management() {
        let mut context = ValidationContext::new();
        
        let validator = InputValidator::new("test_field")
            .with_rule(ValidationRule::Required)
            .with_rule(ValidationRule::MinLength(3));
        
        validator.validate_with_context("", &mut context);
        assert!(!context.is_field_valid("test_field"));
        
        validator.validate_with_context("valid", &mut context);
        assert!(context.is_field_valid("test_field"));
    }
}
