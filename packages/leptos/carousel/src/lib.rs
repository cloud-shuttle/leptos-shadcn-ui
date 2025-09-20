//! Leptos port of shadcn/ui carousel

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext,
    CarouselOrientation, CarouselApi,
};

pub use new_york::{
    Carousel as CarouselNewYork,
    CarouselContent as CarouselContentNewYork,
    CarouselItem as CarouselItemNewYork,
    CarouselPrevious as CarouselPreviousNewYork,
    CarouselNext as CarouselNextNewYork,
    CarouselOrientation as CarouselOrientationNewYork,
    CarouselApi as CarouselApiNewYork,
};

mod tests;

mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
