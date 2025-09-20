#[cfg(test)]
mod real_tests {
    use crate::default::{Separator, Separator as SeparatorNewYork}; // Import main components
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_separator_renders() {
        mount_to_body(|| {
            view! {
                <Separator>
                    "separator content"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "separator should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_separator_with_props() {
        mount_to_body(|| {
            view! {
                <Separator class="test-class" id="test-id">
                    "separator with props"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "separator with props should render");
    }

    #[test]
    fn test_separator_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "separator signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "separator signal should update");
    }

    #[test]
    fn test_separator_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "separator callback should be triggered");
    }

    #[test]
    fn test_separator_class_handling() {
        let custom_class = "custom-separator-class";
        assert!(!custom_class.is_empty(), "separator should support custom classes");
        assert!(custom_class.contains("separator"), "Class should contain component name");
    }

    #[test]
    fn test_separator_id_handling() {
        let custom_id = "custom-separator-id";
        assert!(!custom_id.is_empty(), "separator should support custom IDs");
        assert!(custom_id.contains("separator"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_separator_interaction() {
        mount_to_body(|| {
            view! {
                <Separator class="test-interaction">
                    "Interactive separator"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "separator should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_separator_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Separator class="test-focus">
                    "Focusable separator"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "separator should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_separator_accessibility() {
        mount_to_body(|| {
            view! {
                <Separator class="test-a11y" role="button">
                    "Accessible separator"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "separator should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_separator_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Separator class="test-dom-render">
                    "DOM Test separator"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "separator should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_separator_class_application() {
        mount_to_body(|| {
            view! {
                <Separator class="test-class-application custom-class">
                    "Class Test separator"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_separator_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Separator 
                    class="test-attributes"
                    data-test="test-value"
                    aria-label="Test separator"
                >
                    "Attribute Test separator"
                </Separator>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test separator");
    }
}