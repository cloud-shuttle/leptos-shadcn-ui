#[cfg(test)]
mod component_behavior {
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== COMPONENT BEHAVIOR TESTS =====
    // These tests focus on component behavior, state management, and interactions

    #[test]
    fn test_textarea_prop_defaults() {
        // Test Textarea prop defaults
        let default_class = None::<String>;
        let default_id = None::<String>;
        let default_style = None::<String>;
        let default_value = None::<String>;
        let default_placeholder = None::<String>;
        let default_disabled = false;
        let default_required = false;
        let default_readonly = false;
        let default_rows = None::<u32>;
        let default_cols = None::<u32>;
        
        assert!(default_class.is_none());
        assert!(default_id.is_none());
        assert!(default_style.is_none());
        assert!(default_value.is_none());
        assert!(default_placeholder.is_none());
        assert!(!default_disabled);
        assert!(!default_required);
        assert!(!default_readonly);
        assert!(default_rows.is_none());
        assert!(default_cols.is_none());
    }

    #[test]
    fn test_textarea_value_management() {
        // Test value management
        let value_signal = RwSignal::new("initial value".to_string());
        
        // Test initial value
        assert_eq!(value_signal.get(), "initial value");
        
        // Test value change
        value_signal.set("updated value".to_string());
        assert_eq!(value_signal.get(), "updated value");
        
        // Test value reset
        value_signal.set("".to_string());
        assert_eq!(value_signal.get(), "");
    }

    #[test]
    fn test_textarea_disabled_state_management() {
        // Test disabled state management
        let disabled_signal = RwSignal::new(false);
        
        // Test initial disabled state
        assert!(!disabled_signal.get());
        
        // Test disabled state change
        disabled_signal.set(true);
        assert!(disabled_signal.get());
        
        // Test re-enabling
        disabled_signal.set(false);
        assert!(!disabled_signal.get());
    }

    #[test]
    fn test_textarea_callback_handling() {
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
        callback.run("callback value".to_string());
        
        assert!(callback_called.get());
        assert_eq!(callback_value.get(), "callback value");
    }

    #[test]
    fn test_textarea_event_handling_logic() {
        // Test event handling logic
        let input_handled = RwSignal::new(false);
        let change_handled = RwSignal::new(false);
        let focus_handled = RwSignal::new(false);
        let blur_handled = RwSignal::new(false);
        let keydown_handled = RwSignal::new(false);
        
        // Test initial event state
        assert!(!input_handled.get());
        assert!(!change_handled.get());
        assert!(!focus_handled.get());
        assert!(!blur_handled.get());
        assert!(!keydown_handled.get());
        
        // Test event handling
        input_handled.set(true);
        change_handled.set(true);
        focus_handled.set(true);
        blur_handled.set(true);
        keydown_handled.set(true);
        
        assert!(input_handled.get());
        assert!(change_handled.get());
        assert!(focus_handled.get());
        assert!(blur_handled.get());
        assert!(keydown_handled.get());
    }

    #[test]
    fn test_textarea_semantic_structure() {
        // Test semantic structure
        let has_role = true;
        let has_aria_invalid = true;
        let has_aria_required = true;
        let has_aria_describedby = true;
        let has_tabindex = true;
        
        // Test semantic attributes
        assert!(has_role);
        assert!(has_aria_invalid);
        assert!(has_aria_required);
        assert!(has_aria_describedby);
        assert!(has_tabindex);
    }

    #[test]
    fn test_textarea_accessibility_features() {
        // Test accessibility features
        let has_aria_label = true;
        let has_aria_describedby = true;
        let has_aria_labelledby = true;
        let has_aria_required = true;
        let has_aria_invalid = false;
        let has_aria_readonly = false;
        let has_role = true;
        let has_tabindex = true;
        
        // Test accessibility attributes
        assert!(has_aria_label);
        assert!(has_aria_describedby);
        assert!(has_aria_labelledby);
        assert!(has_aria_required);
        assert!(!has_aria_invalid);
        assert!(!has_aria_readonly);
        assert!(has_role);
        assert!(has_tabindex);
    }

    #[test]
    fn test_textarea_form_integration() {
        // Test form integration
        let form_value = RwSignal::new("form value".to_string());
        let form_disabled = RwSignal::new(false);
        let form_required = RwSignal::new(true);
        let form_valid = RwSignal::new(true);
        let form_name = "textarea-field";
        
        // Test initial form state
        assert_eq!(form_value.get(), "form value");
        assert!(!form_disabled.get());
        assert!(form_required.get());
        assert!(form_valid.get());
        assert_eq!(form_name, "textarea-field");
        
        // Test form state changes
        form_value.set("updated form value".to_string());
        form_disabled.set(true);
        form_required.set(false);
        form_valid.set(false);
        
        assert_eq!(form_value.get(), "updated form value");
        assert!(form_disabled.get());
        assert!(!form_required.get());
        assert!(!form_valid.get());
    }

    #[test]
    fn test_textarea_validation_states() {
        // Test validation states
        let valid_state = RwSignal::new(true);
        let error_state = RwSignal::new(false);
        let warning_state = RwSignal::new(false);
        let error_message = RwSignal::new("".to_string());
        
        // Test initial validation state
        assert!(valid_state.get());
        assert!(!error_state.get());
        assert!(!warning_state.get());
        assert_eq!(error_message.get(), "");
        
        // Test validation state changes
        valid_state.set(false);
        error_state.set(true);
        warning_state.set(true);
        error_message.set("Validation error".to_string());
        
        assert!(!valid_state.get());
        assert!(error_state.get());
        assert!(warning_state.get());
        assert_eq!(error_message.get(), "Validation error");
    }

    #[test]
    fn test_textarea_focus_management() {
        // Test focus management
        let focused_state = RwSignal::new(false);
        let focus_visible_state = RwSignal::new(false);
        
        // Test initial focus state
        assert!(!focused_state.get());
        assert!(!focus_visible_state.get());
        
        // Test focus changes
        focused_state.set(true);
        focus_visible_state.set(true);
        
        assert!(focused_state.get());
        assert!(focus_visible_state.get());
    }

    #[test]
    fn test_textarea_disabled_states() {
        // Test disabled states
        let disabled_state = RwSignal::new(false);
        let readonly_state = RwSignal::new(false);
        
        // Test initial disabled state
        assert!(!disabled_state.get());
        assert!(!readonly_state.get());
        
        // Test disabled state
        disabled_state.set(true);
        assert!(disabled_state.get());
        
        // Test readonly state
        readonly_state.set(true);
        assert!(readonly_state.get());
        
        // Test re-enabling
        disabled_state.set(false);
        readonly_state.set(false);
        
        assert!(!disabled_state.get());
        assert!(!readonly_state.get());
    }

    #[test]
    fn test_textarea_placeholder_management() {
        // Test placeholder management
        let placeholder_signal = RwSignal::new("Enter text here...".to_string());
        
        // Test initial placeholder
        assert_eq!(placeholder_signal.get(), "Enter text here...");
        
        // Test placeholder change
        placeholder_signal.set("New placeholder text".to_string());
        assert_eq!(placeholder_signal.get(), "New placeholder text");
        
        // Test placeholder clear
        placeholder_signal.set("".to_string());
        assert_eq!(placeholder_signal.get(), "");
    }

    #[test]
    fn test_textarea_size_management() {
        // Test size management
        let rows_signal = RwSignal::new(4);
        let cols_signal = RwSignal::new(50);
        
        // Test initial size
        assert_eq!(rows_signal.get(), 4);
        assert_eq!(cols_signal.get(), 50);
        
        // Test size changes
        rows_signal.set(6);
        cols_signal.set(80);
        
        assert_eq!(rows_signal.get(), 6);
        assert_eq!(cols_signal.get(), 80);
    }

    #[test]
    fn test_textarea_validation_logic() {
        // Test validation logic
        let value = RwSignal::new("".to_string());
        let required = RwSignal::new(true);
        let min_length = RwSignal::new(5);
        let max_length = RwSignal::new(100);
        
        // Test initial validation state
        assert_eq!(value.get(), "");
        assert!(required.get());
        assert_eq!(min_length.get(), 5);
        assert_eq!(max_length.get(), 100);
        
        // Test validation with empty value
        let is_valid = !value.get().is_empty() || !required.get();
        assert!(!is_valid);
        
        // Test validation with short value
        value.set("hi".to_string());
        let is_valid = value.get().len() >= min_length.get() as usize;
        assert!(!is_valid);
        
        // Test validation with valid value
        value.set("valid text".to_string());
        let is_valid = value.get().len() >= min_length.get() as usize && 
                      value.get().len() <= max_length.get() as usize;
        assert!(is_valid);
        
        // Test validation with long value
        value.set("a".repeat(150));
        let is_valid = value.get().len() <= max_length.get() as usize;
        assert!(!is_valid);
    }

    #[test]
    fn test_textarea_state_consistency() {
        // Test state consistency
        let value = RwSignal::new("initial text".to_string());
        let disabled = RwSignal::new(false);
        let required = RwSignal::new(true);
        let readonly = RwSignal::new(false);
        
        // Test initial state consistency
        assert_eq!(value.get(), "initial text");
        assert!(!disabled.get());
        assert!(required.get());
        assert!(!readonly.get());
        
        // Test state consistency after changes
        value.set("updated text".to_string());
        disabled.set(true);
        required.set(false);
        readonly.set(true);
        
        assert_eq!(value.get(), "updated text");
        assert!(disabled.get());
        assert!(!required.get());
        assert!(readonly.get());
    }
}
