//! Leptos port of shadcn/ui dialog

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, DialogClose
};
pub use new_york::{
    Dialog as DialogNewYork, DialogTrigger as DialogTriggerNewYork, DialogContent as DialogContentNewYork, 
    DialogHeader as DialogHeaderNewYork, DialogTitle as DialogTitleNewYork, DialogDescription as DialogDescriptionNewYork, 
    DialogFooter as DialogFooterNewYork, DialogClose as DialogCloseNewYork
};

#[cfg(test)]
mod real_tests;
mod tests;


// Signal-managed exports
pub use signal_managed::*;