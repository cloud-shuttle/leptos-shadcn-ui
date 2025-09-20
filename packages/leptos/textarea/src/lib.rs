//! Leptos port of shadcn/ui textarea

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Textarea};
pub use new_york::{Textarea as TextareaNewYork};


mod tests;


mod tdd_tests;


mod implementation_tests;


// Signal-managed exports
pub use signal_managed::*;

#[cfg(test)]
