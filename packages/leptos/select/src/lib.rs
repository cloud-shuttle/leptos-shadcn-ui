//! Leptos port of [shadcn/ui Select](https://ui.shadcn.com/docs/components/select).
//!
//! Component description here.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/select.html) for more documentation.

pub mod signal_managed;
pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::*;

#[cfg(feature = "new_york")]
pub use new_york as select;

// Real focused tests (replaces 891-line bloated file)
#[cfg(test)]
mod tests;

// Legacy tests (will be removed)
#[cfg(test)]
mod implementation_tests_legacy;


// Signal-managed exports
pub use signal_managed::*;