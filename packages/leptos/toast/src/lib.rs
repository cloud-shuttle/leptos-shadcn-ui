//! Leptos port of shadcn/ui toast

pub mod signal_managed;
pub mod default;
pub mod new_york;
pub mod sonner;

pub use default::{Toast};
pub use new_york::{Toast as ToastNewYork};
pub use sonner::{
    SonnerProvider, SonnerViewport, SonnerToast,
    ToastPosition, ToastTheme, ToastVariant, ToastAction, ToastData, ToastBuilder,
    toast
};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tdd_tests;

#[cfg(test)]
mod sonner_tests;

#[cfg(test)]
mod sonner_advanced_tests;


// Signal-managed exports
pub use signal_managed::*;