//! Leptos port of shadcn/ui Form component
//! 
//! Provides form building blocks with validation and accessibility features.

pub mod signal_managed;
pub mod default;
pub mod new_york;

// Re-export common types
pub use default::{Form, FormField, FormItem, FormLabel, FormControl, FormMessage, FormDescription};

mod tests;

mod implementation_tests;

// Signal-managed exports
pub use signal_managed::*;
