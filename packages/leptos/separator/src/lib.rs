//! Leptos port of shadcn/ui separator

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Separator};
pub use new_york::{Separator as SeparatorNewYork};

#[cfg(test)]
mod tests;


// Signal-managed exports
pub use signal_managed::*;