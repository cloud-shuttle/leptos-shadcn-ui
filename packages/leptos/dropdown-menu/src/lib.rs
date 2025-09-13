//! Leptos port of shadcn/ui dropdown-menu

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{DropdownMenu};
pub use new_york::{DropdownMenu as DropdownMenuNewYork};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;