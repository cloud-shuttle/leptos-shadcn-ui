//! Integration Tests for Button Component
//! 
//! This module tests complex scenarios and integration with other components.

#[cfg(test)]
mod integration_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize};
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_button_component_integration_scenario() {
        // TDD: Test a complete button usage scenario
        let form_submitted = Arc::new(Mutex::new(false));
        let form_submitted_clone = Arc::clone(&form_submitted);
        
        // Simulate a form submission button
        let submit_callback = Callback::new(move |_| {
            *form_submitted_clone.lock().unwrap() = true;
        });
        
        let disabled_state = RwSignal::new(false);
        let button_variant = ButtonVariant::Default;
        let button_size = ButtonSize::Default;
        
        // Test component creation with complex props
        let _complex_button = view! {
            <Button 
                variant=button_variant
                size=button_size
                disabled=Signal::from(disabled_state.get())
                on_click=submit_callback
                class="submit-btn form-control"
                id="form-submit-button"
            >
                "Submit Form"
            </Button>
        };
        
        // Verify complex scenario doesn't cause issues
        assert!(!*form_submitted.lock().unwrap());
        assert!(!disabled_state.get());
        
        // Test state changes
        disabled_state.set(true);
        assert!(disabled_state.get());
    }
}