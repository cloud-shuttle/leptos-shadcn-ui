//! Leptos port of shadcn/ui toggle

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Toggle};
pub use new_york::{Toggle as ToggleNewYork};


mod tests;

mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;

#[cfg(test)]
