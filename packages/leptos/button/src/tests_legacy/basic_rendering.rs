#[cfg(test)]
mod basic_rendering {
    use crate::default::{Button, ButtonVariant, ButtonSize, ButtonChildProps, BUTTON_CLASS};
    use leptos::prelude::*;
    use leptos::html::*;
    use leptos::leptos_dom::*;
    use std::sync::{Arc, Mutex};
    use web_sys::wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Helper function to render button for testing
    fn render_button_with_props(variant: ButtonVariant, size: ButtonSize, disabled: bool, children: &str) -> HtmlElement<Button> {
        view! {
            <Button variant=variant size=size disabled=Signal::from(disabled)>
                {children}
            </Button>
        }.unchecked_into()
    }

    // Helper function to create button with click handler
    fn render_button_with_click_handler(children: &str) -> (HtmlElement<Button>, Arc<Mutex<bool>>) {
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = Arc::clone(&clicked);
        
        let button = view! {
            <Button on_click=Callback::new(move |_| {
                *clicked_clone.lock().unwrap() = true;
            })>
                {children}
            </Button>
        }.unchecked_into();
        
        (button, clicked)
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_correct_element_type() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Click me");
        
        // Test that it renders as a button element
        assert_eq!(button.node_name(), "BUTTON");
        assert_eq!(button.get_attribute("type"), Some("button".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_displays_children_content() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Test Button");
        
        // Test that children content is displayed
        assert_eq!(button.text_content(), Some("Test Button".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_default_variant() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Default Button");
        
        // Test that default variant is applied
        assert!(button.class_name().contains("bg-primary"));
        assert!(button.class_name().contains("text-primary-foreground"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_destructive_variant() {
        let button = render_button_with_props(ButtonVariant::Destructive, ButtonSize::Default, false, "Destructive Button");
        
        // Test that destructive variant is applied
        assert!(button.class_name().contains("bg-destructive"));
        assert!(button.class_name().contains("text-destructive-foreground"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_outline_variant() {
        let button = render_button_with_props(ButtonVariant::Outline, ButtonSize::Default, false, "Outline Button");
        
        // Test that outline variant is applied
        assert!(button.class_name().contains("border"));
        assert!(button.class_name().contains("border-input"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_secondary_variant() {
        let button = render_button_with_props(ButtonVariant::Secondary, ButtonSize::Default, false, "Secondary Button");
        
        // Test that secondary variant is applied
        assert!(button.class_name().contains("bg-secondary"));
        assert!(button.class_name().contains("text-secondary-foreground"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_ghost_variant() {
        let button = render_button_with_props(ButtonVariant::Ghost, ButtonSize::Default, false, "Ghost Button");
        
        // Test that ghost variant is applied
        assert!(button.class_name().contains("hover:bg-accent"));
        assert!(button.class_name().contains("hover:text-accent-foreground"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_link_variant() {
        let button = render_button_with_props(ButtonVariant::Link, ButtonSize::Default, false, "Link Button");
        
        // Test that link variant is applied
        assert!(button.class_name().contains("text-primary"));
        assert!(button.class_name().contains("underline-offset-4"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_default_size() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Default Size");
        
        // Test that default size is applied
        assert!(button.class_name().contains("h-10"));
        assert!(button.class_name().contains("px-4"));
        assert!(button.class_name().contains("py-2"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_small_size() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Sm, false, "Small Size");
        
        // Test that small size is applied
        assert!(button.class_name().contains("h-9"));
        assert!(button.class_name().contains("px-3"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_large_size() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Lg, false, "Large Size");
        
        // Test that large size is applied
        assert!(button.class_name().contains("h-11"));
        assert!(button.class_name().contains("px-8"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_icon_size() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Icon, false, "Icon Size");
        
        // Test that icon size is applied
        assert!(button.class_name().contains("h-10"));
        assert!(button.class_name().contains("w-10"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_disabled_state() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, true, "Disabled Button");
        
        // Test that disabled state is applied
        assert!(button.has_attribute("disabled"));
        assert!(button.class_name().contains("disabled:opacity-50"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_without_disabled_state() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Enabled Button");
        
        // Test that disabled state is not applied
        assert!(!button.has_attribute("disabled"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_custom_class() {
        let button = view! {
            <Button class="custom-class">
                "Custom Class Button"
            </Button>
        }.unchecked_into();
        
        // Test that custom class is applied
        assert!(button.class_name().contains("custom-class"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_custom_id() {
        let button = view! {
            <Button id="custom-id">
                "Custom ID Button"
            </Button>
        }.unchecked_into();
        
        // Test that custom id is applied
        assert_eq!(button.get_attribute("id"), Some("custom-id".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_custom_style() {
        let button = view! {
            <Button style="color: red; background: blue;">
                "Custom Style Button"
            </Button>
        }.unchecked_into();
        
        // Test that custom style is applied
        assert_eq!(button.get_attribute("style"), Some("color: red; background: blue;".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_base_classes() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Base Classes");
        
        // Test that base classes are always present
        assert!(button.class_name().contains("inline-flex"));
        assert!(button.class_name().contains("items-center"));
        assert!(button.class_name().contains("justify-center"));
        assert!(button.class_name().contains("rounded-md"));
        assert!(button.class_name().contains("transition-colors"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_combined_props() {
        let button = render_button_with_props(ButtonVariant::Destructive, ButtonSize::Lg, true, "Combined Props");
        
        // Test that all props are combined correctly
        assert!(button.class_name().contains("bg-destructive"));
        assert!(button.class_name().contains("h-11"));
        assert!(button.class_name().contains("px-8"));
        assert!(button.has_attribute("disabled"));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_empty_children() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "");
        
        // Test that empty children are handled
        assert_eq!(button.text_content(), Some("".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_unicode_children() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "🚀 Button 🎉");
        
        // Test that unicode children are handled
        assert_eq!(button.text_content(), Some("🚀 Button 🎉".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_long_children() {
        let long_text = "A".repeat(1000);
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, &long_text);
        
        // Test that long children are handled
        assert_eq!(button.text_content(), Some(long_text));
    }

    #[wasm_bindgen_test]
    fn test_button_renders_with_special_characters() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Button with <>&\"'");
        
        // Test that special characters are handled
        assert_eq!(button.text_content(), Some("Button with <>&\"'".to_string()));
    }
}
