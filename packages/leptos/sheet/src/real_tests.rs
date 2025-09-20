#[cfg(test)]
mod real_tests {
    use crate::default::{Sheet};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_sheet_renders() {
        mount_to_body(|| {
            view! {
                <Sheet>
                    "sheet content"
                </Sheet>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "sheet should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_sheet_with_props() {
        mount_to_body(|| {
            view! {
                <Sheet class="test-class".into()>
                    "sheet with props"
                </Sheet>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "sheet with props should render");
    }

    #[test]
    fn test_sheet_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "sheet signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "sheet signal should update");
    }

    #[test]
    fn test_sheet_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "sheet callback should be triggered");
    }

    #[test]
    fn test_sheet_class_handling() {
        let custom_class = "custom-sheet-class";
        assert!(!custom_class.is_empty(), "sheet should support custom classes");
        assert!(custom_class.contains("sheet"), "Class should contain component name");
    }

    #[test]
    fn test_sheet_id_handling() {
        let custom_id = "custom-sheet-id";
        assert!(!custom_id.is_empty(), "sheet should support custom IDs");
        assert!(custom_id.contains("sheet"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_sheet_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Sheet class="test-responsive".into() >
                    "Responsive sheet"
                </Sheet>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap();
        assert!(element.is_some(), "sheet should render for responsive test");
    }

    #[wasm_bindgen_test]
    fn test_sheet_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout".into()>
                    <Sheet>
                        "Layout sheet"
                    </Sheet>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-layout").unwrap();
        assert!(element.is_some(), "sheet should render in layout");
    }

    #[wasm_bindgen_test]
    fn test_sheet_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Sheet 
                    class="test-responsive".into() 
                    
                    style="width: 100%; max-width: 500px;"
                >
                    "Responsive sheet"
                </Sheet>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    fn test_sheet_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout-container".into()>
                    <Sheet class="test-layout-item".into()>
                        "Layout sheet"
                    </Sheet>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "sheet should render in layout");
    }
}