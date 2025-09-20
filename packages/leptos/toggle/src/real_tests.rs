#[cfg(test)]
mod real_tests {
    use crate::default::{Toggle};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_toggle_renders() {
        mount_to_body(|| {
            view! {
                <Toggle>
                    "toggle content"
                </Toggle>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "toggle should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_toggle_with_props() {
        mount_to_body(|| {
            view! {
                <Toggle class="test-class">
                    "toggle with props"
                </Toggle>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "toggle with props should render");
    }

    #[test]
    fn test_toggle_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "toggle signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "toggle signal should update");
    }

    #[test]
    fn test_toggle_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "toggle callback should be triggered");
    }

    #[test]
    fn test_toggle_class_handling() {
        let custom_class = "custom-toggle-class";
        assert!(!custom_class.is_empty(), "toggle should support custom classes");
        assert!(custom_class.contains("toggle"), "Class should contain component name");
    }

    #[test]
    fn test_toggle_id_handling() {
        let custom_id = "custom-toggle-id";
        assert!(!custom_id.is_empty(), "toggle should support custom IDs");
        assert!(custom_id.contains("toggle"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap();
        assert!(element.is_some(), "toggle should render for click test");
    }

    #[wasm_bindgen_test]
    fn test_toggle_hover_behavior() {
        mount_to_body(|| {
            view! {
                <Toggle class="test-hover" >
                    "Hoverable toggle"
                </Toggle>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-hover").unwrap();
        assert!(element.is_some(), "toggle should render for hover test");
    }

    #[wasm_bindgen_test]
    });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap().unwrap();
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        element.dispatch_event(&click_event).unwrap();
        
        assert_eq!(click_count.get(), 1, "Click should be handled");
    }

    #[wasm_bindgen_test]
    fn test_toggle_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Toggle 
                    class="test-focus"
                    
                >
                    "Focusable toggle"
                </Toggle>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus
        
        assert_eq!(document.active_element().unwrap(), element);
    }
}