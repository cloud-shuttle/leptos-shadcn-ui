//! Leptos port of shadcn/ui card

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
pub use new_york::{Card as CardNewYork, CardHeader as CardHeaderNewYork, CardTitle as CardTitleNewYork, CardDescription as CardDescriptionNewYork, CardContent as CardContentNewYork, CardFooter as CardFooterNewYork};
pub use signal_managed::{
    SignalManagedCard, EnhancedCard, SignalManagedCardState,
    SignalManagedCardHeader, SignalManagedCardTitle, SignalManagedCardDescription,
    SignalManagedCardContent, SignalManagedCardFooter
};

mod tests;

mod tdd_tests;

mod implementation_tests;

mod new_york_tests;
