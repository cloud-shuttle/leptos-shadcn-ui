//! Leptos port of shadcn/ui accordion

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Accordion, AccordionItem, AccordionTrigger, AccordionContent,
    AccordionType, AccordionOrientation,
};

pub use new_york::{
    Accordion as AccordionNewYork,
    AccordionItem as AccordionItemNewYork,
    AccordionTrigger as AccordionTriggerNewYork,
    AccordionContent as AccordionContentNewYork,
    AccordionType as AccordionTypeNewYork,
    AccordionOrientation as AccordionOrientationNewYork,
};

mod tests;

mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;