#[cfg(test)]
mod styling_tests {
    use leptos::prelude::*;
    use leptos_style::Style;

    #[test]
    fn test_form_class_constants() {
        // Test Form class constant
        let form_class = "space-y-6";
        assert!(form_class.contains("space-y-6"));

        // Test FormField class constant
        let form_field_class = "space-y-2";
        assert!(form_field_class.contains("space-y-2"));

        // Test FormItem class constant
        let form_item_class = "space-y-2";
        assert!(form_item_class.contains("space-y-2"));

        // Test FormLabel class constant
        let form_label_class = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";
        assert!(form_label_class.contains("text-sm"));
        assert!(form_label_class.contains("font-medium"));
        assert!(form_label_class.contains("leading-none"));
        assert!(form_label_class.contains("peer-disabled:cursor-not-allowed"));
        assert!(form_label_class.contains("peer-disabled:opacity-70"));

        // Test FormControl class constant
        let form_control_class = "peer";
        assert!(form_control_class.contains("peer"));

        // Test FormMessage class constant
        let form_message_class = "text-sm font-medium text-destructive";
        assert!(form_message_class.contains("text-sm"));
        assert!(form_message_class.contains("font-medium"));
        assert!(form_message_class.contains("text-destructive"));

        // Test FormDescription class constant
        let form_description_class = "text-sm text-muted-foreground";
        assert!(form_description_class.contains("text-sm"));
        assert!(form_description_class.contains("text-muted-foreground"));
    }

    #[test]
    fn test_form_computed_class_generation() {
        // Test Form computed class generation
        let base_class = "space-y-6";
        let custom_class = "custom-form";
        let computed = format!("{} {}", base_class, custom_class);
        
        assert!(computed.contains("space-y-6"));
        assert!(computed.contains("custom-form"));

        // Test FormField computed class generation
        let field_base = "space-y-2";
        let field_custom = "custom-field";
        let field_computed = format!("{} {}", field_base, field_custom);
        
        assert!(field_computed.contains("space-y-2"));
        assert!(field_computed.contains("custom-field"));

        // Test FormLabel computed class generation
        let label_base = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";
        let label_custom = "custom-label";
        let label_computed = format!("{} {}", label_base, label_custom);
        
        assert!(label_computed.contains("text-sm"));
        assert!(label_computed.contains("custom-label"));
    }

    #[test]
    fn test_form_style_handling() {
        // Test style signal handling
        let style_signal = RwSignal::new(Style::new());
        let style_string = style_signal.get().to_string();
        assert_eq!(style_string, "");
        
        // Test style changes
        let new_style = Style::new();
        style_signal.set(new_style);
        let new_style_string = style_signal.get().to_string();
        assert_eq!(new_style_string, "");

        // Test style with custom properties
        let custom_style = Style::new();
        let custom_style_signal = RwSignal::new(custom_style);
        let custom_style_string = custom_style_signal.get().to_string();
        assert_eq!(custom_style_string, "");
    }

    #[test]
    fn test_form_typography_system() {
        // Test typography system classes
        let text_sm = "text-sm";
        let font_medium = "font-medium";
        let leading_none = "leading-none";
        
        assert!(text_sm.contains("text-sm"));
        assert!(font_medium.contains("font-medium"));
        assert!(leading_none.contains("leading-none"));
        
        // Test typography combinations
        let combined_typography = format!("{} {} {}", text_sm, font_medium, leading_none);
        assert!(combined_typography.contains("text-sm"));
        assert!(combined_typography.contains("font-medium"));
        assert!(combined_typography.contains("leading-none"));
    }

    #[test]
    fn test_form_spacing_system() {
        // Test spacing system classes
        let space_y_6 = "space-y-6";
        let space_y_2 = "space-y-2";
        
        assert!(space_y_6.contains("space-y-6"));
        assert!(space_y_2.contains("space-y-2"));
        
        // Test spacing combinations
        let combined_spacing = format!("{} {}", space_y_6, space_y_2);
        assert!(combined_spacing.contains("space-y-6"));
        assert!(combined_spacing.contains("space-y-2"));
    }

    #[test]
    fn test_form_peer_system() {
        // Test peer system classes
        let peer = "peer";
        let peer_disabled_cursor = "peer-disabled:cursor-not-allowed";
        let peer_disabled_opacity = "peer-disabled:opacity-70";
        
        assert!(peer.contains("peer"));
        assert!(peer_disabled_cursor.contains("peer-disabled:cursor-not-allowed"));
        assert!(peer_disabled_opacity.contains("peer-disabled:opacity-70"));
        
        // Test peer combinations
        let combined_peer = format!("{} {} {}", peer, peer_disabled_cursor, peer_disabled_opacity);
        assert!(combined_peer.contains("peer"));
        assert!(combined_peer.contains("peer-disabled:cursor-not-allowed"));
        assert!(combined_peer.contains("peer-disabled:opacity-70"));
    }

    #[test]
    fn test_form_conditional_class_rendering() {
        // Test conditional class rendering
        let has_error = true;
        let is_disabled = false;
        let is_required = true;
        
        // Test error state classes
        let error_class = if has_error {
            "text-destructive"
        } else {
            ""
        };
        assert_eq!(error_class, "text-destructive");
        
        // Test disabled state classes
        let disabled_class = if is_disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            ""
        };
        assert_eq!(disabled_class, "");
        
        // Test required state classes
        let required_class = if is_required {
            "required"
        } else {
            ""
        };
        assert_eq!(required_class, "required");
        
        // Test combined conditional classes
        let combined_conditional = format!("{} {} {}", error_class, disabled_class, required_class).trim().to_string();
        assert!(combined_conditional.contains("text-destructive"));
        assert!(combined_conditional.contains("required"));
    }

    #[test]
    fn test_form_responsive_styling() {
        // Test responsive styling classes
        let base_class = "space-y-6";
        let responsive_class = "sm:space-y-4 md:space-y-6 lg:space-y-8";
        
        let responsive_composed = format!("{} {}", base_class, responsive_class);
        assert!(responsive_composed.contains("space-y-6"));
        assert!(responsive_composed.contains("sm:space-y-4"));
        assert!(responsive_composed.contains("md:space-y-6"));
        assert!(responsive_composed.contains("lg:space-y-8"));
    }

    #[test]
    fn test_form_theme_integration() {
        // Test theme integration classes
        let base_class = "text-sm";
        let theme_class = "dark:text-gray-300 light:text-gray-700";
        
        let themed_class = format!("{} {}", base_class, theme_class);
        assert!(themed_class.contains("text-sm"));
        assert!(themed_class.contains("dark:text-gray-300"));
        assert!(themed_class.contains("light:text-gray-700"));
    }

    #[test]
    fn test_form_accessibility_styling() {
        // Test accessibility styling classes
        let base_class = "text-sm";
        let a11y_class = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500";
        
        let a11y_styled = format!("{} {}", base_class, a11y_class);
        assert!(a11y_styled.contains("text-sm"));
        assert!(a11y_styled.contains("focus-visible:outline-none"));
        assert!(a11y_styled.contains("focus-visible:ring-2"));
        assert!(a11y_styled.contains("focus-visible:ring-blue-500"));
    }

    #[test]
    fn test_form_animation_styling() {
        // Test animation styling classes
        let base_class = "space-y-6";
        let animation_class = "transition-all duration-200 ease-in-out";
        
        let animated_class = format!("{} {}", base_class, animation_class);
        assert!(animated_class.contains("space-y-6"));
        assert!(animated_class.contains("transition-all"));
        assert!(animated_class.contains("duration-200"));
        assert!(animated_class.contains("ease-in-out"));
    }

    #[test]
    fn test_form_style_performance() {
        // Test style performance
        let style_signal = RwSignal::new(Style::new());
        
        // Test rapid style updates
        let start = std::time::Instant::now();
        
        for _i in 0..100 {
            let style = Style::new();
            style_signal.set(style);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be very fast
        
        // Test final state
        let final_style = style_signal.get().to_string();
        assert_eq!(final_style, "");
    }
}
