//! Leptos port of shadcn/ui scroll-area

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{ScrollArea};
pub use new_york::{ScrollArea as ScrollAreaNewYork};

#[cfg(test)]

mod tests;


// Signal-managed exports
pub use signal_managed::*;