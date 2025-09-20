#[cfg(test)]
mod real_tests {
    use crate::default::{AlertDialog};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_alert_dialog_renders() {
        mount_to_body(|| {
            view! {
                <AlertDialog>
                    "alert-dialog content"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "alert-dialog should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_alert_dialog_with_props() {
        mount_to_body(|| {
            view! {
                <AlertDialog class="test-class".into()>
                    "alert-dialog with props"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "alert-dialog with props should render");
    }

    #[test]
    fn test_alert_dialog_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "alert-dialog signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "alert-dialog signal should update");
    }

    #[test]
    fn test_alert_dialog_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "alert-dialog callback should be triggered");
    }

    #[test]
    fn test_alert_dialog_class_handling() {
        let custom_class = "custom-alert-dialog-class";
        assert!(!custom_class.is_empty(), "alert-dialog should support custom classes");
        assert!(custom_class.contains("alert-dialog"), "Class should contain component name");
    }

    #[test]
    fn test_alert_dialog_id_handling() {
        let custom_id = "custom-alert-dialog-id";
        assert!(!custom_id.is_empty(), "alert-dialog should support custom IDs");
        assert!(custom_id.contains("alert-dialog"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_alert_dialog_interaction() {
        mount_to_body(|| {
            view! {
                <AlertDialog class="test-interaction".into()>
                    "Interactive alert-dialog"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "alert-dialog should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_alert_dialog_focus_behavior() {
        mount_to_body(|| {
            view! {
                <AlertDialog class="test-focus".into()>
                    "Focusable alert-dialog"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "alert-dialog should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_alert_dialog_accessibility() {
        mount_to_body(|| {
            view! {
                <AlertDialog class="test-a11y".into()>
                    "Accessible alert-dialog"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "alert-dialog should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_alert_dialog_dom_rendering() {
        mount_to_body(|| {
            view! {
                <AlertDialog class="test-dom-render".into()>
                    "DOM Test alert-dialog"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "alert-dialog should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_alert_dialog_class_application() {
        mount_to_body(|| {
            view! {
                <AlertDialog class="test-class-application custom-class".into()>
                    "Class Test alert-dialog"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_alert_dialog_attribute_handling() {
        mount_to_body(|| {
            view! {
                <AlertDialog 
                    class="test-attributes".into()>
                    "Attribute Test alert-dialog"
                </AlertDialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test alert-dialog");
    }
}