//! Leptos port of shadcn/ui resizable

pub mod default;
pub mod new_york;
pub mod resizable;

pub use default::{ResizablePanelGroup, ResizablePanel, ResizableHandle};
pub use new_york::{ResizablePanelGroup as ResizablePanelGroupNewYork, ResizablePanel as ResizablePanelNewYork, ResizableHandle as ResizableHandleNewYork};
pub use resizable::{
    ResizeDirection, ResizableState, ResizableConfig
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod resizable_tests;
