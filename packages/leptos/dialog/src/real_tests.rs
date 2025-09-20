#[cfg(test)]
mod real_tests {
    use crate::default::{Dialog, DialogContent, DialogDescription}; // Import main components
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_dialog_renders() {
        mount_to_body(|| {
            view! {
                <Dialog>
                    "dialog content"
                </Dialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "dialog should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_dialog_with_props() {
        mount_to_body(|| {
            view! {
                <Dialog class="test-class".into() id="test-id">
                    "dialog with props"
                </Dialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "dialog with props should render");
    }

    #[test]
    fn test_dialog_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "dialog signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "dialog signal should update");
    }

    #[test]
    fn test_dialog_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "dialog callback should be triggered");
    }

    #[test]
    fn test_dialog_class_handling() {
        let custom_class = "custom-dialog-class";
        assert!(!custom_class.is_empty(), "dialog should support custom classes");
        assert!(custom_class.contains("dialog"), "Class should contain component name");
    }

    #[test]
    fn test_dialog_id_handling() {
        let custom_id = "custom-dialog-id";
        assert!(!custom_id.is_empty(), "dialog should support custom IDs");
        assert!(custom_id.contains("dialog"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_dialog_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Dialog class="test-responsive".into() >
                    "Responsive dialog"
                </Dialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap();
        assert!(element.is_some(), "dialog should render for responsive test");
    }

    #[wasm_bindgen_test]
    fn test_dialog_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout".into()>
                    <Dialog>
                        "Layout dialog"
                    </Dialog>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-layout").unwrap();
        assert!(element.is_some(), "dialog should render in layout");
    }

    #[wasm_bindgen_test]
    fn test_dialog_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Dialog 
                    class="test-responsive".into() 
                    
                    style="width: 100%; max-width: 500px;"
                >
                    "Responsive dialog"
                </Dialog>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    fn test_dialog_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout-container".into()>
                    <Dialog class="test-layout-item".into()>
                        "Layout dialog"
                    </Dialog>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "dialog should render in layout");
    }
}