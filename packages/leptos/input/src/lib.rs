//! Leptos port of shadcn/ui input

pub mod default;
pub mod new_york;
pub mod validation;

pub use default::{Input};
pub use new_york::{Input as InputNewYork};
pub use validation::{
    ValidationRule, ValidationError, ValidationResult, 
    InputValidator, ValidationContext, validation_builders
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod leptos_v0_8_compatibility_tests;
