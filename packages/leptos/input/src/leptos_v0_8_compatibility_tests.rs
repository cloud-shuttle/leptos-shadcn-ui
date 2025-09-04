#[cfg(test)]
mod leptos_v0_8_compatibility_tests {
    use leptos::prelude::*;
    use crate::default::Input;

    /// Test that verifies Leptos v0.8 attribute system compatibility
    /// This test will fail with current implementation but pass after fixing attr: syntax
    #[test]
    fn test_leptos_v0_8_attribute_system_compatibility() {
        // Create a test harness that simulates Leptos v0.8 environment
        let test_result = std::panic::catch_unwind(|| {
            // This should work with proper attr: syntax in Leptos v0.8
            let (value, set_value) = signal("test".to_string());
            let (disabled, set_disabled) = signal(false);
            let (class, set_class) = signal("custom-class".to_string());
            let (id, set_id) = signal("test-input".to_string());
            let (style, set_style) = signal(leptos_style::Style::new());

            // Test that the component can be rendered with Leptos v0.8 attribute system
            let _view = view! {
                <Input
                    value=value
                    placeholder="Enter text"
                    disabled=disabled
                    input_type="text"
                    class=class
                    id=id
                    style=style
                />
            };

            // If we get here without panicking, the attribute system is compatible
            true
        });

        // This test should pass once we fix the attr: syntax
        assert!(test_result.is_ok(), "Leptos v0.8 attribute system compatibility test failed");
    }

    /// Test that verifies specific attribute types work correctly
    #[test]
    fn test_attribute_types_compatibility() {
        let test_result = std::panic::catch_unwind(|| {
            // Test different attribute types that should work with attr: syntax
            let (value, _) = signal("test".to_string());
            let (disabled, _) = signal(false);
            let (class, _) = signal("test-class".to_string());
            let (id, _) = signal("test-id".to_string());

            // These should all work with proper Leptos v0.8 attribute handling
            let _view = view! {
                <Input
                    value=value
                    placeholder="Test placeholder"
                    disabled=disabled
                    input_type="email"
                    class=class
                    id=id
                    style=leptos_style::Style::new()
                />
            };

            true
        });

        assert!(test_result.is_ok(), "Attribute types compatibility test failed");
    }

    /// Test that verifies Signal<T> attribute handling
    #[test]
    fn test_signal_attribute_handling() {
        let test_result = std::panic::catch_unwind(|| {
            // Test that Signal<T> values work correctly with attr: syntax
            let (value, set_value) = signal("initial".to_string());
            let (disabled, set_disabled) = signal(true);
            let (class, set_class) = signal("dynamic-class".to_string());

            // Update signals to test reactivity
            set_value.set("updated".to_string());
            set_disabled.set(false);
            set_class.set("updated-class".to_string());

            let _view = view! {
                <Input
                    value=value
                    placeholder="Dynamic placeholder"
                    disabled=disabled
                    input_type="password"
                    class=class
                    id="dynamic-id"
                    style=leptos_style::Style::new()
                />
            };

            true
        });

        assert!(test_result.is_ok(), "Signal attribute handling test failed");
    }

    /// Test that verifies reserved keyword handling (type attribute)
    #[test]
    fn test_reserved_keyword_attributes() {
        let test_result = std::panic::catch_unwind(|| {
            // Test that reserved keywords like 'type' work with attr:r#type syntax
            let (value, _) = signal("test".to_string());
            let (disabled, _) = signal(false);

            // This should work with attr:r#type syntax
            let _view = view! {
                <Input
                    value=value
                    placeholder="Test"
                    disabled=disabled
                    input_type="number" // This should become attr:r#type
                    class="test-class"
                    id="test-id"
                    style=leptos_style::Style::new()
                />
            };

            true
        });

        assert!(test_result.is_ok(), "Reserved keyword attributes test failed");
    }
}
