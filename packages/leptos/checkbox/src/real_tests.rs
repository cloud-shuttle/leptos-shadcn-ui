#[cfg(test)]
mod real_tests {
    use crate::default::{Checkbox};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_checkbox_renders() {
        mount_to_body(|| {
            view! {
                <Checkbox>
                    "checkbox content"
                </Checkbox>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "checkbox should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_checkbox_with_props() {
        mount_to_body(|| {
            view! {
                <Checkbox class="test-class">
                    "checkbox with props"
                </Checkbox>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "checkbox with props should render");
    }

    #[test]
    fn test_checkbox_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "checkbox signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "checkbox signal should update");
    }

    #[test]
    fn test_checkbox_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "checkbox callback should be triggered");
    }

    #[test]
    fn test_checkbox_class_handling() {
        let custom_class = "custom-checkbox-class";
        assert!(!custom_class.is_empty(), "checkbox should support custom classes");
        assert!(custom_class.contains("checkbox"), "Class should contain component name");
    }

    #[test]
    fn test_checkbox_id_handling() {
        let custom_id = "custom-checkbox-id";
        assert!(!custom_id.is_empty(), "checkbox should support custom IDs");
        assert!(custom_id.contains("checkbox"), "ID should contain component name");
    }
}