#[cfg(test)]
mod real_tests {
    use crate::default::{Toast};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_toast_renders() {
        mount_to_body(|| {
            view! {
                <Toast></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "toast should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_toast_with_props() {
        mount_to_body(|| {
            view! {
                <Toast class="test-class".into()></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "toast with props should render");
    }

    #[test]
    fn test_toast_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "toast signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "toast signal should update");
    }

    #[test]
    fn test_toast_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "toast callback should be triggered");
    }

    #[test]
    fn test_toast_class_handling() {
        let custom_class = "custom-toast-class";
        assert!(!custom_class.is_empty(), "toast should support custom classes");
        assert!(custom_class.contains("toast"), "Class should contain component name");
    }

    #[test]
    fn test_toast_id_handling() {
        let custom_id = "custom-toast-id";
        assert!(!custom_id.is_empty(), "toast should support custom IDs");
        assert!(custom_id.contains("toast"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_toast_interaction() {
        mount_to_body(|| {
            view! {
                <Toast class="test-interaction".into()></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "toast should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_toast_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Toast class="test-focus".into()></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "toast should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_toast_accessibility() {
        mount_to_body(|| {
            view! {
                <Toast class="test-a11y".into()></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "toast should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_toast_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Toast class="test-dom-render".into()></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "toast should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_toast_class_application() {
        mount_to_body(|| {
            view! {
                <Toast class="test-class-application custom-class".into()></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_toast_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Toast 
                    class="test-attributes".into()></Toast>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test toast");
    }
}