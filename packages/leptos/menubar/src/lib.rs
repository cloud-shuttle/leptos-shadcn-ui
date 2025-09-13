//! Leptos port of shadcn/ui menubar

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Menubar};
pub use new_york::{Menubar as MenubarNewYork};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;