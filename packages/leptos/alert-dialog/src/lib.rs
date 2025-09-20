//! Leptos port of shadcn/ui alert dialog

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogHeader,
    AlertDialogFooter, AlertDialogTitle, AlertDialogDescription,
    AlertDialogAction, AlertDialogCancel, AlertDialogOverlay,
};

pub use new_york::{
    AlertDialog as AlertDialogNewYork,
    AlertDialogTrigger as AlertDialogTriggerNewYork,
    AlertDialogContent as AlertDialogContentNewYork,
    AlertDialogHeader as AlertDialogHeaderNewYork,
    AlertDialogFooter as AlertDialogFooterNewYork,
    AlertDialogTitle as AlertDialogTitleNewYork,
    AlertDialogDescription as AlertDialogDescriptionNewYork,
    AlertDialogAction as AlertDialogActionNewYork,
    AlertDialogCancel as AlertDialogCancelNewYork,
    AlertDialogOverlay as AlertDialogOverlayNewYork,
};

#[cfg(test)]

mod tests;

// Signal-managed exports
pub use signal_managed::*;