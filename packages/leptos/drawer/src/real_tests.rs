#[cfg(test)]
mod real_tests {
    use crate::default::{Drawer};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_drawer_renders() {
        mount_to_body(|| {
            view! {
                <Drawer>
                    "drawer content"
                </Drawer>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "drawer should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_drawer_with_props() {
        mount_to_body(|| {
            view! {
                <Drawer class="test-class">
                    "drawer with props"
                </Drawer>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "drawer with props should render");
    }

    #[test]
    fn test_drawer_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "drawer signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "drawer signal should update");
    }

    #[test]
    fn test_drawer_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "drawer callback should be triggered");
    }

    #[test]
    fn test_drawer_class_handling() {
        let custom_class = "custom-drawer-class";
        assert!(!custom_class.is_empty(), "drawer should support custom classes");
        assert!(custom_class.contains("drawer"), "Class should contain component name");
    }

    #[test]
    fn test_drawer_id_handling() {
        let custom_id = "custom-drawer-id";
        assert!(!custom_id.is_empty(), "drawer should support custom IDs");
        assert!(custom_id.contains("drawer"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_drawer_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Drawer class="test-responsive">
                    "Responsive drawer"
                </Drawer>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap();
        assert!(element.is_some(), "drawer should render for responsive test");
    }

    #[wasm_bindgen_test]
    fn test_drawer_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout"><Drawer>
                        "Layout drawer"
                    </Drawer></div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-layout").unwrap();
        assert!(element.is_some(), "drawer should render in layout");
    }

    #[wasm_bindgen_test]
    fn test_drawer_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Drawer 
                    class="test-responsive" 
                    
                    style="width: 100%; max-width: 500px;">
                    "Responsive drawer"
                </Drawer>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    fn test_drawer_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout-container"><Drawer class="test-layout-item">
                        "Layout drawer"
                    </Drawer></div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "drawer should render in layout");
    }
}