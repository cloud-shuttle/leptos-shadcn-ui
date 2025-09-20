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

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tdd_tests;

#[cfg(test)]

// Signal-managed exports
pub use signal_managed::*;