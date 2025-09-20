//! Leptos port of shadcn/ui button

pub mod default;
pub mod new_york;
pub mod signal_managed;
// TODO: Enable when API standards crate is ready for v1.0
// pub mod standardized;

pub use default::{Button, ButtonVariant, ButtonSize, ButtonChildProps};
pub use new_york::{Button as ButtonNewYork, ButtonVariant as ButtonVariantNewYork, ButtonSize as ButtonSizeNewYork, ButtonChildProps as ButtonChildPropsNewYork};
pub use signal_managed::{SignalManagedButton, EnhancedButton, SignalManagedButtonState, SignalManagedButtonChildProps};
// TODO: Enable when API standards crate is ready for v1.0
// pub use standardized::{StandardizedButton, StandardizedButtonProps};

// #[cfg(test)]
// mod tests;

// #[cfg(test)]
// mod tdd_tests_simplified;

// Real working tests (replaces placeholders)

mod tests_simple;

// Keep legacy tests for now (will phase out)

mod tdd_tests;

mod implementation_tests;

mod new_york_tests;

mod variant_comparison_tests;
