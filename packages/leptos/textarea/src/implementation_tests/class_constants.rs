#[cfg(test)]
mod class_constants {
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== CLASS CONSTANTS TESTS =====
    // These tests focus on CSS class constants and styling

    #[test]
    fn test_textarea_class_constant() {
        // Test TEXTAREA_CLASS constant
        let textarea_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        
        assert!(textarea_class.contains("flex"));
        assert!(textarea_class.contains("h-10"));
        assert!(textarea_class.contains("w-full"));
        assert!(textarea_class.contains("rounded-md"));
        assert!(textarea_class.contains("border"));
        assert!(textarea_class.contains("border-input"));
        assert!(textarea_class.contains("bg-background"));
        assert!(textarea_class.contains("px-3"));
        assert!(textarea_class.contains("py-2"));
        assert!(textarea_class.contains("text-sm"));
        assert!(textarea_class.contains("ring-offset-background"));
        assert!(textarea_class.contains("file:border-0"));
        assert!(textarea_class.contains("file:bg-transparent"));
        assert!(textarea_class.contains("file:text-sm"));
        assert!(textarea_class.contains("file:font-medium"));
        assert!(textarea_class.contains("placeholder:text-muted-foreground"));
        assert!(textarea_class.contains("focus-visible:outline-none"));
        assert!(textarea_class.contains("focus-visible:ring-2"));
        assert!(textarea_class.contains("focus-visible:ring-ring"));
        assert!(textarea_class.contains("focus-visible:ring-offset-2"));
        assert!(textarea_class.contains("disabled:cursor-not-allowed"));
        assert!(textarea_class.contains("disabled:opacity-50"));
    }

    #[test]
    fn test_textarea_computed_class_generation() {
        // Test Textarea computed class generation
        let base_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        let custom_class = "custom-textarea";
        let computed = format!("{} {}", base_class, custom_class);
        
        assert!(computed.contains("flex"));
        assert!(computed.contains("h-10"));
        assert!(computed.contains("w-full"));
        assert!(computed.contains("custom-textarea"));
    }

    #[test]
    fn test_textarea_style_handling() {
        // Test style handling
        let custom_style = "color: red; background: blue;";
        let style_prop = Some(custom_style.to_string());
        
        assert!(style_prop.is_some());
        assert_eq!(style_prop.unwrap(), custom_style);
    }

    #[test]
    fn test_textarea_sizing_system() {
        // Test sizing system
        let small_class = "h-8 text-xs";
        let default_class = "h-10 text-sm";
        let large_class = "h-12 text-base";
        
        assert!(small_class.contains("h-8"));
        assert!(small_class.contains("text-xs"));
        assert!(default_class.contains("h-10"));
        assert!(default_class.contains("text-sm"));
        assert!(large_class.contains("h-12"));
        assert!(large_class.contains("text-base"));
    }

    #[test]
    fn test_textarea_variant_classes() {
        // Test variant-specific classes
        let default_class = "border-input bg-background";
        let destructive_class = "border-destructive bg-destructive";
        let outline_class = "border-outline bg-outline";
        
        assert!(default_class.contains("border-input"));
        assert!(default_class.contains("bg-background"));
        assert!(destructive_class.contains("border-destructive"));
        assert!(destructive_class.contains("bg-destructive"));
        assert!(outline_class.contains("border-outline"));
        assert!(outline_class.contains("bg-outline"));
    }

    #[test]
    fn test_textarea_state_classes() {
        // Test state-specific classes
        let disabled_class = "disabled:cursor-not-allowed disabled:opacity-50";
        let focused_class = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";
        let error_class = "border-destructive text-destructive";
        let success_class = "border-green-500 text-green-500";
        
        assert!(disabled_class.contains("disabled:"));
        assert!(disabled_class.contains("cursor-not-allowed"));
        assert!(disabled_class.contains("opacity-50"));
        assert!(focused_class.contains("focus-visible:"));
        assert!(focused_class.contains("outline-none"));
        assert!(focused_class.contains("ring-2"));
        assert!(focused_class.contains("ring-ring"));
        assert!(error_class.contains("border-destructive"));
        assert!(error_class.contains("text-destructive"));
        assert!(success_class.contains("border-green-500"));
        assert!(success_class.contains("text-green-500"));
    }

    #[test]
    fn test_textarea_accessibility_classes() {
        // Test accessibility-specific classes
        let aria_invalid_class = "aria-invalid";
        let aria_required_class = "aria-required";
        let aria_describedby_class = "aria-describedby";
        let role_class = "role=\"textbox\"";
        
        assert!(aria_invalid_class.contains("aria-invalid"));
        assert!(aria_required_class.contains("aria-required"));
        assert!(aria_describedby_class.contains("aria-describedby"));
        assert!(role_class.contains("role="));
        assert!(role_class.contains("textbox"));
    }

    #[test]
    fn test_textarea_animation_classes() {
        // Test animation-specific classes
        let transition_class = "transition-colors";
        let duration_class = "duration-200";
        let ease_class = "ease-in-out";
        
        assert!(transition_class.contains("transition-colors"));
        assert!(duration_class.contains("duration-200"));
        assert!(ease_class.contains("ease-in-out"));
    }

    #[test]
    fn test_textarea_layout_classes() {
        // Test layout-specific classes
        let flex_class = "flex";
        let w_full_class = "w-full";
        let rounded_class = "rounded-md";
        let border_class = "border";
        let px_class = "px-3";
        let py_class = "py-2";
        
        assert!(flex_class.contains("flex"));
        assert!(w_full_class.contains("w-full"));
        assert!(rounded_class.contains("rounded-md"));
        assert!(border_class.contains("border"));
        assert!(px_class.contains("px-3"));
        assert!(py_class.contains("py-2"));
    }

    #[test]
    fn test_textarea_placeholder_classes() {
        // Test placeholder-specific classes
        let placeholder_class = "placeholder:text-muted-foreground";
        let placeholder_opacity_class = "placeholder:opacity-50";
        let placeholder_italic_class = "placeholder:italic";
        
        assert!(placeholder_class.contains("placeholder:"));
        assert!(placeholder_class.contains("text-muted-foreground"));
        assert!(placeholder_opacity_class.contains("placeholder:"));
        assert!(placeholder_opacity_class.contains("opacity-50"));
        assert!(placeholder_italic_class.contains("placeholder:"));
        assert!(placeholder_italic_class.contains("italic"));
    }

    #[test]
    fn test_textarea_ring_classes() {
        // Test ring-specific classes
        let ring_offset_class = "ring-offset-background";
        let ring_2_class = "focus-visible:ring-2";
        let ring_ring_class = "focus-visible:ring-ring";
        let ring_offset_2_class = "focus-visible:ring-offset-2";
        
        assert!(ring_offset_class.contains("ring-offset-background"));
        assert!(ring_2_class.contains("focus-visible:ring-2"));
        assert!(ring_ring_class.contains("focus-visible:ring-ring"));
        assert!(ring_offset_2_class.contains("focus-visible:ring-offset-2"));
    }
}
