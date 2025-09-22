//! AlertDialog default components
//! 
//! This module contains all the default alert dialog components organized into focused sub-modules
//! for better maintainability and readability.

pub mod alert_dialog;
pub mod trigger;
pub mod overlay;
pub mod content;
pub mod header_footer;
pub mod title_description;
pub mod action_cancel;

// Re-export all components for easy access
pub use alert_dialog::AlertDialog;
pub use trigger::AlertDialogTrigger;
pub use overlay::AlertDialogOverlay;
pub use content::AlertDialogContent;
pub use header_footer::{AlertDialogHeader, AlertDialogFooter};
pub use title_description::{AlertDialogTitle, AlertDialogDescription};
pub use action_cancel::{AlertDialogAction, AlertDialogCancel};
