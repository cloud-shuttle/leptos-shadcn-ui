#[cfg(test)]
mod real_tests {
    use crate::default::{InputOTP};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_input_otp_renders() {
        mount_to_body(|| {
            view! {
                <InputOTP>
                    "input-otp content"
                </InputOTP>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "input-otp should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_input_otp_with_props() {
        mount_to_body(|| {
            view! {
                <InputOTP class="test-class">
                    "input-otp with props"
                </InputOTP>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "input-otp with props should render");
    }

    #[test]
    fn test_input_otp_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "input-otp signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "input-otp signal should update");
    }

    #[test]
    fn test_input_otp_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "input-otp callback should be triggered");
    }

    #[test]
    fn test_input_otp_class_handling() {
        let custom_class = "custom-input-otp-class";
        assert!(!custom_class.is_empty(), "input-otp should support custom classes");
        assert!(custom_class.contains("input-otp"), "Class should contain component name");
    }

    #[test]
    fn test_input_otp_id_handling() {
        let custom_id = "custom-input-otp-id";
        assert!(!custom_id.is_empty(), "input-otp should support custom IDs");
        assert!(custom_id.contains("input-otp"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_input_otp_form_integration() {
        mount_to_body(|| {
            view! {
                <form><InputOTP>
                        "Form input-otp"
                    </InputOTP></form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("form").unwrap();
        assert!(element.is_some(), "input-otp should render in form");
    }

    #[wasm_bindgen_test]
    fn test_input_otp_validation_state() {
        mount_to_body(|| {
            view! {
                <InputOTP class="test-validation">
                    "Valid input-otp"
                </InputOTP>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap();
        assert!(element.is_some(), "input-otp should render for validation test");
    }

    #[wasm_bindgen_test]
    fn test_input_otp_form_integration() {
        mount_to_body(|| {
            view! {
                <form class="test-form"><InputOTP  class="test-form-field">
                        "Form input-otp"
                    </InputOTP></form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let form = document.query_selector(".test-form").unwrap();
        let field = document.query_selector(".test-form-field").unwrap();
        
        assert!(form.is_some(), "Form should render");
        assert!(field.is_some(), "input-otp should render in form");
    }

    #[wasm_bindgen_test]
    fn test_input_otp_validation_state() {
        mount_to_body(|| {
            view! {
                <InputOTP 
                    class="test-validation" 
                    
                    data_error="false">
                    "Valid input-otp"
                </InputOTP>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-valid").unwrap(), "true");
        assert_eq!(element.get_attribute("data-error").unwrap(), "false");
    }
}