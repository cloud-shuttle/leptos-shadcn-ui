//! Leptos port of shadcn/ui skeleton

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{
    Skeleton, SkeletonText, SkeletonAvatar, SkeletonCard, SkeletonVariant, SkeletonSize
};
pub use new_york::{
    Skeleton as SkeletonNewYork, SkeletonText as SkeletonTextNewYork, 
    SkeletonAvatar as SkeletonAvatarNewYork, SkeletonCard as SkeletonCardNewYork,
    SkeletonVariant as SkeletonVariantNewYork, SkeletonSize as SkeletonSizeNewYork
};

mod tests;

mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
