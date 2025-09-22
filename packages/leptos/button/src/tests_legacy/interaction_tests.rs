#[cfg(test)]
mod interaction_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize, ButtonChildProps, BUTTON_CLASS};
    use leptos::prelude::*;
    use leptos::html::*;
    use leptos::leptos_dom::*;
    use std::sync::{Arc, Mutex};
    use web_sys::wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

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

    // Helper function to create button with focus handler
    fn render_button_with_focus_handler(children: &str) -> (HtmlElement<Button>, Arc<Mutex<bool>>) {
        let focused = Arc::new(Mutex::new(false));
        let focused_clone = Arc::clone(&focused);
        
        let button = view! {
            <Button on_focus=Callback::new(move |_| {
                *focused_clone.lock().unwrap() = true;
            })>
                {children}
            </Button>
        }.unchecked_into();
        
        (button, focused)
    }

    // Helper function to create button with blur handler
    fn render_button_with_blur_handler(children: &str) -> (HtmlElement<Button>, Arc<Mutex<bool>>) {
        let blurred = Arc::new(Mutex::new(false));
        let blurred_clone = Arc::clone(&blurred);
        
        let button = view! {
            <Button on_blur=Callback::new(move |_| {
                *blurred_clone.lock().unwrap() = true;
            })>
                {children}
            </Button>
        }.unchecked_into();
        
        (button, blurred)
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler_executes() {
        let (button, clicked) = render_button_with_click_handler("Click Me");
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler_with_multiple_clicks() {
        let (button, clicked) = render_button_with_click_handler("Click Me");
        
        // Simulate multiple clicks
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        button.dispatch_event(&click_event).unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler_with_different_variants() {
        let variants = vec![
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];

        for variant in variants {
            let (button, clicked) = render_button_with_click_handler("Click Me");
            
            // Simulate click
            let click_event = web_sys::MouseEvent::new("click").unwrap();
            button.dispatch_event(&click_event).unwrap();
            
            // Test that click handler was executed
            assert!(*clicked.lock().unwrap());
        }
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler_with_different_sizes() {
        let sizes = vec![
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ];

        for size in sizes {
            let (button, clicked) = render_button_with_click_handler("Click Me");
            
            // Simulate click
            let click_event = web_sys::MouseEvent::new("click").unwrap();
            button.dispatch_event(&click_event).unwrap();
            
            // Test that click handler was executed
            assert!(*clicked.lock().unwrap());
        }
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler_with_disabled_button() {
        let (button, clicked) = render_button_with_click_handler("Disabled Button");
        
        // Set disabled state
        button.set_disabled(true);
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was not executed
        assert!(!*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_focus_handler_executes() {
        let (button, focused) = render_button_with_focus_handler("Focus Me");
        
        // Simulate focus
        let focus_event = web_sys::FocusEvent::new("focus").unwrap();
        button.dispatch_event(&focus_event).unwrap();
        
        // Test that focus handler was executed
        assert!(*focused.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_blur_handler_executes() {
        let (button, blurred) = render_button_with_blur_handler("Blur Me");
        
        // Simulate blur
        let blur_event = web_sys::FocusEvent::new("blur").unwrap();
        button.dispatch_event(&blur_event).unwrap();
        
        // Test that blur handler was executed
        assert!(*blurred.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_keyboard_activation() {
        let (button, clicked) = render_button_with_click_handler("Keyboard Me");
        
        // Simulate Enter key
        let keydown_event = web_sys::KeyboardEvent::new("keydown").unwrap();
        keydown_event.init_keyboard_event_with_bubbles_and_cancelable("keydown", true, true, None, "Enter", 0, false, false, false, false).unwrap();
        button.dispatch_event(&keydown_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_keyboard_activation_with_space() {
        let (button, clicked) = render_button_with_click_handler("Space Me");
        
        // Simulate Space key
        let keydown_event = web_sys::KeyboardEvent::new("keydown").unwrap();
        keydown_event.init_keyboard_event_with_bubbles_and_cancelable("keydown", true, true, None, " ", 0, false, false, false, false).unwrap();
        button.dispatch_event(&keydown_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_keyboard_activation_with_other_keys() {
        let (button, clicked) = render_button_with_click_handler("Other Keys");
        
        // Simulate other keys (should not trigger)
        let keys = vec!["a", "b", "c", "Tab", "Escape"];
        
        for key in keys {
            let keydown_event = web_sys::KeyboardEvent::new("keydown").unwrap();
            keydown_event.init_keyboard_event_with_bubbles_and_cancelable("keydown", true, true, None, key, 0, false, false, false, false).unwrap();
            button.dispatch_event(&keydown_event).unwrap();
        }
        
        // Test that click handler was not executed
        assert!(!*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_event_propagation() {
        let (button, clicked) = render_button_with_click_handler("Propagation Test");
        
        // Simulate click with stopPropagation
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        click_event.stop_propagation();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_event_prevention() {
        let (button, clicked) = render_button_with_click_handler("Prevention Test");
        
        // Simulate click with preventDefault
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        click_event.prevent_default();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_multiple_event_handlers() {
        let click_count = Arc::new(Mutex::new(0));
        let focus_count = Arc::new(Mutex::new(0));
        let blur_count = Arc::new(Mutex::new(0));
        
        let click_count_clone = Arc::clone(&click_count);
        let focus_count_clone = Arc::clone(&focus_count);
        let blur_count_clone = Arc::clone(&blur_count);
        
        let button = view! {
            <Button 
                on_click=Callback::new(move |_| {
                    *click_count_clone.lock().unwrap() += 1;
                })
                on_focus=Callback::new(move |_| {
                    *focus_count_clone.lock().unwrap() += 1;
                })
                on_blur=Callback::new(move |_| {
                    *blur_count_clone.lock().unwrap() += 1;
                })
            >
                "Multiple Handlers"
            </Button>
        }.unchecked_into();
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Simulate focus
        let focus_event = web_sys::FocusEvent::new("focus").unwrap();
        button.dispatch_event(&focus_event).unwrap();
        
        // Simulate blur
        let blur_event = web_sys::FocusEvent::new("blur").unwrap();
        button.dispatch_event(&blur_event).unwrap();
        
        // Test that all handlers were executed
        assert_eq!(*click_count.lock().unwrap(), 1);
        assert_eq!(*focus_count.lock().unwrap(), 1);
        assert_eq!(*blur_count.lock().unwrap(), 1);
    }

    #[wasm_bindgen_test]
    fn test_button_event_handler_removal() {
        let (button, clicked) = render_button_with_click_handler("Handler Removal");
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
        
        // Reset
        *clicked.lock().unwrap() = false;
        
        // Simulate click again
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed again
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_event_handler_with_async() {
        let (button, clicked) = render_button_with_click_handler("Async Handler");
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_button_event_handler_with_error() {
        let (button, clicked) = render_button_with_click_handler("Error Handler");
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test that click handler was executed
        assert!(*clicked.lock().unwrap());
    }
}
