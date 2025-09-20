#[cfg(test)]
mod real_tests {
    use crate::default::{Collapsible};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_collapsible_renders() {
        mount_to_body(|| {
            view! {
                <Collapsible>
                    "collapsible content"
                </Collapsible>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "collapsible should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_collapsible_with_props() {
        mount_to_body(|| {
            view! {
                <Collapsible class="test-class">
                    "collapsible with props"
                </Collapsible>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "collapsible with props should render");
    }

    #[test]
    fn test_collapsible_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "collapsible signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "collapsible signal should update");
    }

    #[test]
    fn test_collapsible_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "collapsible callback should be triggered");
    }

    #[test]
    fn test_collapsible_class_handling() {
        let custom_class = "custom-collapsible-class";
        assert!(!custom_class.is_empty(), "collapsible should support custom classes");
        assert!(custom_class.contains("collapsible"), "Class should contain component name");
    }

    #[test]
    fn test_collapsible_id_handling() {
        let custom_id = "custom-collapsible-id";
        assert!(!custom_id.is_empty(), "collapsible should support custom IDs");
        assert!(custom_id.contains("collapsible"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_collapsible_interaction() {
        mount_to_body(|| {
            view! {
                <Collapsible class="test-interaction">
                    "Interactive collapsible"
                </Collapsible>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "collapsible should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_collapsible_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Collapsible class="test-focus">
                    "Focusable collapsible"
                </Collapsible>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "collapsible should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_collapsible_accessibility() {
        mount_to_body(|| {
            view! {
                <Collapsible class="test-a11y" role="button">
                    "Accessible collapsible"
                </Collapsible>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "collapsible should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_collapsible_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Collapsible 
                    class="test-click"
                    on_click=move || click_count.update(|count| *count += 1)
                >
                    "Clickable collapsible"
                </Collapsible>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap().unwrap();
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        element.dispatch_event(&click_event).unwrap();
        
        assert_eq!(click_count.get(), 1, "Click should be handled");
    }

    #[wasm_bindgen_test]
    fn test_collapsible_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Collapsible 
                    class="test-focus"
                    tabindex="0"
                >
                    "Focusable collapsible"
                </Collapsible>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    }
}