//! Leptos port of shadcn/ui popover

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Popover};
pub use new_york::{Popover as PopoverNewYork};


mod tests;


mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;

#[cfg(test)]
mod real_tests;