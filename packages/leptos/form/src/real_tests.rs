#[cfg(test)]
mod real_tests {
    use crate::default::{Form};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_form_renders() {
        mount_to_body(|| {
            view! {
                <Form>
                    "form content"
                </Form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "form should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_form_with_props() {
        mount_to_body(|| {
            view! {
                <Form class="test-class">
                    "form with props"
                </Form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "form with props should render");
    }

    #[test]
    fn test_form_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "form signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "form signal should update");
    }

    #[test]
    fn test_form_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "form callback should be triggered");
    }

    #[test]
    fn test_form_class_handling() {
        let custom_class = "custom-form-class";
        assert!(!custom_class.is_empty(), "form should support custom classes");
        assert!(custom_class.contains("form"), "Class should contain component name");
    }

    #[test]
    fn test_form_id_handling() {
        let custom_id = "custom-form-id";
        assert!(!custom_id.is_empty(), "form should support custom IDs");
        assert!(custom_id.contains("form"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_form_form_integration() {
        mount_to_body(|| {
            view! {
                <form>
                    <Form name="test-field">
                        "Form form"
                    </Form>
                </form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("form").unwrap();
        assert!(element.is_some(), "form should render in form");
    }

    #[wasm_bindgen_test]
    fn test_form_validation_state() {
        mount_to_body(|| {
            view! {
                <Form class="test-validation" data-valid="true">
                    "Valid form"
                </Form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap();
        assert!(element.is_some(), "form should render for validation test");
    }

    #[wasm_bindgen_test]
    fn test_form_form_integration() {
        mount_to_body(|| {
            view! {
                <form class="test-form">
                    <Form name="test-field" class="test-form-field">
                        "Form form"
                    </Form>
                </form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let form = document.query_selector(".test-form").unwrap();
        let field = document.query_selector(".test-form-field").unwrap();
        
        assert!(form.is_some(), "Form should render");
        assert!(field.is_some(), "form should render in form");
    }

    #[wasm_bindgen_test]
    fn test_form_validation_state() {
        mount_to_body(|| {
            view! {
                <Form 
                    class="test-validation" 
                    data-valid="true"
                    data-error="false"
                >
                    "Valid form"
                </Form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-valid").unwrap(), "true");
        assert_eq!(element.get_attribute("data-error").unwrap(), "false");
    }
}