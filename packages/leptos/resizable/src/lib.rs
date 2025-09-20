//! Leptos port of shadcn/ui resizable

pub mod signal_managed;
pub mod default;
pub mod new_york;
pub mod resizable;

pub use default::{ResizablePanelGroup, ResizablePanel, ResizableHandle};
pub use new_york::{ResizablePanelGroup as ResizablePanelGroupNewYork, ResizablePanel as ResizablePanelNewYork, ResizableHandle as ResizableHandleNewYork};
pub use resizable::{
    ResizeDirection, ResizableState, ResizableConfig
};


mod tests;


mod resizable_tests;


// Signal-managed exports
pub use signal_managed::*;

#[cfg(test)]
mod real_tests;