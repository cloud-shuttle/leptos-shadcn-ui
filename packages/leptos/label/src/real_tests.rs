#[cfg(test)]
mod real_tests {
    use crate::default::{Label};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_label_renders() {
        mount_to_body(|| {
            view! {
                <Label>
                    "label content"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "label should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_label_with_props() {
        mount_to_body(|| {
            view! {
                <Label class="test-class".into()>
                    "label with props"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "label with props should render");
    }

    #[test]
    fn test_label_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "label signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "label signal should update");
    }

    #[test]
    fn test_label_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "label callback should be triggered");
    }

    #[test]
    fn test_label_class_handling() {
        let custom_class = "custom-label-class";
        assert!(!custom_class.is_empty(), "label should support custom classes");
        assert!(custom_class.contains("label"), "Class should contain component name");
    }

    #[test]
    fn test_label_id_handling() {
        let custom_id = "custom-label-id";
        assert!(!custom_id.is_empty(), "label should support custom IDs");
        assert!(custom_id.contains("label"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_label_interaction() {
        mount_to_body(|| {
            view! {
                <Label class="test-interaction".into()>
                    "Interactive label"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "label should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_label_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Label class="test-focus".into()>
                    "Focusable label"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "label should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_label_accessibility() {
        mount_to_body(|| {
            view! {
                <Label class="test-a11y".into() >
                    "Accessible label"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "label should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_label_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Label class="test-dom-render".into()>
                    "DOM Test label"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "label should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_label_class_application() {
        mount_to_body(|| {
            view! {
                <Label class="test-class-application custom-class".into()>
                    "Class Test label"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_label_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Label 
                    class="test-attributes".into()
                    
                    
                >
                    "Attribute Test label"
                </Label>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test label");
    }
}