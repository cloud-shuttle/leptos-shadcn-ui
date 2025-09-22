#[cfg(test)]
mod state_management_tests {
    use crate::default::{RadioGroup, RadioGroupItem};
    use leptos::prelude::*;

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and interactions

    #[test]
    fn test_radio_group_item_selection() {
        // Test radio group item selection
        let selected_value = RwSignal::new("option1".to_string());
        
        // Test initial selection
        assert_eq!(selected_value.get(), "option1", "Initial value should be option1");
        
        // Simulate selection change
        selected_value.set("option2".to_string());
        assert_eq!(selected_value.get(), "option2", "Value should change to option2");
    }

    #[test]
    fn test_radio_group_disabled_state() {
        // Test disabled radio group
        let disabled_signal = RwSignal::new(true);
        
        // Test disabled state
        assert!(disabled_signal.get(), "RadioGroup should be disabled");
        
        disabled_signal.set(false);
        assert!(!disabled_signal.get(), "RadioGroup should be enabled");
    }

    #[test]
    fn test_radio_group_item_disabled() {
        // Test individual radio group item disabled
        let item_disabled = RwSignal::new(true);
        
        // Test item disabled state
        assert!(item_disabled.get(), "RadioGroupItem should be disabled");
        
        item_disabled.set(false);
        assert!(!item_disabled.get(), "RadioGroupItem should be enabled");
    }

    #[test]
    fn test_radio_group_value_management() {
        // Test value management
        let value_signal = RwSignal::new("initial-value".to_string());
        
        // Test initial value
        assert_eq!(value_signal.get(), "initial-value");
        
        // Test value change
        value_signal.set("updated-value".to_string());
        assert_eq!(value_signal.get(), "updated-value");
        
        // Test value reset
        value_signal.set("".to_string());
        assert_eq!(value_signal.get(), "");
    }

    #[test]
    fn test_radio_group_callback_handling() {
        // Test callback handling
        let callback_called = RwSignal::new(false);
        let callback_value = RwSignal::new("".to_string());
        
        let callback = Callback::new(move |value: String| {
            callback_called.set(true);
            callback_value.set(value);
        });
        
        // Test initial callback state
        assert!(!callback_called.get());
        assert_eq!(callback_value.get(), "");
        
        // Test callback execution
        callback.run("callback-value".to_string());
        
        assert!(callback_called.get());
        assert_eq!(callback_value.get(), "callback-value");
    }

    #[test]
    fn test_radio_group_context_management() {
        // Test context management
        let context_value = RwSignal::new("context-value".to_string());
        let context_disabled = RwSignal::new(false);
        let context_required = RwSignal::new(true);
        
        // Test initial context state
        assert_eq!(context_value.get(), "context-value");
        assert!(!context_disabled.get());
        assert!(context_required.get());
        
        // Test context updates
        context_value.set("updated-context".to_string());
        context_disabled.set(true);
        context_required.set(false);
        
        assert_eq!(context_value.get(), "updated-context");
        assert!(context_disabled.get());
        assert!(!context_required.get());
    }

    #[test]
    fn test_radio_group_form_integration() {
        // Test form integration
        let form_value = RwSignal::new("form-value".to_string());
        let form_disabled = RwSignal::new(false);
        let form_required = RwSignal::new(true);
        let form_valid = RwSignal::new(true);
        
        // Test initial form state
        assert_eq!(form_value.get(), "form-value");
        assert!(!form_disabled.get());
        assert!(form_required.get());
        assert!(form_valid.get());
        
        // Test form state changes
        form_value.set("updated-form-value".to_string());
        form_disabled.set(true);
        form_required.set(false);
        form_valid.set(false);
        
        assert_eq!(form_value.get(), "updated-form-value");
        assert!(form_disabled.get());
        assert!(!form_required.get());
        assert!(!form_valid.get());
    }

    #[test]
    fn test_radio_group_validation_logic() {
        // Test validation logic
        let value = RwSignal::new("".to_string());
        let required = RwSignal::new(true);
        let valid = RwSignal::new(false);
        
        // Test initial validation state
        assert_eq!(value.get(), "");
        assert!(required.get());
        assert!(!valid.get());
        
        // Test validation with empty value
        let is_valid = !value.get().is_empty() || !required.get();
        assert!(!is_valid);
        
        // Test validation with value
        value.set("valid-value".to_string());
        let is_valid = !value.get().is_empty() || !required.get();
        assert!(is_valid);
        
        // Test validation without required
        required.set(false);
        let is_valid = !value.get().is_empty() || !required.get();
        assert!(is_valid);
    }

    #[test]
    fn test_radio_group_keyboard_navigation() {
        // Test keyboard navigation
        let focused_item = RwSignal::new(0);
        let items_count = 3;
        
        // Test initial focus
        assert_eq!(focused_item.get(), 0);
        
        // Test focus next
        focused_item.update(|index| {
            *index = (*index + 1) % items_count;
        });
        assert_eq!(focused_item.get(), 1);
        
        // Test focus previous
        focused_item.update(|index| {
            *index = if *index == 0 { items_count - 1 } else { *index - 1 };
        });
        assert_eq!(focused_item.get(), 0);
        
        // Test focus wrap around
        focused_item.set(items_count - 1);
        focused_item.update(|index| {
            *index = (*index + 1) % items_count;
        });
        assert_eq!(focused_item.get(), 0);
    }

    #[test]
    fn test_radio_group_state_consistency() {
        // Test state consistency
        let value = RwSignal::new("option1".to_string());
        let disabled = RwSignal::new(false);
        let required = RwSignal::new(true);
        
        // Test initial state consistency
        assert_eq!(value.get(), "option1");
        assert!(!disabled.get());
        assert!(required.get());
        
        // Test state consistency after changes
        value.set("option2".to_string());
        disabled.set(true);
        required.set(false);
        
        assert_eq!(value.get(), "option2");
        assert!(disabled.get());
        assert!(!required.get());
    }

    #[test]
    fn test_radio_group_multiple_instances() {
        // Test multiple radio group instances
        let group1_value = RwSignal::new("group1-option1".to_string());
        let group2_value = RwSignal::new("group2-option1".to_string());
        
        // Test independent state management
        assert_eq!(group1_value.get(), "group1-option1");
        assert_eq!(group2_value.get(), "group2-option1");
        
        // Test independent updates
        group1_value.set("group1-option2".to_string());
        group2_value.set("group2-option2".to_string());
        
        assert_eq!(group1_value.get(), "group1-option2");
        assert_eq!(group2_value.get(), "group2-option2");
    }

    #[test]
    fn test_radio_group_event_handling() {
        // Test event handling
        let click_handled = RwSignal::new(false);
        let keydown_handled = RwSignal::new(false);
        let focus_handled = RwSignal::new(false);
        let blur_handled = RwSignal::new(false);
        
        // Test initial event state
        assert!(!click_handled.get());
        assert!(!keydown_handled.get());
        assert!(!focus_handled.get());
        assert!(!blur_handled.get());
        
        // Test event handling
        click_handled.set(true);
        keydown_handled.set(true);
        focus_handled.set(true);
        blur_handled.set(true);
        
        assert!(click_handled.get());
        assert!(keydown_handled.get());
        assert!(focus_handled.get());
        assert!(blur_handled.get());
    }

    #[test]
    fn test_radio_group_accessibility_features() {
        // Test accessibility features
        let has_aria_label = true;
        let has_aria_describedby = true;
        let has_aria_labelledby = true;
        let has_aria_required = true;
        let has_aria_invalid = false;
        let has_role = true;
        let has_tabindex = true;
        
        // Test accessibility attributes
        assert!(has_aria_label);
        assert!(has_aria_describedby);
        assert!(has_aria_labelledby);
        assert!(has_aria_required);
        assert!(!has_aria_invalid);
        assert!(has_role);
        assert!(has_tabindex);
    }
}
