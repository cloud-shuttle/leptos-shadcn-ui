//! ContextMenu default components
//! 
//! This module contains all the default context menu components organized into focused sub-modules
//! for better maintainability and readability.

pub mod context_menu;
pub mod trigger;
pub mod content;
pub mod items;
pub mod checkbox_radio;
pub mod label_separator;
pub mod shortcut;
pub mod submenu;

// Re-export all components for easy access
pub use context_menu::ContextMenu;
pub use trigger::ContextMenuTrigger;
pub use content::ContextMenuContent;
pub use items::ContextMenuItem;
pub use checkbox_radio::{ContextMenuCheckboxItem, ContextMenuRadioGroup, ContextMenuRadioItem};
pub use label_separator::{ContextMenuLabel, ContextMenuSeparator};
pub use shortcut::ContextMenuShortcut;
pub use submenu::{ContextMenuSub, ContextMenuSubTrigger, ContextMenuSubContent};
