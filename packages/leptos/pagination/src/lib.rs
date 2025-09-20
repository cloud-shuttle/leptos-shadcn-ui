#[cfg(feature = "new_york")]
pub use new_york::*;

#[cfg(not(feature = "new_york"))]
pub use default::*;

#[cfg(feature = "new_york")]
mod new_york;

#[cfg(not(feature = "new_york"))]
mod default;

pub mod signal_managed;

mod tests;

mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
