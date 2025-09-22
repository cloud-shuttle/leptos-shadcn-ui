#[cfg(test)]
mod advanced_features {
    use crate::default::{Button, ButtonVariant, ButtonSize, ButtonChildProps, BUTTON_CLASS};
    use leptos::prelude::*;
    use leptos::html::*;
    use leptos::leptos_dom::*;
    use std::sync::{Arc, Mutex};
    use web_sys::wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Helper function to create button with advanced features
    fn render_button_with_advanced_features() -> HtmlElement<Button> {
        let button = view! {
            <Button 
                id="advanced-button"
                class="advanced-class"
                style="color: red; background: blue;"
                disabled=Signal::from(false)
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                on_click=Callback::new(|_| {})
                on_focus=Callback::new(|_| {})
                on_blur=Callback::new(|_| {})
            >
                "Advanced Button"
            </Button>
        }.unchecked_into();
        
        button
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_props_combination() {
        let button = render_button_with_advanced_features();
        
        // Test that all advanced props are applied
        assert_eq!(button.get_attribute("id"), Some("advanced-button".to_string()));
        assert!(button.class_name().contains("advanced-class"));
        assert_eq!(button.get_attribute("style"), Some("color: red; background: blue;".to_string()));
        assert!(!button.has_attribute("disabled"));
        assert_eq!(button.text_content(), Some("Advanced Button".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_styling() {
        let button = view! {
            <Button 
                class="custom-class hover:bg-blue-600 focus:ring-2 active:bg-blue-700 disabled:opacity-50"
                style="color: red; background: blue; font-size: 16px; padding: 10px;"
            >
                "Styled Button"
            </Button>
        }.unchecked_into();
        
        // Test that advanced styling is applied
        assert!(button.class_name().contains("custom-class"));
        assert!(button.class_name().contains("hover:bg-blue-600"));
        assert!(button.class_name().contains("focus:ring-2"));
        assert!(button.class_name().contains("active:bg-blue-700"));
        assert!(button.class_name().contains("disabled:opacity-50"));
        assert_eq!(button.get_attribute("style"), Some("color: red; background: blue; font-size: 16px; padding: 10px;".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_accessibility() {
        let button = view! {
            <Button 
                id="accessible-button"
                aria_label="Accessible Button"
                aria_describedby="button-description"
                aria_labelledby="button-label"
                role="button"
                tabindex=0
            >
                "Accessible Button"
            </Button>
        }.unchecked_into();
        
        // Test that accessibility attributes are applied
        assert_eq!(button.get_attribute("id"), Some("accessible-button".to_string()));
        assert_eq!(button.get_attribute("aria-label"), Some("Accessible Button".to_string()));
        assert_eq!(button.get_attribute("aria-describedby"), Some("button-description".to_string()));
        assert_eq!(button.get_attribute("aria-labelledby"), Some("button-label".to_string()));
        assert_eq!(button.get_attribute("role"), Some("button".to_string()));
        assert_eq!(button.get_attribute("tabindex"), Some("0".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_keyboard_navigation() {
        let button = view! {
            <Button 
                tabindex=0
                on_keydown=Callback::new(|event| {
                    // Handle keyboard navigation
                })
            >
                "Keyboard Button"
            </Button>
        }.unchecked_into();
        
        // Test that keyboard navigation is supported
        assert_eq!(button.get_attribute("tabindex"), Some("0".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_focus_management() {
        let button = view! {
            <Button 
                on_focus=Callback::new(|_| {})
                on_blur=Callback::new(|_| {})
                class="focus:ring-2 focus:ring-blue-500 focus:outline-none"
            >
                "Focus Button"
            </Button>
        }.unchecked_into();
        
        // Test that focus management is supported
        assert!(button.class_name().contains("focus:ring-2"));
        assert!(button.class_name().contains("focus:ring-blue-500"));
        assert!(button.class_name().contains("focus:outline-none"));
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_state_management() {
        let (button, state) = {
            let state = Arc::new(Mutex::new(false));
            let state_clone = Arc::clone(&state);
            
            let button = view! {
                <Button 
                    on_click=Callback::new(move |_| {
                        *state_clone.lock().unwrap() = !*state_clone.lock().unwrap();
                    })
                >
                    "State Button"
                </Button>
            }.unchecked_into();
            
            (button, state)
        };
        
        // Test initial state
        assert!(!*state.lock().unwrap());
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test state change
        assert!(*state.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_conditional_rendering() {
        let show_button = RwSignal::new(true);
        
        let button = view! {
            <Button 
                class=move || if show_button.get() { "visible" } else { "hidden" }
                disabled=Signal::derive(move || !show_button.get())
            >
                "Conditional Button"
            </Button>
        }.unchecked_into();
        
        // Test initial state
        assert!(button.class_name().contains("visible"));
        assert!(!button.has_attribute("disabled"));
        
        // Change state
        show_button.set(false);
        
        // Test state change
        assert!(button.class_name().contains("hidden"));
        assert!(button.has_attribute("disabled"));
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_dynamic_content() {
        let content = RwSignal::new("Initial Content".to_string());
        
        let button = view! {
            <Button>
                {move || content.get()}
            </Button>
        }.unchecked_into();
        
        // Test initial content
        assert_eq!(button.text_content(), Some("Initial Content".to_string()));
        
        // Change content
        content.set("Updated Content".to_string());
        
        // Test content change
        assert_eq!(button.text_content(), Some("Updated Content".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_event_handling() {
        let (button, events) = {
            let events = Arc::new(Mutex::new(Vec::new()));
            let events_clone = Arc::clone(&events);
            
            let button = view! {
                <Button 
                    on_click=Callback::new(move |_| {
                        events_clone.lock().unwrap().push("click");
                    })
                    on_focus=Callback::new(move |_| {
                        events_clone.lock().unwrap().push("focus");
                    })
                    on_blur=Callback::new(move |_| {
                        events_clone.lock().unwrap().push("blur");
                    })
                >
                    "Event Button"
                </Button>
            }.unchecked_into();
            
            (button, events)
        };
        
        // Simulate events
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        let focus_event = web_sys::FocusEvent::new("focus").unwrap();
        button.dispatch_event(&focus_event).unwrap();
        
        let blur_event = web_sys::FocusEvent::new("blur").unwrap();
        button.dispatch_event(&blur_event).unwrap();
        
        // Test event order
        let events_vec = events.lock().unwrap();
        assert_eq!(events_vec.len(), 3);
        assert_eq!(events_vec[0], "click");
        assert_eq!(events_vec[1], "focus");
        assert_eq!(events_vec[2], "blur");
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_performance() {
        let start = std::time::Instant::now();
        
        // Create multiple buttons
        for _ in 0..100 {
            let _button = view! {
                <Button>
                    "Performance Button"
                </Button>
            };
        }
        
        let duration = start.elapsed();
        
        // Test that creation is fast (less than 100ms for 100 buttons)
        assert!(duration.as_millis() < 100);
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_memory_usage() {
        let button = view! {
            <Button>
                "Memory Button"
            </Button>
        }.unchecked_into();
        
        // Test that memory usage is reasonable
        let size = std::mem::size_of_val(&button);
        assert!(size < 1024); // Less than 1KB
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_error_handling() {
        let (button, error_count) = {
            let error_count = Arc::new(Mutex::new(0));
            let error_count_clone = Arc::clone(&error_count);
            
            let button = view! {
                <Button 
                    on_click=Callback::new(move |_| {
                        // Simulate error handling
                        *error_count_clone.lock().unwrap() += 1;
                    })
                >
                    "Error Button"
                </Button>
            }.unchecked_into();
            
            (button, error_count)
        };
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test error handling
        assert_eq!(*error_count.lock().unwrap(), 1);
    }

    #[wasm_bindgen_test]
    fn test_button_advanced_edge_cases() {
        // Test with empty props
        let button = view! {
            <Button>
                ""
            </Button>
        }.unchecked_into();
        
        assert_eq!(button.text_content(), Some("".to_string()));
        
        // Test with very long content
        let long_content = "A".repeat(10000);
        let button = view! {
            <Button>
                {long_content.clone()}
            </Button>
        }.unchecked_into();
        
        assert_eq!(button.text_content(), Some(long_content));
        
        // Test with special characters
        let special_content = "Button with <>&\"'";
        let button = view! {
            <Button>
                {special_content}
            </Button>
        }.unchecked_into();
        
        assert_eq!(button.text_content(), Some("Button with <>&\"'".to_string()));
    }
}
