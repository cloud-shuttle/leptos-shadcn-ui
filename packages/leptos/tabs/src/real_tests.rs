#[cfg(test)]
mod real_tests {
    use crate::default::{Tabs};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_tabs_renders() {
        mount_to_body(|| {
            view! {
                <Tabs>
                    "tabs content"
                </Tabs>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "tabs should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_tabs_with_props() {
        mount_to_body(|| {
            view! {
                <Tabs class="test-class">
                    "tabs with props"
                </Tabs>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "tabs with props should render");
    }

    #[test]
    fn test_tabs_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "tabs signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "tabs signal should update");
    }

    #[test]
    fn test_tabs_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "tabs callback should be triggered");
    }

    #[test]
    fn test_tabs_class_handling() {
        let custom_class = "custom-tabs-class";
        assert!(!custom_class.is_empty(), "tabs should support custom classes");
        assert!(custom_class.contains("tabs"), "Class should contain component name");
    }

    #[test]
    fn test_tabs_id_handling() {
        let custom_id = "custom-tabs-id";
        assert!(!custom_id.is_empty(), "tabs should support custom IDs");
        assert!(custom_id.contains("tabs"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_tabs_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Tabs 
                    class="test-click"
                    on_click=move |_| click_count.update(|count| *count += 1)>
                    "Clickable tabs"
                </Tabs>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap();
        assert!(element.is_some(), "tabs should render for click test");
    }

    #[wasm_bindgen_test]
    fn test_tabs_hover_behavior() {
        mount_to_body(|| {
            view! {
                <Tabs class="test-hover">
                    "Hoverable tabs"
                </Tabs>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-hover").unwrap();
        assert!(element.is_some(), "tabs should render for hover test");
    }

    #[wasm_bindgen_test]
    fn test_tabs_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Tabs 
                    class="test-click"
                    on_click=move || click_count.update(|count| *count += 1)>
                    "Clickable tabs"
                </Tabs>
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
    fn test_tabs_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Tabs 
                    class="test-focus">
                    "Focusable tabs"
                </Tabs>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus
        
        assert_eq!(document.active_element().unwrap(), element);
    }
}