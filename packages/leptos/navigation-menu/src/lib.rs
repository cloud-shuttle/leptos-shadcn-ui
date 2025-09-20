//! Leptos port of shadcn/ui navigation-menu

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{NavigationMenu};
pub use new_york::{NavigationMenu as NavigationMenuNewYork};


mod tests;

mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;

#[cfg(test)]
mod real_tests;