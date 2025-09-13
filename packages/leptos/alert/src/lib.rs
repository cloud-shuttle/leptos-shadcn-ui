//! Leptos port of shadcn/ui alert

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{Alert, AlertTitle, AlertDescription, AlertVariant};
pub use new_york::{Alert as AlertNewYork, AlertTitle as AlertTitleNewYork, AlertDescription as AlertDescriptionNewYork, AlertVariant as AlertVariantNewYork};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;