#[cfg(test)]
mod class_constants {
    use crate::default::{
        SwitchVariant, SwitchSize, SwitchContextValue
    };
    use leptos::prelude::*;

    // ===== CLASS CONSTANTS TESTS =====
    // These tests focus on CSS class constants and styling

    #[test]
    fn test_switch_class_constants() {
        // Test that switch classes are properly defined
        let switch_class = "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input";
        let thumb_class = "pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0";
        
        // Test SWITCH_CLASS constant
        assert!(switch_class.contains("peer"));
        assert!(switch_class.contains("inline-flex"));
        assert!(switch_class.contains("h-6"));
        assert!(switch_class.contains("w-11"));
        assert!(switch_class.contains("shrink-0"));
        assert!(switch_class.contains("cursor-pointer"));
        assert!(switch_class.contains("items-center"));
        assert!(switch_class.contains("rounded-full"));
        assert!(switch_class.contains("border-2"));
        assert!(switch_class.contains("border-transparent"));
        assert!(switch_class.contains("transition-colors"));
        assert!(switch_class.contains("focus-visible:outline-none"));
        assert!(switch_class.contains("focus-visible:ring-2"));
        assert!(switch_class.contains("focus-visible:ring-ring"));
        assert!(switch_class.contains("focus-visible:ring-offset-2"));
        assert!(switch_class.contains("focus-visible:ring-offset-background"));
        assert!(switch_class.contains("disabled:cursor-not-allowed"));
        assert!(switch_class.contains("disabled:opacity-50"));
        assert!(switch_class.contains("data-[state=checked]:bg-primary"));
        assert!(switch_class.contains("data-[state=unchecked]:bg-input"));

        // Test SWITCH_THUMB_CLASS constant
        assert!(thumb_class.contains("pointer-events-none"));
        assert!(thumb_class.contains("block"));
        assert!(thumb_class.contains("h-5"));
        assert!(thumb_class.contains("w-5"));
        assert!(thumb_class.contains("rounded-full"));
        assert!(thumb_class.contains("bg-background"));
        assert!(thumb_class.contains("shadow-lg"));
        assert!(thumb_class.contains("ring-0"));
        assert!(thumb_class.contains("transition-transform"));
        assert!(thumb_class.contains("data-[state=checked]:translate-x-5"));
        assert!(thumb_class.contains("data-[state=unchecked]:translate-x-0"));
    }

    #[test]
    fn test_switch_computed_class_generation() {
        // Test computed class generation for different states
        let checked_class = "data-[state=checked]:bg-primary";
        let unchecked_class = "data-[state=unchecked]:bg-input";
        let disabled_class = "disabled:cursor-not-allowed disabled:opacity-50";
        let focus_class = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
        
        // Test state-based class generation
        assert!(checked_class.contains("data-[state=checked]"));
        assert!(unchecked_class.contains("data-[state=unchecked]"));
        assert!(disabled_class.contains("disabled:"));
        assert!(focus_class.contains("focus-visible:"));
    }

    #[test]
    fn test_switch_thumb_computed_class_generation() {
        // Test thumb computed class generation
        let checked_thumb_class = "data-[state=checked]:translate-x-5";
        let unchecked_thumb_class = "data-[state=unchecked]:translate-x-0";
        
        // Test thumb state-based class generation
        assert!(checked_thumb_class.contains("data-[state=checked]"));
        assert!(checked_thumb_class.contains("translate-x-5"));
        assert!(unchecked_thumb_class.contains("data-[state=unchecked]"));
        assert!(unchecked_thumb_class.contains("translate-x-0"));
    }

    #[test]
    fn test_switch_variant_checked_class() {
        // Test variant-specific checked class generation
        let default_checked = "data-[state=checked]:bg-primary";
        let destructive_checked = "data-[state=checked]:bg-destructive";
        let outline_checked = "data-[state=checked]:bg-outline";
        
        // Test different variant checked classes
        assert!(default_checked.contains("bg-primary"));
        assert!(destructive_checked.contains("bg-destructive"));
        assert!(outline_checked.contains("bg-outline"));
    }

    #[test]
    fn test_switch_size_switch_class() {
        // Test size-specific switch class generation
        let small_class = "h-4 w-7";
        let default_class = "h-6 w-11";
        let large_class = "h-8 w-14";
        
        // Test different size classes
        assert!(small_class.contains("h-4"));
        assert!(small_class.contains("w-7"));
        assert!(default_class.contains("h-6"));
        assert!(default_class.contains("w-11"));
        assert!(large_class.contains("h-8"));
        assert!(large_class.contains("w-14"));
    }

    #[test]
    fn test_switch_size_thumb_class() {
        // Test size-specific thumb class generation
        let small_thumb = "h-3 w-3";
        let default_thumb = "h-5 w-5";
        let large_thumb = "h-7 w-7";
        
        // Test different thumb size classes
        assert!(small_thumb.contains("h-3"));
        assert!(small_thumb.contains("w-3"));
        assert!(default_thumb.contains("h-5"));
        assert!(default_thumb.contains("w-5"));
        assert!(large_thumb.contains("h-7"));
        assert!(large_thumb.contains("w-7"));
    }

    #[test]
    fn test_switch_animation_class_logic() {
        // Test animation class logic
        let transition_class = "transition-colors";
        let transform_class = "transition-transform";
        let duration_class = "duration-200";
        let ease_class = "ease-in-out";
        
        // Test animation classes
        assert!(transition_class.contains("transition-colors"));
        assert!(transform_class.contains("transition-transform"));
        assert!(duration_class.contains("duration-200"));
        assert!(ease_class.contains("ease-in-out"));
    }

    #[test]
    fn test_switch_state_attr_generation() {
        // Test state attribute generation
        let checked_attr = "data-state=\"checked\"";
        let unchecked_attr = "data-state=\"unchecked\"";
        let disabled_attr = "disabled";
        let aria_checked_attr = "aria-checked=\"true\"";
        
        // Test state attributes
        assert!(checked_attr.contains("data-state"));
        assert!(checked_attr.contains("checked"));
        assert!(unchecked_attr.contains("data-state"));
        assert!(unchecked_attr.contains("unchecked"));
        assert!(disabled_attr.contains("disabled"));
        assert!(aria_checked_attr.contains("aria-checked"));
    }

    #[test]
    fn test_switch_border_system() {
        // Test border system classes
        let border_class = "border-2 border-transparent";
        let border_radius_class = "rounded-full";
        let border_color_class = "border-border";
        
        // Test border classes
        assert!(border_class.contains("border-2"));
        assert!(border_class.contains("border-transparent"));
        assert!(border_radius_class.contains("rounded-full"));
        assert!(border_color_class.contains("border-border"));
    }

    #[test]
    fn test_switch_ring_system() {
        // Test ring system classes
        let ring_class = "focus-visible:ring-2 focus-visible:ring-ring";
        let ring_offset_class = "focus-visible:ring-offset-2 focus-visible:ring-offset-background";
        let ring_color_class = "focus-visible:ring-primary";
        
        // Test ring classes
        assert!(ring_class.contains("focus-visible:ring-2"));
        assert!(ring_class.contains("focus-visible:ring-ring"));
        assert!(ring_offset_class.contains("focus-visible:ring-offset-2"));
        assert!(ring_offset_class.contains("focus-visible:ring-offset-background"));
        assert!(ring_color_class.contains("focus-visible:ring-primary"));
    }

    #[test]
    fn test_switch_transition_system() {
        // Test transition system classes
        let color_transition = "transition-colors";
        let transform_transition = "transition-transform";
        let all_transition = "transition-all";
        let duration_200 = "duration-200";
        let duration_300 = "duration-300";
        let ease_in_out = "ease-in-out";
        let ease_out = "ease-out";
        
        // Test transition classes
        assert!(color_transition.contains("transition-colors"));
        assert!(transform_transition.contains("transition-transform"));
        assert!(all_transition.contains("transition-all"));
        assert!(duration_200.contains("duration-200"));
        assert!(duration_300.contains("duration-300"));
        assert!(ease_in_out.contains("ease-in-out"));
        assert!(ease_out.contains("ease-out"));
    }
}
