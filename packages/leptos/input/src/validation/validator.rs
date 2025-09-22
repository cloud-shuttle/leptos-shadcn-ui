//! Input validator implementation for the Input component
//! 
//! This module provides the main InputValidator struct and its functionality.

use super::types::{ValidationRule, ValidationResult, ValidationContext};
use super::rules::validate_rules;

/// Input validator that applies multiple validation rules
pub struct InputValidator {
    pub field_name: String,
    pub rules: Vec<ValidationRule>,
    pub custom_validators: Vec<Box<dyn Fn(&str) -> bool + Send + Sync>>,
}

impl InputValidator {
    pub fn new(field_name: impl Into<String>) -> Self {
        Self {
            field_name: field_name.into(),
            rules: Vec::new(),
            custom_validators: Vec::new(),
        }
    }

    pub fn with_rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn with_rules(mut self, rules: Vec<ValidationRule>) -> Self {
        self.rules.extend(rules);
        self
    }

    pub fn with_custom_validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.custom_validators.push(Box::new(validator));
        self
    }

    pub fn validate(&self, value: &str) -> ValidationResult {
        let mut result = validate_rules(value, &self.rules, &self.field_name);
        
        // Apply custom validators
        for validator in &self.custom_validators {
            if !validator(value) {
                result.add_error(
                    &self.field_name,
                    "Custom validation failed",
                    ValidationRule::Custom("Custom validator".to_string()),
                );
            }
        }
        
        result
    }

    pub fn validate_with_context(&self, value: &str, context: &mut ValidationContext) {
        let result = self.validate(value);
        context.add_field_result(self.field_name.clone(), result);
    }
}

/// Builder functions for common validation scenarios
pub mod validation_builders {
    use super::*;

    pub fn email_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .with_rule(ValidationRule::Required)
            .with_rule(ValidationRule::Email)
    }

    pub fn password_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .with_rule(ValidationRule::Required)
            .with_rule(ValidationRule::MinLength(8))
            .with_rule(ValidationRule::MaxLength(128))
    }

    pub fn username_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .with_rule(ValidationRule::Required)
            .with_rule(ValidationRule::MinLength(3))
            .with_rule(ValidationRule::MaxLength(20))
            .with_rule(ValidationRule::Pattern("alphanumeric".to_string()))
    }

    pub fn phone_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .with_rule(ValidationRule::Required)
            .with_rule(ValidationRule::Pattern("phone".to_string()))
    }

    pub fn required_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .with_rule(ValidationRule::Required)
    }

    pub fn optional_email_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .with_rule(ValidationRule::Email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_validator_creation() {
        let validator = InputValidator::new("test_field");
        assert_eq!(validator.field_name, "test_field");
        assert!(validator.rules.is_empty());
    }

    #[test]
    fn test_input_validator_with_rules() {
        let validator = InputValidator::new("test_field")
            .with_rule(ValidationRule::Required)
            .with_rule(ValidationRule::MinLength(3));
        
        assert_eq!(validator.rules.len(), 2);
    }

    #[test]
    fn test_input_validator_validation() {
        let validator = InputValidator::new("test_field")
            .with_rule(ValidationRule::Required)
            .with_rule(ValidationRule::MinLength(3));
        
        let result = validator.validate("");
        assert!(!result.is_valid);
        assert!(result.has_field_error("test_field"));
        
        let result = validator.validate("valid");
        assert!(result.is_valid);
    }

    #[test]
    fn test_validation_builders() {
        let email_validator = validation_builders::email_validator("email");
        let result = email_validator.validate("invalid-email");
        assert!(!result.is_valid);
        
        let result = email_validator.validate("test@example.com");
        assert!(result.is_valid);
    }

    #[test]
    fn test_custom_validator() {
        let validator = InputValidator::new("test_field")
            .with_custom_validator(|value| value.starts_with("prefix"));
        
        let result = validator.validate("prefix_valid");
        assert!(result.is_valid);
        
        let result = validator.validate("invalid");
        assert!(!result.is_valid);
    }
}
