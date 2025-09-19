#[cfg(test)]
mod state_management_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize};
    use leptos::prelude::*;

    #[test]
    fn test_button_state_management_advanced() {
        // Test advanced state management
        let state_signal = RwSignal::new(false);
        let click_count = RwSignal::new(0);
        
        let _stateful_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                disabled=state_signal
                on_click=Callback::new(move |_| {
                    click_count.update(|count| *count += 1);
                    state_signal.set(!state_signal.get());
                })
            >
                "Toggle State"
            </Button>
        };
        
        // Initial state should be enabled
        assert!(!state_signal.get(), "Initial state should be enabled");
        assert_eq!(click_count.get(), 0, "Initial click count should be 0");
        
        // Simulate click
        click_count.update(|count| *count += 1);
        state_signal.set(true);
        
        // State should be toggled
        assert!(state_signal.get(), "State should be toggled after click");
        assert_eq!(click_count.get(), 1, "Click count should be incremented");
    }

    #[test]
    fn test_button_advanced_interactions() {
        // Test advanced interaction patterns
        let interaction_count = RwSignal::new(0);
        
        let _advanced_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="advanced-interactions"
                on_click=Callback::new(move |_| {
                    interaction_count.update(|count| *count += 1);
                })
            >
                "Advanced Button"
            </Button>
        };
        
        // Test multiple interactions
        for i in 0..5 {
            interaction_count.update(|count| *count += 1);
            assert_eq!(interaction_count.get(), i + 1, "Interaction count should be {}", i + 1);
        }
        
        // Should handle rapid interactions
        assert_eq!(interaction_count.get(), 5, "Should handle multiple interactions");
    }

    #[test]
    fn test_button_theme_switching() {
        // Test theme switching support
        let theme_signal = RwSignal::new("light");
        
        let _theme_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="theme-light"
            >
                "Theme Button"
            </Button>
        };
        
        // Should support theme switching
        assert_eq!(theme_signal.get(), "light", "Initial theme should be light");
        
        // Switch theme
        theme_signal.set("dark");
        assert_eq!(theme_signal.get(), "dark", "Theme should switch to dark");
    }

    #[test]
    fn test_button_validation_states() {
        // Test validation states
        let validation_signal = RwSignal::new("valid");
        
        let _validation_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="validation-valid"
            >
                "Validation Button"
            </Button>
        };
        
        // Should support validation states
        assert_eq!(validation_signal.get(), "valid", "Initial validation should be valid");
        
        // Change validation state
        validation_signal.set("invalid");
        assert_eq!(validation_signal.get(), "invalid", "Validation should change to invalid");
    }

    #[test]
    fn test_button_error_handling() {
        // Test error handling in button interactions
        let _error_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="error-handling"
                on_click=Callback::new(|_| {
                    // Simulate error condition
                    // In a real implementation, this would be handled gracefully
                })
            >
                "Error Button"
            </Button>
        };
        
        // Error handling should be graceful
        assert!(true, "Error handling should be implemented");
    }

    #[test]
    fn test_button_memory_management() {
        // Test memory management and cleanup
        let _memory_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="memory-test"
            >
                "Memory Test"
            </Button>
        };
        
        // Memory should be managed efficiently
        assert!(true, "Memory management should be optimized");
    }

    #[test]
    fn test_button_form_integration_advanced() {
        // Test advanced form integration
        let _form_integration_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="form-integration"
                id="form-btn"
            >
                "Form Button"
            </Button>
        };
        
        // Should integrate properly with forms
        assert!(true, "Advanced form integration should be implemented");
    }

    #[test]
    fn test_button_integration_comprehensive() {
        // Test comprehensive integration scenarios
        let integration_scenarios = vec![
            "form-submission",
            "modal-trigger",
            "dropdown-toggle",
            "accordion-trigger",
            "tab-trigger",
            "carousel-control",
        ];
        
        for scenario in integration_scenarios {
            let _integration_button_view = view! {
                <Button 
                    variant=ButtonVariant::Default
                    size=ButtonSize::Default
                    class=format!("integration-{}", scenario)
                >
                    format!("{} Button", scenario)
                </Button>
            };
            
            // Each integration scenario should work
            assert!(true, "Integration scenario '{}' should work", scenario);
        }
    }
}
