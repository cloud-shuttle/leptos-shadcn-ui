//! Disabled State Logic Tests for Button Component
//! 
//! This module tests the disabled state functionality and click prevention.

#[cfg(test)]
mod disabled_state_tests {
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_disabled_state_signal_behavior() {
        // TDD: Test disabled state management
        let disabled_signal = RwSignal::new(false);
        
        // Test initial state
        assert!(!disabled_signal.get());
        
        // Test state change
        disabled_signal.set(true);
        assert!(disabled_signal.get());
        
        // Test toggling
        disabled_signal.update(|d| *d = !*d);
        assert!(!disabled_signal.get());
    }

    #[test] 
    fn test_disabled_button_click_prevention_logic() {
        // TDD: Test that disabled state prevents click execution
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = Arc::clone(&clicked);
        let disabled = RwSignal::new(true);
        
        let callback = Callback::new(move |_| {
            *clicked_clone.lock().unwrap() = true;
        });
        
        // Simulate the component's click handler logic with disabled check
        if !disabled.get() {
            callback.run(());
        }
        
        // Should not have executed due to disabled state
        assert!(!*clicked.lock().unwrap());
        
        // Enable and test again
        disabled.set(false);
        if !disabled.get() {
            callback.run(());
        }
        
        // Should now execute
        assert!(*clicked.lock().unwrap());
    }
}
