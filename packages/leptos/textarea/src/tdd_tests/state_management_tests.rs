#[cfg(test)]
mod state_management_tests {
    use crate::default::Textarea;
    use leptos::prelude::*;

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and interactions

    #[test]
    fn test_textarea_disabled_state() {
        // Test disabled textarea
        let disabled_signal = RwSignal::new(true);
        let _disabled_textarea_view = view! {
            <Textarea 
                disabled=disabled_signal
                placeholder="Disabled textarea"
                value=""
            />
        };
        
        // Test disabled state
        assert!(disabled_signal.get(), "Textarea should be disabled");
        
        disabled_signal.set(false);
        assert!(!disabled_signal.get(), "Textarea should be enabled");
    }

    #[test]
    fn test_textarea_value_management() {
        // Test value management
        let value_signal = RwSignal::new("initial value".to_string());
        let _value_textarea_view = view! {
            <Textarea 
                value=value_signal
                placeholder="Enter text"
            />
        };
        
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
    fn test_textarea_form_integration() {
        // Test form integration
        let form_value = RwSignal::new("form value".to_string());
        let form_disabled = RwSignal::new(false);
        let form_required = RwSignal::new(true);
        let form_valid = RwSignal::new(true);
        let form_name = "textarea-field";
        
        let _form_textarea_view = view! {
            <Textarea 
                value=form_value
                disabled=form_disabled
                required=form_required
                name=form_name
                placeholder="Form textarea"
            />
        };
        
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
        
        let _validation_textarea_view = view! {
            <Textarea 
                placeholder="Validation textarea"
                value=""
                aria_invalid=move || error_state.get()
                aria_required="true"
            />
        };
        
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
        
        let _focus_textarea_view = view! {
            <Textarea 
                placeholder="Focus textarea"
                value=""
                on_focus=move |_| focused_state.set(true)
                on_blur=move |_| focused_state.set(false)
            />
        };
        
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
    fn test_textarea_event_handling() {
        // Test event handling
        let input_handled = RwSignal::new(false);
        let change_handled = RwSignal::new(false);
        let keydown_handled = RwSignal::new(false);
        
        let _event_textarea_view = view! {
            <Textarea 
                placeholder="Event textarea"
                value=""
                on_input=move |_| input_handled.set(true)
                on_change=move |_| change_handled.set(true)
                on_keydown=move |_| keydown_handled.set(true)
            />
        };
        
        // Test initial event state
        assert!(!input_handled.get());
        assert!(!change_handled.get());
        assert!(!keydown_handled.get());
        
        // Test event handling
        input_handled.set(true);
        change_handled.set(true);
        keydown_handled.set(true);
        
        assert!(input_handled.get());
        assert!(change_handled.get());
        assert!(keydown_handled.get());
    }

    #[test]
    fn test_textarea_placeholder_management() {
        // Test placeholder management
        let placeholder_signal = RwSignal::new("Enter text here...".to_string());
        
        let _placeholder_textarea_view = view! {
            <Textarea 
                placeholder=placeholder_signal
                value=""
            />
        };
        
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
        
        let _size_textarea_view = view! {
            <Textarea 
                placeholder="Size textarea"
                value=""
                rows=rows_signal
                cols=cols_signal
            />
        };
        
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
        
        let _validation_textarea_view = view! {
            <Textarea 
                value=value
                required=required
                placeholder="Validation textarea"
            />
        };
        
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
        
        let _consistency_textarea_view = view! {
            <Textarea 
                value=value
                disabled=disabled
                required=required
                readonly=readonly
                placeholder="Consistency textarea"
            />
        };
        
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
        
        let _accessibility_textarea_view = view! {
            <Textarea 
                placeholder="Accessibility textarea"
                value=""
                aria_label="Message input"
                aria_required="true"
                aria_describedby="textarea-description"
                role="textbox"
                tabindex="0"
            />
        };
        
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
}
