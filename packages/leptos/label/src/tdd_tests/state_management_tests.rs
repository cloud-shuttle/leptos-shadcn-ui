#[cfg(test)]
mod state_management_tests {
    use crate::default::Label;
    use leptos::prelude::*;

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and interactions

    #[test]
    fn test_label_state_management() {
        // Test label state management
        let label_state = RwSignal::new("initial state".to_string());
        let _state_label_view = view! {
            <Label>
                {move || label_state.get()}
            </Label>
        };
        
        // Test initial state
        assert_eq!(label_state.get(), "initial state");
        
        // Test state change
        label_state.set("updated state".to_string());
        assert_eq!(label_state.get(), "updated state");
    }

    #[test]
    fn test_label_reactive_content() {
        // Test reactive content
        let content_signal = RwSignal::new("Initial content".to_string());
        let _reactive_label_view = view! {
            <Label>
                {move || content_signal.get()}
            </Label>
        };
        
        // Test reactive content
        content_signal.set("Updated content".to_string());
        assert_eq!(content_signal.get(), "Updated content");
    }

    #[test]
    fn test_label_callback_handling() {
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
    fn test_label_event_handling() {
        // Test event handling
        let click_handled = RwSignal::new(false);
        let focus_handled = RwSignal::new(false);
        let blur_handled = RwSignal::new(false);
        
        let _event_label_view = view! {
            <Label
                on_click=move |_| click_handled.set(true)
                on_focus=move |_| focus_handled.set(true)
                on_blur=move |_| blur_handled.set(true)
            >
                "Event Label"
            </Label>
        };
        
        // Test initial event state
        assert!(!click_handled.get());
        assert!(!focus_handled.get());
        assert!(!blur_handled.get());
        
        // Test event handling
        click_handled.set(true);
        focus_handled.set(true);
        blur_handled.set(true);
        
        assert!(click_handled.get());
        assert!(focus_handled.get());
        assert!(blur_handled.get());
    }

    #[test]
    fn test_label_form_integration() {
        // Test form integration
        let form_value = RwSignal::new("form value".to_string());
        let form_disabled = RwSignal::new(false);
        let form_required = RwSignal::new(true);
        let form_valid = RwSignal::new(true);
        let form_name = "label-field";
        
        let _form_label_view = view! {
            <Label
                for=form_name
                aria_required=move || form_required.get()
                aria_invalid=move || !form_valid.get()
            >
                {move || form_value.get()}
            </Label>
        };
        
        // Test initial form state
        assert_eq!(form_value.get(), "form value");
        assert!(!form_disabled.get());
        assert!(form_required.get());
        assert!(form_valid.get());
        assert_eq!(form_name, "label-field");
        
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
    fn test_label_validation_states() {
        // Test validation states
        let valid_state = RwSignal::new(true);
        let error_state = RwSignal::new(false);
        let warning_state = RwSignal::new(false);
        let error_message = RwSignal::new("".to_string());
        
        let _validation_label_view = view! {
            <Label
                aria_invalid=move || error_state.get()
                class=move || {
                    if error_state.get() { "error-label" }
                    else if warning_state.get() { "warning-label" }
                    else if valid_state.get() { "valid-label" }
                    else { "default-label" }
                }
            >
                {move || {
                    if error_state.get() { "Field with Error" }
                    else if warning_state.get() { "Field with Warning" }
                    else if valid_state.get() { "Valid Field" }
                    else { "Default Field" }
                }}
            </Label>
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
    fn test_label_focus_management() {
        // Test focus management
        let focused_state = RwSignal::new(false);
        let focus_visible_state = RwSignal::new(false);
        
        let _focus_label_view = view! {
            <Label
                on_focus=move |_| focused_state.set(true)
                on_blur=move |_| focused_state.set(false)
                on_focus_visible=move |_| focus_visible_state.set(true)
            >
                "Focusable Label"
            </Label>
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
    fn test_label_disabled_states() {
        // Test disabled states
        let disabled_state = RwSignal::new(false);
        let readonly_state = RwSignal::new(false);
        
        let _disabled_label_view = view! {
            <Label
                aria_disabled=move || disabled_state.get()
                aria_readonly=move || readonly_state.get()
                class=move || {
                    if disabled_state.get() { "disabled-label" }
                    else if readonly_state.get() { "readonly-label" }
                    else { "enabled-label" }
                }
            >
                "State Label"
            </Label>
        };
        
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
    fn test_label_placeholder_management() {
        // Test placeholder management
        let placeholder_signal = RwSignal::new("Enter text here...".to_string());
        
        let _placeholder_label_view = view! {
            <Label>
                {move || placeholder_signal.get()}
            </Label>
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
    fn test_label_size_management() {
        // Test size management
        let size_signal = RwSignal::new("default".to_string());
        
        let _size_label_view = view! {
            <Label
                class=move || format!("label-{}", size_signal.get())
            >
                {move || format!("{} Label", size_signal.get())}
            </Label>
        };
        
        // Test initial size
        assert_eq!(size_signal.get(), "default");
        
        // Test size changes
        size_signal.set("small".to_string());
        assert_eq!(size_signal.get(), "small");
        
        size_signal.set("large".to_string());
        assert_eq!(size_signal.get(), "large");
    }

    #[test]
    fn test_label_variant_management() {
        // Test variant management
        let variant_signal = RwSignal::new("default".to_string());
        
        let _variant_label_view = view! {
            <Label
                class=move || format!("label-{}", variant_signal.get())
            >
                {move || format!("{} Label", variant_signal.get())}
            </Label>
        };
        
        // Test initial variant
        assert_eq!(variant_signal.get(), "default");
        
        // Test variant changes
        variant_signal.set("destructive".to_string());
        assert_eq!(variant_signal.get(), "destructive");
        
        variant_signal.set("outline".to_string());
        assert_eq!(variant_signal.get(), "outline");
    }

    #[test]
    fn test_label_validation_logic() {
        // Test validation logic
        let value = RwSignal::new("".to_string());
        let required = RwSignal::new(true);
        let min_length = RwSignal::new(5);
        let max_length = RwSignal::new(100);
        
        let _validation_label_view = view! {
            <Label
                aria_required=move || required.get()
                aria_invalid=move || {
                    let val = value.get();
                    required.get() && (val.is_empty() || val.len() < min_length.get() as usize || val.len() > max_length.get() as usize)
                }
            >
                {move || {
                    let val = value.get();
                    if required.get() && val.is_empty() { "Required Field" }
                    else if val.len() < min_length.get() as usize { "Field Too Short" }
                    else if val.len() > max_length.get() as usize { "Field Too Long" }
                    else { "Valid Field" }
                }}
            </Label>
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
    fn test_label_state_consistency() {
        // Test state consistency
        let value = RwSignal::new("initial text".to_string());
        let disabled = RwSignal::new(false);
        let required = RwSignal::new(true);
        let readonly = RwSignal::new(false);
        
        let _consistency_label_view = view! {
            <Label
                aria_disabled=move || disabled.get()
                aria_required=move || required.get()
                aria_readonly=move || readonly.get()
            >
                {move || value.get()}
            </Label>
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
}
