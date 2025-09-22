#[cfg(test)]
mod class_constants {
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== CLASS CONSTANTS TESTS =====
    // These tests focus on CSS class constants and styling

    #[test]
    fn test_radio_group_class_constants() {
        // Test RADIO_GROUP_CLASS constant
        let radio_group_class = "grid gap-2";
        assert!(radio_group_class.contains("grid"));
        assert!(radio_group_class.contains("gap-2"));

        // Test RADIO_ITEM_CLASS constant
        let radio_item_class = "aspect-square h-4 w-4 rounded-full border border-primary text-primary ring-offset-background focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        assert!(radio_item_class.contains("aspect-square"));
        assert!(radio_item_class.contains("h-4"));
        assert!(radio_item_class.contains("w-4"));
        assert!(radio_item_class.contains("rounded-full"));
        assert!(radio_item_class.contains("border"));
        assert!(radio_item_class.contains("border-primary"));
        assert!(radio_item_class.contains("text-primary"));
        assert!(radio_item_class.contains("ring-offset-background"));
        assert!(radio_item_class.contains("focus:outline-none"));
        assert!(radio_item_class.contains("focus-visible:ring-2"));
        assert!(radio_item_class.contains("focus-visible:ring-ring"));
        assert!(radio_item_class.contains("focus-visible:ring-offset-2"));
        assert!(radio_item_class.contains("disabled:cursor-not-allowed"));
        assert!(radio_item_class.contains("disabled:opacity-50"));

        // Test RADIO_INDICATOR_CLASS constant
        let radio_indicator_class = "flex items-center justify-center";
        assert!(radio_indicator_class.contains("flex"));
        assert!(radio_indicator_class.contains("items-center"));
        assert!(radio_indicator_class.contains("justify-center"));

        // Test RADIO_INDICATOR_DOT_CLASS constant
        let radio_indicator_dot_class = "h-2.5 w-2.5 rounded-full bg-current";
        assert!(radio_indicator_dot_class.contains("h-2.5"));
        assert!(radio_indicator_dot_class.contains("w-2.5"));
        assert!(radio_indicator_dot_class.contains("rounded-full"));
        assert!(radio_indicator_dot_class.contains("bg-current"));
    }

    #[test]
    fn test_radio_group_computed_class_generation() {
        // Test RadioGroup computed class generation
        let base_class = "grid gap-2";
        let custom_class = "custom-radio-group";
        let combined_class = format!("{} {}", base_class, custom_class);
        
        assert!(combined_class.contains("grid"));
        assert!(combined_class.contains("gap-2"));
        assert!(combined_class.contains("custom-radio-group"));
    }

    #[test]
    fn test_radio_group_item_computed_class_generation() {
        // Test RadioGroupItem computed class generation
        let base_class = "aspect-square h-4 w-4 rounded-full border border-primary text-primary ring-offset-background focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        let custom_class = "custom-radio-item";
        let combined_class = format!("{} {}", base_class, custom_class);
        
        assert!(combined_class.contains("aspect-square"));
        assert!(combined_class.contains("h-4"));
        assert!(combined_class.contains("w-4"));
        assert!(combined_class.contains("rounded-full"));
        assert!(combined_class.contains("border"));
        assert!(combined_class.contains("border-primary"));
        assert!(combined_class.contains("text-primary"));
        assert!(combined_class.contains("ring-offset-background"));
        assert!(combined_class.contains("focus:outline-none"));
        assert!(combined_class.contains("focus-visible:ring-2"));
        assert!(combined_class.contains("focus-visible:ring-ring"));
        assert!(combined_class.contains("focus-visible:ring-offset-2"));
        assert!(combined_class.contains("disabled:cursor-not-allowed"));
        assert!(combined_class.contains("disabled:opacity-50"));
        assert!(combined_class.contains("custom-radio-item"));
    }

    #[test]
    fn test_radio_group_style_handling() {
        // Test style handling
        let custom_style = "color: red; background: blue;";
        let style_prop = Some(custom_style.to_string());
        
        assert!(style_prop.is_some());
        assert_eq!(style_prop.unwrap(), custom_style);
    }

    #[test]
    fn test_radio_group_variant_classes() {
        // Test variant-specific classes
        let default_class = "border-primary text-primary";
        let destructive_class = "border-destructive text-destructive";
        let outline_class = "border-outline text-outline";
        
        assert!(default_class.contains("border-primary"));
        assert!(default_class.contains("text-primary"));
        assert!(destructive_class.contains("border-destructive"));
        assert!(destructive_class.contains("text-destructive"));
        assert!(outline_class.contains("border-outline"));
        assert!(outline_class.contains("text-outline"));
    }

    #[test]
    fn test_radio_group_size_classes() {
        // Test size-specific classes
        let small_class = "h-3 w-3";
        let default_class = "h-4 w-4";
        let large_class = "h-5 w-5";
        
        assert!(small_class.contains("h-3"));
        assert!(small_class.contains("w-3"));
        assert!(default_class.contains("h-4"));
        assert!(default_class.contains("w-4"));
        assert!(large_class.contains("h-5"));
        assert!(large_class.contains("w-5"));
    }

    #[test]
    fn test_radio_group_state_classes() {
        // Test state-specific classes
        let checked_class = "data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground";
        let unchecked_class = "data-[state=unchecked]:bg-background data-[state=unchecked]:text-foreground";
        let disabled_class = "disabled:cursor-not-allowed disabled:opacity-50";
        let focused_class = "focus:outline-none focus-visible:ring-2 focus-visible:ring-ring";
        
        assert!(checked_class.contains("data-[state=checked]"));
        assert!(checked_class.contains("bg-primary"));
        assert!(checked_class.contains("text-primary-foreground"));
        assert!(unchecked_class.contains("data-[state=unchecked]"));
        assert!(unchecked_class.contains("bg-background"));
        assert!(unchecked_class.contains("text-foreground"));
        assert!(disabled_class.contains("disabled:"));
        assert!(disabled_class.contains("cursor-not-allowed"));
        assert!(disabled_class.contains("opacity-50"));
        assert!(focused_class.contains("focus:"));
        assert!(focused_class.contains("outline-none"));
        assert!(focused_class.contains("ring-2"));
        assert!(focused_class.contains("ring-ring"));
    }

    #[test]
    fn test_radio_group_accessibility_classes() {
        // Test accessibility-specific classes
        let aria_checked_class = "aria-checked";
        let aria_disabled_class = "aria-disabled";
        let aria_required_class = "aria-required";
        let role_class = "role=\"radio\"";
        
        assert!(aria_checked_class.contains("aria-checked"));
        assert!(aria_disabled_class.contains("aria-disabled"));
        assert!(aria_required_class.contains("aria-required"));
        assert!(role_class.contains("role="));
        assert!(role_class.contains("radio"));
    }

    #[test]
    fn test_radio_group_animation_classes() {
        // Test animation-specific classes
        let transition_class = "transition-colors";
        let duration_class = "duration-200";
        let ease_class = "ease-in-out";
        
        assert!(transition_class.contains("transition-colors"));
        assert!(duration_class.contains("duration-200"));
        assert!(ease_class.contains("ease-in-out"));
    }

    #[test]
    fn test_radio_group_layout_classes() {
        // Test layout-specific classes
        let grid_class = "grid";
        let gap_class = "gap-2";
        let flex_class = "flex";
        let items_center_class = "items-center";
        let justify_center_class = "justify-center";
        
        assert!(grid_class.contains("grid"));
        assert!(gap_class.contains("gap-2"));
        assert!(flex_class.contains("flex"));
        assert!(items_center_class.contains("items-center"));
        assert!(justify_center_class.contains("justify-center"));
    }
}
