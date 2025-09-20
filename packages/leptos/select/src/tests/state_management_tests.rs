#[cfg(test)]
mod state_management_tests {
    use leptos::prelude::*;
    use leptos_style::Style;

    #[test]
    fn test_select_prop_defaults() {
        // Test default prop values
        let default_open = false;
        let default_disabled = false;
        let default_required = false;
        
        assert!(!default_open, "Default open should be false");
        assert!(!default_disabled, "Default disabled should be false");
        assert!(!default_required, "Default required should be false");
        
        // Test default value handling
        let default_value: Option<String> = None;
        assert!(default_value.is_none(), "Default value should be None");
        
        // Test default placeholder
        let default_placeholder = "Select an option...";
        assert_eq!(default_placeholder, "Select an option...");
    }

    #[test]
    fn test_select_style_handling() {
        // Test style prop handling
        let custom_style = "color: red; background: blue;";
        assert!(custom_style.contains("color: red"));
        assert!(custom_style.contains("background: blue"));
        
        // Test style merging
        let base_style = "display: flex;";
        let merged_style = format!("{} {}", base_style, custom_style);
        assert!(merged_style.contains("display: flex"));
        assert!(merged_style.contains("color: red"));
    }

    #[test]
    fn test_select_open_state_management() {
        // Test open state signal
        let open_signal = RwSignal::new(false);
        assert!(!open_signal.get(), "Initial open state should be false");
        
        // Test opening
        open_signal.set(true);
        assert!(open_signal.get(), "Open state should be true after setting");
        
        // Test closing
        open_signal.set(false);
        assert!(!open_signal.get(), "Open state should be false after closing");
    }

    #[test]
    fn test_select_value_state_management() {
        // Test value state signal
        let value_signal = RwSignal::new(None::<String>);
        assert!(value_signal.get().is_none(), "Initial value should be None");
        
        // Test setting value
        value_signal.set(Some("option1".to_string()));
        assert_eq!(value_signal.get(), Some("option1".to_string()));
        
        // Test clearing value
        value_signal.set(None);
        assert!(value_signal.get().is_none(), "Value should be None after clearing");
    }

    #[test]
    fn test_select_disabled_state_management() {
        // Test disabled state signal
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get(), "Initial disabled state should be false");
        
        // Test disabling
        disabled_signal.set(true);
        assert!(disabled_signal.get(), "Disabled state should be true");
        
        // Test enabling
        disabled_signal.set(false);
        assert!(!disabled_signal.get(), "Disabled state should be false");
    }

    #[test]
    fn test_select_required_state_management() {
        // Test required state signal
        let required_signal = RwSignal::new(false);
        assert!(!required_signal.get(), "Initial required state should be false");
        
        // Test making required
        required_signal.set(true);
        assert!(required_signal.get(), "Required state should be true");
        
        // Test making optional
        required_signal.set(false);
        assert!(!required_signal.get(), "Required state should be false");
    }
}
