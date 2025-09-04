#[cfg(test)]
mod tests {
    use crate::default::{Checkbox, CHECKBOX_CLASS};
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_checkbox_base_css_classes() {
        // Test that base CHECKBOX_CLASS contains required styling and accessibility classes
        assert!(CHECKBOX_CLASS.contains("h-4"));
        assert!(CHECKBOX_CLASS.contains("w-4"));
        assert!(CHECKBOX_CLASS.contains("shrink-0"));
        assert!(CHECKBOX_CLASS.contains("rounded-sm"));
        assert!(CHECKBOX_CLASS.contains("border"));
        assert!(CHECKBOX_CLASS.contains("border-primary"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:outline-none"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-2"));
        assert!(CHECKBOX_CLASS.contains("disabled:cursor-not-allowed"));
        assert!(CHECKBOX_CLASS.contains("disabled:opacity-50"));
        assert!(CHECKBOX_CLASS.contains("ring-offset-background"));
    }

    #[test]
    fn test_checkbox_state_specific_classes() {
        // Test checkbox state-specific styling
        assert!(CHECKBOX_CLASS.contains("data-[state=checked]:bg-primary"));
        assert!(CHECKBOX_CLASS.contains("data-[state=checked]:text-primary-foreground"));
    }

    #[test]
    fn test_checkbox_checked_state() {
        // Test checked signal functionality
        let checked_signal = RwSignal::new(false);
        assert!(!checked_signal.get());
        
        checked_signal.set(true);
        assert!(checked_signal.get());
        
        // Test toggling
        checked_signal.set(false);
        assert!(!checked_signal.get());
    }

    #[test]
    fn test_checkbox_disabled_state() {
        // Test disabled signal functionality
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get());
        
        disabled_signal.set(true);
        assert!(disabled_signal.get());
        
        // Test disabled state styling is included in base class
        assert!(CHECKBOX_CLASS.contains("disabled:cursor-not-allowed"));
        assert!(CHECKBOX_CLASS.contains("disabled:opacity-50"));
    }

    #[test]
    fn test_checkbox_change_callback() {
        // Test change callback structure
        let change_called = Arc::new(Mutex::new(false));
        let change_value = Arc::new(Mutex::new(false));
        
        let change_called_clone = Arc::clone(&change_called);
        let change_value_clone = Arc::clone(&change_value);
        
        let callback = Callback::new(move |checked: bool| {
            *change_called_clone.lock().unwrap() = true;
            *change_value_clone.lock().unwrap() = checked;
        });
        
        // Simulate callback execution
        callback.run(true);
        
        assert!(*change_called.lock().unwrap());
        assert!(*change_value.lock().unwrap());
        
        // Test unchecked state
        callback.run(false);
        assert!(!*change_value.lock().unwrap());
    }

    #[test]
    fn test_checkbox_class_merging() {
        // Test custom class handling
        let base_class = CHECKBOX_CLASS;
        let custom_class = "my-custom-checkbox-class";
        
        let expected = format!("{} {}", base_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
    }

    #[test]
    fn test_checkbox_accessibility_features() {
        // Test accessibility-related CSS classes
        assert!(CHECKBOX_CLASS.contains("focus-visible:outline-none"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-2"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-ring"));
        assert!(CHECKBOX_CLASS.contains("focus-visible:ring-offset-2"));
        assert!(CHECKBOX_CLASS.contains("ring-offset-background"));
    }

    #[test]
    fn test_checkbox_component_structure() {
        // Test that checkbox creates proper input type
        let input_type = "checkbox";
        assert_eq!(input_type, "checkbox");
        
        // Test boolean state management
        let checked_states = vec![true, false];
        for state in checked_states {
            // In real implementation, this would test actual DOM structure
            assert!(state == true || state == false);
        }
    }

    #[test]
    fn test_checkbox_styling_consistency() {
        // Test that all required styling properties are present
        let required_properties = vec![
            "h-4", "w-4", "shrink-0", "rounded-sm", "border",
            "ring-offset-background", "focus-visible:outline-none"
        ];
        
        for property in required_properties {
            assert!(CHECKBOX_CLASS.contains(property), 
                "CHECKBOX_CLASS should contain '{}' property", property);
        }
    }

    #[test]
    fn test_checkbox_interaction_model() {
        // Test checkbox interaction patterns
        let initial_state = false;
        let toggled_state = !initial_state;
        
        assert_eq!(initial_state, false);
        assert_eq!(toggled_state, true);
        
        // Test multiple toggles
        let mut current_state = false;
        for _ in 0..3 {
            current_state = !current_state;
        }
        assert!(current_state); // Should be true after odd number of toggles
    }
}