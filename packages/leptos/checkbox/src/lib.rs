//! Leptos port of shadcn/ui checkbox

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Checkbox};
pub use new_york::{Checkbox as CheckboxNewYork};

mod tests;

#[cfg(test)]
#[cfg(test)]
mod tdd_tests;

mod implementation_tests;

// Signal-managed exports
pub use signal_managed::*;
