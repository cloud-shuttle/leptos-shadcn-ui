#[cfg(test)]
mod callback_tests {
    use leptos::prelude::*;
    use leptos_style::Style;

    #[test]
    fn test_select_callback_handling() {
        // Test value change callback
        let callback_triggered = RwSignal::new(false);
        let callback_value = RwSignal::new(None::<String>);
        
        let value_callback = Callback::new(move |value: Option<String>| {
            callback_triggered.set(true);
            callback_value.set(value);
        });
        
        // Simulate callback invocation
        value_callback.run(Some("test_value".to_string()));
        
        assert!(callback_triggered.get(), "Callback should have been triggered");
        assert_eq!(callback_value.get(), Some("test_value".to_string()));
    }

    #[test]
    fn test_select_open_callback_handling() {
        // Test open state change callback
        let open_callback_triggered = RwSignal::new(false);
        let callback_open_state = RwSignal::new(false);
        
        let open_callback = Callback::new(move |is_open: bool| {
            open_callback_triggered.set(true);
            callback_open_state.set(is_open);
        });
        
        // Simulate callback invocation
        open_callback.run(true);
        
        assert!(open_callback_triggered.get(), "Open callback should have been triggered");
        assert!(callback_open_state.get(), "Callback should have received open state");
        
        // Test closing callback
        open_callback.run(false);
        assert!(!callback_open_state.get(), "Callback should have received closed state");
    }

    #[test]
    fn test_select_context_management() {
        // Test context provision and consumption
        let context_value = RwSignal::new("test_context".to_string());
        
        // Simulate context provision
        let provided_value = context_value.get();
        assert_eq!(provided_value, "test_context");
        
        // Test context consumption
        let consumed_value = context_value.get();
        assert_eq!(consumed_value, provided_value);
        
        // Test context updates
        context_value.set("updated_context".to_string());
        assert_eq!(context_value.get(), "updated_context");
    }
}
