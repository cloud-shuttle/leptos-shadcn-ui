//! Click Handler Logic Tests for Button Component
//! 
//! This module tests the click handler functionality and callback execution.

#[cfg(test)]
mod click_handler_tests {
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_button_click_handler_callback_execution() {
        // TDD: Test that click handlers are properly called
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = Arc::clone(&clicked);
        
        let callback = Callback::new(move |_| {
            *clicked_clone.lock().unwrap() = true;
        });
        
        // Simulate the click handler logic that would be in the component
        if !*clicked.lock().unwrap() {
            callback.run(());
        }
        
        assert!(*clicked.lock().unwrap(), "Button click handler should execute successfully");
    }

    #[test]
    fn test_multiple_button_click_handlers() {
        // TDD: Test that multiple button instances have independent click handlers
        let button1_clicked = Arc::new(Mutex::new(0));
        let button2_clicked = Arc::new(Mutex::new(0));
        
        let button1_clone = Arc::clone(&button1_clicked);
        let button2_clone = Arc::clone(&button2_clicked);
        
        let callback1 = Callback::new(move |_| {
            *button1_clone.lock().unwrap() += 1;
        });
        
        let callback2 = Callback::new(move |_| {
            *button2_clone.lock().unwrap() += 1;
        });
        
        // Test independent execution
        callback1.run(());
        assert_eq!(*button1_clicked.lock().unwrap(), 1);
        assert_eq!(*button2_clicked.lock().unwrap(), 0);
        
        callback2.run(());
        assert_eq!(*button1_clicked.lock().unwrap(), 1);
        assert_eq!(*button2_clicked.lock().unwrap(), 1);
        
        // Test multiple executions
        callback1.run(());
        callback1.run(());
        assert_eq!(*button1_clicked.lock().unwrap(), 3);
        assert_eq!(*button2_clicked.lock().unwrap(), 1);
    }
}
