//! Drawer default components
//! 
//! This module contains all the default drawer components organized into focused sub-modules
//! for better maintainability and readability.

pub mod types;
pub mod drawer;
pub mod trigger;
pub mod portal_overlay;
pub mod content;
pub mod header_footer;
pub mod title_description;
pub mod close;
pub mod nested;

// Re-export all components and types for easy access
pub use types::*;
pub use drawer::Drawer;
pub use trigger::{DrawerTrigger, DrawerTriggerChildProps};
pub use portal_overlay::{DrawerPortal, DrawerOverlay};
pub use content::DrawerContent;
pub use header_footer::{DrawerHeader, DrawerFooter};
pub use title_description::{DrawerTitle, DrawerDescription};
pub use close::{DrawerClose, DrawerCloseChildProps};
pub use nested::DrawerNestedRoot;
