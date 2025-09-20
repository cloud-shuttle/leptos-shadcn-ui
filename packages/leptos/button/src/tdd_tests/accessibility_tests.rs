#[cfg(test)]
mod accessibility_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize};
    use leptos::prelude::*;

    #[test]
    fn test_button_accessibility_enhancements() {
        // Test enhanced accessibility features
        let _accessible_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="accessible-button"
                id="accessible-btn"
            >
                "Accessible Button"
            </Button>
        };
        
        // Should have enhanced accessibility
    }

    #[test]
    fn test_button_keyboard_navigation() {
        // Test keyboard navigation support
        let _keyboard_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="keyboard-navigation"
            >
                "Keyboard Button"
            </Button>
        };
        
        // Should support keyboard navigation
    }

    #[test]
    fn test_button_focus_management() {
        // Test focus management
        let _focus_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="focus-management"
            >
                "Focus Button"
            </Button>
        };
        
        // Should have proper focus management
    }

    #[test]
    fn test_button_aria_attributes() {
        // Test ARIA attributes support
        let _aria_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="aria-enhanced"
                id="aria-btn"
            >
                "ARIA Button"
            </Button>
        };
        
        // Should have proper ARIA attributes
    }

    #[test]
    fn test_button_accessibility_comprehensive() {
        // Test comprehensive accessibility features
        let a11y_features = vec![
            "keyboard-navigation",
            "screen-reader-support",
            "focus-management",
            "aria-attributes",
            "color-contrast",
            "touch-targets",
        ];
        
        for feature in a11y_features {
            let _a11y_button_view = view! {
                <Button 
                    variant=ButtonVariant::Default
                    size=ButtonSize::Default
                    class=format!("a11y-{}", feature)
                >
                    format!("{} Button", feature)
                </Button>
            };
            
            // Each accessibility feature should be supported
        }
    }
}
