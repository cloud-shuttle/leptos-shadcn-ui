//! Leptos port of shadcn/ui textarea

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Textarea};
pub use new_york::{Textarea as TextareaNewYork};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;