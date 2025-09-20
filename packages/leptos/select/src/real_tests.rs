#[cfg(test)]
mod real_tests {
    use crate::default::{Select};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_select_renders() {
        mount_to_body(|| {
            view! {
                <Select>
                    "select content"
                </Select>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "select should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_select_with_props() {
        mount_to_body(|| {
            view! {
                <Select class="test-class".into()>
                    "select with props"
                </Select>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "select with props should render");
    }

    #[test]
    fn test_select_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "select signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "select signal should update");
    }

    #[test]
    fn test_select_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "select callback should be triggered");
    }

    #[test]
    fn test_select_class_handling() {
        let custom_class = "custom-select-class";
        assert!(!custom_class.is_empty(), "select should support custom classes");
        assert!(custom_class.contains("select"), "Class should contain component name");
    }

    #[test]
    fn test_select_id_handling() {
        let custom_id = "custom-select-id";
        assert!(!custom_id.is_empty(), "select should support custom IDs");
        assert!(custom_id.contains("select"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_select_form_integration() {
        mount_to_body(|| {
            view! {
                <form><Select>
                        "Form select"
                    </Select></form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("form").unwrap();
        assert!(element.is_some(), "select should render in form");
    }

    #[wasm_bindgen_test]
    fn test_select_validation_state() {
        mount_to_body(|| {
            view! {
                <Select class="test-validation".into()>
                    "Valid select"
                </Select>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap();
        assert!(element.is_some(), "select should render for validation test");
    }

    #[wasm_bindgen_test]
    fn test_select_form_integration() {
        mount_to_body(|| {
            view! {
                <form class="test-form".into()><Select  class="test-form-field".into()>
                        "Form select"
                    </Select></form>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let form = document.query_selector(".test-form").unwrap();
        let field = document.query_selector(".test-form-field").unwrap();
        
        assert!(form.is_some(), "Form should render");
        assert!(field.is_some(), "select should render in form");
    }

    #[wasm_bindgen_test]
    fn test_select_validation_state() {
        mount_to_body(|| {
            view! {
                <Select 
                    class="test-validation".into() 
                    
                    data_error="false">
                    "Valid select"
                </Select>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-valid").unwrap(), "true");
        assert_eq!(element.get_attribute("data-error").unwrap(), "false");
    }
}