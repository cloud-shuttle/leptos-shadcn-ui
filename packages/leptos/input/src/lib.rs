//! Leptos port of shadcn/ui input

pub mod default;
pub mod new_york;

pub use default::{Input};
pub use new_york::{Input as InputNewYork};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod leptos_v0_8_compatibility_tests;
