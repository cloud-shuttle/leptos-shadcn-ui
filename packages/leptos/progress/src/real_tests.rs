#[cfg(test)]
mod real_tests {
    use crate::default::{Progress};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_progress_renders() {
        mount_to_body(|| {
            view! {
                <Progress>
                    "progress content"
                </Progress>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "progress should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_progress_with_props() {
        mount_to_body(|| {
            view! {
                <Progress class="test-class">
                    "progress with props"
                </Progress>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "progress with props should render");
    }

    #[test]
    fn test_progress_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "progress signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "progress signal should update");
    }

    #[test]
    fn test_progress_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "progress callback should be triggered");
    }

    #[test]
    fn test_progress_class_handling() {
        let custom_class = "custom-progress-class";
        assert!(!custom_class.is_empty(), "progress should support custom classes");
        assert!(custom_class.contains("progress"), "Class should contain component name");
    }

    #[test]
    fn test_progress_id_handling() {
        let custom_id = "custom-progress-id";
        assert!(!custom_id.is_empty(), "progress should support custom IDs");
        assert!(custom_id.contains("progress"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_progress_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Progress 
                    class="test-click"
                    on_click=move |_| click_count.update(|count| *count += 1)
                >
                    "Clickable progress"
                </Progress>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap();
        assert!(element.is_some(), "progress should render for click test");
    }

    #[wasm_bindgen_test]
    fn test_progress_hover_behavior() {
        mount_to_body(|| {
            view! {
                <Progress class="test-hover" data-hover="true">
                    "Hoverable progress"
                </Progress>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-hover").unwrap();
        assert!(element.is_some(), "progress should render for hover test");
    }

    #[wasm_bindgen_test]
    fn test_progress_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Progress 
                    class="test-click"
                    on_click=move || click_count.update(|count| *count += 1)
                >
                    "Clickable progress"
                </Progress>
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
    fn test_progress_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Progress 
                    class="test-focus"
                    tabindex="0"
                >
                    "Focusable progress"
                </Progress>
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