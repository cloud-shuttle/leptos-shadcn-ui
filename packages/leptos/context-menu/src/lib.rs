//! Leptos port of shadcn/ui context menu

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
    ContextMenuSeparator, ContextMenuLabel, ContextMenuCheckboxItem,
    ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSub,
    ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuShortcut,
};

pub use new_york::{
    ContextMenu as ContextMenuNewYork,
    ContextMenuContent as ContextMenuContentNewYork,
    ContextMenuItem as ContextMenuItemNewYork,
    ContextMenuTrigger as ContextMenuTriggerNewYork,
    ContextMenuSeparator as ContextMenuSeparatorNewYork,
    ContextMenuLabel as ContextMenuLabelNewYork,
    ContextMenuCheckboxItem as ContextMenuCheckboxItemNewYork,
    ContextMenuRadioGroup as ContextMenuRadioGroupNewYork,
    ContextMenuRadioItem as ContextMenuRadioItemNewYork,
    ContextMenuSub as ContextMenuSubNewYork,
    ContextMenuSubContent as ContextMenuSubContentNewYork,
    ContextMenuSubTrigger as ContextMenuSubTriggerNewYork,
    ContextMenuShortcut as ContextMenuShortcutNewYork,
};

mod tests;

mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
