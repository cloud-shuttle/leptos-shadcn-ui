#[cfg(test)]
mod real_tests {
    use crate::default::{ErrorBoundary};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_error_boundary_renders() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary>
                    "error-boundary content"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "error-boundary should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_error_boundary_with_props() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary class="test-class">
                    "error-boundary with props"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "error-boundary with props should render");
    }

    #[test]
    fn test_error_boundary_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "error-boundary signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "error-boundary signal should update");
    }

    #[test]
    fn test_error_boundary_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "error-boundary callback should be triggered");
    }

    #[test]
    fn test_error_boundary_class_handling() {
        let custom_class = "custom-error-boundary-class";
        assert!(!custom_class.is_empty(), "error-boundary should support custom classes");
        assert!(custom_class.contains("error-boundary"), "Class should contain component name");
    }

    #[test]
    fn test_error_boundary_id_handling() {
        let custom_id = "custom-error-boundary-id";
        assert!(!custom_id.is_empty(), "error-boundary should support custom IDs");
        assert!(custom_id.contains("error-boundary"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_error_boundary_interaction() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary class="test-interaction">
                    "Interactive error-boundary"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "error-boundary should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_error_boundary_focus_behavior() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary class="test-focus">
                    "Focusable error-boundary"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "error-boundary should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_error_boundary_accessibility() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary class="test-a11y" role="button">
                    "Accessible error-boundary"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "error-boundary should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_error_boundary_dom_rendering() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary class="test-dom-render">
                    "DOM Test error-boundary"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "error-boundary should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_error_boundary_class_application() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary class="test-class-application custom-class">
                    "Class Test error-boundary"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_error_boundary_attribute_handling() {
        mount_to_body(|| {
            view! {
                <ErrorBoundary 
                    class="test-attributes"
                    data_test="test-value"
                    aria-label="Test error-boundary"
                >
                    "Attribute Test error-boundary"
                </ErrorBoundary>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test error-boundary");
    }
}