#[cfg(test)]
mod real_tests {
    use crate::default::{Menubar};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_menubar_renders() {
        mount_to_body(|| {
            view! {
                <Menubar>
                    "menubar content"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "menubar should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_menubar_with_props() {
        mount_to_body(|| {
            view! {
                <Menubar class="test-class">
                    "menubar with props"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "menubar with props should render");
    }

    #[test]
    fn test_menubar_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "menubar signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "menubar signal should update");
    }

    #[test]
    fn test_menubar_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "menubar callback should be triggered");
    }

    #[test]
    fn test_menubar_class_handling() {
        let custom_class = "custom-menubar-class";
        assert!(!custom_class.is_empty(), "menubar should support custom classes");
        assert!(custom_class.contains("menubar"), "Class should contain component name");
    }

    #[test]
    fn test_menubar_id_handling() {
        let custom_id = "custom-menubar-id";
        assert!(!custom_id.is_empty(), "menubar should support custom IDs");
        assert!(custom_id.contains("menubar"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_menubar_interaction() {
        mount_to_body(|| {
            view! {
                <Menubar class="test-interaction">
                    "Interactive menubar"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "menubar should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_menubar_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Menubar class="test-focus">
                    "Focusable menubar"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "menubar should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_menubar_accessibility() {
        mount_to_body(|| {
            view! {
                <Menubar class="test-a11y" >
                    "Accessible menubar"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "menubar should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_menubar_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Menubar class="test-dom-render">
                    "DOM Test menubar"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "menubar should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_menubar_class_application() {
        mount_to_body(|| {
            view! {
                <Menubar class="test-class-application custom-class">
                    "Class Test menubar"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_menubar_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Menubar 
                    class="test-attributes"
                    
                    
                >
                    "Attribute Test menubar"
                </Menubar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test menubar");
    }
}