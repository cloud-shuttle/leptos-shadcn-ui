//! Leptos port of shadcn/ui switch

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Switch, SwitchRoot, SwitchThumb, SwitchLabel, SwitchVariant, SwitchSize
};
pub use new_york::{
    Switch as SwitchNewYork, SwitchRoot as SwitchRootNewYork, 
    SwitchThumb as SwitchThumbNewYork, SwitchLabel as SwitchLabelNewYork,
    SwitchVariant as SwitchVariantNewYork, SwitchSize as SwitchSizeNewYork
};

mod tests;

mod tdd_tests;

mod implementation_tests;

// Signal-managed exports
pub use signal_managed::*;
