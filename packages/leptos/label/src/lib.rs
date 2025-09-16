//! Leptos port of shadcn/ui label

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Label};
pub use new_york::{Label as LabelNewYork};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tdd_tests;

#[cfg(test)]
mod implementation_tests;


// Signal-managed exports
pub use signal_managed::*;