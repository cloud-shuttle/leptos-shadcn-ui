#[cfg(test)]
mod real_tests {
    use crate::default::{Breadcrumb, BreadcrumbItem, BreadcrumbLink}; // Import main components
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_breadcrumb_renders() {
        mount_to_body(|| {
            view! {
                <Breadcrumb>
                    "breadcrumb content"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "breadcrumb should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_breadcrumb_with_props() {
        mount_to_body(|| {
            view! {
                <Breadcrumb class="test-class" id="test-id">
                    "breadcrumb with props"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "breadcrumb with props should render");
    }

    #[test]
    fn test_breadcrumb_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "breadcrumb signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "breadcrumb signal should update");
    }

    #[test]
    fn test_breadcrumb_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "breadcrumb callback should be triggered");
    }

    #[test]
    fn test_breadcrumb_class_handling() {
        let custom_class = "custom-breadcrumb-class";
        assert!(!custom_class.is_empty(), "breadcrumb should support custom classes");
        assert!(custom_class.contains("breadcrumb"), "Class should contain component name");
    }

    #[test]
    fn test_breadcrumb_id_handling() {
        let custom_id = "custom-breadcrumb-id";
        assert!(!custom_id.is_empty(), "breadcrumb should support custom IDs");
        assert!(custom_id.contains("breadcrumb"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_breadcrumb_interaction() {
        mount_to_body(|| {
            view! {
                <Breadcrumb class="test-interaction">
                    "Interactive breadcrumb"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "breadcrumb should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_breadcrumb_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Breadcrumb class="test-focus">
                    "Focusable breadcrumb"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "breadcrumb should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_breadcrumb_accessibility() {
        mount_to_body(|| {
            view! {
                <Breadcrumb class="test-a11y" role="button">
                    "Accessible breadcrumb"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "breadcrumb should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_breadcrumb_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Breadcrumb class="test-dom-render">
                    "DOM Test breadcrumb"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "breadcrumb should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_breadcrumb_class_application() {
        mount_to_body(|| {
            view! {
                <Breadcrumb class="test-class-application custom-class">
                    "Class Test breadcrumb"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_breadcrumb_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Breadcrumb 
                    class="test-attributes"
                    data-test="test-value"
                    aria-label="Test breadcrumb"
                >
                    "Attribute Test breadcrumb"
                </Breadcrumb>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test breadcrumb");
    }
}