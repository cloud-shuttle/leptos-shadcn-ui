//! Core validation types for the Input component
//! 
//! This module defines the fundamental types used throughout the validation system.

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

    pub fn get_field_error(&self, field: &str) -> Option<&ValidationError> {
        self.errors.iter().find(|error| error.field == field)
    }

    pub fn has_field_error(&self, field: &str) -> bool {
        self.errors.iter().any(|error| error.field == field)
    }

    pub fn is_field_valid(&self, field: &str) -> bool {
        !self.has_field_error(field)
    }

    pub fn has_errors(&self) -> bool {
        !self.is_valid
    }

    pub fn get_error_message(&self, field: &str) -> Option<&str> {
        self.get_field_error(field).map(|error| error.message.as_str())
    }
}

/// Validation context for managing multiple field validations
#[derive(Clone, Debug)]
pub struct ValidationContext {
    pub results: HashMap<String, ValidationResult>,
}

impl ValidationContext {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    pub fn add_field_result(&mut self, field: String, result: ValidationResult) {
        self.results.insert(field, result);
    }

    pub fn get_field_result(&self, field: &str) -> Option<&ValidationResult> {
        self.results.get(field)
    }

    pub fn get_field_error(&self, field: &str) -> Option<&ValidationError> {
        self.results.get(field)
            .and_then(|result| result.get_field_error(field))
    }

    pub fn has_field_error(&self, field: &str) -> bool {
        self.results.get(field)
            .map(|result| result.has_field_error(field))
            .unwrap_or(false)
    }

    pub fn is_field_valid(&self, field: &str) -> bool {
        self.results.get(field)
            .map(|result| result.is_valid)
            .unwrap_or(true)
    }

    pub fn is_all_valid(&self) -> bool {
        self.results.values().all(|result| result.is_valid)
    }

    pub fn get_all_errors(&self) -> Vec<&ValidationError> {
        self.results.values()
            .flat_map(|result| &result.errors)
            .collect()
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}
