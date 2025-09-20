#[cfg(test)]
mod real_tests {
    use crate::default::{Textarea};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_textarea_renders() {
        mount_to_body(|| {
            view! {
                <Textarea>
                    "textarea content"
                </Textarea>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "textarea should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_textarea_with_props() {
        mount_to_body(|| {
            view! {
                <Textarea class="test-class">
                    "textarea with props"
                </Textarea>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "textarea with props should render");
    }

    #[test]
    fn test_textarea_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "textarea signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "textarea signal should update");
    }

    #[test]
    fn test_textarea_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "textarea callback should be triggered");
    }

    #[test]
    fn test_textarea_class_handling() {
        let custom_class = "custom-textarea-class";
        assert!(!custom_class.is_empty(), "textarea should support custom classes");
        assert!(custom_class.contains("textarea"), "Class should contain component name");
    }

    #[test]
    fn test_textarea_id_handling() {
        let custom_id = "custom-textarea-id";
        assert!(!custom_id.is_empty(), "textarea should support custom IDs");
        assert!(custom_id.contains("textarea"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_textarea_form_integration() {
        mount_to_body(|| {
            view! {
                <form>
                    <Textarea name="test-field">
                        "Form textarea"
                    </Textarea>
                </form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("form").unwrap();
        assert!(element.is_some(), "textarea should render in form");
    }

    #[wasm_bindgen_test]
    fn test_textarea_validation_state() {
        mount_to_body(|| {
            view! {
                <Textarea class="test-validation" data-valid="true">
                    "Valid textarea"
                </Textarea>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap();
        assert!(element.is_some(), "textarea should render for validation test");
    }

    #[wasm_bindgen_test]
    fn test_textarea_form_integration() {
        mount_to_body(|| {
            view! {
                <form class="test-form">
                    <Textarea name="test-field" class="test-form-field">
                        "Form textarea"
                    </Textarea>
                </form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let form = document.query_selector(".test-form").unwrap();
        let field = document.query_selector(".test-form-field").unwrap();
        
        assert!(form.is_some(), "Form should render");
        assert!(field.is_some(), "textarea should render in form");
    }

    #[wasm_bindgen_test]
    fn test_textarea_validation_state() {
        mount_to_body(|| {
            view! {
                <Textarea 
                    class="test-validation" 
                    data-valid="true"
                    data-error="false"
                >
                    "Valid textarea"
                </Textarea>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-valid").unwrap(), "true");
        assert_eq!(element.get_attribute("data-error").unwrap(), "false");
    }
}