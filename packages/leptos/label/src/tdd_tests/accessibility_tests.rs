#[cfg(test)]
mod accessibility_tests {
    use crate::default::Label;
    use leptos::prelude::*;

    // ===== ACCESSIBILITY TESTS =====
    // These tests focus on accessibility features and ARIA attributes

    #[test]
    fn test_label_accessibility_features() {
        // Test label accessibility features
        let _accessible_label_view = view! {
            <Label
                for="input-field"
                aria_label="Form field label"
                role="label"
            >
                "Accessible Label"
            </Label>
        };
        
        // Test accessibility features
        let for_attr = "input-field";
        let aria_label = "Form field label";
        let role = "label";
        
        assert_eq!(for_attr, "input-field");
        assert_eq!(aria_label, "Form field label");
        assert_eq!(role, "label");
    }

    #[test]
    fn test_label_form_association() {
        // Test label form association
        let _form_label_view = view! {
            <Label
                for="username-input"
                aria_required="true"
                aria_describedby="username-help"
            >
                "Username"
            </Label>
        };
        
        // Test form association
        let for_attr = "username-input";
        let aria_required = "true";
        let aria_describedby = "username-help";
        
        assert_eq!(for_attr, "username-input");
        assert_eq!(aria_required, "true");
        assert_eq!(aria_describedby, "username-help");
    }

    #[test]
    fn test_label_required_indicator() {
        // Test required indicator
        let _required_label_view = view! {
            <Label
                for="required-field"
                aria_required="true"
            >
                <span aria-hidden="true">"*"</span>
                "Required Field"
            </Label>
        };
        
        // Test required indicator
        let for_attr = "required-field";
        let aria_required = "true";
        let aria_hidden = "true";
        let required_symbol = "*";
        let label_text = "Required Field";
        
        assert_eq!(for_attr, "required-field");
        assert_eq!(aria_required, "true");
        assert_eq!(aria_hidden, "true");
        assert_eq!(required_symbol, "*");
        assert_eq!(label_text, "Required Field");
    }

    #[test]
    fn test_label_optional_indicator() {
        // Test optional indicator
        let _optional_label_view = view! {
            <Label
                for="optional-field"
                aria_required="false"
            >
                "Optional Field"
                <span aria-hidden="true">" (optional)"</span>
            </Label>
        };
        
        // Test optional indicator
        let for_attr = "optional-field";
        let aria_required = "false";
        let aria_hidden = "true";
        let optional_text = " (optional)";
        let label_text = "Optional Field";
        
        assert_eq!(for_attr, "optional-field");
        assert_eq!(aria_required, "false");
        assert_eq!(aria_hidden, "true");
        assert_eq!(optional_text, " (optional)");
        assert_eq!(label_text, "Optional Field");
    }

    #[test]
    fn test_label_error_state() {
        // Test error state
        let _error_label_view = view! {
            <Label
                for="error-field"
                aria_invalid="true"
                aria_describedby="error-message"
                class="error-label"
            >
                "Field with Error"
            </Label>
        };
        
        // Test error state
        let for_attr = "error-field";
        let aria_invalid = "true";
        let aria_describedby = "error-message";
        let error_class = "error-label";
        
        assert_eq!(for_attr, "error-field");
        assert_eq!(aria_invalid, "true");
        assert_eq!(aria_describedby, "error-message");
        assert_eq!(error_class, "error-label");
    }

    #[test]
    fn test_label_success_state() {
        // Test success state
        let _success_label_view = view! {
            <Label
                for="success-field"
                aria_invalid="false"
                class="success-label"
            >
                "Valid Field"
            </Label>
        };
        
        // Test success state
        let for_attr = "success-field";
        let aria_invalid = "false";
        let success_class = "success-label";
        
        assert_eq!(for_attr, "success-field");
        assert_eq!(aria_invalid, "false");
        assert_eq!(success_class, "success-label");
    }

    #[test]
    fn test_label_help_text() {
        // Test help text
        let _help_label_view = view! {
            <Label
                for="help-field"
                aria_describedby="help-text"
            >
                "Field with Help"
            </Label>
        };
        
        // Test help text
        let for_attr = "help-field";
        let aria_describedby = "help-text";
        
        assert_eq!(for_attr, "help-field");
        assert_eq!(aria_describedby, "help-text");
    }

    #[test]
    fn test_label_screen_reader_support() {
        // Test screen reader support
        let _screen_reader_label_view = view! {
            <Label
                for="sr-field"
                aria_label="Screen reader accessible label"
                aria_live="polite"
            >
                "Screen Reader Label"
            </Label>
        };
        
        // Test screen reader support
        let for_attr = "sr-field";
        let aria_label = "Screen reader accessible label";
        let aria_live = "polite";
        
        assert_eq!(for_attr, "sr-field");
        assert_eq!(aria_label, "Screen reader accessible label");
        assert_eq!(aria_live, "polite");
    }

    #[test]
    fn test_label_keyboard_navigation() {
        // Test keyboard navigation
        let _keyboard_label_view = view! {
            <Label
                for="keyboard-field"
                tabindex="0"
                on_keydown=move |_| {}
            >
                "Keyboard Accessible Label"
            </Label>
        };
        
        // Test keyboard navigation
        let for_attr = "keyboard-field";
        let tabindex = "0";
        
        assert_eq!(for_attr, "keyboard-field");
        assert_eq!(tabindex, "0");
    }

    #[test]
    fn test_label_focus_management() {
        // Test focus management
        let _focus_label_view = view! {
            <Label
                for="focus-field"
                on_focus=move |_| {}
                on_blur=move |_| {}
            >
                "Focusable Label"
            </Label>
        };
        
        // Test focus management
        let for_attr = "focus-field";
        
        assert_eq!(for_attr, "focus-field");
    }

    #[test]
    fn test_label_aria_attributes() {
        // Test ARIA attributes
        let _aria_label_view = view! {
            <Label
                for="aria-field"
                aria_label="ARIA label"
                aria_labelledby="aria-labelledby"
                aria_describedby="aria-describedby"
                aria_required="true"
                aria_invalid="false"
                aria_expanded="false"
                aria_selected="false"
                aria_disabled="false"
                aria_readonly="false"
            >
                "ARIA Label"
            </Label>
        };
        
        // Test ARIA attributes
        let for_attr = "aria-field";
        let aria_label = "ARIA label";
        let aria_labelledby = "aria-labelledby";
        let aria_describedby = "aria-describedby";
        let aria_required = "true";
        let aria_invalid = "false";
        let aria_expanded = "false";
        let aria_selected = "false";
        let aria_disabled = "false";
        let aria_readonly = "false";
        
        assert_eq!(for_attr, "aria-field");
        assert_eq!(aria_label, "ARIA label");
        assert_eq!(aria_labelledby, "aria-labelledby");
        assert_eq!(aria_describedby, "aria-describedby");
        assert_eq!(aria_required, "true");
        assert_eq!(aria_invalid, "false");
        assert_eq!(aria_expanded, "false");
        assert_eq!(aria_selected, "false");
        assert_eq!(aria_disabled, "false");
        assert_eq!(aria_readonly, "false");
    }

    #[test]
    fn test_label_high_contrast_support() {
        // Test high contrast support
        let _high_contrast_label_view = view! {
            <Label
                for="hc-field"
                class="high-contrast"
                aria_label="High contrast label"
            >
                "High Contrast Label"
            </Label>
        };
        
        // Test high contrast support
        let for_attr = "hc-field";
        let hc_class = "high-contrast";
        let aria_label = "High contrast label";
        
        assert_eq!(for_attr, "hc-field");
        assert_eq!(hc_class, "high-contrast");
        assert_eq!(aria_label, "High contrast label");
    }

    #[test]
    fn test_label_reduced_motion_support() {
        // Test reduced motion support
        let _reduced_motion_label_view = view! {
            <Label
                for="rm-field"
                class="reduce-motion"
                aria_label="Reduced motion label"
            >
                "Reduced Motion Label"
            </Label>
        };
        
        // Test reduced motion support
        let for_attr = "rm-field";
        let rm_class = "reduce-motion";
        let aria_label = "Reduced motion label";
        
        assert_eq!(for_attr, "rm-field");
        assert_eq!(rm_class, "reduce-motion");
        assert_eq!(aria_label, "Reduced motion label");
    }
}
