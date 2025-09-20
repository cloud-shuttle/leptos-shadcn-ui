//! Leptos port of shadcn/ui slider

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Slider, RangeSlider, SliderRoot, SliderVariant, SliderSize
};
pub use new_york::{
    Slider as SliderNewYork, RangeSlider as RangeSliderNewYork, 
    SliderRoot as SliderRootNewYork, SliderVariant as SliderVariantNewYork,
    SliderSize as SliderSizeNewYork
};

mod tests;

mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
