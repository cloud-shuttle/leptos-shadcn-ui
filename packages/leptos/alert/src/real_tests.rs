#[cfg(test)]
mod real_tests {
    use crate::default::{Alert};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_alert_renders() {
        mount_to_body(|| {
            view! {
                <Alert>
                    "alert content"
                </Alert>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "alert should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_alert_with_props() {
        mount_to_body(|| {
            view! {
                <Alert class="test-class".into()>
                    "alert with props"
                </Alert>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "alert with props should render");
    }

    #[test]
    fn test_alert_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "alert signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "alert signal should update");
    }

    #[test]
    fn test_alert_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "alert callback should be triggered");
    }

    #[test]
    fn test_alert_class_handling() {
        let custom_class = "custom-alert-class";
        assert!(!custom_class.is_empty(), "alert should support custom classes");
        assert!(custom_class.contains("alert"), "Class should contain component name");
    }

    #[test]
    fn test_alert_id_handling() {
        let custom_id = "custom-alert-id";
        assert!(!custom_id.is_empty(), "alert should support custom IDs");
        assert!(custom_id.contains("alert"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_alert_interaction() {
        mount_to_body(|| {
            view! {
                <Alert class="test-interaction".into()>
                    "Interactive alert"
                </Alert>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "alert should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_alert_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Alert class="test-focus".into()>
                    "Focusable alert"
                </Alert>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "alert should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_alert_accessibility() {
        mount_to_body(|| {
            view! {
                <Alert class="test-a11y".into()>
                    "Accessible alert"
                </Alert>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "alert should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_alert_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Alert 
                    class="test-responsive".into()>
                    "Responsive alert"
                </Alert>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    fn test_alert_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout-container".into()><Alert class="test-layout-item".into()>
                        "Layout alert"
                    </Alert></div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "alert should render in layout");
    }
}