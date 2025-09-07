//! Input validation system for TDD implementation
//! 
//! This module provides comprehensive validation functionality for the Input component
//! following TDD patterns with proper error handling and real-time feedback.

use leptos::prelude::*;
use std::collections::HashMap;

/// Validation rule types for different input validation scenarios
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Email,
    Pattern(String),
    Custom(String), // Store a description instead of the function for Clone/Debug/PartialEq
}

/// Validation error with field name and message
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub rule: ValidationRule,
}

/// Validation result containing errors and overall validity
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, field: impl Into<String>, message: impl Into<String>, rule: ValidationRule) {
        self.is_valid = false;
        self.errors.push(ValidationError {
            field: field.into(),
            message: message.into(),
            rule,
        });
    }

    pub fn get_error(&self, field: &str) -> Option<&ValidationError> {
        self.errors.iter().find(|error| error.field == field)
    }

    pub fn get_error_message(&self, field: &str) -> Option<&str> {
        self.get_error(field).map(|error| error.message.as_str())
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn clear_errors(&mut self) {
        self.is_valid = true;
        self.errors.clear();
    }
}

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

    pub fn required(mut self) -> Self {
        self.rules.push(ValidationRule::Required);
        self
    }

    pub fn min_length(mut self, length: usize) -> Self {
        self.rules.push(ValidationRule::MinLength(length));
        self
    }

    pub fn max_length(mut self, length: usize) -> Self {
        self.rules.push(ValidationRule::MaxLength(length));
        self
    }

    pub fn email(mut self) -> Self {
        self.rules.push(ValidationRule::Email);
        self
    }

    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.rules.push(ValidationRule::Pattern(pattern.into()));
        self
    }

    pub fn custom<F>(mut self, validator: F) -> Self 
    where 
        F: Fn(&str) -> bool + Send + Sync + 'static 
    {
        self.rules.push(ValidationRule::Custom("Custom validation".to_string()));
        self.custom_validators.push(Box::new(validator));
        self
    }

    pub fn validate(&self, value: &str) -> ValidationResult {
        let mut result = ValidationResult::new();

        for rule in &self.rules {
            match rule {
                ValidationRule::Required => {
                    if value.trim().is_empty() {
                        result.add_error(
                            &self.field_name,
                            "This field is required",
                            ValidationRule::Required,
                        );
                    }
                }
                ValidationRule::MinLength(min_len) => {
                    if value.len() < *min_len {
                        result.add_error(
                            &self.field_name,
                            format!("Must be at least {} characters", min_len),
                            ValidationRule::MinLength(*min_len),
                        );
                    }
                }
                ValidationRule::MaxLength(max_len) => {
                    if value.len() > *max_len {
                        result.add_error(
                            &self.field_name,
                            format!("Must be no more than {} characters", max_len),
                            ValidationRule::MaxLength(*max_len),
                        );
                    }
                }
                ValidationRule::Email => {
                    if !self.is_valid_email(value) {
                        result.add_error(
                            &self.field_name,
                            "Please enter a valid email address",
                            ValidationRule::Email,
                        );
                    }
                }
                ValidationRule::Pattern(pattern) => {
                    if let Ok(regex) = regex::Regex::new(pattern) {
                        if !regex.is_match(value) {
                            result.add_error(
                                &self.field_name,
                                "Invalid format",
                                ValidationRule::Pattern(pattern.clone()),
                            );
                        }
                    }
                }
                ValidationRule::Custom(_) => {
                    // Apply custom validators
                    for (i, validator) in self.custom_validators.iter().enumerate() {
                        if !validator(value) {
                            result.add_error(
                                &self.field_name,
                                "Invalid value",
                                ValidationRule::Custom(format!("Custom validation {}", i)),
                            );
                        }
                    }
                }
            }
        }

        result
    }

    fn is_valid_email(&self, email: &str) -> bool {
        // Basic email validation - can be enhanced with more sophisticated regex
        email.contains('@') && 
        email.contains('.') && 
        email.len() > 5 &&
        !email.starts_with('@') &&
        !email.ends_with('@') &&
        !email.starts_with('.') &&
        !email.ends_with('.')
    }
}

/// Validation context for managing multiple field validations
pub struct ValidationContext {
    pub validators: HashMap<String, Box<dyn Fn(&str) -> ValidationResult + Send + Sync>>,
    pub results: RwSignal<HashMap<String, ValidationResult>>,
}

impl ValidationContext {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            results: RwSignal::new(HashMap::new()),
        }
    }

    pub fn add_validator(&mut self, validator: InputValidator) {
        let field_name = validator.field_name.clone();
        let validator_fn = Box::new(move |value: &str| validator.validate(value));
        self.validators.insert(field_name, validator_fn);
    }

    pub fn validate_field(&self, field_name: &str, value: &str) -> ValidationResult {
        if let Some(validator) = self.validators.get(field_name) {
            let result = validator(value);
            
            // Update the results signal
            let mut current_results = self.results.get();
            current_results.insert(field_name.to_string(), result.clone());
            self.results.set(current_results);
            
            result
        } else {
            ValidationResult::new()
        }
    }

    pub fn validate_all(&self, values: &HashMap<String, String>) -> ValidationResult {
        let mut overall_result = ValidationResult::new();
        let mut field_results = HashMap::new();

        for (field_name, value) in values {
            if let Some(validator) = self.validators.get(field_name) {
                let result = validator(value);
                field_results.insert(field_name.clone(), result.clone());
                
                if !result.is_valid {
                    overall_result.is_valid = false;
                    overall_result.errors.extend(result.errors);
                }
            }
        }

        self.results.set(field_results);
        overall_result
    }

    pub fn get_field_error(&self, field_name: &str) -> Option<String> {
        self.results.get()
            .get(field_name)
            .and_then(|result| result.get_error_message(field_name))
            .map(|msg| msg.to_string())
    }

    pub fn is_field_valid(&self, field_name: &str) -> bool {
        self.results.get()
            .get(field_name)
            .map(|result| result.is_valid)
            .unwrap_or(true)
    }

    pub fn is_form_valid(&self) -> bool {
        self.results.get()
            .values()
            .all(|result| result.is_valid)
    }
}

/// Helper function to create common validation patterns
pub mod validation_builders {
    use super::*;

    pub fn email_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .required()
            .email()
    }

    pub fn password_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .required()
            .min_length(8)
            .pattern(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).*$".to_string())
    }

    pub fn username_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .required()
            .min_length(3)
            .max_length(20)
            .pattern(r"^[a-zA-Z0-9_]+$".to_string())
    }

    pub fn phone_validator(field_name: impl Into<String>) -> InputValidator {
        InputValidator::new(field_name)
            .pattern(r"^\+?[\d\s\-\(\)]+$".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_new() {
        let result = ValidationResult::new();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validation_result_add_error() {
        let mut result = ValidationResult::new();
        result.add_error("email", "Invalid email", ValidationRule::Email);
        
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].field, "email");
        assert_eq!(result.errors[0].message, "Invalid email");
    }

    #[test]
    fn test_input_validator_required() {
        let validator = InputValidator::new("test_field").required();
        let result = validator.validate("");
        
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].message, "This field is required");
    }

    #[test]
    fn test_input_validator_min_length() {
        let validator = InputValidator::new("test_field").min_length(5);
        let result = validator.validate("abc");
        
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("at least 5 characters"));
    }

    #[test]
    fn test_input_validator_email() {
        let validator = InputValidator::new("email").email();
        
        let invalid_result = validator.validate("invalid-email");
        assert!(!invalid_result.is_valid);
        
        let valid_result = validator.validate("user@example.com");
        assert!(valid_result.is_valid);
    }

    #[test]
    fn test_validation_context() {
        let mut context = ValidationContext::new();
        context.add_validator(InputValidator::new("email").required().email());
        
        let result = context.validate_field("email", "invalid");
        assert!(!result.is_valid);
        
        let error = context.get_field_error("email");
        assert!(error.is_some());
    }
}
