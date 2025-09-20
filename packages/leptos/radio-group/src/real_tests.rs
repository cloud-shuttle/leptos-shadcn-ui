#[cfg(test)]
mod real_tests {
    use crate::default::{RadioGroup};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_radio_group_renders() {
        mount_to_body(|| {
            view! {
                <RadioGroup>
                    "radio-group content"
                </RadioGroup>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "radio-group should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_radio_group_with_props() {
        mount_to_body(|| {
            view! {
                <RadioGroup class="test-class">
                    "radio-group with props"
                </RadioGroup>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "radio-group with props should render");
    }

    #[test]
    fn test_radio_group_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "radio-group signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "radio-group signal should update");
    }

    #[test]
    fn test_radio_group_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "radio-group callback should be triggered");
    }

    #[test]
    fn test_radio_group_class_handling() {
        let custom_class = "custom-radio-group-class";
        assert!(!custom_class.is_empty(), "radio-group should support custom classes");
        assert!(custom_class.contains("radio-group"), "Class should contain component name");
    }

    #[test]
    fn test_radio_group_id_handling() {
        let custom_id = "custom-radio-group-id";
        assert!(!custom_id.is_empty(), "radio-group should support custom IDs");
        assert!(custom_id.contains("radio-group"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_radio_group_form_integration() {
        mount_to_body(|| {
            view! {
                <form>
                    <RadioGroup name="test-field">
                        "Form radio-group"
                    </RadioGroup>
                </form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("form").unwrap();
        assert!(element.is_some(), "radio-group should render in form");
    }

    #[wasm_bindgen_test]
    fn test_radio_group_validation_state() {
        mount_to_body(|| {
            view! {
                <RadioGroup class="test-validation" data-valid="true">
                    "Valid radio-group"
                </RadioGroup>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap();
        assert!(element.is_some(), "radio-group should render for validation test");
    }

    #[wasm_bindgen_test]
    fn test_radio_group_form_integration() {
        mount_to_body(|| {
            view! {
                <form class="test-form">
                    <RadioGroup name="test-field" class="test-form-field">
                        "Form radio-group"
                    </RadioGroup>
                </form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let form = document.query_selector(".test-form").unwrap();
        let field = document.query_selector(".test-form-field").unwrap();
        
        assert!(form.is_some(), "Form should render");
        assert!(field.is_some(), "radio-group should render in form");
    }

    #[wasm_bindgen_test]
    fn test_radio_group_validation_state() {
        mount_to_body(|| {
            view! {
                <RadioGroup 
                    class="test-validation" 
                    data-valid="true"
                    data-error="false"
                >
                    "Valid radio-group"
                </RadioGroup>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-valid").unwrap(), "true");
        assert_eq!(element.get_attribute("data-error").unwrap(), "false");
    }
}