//! Leptos port of shadcn/ui tabs

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Tabs, TabsList, TabsTrigger, TabsContent
};
pub use new_york::{
    Tabs as TabsNewYork, TabsList as TabsListNewYork, TabsTrigger as TabsTriggerNewYork, TabsContent as TabsContentNewYork
};


mod tests;


mod tdd_tests;


// Signal-managed exports
pub use signal_managed::*;

#[cfg(test)]
mod real_tests;