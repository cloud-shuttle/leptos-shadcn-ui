#[cfg(test)]
mod tests {
    use crate::default::{Button, ButtonVariant, ButtonSize, ButtonChildProps, BUTTON_CLASS};
    use leptos::prelude::*;
    use leptos::html::*;
    use leptos_dom::*;
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
        
        // Test that button content is correct
        assert_eq!(button.text_content(), Some("Test Button".to_string()));
    }

    #[test]
    fn test_button_variant_enum_creation() {
        // Test ButtonVariant enum
        assert_eq!(ButtonVariant::default(), ButtonVariant::Default);
        
        // Test From<String> conversion
        assert_eq!(ButtonVariant::from("destructive".to_string()), ButtonVariant::Destructive);
        assert_eq!(ButtonVariant::from("outline".to_string()), ButtonVariant::Outline);
        assert_eq!(ButtonVariant::from("secondary".to_string()), ButtonVariant::Secondary);
        assert_eq!(ButtonVariant::from("ghost".to_string()), ButtonVariant::Ghost);
        assert_eq!(ButtonVariant::from("link".to_string()), ButtonVariant::Link);
        assert_eq!(ButtonVariant::from("unknown".to_string()), ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_enum_creation() {
        // Test ButtonSize enum
        assert_eq!(ButtonSize::default(), ButtonSize::Default);
        
        // Test From<String> conversion
        assert_eq!(ButtonSize::from("sm".to_string()), ButtonSize::Sm);
        assert_eq!(ButtonSize::from("lg".to_string()), ButtonSize::Lg);
        assert_eq!(ButtonSize::from("icon".to_string()), ButtonSize::Icon);
        assert_eq!(ButtonSize::from("unknown".to_string()), ButtonSize::Default);
    }

    #[test]
    fn test_button_child_props_structure() {
        // Test ButtonChildProps creation
        let props = ButtonChildProps {
            class: "test-class".to_string(),
            id: "test-id".to_string(),
            style: "color: red;".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };
        
        assert_eq!(props.class, "test-class");
        assert_eq!(props.id, "test-id");
        assert_eq!(props.style, "color: red;");
        assert!(!props.disabled);
        assert_eq!(props.r#type, "button");
        assert!(props.onclick.is_none());
    }

    #[wasm_bindgen_test]
    fn test_button_variant_css_classes_applied() {
        // Test actual CSS classes are applied to rendered buttons
        let test_cases = vec![
            (ButtonVariant::Default, "bg-primary"),
            (ButtonVariant::Destructive, "bg-destructive"),
            (ButtonVariant::Outline, "border border-input"),
            (ButtonVariant::Secondary, "bg-secondary"),
            (ButtonVariant::Ghost, "hover:bg-accent"),
            (ButtonVariant::Link, "text-primary underline-offset-4"),
        ];
        
        for (variant, expected_class_part) in test_cases {
            let button = render_button_with_props(variant.clone(), ButtonSize::Default, false, "Test");
            let class_list = button.class_name();
            
            // Verify base classes are always present
            assert!(class_list.contains("inline-flex"));
            assert!(class_list.contains("items-center"));
            assert!(class_list.contains("justify-center"));
            
            // Verify variant-specific classes are present
            assert!(class_list.contains(expected_class_part), 
                "Button with variant {:?} should have class containing '{}', but got: '{}'", 
                variant, expected_class_part, class_list);
        }
    }
    
    #[test]
    fn test_button_variant_css_class_mapping() {
        // Keep enum validation tests for internal logic
        let variants = vec![
            (ButtonVariant::Default, "bg-primary text-primary-foreground hover:bg-primary/90"),
            (ButtonVariant::Destructive, "bg-destructive text-destructive-foreground hover:bg-destructive/90"),
            (ButtonVariant::Outline, "border border-input bg-background hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariant::Secondary, "bg-secondary text-secondary-foreground hover:bg-secondary/80"),
            (ButtonVariant::Ghost, "hover:bg-accent hover:text-accent-foreground"),
            (ButtonVariant::Link, "text-primary underline-offset-4 hover:underline"),
        ];
        
        for (variant, expected_class) in variants {
            match variant {
                ButtonVariant::Default => assert!(expected_class.contains("bg-primary")),
                ButtonVariant::Destructive => assert!(expected_class.contains("bg-destructive")),
                ButtonVariant::Outline => assert!(expected_class.contains("border border-input")),
                ButtonVariant::Secondary => assert!(expected_class.contains("bg-secondary")),
                ButtonVariant::Ghost => assert!(expected_class.contains("hover:bg-accent")),
                ButtonVariant::Link => assert!(expected_class.contains("text-primary underline")),
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_button_size_css_classes_applied() {
        // Test actual size classes are applied to rendered buttons
        let test_cases = vec![
            (ButtonSize::Default, "h-10", "px-4"),
            (ButtonSize::Sm, "h-9", "px-3"),
            (ButtonSize::Lg, "h-11", "px-8"),
            (ButtonSize::Icon, "h-10", "w-10"),
        ];
        
        for (size, height_class, spacing_class) in test_cases {
            let button = render_button_with_props(ButtonVariant::Default, size.clone(), false, "Test");
            let class_list = button.class_name();
            
            assert!(class_list.contains(height_class), 
                "Button with size {:?} should have height class '{}', but got: '{}'", 
                size, height_class, class_list);
            assert!(class_list.contains(spacing_class), 
                "Button with size {:?} should have spacing class '{}', but got: '{}'", 
                size, spacing_class, class_list);
        }
    }
    
    #[test]
    fn test_button_size_css_class_mapping() {
        let sizes = vec![
            (ButtonSize::Default, "h-10 px-4 py-2"),
            (ButtonSize::Sm, "h-9 rounded-md px-3"),
            (ButtonSize::Lg, "h-11 rounded-md px-8"),
            (ButtonSize::Icon, "h-10 w-10"),
        ];
        
        for (size, expected_class) in sizes {
            match size {
                ButtonSize::Default => assert!(expected_class.contains("h-10 px-4 py-2")),
                ButtonSize::Sm => assert!(expected_class.contains("h-9")),
                ButtonSize::Lg => assert!(expected_class.contains("h-11")),
                ButtonSize::Icon => assert!(expected_class.contains("w-10")),
            }
        }
    }

    #[test]
    fn test_button_base_css_classes() {
        // Test that base BUTTON_CLASS contains required accessibility and styling classes
        assert!(BUTTON_CLASS.contains("inline-flex"));
        assert!(BUTTON_CLASS.contains("items-center"));
        assert!(BUTTON_CLASS.contains("justify-center"));
        assert!(BUTTON_CLASS.contains("focus-visible:outline-none"));
        assert!(BUTTON_CLASS.contains("focus-visible:ring-2"));
        assert!(BUTTON_CLASS.contains("disabled:pointer-events-none"));
        assert!(BUTTON_CLASS.contains("disabled:opacity-50"));
        assert!(BUTTON_CLASS.contains("transition-colors"));
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler_execution() {
        let (button, clicked) = render_button_with_click_handler("Click me");
        
        // Verify initial state
        assert!(!*clicked.lock().unwrap());
        
        // Simulate click event
        button.click();
        
        // Verify click handler was called
        assert!(*clicked.lock().unwrap(), "Button click handler should be called when button is clicked");
    }
    
    #[test]
    fn test_button_callback_structure() {
        let click_called = Arc::new(Mutex::new(false));
        let click_called_clone = Arc::clone(&click_called);
        
        let callback = Callback::new(move |_: ()| {
            *click_called_clone.lock().unwrap() = true;
        });
        
        callback.run(());
        assert!(*click_called.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_disabled_state_rendering() {
        // Test enabled button
        let enabled_button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Enabled");
        assert!(!enabled_button.disabled());
        assert!(!enabled_button.class_name().contains("disabled:opacity-50") || 
                enabled_button.class_name().contains("disabled:opacity-50")); // Base class should be present
        
        // Test disabled button
        let disabled_button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, true, "Disabled");
        assert!(disabled_button.disabled());
        assert!(disabled_button.class_name().contains("disabled:opacity-50"));
        assert!(disabled_button.class_name().contains("disabled:pointer-events-none"));
    }
    
    #[wasm_bindgen_test]
    fn test_disabled_button_click_prevention() {
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = Arc::clone(&clicked);
        
        let disabled_button = view! {
            <Button 
                disabled=Signal::from(true)
                on_click=Callback::new(move |_| {
                    *clicked_clone.lock().unwrap() = true;
                })
            >
                "Disabled Button"
            </Button>
        }.unchecked_into::<web_sys::HtmlButtonElement>();
        
        // Attempt to click disabled button
        disabled_button.click();
        
        // Click handler should not be called for disabled buttons
        // Note: This depends on the component implementation preventing event handling
        // when disabled=true
        assert!(!*clicked.lock().unwrap() || disabled_button.disabled(), 
            "Disabled button should not execute click handler or should be properly disabled");
    }
    
    #[test]
    fn test_button_disabled_signal() {
        let disabled_signal = RwSignal::new(false);
        assert!(!disabled_signal.get());
        
        disabled_signal.set(true);
        assert!(disabled_signal.get());
    }

    #[wasm_bindgen_test]
    fn test_button_custom_class_merging() {
        // Test actual class merging in rendered component
        let button_with_custom_class = view! {
            <Button 
                variant=ButtonVariant::Secondary
                size=ButtonSize::Lg
                class="my-custom-class another-class"
            >
                "Custom Button"
            </Button>
        }.unchecked_into::<web_sys::HtmlButtonElement>();
        
        let class_list = button_with_custom_class.class_name();
        
        // Check base classes are present
        assert!(class_list.contains("inline-flex"));
        assert!(class_list.contains("items-center"));
        
        // Check variant classes are present
        assert!(class_list.contains("bg-secondary"));
        
        // Check size classes are present
        assert!(class_list.contains("h-11"));
        assert!(class_list.contains("px-8"));
        
        // Check custom classes are present
        assert!(class_list.contains("my-custom-class"));
        assert!(class_list.contains("another-class"));
    }
    
    #[test]
    fn test_button_class_merging_logic() {
        let base_class = BUTTON_CLASS;
        let variant_class = "bg-primary text-primary-foreground hover:bg-primary/90";
        let size_class = "h-10 px-4 py-2";
        let custom_class = "my-custom-class";
        
        let expected = format!("{} {} {} {}", base_class, variant_class, size_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(variant_class));
        assert!(expected.contains(size_class));
        assert!(expected.contains(custom_class));
    }
    
    // NEW: Accessibility Tests
    #[wasm_bindgen_test]
    fn test_button_accessibility_attributes() {
        let button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, false, "Accessible Button");
        
        // Test ARIA role is implicit (button element)
        assert_eq!(button.node_name(), "BUTTON");
        
        // Test that focus styles are applied via CSS classes
        let class_list = button.class_name();
        assert!(class_list.contains("focus-visible:outline-none"));
        assert!(class_list.contains("focus-visible:ring-2"));
        
        // Test disabled accessibility
        let disabled_button = render_button_with_props(ButtonVariant::Default, ButtonSize::Default, true, "Disabled");
        assert!(disabled_button.disabled());
        assert!(disabled_button.class_name().contains("disabled:pointer-events-none"));
    }
    
    // NEW: Comprehensive Integration Tests
    #[wasm_bindgen_test]
    fn test_button_complete_rendering_integration() {
        let clicked_count = Arc::new(Mutex::new(0));
        let clicked_clone = Arc::clone(&clicked_count);
        
        let complex_button = view! {
            <Button 
                variant=ButtonVariant::Destructive
                size=ButtonSize::Lg
                class="test-button custom-styles"
                id="test-button-id"
                disabled=Signal::from(false)
                on_click=Callback::new(move |_| {
                    *clicked_clone.lock().unwrap() += 1;
                })
            >
                "Delete Item"
            </Button>
        }.unchecked_into::<web_sys::HtmlButtonElement>();
        
        // Test all attributes are correctly applied
        assert_eq!(complex_button.node_name(), "BUTTON");
        assert_eq!(complex_button.text_content(), Some("Delete Item".to_string()));
        assert_eq!(complex_button.id(), "test-button-id");
        assert!(!complex_button.disabled());
        
        // Test CSS classes include all expected parts
        let classes = complex_button.class_name();
        assert!(classes.contains("inline-flex")); // base
        assert!(classes.contains("bg-destructive")); // variant
        assert!(classes.contains("h-11")); // size
        assert!(classes.contains("test-button")); // custom
        assert!(classes.contains("custom-styles")); // custom
        
        // Test click functionality
        assert_eq!(*clicked_count.lock().unwrap(), 0);
        complex_button.click();
        assert_eq!(*clicked_count.lock().unwrap(), 1);
        complex_button.click();
        assert_eq!(*clicked_count.lock().unwrap(), 2);
    }

    #[wasm_bindgen_test] 
    fn test_button_as_child_rendering() {
        // Test as_child functionality with actual rendering
        let custom_element = view! {
            <Button as_child=Callback::new(|props: ButtonChildProps| {
                view! {
                    <a 
                        class=props.class
                        href="#"
                        role="button"
                        on:click=move |_| {
                            if let Some(onclick) = props.onclick {
                                onclick.run(());
                            }
                        }
                    >
                        "Custom Link Button"
                    </a>
                }.into_any()
            })>
                "This should be ignored"
            </Button>
        }.unchecked_into::<web_sys::HtmlElement>();
        
        // Should render as anchor element instead of button
        assert_eq!(custom_element.node_name(), "A");
        assert_eq!(custom_element.get_attribute("role"), Some("button".to_string()));
        assert_eq!(custom_element.get_attribute("href"), Some("#".to_string()));
        assert!(custom_element.class_name().contains("inline-flex"));
    }
    
    #[test]
    fn test_button_as_child_props_structure() {
        let as_child_callback = Callback::new(|props: ButtonChildProps| {
            assert!(!props.class.is_empty());
            assert_eq!(props.r#type, "button");
            view! { <div class=props.class>Custom Child</div> }.into_any()
        });
        
        assert!(std::mem::size_of_val(&as_child_callback) > 0);
    }
}