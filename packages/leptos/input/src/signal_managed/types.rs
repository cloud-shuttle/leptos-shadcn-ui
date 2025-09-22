//! Signal-managed Input component types and constants
//! 
//! This module contains the core types and constants for the signal-managed Input component.

use crate::validation::ValidationResult;

/// Base CSS class for input elements
pub const INPUT_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

/// CSS class for input elements with validation errors
pub const INPUT_ERROR_CLASS: &str = "border-destructive focus-visible:ring-destructive";

/// Signal-managed input state
#[derive(Debug, Clone, PartialEq)]
pub struct SignalManagedInputState {
    pub value: String,
    pub placeholder: String,
    pub disabled: bool,
    pub input_type: String,
    pub validation_result: ValidationResult,
    pub is_validating: bool,
    pub has_error: bool,
    pub focus_count: u32,
}

impl Default for SignalManagedInputState {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            disabled: false,
            input_type: "text".to_string(),
            validation_result: ValidationResult::new(),
            is_validating: false,
            has_error: false,
            focus_count: 0,
        }
    }
}
